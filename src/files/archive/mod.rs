//! Archives: packing into zip or tar, unpacking, and looking inside one as a folder.

mod pack;
mod sevenz;
#[cfg(test)]
mod tests;
mod trouble;
mod unpack;
mod walk;
mod zip;

pub use self::pack::compress;
pub use self::trouble::{trouble_of, wants_key, why, Key, Trouble};
pub use self::unpack::{extract, take_out, Place};
pub(super) use self::trouble::trouble;

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::sync::atomic::AtomicBool;

use flate2::read::MultiGzDecoder;

use crate::common::files::{open_named, readable, Named, Which};
use crate::files::archive::pack::Gathered;
use crate::files::archive::sevenz::SZ_MAGIC;
use crate::files::archive::sevenz::unpack::walk_7z;
use crate::files::archive::walk::{walk, walk_zip};
use crate::toolkit::t;
use super::unarchive;
use super::meter::Meter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Zip,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    SevenZ,
    Gz,
    Bz2,
    Xz,
    Other,
}

const ENDINGS: [(&str, Format); 36] = [
    (".tar.bz2", Format::TarBz2),
    (".tar.gz", Format::TarGz),
    (".tar.xz", Format::TarXz),
    (".tar.zst", Format::Other),
    (".tar.lz4", Format::Other),
    (".tar.lzma", Format::Other),
    (".tar.lz", Format::Other),
    (".tar.z", Format::Other),
    (".tbz2", Format::TarBz2),
    (".tbz", Format::TarBz2),
    (".tgz", Format::TarGz),
    (".txz", Format::TarXz),
    (".tzst", Format::Other),
    (".tlz", Format::Other),
    (".tar", Format::Tar),
    (".zip", Format::Zip),
    (".7z", Format::SevenZ),
    (".rar", Format::Other),
    (".iso", Format::Other),
    (".cab", Format::Other),
    (".cpio", Format::Other),
    (".rpm", Format::Other),
    (".deb", Format::Other),
    (".udeb", Format::Other),
    (".ar", Format::Other),
    (".lzh", Format::Other),
    (".lha", Format::Other),
    (".xar", Format::Other),
    (".gz", Format::Gz),
    (".bz2", Format::Bz2),
    (".xz", Format::Xz),
    (".zst", Format::Other),
    (".lz4", Format::Other),
    (".lzma", Format::Other),
    (".lz", Format::Other),
    (".z", Format::Other),
];

impl Format {
    pub const ALL: [Format; 5] = [Format::Zip, Format::Tar, Format::TarGz, Format::TarBz2, Format::TarXz];

    pub fn ending(self) -> &'static str {
        match self {
            Format::Zip => ".zip",
            Format::Tar => ".tar",
            Format::TarGz => ".tar.gz",
            Format::TarBz2 => ".tar.bz2",
            Format::TarXz => ".tar.xz",
            Format::SevenZ => ".7z",
            Format::Gz => ".gz",
            Format::Bz2 => ".bz2",
            Format::Xz => ".xz",
            Format::Other => "",
        }
    }

    pub fn of(path: &Path) -> Option<Format> {
        let name = path.file_name()?.to_string_lossy().to_lowercase();
        ENDINGS.iter().find(|(end, _)| name.len() > end.len() && name.ends_with(end)).map(|&(_, f)| f)
    }
}

pub(super) fn opened(path: &Path) -> io::Result<File> {
    let named = match open_named(path, None, None)? {
        Named::Link(_) => open_named(&fs::canonicalize(path)?, None, None)?,
        named => named,
    };
    match named {
        Named::File(file, _) => readable(&file),
        _ => Err(io::Error::new(io::ErrorKind::Unsupported, t!("kit-files-not-a-file"))),
    }
}

fn opened_as_found(g: &Gathered) -> io::Result<File> {
    match open_named(&g.path, None, Some(Which::of(&g.meta)))? {
        Named::File(file, _) => readable(&file),
        _ => Err(io::Error::new(io::ErrorKind::Unsupported, t!("kit-files-not-a-file"))),
    }
}

pub fn sniff(path: &Path) -> io::Result<Option<Format>> {
    let mut head = Vec::with_capacity(512);
    opened(path)?.take(512).read_to_end(&mut head)?;
    let tar_or = |tar: Format, alone: Format| if tar_within(path, tar) { tar } else { alone };
    let found = if head.starts_with(b"PK\x03\x04") || head.starts_with(b"PK\x05\x06") || head.starts_with(b"PK\x07\x08") {
        Some(Format::Zip)
    } else if head.starts_with(&[0x1f, 0x8b]) {
        Some(tar_or(Format::TarGz, Format::Gz))
    } else if head.starts_with(b"BZh") {
        Some(tar_or(Format::TarBz2, Format::Bz2))
    } else if head.starts_with(&SZ_MAGIC) {
        Some(Format::SevenZ)
    } else if head.starts_with(&[0xfd, b'7', b'z', b'X', b'Z', 0]) {
        Some(tar_or(Format::TarXz, Format::Xz))
    } else if OTHER_MARKS.iter().any(|m| head.starts_with(m)) {
        Some(Format::Other)
    } else if looks_like_tar(&head) {
        Some(Format::Tar)
    } else {
        None
    };
    Ok(found)
}

const OTHER_MARKS: [&[u8]; 12] = [
    b"Rar!\x1a\x07",
    &[0x28, 0xb5, 0x2f, 0xfd],
    &[0x04, 0x22, 0x4d, 0x18],
    b"MSCF",
    b"!<arch>\n",
    b"07070",
    &[0xc7, 0x71],
    &[0x71, 0xc7],
    &[0xed, 0xab, 0xee, 0xdb],
    b"xar!",
    b"LZIP",
    &[0x1f, 0x9d],
];

fn looks_like_tar(head: &[u8]) -> bool {
    head.len() == 512 && (&head[257..262] == b"ustar" || tar_sum_holds(head) || head.iter().all(|&b| b == 0))
}

fn tar_within(path: &Path, tar: Format) -> bool {
    let Ok(file) = opened(path) else { return true };
    let mut head = Vec::with_capacity(512);
    match unsqueeze(tar, BufReader::new(file)).take(512).read_to_end(&mut head) {
        Ok(_) => looks_like_tar(&head),
        Err(_) => true,
    }
}

fn unsqueeze<'r>(format: Format, raw: impl Read + 'r) -> Box<dyn Read + 'r> {
    match format {
        Format::TarGz | Format::Gz => Box::new(MultiGzDecoder::new(raw)),
        Format::TarBz2 | Format::Bz2 => Box::new(bzip2::read::MultiBzDecoder::new(raw)),
        Format::TarXz | Format::Xz => Box::new(lzma_rust2::XzReader::new(raw, true)),
        _ => Box::new(raw),
    }
}

fn tar_sum_holds(block: &[u8]) -> bool {
    let text = String::from_utf8_lossy(&block[148..156]);
    let Ok(want) = u32::from_str_radix(text.trim_matches(|c: char| c == ' ' || c == '\0'), 8) else { return false };
    let sum: u32 = block.iter().enumerate().map(|(i, &b)| if (148..156).contains(&i) { 32 } else { u32::from(b) }).sum();
    sum == want && block.iter().any(|&b| b != 0)
}

pub fn find(path: &Path) -> io::Result<Format> {
    if let Some(f) = sniff(path)? {
        return Ok(f);
    }
    Format::of(path).ok_or_else(|| trouble(Trouble::Unknown))
}

pub const MOST_THINGS: usize = 1_000_000;
const ROOM_KEPT: u64 = 256 << 20;

pub fn stem(path: &Path) -> String {
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let lower = name.to_lowercase();
    let cut = ENDINGS.iter().find(|(end, _)| lower.len() > end.len() && lower.ends_with(end)).map(|(end, _)| name.len() - end.len());
    let cut = cut.or_else(|| name.rfind('.').filter(|&i| i > 0));
    match cut {
        Some(i) if name.is_char_boundary(i) && lower.ends_with(".rar") => {
            let whole = &name[..i];
            unarchive::rar_part(whole).map_or(whole, |(head, _)| &head[..head.len() - 5]).to_string()
        }
        Some(i) if name.is_char_boundary(i) => name[..i].to_string(),
        _ => name,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub folder: bool,
    pub size: u64,
    pub modified: Option<i64>,
    pub mode: Option<u32>,
    pub locked: bool,
}

pub fn list(archive: &Path, key: Option<&Key>) -> io::Result<Vec<Entry>> {
    list_up_to(archive, key, MOST_THINGS)
}

fn list_up_to(archive: &Path, key: Option<&Key>, most: usize) -> io::Result<Vec<Entry>> {
    let stop = AtomicBool::new(false);
    let quiet = |_: u64, _: u64| {};
    let meter = Meter::new(&stop, &quiet);
    let mut out = Vec::new();
    walk(archive, key, &meter, Need::Names, &|_| true, &mut |item, _| {
        if out.len() >= most {
            return Err(trouble(Trouble::TooMany));
        }
        out.push(Entry {
            name: item.name.clone(),
            folder: matches!(item.kind, Kind::Folder),
            size: item.size,
            modified: item.modified,
            mode: item.mode,
            locked: item.locked,
        });
        Ok(true)
    })?;
    Ok(out)
}

pub fn inside(entries: &[Entry], folder: &str) -> Vec<Entry> {
    let base: Vec<&str> = parts(folder).collect();
    let mut out: BTreeMap<String, Entry> = BTreeMap::new();
    for e in entries {
        let p: Vec<&str> = parts(&e.name).collect();
        if p.len() <= base.len() || p[..base.len()] != base[..] {
            continue;
        }
        let name = p[..=base.len()].join("/");
        let made = || Entry { name: name.clone(), folder: true, size: 0, modified: None, mode: None, locked: false };
        if p.len() == base.len() + 1 {
            if e.folder {
                let slot = out.entry(name.clone()).or_insert_with(made);
                slot.folder = true;
                slot.modified = e.modified;
                slot.mode = e.mode;
            } else {
                out.insert(name.clone(), Entry { name, ..e.clone() });
            }
        } else {
            let slot = out.entry(name.clone()).or_insert_with(made);
            if !e.folder {
                slot.size += e.size;
            }
        }
    }
    out.into_values().collect()
}

fn parts(name: &str) -> impl Iterator<Item = &str> {
    name.split('/').filter(|p| !p.is_empty() && *p != ".")
}

pub fn read(archive: &Path, key: Option<&Key>, name: &str, limit: u64) -> io::Result<Vec<u8>> {
    let stop = AtomicBool::new(false);
    let quiet = |_: u64, _: u64| {};
    let meter = Meter::new(&stop, &quiet);
    let mut found = None;
    walk(archive, key, &meter, Need::Some, &|n| n == name, &mut |item, r| {
        if !matches!(item.kind, Kind::File) {
            return Ok(true);
        }
        let mut out = Vec::new();
        r.take(limit + 1).read_to_end(&mut out)?;
        if out.len() as u64 > limit {
            return Err(io::Error::new(io::ErrorKind::InvalidData, t!("folders-archive-inside-too-big")));
        }
        found = Some(out);
        Ok(false)
    })?;
    found.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, t!("folders-archive-inside-missing")))
}

const KEY_TRY_MOST: u64 = 1 << 20;

pub fn unlocks(archive: &Path, key: &Key) -> io::Result<()> {
    let stop = AtomicBool::new(false);
    let quiet = |_: u64, _: u64| {};
    let meter = Meter::new(&stop, &quiet);
    let mut f = |item: &Item, r: &mut dyn Read| {
        if !item.locked || !matches!(item.kind, Kind::File) {
            return Ok(true);
        }
        io::copy(&mut r.take(KEY_TRY_MOST), &mut io::sink())?;
        Ok(false)
    };
    let done = match find(archive)? {
        Format::Zip => walk_zip(archive, Some(key), &meter, Need::Some, &|_| true, &mut f),
        Format::SevenZ => walk_7z(archive, Some(key), &meter, Need::Some, &|_| true, &mut f),
        _ => Ok(()),
    };
    match done {
        Err(e) if trouble_of(&e) == Some(Trouble::Foreign) => Ok(()),
        other => other,
    }
}

pub(super) struct Item {
    pub name: String,
    pub kind: Kind,
    pub mode: Option<u32>,
    pub modified: Option<i64>,
    pub size: u64,
    pub locked: bool,
}

pub(super) enum Kind {
    File,
    Folder,
    Link(String),
    Hard(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Need {
    Names,
    All,
    Some,
}

pub(super) fn tidy(name: &str) -> String {
    let mut s = name;
    while let Some(rest) = s.strip_prefix("./") {
        s = rest;
    }
    s.trim_end_matches('/').to_string()
}
