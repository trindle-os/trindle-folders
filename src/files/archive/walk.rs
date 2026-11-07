// Going through an archive of any kind, handing each thing and its bytes on.

use std::io::{self, BufReader, Read};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::files::unarchive;
use crate::files::archive::{find, opened, stem, tidy, unsqueeze, Format, Item, Kind, Need};
use crate::files::archive::sevenz::unpack::walk_7z;
use crate::files::archive::trouble::{trouble, trouble_of, Key, Trouble};
use crate::files::archive::zip::dir::{zip_data, zip_dir};
use crate::files::meter::{Counted, Meter};

pub(super) fn walk(
    archive: &Path,
    key: Option<&Key>,
    meter: &Meter,
    need: Need,
    want: &dyn Fn(&str) -> bool,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    let format = find(archive)?;
    let mut begun = false;
    let done = {
        let mut g = |item: &Item, r: &mut dyn Read| {
            begun = true;
            f(item, r)
        };
        match format {
            Format::Zip => walk_zip(archive, key, meter, need, want, &mut g),
            Format::SevenZ => walk_7z(archive, key, meter, need, want, &mut g),
            Format::Gz | Format::Bz2 | Format::Xz => walk_single(archive, format, meter, need, want, &mut g),
            Format::Other => walk_boxed(archive, meter, need, want, &mut g),
            _ => walk_tar(archive, format, meter, want, &mut g),
        }
    };
    let done = match done {
        Err(e) if !begun && trouble_of(&e) == Some(Trouble::Foreign) => walk_boxed(archive, meter, need, want, f),
        other => other,
    };
    done.map_err(|e| meter.check().err().unwrap_or(e))
}

fn walk_boxed(
    archive: &Path,
    meter: &Meter,
    need: Need,
    want: &dyn Fn(&str) -> bool,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    unarchive::walk(archive, meter, need, want, f).map_err(|e| {
        if trouble_of(&e) != Some(Trouble::Locked) {
            return e;
        }
        let mut mark = [0u8; 4];
        let rar = opened(archive).and_then(|mut f| f.read_exact(&mut mark)).is_ok() && mark == *b"Rar!";
        trouble(if rar { Trouble::LockedRar } else { Trouble::LockedForeign })
    })
}

const SINGLE_COUNT_MOST: u64 = 256 << 20;

fn walk_single(
    archive: &Path,
    format: Format,
    meter: &Meter,
    need: Need,
    want: &dyn Fn(&str) -> bool,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    let name = stem(archive);
    if !want(&name) {
        return Ok(());
    }
    let open = || opened(archive).map(|file| BufReader::with_capacity(1 << 16, file));
    let meta = opened(archive)?.metadata()?;
    let size = if need == Need::Names {
        let quiet = meter.hushed();
        let n = io::copy(&mut unsqueeze(format, Counted::new(open()?, &quiet)).take(SINGLE_COUNT_MOST + 1), &mut io::sink())?;
        if n > SINGLE_COUNT_MOST { meta.len() } else { n }
    } else {
        0
    };
    meter.total(meta.len());
    let item = Item { name, kind: Kind::File, mode: None, modified: Some(meta.mtime()), size, locked: false };
    f(&item, &mut unsqueeze(format, Counted::new(open()?, meter)))?;
    meter.end();
    Ok(())
}

pub(super) fn walk_zip(
    archive: &Path,
    key: Option<&Key>,
    meter: &Meter,
    need: Need,
    want: &dyn Fn(&str) -> bool,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    let file = opened(archive)?;
    let dir = zip_dir(&file)?;
    if dir.iter().any(|e| !e.folder && !e.sealed() && !e.ours() && want(&e.name)) {
        return Err(trouble(Trouble::Foreign));
    }
    if need != Need::Names {
        let mut sealed = dir.iter().filter(|e| e.sealed() && want(&e.name)).peekable();
        if sealed.peek().is_some() {
            if sealed.any(|e| !e.ours() || !e.lock_known()) {
                return Err(trouble(Trouble::LockedForeign));
            }
            if key.is_none() {
                return Err(trouble(Trouble::Locked));
            }
        }
    }
    meter.total(dir.iter().filter(|e| want(&e.name)).map(|e| e.packed).sum());
    for e in &dir {
        if !want(&e.name) {
            continue;
        }
        meter.check()?;
        let shut = need == Need::Names;
        let kind = if e.folder {
            Kind::Folder
        } else if e.link && shut && e.sealed() {
            Kind::Link(String::new())
        } else if e.link {
            let mut target = Vec::new();
            zip_data(&file, e, key, meter)?.take(4096).read_to_end(&mut target)?;
            Kind::Link(String::from_utf8_lossy(&target).into_owned())
        } else {
            Kind::File
        };
        let item = Item { name: e.name.clone(), kind, mode: e.mode, modified: e.modified, size: e.size, locked: e.sealed() };
        let go = if matches!(item.kind, Kind::File) && !shut { f(&item, &mut zip_data(&file, e, key, meter)?)? } else { f(&item, &mut io::empty())? };
        if !go {
            break;
        }
    }
    meter.end();
    Ok(())
}

fn walk_tar(
    archive: &Path,
    format: Format,
    meter: &Meter,
    want: &dyn Fn(&str) -> bool,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    let file = opened(archive)?;
    meter.total(file.metadata()?.len());
    let raw = Counted::new(BufReader::with_capacity(1 << 16, file), meter);
    let mut tar = tar::Archive::new(unsqueeze(format, raw));
    for entry in tar.entries()? {
        let mut entry = entry?;
        let name = tidy(&String::from_utf8_lossy(&entry.path_bytes()));
        let header = entry.header();
        let et = header.entry_type();
        let target = || entry.link_name_bytes().map(|t| String::from_utf8_lossy(&t).into_owned()).unwrap_or_default();
        let kind = if et.is_dir() {
            Kind::Folder
        } else if et.is_symlink() {
            Kind::Link(target())
        } else if et.is_hard_link() {
            Kind::Hard(tidy(&target()))
        } else if et.is_file() || et.is_contiguous() || et.is_gnu_sparse() {
            Kind::File
        } else {
            continue;
        };
        if !want(&name) {
            continue;
        }
        let item = Item {
            name,
            mode: header.mode().ok(),
            modified: header.mtime().ok().map(|t| t as i64),
            size: if matches!(kind, Kind::File) { entry.size() } else { 0 },
            kind,
            locked: false,
        };
        if !f(&item, &mut entry)? {
            break;
        }
    }
    meter.end();
    Ok(())
}
