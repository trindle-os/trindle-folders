//! MTP, the way phones and cameras share their files over USB, in Trindle's

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;

use super::usb::{self, Bulk, Found, Usb};
use super::{join, name_of, now, parent, too_much, unix_time, Entry, Error, Kind, Progress, Remote};
use crate::toolkit::t;

const COMMAND: u16 = 1;
const DATA: u16 = 2;
const RESPONSE: u16 = 3;

const OPEN_SESSION: u16 = 0x1002;
const CLOSE_SESSION: u16 = 0x1003;
const GET_STORAGE_IDS: u16 = 0x1004;
const GET_STORAGE_INFO: u16 = 0x1005;
const GET_OBJECT_HANDLES: u16 = 0x1007;
const GET_OBJECT_INFO: u16 = 0x1008;
const GET_OBJECT: u16 = 0x1009;
const DELETE_OBJECT: u16 = 0x100B;
const SEND_OBJECT_INFO: u16 = 0x100C;
const SEND_OBJECT: u16 = 0x100D;
const MOVE_OBJECT: u16 = 0x1019;
const GET_OBJECT_PROP_VALUE: u16 = 0x9803;
const SET_OBJECT_PROP_VALUE: u16 = 0x9804;

const OK: u16 = 0x2001;
const SESSION_ALREADY_OPEN: u16 = 0x201E;

const UNDEFINED: u16 = 0x3000;
const ASSOCIATION: u16 = 0x3001;
const PROP_SIZE: u16 = 0xDC04;
const PROP_NAME: u16 = 0xDC07;

const ROOT: u32 = 0xFFFF_FFFF;
const CHUNK: usize = 256 * 1024;
const MOST_RESPONSE: usize = 4096;
const MOST_ANSWER: usize = 64 << 20;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phone {
    pub id: String,
    pub name: String,
}

pub fn phones() -> Vec<Phone> {
    found(Path::new(usb::SYSFS)).into_iter().map(|(p, _)| p).collect()
}

fn found(root: &Path) -> Vec<(Phone, Found)> {
    usb::find(root).into_iter().map(|f| (phone(&f), f)).collect()
}

fn phone(f: &Found) -> Phone {
    let ok = !f.serial.is_empty()
        && !f.serial.starts_with('-')
        && f.serial.chars().all(|c| c.is_ascii_graphic() && !"/@:?#%[]\\".contains(c));
    let id = if ok { f.serial.clone() } else { f.port.clone() };
    let (maker, product) = (f.maker.trim(), f.product.trim());
    let name = if product.is_empty() {
        if maker.is_empty() { t!("folders-remote-mtp-phone").to_string() } else { maker.to_string() }
    } else if maker.is_empty() || product.to_lowercase().starts_with(&maker.to_lowercase()) {
        product.to_string()
    } else {
        format!("{maker} {product}")
    };
    Phone { id, name }
}

fn nonsense() -> Error {
    Error::Lost(t!("folders-remote-mtp-nonsense").into())
}

#[derive(Default)]
struct Put(Vec<u8>);

impl Put {
    fn u16(&mut self, v: u16) -> &mut Put {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn u32(&mut self, v: u32) -> &mut Put {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn str(&mut self, s: &str) -> &mut Put {
        let units: Vec<u16> = s.encode_utf16().take(254).collect();
        if units.is_empty() {
            self.0.push(0);
            return self;
        }
        self.0.push(units.len() as u8 + 1);
        for u in units {
            self.u16(u);
        }
        self.u16(0)
    }
}

struct Take<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Take<'a> {
    fn new(b: &'a [u8]) -> Take<'a> {
        Take { b, at: 0 }
    }
    fn bytes(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let s = self.b.get(self.at..self.at + n).ok_or_else(nonsense)?;
        self.at += n;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.bytes(1)?[0])
    }
    fn u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_le_bytes(self.bytes(2)?.try_into().map_err(|_| nonsense())?))
    }
    fn u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.bytes(4)?.try_into().map_err(|_| nonsense())?))
    }
    fn u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.bytes(8)?.try_into().map_err(|_| nonsense())?))
    }
    fn str(&mut self) -> Result<String, Error> {
        let n = self.u8()? as usize;
        let units: Vec<u16> = (0..n).map(|_| self.u16()).collect::<Result<_, _>>()?;
        let end = units.iter().position(|&u| u == 0).unwrap_or(units.len());
        Ok(String::from_utf16_lossy(&units[..end]))
    }
    fn array(&mut self) -> Result<Vec<u32>, Error> {
        let n = self.u32()? as usize;
        if n > (self.b.len() - self.at) / 4 {
            return Err(nonsense());
        }
        (0..n).map(|_| self.u32()).collect()
    }
}

fn container(kind: u16, code: u16, id: u32, params: &[u32]) -> Vec<u8> {
    let mut p = Put::default();
    p.u32(12 + 4 * params.len() as u32).u16(kind).u16(code).u32(id);
    for &v in params {
        p.u32(v);
    }
    p.0
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ObjectInfo {
    storage: u32,
    format: u16,
    size: u64,
    parent: u32,
    name: String,
    modified: Option<i64>,
}

impl ObjectInfo {
    fn folder(&self) -> bool {
        self.format == ASSOCIATION
    }

    fn encode(&self) -> Vec<u8> {
        let mut p = Put::default();
        p.u32(self.storage).u16(self.format).u16(0).u32(u32::try_from(self.size).unwrap_or(u32::MAX));
        p.u16(0).u32(0).u32(0).u32(0).u32(0).u32(0).u32(0);
        p.u32(self.parent);
        p.u16(if self.folder() { 1 } else { 0 }).u32(0).u32(0);
        p.str(&self.name).str("").str(&self.modified.map(date).unwrap_or_default()).str("");
        p.0
    }

    fn decode(b: &[u8]) -> Result<ObjectInfo, Error> {
        let mut t = Take::new(b);
        let storage = t.u32()?;
        let format = t.u16()?;
        t.u16()?;
        let size = t.u32()? as u64;
        t.bytes(2 + 4 * 6)?;
        let parent = t.u32()?;
        t.bytes(2 + 4 + 4)?;
        let name = t.str()?;
        let _created = t.str()?;
        let modified = undate(&t.str()?);
        Ok(ObjectInfo { storage, format, size, parent, name, modified })
    }
}

fn date(t: i64) -> String {
    let (y, mo, d) = super::civil(t);
    let s = t.rem_euclid(86_400);
    format!("{y:04}{mo:02}{d:02}T{:02}{:02}{:02}", s / 3600, s / 60 % 60, s % 60)
}

fn undate(s: &str) -> Option<i64> {
    let b = s.as_bytes();
    if b.len() < 15 || b[8] != b'T' {
        return None;
    }
    let n = |a: usize, z: usize| s.get(a..z)?.parse::<u32>().ok();
    let t = unix_time(n(0, 4)? as i64, n(4, 6)?, n(6, 8)?, n(9, 11)?, n(11, 13)?, n(13, 15)?);
    Some(t)
}

fn refused(code: u16, what: &str) -> Error {
    match code {
        0x2008 | 0x2009 | 0x2013 => Error::NotFound(what.into()),
        0x200C => Error::Failed(t!("folders-remote-mtp-full").into()),
        0x200D..=0x200F => Error::Denied(what.into()),
        0x2019 => Error::Busy(t!("folders-remote-mtp-the-phone").into()),
        0x2005 => Error::NotSupported(t!("folders-remote-mtp-that-here").into()),
        0x201F => Error::Cancelled,
        _ => Error::Failed(t!("folders-remote-mtp-refused", code = format!("{code:04X}"))),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Node {
    storage: u32,
    handle: u32,
    folder: bool,
}

struct Header {
    len: u32,
    kind: u16,
    code: u16,
}

type Sink<'a> = &'a mut dyn FnMut(&[u8]) -> Result<bool, Error>;

pub struct Mtp {
    dev: Box<dyn Bulk>,
    id: u32,
    left: Vec<u8>,
    buf: Vec<u8>,
    last: u16,
    storages: Option<Vec<(u32, String)>>,
    known: HashMap<String, Node>,
}

impl Mtp {
    pub fn open(id: &str) -> Result<Mtp, Error> {
        let all = found(Path::new(usb::SYSFS));
        let (_, f) = all
            .into_iter()
            .find(|(p, _)| p.id == id)
            .ok_or_else(|| Error::Unreachable(t!("folders-remote-mtp-not-plugged").into()))?;
        Mtp::over(Box::new(Usb::open(&f)?))
    }

    fn over(dev: Box<dyn Bulk>) -> Result<Mtp, Error> {
        let mut m = Mtp {
            dev,
            id: 0,
            left: Vec::new(),
            buf: vec![0; CHUNK],
            last: 0,
            storages: None,
            known: HashMap::new(),
        };
        for _ in 0..2 {
            m.id = 0;
            m.left.clear();
            match m.run(OPEN_SESSION, &[1], None, t!("folders-remote-mtp-what-phone")) {
                Ok(_) => return Ok(m),
                Err(Error::Failed(_)) if m.last == SESSION_ALREADY_OPEN => {
                    let _ = m.run(CLOSE_SESSION, &[], None, t!("folders-remote-mtp-what-phone"));
                }
                Err(e) => return Err(e),
            }
        }
        Err(Error::Busy(t!("folders-remote-mtp-the-phone").into()))
    }

    fn more(&mut self) -> Result<usize, Error> {
        let n = self.dev.recv(&mut self.buf)?;
        self.left.extend_from_slice(&self.buf[..n]);
        Ok(n)
    }

    fn header(&mut self) -> Result<(Header, bool), Error> {
        let mut ended = false;
        let mut empty = 0;
        while self.left.len() < 12 {
            let n = self.more()?;
            if n == 0 {
                empty += 1;
                if empty > 3 {
                    return Err(nonsense());
                }
                continue;
            }
            ended = n < self.buf.len();
            if ended && self.left.len() < 12 {
                self.left.clear();
                return Err(nonsense());
            }
        }
        let mut t = Take::new(&self.left);
        let h = Header { len: t.u32()?, kind: t.u16()?, code: t.u16()? };
        self.left.drain(..12);
        if h.len < 12 {
            return Err(nonsense());
        }
        Ok((h, ended))
    }

    fn take(&mut self, n: usize) -> Result<Vec<u8>, Error> {
        while self.left.len() < n {
            if self.more()? == 0 {
                return Err(nonsense());
            }
        }
        Ok(self.left.drain(..n).collect())
    }

    fn command(&mut self, code: u16, params: &[u32]) -> Result<u32, Error> {
        let id = self.id;
        self.id = self.id.wrapping_add(1).max(1);
        self.dev.send(&container(COMMAND, code, id, params))?;
        Ok(id)
    }

    fn data_out(&mut self, code: u16, id: u32, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        let total = size + 12;
        let len = u32::try_from(total).unwrap_or(u32::MAX);
        let mut p = Put(Vec::with_capacity(CHUNK));
        p.u32(len).u16(DATA).u16(code).u32(id);
        let mut chunk = p.0;
        let mut from = from.take(size);
        let mut done = 0u64;
        loop {
            let start = chunk.len();
            chunk.resize(CHUNK, 0);
            let mut filled = start;
            while filled < CHUNK {
                match from.read(&mut chunk[filled..]) {
                    Ok(0) => break,
                    Ok(n) => filled += n,
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) => {
                        self.dev.cancel(id);
                        return Err(Error::Io(e));
                    }
                }
            }
            chunk.truncate(filled);
            done += (filled - start) as u64;
            if !chunk.is_empty() {
                self.dev.send(&chunk)?;
            }
            if !progress(done) {
                self.dev.cancel(id);
                self.left.clear();
                return Err(Error::Cancelled);
            }
            if filled < CHUNK {
                break;
            }
            chunk.clear();
        }
        if done != size {
            self.dev.cancel(id);
            self.left.clear();
            return Err(Error::Failed(t!("folders-remote-mtp-shorter").into()));
        }
        if total % self.dev.max_packet() as u64 == 0 {
            self.dev.send(&[])?;
        }
        Ok(done)
    }

    fn reply(&mut self, id: u32, sink: Sink<'_>, what: &str) -> Result<Vec<u32>, Error> {
        let (mut h, ended) = self.header()?;
        if h.kind == DATA {
            if let Err(e) = self.data_in(&h, ended, sink) {
                if !matches!(e, Error::Unreachable(_)) {
                    self.dev.cancel(id);
                }
                self.left.clear();
                return Err(e);
            }
            h = self.header()?.0;
        }
        if h.kind != RESPONSE {
            self.left.clear();
            return Err(nonsense());
        }
        if h.len as usize - 12 > MOST_RESPONSE {
            self.left.clear();
            return Err(too_much());
        }
        let body = self.take(h.len as usize - 12)?;
        self.last = h.code;
        if h.code != OK {
            return Err(refused(h.code, what));
        }
        Ok(body.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect())
    }

    fn data_in(&mut self, h: &Header, ended: bool, sink: Sink<'_>) -> Result<(), Error> {
        if h.len == u32::MAX {
            let first = std::mem::take(&mut self.left);
            if !sink(&first)? {
                return Err(Error::Cancelled);
            }
            let mut over = ended;
            while !over {
                let n = self.dev.recv(&mut self.buf)?;
                if !sink(&self.buf[..n])? {
                    return Err(Error::Cancelled);
                }
                over = n < self.buf.len();
            }
            return Ok(());
        }
        let mut rest = h.len as usize - 12;
        let first = rest.min(self.left.len());
        let head: Vec<u8> = self.left.drain(..first).collect();
        rest -= first;
        if !sink(&head)? {
            return Err(Error::Cancelled);
        }
        let mut empty = 0;
        while rest > 0 {
            let n = self.dev.recv(&mut self.buf)?;
            if n == 0 {
                empty += 1;
                if empty > 3 {
                    return Err(nonsense());
                }
                continue;
            }
            let mine = n.min(rest);
            rest -= mine;
            self.left.extend_from_slice(&self.buf[mine..n]);
            if !sink(&self.buf[..mine])? {
                return Err(Error::Cancelled);
            }
        }
        Ok(())
    }

    fn run(&mut self, code: u16, params: &[u32], out: Option<&[u8]>, what: &str) -> Result<(Vec<u8>, Vec<u32>), Error> {
        let id = self.command(code, params)?;
        if let Some(data) = out {
            self.data_out(code, id, &mut &data[..], data.len() as u64, &mut |_| true)?;
        }
        let mut got = Vec::new();
        let params = self.reply(
            id,
            &mut |b: &[u8]| {
                if got.len() + b.len() > MOST_ANSWER {
                    return Err(too_much());
                }
                got.extend_from_slice(b);
                Ok(true)
            },
            what,
        )?;
        Ok((got, params))
    }

    fn storages(&mut self) -> Result<Vec<(u32, String)>, Error> {
        if let Some(s) = &self.storages {
            return Ok(s.clone());
        }
        let what = t!("folders-remote-mtp-what-storage");
        let (data, _) = self.run(GET_STORAGE_IDS, &[], None, what)?;
        let ids = Take::new(&data).array()?;
        let mut named: Vec<(u32, String)> = Vec::new();
        for id in ids.into_iter().filter(|id| id & 0xFFFF != 0) {
            let (info, _) = self.run(GET_STORAGE_INFO, &[id], None, what)?;
            named.push((id, storage_name(&info)?));
        }
        let names: Vec<String> = named.iter().map(|(_, n)| n.clone()).collect();
        for (id, name) in named.iter_mut() {
            if names.iter().filter(|n| n.as_str() == name.as_str()).count() > 1 {
                *name = format!("{name} ({id:08X})");
            }
        }
        self.storages = Some(named.clone());
        Ok(named)
    }

    fn info(&mut self, handle: u32, what: &str) -> Result<ObjectInfo, Error> {
        let (data, _) = self.run(GET_OBJECT_INFO, &[handle], None, what)?;
        let mut info = ObjectInfo::decode(&data)?;
        info.name = info.name.replace('/', "-");
        if info.size == u32::MAX as u64 && !info.folder() {
            if let Ok((b, _)) = self.run(GET_OBJECT_PROP_VALUE, &[handle, PROP_SIZE as u32], None, what) {
                info.size = Take::new(&b).u64().unwrap_or(info.size);
            }
        }
        Ok(info)
    }

    fn children(&mut self, dir: &str, at: Node) -> Result<Vec<ObjectInfo>, Error> {
        let what = name_of(dir).to_string();
        let (data, _) = self.run(GET_OBJECT_HANDLES, &[at.storage, 0, at.handle], None, &what)?;
        let handles = Take::new(&data).array()?;
        self.known.retain(|k, _| parent(k) != dir);
        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for handle in handles {
            let mut info = match self.info(handle, &what) {
                Ok(info) => info,
                Err(Error::NotFound(_)) => continue,
                Err(e) => return Err(e),
            };
            if !seen.insert(info.name.clone()) {
                let (stem, ending) = match info.name.rfind('.').filter(|&i| i > 0) {
                    Some(i) => info.name.split_at(i),
                    None => (info.name.as_str(), ""),
                };
                let name = (2..).map(|n| format!("{stem} ({n}){ending}")).find(|n| !seen.contains(n)).expect("a free name");
                seen.insert(name.clone());
                info.name = name;
            }
            let node = Node { storage: at.storage, handle, folder: info.folder() };
            self.known.insert(join(dir, &info.name), node);
            out.push(info);
        }
        Ok(out)
    }

    fn node(&mut self, path: &str) -> Result<Node, Error> {
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        if parts.is_empty() {
            return Ok(Node { storage: 0, handle: ROOT, folder: true });
        }
        let missing = || Error::NotFound(name_of(path).to_string());
        let storage = self.storages()?.into_iter().find(|(_, n)| n == parts[0]).map(|(id, _)| id).ok_or_else(missing)?;
        let mut at = Node { storage, handle: ROOT, folder: true };
        let mut dir = format!("/{}", parts[0]);
        for part in &parts[1..] {
            if !at.folder {
                return Err(missing());
            }
            let next = join(&dir, part);
            if !self.known.contains_key(&next) {
                self.children(&dir, at)?;
            }
            at = *self.known.get(&next).ok_or_else(missing)?;
            dir = next;
        }
        Ok(at)
    }

    fn place(&mut self, path: &str) -> Result<(String, Node), Error> {
        let dir = parent(path);
        let at = self.node(&dir)?;
        if at.storage == 0 {
            return Err(Error::Failed(t!("folders-remote-mtp-only-storage").into()));
        }
        if !at.folder {
            return Err(Error::Failed(t!("folders-remote-mtp-not-folder", name = name_of(&dir))));
        }
        Ok((dir, at))
    }

    fn existing(&mut self, dir: &str, at: Node, path: &str) -> Result<Option<Node>, Error> {
        self.children(dir, at)?;
        Ok(self.known.get(path).copied())
    }

    fn send_info(&mut self, at: Node, name: &str, format: u16, size: u64) -> Result<u32, Error> {
        let info = ObjectInfo { storage: at.storage, format, size, parent: at.handle, name: name.into(), modified: Some(now()) };
        let (_, params) = self.run(SEND_OBJECT_INFO, &[at.storage, at.handle], Some(&info.encode()), name)?;
        self.known.clear();
        params.get(2).copied().ok_or_else(nonsense)
    }

    fn delete(&mut self, handle: u32, what: &str) -> Result<(), Error> {
        let done = self.run(DELETE_OBJECT, &[handle, 0], None, what);
        self.known.clear();
        done.map(|_| ())
    }
}

impl Remote for Mtp {
    fn home(&mut self) -> Result<String, Error> {
        Ok("/".into())
    }

    fn list(&mut self, path: &str) -> Result<Vec<Entry>, Error> {
        let dir = clean(path);
        let at = self.node(&dir)?;
        if at.storage == 0 {
            let storages = self.storages()?;
            if storages.is_empty() {
                return Err(Error::Failed(no_storage().into()));
            }
            return Ok(storages.into_iter().map(|(_, name)| folder_entry(name)).collect());
        }
        if !at.folder {
            return Err(Error::Failed(t!("folders-remote-mtp-not-folder", name = name_of(&dir))));
        }
        Ok(self.children(&dir, at)?.iter().map(entry).collect())
    }

    fn stat(&mut self, path: &str) -> Result<Entry, Error> {
        let path = clean(path);
        let at = self.node(&path)?;
        if at.handle == ROOT {
            return Ok(folder_entry(name_of(&path).to_string()));
        }
        Ok(entry(&self.info(at.handle, name_of(&path))?))
    }

    fn read(&mut self, path: &str, to: &mut dyn Write, progress: Progress) -> Result<u64, Error> {
        let path = clean(path);
        let at = self.node(&path)?;
        if at.folder {
            return Err(Error::Failed(t!("folders-remote-mtp-is-folder", name = name_of(&path))));
        }
        let id = self.command(GET_OBJECT, &[at.handle])?;
        let mut done = 0u64;
        self.reply(
            id,
            &mut |b: &[u8]| {
                to.write_all(b)?;
                done += b.len() as u64;
                Ok(progress(done))
            },
            name_of(&path),
        )?;
        Ok(done)
    }

    fn write_new(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        let path = clean(path);
        let name = name_of(&path).to_string();
        let (dir, at) = self.place(&path)?;
        if self.existing(&dir, at, &path)?.is_some() {
            return Err(Error::Exists(name));
        }
        let handle = self.send_info(at, &name, UNDEFINED, size)?;
        let sent = self.command(SEND_OBJECT, &[]).and_then(|id| {
            let n = self.data_out(SEND_OBJECT, id, from, size, progress)?;
            self.reply(id, &mut skip, &name)?;
            Ok(n)
        });
        if sent.is_err() {
            let _ = self.delete(handle, &name);
        }
        sent
    }

    fn mkdir(&mut self, path: &str) -> Result<(), Error> {
        let path = clean(path);
        let name = name_of(&path).to_string();
        let (dir, at) = self.place(&path)?;
        if self.existing(&dir, at, &path)?.is_some() {
            return Err(Error::Exists(name));
        }
        self.send_info(at, &name, ASSOCIATION, 0).map(|_| ())
    }

    fn rmdir(&mut self, path: &str) -> Result<(), Error> {
        let path = clean(path);
        let name = name_of(&path).to_string();
        let at = self.node(&path)?;
        if at.handle == ROOT {
            return Err(Error::NotSupported(t!("folders-remote-mtp-remove-storage").into()));
        }
        if !at.folder {
            return Err(Error::Failed(t!("folders-remote-mtp-not-folder", name = &name)));
        }
        if !self.children(&path, at)?.is_empty() {
            return Err(Error::Failed(t!("folders-remote-mtp-not-empty", name = &name)));
        }
        self.delete(at.handle, &name)
    }

    fn remove(&mut self, path: &str) -> Result<(), Error> {
        let path = clean(path);
        let name = name_of(&path).to_string();
        let at = self.node(&path)?;
        if at.folder {
            return Err(Error::Failed(t!("folders-remote-mtp-is-folder", name = &name)));
        }
        self.delete(at.handle, &name)
    }

    fn rename(&mut self, from: &str, to: &str) -> Result<(), Error> {
        let (from, to) = (clean(from), clean(to));
        if from == to {
            return Ok(());
        }
        let (name, new) = (name_of(&from).to_string(), name_of(&to).to_string());
        let at = self.node(&from)?;
        if at.handle == ROOT {
            return Err(Error::NotSupported(t!("folders-remote-mtp-rename-storage").into()));
        }
        let (dir, place) = self.place(&to)?;
        if self.existing(&dir, place, &to)?.is_some() {
            return Err(Error::Exists(new));
        }
        if parent(&from) != dir {
            let into = if place.handle == ROOT { 0 } else { place.handle };
            let moved = self.run(MOVE_OBJECT, &[at.handle, place.storage, into], None, &name);
            self.known.clear();
            moved?;
        }
        if new != name {
            let mut p = Put::default();
            p.str(&new);
            let named = self.run(SET_OBJECT_PROP_VALUE, &[at.handle, PROP_NAME as u32], Some(&p.0), &name);
            self.known.clear();
            named?;
        }
        Ok(())
    }
}

impl Drop for Mtp {
    fn drop(&mut self) {
        let _ = self.run(CLOSE_SESSION, &[], None, t!("folders-remote-mtp-what-phone"));
    }
}

fn no_storage() -> &'static str {
    t!("folders-remote-mtp-no-storage")
}

fn storage_name(b: &[u8]) -> Result<String, Error> {
    let mut t = Take::new(b);
    t.bytes(2 + 2 + 2 + 8 + 8 + 4)?;
    let description = t.str()?;
    let label = t.str().unwrap_or_default();
    Ok([description, label]
        .into_iter()
        .map(|s| s.trim().replace('/', "-"))
        .find(|s| !s.is_empty())
        .unwrap_or_else(|| t!("folders-remote-mtp-storage-name").into()))
}

fn clean(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty() && *p != ".").collect();
    format!("/{}", parts.join("/"))
}

fn folder_entry(name: String) -> Entry {
    Entry { name, kind: Kind::Folder, size: 0, modified: None, mode: None }
}

fn entry(info: &ObjectInfo) -> Entry {
    let folder = info.folder();
    Entry {
        name: info.name.clone(),
        kind: if folder { Kind::Folder } else { Kind::File },
        size: if folder { 0 } else { info.size },
        modified: info.modified,
        mode: None,
    }
}

fn skip(_: &[u8]) -> Result<bool, Error> {
    Ok(true)
}

#[cfg(test)]
mod tests;
