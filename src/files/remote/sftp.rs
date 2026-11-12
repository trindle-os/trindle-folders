//! SFTP, version 3 (the one OpenSSH speaks), in Trindle's own Rust. The

use std::collections::{BTreeMap, HashMap};
use std::io::{self, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

use super::{Address, Entry, Error, Kind, Login, Progress, Remote};
use crate::toolkit::t;

const VERSION: u32 = 3;

const INIT: u8 = 1;
const VERSION_REPLY: u8 = 2;
const OPEN: u8 = 3;
const CLOSE: u8 = 4;
const READ: u8 = 5;
const WRITE: u8 = 6;
const LSTAT: u8 = 7;
const OPENDIR: u8 = 11;
const READDIR: u8 = 12;
const REMOVE: u8 = 13;
const MKDIR: u8 = 14;
const RMDIR: u8 = 15;
const REALPATH: u8 = 16;
const STAT: u8 = 17;
const RENAME: u8 = 18;
const READLINK: u8 = 19;
const STATUS: u8 = 101;
const HANDLE: u8 = 102;
const DATA: u8 = 103;
const NAME: u8 = 104;
const ATTRS: u8 = 105;

const FX_OK: u32 = 0;
const FX_EOF: u32 = 1;
const FX_NO_SUCH_FILE: u32 = 2;
const FX_PERMISSION_DENIED: u32 = 3;
const FX_OP_UNSUPPORTED: u32 = 8;

const OPEN_READ: u32 = 0x01;
const OPEN_WRITE: u32 = 0x02;
const OPEN_CREAT: u32 = 0x08;
const OPEN_EXCL: u32 = 0x20;

const ATTR_SIZE: u32 = 0x01;
const ATTR_UIDGID: u32 = 0x02;
const ATTR_PERMISSIONS: u32 = 0x04;
const ATTR_ACMODTIME: u32 = 0x08;
const ATTR_EXTENDED: u32 = 0x8000_0000;

const CHUNK: u32 = 32 * 1024;
const WINDOW: usize = 32;
const IN_FLIGHT: usize = 64;
const WINDOW_LEAST: usize = 8;
static GOING: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

struct Going;

impl Going {
    fn new() -> Going {
        GOING.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Going
    }
}

impl Drop for Going {
    fn drop(&mut self) {
        GOING.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}

fn window() -> usize {
    share_of(GOING.load(std::sync::atomic::Ordering::Relaxed))
}

fn share_of(going: usize) -> usize {
    (IN_FLIGHT / going.max(1)).clamp(WINDOW_LEAST, WINDOW)
}
const LARGEST: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attrs {
    pub size: Option<u64>,
    pub owner: Option<(u32, u32)>,
    pub mode: Option<u32>,
    pub times: Option<(u32, u32)>,
}

impl Attrs {
    fn kind(&self) -> Kind {
        match self.mode.map(|m| m & 0o170_000) {
            Some(0o040_000) => Kind::Folder,
            Some(0o100_000) => Kind::File,
            Some(0o120_000) => Kind::Link,
            Some(_) => Kind::Other,
            None => Kind::File,
        }
    }

    fn entry(&self, name: String) -> Entry {
        Entry {
            name,
            kind: self.kind(),
            size: self.size.unwrap_or(0),
            modified: self.times.map(|(_, m)| m as i64),
            mode: self.mode.map(|m| m & 0o7777),
        }
    }
}

pub(super) struct Out(Vec<u8>);

impl Out {
    pub(super) fn new(kind: u8, id: Option<u32>) -> Out {
        let mut out = Out(vec![0, 0, 0, 0, kind]);
        if let Some(id) = id {
            out.u32(id);
        }
        out
    }

    pub(super) fn u32(&mut self, v: u32) -> &mut Out {
        self.0.extend_from_slice(&v.to_be_bytes());
        self
    }

    pub(super) fn u64(&mut self, v: u64) -> &mut Out {
        self.0.extend_from_slice(&v.to_be_bytes());
        self
    }

    pub(super) fn bytes(&mut self, v: &[u8]) -> &mut Out {
        self.u32(v.len() as u32);
        self.0.extend_from_slice(v);
        self
    }

    pub(super) fn str(&mut self, v: &str) -> &mut Out {
        self.bytes(v.as_bytes())
    }

    pub(super) fn attrs(&mut self, a: &Attrs) -> &mut Out {
        let mut flags = 0;
        for (on, bit) in [
            (a.size.is_some(), ATTR_SIZE),
            (a.owner.is_some(), ATTR_UIDGID),
            (a.mode.is_some(), ATTR_PERMISSIONS),
            (a.times.is_some(), ATTR_ACMODTIME),
        ] {
            if on {
                flags |= bit;
            }
        }
        self.u32(flags);
        if let Some(s) = a.size {
            self.u64(s);
        }
        if let Some((u, g)) = a.owner {
            self.u32(u).u32(g);
        }
        if let Some(m) = a.mode {
            self.u32(m);
        }
        if let Some((at, mt)) = a.times {
            self.u32(at).u32(mt);
        }
        self
    }

    pub(super) fn done(&mut self) -> Vec<u8> {
        let mut v = std::mem::take(&mut self.0);
        let len = (v.len() - 4) as u32;
        v[..4].copy_from_slice(&len.to_be_bytes());
        v
    }
}

pub(super) struct In<'a>(&'a [u8]);

const MOST_NAMES: usize = if cfg!(test) { 3000 } else { 1_000_000 };

fn garbled() -> Error {
    Error::Lost(t!("folders-remote-sftp-nonsense").into())
}

impl<'a> In<'a> {
    pub(super) fn new(body: &'a [u8]) -> In<'a> {
        In(body)
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        if self.0.len() < n {
            return Err(garbled());
        }
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        Ok(a)
    }

    pub(super) fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }

    pub(super) fn u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().unwrap_or_default()))
    }

    pub(super) fn u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_be_bytes(self.take(8)?.try_into().unwrap_or_default()))
    }

    pub(super) fn bytes(&mut self) -> Result<&'a [u8], Error> {
        let n = self.u32()? as usize;
        self.take(n)
    }

    pub(super) fn string(&mut self) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(self.bytes()?).into_owned())
    }

    pub(super) fn attrs(&mut self) -> Result<Attrs, Error> {
        let flags = self.u32()?;
        let mut a = Attrs::default();
        if flags & ATTR_SIZE != 0 {
            a.size = Some(self.u64()?);
        }
        if flags & ATTR_UIDGID != 0 {
            a.owner = Some((self.u32()?, self.u32()?));
        }
        if flags & ATTR_PERMISSIONS != 0 {
            a.mode = Some(self.u32()?);
        }
        if flags & ATTR_ACMODTIME != 0 {
            a.times = Some((self.u32()?, self.u32()?));
        }
        if flags & ATTR_EXTENDED != 0 {
            for _ in 0..self.u32()? {
                self.bytes()?;
                self.bytes()?;
            }
        }
        Ok(a)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Reply {
    Status(u32, String),
    Handle(Vec<u8>),
    Data(Vec<u8>),
    Name(Vec<(String, Attrs)>),
    Attrs(Attrs),
}

pub(super) fn decode(body: &[u8]) -> Result<(u32, Reply), Error> {
    let mut i = In::new(body);
    let kind = i.u8()?;
    let id = i.u32()?;
    let reply = match kind {
        STATUS => {
            let code = i.u32()?;
            let msg = if i.0.is_empty() { String::new() } else { i.string()? };
            Reply::Status(code, msg)
        }
        HANDLE => Reply::Handle(i.bytes()?.to_vec()),
        DATA => Reply::Data(i.bytes()?.to_vec()),
        NAME => {
            let n = i.u32()?;
            let mut names = Vec::with_capacity(n.min(1024) as usize);
            for _ in 0..n {
                let name = i.string()?;
                i.bytes()?;
                names.push((name, i.attrs()?));
            }
            Reply::Name(names)
        }
        ATTRS => Reply::Attrs(i.attrs()?),
        _ => return Err(garbled()),
    };
    Ok((id, reply))
}

#[derive(Default)]
pub(super) struct Pieces {
    written: u64,
    asked: u64,
    eof: Option<u64>,
    again: Vec<(u64, u32)>,
    held: BTreeMap<u64, Vec<u8>>,
}

impl Pieces {
    pub(super) fn next(&mut self) -> Option<(u64, u32)> {
        let eof = self.eof;
        let before_end = |at: u64| eof.is_none_or(|e| at < e);
        while let Some(piece) = self.again.pop() {
            if before_end(piece.0) {
                return Some(piece);
            }
        }
        if !before_end(self.asked) {
            return None;
        }
        let piece = (self.asked, CHUNK);
        self.asked += CHUNK as u64;
        Some(piece)
    }

    pub(super) fn came(&mut self, at: u64, asked: u32, data: Option<Vec<u8>>) -> Result<(), Error> {
        match data {
            Some(d) if d.len() as u64 > asked as u64 => return Err(garbled()),
            Some(d) if !d.is_empty() => {
                if (d.len() as u64) < asked as u64 {
                    self.again.push((at + d.len() as u64, asked - d.len() as u32));
                }
                self.held.insert(at, d);
            }
            _ => self.eof = Some(self.eof.map_or(at, |e| e.min(at))),
        }
        Ok(())
    }

    pub(super) fn ready(&mut self) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        while let Some(d) = self.held.remove(&self.written) {
            self.written += d.len() as u64;
            out.push(d);
        }
        out
    }

    pub(super) fn written(&self) -> u64 {
        self.written
    }
}

pub struct Sftp {
    child: Option<Child>,
    to: Box<dyn Write + Send>,
    from: Box<dyn Read + Send>,
    id: u32,
}

impl Sftp {
    #[cfg_attr(test, allow(dead_code))]
    pub fn connect(address: &Address, login: &Login) -> Result<Sftp, Error> {
        let args = ssh_args(address, login)?;
        let mut cmd = Command::new("ssh");
        cmd.args(args);
        if let Some(ask) = &login.askpass {
            cmd.env("SSH_ASKPASS", ask).env("SSH_ASKPASS_REQUIRE", "force");
        }
        cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = cmd.spawn().map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => Error::Failed(t!("folders-remote-sftp-no-ssh").into()),
            _ => Error::Io(e),
        })?;
        let said = Arc::new(Mutex::new(String::new()));
        let reader = child.stderr.take().map(|mut err| {
            let said = said.clone();
            std::thread::spawn(move || {
                let mut buf = [0u8; 1024];
                while let Ok(n) = err.read(&mut buf) {
                    if n == 0 {
                        break;
                    }
                    if let Ok(mut s) = said.lock() {
                        if s.len() < 64 * 1024 {
                            s.push_str(&String::from_utf8_lossy(&buf[..n]));
                        }
                    }
                }
            })
        });
        let (to, from) = match (child.stdin.take(), child.stdout.take()) {
            (Some(i), Some(o)) => (i, o),
            _ => return Err(Error::Failed(t!("folders-remote-sftp-ssh-failed").into())),
        };
        let mut sftp = Sftp { child: None, to: Box::new(to), from: Box::new(from), id: 0 };
        match sftp.init() {
            Ok(()) => {
                sftp.child = Some(child);
                Ok(sftp)
            }
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                let until = std::time::Instant::now() + std::time::Duration::from_secs(2);
                while reader.as_ref().is_some_and(|r| !r.is_finished()) && std::time::Instant::now() < until {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
                let said = said.lock().map(|s| s.clone()).unwrap_or_default();
                Err(why(&said, &address.host, login).unwrap_or(e))
            }
        }
    }

    #[cfg(test)]
    fn over(from: impl Read + Send + 'static, to: impl Write + Send + 'static) -> Result<Sftp, Error> {
        let mut s = Sftp { child: None, to: Box::new(to), from: Box::new(from), id: 0 };
        s.init()?;
        Ok(s)
    }

    fn init(&mut self) -> Result<(), Error> {
        self.send(&Out::new(INIT, None).u32(VERSION).done())?;
        let body = self.recv()?;
        let mut i = In::new(&body);
        if i.u8()? != VERSION_REPLY {
            return Err(garbled());
        }
        if i.u32()? < VERSION {
            return Err(Error::NotSupported(t!("folders-remote-sftp-old").into()));
        }
        Ok(())
    }

    fn send(&mut self, packet: &[u8]) -> Result<(), Error> {
        self.to.write_all(packet).map_err(lost)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.to.flush().map_err(lost)
    }

    fn recv(&mut self) -> Result<Vec<u8>, Error> {
        self.flush()?;
        let mut len = [0u8; 4];
        self.from.read_exact(&mut len).map_err(lost)?;
        let len = u32::from_be_bytes(len) as usize;
        if len == 0 || len > LARGEST {
            return Err(garbled());
        }
        let mut body = vec![0; len];
        self.from.read_exact(&mut body).map_err(lost)?;
        Ok(body)
    }

    fn next_id(&mut self) -> u32 {
        self.id = self.id.wrapping_add(1);
        self.id
    }

    fn ask(&mut self, kind: u8, fill: impl FnOnce(&mut Out)) -> Result<Reply, Error> {
        let id = self.next_id();
        let mut out = Out::new(kind, Some(id));
        fill(&mut out);
        self.send(&out.done())?;
        let (got, reply) = decode(&self.recv()?)?;
        if got != id {
            return Err(garbled());
        }
        Ok(reply)
    }

    fn ok(&mut self, kind: u8, path: &str, fill: impl FnOnce(&mut Out)) -> Result<(), Error> {
        match self.ask(kind, fill)? {
            Reply::Status(FX_OK, _) => Ok(()),
            Reply::Status(code, msg) => Err(status(code, &msg, path)),
            _ => Err(garbled()),
        }
    }

    fn handle(&mut self, kind: u8, path: &str, fill: impl FnOnce(&mut Out)) -> Result<Vec<u8>, Error> {
        match self.ask(kind, fill)? {
            Reply::Handle(h) => Ok(h),
            Reply::Status(code, msg) => Err(status(code, &msg, path)),
            _ => Err(garbled()),
        }
    }

    fn close(&mut self, handle: &[u8]) -> Result<(), Error> {
        self.ok(CLOSE, "", |o| {
            o.bytes(handle);
        })
    }

    fn attrs(&mut self, kind: u8, path: &str) -> Result<Attrs, Error> {
        match self.ask(kind, |o| {
            o.str(path);
        })? {
            Reply::Attrs(a) => Ok(a),
            Reply::Status(code, msg) => Err(status(code, &msg, path)),
            _ => Err(garbled()),
        }
    }

    fn one_name(&mut self, kind: u8, path: &str) -> Result<String, Error> {
        match self.ask(kind, |o| {
            o.str(path);
        })? {
            Reply::Name(mut n) if !n.is_empty() => Ok(n.swap_remove(0).0),
            Reply::Status(code, msg) => Err(status(code, &msg, path)),
            _ => Err(garbled()),
        }
    }

    fn drain(&mut self, waiting: &mut HashMap<u32, (u64, u32)>) {
        while !waiting.is_empty() {
            match self.recv().and_then(|b| decode(&b)) {
                Ok((id, _)) => {
                    waiting.remove(&id);
                }
                Err(_) => break,
            }
        }
    }
}

impl Drop for Sftp {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[derive(Default)]
struct Flight {
    waiting: HashMap<u32, (u64, u32)>,
    sent: u64,
    done: u64,
    failed: Option<Error>,
    ended: bool,
}

impl Sftp {
    fn write_as(&mut self, path: &str, from: &mut dyn Read, _size: u64, progress: Progress, flags: u32) -> Result<u64, Error> {
        let handle = self.handle(OPEN, path, |o| {
            o.str(path).u32(flags).attrs(&Attrs::default());
        })?;
        let _going = Going::new();
        let mut flight = Flight::default();
        let mut buf = vec![0u8; CHUNK as usize];
        loop {
            self.send_window(&handle, from, &mut buf, &mut flight);
            if flight.waiting.is_empty() || flight.failed.is_some() {
                break;
            }
            self.take_status(&mut flight, path);
            if flight.failed.is_none() && !progress(flight.done) {
                flight.failed = Some(Error::Cancelled);
            }
        }
        self.drain(&mut flight.waiting);
        let closed = self.close(&handle);
        if let Some(e) = flight.failed {
            let _ = self.remove(path);
            return Err(e);
        }
        closed?;
        Ok(flight.done)
    }

    fn send_window(&mut self, handle: &[u8], from: &mut dyn Read, buf: &mut [u8], flight: &mut Flight) {
        while flight.failed.is_none() && !flight.ended && flight.waiting.len() < window() {
            let n = match read_full(from, buf) {
                Ok(0) => {
                    flight.ended = true;
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    flight.failed = Some(Error::Io(e));
                    break;
                }
            };
            let id = self.next_id();
            let mut out = Out::new(WRITE, Some(id));
            out.bytes(handle).u64(flight.sent).bytes(&buf[..n]);
            if let Err(e) = self.send(&out.done()) {
                flight.failed = Some(e);
                break;
            }
            flight.waiting.insert(id, (flight.sent, n as u32));
            flight.sent += n as u64;
        }
    }

    fn take_status(&mut self, flight: &mut Flight, path: &str) {
        match self.recv().and_then(|b| decode(&b)) {
            Ok((id, Reply::Status(FX_OK, _))) => match flight.waiting.remove(&id) {
                Some((_, n)) => flight.done += n as u64,
                None => flight.failed = Some(garbled()),
            },
            Ok((id, Reply::Status(code, msg))) => {
                flight.waiting.remove(&id);
                flight.failed = Some(status(code, &msg, path));
            }
            Ok(_) => flight.failed = Some(garbled()),
            Err(e) => flight.failed = Some(e),
        }
    }
}

impl Remote for Sftp {
    fn home(&mut self) -> Result<String, Error> {
        self.one_name(REALPATH, ".")
    }

    fn list(&mut self, path: &str) -> Result<Vec<Entry>, Error> {
        let handle = self.handle(OPENDIR, path, |o| {
            o.str(path);
        })?;
        let mut out = Vec::new();
        let result = loop {
            let h = handle.clone();
            match self.ask(READDIR, |o| {
                o.bytes(&h);
            }) {
                Ok(Reply::Name(_)) if out.len() >= MOST_NAMES => break Err(super::too_much()),
                Ok(Reply::Name(names)) => {
                    for (name, a) in names {
                        if name != "." && name != ".." {
                            out.push(a.entry(name));
                        }
                    }
                }
                Ok(Reply::Status(FX_EOF, _)) => break Ok(()),
                Ok(Reply::Status(code, msg)) => break Err(status(code, &msg, path)),
                Ok(_) => break Err(garbled()),
                Err(e) => break Err(e),
            }
        };
        let closed = self.close(&handle);
        result.and(closed)?;
        Ok(out)
    }

    fn stat(&mut self, path: &str) -> Result<Entry, Error> {
        let a = self.attrs(LSTAT, path)?;
        Ok(a.entry(super::name_of(path).to_string()))
    }

    fn read(&mut self, path: &str, to: &mut dyn Write, progress: Progress) -> Result<u64, Error> {
        let handle = self.handle(OPEN, path, |o| {
            o.str(path).u32(OPEN_READ).attrs(&Attrs::default());
        })?;
        let _going = Going::new();
        let mut pieces = Pieces::default();
        let mut waiting: HashMap<u32, (u64, u32)> = HashMap::new();
        let mut failed = None;
        loop {
            while failed.is_none() && waiting.len() < window() {
                let Some((at, len)) = pieces.next() else { break };
                let id = self.next_id();
                let mut out = Out::new(READ, Some(id));
                out.bytes(&handle).u64(at).u32(len);
                if let Err(e) = self.send(&out.done()) {
                    failed = Some(e);
                    break;
                }
                waiting.insert(id, (at, len));
            }
            if waiting.is_empty() || failed.is_some() {
                break;
            }
            let (id, reply) = match self.recv().and_then(|b| decode(&b)) {
                Ok(r) => r,
                Err(e) => {
                    failed = Some(e);
                    break;
                }
            };
            let Some((at, len)) = waiting.remove(&id) else {
                failed = Some(garbled());
                break;
            };
            match reply {
                Reply::Data(d) => failed = pieces.came(at, len, Some(d)).err(),
                Reply::Status(FX_EOF, _) => failed = pieces.came(at, len, None).err(),
                Reply::Status(code, msg) => failed = Some(status(code, &msg, path)),
                _ => failed = Some(garbled()),
            }
            for d in pieces.ready() {
                if let Err(e) = to.write_all(&d) {
                    failed = Some(Error::Io(e));
                    break;
                }
            }
            if failed.is_none() && !progress(pieces.written()) {
                failed = Some(Error::Cancelled);
            }
        }
        self.drain(&mut waiting);
        let closed = self.close(&handle);
        if let Some(e) = failed {
            return Err(e);
        }
        closed?;
        Ok(pieces.written())
    }

    fn write_new(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        match self.write_as(path, from, size, progress, OPEN_WRITE | OPEN_CREAT | OPEN_EXCL) {
            Err(Error::Failed(_) | Error::Denied(_)) if self.stat(path).is_ok() => Err(Error::Exists(super::name_of(path).to_string())),
            other => other,
        }
    }

    fn refuses_taken(&self) -> bool {
        true
    }

    fn mkdir(&mut self, path: &str) -> Result<(), Error> {
        self.ok(MKDIR, path, |o| {
            o.str(path).attrs(&Attrs::default());
        })
    }

    fn rmdir(&mut self, path: &str) -> Result<(), Error> {
        self.ok(RMDIR, path, |o| {
            o.str(path);
        })
    }

    fn remove(&mut self, path: &str) -> Result<(), Error> {
        self.ok(REMOVE, path, |o| {
            o.str(path);
        })
    }

    fn rename(&mut self, from: &str, to: &str) -> Result<(), Error> {
        self.ok(RENAME, from, |o| {
            o.str(from).str(to);
        })
    }

    fn readlink(&mut self, path: &str) -> Result<String, Error> {
        self.one_name(READLINK, path)
    }

    fn follow(&mut self, path: &str) -> Result<Entry, Error> {
        let a = self.attrs(STAT, path)?;
        Ok(a.entry(super::name_of(path).to_string()))
    }
}

fn read_full(from: &mut dyn Read, buf: &mut [u8]) -> io::Result<usize> {
    let mut n = 0;
    while n < buf.len() {
        match from.read(&mut buf[n..]) {
            Ok(0) => break,
            Ok(k) => n += k,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(n)
}

fn lost(e: io::Error) -> Error {
    match e.kind() {
        io::ErrorKind::UnexpectedEof | io::ErrorKind::BrokenPipe => {
            Error::Lost(t!("folders-remote-sftp-lost").into())
        }
        _ => Error::Io(e),
    }
}

fn status(code: u32, msg: &str, path: &str) -> Error {
    match code {
        FX_NO_SUCH_FILE => Error::NotFound(path.into()),
        FX_PERMISSION_DENIED => Error::Denied(path.into()),
        FX_OP_UNSUPPORTED => Error::NotSupported(t!("folders-remote-sftp-that-here").into()),
        _ if msg.is_empty() => Error::Failed(t!("folders-remote-sftp-could-not", path = path)),
        _ => Error::Failed(t!("folders-remote-sftp-could-not-said", path = path, said = msg)),
    }
}

fn ssh_args(address: &Address, login: &Login) -> Result<Vec<String>, Error> {
    let mut args: Vec<String> = ["-T", "-x", "-a"].iter().map(|s| s.to_string()).collect();
    #[cfg(test)]
    if let Ok(more) = std::env::var("TRINDLE_TEST_SSH") {
        args.extend(more.split(' ').map(str::to_string));
    }
    let mut opt = |o: String| {
        args.push("-o".into());
        args.push(o);
    };
    opt("ClearAllForwardings=yes".into());
    opt("ConnectTimeout=20".into());
    opt("ServerAliveInterval=15".into());
    opt("UpdateHostKeys=no".into());
    opt(format!("StrictHostKeyChecking={}", if login.trust_new_host { "accept-new" } else { "yes" }));
    if login.askpass.is_none() {
        opt("BatchMode=yes".into());
    }
    if let Some(socket) = share_path() {
        opt("ControlMaster=auto".into());
        opt(format!("ControlPath={socket}"));
        opt("ControlPersist=60".into());
    }
    if let Some(key) = &login.key {
        if !key.is_file() {
            return Err(Error::Key(t!("folders-remote-sftp-no-key", key = key.display().to_string())));
        }
        opt("IdentitiesOnly=yes".into());
        args.push("-i".into());
        args.push(key.display().to_string());
    }
    args.push("-p".into());
    args.push(address.port().to_string());
    if let Some(user) = &address.user {
        args.push("-l".into());
        args.push(user.clone());
    }
    args.extend(["-s", "--", &address.host, "sftp"].iter().map(|s| s.to_string()));
    Ok(args)
}

fn share_path() -> Option<String> {
    let run = std::env::var_os("XDG_RUNTIME_DIR")?;
    let run = std::path::Path::new(&run);
    run.is_dir().then(|| run.join("trindle-ssh-%C").display().to_string())
}

fn why(said: &str, host: &str, login: &Login) -> Option<Error> {
    let has = |s: &str| said.contains(s);
    Some(if has("REMOTE HOST IDENTIFICATION HAS CHANGED") || has("Host key for") && has("has changed") {
        Error::HostKeyChanged(host.into())
    } else if has("Host key verification failed") || has("No ") && has("host key is known") {
        Error::HostKeyUnknown(host.into())
    } else if has("Permission denied") || has("Too many authentication failures") {
        if login.key.is_some() || login.askpass.is_some() {
            Error::LoginRefused
        } else {
            Error::NeedsPassword
        }
    } else if has("Could not resolve hostname") {
        Error::Unreachable(t!("folders-remote-sftp-no-server", host = host))
    } else if has("Connection refused") {
        Error::Unreachable(t!("folders-remote-sftp-turned-away", host = host))
    } else if has("timed out") {
        Error::Unreachable(t!("folders-remote-sftp-no-answer", host = host))
    } else if has("No route to host") || has("Network is unreachable") {
        Error::Unreachable(t!("folders-remote-sftp-no-way", host = host))
    } else if has("subsystem request failed") {
        Error::Failed(t!("folders-remote-sftp-none-here").into())
    } else {
        let last = said.lines().map(str::trim).rfind(|l| !l.is_empty())?;
        Error::Failed(t!("folders-remote-sftp-ssh-said", said = last))
    })
}

#[cfg(test)]
mod tests;
