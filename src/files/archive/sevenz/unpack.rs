// Unpacking the blocks of a 7z and walking its files.

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use flate2::Crc;

use crate::files::archive::{opened, tidy, Item, Kind, Need};
use crate::files::archive::sevenz::{sz_bad, sz_locked, sz_ours, sz_sealed, sz_unknown, SzArchive, SzCoder, SzFile, SzStreams, SZ_AES, SZ_DICT_MAX};
use crate::files::archive::sevenz::header::sz_open;
use crate::files::archive::sevenz::lock::{SzAes, SzKeys};
use crate::files::archive::trouble::{key_err, trouble, Key, Trouble};
use crate::files::archive::zip::{u32_at, S_IFLNK, S_IFMT};
use crate::files::meter::{Counted, Meter};
use crate::toolkit::t;

pub(in crate::files::archive) fn sz_unpack<'a>(file: &'a File, s: &SzStreams, index: usize, keys: &SzKeys, meter: &'a Meter<'a>) -> io::Result<Box<dyn Read + 'a>> {
    let folder = &s.folders[index];
    if folder.packed.len() != 1 || folder.coders.iter().any(|c| c.ins != 1 || c.outs != 1) {
        return Err(sz_unknown());
    }
    let (at, size) = s.packed(index)?;
    let len = file.metadata()?.len();
    if at.checked_add(size).is_none_or(|e| e > len) {
        return Err(sz_bad(t!("folders-archive-7z-a-block-runs-past-the-end")));
    }
    let mut f = file;
    f.seek(SeekFrom::Start(at))?;
    let mut reader: Box<dyn Read + 'a> = Box::new(BufReader::with_capacity(1 << 16, Counted::new(f.take(size), meter)));
    let mut coder = folder.packed[0];
    let mut memory = SZ_DICT_MAX;
    for _ in 0..folder.coders.len() {
        let c = folder.coders.get(coder).ok_or_else(|| sz_bad(t!("folders-archive-7z-a-block-s-coders-do-not-join-up")))?;
        let out = *folder.sizes.get(coder).ok_or_else(|| sz_bad(t!("folders-archive-7z-a-block-s-coders-do-not-join-up")))?;
        reader = sz_coder(c, reader, out, keys, &mut memory)?;
        match folder.binds.iter().find(|b| b.1 == coder) {
            Some(b) => coder = b.0,
            None => return Ok(Box::new(reader.take(folder.size()))),
        }
    }
    Err(sz_bad(t!("folders-archive-7z-a-block-s-coders-go-round-in-a-loop")))
}

fn sz_coder<'a>(c: &SzCoder, inner: Box<dyn Read + 'a>, size: u64, keys: &SzKeys, memory: &mut u64) -> io::Result<Box<dyn Read + 'a>> {
    let mut room = |dict: u64| {
        if dict > *memory {
            Err(io::Error::new(io::ErrorKind::OutOfMemory, t!("folders-archive-7z-memory")))
        } else {
            *memory -= dict;
            Ok(())
        }
    };
    let reader: Box<dyn Read + 'a> = match c.id.as_slice() {
        [0x00] => inner,
        [0x03, 0x01, 0x01] => {
            if c.props.len() < 5 {
                return Err(sz_bad(t!("folders-archive-7z-lzma-with-no-settings")));
            }
            let dict = u32_at(&c.props, 1);
            room(u64::from(dict).min(size))?;
            Box::new(lzma_rust2::LzmaReader::new_with_props(inner, size, c.props[0], dict, None)?)
        }
        [0x21] => {
            let bits = *c.props.first().ok_or_else(|| sz_bad(t!("folders-archive-7z-lzma2-with-no-settings")))?;
            if bits > 40 {
                return Err(sz_bad(t!("folders-archive-7z-lzma2-with-impossible-settings")));
            }
            let dict = if bits == 40 { u32::MAX } else { (2 | u32::from(bits & 1)) << (bits / 2 + 11) };
            let dict = u64::from(dict).min(size.max(4096));
            room(dict)?;
            Box::new(lzma_rust2::Lzma2Reader::new(inner, dict as u32, None))
        }
        [0x03, 0x03, 0x01, 0x03] => Box::new(lzma_rust2::filter::bcj::BcjReader::new_x86(inner, 0)),
        [0x03] => {
            let distance = usize::from(c.props.first().copied().unwrap_or(0)) + 1;
            Box::new(lzma_rust2::filter::delta::DeltaReader::new(inner, distance))
        }
        id if id == SZ_AES => Box::new(SzAes::open(inner, &c.props, keys)?.take(size)),
        _ => return Err(sz_unknown()),
    };
    Ok(reader)
}

struct SzOpen<'a> {
    folder: usize,
    reader: Box<dyn Read + 'a>,
    pos: u64,
}

struct SzCursor<'a> {
    file: &'a File,
    streams: &'a SzStreams,
    keys: &'a SzKeys<'a>,
    meter: &'a Meter<'a>,
    open: Option<SzOpen<'a>>,
}

impl<'a> SzCursor<'a> {
    fn at(&mut self, folder: usize, offset: u64) -> io::Result<&mut SzOpen<'a>> {
        let fresh = self.open.as_ref().is_none_or(|o| o.folder != folder || o.pos > offset);
        if fresh {
            self.open = None;
            let reader = sz_unpack(self.file, self.streams, folder, self.keys, self.meter)?;
            self.open = Some(SzOpen { folder, reader, pos: 0 });
        }
        let open = self.open.as_mut().ok_or_else(|| sz_bad(t!("folders-archive-7z-a-block-would-not-open")))?;
        if open.pos < offset {
            let skip = offset - open.pos;
            let got = io::copy(&mut open.reader.by_ref().take(skip), &mut io::sink())?;
            open.pos += got;
            if got < skip {
                return Err(sz_bad(t!("folders-archive-7z-a-block-ends-too-soon")));
            }
        }
        Ok(open)
    }
}

struct SzPart<'c, 'a> {
    cur: &'c mut SzCursor<'a>,
    folder: usize,
    start: u64,
    size: u64,
    sum: Option<u32>,
    crc: Crc,
    got: u64,
    sealed: bool,
}

impl Read for SzPart<'_, '_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let sealed = self.sealed;
        self.pull(buf).map_err(|e| if sealed { key_err(e, Trouble::WrongOrBad) } else { e })
    }
}

impl SzPart<'_, '_> {
    fn pull(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let room = (self.size - self.got).min(buf.len() as u64) as usize;
        let n = if room == 0 {
            0
        } else {
            let open = self.cur.at(self.folder, self.start + self.got)?;
            let n = open.reader.read(&mut buf[..room])?;
            open.pos += n as u64;
            n
        };
        if n == 0 && !buf.is_empty() {
            if self.got != self.size {
                return Err(sz_bad(t!("folders-archive-7z-a-file-ends-too-soon")));
            }
            if self.sum.is_some_and(|s| s != self.crc.sum()) {
                return Err(sz_bad(t!("folders-archive-7z-a-file-s-check-does-not-match")));
            }
            return Ok(0);
        }
        self.crc.update(&buf[..n]);
        self.got += n as u64;
        Ok(n)
    }
}

pub(in crate::files::archive) fn walk_7z(
    archive: &Path,
    key: Option<&Key>,
    meter: &Meter,
    need: Need,
    want: &dyn Fn(&str) -> bool,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    let file = opened(archive)?;
    let keys = SzKeys::new(key);
    let sz = sz_open(&file, &keys)?;
    let s = &sz.streams;
    if s.sizes.len() != s.crcs.len() {
        return Err(sz_bad(t!("folders-archive-7z-files-sizes-and-checks-do-not-match-up")));
    }
    let places = places(&sz)?;
    meter.total(packed_to_read(&sz, &places, need, key, want)?);
    let mut cur = SzCursor { file: &file, streams: s, keys: &keys, meter, open: None };
    for (file, place) in sz.files.iter().zip(&places) {
        let name = name_of(file);
        if file.anti || !want(&name) {
            continue;
        }
        meter.check()?;
        if !give(&mut cur, file, name, *place, need, f)? {
            break;
        }
    }
    meter.end();
    Ok(())
}

type Spot = (usize, u64, u64, Option<u32>);

fn name_of(file: &SzFile) -> String {
    tidy(&file.name.replace('\\', "/"))
}

fn places(sz: &SzArchive) -> io::Result<Vec<Option<Spot>>> {
    let s = &sz.streams;
    let mut places = Vec::new();
    let (mut folder, mut left, mut stream, mut pos) = (0usize, 0usize, 0usize, 0u64);
    for file in &sz.files {
        if !file.stream {
            places.push(None);
            continue;
        }
        while left == 0 {
            left = *s.counts.get(folder).ok_or_else(|| sz_bad(t!("folders-archive-7z-more-files-than-blocks-hold")))?;
            folder += 1;
            pos = 0;
        }
        let size = *s.sizes.get(stream).ok_or_else(|| sz_bad(t!("folders-archive-7z-more-files-than-sizes")))?;
        places.push(Some((folder - 1, pos, size, s.crcs[stream])));
        left -= 1;
        stream += 1;
        pos += size;
    }
    Ok(places)
}

fn packed_to_read(sz: &SzArchive, places: &[Option<Spot>], need: Need, key: Option<&Key>, want: &dyn Fn(&str) -> bool) -> io::Result<u64> {
    let s = &sz.streams;
    let mut needed = vec![false; s.folders.len()];
    for (file, place) in sz.files.iter().zip(places) {
        if let Some((fo, ..)) = place {
            if !file.anti && want(&name_of(file)) {
                needed[*fo] = true;
            }
        }
    }
    let mut total = 0u64;
    for (i, _) in needed.iter().enumerate().filter(|(_, &n)| n) {
        if sz_sealed(&s.folders[i]) {
            if need != Need::Names {
                if !sz_ours(&s.folders[i]) {
                    return Err(trouble(Trouble::LockedForeign));
                }
                if key.is_none() {
                    return Err(sz_locked());
                }
            }
        } else if !sz_ours(&s.folders[i]) {
            return Err(sz_unknown());
        }
        total = total.saturating_add(s.packed(i)?.1);
    }
    Ok(total)
}

fn give(cur: &mut SzCursor, file: &SzFile, name: String, place: Option<Spot>, need: Need, f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>) -> io::Result<bool> {
    let mode = file.attrib.filter(|a| a & 0x8000 != 0).map(|a| a >> 16);
    let modified = file.modified;
    let Some((folder, start, size, sum)) = place else {
        let kind = if file.folder || file.attrib.is_some_and(|a| a & 0x10 != 0) { Kind::Folder } else { Kind::File };
        return f(&Item { name, kind, mode, modified, size: 0, locked: false }, &mut io::empty());
    };
    let locked = sz_sealed(&cur.streams.folders[folder]);
    let mut part = SzPart { cur, folder, start, size, sum, crc: Crc::new(), got: 0, sealed: locked };
    if mode.is_some_and(|m| m & S_IFMT == S_IFLNK) {
        let mut target = Vec::new();
        if !(locked && need == Need::Names) {
            (&mut part).take(4096).read_to_end(&mut target)?;
        }
        let kind = Kind::Link(String::from_utf8_lossy(&target).into_owned());
        f(&Item { name, kind, mode, modified, size: 0, locked }, &mut io::empty())
    } else {
        f(&Item { name, kind: Kind::File, mode, modified, size, locked }, &mut part)
    }
}
