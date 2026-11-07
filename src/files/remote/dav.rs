// WebDAV, in Trindle's own Rust.

use std::io::{self, BufReader, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

use md5::{Digest, Md5};
use quick_xml::events::Event;
use quick_xml::Reader;

use super::tls::{Link, Secure};
use super::{name_of, read_line_most, too_much, unix_time, Address, Entry, Error, Kind, Login, Progress, Remote, Scheme};
use crate::files::leave::Leave;
use crate::toolkit::t;

const WAIT: Duration = Duration::from_secs(20);
const CHUNK: usize = 64 * 1024;
const MOST_LISTING: u64 = 64 << 20;
const MOST_LINE: usize = 64 << 10;
const MOST_HEAD: usize = 1 << 20;
const MOST_CHUNK_LINE: usize = 4096;
const PROPFIND: &str = r#"<?xml version="1.0" encoding="utf-8"?><D:propfind xmlns:D="DAV:"><D:prop><D:resourcetype/><D:getcontentlength/><D:getlastmodified/><D:getetag/></D:prop></D:propfind>"#;

type Listed = (String, Entry, Option<String>);

fn strong(tag: &Option<String>) -> Option<String> {
    tag.as_ref().filter(|t| t.starts_with('"') && t.len() > 2).cloned()
}

fn changed(path: &str) -> Error {
    Error::Failed(t!("folders-remote-dav-changed-at-removal", name = name_of(bare(path))))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Auth {
    None,
    Basic,
    Digest { realm: String, nonce: String, opaque: Option<String>, qop: bool, count: u32 },
}

pub struct Dav {
    secure: Option<Secure>,
    host: String,
    at: Option<IpAddr>,
    port: u16,
    user: Option<String>,
    password: Option<String>,
    auth: Auth,
    home: String,
}

enum Payload<'a> {
    Empty,
    Bytes(&'a [u8]),
    Stream(&'a mut dyn Read, u64, Progress<'a>),
}

struct Reply {
    status: u16,
    headers: Vec<(String, String)>,
    body: Body,
}

impl Reply {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    }
}

struct Body {
    from: BufReader<Link>,
    left: Framing,
}

enum Framing {
    Length(u64),
    Chunked(u64, bool),
    ToClose,
}

impl Read for Body {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &mut self.left {
            Framing::Length(0) => Ok(0),
            Framing::Length(n) => {
                let want = buf.len().min(usize::try_from(*n).unwrap_or(usize::MAX));
                let got = self.from.read(&mut buf[..want])?;
                if got == 0 {
                    return Err(io::ErrorKind::UnexpectedEof.into());
                }
                *n -= got as u64;
                Ok(got)
            }
            Framing::ToClose => self.from.read(buf),
            Framing::Chunked(_, true) => Ok(0),
            Framing::Chunked(left, done) => {
                if *left == 0 {
                    let mut raw = Vec::new();
                    read_line_most(&mut self.from, MOST_CHUNK_LINE, &mut raw)?;
                    if raw.iter().all(u8::is_ascii_whitespace) {
                        raw.clear();
                        read_line_most(&mut self.from, MOST_CHUNK_LINE, &mut raw)?;
                    }
                    let line = String::from_utf8_lossy(&raw);
                    let size = line.trim().split(';').next().unwrap_or("");
                    *left = u64::from_str_radix(size, 16).map_err(|_| io::Error::other(t!("folders-remote-dav-chunk")))?;
                    if *left == 0 {
                        *done = true;
                        return Ok(0);
                    }
                }
                let want = buf.len().min(usize::try_from(*left).unwrap_or(usize::MAX));
                let got = self.from.read(&mut buf[..want])?;
                if got == 0 {
                    return Err(io::ErrorKind::UnexpectedEof.into());
                }
                *left -= got as u64;
                Ok(got)
            }
        }
    }
}

impl Dav {
    pub fn connect(address: &Address, to: &Leave<Login>) -> Result<Dav, Error> {
        let login = to.login();
        let at = match to.address() {
            Some(at) => Some(at.parse::<IpAddr>().map_err(|e| Error::Unreachable(t!("folders-remote-dav-not-found", host = address.host.as_str(), why = e.to_string())))?),
            None => None,
        };
        let port = address.port();
        let secure = match address.scheme {
            Scheme::Davs => Some(Secure::new(&address.host, port, login.trust_cert.as_deref())?),
            _ => None,
        };
        let home = if address.path.is_empty() { "/".to_string() } else { address.path.clone() };
        let mut dav = Dav {
            secure,
            host: address.host.clone(),
            at,
            port,
            user: address.user.clone(),
            password: login.password.clone(),
            auth: Auth::None,
            home,
        };
        let at = dav.stat(&dav.home.clone())?;
        if !at.is_folder() {
            return Err(Error::Failed(t!("folders-remote-dav-is-file", path = dav.home.as_str())));
        }
        Ok(dav)
    }

    fn link(&self) -> Result<Link, Error> {
        let addrs: Vec<SocketAddr> = match self.at {
            Some(at) => vec![SocketAddr::new(at, self.port)],
            None => (self.host.as_str(), self.port).to_socket_addrs()
                .map_err(|e| Error::Unreachable(t!("folders-remote-dav-not-found", host = self.host.as_str(), why = e.to_string())))?.collect(),
        };
        let mut last = None;
        for a in addrs {
            match TcpStream::connect_timeout(&a, WAIT) {
                Ok(tcp) => {
                    tcp.set_read_timeout(Some(WAIT))?;
                    tcp.set_write_timeout(Some(WAIT))?;
                    return match &self.secure {
                        Some(s) => s.wrap(tcp),
                        None => Ok(Link::Plain(tcp)),
                    };
                }
                Err(e) => last = Some(e),
            }
        }
        Err(Error::Unreachable(last.map_or_else(|| t!("folders-remote-dav-no-address", host = self.host.as_str()), |e| e.to_string())))
    }

    fn authorization(&mut self, method: &str, target: &str) -> Option<String> {
        let user = self.user.clone().unwrap_or_default();
        let password = self.password.clone()?;
        match &mut self.auth {
            Auth::None => None,
            Auth::Basic => Some(format!("Basic {}", base64(format!("{user}:{password}").as_bytes()))),
            Auth::Digest { realm, nonce, opaque, qop, count } => {
                *count += 1;
                let ha1 = md5_hex(&format!("{user}:{realm}:{password}"));
                let ha2 = md5_hex(&format!("{method}:{target}"));
                let mut cnonce = [0u8; 8];
                let _ = getrandom::fill(&mut cnonce);
                let cnonce: String = cnonce.iter().map(|b| format!("{b:02x}")).collect();
                let nc = format!("{count:08x}");
                let (response, extra) = if *qop {
                    (md5_hex(&format!("{ha1}:{nonce}:{nc}:{cnonce}:auth:{ha2}")), format!(", qop=auth, nc={nc}, cnonce=\"{cnonce}\""))
                } else {
                    (md5_hex(&format!("{ha1}:{nonce}:{ha2}")), String::new())
                };
                let opaque = opaque.as_ref().map_or(String::new(), |o| format!(", opaque=\"{o}\""));
                Some(format!("Digest username=\"{user}\", realm=\"{realm}\", nonce=\"{nonce}\", uri=\"{target}\", response=\"{response}\"{extra}{opaque}"))
            }
        }
    }

    fn send(&mut self, method: &str, path: &str, headers: &[(&str, String)], mut body: Payload) -> Result<Reply, Error> {
        let target = encode(path);
        for attempt in 0..2 {
            let mut link = self.link()?;
            let mut head = format!("{method} {target} HTTP/1.1\r\nHost: {}\r\nUser-Agent: Trindle Folders\r\nConnection: close\r\n", self.host_header());
            for (k, v) in headers {
                head.push_str(&format!("{k}: {v}\r\n"));
            }
            if let Some(auth) = self.authorization(method, &target) {
                head.push_str(&format!("Authorization: {auth}\r\n"));
            }
            let size = match &body {
                Payload::Empty => 0,
                Payload::Bytes(b) => b.len() as u64,
                Payload::Stream(_, n, _) => *n,
            };
            if !matches!(body, Payload::Empty) {
                head.push_str(&format!("Content-Length: {size}\r\n"));
            }
            head.push_str("\r\n");
            link.write_all(head.as_bytes())?;
            match &mut body {
                Payload::Empty => {}
                Payload::Bytes(b) => link.write_all(b)?,
                Payload::Stream(from, n, progress) => {
                    let mut buf = vec![0u8; CHUNK];
                    let mut sent = 0u64;
                    while sent < *n {
                        let want = buf.len().min(usize::try_from(*n - sent).unwrap_or(usize::MAX));
                        let got = from.read(&mut buf[..want])?;
                        if got == 0 {
                            return Err(Error::Failed(t!("folders-remote-dav-file-ended").into()));
                        }
                        link.write_all(&buf[..got])?;
                        sent += got as u64;
                        if !progress(sent) {
                            return Err(Error::Cancelled);
                        }
                    }
                }
            }
            link.flush()?;
            let reply = read_head(link)?;
            if reply.status != 401 {
                return Ok(reply);
            }
            let offered = reply.headers.iter().filter(|(k, _)| k.eq_ignore_ascii_case("www-authenticate")).map(|(_, v)| v.clone()).collect::<Vec<_>>();
            if self.password.is_none() {
                return Err(Error::NeedsPassword);
            }
            let was = self.auth.clone();
            self.auth = choose_auth(&offered).ok_or_else(|| Error::Failed(t!("folders-remote-dav-unknown-login").into()))?;
            let stale = offered.iter().any(|o| o.to_ascii_lowercase().contains("stale=true"));
            if attempt == 1 || (was != Auth::None && !stale) || matches!(body, Payload::Stream(..)) {
                return Err(Error::LoginRefused);
            }
        }
        Err(Error::LoginRefused)
    }

    fn host_header(&self) -> String {
        let usual = if self.secure.is_some() { 443 } else { 80 };
        let host = if self.host.contains(':') { format!("[{}]", self.host) } else { self.host.clone() };
        if self.port == usual { host } else { format!("{host}:{}", self.port) }
    }

    fn propfind(&mut self, path: &str, depth: u8) -> Result<Vec<Listed>, Error> {
        let headers = [("Depth", depth.to_string()), ("Content-Type", "application/xml; charset=utf-8".into())];
        let mut reply = self.send("PROPFIND", path, &headers, Payload::Bytes(PROPFIND.as_bytes()))?;
        check(reply.status, path)?;
        if reply.status != 207 {
            return Err(Error::Failed(t!("folders-remote-dav-no-listing").into()));
        }
        let mut xml = String::new();
        (&mut reply.body).take(MOST_LISTING + 1).read_to_string(&mut xml)?;
        if xml.len() as u64 > MOST_LISTING {
            return Err(too_much());
        }
        parse_listing(&xml)
    }
}

fn choose_auth(offered: &[String]) -> Option<Auth> {
    let digest = offered.iter().find(|o| o.trim_start().to_ascii_lowercase().starts_with("digest"));
    if let Some(d) = digest {
        let field = |name: &str| fields(d).into_iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v);
        let algo = field("algorithm").unwrap_or_else(|| "MD5".into());
        if algo.eq_ignore_ascii_case("MD5") {
            return Some(Auth::Digest {
                realm: field("realm").unwrap_or_default(),
                nonce: field("nonce")?,
                opaque: field("opaque"),
                qop: field("qop").is_some_and(|q| q.split(',').any(|x| x.trim().eq_ignore_ascii_case("auth"))),
                count: 0,
            });
        }
    }
    offered.iter().any(|o| o.trim_start().to_ascii_lowercase().starts_with("basic")).then_some(Auth::Basic)
}

fn fields(challenge: &str) -> Vec<(String, String)> {
    let rest = challenge.trim_start().split_once(' ').map_or("", |(_, r)| r);
    let mut out = Vec::new();
    let mut chars = rest.chars().peekable();
    loop {
        let key: String = chars.by_ref().skip_while(|c| *c == ',' || c.is_whitespace()).take_while(|c| *c != '=').collect();
        if key.is_empty() {
            break;
        }
        let value = if chars.peek() == Some(&'"') {
            chars.next();
            let mut v = String::new();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => v.extend(chars.next()),
                    '"' => break,
                    c => v.push(c),
                }
            }
            v
        } else {
            chars.by_ref().take_while(|c| *c != ',').collect::<String>().trim().to_string()
        };
        out.push((key.trim().to_string(), value));
    }
    out
}

fn read_head(link: Link) -> Result<Reply, Error> {
    let mut from = BufReader::new(link);
    let mut raw = Vec::new();
    read_line_most(&mut from, MOST_LINE, &mut raw)?;
    let mut line = String::from_utf8_lossy(&raw).into_owned();
    let status = line.split_whitespace().nth(1).and_then(|s| s.parse::<u16>().ok())
        .ok_or_else(|| Error::Failed(t!("folders-remote-dav-not-web").into()))?;
    let mut headers = Vec::new();
    let mut used = raw.len();
    loop {
        raw.clear();
        let got = read_line_most(&mut from, MOST_LINE, &mut raw)?;
        used += got;
        if used > MOST_HEAD {
            return Err(too_much());
        }
        line = String::from_utf8_lossy(&raw).into_owned();
        if got == 0 || line.trim().is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    let get = |name: &str| headers.iter().find(|(k, _): &&(String, String)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.clone());
    let left = if get("transfer-encoding").is_some_and(|t| t.to_ascii_lowercase().contains("chunked")) {
        Framing::Chunked(0, false)
    } else if let Some(n) = get("content-length").and_then(|n| n.parse().ok()) {
        Framing::Length(n)
    } else if status == 204 || status == 304 || (100..200).contains(&status) {
        Framing::Length(0)
    } else {
        Framing::ToClose
    };
    Ok(Reply { status, headers, body: Body { from, left } })
}

fn check(status: u16, path: &str) -> Result<(), Error> {
    let name = || name_of(path).to_string();
    match status {
        200..=299 => Ok(()),
        403 => Err(Error::Denied(name())),
        404 | 409 => Err(Error::NotFound(name())),
        405 => Err(Error::Exists(name())),
        412 => Err(Error::Exists(name())),
        507 => Err(Error::Failed(t!("folders-remote-dav-full").into())),
        s => Err(Error::Failed(t!("folders-remote-dav-status", status = s, name = name()))),
    }
}

fn parse_listing(xml: &str) -> Result<Vec<Listed>, Error> {
    let mut reader = Reader::from_str(xml);
    let mut walk = Walk::default();
    loop {
        let ev = reader.read_event().map_err(|e| Error::Failed(t!("folders-remote-dav-listing-unreadable", why = e.to_string())))?;
        match ev {
            Event::Eof => break,
            Event::Start(ref e) | Event::Empty(ref e) => walk.opened(&local_name(e.local_name().as_ref()), matches!(ev, Event::Start(_))),
            Event::Text(ref t) => walk.text.push_str(&t.decode().map(|s| s.to_string()).unwrap_or_default()),
            Event::GeneralRef(ref r) => match r.resolve_char_ref() {
                Ok(Some(c)) => walk.text.push(c),
                _ => walk.text.push_str(match r.decode().as_deref().unwrap_or("") {
                    "amp" => "&",
                    "lt" => "<",
                    "gt" => ">",
                    "quot" => "\"",
                    "apos" => "'",
                    _ => "",
                }),
            },
            Event::End(ref e) => walk.closed(&local_name(e.local_name().as_ref())),
            _ => {}
        }
    }
    Ok(walk.out)
}

fn local_name(name: &[u8]) -> String {
    String::from_utf8_lossy(name).to_ascii_lowercase()
}

#[derive(Default)]
struct Walk {
    href: String,
    folder: bool,
    size: u64,
    modified: Option<i64>,
    ok: bool,
    tag: Option<String>,
    prop: (bool, u64, Option<i64>, Option<String>),
    inside: String,
    text: String,
    out: Vec<Listed>,
}

impl Walk {
    fn opened(&mut self, name: &str, start: bool) {
        match name {
            "response" => {
                (self.href, self.folder, self.size, self.modified, self.ok, self.tag) = (String::new(), false, 0, None, false, None);
            }
            "propstat" => self.prop = (false, 0, None, None),
            "collection" => self.prop.0 = true,
            _ => {}
        }
        if start {
            self.inside = name.to_string();
            self.text.clear();
        }
    }

    fn closed(&mut self, name: &str) {
        let text = self.text.trim();
        match name {
            "href" => self.href = text.to_string(),
            "getcontentlength" => self.prop.1 = text.parse().unwrap_or(0),
            "getlastmodified" => self.prop.2 = http_date(text),
            "getetag" => self.prop.3 = Some(text.to_string()).filter(|t| !t.is_empty()),
            "status" if self.inside == "status" => {
                if self.text.split_whitespace().nth(1) == Some("200") {
                    (self.folder, self.size, self.modified, self.ok) = (self.folder || self.prop.0, self.size.max(self.prop.1), self.modified.or(self.prop.2), true);
                    self.tag = self.tag.take().or(self.prop.3.take());
                }
            }
            "response" if !self.href.is_empty() => {
                let path = href_path(&self.href);
                let name = name_of(path.trim_end_matches('/')).to_string();
                let kind = if !self.ok { Kind::Other } else if self.folder { Kind::Folder } else { Kind::File };
                self.out.push((path, Entry { name, kind, size: self.size, modified: self.modified, mode: None }, self.tag.take()));
            }
            _ => {}
        }
        self.text.clear();
    }
}

fn href_path(href: &str) -> String {
    let path = match href.split_once("://") {
        Some((_, rest)) => rest.find('/').map_or("/", |i| &rest[i..]),
        None => href,
    };
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if let Some(v) = bytes.get(i + 1..i + 3).and_then(|h| std::str::from_utf8(h).ok()).and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn encode(path: &str) -> String {
    let path = if path.is_empty() { "/" } else { path };
    path.bytes().map(|b| match b {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => (b as char).to_string(),
        b => format!("%{b:02X}"),
    }).collect()
}

fn http_date(text: &str) -> Option<i64> {
    const MONTHS: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
    let parts: Vec<&str> = text.split([' ', ',']).filter(|p| !p.is_empty()).collect();
    let (day, month, year, time) = (parts.get(1)?, parts.get(2)?, parts.get(3)?, parts.get(4)?);
    let month = MONTHS.iter().position(|m| month.to_ascii_lowercase().starts_with(m))? as u32 + 1;
    let t: Vec<u32> = time.split(':').filter_map(|n| n.parse().ok()).collect();
    if t.len() != 3 {
        return None;
    }
    Some(unix_time(year.parse().ok()?, month, day.parse().ok()?, t[0], t[1], t[2]))
}

fn md5_hex(text: &str) -> String {
    Md5::digest(text.as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
}

fn base64(bytes: &[u8]) -> String {
    const ABC: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = (u32::from(chunk[0]) << 16) | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8) | u32::from(*chunk.get(2).unwrap_or(&0));
        for k in 0..4 {
            if k <= chunk.len() {
                out.push(ABC[(n >> (18 - 6 * k) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

fn bare(p: &str) -> &str {
    let t = p.trim_end_matches('/');
    if t.is_empty() { "/" } else { t }
}

impl Remote for Dav {
    fn home(&mut self) -> Result<String, Error> {
        Ok(self.home.clone())
    }

    fn list(&mut self, path: &str) -> Result<Vec<Entry>, Error> {
        let want = format!("{}/", bare(path).trim_end_matches('/'));
        let found = self.propfind(&want, 1)?;
        Ok(found.into_iter().filter(|(p, ..)| bare(p) != bare(path)).map(|(_, e, _)| e).collect())
    }

    fn stat(&mut self, path: &str) -> Result<Entry, Error> {
        let found = self.propfind(path, 0)?;
        let (_, mut e, _) = found.into_iter().next().ok_or_else(|| Error::NotFound(name_of(path).to_string()))?;
        e.name = name_of(bare(path)).to_string();
        Ok(e)
    }

    fn read(&mut self, path: &str, to: &mut dyn Write, progress: Progress) -> Result<u64, Error> {
        let mut reply = self.send("GET", path, &[], Payload::Empty)?;
        check(reply.status, path)?;
        let mut buf = vec![0u8; CHUNK];
        let mut done = 0u64;
        loop {
            let n = reply.body.read(&mut buf)?;
            if n == 0 {
                break;
            }
            to.write_all(&buf[..n])?;
            done += n as u64;
            if !progress(done) {
                return Err(Error::Cancelled);
            }
        }
        if let Some(want) = reply.header("content-length").and_then(|n| n.parse::<u64>().ok()) {
            if want != done {
                return Err(Error::Failed(t!("folders-remote-dav-cut-short").into()));
            }
        }
        Ok(done)
    }

    fn write(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        let reply = self.send("PUT", path, &[("Content-Type", "application/octet-stream".into())], Payload::Stream(from, size, progress))?;
        check(reply.status, path)?;
        Ok(size)
    }

    fn write_new(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        let headers = [("Content-Type", "application/octet-stream".to_string()), ("If-None-Match", "*".to_string())];
        let reply = self.send("PUT", path, &headers, Payload::Stream(from, size, progress))?;
        check(reply.status, path)?;
        Ok(size)
    }

    fn refuses_taken(&self) -> bool {
        true
    }

    fn mkdir(&mut self, path: &str) -> Result<(), Error> {
        let reply = self.send("MKCOL", bare(path), &[], Payload::Empty)?;
        check(reply.status, path)
    }

    fn rmdir(&mut self, path: &str) -> Result<(), Error> {
        let want = format!("{}/", bare(path).trim_end_matches('/'));
        let found = self.propfind(&want, 1)?;
        if found.iter().any(|(p, ..)| bare(p) != bare(path)) {
            return Err(Error::Failed(t!("folders-remote-dav-not-empty", name = name_of(bare(path).trim_end_matches('/')))));
        }
        let held = found.iter().find_map(|(_, _, tag)| strong(tag));
        let headers: Vec<(&str, String)> = held.into_iter().map(|tag| ("If-Match", tag)).collect();
        let reply = self.send("DELETE", &want, &headers, Payload::Empty)?;
        if reply.status == 412 && !headers.is_empty() {
            return Err(changed(path));
        }
        check(reply.status, path)
    }

    fn remove(&mut self, path: &str) -> Result<(), Error> {
        let reply = self.send("DELETE", path, &[], Payload::Empty)?;
        check(reply.status, path)
    }

    fn remove_if(&mut self, path: &str, size: u64, modified: Option<i64>) -> Result<bool, Error> {
        let found = self.propfind(path, 0)?;
        let (_, now, tag) = found.into_iter().next().ok_or_else(|| Error::NotFound(name_of(path).to_string()))?;
        if now.kind != Kind::File || now.size != size || now.modified != modified {
            return Ok(false);
        }
        let headers: Vec<(&str, String)> = strong(&tag).into_iter().map(|tag| ("If-Match", tag)).collect();
        let reply = self.send("DELETE", path, &headers, Payload::Empty)?;
        if reply.status == 412 && !headers.is_empty() {
            return Ok(false);
        }
        check(reply.status, path).map(|()| true)
    }

    fn rename(&mut self, from: &str, to: &str) -> Result<(), Error> {
        let scheme = if self.secure.is_some() { "https" } else { "http" };
        let destination = format!("{scheme}://{}{}", self.host_header(), encode(to));
        let reply = self.send("MOVE", from, &[("Destination", destination), ("Overwrite", "F".into())], Payload::Empty)?;
        check(reply.status, to)
    }
}

#[cfg(test)]
mod tests;
