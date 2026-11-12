//! Reading a 7z's headers, packed or locked or plain.

use std::fs::File;
use std::io::{self, Read};
use std::sync::atomic::AtomicBool;

use flate2::Crc;

use crate::files::archive::sevenz::{sz_bad, sz_locked, sz_ours, sz_sealed, sz_unknown, SzArchive, SzBytes, SzCoder, SzFile, SzFolder, SzStreams, SZ_COUNT_MAX, SZ_EPOCH, SZ_HEAD_MAX, SZ_MAGIC};
use crate::files::archive::sevenz::lock::SzKeys;
use crate::files::archive::sevenz::unpack::sz_unpack;
use crate::files::archive::trouble::{key_err, trouble, Trouble};
use crate::files::archive::zip::{read_at, u32_at, u64_at};
use crate::files::meter::Meter;
use crate::toolkit::t;

pub(super) fn sz_open(file: &File, keys: &SzKeys) -> io::Result<SzArchive> {
    let sealed = std::cell::Cell::new(false);
    sz_open_in(file, keys, &sealed).map_err(|e| if sealed.get() { key_err(e, Trouble::WrongKey) } else { e })
}

fn sz_open_in(file: &File, keys: &SzKeys, sealed: &std::cell::Cell<bool>) -> io::Result<SzArchive> {
    let Some(mut head) = header_bytes(file)? else {
        return Ok(SzArchive { streams: SzStreams::default(), files: Vec::new() });
    };
    for _ in 0..4 {
        let mut r = SzBytes::new(&head);
        match r.byte()? {
            0x01 => return sz_header(&mut r),
            0x17 => head = packed_header(file, &mut r, keys, sealed)?,
            _ => return Err(sz_bad(t!("folders-archive-7z-an-unknown-header"))),
        }
    }
    Err(sz_bad(t!("folders-archive-7z-headers-packed-too-many-times-over")))
}

fn header_bytes(file: &File) -> io::Result<Option<Vec<u8>>> {
    let len = file.metadata()?.len();
    if len < 32 {
        return Err(sz_bad(t!("folders-archive-7z-too-short")));
    }
    let start = read_at(file, 0, 32)?;
    if start[..6] != SZ_MAGIC {
        return Err(sz_bad(t!("folders-archive-7z-no-7z-mark")));
    }
    let mut crc = Crc::new();
    crc.update(&start[12..32]);
    if crc.sum() != u32_at(&start, 8) {
        return Err(sz_bad(t!("folders-archive-7z-the-start-header-s-check-does-not-match")));
    }
    let (offset, size, sum) = (u64_at(&start, 12), u64_at(&start, 20), u32_at(&start, 28));
    if size == 0 {
        return Ok(None);
    }
    let end = offset.checked_add(32).and_then(|a| a.checked_add(size));
    if size > SZ_HEAD_MAX || end.is_none_or(|e| e > len) {
        return Err(sz_bad(t!("folders-archive-7z-the-header-is-out-of-reach")));
    }
    let head = read_at(file, 32 + offset, size as usize)?;
    let mut crc = Crc::new();
    crc.update(&head);
    if crc.sum() != sum {
        return Err(sz_bad(t!("folders-archive-7z-the-header-s-check-does-not-match")));
    }
    Ok(Some(head))
}

fn packed_header(file: &File, r: &mut SzBytes, keys: &SzKeys, sealed: &std::cell::Cell<bool>) -> io::Result<Vec<u8>> {
    let s = sz_streams(r)?;
    let folder = s.folders.first().ok_or_else(|| sz_bad(t!("folders-archive-7z-a-packed-header-with-no-block")))?;
    if folder.size() > SZ_HEAD_MAX {
        return Err(sz_bad(t!("folders-archive-7z-the-header-is-far-too-big")));
    }
    if sz_sealed(folder) {
        if !sz_ours(folder) {
            return Err(trouble(Trouble::LockedForeign));
        }
        if keys.key.is_none() {
            return Err(sz_locked());
        }
        sealed.set(true);
    }
    let stop = AtomicBool::new(false);
    let quiet = |_: u64, _: u64| {};
    let meter = Meter::new(&stop, &quiet);
    let mut out = Vec::new();
    sz_unpack(file, &s, 0, keys, &meter)?.read_to_end(&mut out)?;
    let mut crc = Crc::new();
    crc.update(&out);
    if out.len() as u64 != folder.size() || folder.crc.is_some_and(|c| c != crc.sum()) {
        return Err(sz_bad(t!("folders-archive-7z-the-packed-header-s-check-does-not-match")));
    }
    Ok(out)
}

fn sz_header(r: &mut SzBytes) -> io::Result<SzArchive> {
    let mut id = r.byte()?;
    if id == 0x02 {
        while r.byte()? != 0 {
            let n = r.count()?;
            r.take(n)?;
        }
        id = r.byte()?;
    }
    if id == 0x03 {
        sz_streams(r)?;
        id = r.byte()?;
    }
    let mut streams = SzStreams::default();
    if id == 0x04 {
        streams = sz_streams(r)?;
        id = r.byte()?;
    }
    let mut files = Vec::new();
    if id == 0x05 {
        files = sz_files(r)?;
        id = r.byte()?;
    }
    if id != 0 {
        return Err(sz_bad(t!("folders-archive-7z-an-unknown-part-in-the-header")));
    }
    Ok(SzArchive { streams, files })
}

fn sz_streams(r: &mut SzBytes) -> io::Result<SzStreams> {
    let mut s = SzStreams::default();
    let mut subs = false;
    loop {
        match r.byte()? {
            0x00 => break,
            0x06 => sz_pack_info(r, &mut s)?,
            0x07 => sz_folders(r, &mut s)?,
            0x08 => {
                sz_substreams(r, &mut s)?;
                subs = true;
            }
            _ => return Err(sz_bad(t!("folders-archive-7z-an-unknown-part-in-the-streams"))),
        }
    }
    if !subs {
        s.counts = vec![1; s.folders.len()];
        s.sizes = s.folders.iter().map(SzFolder::size).collect();
        s.crcs = s.folders.iter().map(|f| f.crc).collect();
    }
    Ok(s)
}

fn sz_pack_info(r: &mut SzBytes, s: &mut SzStreams) -> io::Result<()> {
    s.pack_pos = r.number()?;
    let n = r.count()?;
    loop {
        match r.byte()? {
            0x00 => break,
            0x09 => s.pack_sizes = (0..n).map(|_| r.number()).collect::<io::Result<_>>()?,
            0x0a => {
                r.crcs(n)?;
            }
            _ => return Err(sz_bad(t!("folders-archive-7z-an-unknown-part-in-the-packed-streams"))),
        }
    }
    if s.pack_sizes.len() != n {
        return Err(sz_bad(t!("folders-archive-7z-packed-streams-with-no-sizes")));
    }
    Ok(())
}

fn sz_folders(r: &mut SzBytes, s: &mut SzStreams) -> io::Result<()> {
    if r.byte()? != 0x0b {
        return Err(sz_bad(t!("folders-archive-7z-no-list-of-blocks")));
    }
    let n = r.count()?;
    if r.byte()? != 0 {
        return Err(sz_bad(t!("folders-archive-7z-blocks-kept-elsewhere")));
    }
    for _ in 0..n {
        s.folders.push(sz_folder(r)?);
    }
    if r.byte()? != 0x0c {
        return Err(sz_bad(t!("folders-archive-7z-blocks-with-no-sizes")));
    }
    for f in &mut s.folders {
        let outs: usize = f.coders.iter().map(|c| c.outs).sum();
        f.sizes = (0..outs).map(|_| r.number()).collect::<io::Result<_>>()?;
    }
    loop {
        match r.byte()? {
            0x00 => break,
            0x0a => {
                for (f, c) in s.folders.iter_mut().zip(r.crcs(n)?) {
                    f.crc = c;
                }
            }
            _ => return Err(sz_bad(t!("folders-archive-7z-an-unknown-part-in-the-blocks"))),
        }
    }
    Ok(())
}

fn sz_folder(r: &mut SzBytes) -> io::Result<SzFolder> {
    let n = r.count()?;
    if n == 0 || n > 32 {
        return Err(sz_bad(t!("folders-archive-7z-a-block-with-no-sensible-coders")));
    }
    let mut coders = Vec::new();
    for _ in 0..n {
        let flags = r.byte()?;
        if flags & 0x80 != 0 {
            return Err(sz_unknown());
        }
        let id = r.take(usize::from(flags & 0x0f))?.to_vec();
        let (ins, outs) = if flags & 0x10 != 0 { (r.count()?, r.count()?) } else { (1, 1) };
        if ins > 32 || outs > 32 {
            return Err(sz_bad(t!("folders-archive-7z-a-coder-with-too-many-ends")));
        }
        let props = if flags & 0x20 != 0 {
            let len = r.count()?;
            r.take(len)?.to_vec()
        } else {
            Vec::new()
        };
        coders.push(SzCoder { id, props, ins, outs });
    }
    let ins: usize = coders.iter().map(|c| c.ins).sum();
    let outs: usize = coders.iter().map(|c| c.outs).sum();
    if outs == 0 || outs > ins + 1 {
        return Err(sz_bad(t!("folders-archive-7z-a-block-s-coders-do-not-join-up")));
    }
    let mut binds = Vec::new();
    for _ in 1..outs {
        let (i, o) = (r.count()?, r.count()?);
        if i >= ins || o >= outs {
            return Err(sz_bad(t!("folders-archive-7z-a-block-s-coders-do-not-join-up")));
        }
        binds.push((i, o));
    }
    let packs = ins - binds.len();
    let packed: Vec<usize> = if packs == 1 {
        (0..ins).filter(|i| !binds.iter().any(|b| b.0 == *i)).take(1).collect()
    } else {
        (0..packs).map(|_| r.count()).collect::<io::Result<_>>()?
    };
    if packed.is_empty() || packed.iter().any(|&p| p >= ins) {
        return Err(sz_bad(t!("folders-archive-7z-a-block-s-coders-do-not-join-up")));
    }
    Ok(SzFolder { coders, binds, packed, sizes: Vec::new(), crc: None })
}

fn sz_substreams(r: &mut SzBytes, s: &mut SzStreams) -> io::Result<()> {
    let mut counts = vec![1usize; s.folders.len()];
    let mut id = r.byte()?;
    if id == 0x0d {
        for c in counts.iter_mut() {
            *c = r.count()?;
        }
        id = r.byte()?;
    }
    if counts.iter().sum::<usize>() as u64 > SZ_COUNT_MAX {
        return Err(sz_bad(t!("folders-archive-7z-far-too-many-files")));
    }
    let mut sizes = Vec::new();
    for (f, &c) in s.folders.iter().zip(&counts) {
        if c == 0 {
            continue;
        }
        if c > 1 && id != 0x09 {
            return Err(sz_bad(t!("folders-archive-7z-files-in-a-block-with-no-sizes")));
        }
        let mut sum = 0u64;
        for _ in 1..c {
            let n = r.number()?;
            sum = sum.checked_add(n).ok_or_else(|| sz_bad(t!("folders-archive-7z-files-bigger-than-their-block")))?;
            sizes.push(n);
        }
        sizes.push(f.size().checked_sub(sum).ok_or_else(|| sz_bad(t!("folders-archive-7z-files-bigger-than-their-block")))?);
    }
    if id == 0x09 {
        id = r.byte()?;
    }
    let alone = |f: &SzFolder, c: usize| c == 1 && f.crc.is_some();
    let need: usize = s.folders.iter().zip(&counts).map(|(f, &c)| if alone(f, c) { 0 } else { c }).sum();
    let mut listed = Vec::new();
    loop {
        match id {
            0x00 => break,
            0x0a => listed = r.crcs(need)?,
            _ => return Err(sz_bad(t!("folders-archive-7z-an-unknown-part-in-the-files-sizes"))),
        }
        id = r.byte()?;
    }
    let mut listed = listed.into_iter();
    let mut crcs = Vec::new();
    for (f, &c) in s.folders.iter().zip(&counts) {
        if alone(f, c) {
            crcs.push(f.crc);
        } else {
            crcs.extend((0..c).map(|_| listed.next().flatten()));
        }
    }
    s.counts = counts;
    s.sizes = sizes;
    s.crcs = crcs;
    Ok(())
}

fn sz_files(r: &mut SzBytes) -> io::Result<Vec<SzFile>> {
    let n = r.count()?;
    if n > r.left() / 2 {
        return Err(sz_bad(t!("folders-archive-7z-a-count-is-far-too-big")));
    }
    let mut files: Vec<SzFile> =
        (0..n).map(|_| SzFile { name: String::new(), stream: true, folder: false, anti: false, modified: None, attrib: None }).collect();
    let mut empty_file = Vec::new();
    let mut anti = Vec::new();
    let mut empties = 0;
    loop {
        let kind = r.byte()?;
        if kind == 0 {
            break;
        }
        let len = usize::try_from(r.number()?).map_err(|_| sz_bad(t!("folders-archive-7z-a-part-is-far-too-big")))?;
        let mut p = SzBytes::new(r.take(len)?);
        match kind {
            0x0e => {
                for (f, e) in files.iter_mut().zip(p.bits(n)?) {
                    f.stream = !e;
                }
                empties = files.iter().filter(|f| !f.stream).count();
            }
            0x0f => empty_file = p.bits(empties)?,
            0x10 => anti = p.bits(empties)?,
            0x11 => sz_names(&mut p, &mut files)?,
            0x14 => sz_times(&mut p, &mut files)?,
            0x15 => sz_attribs(&mut p, &mut files)?,
            _ => {}
        }
    }
    for (k, f) in files.iter_mut().filter(|f| !f.stream).enumerate() {
        f.folder = !empty_file.get(k).copied().unwrap_or(false);
        f.anti = anti.get(k).copied().unwrap_or(false);
    }
    Ok(files)
}

fn sz_names(p: &mut SzBytes, files: &mut [SzFile]) -> io::Result<()> {
    if p.byte()? != 0 {
        return Err(sz_bad(t!("folders-archive-7z-names-kept-elsewhere")));
    }
    let rest = p.take(p.left())?;
    let units: Vec<u16> = rest.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let mut names = units.split(|&u| u == 0);
    for f in files.iter_mut() {
        f.name = String::from_utf16_lossy(names.next().ok_or_else(|| sz_bad(t!("folders-archive-7z-too-few-names")))?);
    }
    Ok(())
}

fn sz_times(p: &mut SzBytes, files: &mut [SzFile]) -> io::Result<()> {
    let have = p.defined(files.len())?;
    if p.byte()? != 0 {
        return Err(sz_bad(t!("folders-archive-7z-times-kept-elsewhere")));
    }
    for (f, h) in files.iter_mut().zip(have) {
        if h {
            f.modified = i64::try_from(p.u64()? / 10_000_000).ok().map(|s| s - SZ_EPOCH);
        }
    }
    Ok(())
}

fn sz_attribs(p: &mut SzBytes, files: &mut [SzFile]) -> io::Result<()> {
    let have = p.defined(files.len())?;
    if p.byte()? != 0 {
        return Err(sz_bad(t!("folders-archive-7z-attributes-kept-elsewhere")));
    }
    for (f, h) in files.iter_mut().zip(have) {
        if h {
            f.attrib = Some(p.u32()?);
        }
    }
    Ok(())
}
