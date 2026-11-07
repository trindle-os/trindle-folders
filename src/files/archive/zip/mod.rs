// Zip, read: the end record's shapes and the numbers zip is written in.

pub(super) mod dir;
pub(super) mod dos;
pub(super) mod lock;
pub(super) mod write;

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::toolkit::t;

pub(super) const LOCAL: u32 = 0x0403_4b50;
pub(super) const CENTRAL: u32 = 0x0201_4b50;
pub(super) const END: u32 = 0x0605_4b50;
const END64: u32 = 0x0606_4b50;
const LOCATOR: u32 = 0x0706_4b50;
pub(super) const S_IFMT: u32 = 0o170_000;
pub(super) const S_IFDIR: u32 = 0o040_000;
pub(super) const S_IFLNK: u32 = 0o120_000;
#[cfg(test)]
pub(super) const S_IFREG: u32 = 0o100_000;

pub(in crate::files::archive) struct ZipEntry {
    pub(in crate::files::archive) name: String,
    pub(super) flags: u16,
    pub(super) method: u16,
    pub(super) crc: u32,
    pub(in crate::files::archive) packed: u64,
    pub(in crate::files::archive) size: u64,
    pub(super) offset: u64,
    pub(in crate::files::archive) mode: Option<u32>,
    pub(in crate::files::archive) folder: bool,
    pub(in crate::files::archive) link: bool,
    pub(in crate::files::archive) modified: Option<i64>,
    pub(super) aes: Option<ZipAes>,
    pub(super) check: [u8; 2],
}

#[derive(Clone, Copy)]
pub(super) struct ZipAes {
    pub(super) strength: u8,
    pub(super) method: u16,
    pub(super) no_crc: bool,
}

impl ZipEntry {
    pub(super) fn sealed(&self) -> bool {
        self.flags & 1 != 0 && !self.folder
    }

    fn squeezed(&self) -> u16 {
        self.aes.map_or(self.method, |a| a.method)
    }

    pub(super) fn ours(&self) -> bool {
        matches!(self.squeezed(), 0 | 8 | 9)
    }

    pub(super) fn lock_known(&self) -> bool {
        self.flags & 0x40 == 0 && (self.method != 99 || self.aes.is_some_and(|a| (1..=3).contains(&a.strength)))
    }
}

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

pub(super) fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap_or_default())
}

pub(super) fn u64_at(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap_or_default())
}

pub(super) fn bad(what: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, t!("folders-archive-damaged-zip", why = what))
}

pub(super) fn read_at(file: &File, at: u64, len: usize) -> io::Result<Vec<u8>> {
    let mut f = file;
    f.seek(SeekFrom::Start(at))?;
    let mut buf = vec![0; len];
    f.read_exact(&mut buf)?;
    Ok(buf)
}
