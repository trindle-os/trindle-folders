//! FTP in Trindle's own Rust: passive mode only (EPSV, then PASV), binary

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

use super::tls::{Link, Secure};
use super::{now, read_line_most, too_much, too_much_read, unix_time, Address, Entry, Error, Kind, Login, Progress, Remote, Scheme};
use crate::files::leave::Leave;
use crate::toolkit::t;

const WAIT: Duration = Duration::from_secs(30);
const BUF: usize = 64 * 1024;

pub struct Ftp {
    ctl: BufReader<Link>,
    peer: IpAddr,
    secure: Option<Secure>,
    epsv: Option<bool>,
    mlsd: Option<bool>,
}

impl Ftp {
    pub fn connect(address: &Address, to: &Leave<Login>) -> Result<Ftp, Error> {
        let login = to.login();
        let host = &address.host;
        let addrs: Vec<SocketAddr> = match to.address() {
            Some(at) => {
                let at: IpAddr = at.parse().map_err(|_| Error::Unreachable(t!("folders-remote-ftp-no-server", host = host)))?;
                vec![SocketAddr::new(at, address.port())]
            }
            None => (host.as_str(), address.port())
                .to_socket_addrs()
                .map_err(|_| Error::Unreachable(t!("folders-remote-ftp-no-server", host = host)))?
                .collect(),
        };
        let mut last = None;
        let mut stream = None;
        for a in &addrs {
            match TcpStream::connect_timeout(a, Duration::from_secs(20)) {
                Ok(s) => {
                    stream = Some(s);
                    break;
                }
                Err(e) => last = Some(e),
            }
        }
        let stream = stream.ok_or_else(|| match last.map(|e| e.kind()) {
            Some(io::ErrorKind::ConnectionRefused) => Error::Unreachable(t!("folders-remote-ftp-turned-away", host = host)),
            Some(io::ErrorKind::TimedOut) => Error::Unreachable(t!("folders-remote-ftp-no-answer", host = host)),
            _ => Error::Unreachable(t!("folders-remote-ftp-no-way", host = host)),
        })?;
        let secure = || Secure::new(host, address.port(), login.trust_cert.as_deref());
        let mut ftp = if address.scheme == Scheme::Ftps {
            times(&stream)?;
            let tls = secure()?;
            let link = tls.wrap(stream)?;
            let mut ftp = Ftp::over(link)?;
            ftp.secure = Some(tls);
            ftp
        } else {
            Ftp::over(Link::Plain(stream))?
        };
        if address.scheme == Scheme::Ftpes {
            ftp.upgrade(secure()?)?;
        }
        match ftp.login(address, login) {
            Err(Error::NotSupported(what)) if what == secure_ftp_name() && ftp.secure.is_none() => {
                ftp.upgrade(secure()?)?;
                ftp.login(address, login)?;
            }
            r => r?,
        }
        if ftp.secure.is_some() {
            let _ = ftp.command("PBSZ 0");
            let (code, text) = ftp.command("PROT P")?;
            if code != 200 {
                return Err(Error::Failed(t!("folders-remote-ftp-no-protect", said = text.lines().last().unwrap_or("").trim())));
            }
        }
        ftp.setup()?;
        Ok(ftp)
    }

    fn over(link: Link) -> Result<Ftp, Error> {
        times(link.tcp())?;
        let peer = link.tcp().peer_addr()?.ip();
        let mut ftp = Ftp { ctl: BufReader::new(link), peer, secure: None, epsv: None, mlsd: None };
        let (mut code, mut text) = ftp.reply()?;
        if code == 120 {
            (code, text) = ftp.reply()?;
        }
        if code != 220 {
            return Err(Error::Failed(t!("folders-remote-ftp-no-welcome", said = text.trim())));
        }
        Ok(ftp)
    }

    fn upgrade(&mut self, tls: Secure) -> Result<(), Error> {
        let (code, text) = self.command("AUTH TLS")?;
        if code != 234 {
            return Err(if code == 500 || code == 502 || code == 504 {
                Error::Failed(t!("folders-remote-ftp-no-secure").into())
            } else {
                failed(code, &text, t!("folders-remote-ftp-what-secure"))
            });
        }
        let tcp = self.ctl.get_ref().tcp().try_clone()?;
        let link = tls.wrap(tcp)?;
        self.ctl = BufReader::new(link);
        self.secure = Some(tls);
        Ok(())
    }

    fn login(&mut self, address: &Address, login: &Login) -> Result<(), Error> {
        let user = address.user.clone().unwrap_or_else(|| "anonymous".into());
        let anonymous = user == "anonymous" || user == "ftp";
        let password = login.password.clone();
        let (mut code, mut text) = self.command(&format!("USER {user}"))?;
        if code == 331 {
            let pass = match (&password, anonymous) {
                (Some(p), _) => p.clone(),
                (None, true) => super::Secret::new("anonymous@".into()),
                (None, false) => return Err(Error::NeedsPassword),
            };
            (code, text) = self.command(&super::Secret::new(format!("PASS {}", pass.as_str())))?;
        }
        match code {
            230 | 202 => Ok(()),
            332 => Err(Error::NotSupported(t!("folders-remote-ftp-accounts").into())),
            _ if self.secure.is_none() && secure_only(&text) => Err(Error::NotSupported(secure_ftp_name().into())),
            530 if password.is_none() => Err(Error::NeedsPassword),
            530 => Err(Error::LoginRefused),
            _ => Err(failed(code, &text, t!("folders-remote-ftp-what-login"))),
        }
    }

    fn setup(&mut self) -> Result<(), Error> {
        self.expect("TYPE I", &[200], t!("folders-remote-ftp-what-binary"))?;
        let _ = self.command("OPTS UTF8 ON");
        Ok(())
    }

    fn reply(&mut self) -> Result<(u16, String), Error> {
        read_reply(&mut self.ctl).map_err(lost)
    }

    fn send(&mut self, line: &str) -> io::Result<()> {
        let out = self.ctl.get_mut();
        out.write_all(format!("{line}\r\n").as_bytes())?;
        out.flush()
    }

    fn command(&mut self, line: &str) -> Result<(u16, String), Error> {
        if line.contains(['\r', '\n']) {
            return Err(Error::BadAddress(t!("folders-remote-address-line-break").into()));
        }
        self.send(line).map_err(lost)?;
        self.reply()
    }

    fn expect(&mut self, line: &str, ok: &[u16], what: &str) -> Result<String, Error> {
        let (code, text) = self.command(line)?;
        if ok.contains(&code) {
            Ok(text)
        } else {
            Err(failed(code, &text, what))
        }
    }

    fn passive(&mut self) -> Result<TcpStream, Error> {
        let port = loop {
            if self.epsv != Some(false) {
                let (code, text) = self.command("EPSV")?;
                if code == 229 {
                    if let Some(p) = epsv_port(&text) {
                        self.epsv = Some(true);
                        break p;
                    }
                }
                if self.epsv == Some(true) {
                    return Err(failed(code, &text, t!("folders-remote-ftp-what-data")));
                }
                self.epsv = Some(false);
                continue;
            }
            let (code, text) = self.command("PASV")?;
            match (code, pasv_port(&text)) {
                (227, Some(p)) => break p,
                _ => return Err(failed(code, &text, t!("folders-remote-ftp-what-data"))),
            }
        };
        let data = TcpStream::connect_timeout(&SocketAddr::new(self.peer, port), WAIT)
            .map_err(|_| Error::Failed(t!("folders-remote-ftp-data-failed").into()))?;
        times(&data)?;
        Ok(data)
    }

    fn open(&mut self, data: TcpStream) -> Result<Link, Error> {
        let Some(tls) = &self.secure else { return Ok(Link::Plain(data)) };
        match tls.wrap(data) {
            Ok(link) => Ok(link),
            Err(e) => {
                self.settle();
                Err(e)
            }
        }
    }

    fn transfer(&mut self, line: &str, what: &str) -> Result<Link, Error> {
        let data = self.passive()?;
        let (code, text) = self.command(line)?;
        if code == 125 || code == 150 {
            self.open(data)
        } else {
            Err(failed(code, &text, what))
        }
    }

    fn finish(&mut self, what: &str) -> Result<(), Error> {
        let (code, text) = self.reply()?;
        if code == 226 || code == 250 {
            Ok(())
        } else {
            Err(failed(code, &text, what))
        }
    }

    fn abort(&mut self, data: Link) {
        drop(data);
        self.settle();
    }

    fn settle(&mut self) {
        let _ = self.send("ABOR");
        let _ = self.send("NOOP");
        for _ in 0..5 {
            match self.reply() {
                Ok((200, _)) | Err(_) => break,
                Ok(_) => {}
            }
        }
    }

    fn lines(&mut self, path: &str, force_list: bool) -> Result<(Vec<String>, bool), Error> {
        if self.mlsd != Some(false) && !force_list {
            let data = self.passive()?;
            let (code, text) = self.command(&format!("MLSD {path}"))?;
            match code {
                125 | 150 => {
                    self.mlsd = Some(true);
                    let data = self.open(data)?;
                    let lines = read_lines(data)?;
                    self.finish(path)?;
                    return Ok((lines, true));
                }
                500 | 501 | 502 | 504 => self.mlsd = Some(false),
                _ => return Err(failed(code, &text, path)),
            }
        }
        self.expect(&format!("CWD {path}"), &[200, 250], path)?;
        let data = self.transfer("LIST", path)?;
        let lines = read_lines(data)?;
        self.finish(path)?;
        Ok((lines, false))
    }
}

fn secure_ftp_name() -> &'static str {
    t!("folders-remote-ftp-secure-ftp")
}

fn times(s: &TcpStream) -> Result<(), Error> {
    s.set_read_timeout(Some(WAIT))?;
    s.set_write_timeout(Some(WAIT))?;
    let _ = s.set_nodelay(true);
    Ok(())
}

impl Drop for Ftp {
    fn drop(&mut self) {
        let _ = self.send("QUIT");
        let link = self.ctl.get_mut();
        if link.is_tls() {
            link.finish_writing();
        }
    }
}

impl Remote for Ftp {
    fn home(&mut self) -> Result<String, Error> {
        let text = self.expect("PWD", &[257], t!("folders-remote-ftp-what-home"))?;
        Ok(quoted(&text).unwrap_or_else(|| "/".into()))
    }

    fn list(&mut self, path: &str) -> Result<Vec<Entry>, Error> {
        let (lines, mlsd) = self.lines(path, false)?;
        let now = now();
        Ok(lines
            .iter()
            .filter_map(|l| if mlsd { mlsd_line(l) } else { list_line(l, now).map(|e| e.0) })
            .filter(|e| e.name != "." && e.name != "..")
            .collect())
    }

    fn stat(&mut self, path: &str) -> Result<Entry, Error> {
        if path.trim_end_matches('/').is_empty() {
            return Ok(Entry { name: "/".into(), kind: Kind::Folder, size: 0, modified: None, mode: None });
        }
        if self.mlsd != Some(false) {
            let (code, text) = self.command(&format!("MLST {path}"))?;
            if code == 250 {
                if let Some(mut e) = text.lines().find(|l| l.starts_with(' ')).and_then(|l| mlsd_line(l.trim_start())) {
                    e.name = super::name_of(path).to_string();
                    return Ok(e);
                }
            } else if code == 550 {
                return Err(failed(code, &text, path));
            }
        }
        let name = super::name_of(path);
        self.list(&super::parent(path))?
            .into_iter()
            .find(|e| e.name == name)
            .ok_or_else(|| Error::NotFound(path.into()))
    }

    fn read(&mut self, path: &str, to: &mut dyn Write, progress: Progress) -> Result<u64, Error> {
        let mut data = self.transfer(&format!("RETR {path}"), path)?;
        let mut buf = vec![0u8; BUF];
        let mut done = 0u64;
        loop {
            let n = match data.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => {
                    self.abort(data);
                    return Err(lost(e));
                }
            };
            if let Err(e) = to.write_all(&buf[..n]) {
                self.abort(data);
                return Err(Error::Io(e));
            }
            done += n as u64;
            if !progress(done) {
                self.abort(data);
                return Err(Error::Cancelled);
            }
        }
        drop(data);
        self.finish(path)?;
        Ok(done)
    }

    fn write_new(&mut self, path: &str, from: &mut dyn Read, _size: u64, progress: Progress) -> Result<u64, Error> {
        super::free(self, path)?;
        let mut data = self.transfer(&format!("STOR {path}"), path)?;
        let mut buf = vec![0u8; BUF];
        let mut done = 0u64;
        let failed = loop {
            let n = match from.read(&mut buf) {
                Ok(0) => break None,
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => break Some(Error::Io(e)),
            };
            if let Err(e) = data.write_all(&buf[..n]) {
                break Some(lost(e));
            }
            done += n as u64;
            if !progress(done) {
                break Some(Error::Cancelled);
            }
        };
        if let Some(e) = failed {
            self.abort(data);
            let _ = self.command(&format!("DELE {path}"));
            return Err(e);
        }
        data.finish_writing();
        drop(data);
        self.finish(path)?;
        Ok(done)
    }

    fn mkdir(&mut self, path: &str) -> Result<(), Error> {
        self.expect(&format!("MKD {path}"), &[250, 257], path).map(drop)
    }

    fn rmdir(&mut self, path: &str) -> Result<(), Error> {
        self.expect(&format!("RMD {path}"), &[250], path).map(drop)
    }

    fn remove(&mut self, path: &str) -> Result<(), Error> {
        self.expect(&format!("DELE {path}"), &[250], path).map(drop)
    }

    fn rename(&mut self, from: &str, to: &str) -> Result<(), Error> {
        self.expect(&format!("RNFR {from}"), &[350], from)?;
        self.expect(&format!("RNTO {to}"), &[250], to).map(drop)
    }

    fn readlink(&mut self, path: &str) -> Result<String, Error> {
        let (lines, _) = self.lines(&super::parent(path), true)?;
        let name = super::name_of(path);
        let now = now();
        lines
            .iter()
            .filter_map(|l| list_line(l, now))
            .find(|(e, _)| e.name == name)
            .and_then(|(_, target)| target)
            .ok_or_else(|| Error::NotFound(path.into()))
    }
}

fn lost(e: io::Error) -> Error {
    match e.kind() {
        io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => Error::Lost(t!("folders-remote-ftp-stopped").into()),
        io::ErrorKind::UnexpectedEof | io::ErrorKind::BrokenPipe | io::ErrorKind::ConnectionReset => {
            Error::Lost(t!("folders-remote-ftp-lost").into())
        }
        _ => Error::Io(e),
    }
}

fn secure_only(text: &str) -> bool {
    let t = text.to_ascii_lowercase();
    ["tls", "ssl", "encrypt", "auth "].iter().any(|w| t.contains(w))
}

fn failed(code: u16, text: &str, what: &str) -> Error {
    let t = text.to_ascii_lowercase();
    match code {
        421 => Error::Lost(t!("folders-remote-ftp-closed").into()),
        530 => Error::LoginRefused,
        550 if t.contains("denied") || t.contains("permission") || t.contains("not allowed") => Error::Denied(what.into()),
        550 if t.contains("exists") => Error::Exists(what.into()),
        550 => Error::NotFound(what.into()),
        553 => Error::Denied(what.into()),
        502 | 504 => Error::NotSupported(t!("folders-remote-ftp-that-here").into()),
        _ => {
            let said = text.lines().last().unwrap_or("").trim();
            Error::Failed(t!("folders-remote-ftp-could-not", what = what, said = said))
        }
    }
}

const MOST_LINE: usize = 64 << 10;
const MOST_REPLY: usize = 1 << 20;

fn read_reply(r: &mut dyn BufRead) -> io::Result<(u16, String)> {
    let mut line = String::new();
    let mut take = |line: &mut String| -> io::Result<()> {
        line.clear();
        let mut raw = Vec::new();
        if read_line_most(&mut *r, MOST_LINE, &mut raw)? == 0 {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        line.push_str(String::from_utf8_lossy(&raw).trim_end_matches(['\r', '\n']));
        Ok(())
    };
    take(&mut line)?;
    let code: u16 = line.get(..3).and_then(|c| c.parse().ok()).ok_or_else(|| io::Error::other(t!("folders-remote-ftp-not-reply")))?;
    let mut text = line.get(4..).unwrap_or("").to_string();
    if line.as_bytes().get(3) == Some(&b'-') {
        let end = format!("{code} ");
        loop {
            take(&mut line)?;
            if text.len() > MOST_REPLY {
                return Err(too_much_read());
            }
            text.push('\n');
            if let Some(last) = line.strip_prefix(&end) {
                text.push_str(last);
                break;
            }
            text.push_str(&line);
        }
    }
    Ok((code, text))
}

const MOST_LISTING: u64 = 64 << 20;

fn read_lines(mut data: impl Read) -> Result<Vec<String>, Error> {
    let mut raw = Vec::new();
    (&mut data).take(MOST_LISTING + 1).read_to_end(&mut raw).map_err(lost)?;
    if raw.len() as u64 > MOST_LISTING {
        return Err(too_much());
    }
    Ok(String::from_utf8_lossy(&raw).lines().map(|l| l.trim_end_matches('\r').to_string()).filter(|l| !l.is_empty()).collect())
}

fn epsv_port(text: &str) -> Option<u16> {
    let inner = &text[text.find('(')? + 1..];
    let d = inner.chars().next()?;
    inner.split(d).nth(3)?.parse().ok()
}

fn pasv_port(text: &str) -> Option<u16> {
    let start = text.find(|c: char| c.is_ascii_digit())?;
    let nums: Vec<u16> = text[start..]
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .take(6)
        .map(|s| s.parse().ok())
        .collect::<Option<_>>()?;
    if nums.len() != 6 || nums.iter().any(|&n| n > 255) {
        return None;
    }
    Some(nums[4] * 256 + nums[5])
}

fn quoted(text: &str) -> Option<String> {
    let rest = &text[text.find('"')? + 1..];
    let mut out = String::new();
    let mut chars = rest.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '"' {
            if chars.peek() == Some(&'"') {
                chars.next();
                out.push('"');
            } else {
                return Some(out);
            }
        } else {
            out.push(c);
        }
    }
    None
}

fn mlsd_line(line: &str) -> Option<Entry> {
    let (facts, name) = line.split_once(' ')?;
    let mut e = Entry { name: name.to_string(), kind: Kind::File, size: 0, modified: None, mode: None };
    for fact in facts.split(';') {
        let Some((key, value)) = fact.split_once('=') else { continue };
        match key.to_ascii_lowercase().as_str() {
            "type" => {
                let v = value.to_ascii_lowercase();
                e.kind = match v.as_str() {
                    "file" => Kind::File,
                    "dir" => Kind::Folder,
                    "cdir" | "pdir" => return None,
                    _ if v.starts_with("os.unix=slink") || v.starts_with("os.unix=symlink") => Kind::Link,
                    _ => Kind::Other,
                };
            }
            "size" | "sizd" => e.size = value.parse().unwrap_or(0),
            "modify" => e.modified = stamp(value),
            "unix.mode" => e.mode = u32::from_str_radix(value, 8).ok(),
            _ => {}
        }
    }
    Some(e)
}

fn stamp(v: &str) -> Option<i64> {
    let v = v.get(..14)?;
    let n = |a: usize, b: usize| v.get(a..b)?.parse::<u32>().ok();
    Some(unix_time(n(0, 4)? as i64, n(4, 6)?, n(6, 8)?, n(8, 10)?, n(10, 12)?, n(12, 14)?))
}

fn words(line: &str) -> Vec<(usize, usize, &str)> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in line.char_indices().chain(std::iter::once((line.len(), ' '))) {
        match (c.is_whitespace(), start) {
            (true, Some(s)) => {
                out.push((s, i, &line[s..i]));
                start = None;
            }
            (false, None) => start = Some(i),
            _ => {}
        }
    }
    out
}

fn month(word: &str) -> Option<u32> {
    let names = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
    let w = word.to_ascii_lowercase();
    names.iter().position(|m| *m == w).map(|i| i as u32 + 1)
}

fn list_line(line: &str, now: i64) -> Option<(Entry, Option<String>)> {
    unix_line(line, now).or_else(|| dos_line(line))
}

fn unix_line(line: &str, now: i64) -> Option<(Entry, Option<String>)> {
    let w = words(line);
    let perms = w.first()?.2;
    let kind = match perms.chars().next()? {
        'd' => Kind::Folder,
        'l' => Kind::Link,
        '-' => Kind::File,
        'b' | 'c' | 'p' | 's' => Kind::Other,
        _ => return None,
    };
    let i = (2..w.len().saturating_sub(3)).find(|&i| {
        month(w[i].2).is_some()
            && w[i - 1].2.parse::<u64>().is_ok()
            && w[i + 1].2.parse::<u32>().is_ok_and(|d| (1..=31).contains(&d))
            && (w[i + 2].2.contains(':') || w[i + 2].2.parse::<u32>().is_ok())
    })?;
    let size = w[i - 1].2.parse().ok()?;
    let (m, d) = (month(w[i].2)?, w[i + 1].2.parse().ok()?);
    let when = w[i + 2].2;
    let modified = match when.split_once(':') {
        Some((h, mi)) => {
            let (h, mi) = (h.parse().ok()?, mi.parse().ok()?);
            let year = super::civil(now).0;
            let t = unix_time(year, m, d, h, mi, 0);
            Some(if t > now + 86_400 { unix_time(year - 1, m, d, h, mi, 0) } else { t })
        }
        None => Some(unix_time(when.parse().ok()?, m, d, 0, 0, 0)),
    };
    let rest = &line[w[i + 2].1..];
    let rest = rest.strip_prefix([' ', '\t']).unwrap_or(rest);
    let (name, target) = match (kind, rest.split_once(" -> ")) {
        (Kind::Link, Some((n, t))) => (n, Some(t.to_string())),
        _ => (rest, None),
    };
    if name.is_empty() {
        return None;
    }
    let e = Entry { name: name.to_string(), kind, size, modified, mode: mode(perms) };
    Some((e, target))
}

fn mode(perms: &str) -> Option<u32> {
    let b = perms.as_bytes();
    if b.len() < 10 {
        return None;
    }
    let mut m = 0;
    for (i, &c) in b[1..10].iter().enumerate() {
        let bit = 1 << (8 - i);
        let special = match i {
            2 => 0o4000,
            5 => 0o2000,
            8 => 0o1000,
            _ => 0,
        };
        match c {
            b'-' => {}
            b'r' | b'w' | b'x' => m |= bit,
            b's' | b't' => m |= bit | special,
            b'S' | b'T' => m |= special,
            _ => return None,
        }
    }
    Some(m)
}

fn dos_line(line: &str) -> Option<(Entry, Option<String>)> {
    let w = words(line);
    if w.len() < 4 {
        return None;
    }
    let date: Vec<u32> = w[0].2.split('-').map(|n| n.parse().ok()).collect::<Option<_>>()?;
    let [mo, d, y] = date[..] else { return None };
    let year = match y {
        0..=69 => 2000 + y as i64,
        70..=99 => 1900 + y as i64,
        _ => y as i64,
    };
    let t = w[1].2.to_ascii_uppercase();
    let (clock, pm) = match (t.strip_suffix("PM"), t.strip_suffix("AM")) {
        (Some(c), _) => (c, Some(true)),
        (_, Some(c)) => (c, Some(false)),
        _ => (t.as_str(), None),
    };
    let (h, mi) = clock.split_once(':')?;
    let (mut h, mi): (u32, u32) = (h.parse().ok()?, mi.parse().ok()?);
    match pm {
        Some(true) if h < 12 => h += 12,
        Some(false) if h == 12 => h = 0,
        _ => {}
    }
    let (kind, size) = if w[2].2.eq_ignore_ascii_case("<DIR>") { (Kind::Folder, 0) } else { (Kind::File, w[2].2.parse().ok()?) };
    let name = line[w[2].1..].trim_start();
    if !(1..=12).contains(&mo) || name.is_empty() {
        return None;
    }
    let e = Entry { name: name.to_string(), kind, size, modified: Some(unix_time(year, mo, d, h, mi, 0)), mode: None };
    Some((e, None))
}

#[cfg(test)]
mod tests;
