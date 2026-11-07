// Reading a zip's central directory and the bytes of one entry.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use flate2::Crc;
use flate2::read::DeflateDecoder;

use crate::files::archive::tidy;
use crate::files::archive::trouble::{trouble, Key, Sealed, Trouble};
use crate::files::archive::zip::{bad, read_at, u16_at, u32_at, u64_at, ZipAes, ZipEntry, CENTRAL, END, END64, LOCAL, LOCATOR, S_IFDIR, S_IFLNK, S_IFMT};
use crate::files::archive::zip::dos::{cp437, from_dos};
use crate::files::archive::zip::lock::{ZipOld, ZipUnlock};
use crate::files::meter::{Counted, Meter};
use crate::toolkit::t;

pub(in crate::files::archive) fn zip_dir(file: &File) -> io::Result<Vec<ZipEntry>> {
    let (cd_size, cd_off, cd_end) = end_record(file)?;
    let shift = cd_end.checked_sub(cd_off).and_then(|s| s.checked_sub(cd_size)).ok_or_else(|| bad(t!("folders-archive-zip-directory-out-of-place")))?;
    let cd = read_at(file, cd_off + shift, usize::try_from(cd_size).map_err(|_| bad(t!("folders-archive-zip-directory-too-big")))?)?;
    let mut out = Vec::new();
    let mut i = 0;
    while i + 46 <= cd.len() && u32_at(&cd, i) == CENTRAL {
        let r = &cd[i..];
        let (nlen, xlen, clen) = (usize::from(u16_at(r, 28)), usize::from(u16_at(r, 30)), usize::from(u16_at(r, 32)));
        if 46 + nlen + xlen + clen > r.len() {
            return Err(bad(t!("folders-archive-zip-record-runs-past-the-directory")));
        }
        out.push(zip_entry(r, nlen, xlen, shift)?);
        i += 46 + nlen + xlen + clen;
    }
    Ok(out)
}

fn end_record(file: &File) -> io::Result<(u64, u64, u64)> {
    let len = file.metadata()?.len();
    let tail_len = len.min(22 + 65_535);
    let tail = read_at(file, len - tail_len, tail_len as usize)?;
    let at = (0..tail.len().saturating_sub(21))
        .rev()
        .find(|&i| u32_at(&tail, i) == END && i + 22 + usize::from(u16_at(&tail, i + 20)) <= tail.len())
        .ok_or_else(|| bad(t!("folders-archive-zip-no-end-record")))?;
    let end_at = len - tail_len + at as u64;
    let end = &tail[at..at + 22];
    let (mut cd_size, mut cd_off) = (u64::from(u32_at(end, 12)), u64::from(u32_at(end, 16)));
    let mut cd_end = end_at;
    if end_at >= 20 {
        let loc = read_at(file, end_at - 20, 20)?;
        if u32_at(&loc, 0) == LOCATOR {
            let mut z_at = u64_at(&loc, 8);
            let looks = |at: u64| at.checked_add(56).is_some_and(|to| to <= end_at) && read_at(file, at, 4).is_ok_and(|b| u32_at(&b, 0) == END64);
            if !looks(z_at) {
                z_at = (end_at - 20).checked_sub(56).filter(|&a| looks(a)).ok_or_else(|| bad(t!("folders-archive-zip-no-zip64-end-record")))?;
            }
            let z = read_at(file, z_at, 56)?;
            cd_size = u64_at(&z, 40);
            cd_off = u64_at(&z, 48);
            cd_end = z_at;
        }
    }
    Ok((cd_size, cd_off, cd_end))
}

#[derive(Default)]
struct Extras {
    stamp: Option<i64>,
    unicode: Option<String>,
    aes: Option<ZipAes>,
}

fn read_extras(extra: &[u8], raw: &[u8], sizes: [&mut u64; 3]) -> Extras {
    let mut found = Extras::default();
    let mut sizes = sizes;
    let mut x = 0;
    while x + 4 <= extra.len() {
        let (id, n) = (u16_at(extra, x), usize::from(u16_at(extra, x + 2)));
        let Some(data) = extra.get(x + 4..x + 4 + n) else { break };
        match id {
            0x0001 => {
                let mut at = 0;
                for field in sizes.iter_mut() {
                    if **field == 0xFFFF_FFFF && at + 8 <= data.len() {
                        **field = u64_at(data, at);
                        at += 8;
                    }
                }
            }
            0x5455 if data.len() >= 5 && data[0] & 1 != 0 => {
                found.stamp = Some(i64::from(i32::from_le_bytes([data[1], data[2], data[3], data[4]])));
            }
            0x9901 if data.len() >= 7 => {
                found.aes = Some(ZipAes { no_crc: u16_at(data, 0) == 2, strength: data[4], method: u16_at(data, 5) });
            }
            0x7075 if data.len() >= 5 && data[0] == 1 => {
                let mut crc = Crc::new();
                crc.update(raw);
                if crc.sum() == u32_at(data, 1) {
                    found.unicode = std::str::from_utf8(&data[5..]).ok().map(str::to_string);
                }
            }
            _ => {}
        }
        x += 4 + n;
    }
    found
}

fn zip_entry(r: &[u8], nlen: usize, xlen: usize, shift: u64) -> io::Result<ZipEntry> {
    let raw = &r[46..46 + nlen];
    let extra = &r[46 + nlen..46 + nlen + xlen];
    let made = u16_at(r, 4);
    let flags = u16_at(r, 8);
    let mut size = u64::from(u32_at(r, 24));
    let mut packed = u64::from(u32_at(r, 20));
    let mut offset = u64::from(u32_at(r, 42));
    let Extras { stamp, unicode, aes } = read_extras(extra, raw, [&mut size, &mut packed, &mut offset]);
    let name = unicode.unwrap_or_else(|| match std::str::from_utf8(raw) {
        Ok(s) => s.to_string(),
        Err(_) if flags & 0x800 == 0 => raw.iter().map(|&b| cp437(b)).collect(),
        Err(_) => String::from_utf8_lossy(raw).into_owned(),
    });
    let ext = u32_at(r, 38);
    let mode = if made >> 8 == 3 && ext >> 16 != 0 { Some(ext >> 16) } else { None };
    let folder = name.ends_with('/') || name.ends_with('\\') || ext & 0x10 != 0 || mode.is_some_and(|m| m & S_IFMT == S_IFDIR);
    Ok(ZipEntry {
        name: tidy(&name.replace('\\', "/")),
        flags,
        method: u16_at(r, 10),
        crc: u32_at(r, 16),
        packed,
        size,
        offset: offset.checked_add(shift).ok_or_else(|| bad(t!("folders-archive-zip-entry-runs-past-the-end")))?,
        mode,
        folder,
        link: !folder && mode.is_some_and(|m| m & S_IFMT == S_IFLNK),
        modified: stamp.or_else(|| from_dos(u16_at(r, 12), u16_at(r, 14))),
        aes,
        check: [r[19], r[13]],
    })
}

pub(in crate::files::archive) fn zip_data<'f, 'm>(file: &'f File, e: &ZipEntry, key: Option<&Key>, meter: &'m Meter<'m>) -> io::Result<Box<dyn Read + 'f>>
where
    'm: 'f,
{
    let head = read_at(file, e.offset, 30)?;
    if u32_at(&head, 0) != LOCAL {
        return Err(bad(t!("folders-archive-zip-no-local-header")));
    }
    let len = file.metadata()?.len();
    let past = || bad(t!("folders-archive-zip-entry-runs-past-the-end"));
    let start = e.offset.checked_add(30 + u64::from(u16_at(&head, 26)) + u64::from(u16_at(&head, 28))).ok_or_else(past)?;
    if start.checked_add(e.packed).is_none_or(|to| to > len) {
        return Err(past());
    }
    let mut f = file;
    f.seek(SeekFrom::Start(start))?;
    let raw = Counted::new(f.take(e.packed), meter);
    let (plain, sum): (Box<dyn Read + 'f>, Option<u32>) = if !e.sealed() {
        (Box::new(raw), Some(e.crc))
    } else {
        if !e.lock_known() {
            return Err(trouble(Trouble::LockedForeign));
        }
        let key = key.ok_or_else(|| trouble(Trouble::Locked))?;
        match e.aes {
            Some(a) if e.method == 99 => (Box::new(ZipUnlock::open(raw, e.packed, a.strength, key)?), (!a.no_crc).then_some(e.crc)),
            _ => (Box::new(ZipOld::open(raw, key, e.check)?), Some(e.crc)),
        }
    };
    let inner: Box<dyn Read + 'f> = match e.squeezed() {
        0 => plain,
        8 => Box::new(DeflateDecoder::new(plain)),
        9 => Box::new(deflate64::Deflate64Decoder::new(plain)),
        _ if e.sealed() => return Err(trouble(Trouble::LockedForeign)),
        _ => return Err(trouble(Trouble::Foreign)),
    };
    let checked = Checked { inner, crc: Crc::new(), got: 0, want: e.size, sum };
    Ok(if e.sealed() { Box::new(Sealed(checked)) } else { Box::new(checked) })
}

struct Checked<R> {
    inner: R,
    crc: Crc,
    got: u64,
    want: u64,
    sum: Option<u32>,
}

impl<R: Read> Read for Checked<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        if n == 0 && !buf.is_empty() && (self.got != self.want || self.sum.is_some_and(|s| s != self.crc.sum())) {
            return Err(bad(t!("folders-archive-zip-an-entry-s-check-does-not-match")));
        }
        self.got += n as u64;
        if self.got > self.want {
            return Err(bad(t!("folders-archive-zip-an-entry-is-longer-than-it-says")));
        }
        self.crc.update(&buf[..n]);
        Ok(n)
    }
}
