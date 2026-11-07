// Zip, written, entry by entry.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};

use flate2::{Compression, Crc};
use flate2::write::DeflateEncoder;

use crate::files::archive::opened_as_found;
use crate::files::archive::pack::Gathered;
use crate::files::archive::zip::{CENTRAL, END, END64, LOCAL, LOCATOR};
use crate::files::archive::zip::dos::to_dos;
use crate::files::meter::{Counted, Meter};
use crate::toolkit::t;

pub(in crate::files::archive) enum Body<'b> {
    Folder,
    File(&'b Gathered),
    Bytes(&'b [u8]),
}

pub(in crate::files::archive) struct ZipWriter<W: Write + Seek> {
    pub(in crate::files::archive) w: W,
    pub(in crate::files::archive) central: Vec<u8>,
    pub(in crate::files::archive) count: u64,
    pub(in crate::files::archive) big: u64,
}

struct Stored {
    name: Vec<u8>,
    folder: bool,
    flags: u16,
    method: u16,
    time: u16,
    date: u16,
    stamp: Option<i32>,
}

struct Done {
    offset: u64,
    crc: u32,
    size: u64,
    packed: u64,
    wide: bool,
}

impl Stored {
    fn local_header(&self, wide: bool) -> Vec<u8> {
        let mut extra = Vec::new();
        if wide {
            extra.extend_from_slice(&1u16.to_le_bytes());
            extra.extend_from_slice(&16u16.to_le_bytes());
            extra.extend_from_slice(&[0; 16]);
        }
        stamp_field(&mut extra, self.stamp);
        let mut head = Vec::with_capacity(30 + self.name.len() + extra.len());
        head.extend_from_slice(&LOCAL.to_le_bytes());
        head.extend_from_slice(&(if wide { 45u16 } else { 20 }).to_le_bytes());
        head.extend_from_slice(&self.flags.to_le_bytes());
        head.extend_from_slice(&self.method.to_le_bytes());
        head.extend_from_slice(&self.time.to_le_bytes());
        head.extend_from_slice(&self.date.to_le_bytes());
        head.extend_from_slice(&[0; 4]);
        let sizes = if wide { [0xFF; 8] } else { [0; 8] };
        head.extend_from_slice(&sizes);
        head.extend_from_slice(&(self.name.len() as u16).to_le_bytes());
        head.extend_from_slice(&(extra.len() as u16).to_le_bytes());
        head.extend_from_slice(&self.name);
        head.extend_from_slice(&extra);
        head
    }
}

fn stamp_field(extra: &mut Vec<u8>, stamp: Option<i32>) {
    if let Some(s) = stamp {
        extra.extend_from_slice(&0x5455u16.to_le_bytes());
        extra.extend_from_slice(&5u16.to_le_bytes());
        extra.push(1);
        extra.extend_from_slice(&s.to_le_bytes());
    }
}

impl<W: Write + Seek> ZipWriter<W> {
    pub(in crate::files::archive) fn new(w: W, big: u64) -> Self {
        ZipWriter { w, central: Vec::new(), count: 0, big }
    }

    pub(in crate::files::archive) fn add(&mut self, name: &[u8], body: Body, mode: u32, mtime: i64, meter: &Meter) -> io::Result<()> {
        let offset = self.w.stream_position()?;
        let folder = matches!(body, Body::Folder);
        let mut name = name.to_vec();
        if folder {
            name.push(b'/');
        }
        let hint = match body {
            Body::Folder => 0,
            Body::File(g) => g.meta.len(),
            Body::Bytes(b) => b.len() as u64,
        };
        let wide = hint + hint / 512 + 1024 >= self.big;
        let mut source = match body {
            Body::File(g) => Some(opened_as_found(g)?),
            _ => None,
        };
        let method = match source.as_mut() {
            Some(f) if hint > 0 => {
                if worth_squeezing(f)? {
                    8u16
                } else {
                    0
                }
            }
            _ => 0,
        };
        let (time, date) = to_dos(mtime);
        let flags: u16 = if std::str::from_utf8(&name).is_ok() { 0x800 } else { 0 };
        let stored = Stored { name, folder, flags, method, time, date, stamp: i32::try_from(mtime).ok() };
        self.w.write_all(&stored.local_header(wide))?;
        let start = self.w.stream_position()?;
        let (crc, size) = self.copy_body(body, source, method, meter)?;
        let end = self.w.stream_position()?;
        let packed = end - start;
        if !wide && (packed >= self.big || size >= self.big) {
            return Err(io::Error::other(t!("folders-archive-pack-grew")));
        }
        let done = Done { offset, crc, size, packed, wide };
        self.fill_in(stored.name.len() as u64, &done)?;
        self.w.seek(SeekFrom::Start(end))?;
        self.central_record(&stored, &done, mode);
        Ok(())
    }

    fn copy_body(&mut self, body: Body, mut source: Option<File>, method: u16, meter: &Meter) -> io::Result<(u32, u64)> {
        Ok(match body {
            Body::Folder => (0, 0),
            Body::Bytes(b) => {
                let mut s = Summed { inner: b, crc: Crc::new(), n: 0 };
                io::copy(&mut s, &mut self.w)?;
                (s.crc.sum(), s.n)
            }
            Body::File(..) => {
                let f = source.take().unwrap_or_else(|| unreachable!());
                let mut s = Summed { inner: Counted::new(f, meter), crc: Crc::new(), n: 0 };
                if method == 8 {
                    let mut enc = DeflateEncoder::new(&mut self.w, Compression::default());
                    io::copy(&mut s, &mut enc)?;
                    enc.finish()?;
                } else {
                    io::copy(&mut s, &mut self.w)?;
                }
                (s.crc.sum(), s.n)
            }
        })
    }

    fn fill_in(&mut self, name_len: u64, done: &Done) -> io::Result<()> {
        self.w.seek(SeekFrom::Start(done.offset + 14))?;
        self.w.write_all(&done.crc.to_le_bytes())?;
        if done.wide {
            self.w.seek(SeekFrom::Start(done.offset + 30 + name_len + 4))?;
            self.w.write_all(&done.size.to_le_bytes())?;
            self.w.write_all(&done.packed.to_le_bytes())?;
        } else {
            self.w.write_all(&(done.packed as u32).to_le_bytes())?;
            self.w.write_all(&(done.size as u32).to_le_bytes())?;
        }
        Ok(())
    }

    fn central_record(&mut self, stored: &Stored, done: &Done, mode: u32) {
        let big = self.big;
        let clip = |v: u64| if v >= big { 0xFFFF_FFFF } else { v as u32 };
        let mut extra = Vec::new();
        let wide_fields: Vec<u64> = [done.size, done.packed, done.offset].into_iter().filter(|&v| v >= big).collect();
        if !wide_fields.is_empty() {
            extra.extend_from_slice(&1u16.to_le_bytes());
            extra.extend_from_slice(&(wide_fields.len() as u16 * 8).to_le_bytes());
            for v in &wide_fields {
                extra.extend_from_slice(&v.to_le_bytes());
            }
        }
        stamp_field(&mut extra, stored.stamp);
        let c = &mut self.central;
        c.extend_from_slice(&CENTRAL.to_le_bytes());
        c.extend_from_slice(&((3u16 << 8) | 63).to_le_bytes());
        c.extend_from_slice(&(if done.wide || !wide_fields.is_empty() { 45u16 } else { 20 }).to_le_bytes());
        c.extend_from_slice(&stored.flags.to_le_bytes());
        c.extend_from_slice(&stored.method.to_le_bytes());
        c.extend_from_slice(&stored.time.to_le_bytes());
        c.extend_from_slice(&stored.date.to_le_bytes());
        c.extend_from_slice(&done.crc.to_le_bytes());
        c.extend_from_slice(&clip(done.packed).to_le_bytes());
        c.extend_from_slice(&clip(done.size).to_le_bytes());
        c.extend_from_slice(&(stored.name.len() as u16).to_le_bytes());
        c.extend_from_slice(&(extra.len() as u16).to_le_bytes());
        c.extend_from_slice(&[0; 6]);
        c.extend_from_slice(&((mode << 16) | if stored.folder { 0x10 } else { 0 }).to_le_bytes());
        c.extend_from_slice(&clip(done.offset).to_le_bytes());
        c.extend_from_slice(&stored.name);
        c.extend_from_slice(&extra);
        self.count += 1;
    }

    pub(in crate::files::archive) fn finish(mut self) -> io::Result<W> {
        let cd_off = self.w.stream_position()?;
        self.w.write_all(&self.central)?;
        let cd_size = self.central.len() as u64;
        let big = self.big;
        if self.count >= 0xFFFF || cd_off >= big || cd_size >= big {
            let z_at = self.w.stream_position()?;
            let mut z = Vec::with_capacity(76);
            z.extend_from_slice(&END64.to_le_bytes());
            z.extend_from_slice(&44u64.to_le_bytes());
            z.extend_from_slice(&((3u16 << 8) | 63).to_le_bytes());
            z.extend_from_slice(&45u16.to_le_bytes());
            z.extend_from_slice(&[0; 8]);
            z.extend_from_slice(&self.count.to_le_bytes());
            z.extend_from_slice(&self.count.to_le_bytes());
            z.extend_from_slice(&cd_size.to_le_bytes());
            z.extend_from_slice(&cd_off.to_le_bytes());
            z.extend_from_slice(&LOCATOR.to_le_bytes());
            z.extend_from_slice(&0u32.to_le_bytes());
            z.extend_from_slice(&z_at.to_le_bytes());
            z.extend_from_slice(&1u32.to_le_bytes());
            self.w.write_all(&z)?;
        }
        let clip = |v: u64| if v >= big { 0xFFFF_FFFF } else { v as u32 };
        let n = self.count.min(0xFFFF) as u16;
        let mut e = Vec::with_capacity(22);
        e.extend_from_slice(&END.to_le_bytes());
        e.extend_from_slice(&[0; 4]);
        e.extend_from_slice(&n.to_le_bytes());
        e.extend_from_slice(&n.to_le_bytes());
        e.extend_from_slice(&clip(cd_size).to_le_bytes());
        e.extend_from_slice(&clip(cd_off).to_le_bytes());
        e.extend_from_slice(&0u16.to_le_bytes());
        self.w.write_all(&e)?;
        self.w.flush()?;
        Ok(self.w)
    }
}

fn worth_squeezing(f: &mut File) -> io::Result<bool> {
    let mut probe = Vec::with_capacity(1 << 16);
    (&mut *f).take(1 << 16).read_to_end(&mut probe)?;
    f.seek(SeekFrom::Start(0))?;
    let mut enc = DeflateEncoder::new(Vec::new(), Compression::fast());
    enc.write_all(&probe)?;
    let out = enc.finish()?;
    Ok(out.len() * 100 < probe.len() * 97)
}

struct Summed<R> {
    inner: R,
    crc: Crc,
    n: u64,
}

impl<R: Read> Read for Summed<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.crc.update(&buf[..n]);
        self.n += n as u64;
        Ok(n)
    }
}
