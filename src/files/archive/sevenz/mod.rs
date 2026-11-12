//! 7z, read: the numbers and shapes its headers are written in.

pub(super) mod header;
pub(super) mod lock;
pub(super) mod unpack;

use std::io;

use crate::files::archive::trouble::{trouble, Trouble};
use crate::files::archive::zip::{u32_at, u64_at};
use crate::toolkit::t;

pub(super) const SZ_MAGIC: [u8; 6] = [b'7', b'z', 0xbc, 0xaf, 0x27, 0x1c];
const SZ_AES: [u8; 4] = [0x06, 0xf1, 0x07, 0x01];
const SZ_HEAD_MAX: u64 = 1 << 28;
const SZ_DICT_MAX: u64 = 1 << 31;
const SZ_COUNT_MAX: u64 = 1 << 24;
const SZ_EPOCH: i64 = 11_644_473_600;

fn sz_bad(what: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, t!("folders-archive-damaged-7z", why = what))
}

fn sz_locked() -> io::Error {
    trouble(Trouble::Locked)
}

fn sz_unknown() -> io::Error {
    trouble(Trouble::Foreign)
}

fn sz_sealed(folder: &SzFolder) -> bool {
    folder.coders.iter().any(|c| c.id == SZ_AES)
}

fn sz_ours(folder: &SzFolder) -> bool {
    let ours = |c: &SzCoder| {
        c.ins == 1 && c.outs == 1 && (c.id == SZ_AES || matches!(c.id.as_slice(), [0x00] | [0x03, 0x01, 0x01] | [0x21] | [0x03, 0x03, 0x01, 0x03] | [0x03]))
    };
    folder.packed.len() == 1 && folder.coders.iter().all(ours)
}

struct SzBytes<'b> {
    b: &'b [u8],
    at: usize,
}

impl<'b> SzBytes<'b> {
    fn new(b: &'b [u8]) -> Self {
        SzBytes { b, at: 0 }
    }

    fn left(&self) -> usize {
        self.b.len() - self.at
    }

    fn take(&mut self, n: usize) -> io::Result<&'b [u8]> {
        if n > self.left() {
            return Err(sz_bad(t!("folders-archive-7z-the-header-ends-too-soon")));
        }
        let s = &self.b[self.at..self.at + n];
        self.at += n;
        Ok(s)
    }

    fn byte(&mut self) -> io::Result<u8> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> io::Result<u32> {
        Ok(u32_at(self.take(4)?, 0))
    }

    fn u64(&mut self) -> io::Result<u64> {
        Ok(u64_at(self.take(8)?, 0))
    }

    fn number(&mut self) -> io::Result<u64> {
        let first = self.byte()?;
        let mut mask = 0x80u8;
        let mut value = 0u64;
        for i in 0..8 {
            if first & mask == 0 {
                let high = u64::from(first & mask.wrapping_sub(1));
                return Ok(value | (high << (8 * i)));
            }
            value |= u64::from(self.byte()?) << (8 * i);
            mask >>= 1;
        }
        Ok(value)
    }

    fn count(&mut self) -> io::Result<usize> {
        let n = self.number()?;
        if n > SZ_COUNT_MAX {
            return Err(sz_bad(t!("folders-archive-7z-a-count-is-far-too-big")));
        }
        Ok(n as usize)
    }

    fn bits(&mut self, n: usize) -> io::Result<Vec<bool>> {
        let bytes = self.take(n.div_ceil(8))?;
        Ok((0..n).map(|i| bytes[i / 8] & (0x80 >> (i % 8)) != 0).collect())
    }

    fn defined(&mut self, n: usize) -> io::Result<Vec<bool>> {
        if self.byte()? != 0 {
            Ok(vec![true; n])
        } else {
            self.bits(n)
        }
    }

    fn crcs(&mut self, n: usize) -> io::Result<Vec<Option<u32>>> {
        let have = self.defined(n)?;
        have.into_iter().map(|h| if h { self.u32().map(Some) } else { Ok(None) }).collect()
    }
}

pub(super) struct SzCoder {
    pub(super) id: Vec<u8>,
    pub(super) props: Vec<u8>,
    pub(super) ins: usize,
    pub(super) outs: usize,
}

#[derive(Default)]
pub(super) struct SzFolder {
    pub(super) coders: Vec<SzCoder>,
    pub(super) binds: Vec<(usize, usize)>,
    pub(super) packed: Vec<usize>,
    pub(super) sizes: Vec<u64>,
    pub(super) crc: Option<u32>,
}

impl SzFolder {
    fn size(&self) -> u64 {
        (0..self.sizes.len()).find(|o| !self.binds.iter().any(|b| b.1 == *o)).map(|o| self.sizes[o]).unwrap_or(0)
    }
}

#[derive(Default)]
pub(super) struct SzStreams {
    pub(super) pack_pos: u64,
    pub(super) pack_sizes: Vec<u64>,
    pub(super) folders: Vec<SzFolder>,
    pub(super) counts: Vec<usize>,
    pub(super) sizes: Vec<u64>,
    pub(super) crcs: Vec<Option<u32>>,
}

impl SzStreams {
    fn packed(&self, folder: usize) -> io::Result<(u64, u64)> {
        let first: usize = self.folders[..folder].iter().map(|f| f.packed.len()).sum();
        let size = *self.pack_sizes.get(first).ok_or_else(|| sz_bad(t!("folders-archive-7z-a-block-has-no-packed-bytes")))?;
        let at = self.pack_sizes[..first].iter().fold(32u64.checked_add(self.pack_pos), |a, &b| a.and_then(|a| a.checked_add(b)));
        Ok((at.ok_or_else(|| sz_bad(t!("folders-archive-7z-a-block-is-out-of-reach")))?, size))
    }
}

struct SzFile {
    name: String,
    stream: bool,
    folder: bool,
    anti: bool,
    modified: Option<i64>,
    attrib: Option<u32>,
}

struct SzArchive {
    streams: SzStreams,
    files: Vec<SzFile>,
}
