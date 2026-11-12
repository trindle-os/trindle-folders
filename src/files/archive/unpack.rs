//! Unpacking an archive, the one place the rules for a stranger's files are kept.

use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::{self, File, FileTimes, OpenOptions};
use std::io::{self, Read};
use std::os::unix::fs::{symlink, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::common::gentle;
use crate::files::archive::{parts, stem, Item, Kind, Need, MOST_THINGS, ROOM_KEPT};
use crate::files::archive::pack::{put_new, temp_name};
use crate::files::archive::trouble::{trouble, Key, Trouble};
use crate::files::archive::walk::walk;
use crate::files::meter::Meter;
use crate::files::work::free_name;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    Here,
    Folder,
}

#[derive(Debug, Default)]
pub struct Unpacked {
    pub made: Vec<PathBuf>,
    pub refused: Vec<String>,
}

pub fn extract(archive: &Path, key: Option<&Key>, into: &Path, place: Place, stop: &AtomicBool, tell: &dyn Fn(u64, u64)) -> io::Result<Unpacked> {
    let roots = if place == Place::Here { Roots::Here } else { Roots::Together };
    unpack(archive, key, into, Taking { need: Need::All, want: &|_| true, roots, most: Most::here(into) }, stop, tell)
}

pub fn take_out(archive: &Path, key: Option<&Key>, names: &[String], into: &Path, stop: &AtomicBool, tell: &dyn Fn(u64, u64)) -> io::Result<Unpacked> {
    let mut chosen: Vec<Vec<String>> = names.iter().filter_map(|n| clean(n)).filter(|c| !c.is_empty()).collect();
    chosen.sort();
    chosen.dedup();
    let all = chosen.clone();
    chosen.retain(|c| !all.iter().any(|o| o.len() < c.len() && c.starts_with(o)));
    let want = |name: &str| clean(name).is_some_and(|c| chosen.iter().any(|r| c.starts_with(r)));
    unpack(archive, key, into, Taking { need: Need::Some, want: &want, roots: Roots::Chosen(chosen.clone()), most: Most::here(into) }, stop, tell)
}

#[derive(Clone, Copy)]
pub(super) struct Most {
    pub(super) things: usize,
    pub(super) bytes: u64,
}

impl Most {
    pub(super) fn here(into: &Path) -> Most {
        let free = crate::common::files::room_left(into).unwrap_or(u64::MAX);
        Most { things: MOST_THINGS, bytes: free.saturating_sub(ROOM_KEPT) }
    }
}

struct Rationed<'a, R> {
    inner: R,
    left: &'a mut u64,
}

impl<R: Read> Read for Rationed<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        *self.left = self.left.checked_sub(n as u64).ok_or_else(|| trouble(Trouble::NoRoom))?;
        Ok(n)
    }
}

pub(super) enum Roots {
    Here,
    Together,
    Chosen(Vec<Vec<String>>),
}

fn clean(name: &str) -> Option<Vec<String>> {
    if name.starts_with('/') || name.contains('\0') {
        return None;
    }
    let mut out = Vec::new();
    for p in parts(name) {
        if p == ".." {
            return None;
        }
        out.push(p.to_string());
    }
    Some(out)
}

fn joined(base: &Path, comps: &[String]) -> PathBuf {
    comps.iter().fold(base.to_path_buf(), |p, c| p.join(c))
}

pub(super) fn room(base: &Path, dirs: &[String]) -> io::Result<bool> {
    let mut p = base.to_path_buf();
    for d in dirs {
        p.push(d);
        match fs::symlink_metadata(&p) {
            Ok(m) if m.file_type().is_dir() => {}
            Ok(_) => return Ok(false),
            Err(e) if e.kind() == io::ErrorKind::NotFound => fs::create_dir(&p)?,
            Err(e) => return Err(e),
        }
    }
    Ok(true)
}

pub(super) fn time_of(secs: i64) -> SystemTime {
    if secs >= 0 { UNIX_EPOCH + Duration::from_secs(secs as u64) } else { UNIX_EPOCH - Duration::from_secs(secs.unsigned_abs()) }
}

pub(super) struct Taking<'a> {
    pub(super) need: Need,
    pub(super) want: &'a dyn Fn(&str) -> bool,
    pub(super) roots: Roots,
    pub(super) most: Most,
}

pub(super) fn unpack(archive: &Path, key: Option<&Key>, into: &Path, taking: Taking, stop: &AtomicBool, tell: &dyn Fn(u64, u64)) -> io::Result<Unpacked> {
    let meter = Meter::new(stop, tell);
    let temp = loop {
        let t = temp_name(into, "unpack");
        match fs::create_dir(&t) {
            Ok(()) => break t,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    };
    match unpack_in(archive, key, into, &temp, taking, &meter) {
        Ok(u) => Ok(u),
        Err(e) => {
            let _ = fs::remove_dir_all(&temp);
            Err(meter.check().err().unwrap_or(e))
        }
    }
}

fn unpack_in(archive: &Path, key: Option<&Key>, into: &Path, temp: &Path, taking: Taking, meter: &Meter) -> io::Result<Unpacked> {
    let Taking { need, want, roots, most } = taking;
    let mut laid = Laid::new(temp, most.bytes);
    let mut things = 0usize;
    walk(archive, key, meter, need, want, &mut |item, r| {
        things += 1;
        if things > most.things {
            return Err(trouble(Trouble::TooMany));
        }
        laid.take(item, r)?;
        Ok(true)
    })?;
    if let Some(lost) = laid.held.settle(&|_| {}).into_iter().next() {
        return Err(lost.why);
    }
    meter.check()?;

    let chosen = match roots {
        Roots::Here if laid.tops.len() == 1 => Some(vec![vec![laid.tops[0].clone()]]),
        Roots::Here | Roots::Together => None,
        Roots::Chosen(c) => Some(c),
    };
    laid.put_links(chosen.as_deref())?;
    let Laid { refused, folders, tops, .. } = laid;
    let (made, placed) = place(archive, into, temp, chosen, tops.is_empty())?;
    set_folder_marks(folders, &placed);
    meter.end();
    Ok(Unpacked { made, refused })
}

type Folder = (Vec<String>, Option<u32>, Option<i64>);

type Placed = Vec<(Vec<String>, PathBuf)>;

struct Laid<'t> {
    temp: &'t Path,
    room_left: u64,
    held: gentle::Held<File>,
    refused: Vec<String>,
    folders: Vec<Folder>,
    links: Vec<(Vec<String>, String, String)>,
    files: HashSet<Vec<String>>,
    tops: Vec<String>,
}

impl<'t> Laid<'t> {
    fn new(temp: &'t Path, room_left: u64) -> Laid<'t> {
        Laid { temp, room_left, held: gentle::Held::default(), refused: Vec::new(), folders: Vec::new(), links: Vec::new(), files: HashSet::new(), tops: Vec::new() }
    }

    fn take(&mut self, item: &Item, r: &mut dyn Read) -> io::Result<()> {
        let Some(comps) = clean(&item.name) else {
            self.refused.push(item.name.clone());
            return Ok(());
        };
        if comps.is_empty() {
            return Ok(());
        }
        let fits = match &item.kind {
            Kind::Folder => self.folder(&comps, item)?,
            Kind::File => self.file(&comps, item, r)?,
            Kind::Hard(target) => self.hard(&comps, target)?,
            Kind::Link(target) => {
                self.links.push((comps.clone(), target.clone(), item.name.clone()));
                true
            }
        };
        if fits {
            self.note_top(&comps);
        } else {
            self.refused.push(item.name.clone());
        }
        Ok(())
    }

    fn note_top(&mut self, comps: &[String]) {
        if !self.tops.contains(&comps[0]) {
            self.tops.push(comps[0].clone());
        }
    }

    fn folder(&mut self, comps: &[String], item: &Item) -> io::Result<bool> {
        let fits = room(self.temp, comps)?;
        if fits {
            self.folders.push((comps.to_vec(), item.mode, item.modified));
        }
        Ok(fits)
    }

    fn file(&mut self, comps: &[String], item: &Item, r: &mut dyn Read) -> io::Result<bool> {
        let path = joined(self.temp, comps);
        let fits = room(self.temp, &comps[..comps.len() - 1])? && make_way(&path)?;
        if fits {
            let mut out = gentle::Out::new(OpenOptions::new().write(true).create_new(true).mode(0o666).open(&path)?, &|_| {});
            gentle::pour(&mut Rationed { inner: r, left: &mut self.room_left }, &mut out, &mut || true)?;
            if let Some(m) = item.mode {
                out.file().set_permissions(fs::Permissions::from_mode(m & 0o777 & !umask()))?;
            }
            if let Some(t) = item.modified {
                out.file().set_times(FileTimes::new().set_modified(time_of(t)))?;
            }
            out.finish(&self.held, ())?;
            self.files.insert(comps.to_vec());
        }
        Ok(fits)
    }

    fn hard(&mut self, comps: &[String], target: &str) -> io::Result<bool> {
        let path = joined(self.temp, comps);
        let to = clean(target).filter(|t| self.files.contains(t));
        let fits = to.is_some() && room(self.temp, &comps[..comps.len() - 1])? && make_way(&path)?;
        if let (true, Some(to)) = (fits, to) {
            fs::hard_link(joined(self.temp, &to), &path)?;
            self.files.insert(comps.to_vec());
        }
        Ok(fits)
    }

    fn put_links(&mut self, chosen: Option<&[Vec<String>]>) -> io::Result<()> {
        for (comps, target, name) in std::mem::take(&mut self.links) {
            let root = chosen.and_then(|c| c.iter().filter(|r| comps.starts_with(r)).map(Vec::len).max()).unwrap_or(0);
            let climb = (comps.len() as isize - 1 - root as isize).max(0) as usize;
            let path = joined(self.temp, &comps);
            let fits = link_stays_in(&target, climb) && room(self.temp, &comps[..comps.len() - 1])? && make_way(&path)?;
            if fits {
                symlink(&target, &path)?;
                self.note_top(&comps);
            } else {
                self.refused.push(name);
            }
        }
        Ok(())
    }
}

fn place(archive: &Path, into: &Path, temp: &Path, chosen: Option<Vec<Vec<String>>>, empty: bool) -> io::Result<(Vec<PathBuf>, Placed)> {
    let mut made = Vec::new();
    let mut placed: Placed = Vec::new();
    match chosen {
        None if empty => fs::remove_dir_all(temp)?,
        None => {
            let to = put_folder(temp, into, &stem(archive))?;
            placed.push((Vec::new(), to.clone()));
            made.push(to);
        }
        Some(chosen) => {
            for r in chosen {
                let from = joined(temp, &r);
                if fs::symlink_metadata(&from).is_err() {
                    continue;
                }
                match put(&from, into, &r[r.len() - 1]) {
                    Ok(to) => {
                        placed.push((r, to.clone()));
                        made.push(to);
                    }
                    Err(e) => {
                        for (r, to) in placed.iter().rev() {
                            let _ = crate::common::files::rename_new(to, &joined(temp, r));
                        }
                        return Err(e);
                    }
                }
            }
            fs::remove_dir_all(temp)?;
        }
    }
    Ok((made, placed))
}

pub(super) fn umask() -> u32 {
    static MASK: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *MASK.get_or_init(|| {
        fs::read_to_string("/proc/self/status").ok()
            .and_then(|s| s.lines().find_map(|l| l.strip_prefix("Umask:")).and_then(|m| u32::from_str_radix(m.trim(), 8).ok()))
            .unwrap_or(0o022)
    })
}

fn set_folder_marks(mut folders: Vec<Folder>, placed: &Placed) {
    folders.sort_by_key(|f| std::cmp::Reverse(f.0.len()));
    for (comps, mode, modified) in folders {
        let Some((root, to)) = placed.iter().filter(|(r, _)| comps.starts_with(r)).max_by_key(|(r, _)| r.len()) else { continue };
        let path = joined(to, &comps[root.len()..]);
        let Ok(f) = OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW | libc::O_DIRECTORY).open(&path) else { continue };
        if let Some(m) = mode {
            let _ = f.set_permissions(fs::Permissions::from_mode((m & 0o777 & !umask()) | 0o700));
        }
        if let Some(t) = modified {
            let _ = f.set_times(FileTimes::new().set_modified(time_of(t)));
        }
    }
}

fn make_way(path: &Path) -> io::Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(m) if m.file_type().is_dir() => Ok(false),
        Ok(_) => fs::remove_file(path).map(|()| true),
        Err(_) => Ok(true),
    }
}

fn link_stays_in(target: &str, climb: usize) -> bool {
    if target.is_empty() || target.starts_with('/') || target.contains('\0') {
        return false;
    }
    let mut ups = 0;
    let mut down = false;
    for p in parts(target) {
        if p == ".." {
            if down {
                return false;
            }
            ups += 1;
        } else {
            down = true;
        }
    }
    ups <= climb
}

pub(super) fn free(dir: &Path, name: &str, folder: bool) -> PathBuf {
    let p = dir.join(name);
    if fs::symlink_metadata(&p).is_err() { p } else { free_name(dir, OsStr::new(name), folder) }
}

fn put_folder(from: &Path, into: &Path, name: &str) -> io::Result<PathBuf> {
    loop {
        let to = free(into, name, true);
        match crate::common::files::rename_new(from, &to) {
            Ok(()) => return Ok(to),
            Err(e) if fs::symlink_metadata(&to).is_ok() && e.kind() != io::ErrorKind::NotFound => continue,
            Err(e) => return Err(e),
        }
    }
}

fn put(from: &Path, into: &Path, name: &str) -> io::Result<PathBuf> {
    #[cfg(test)]
    if name.contains("cannot-be-placed") {
        return Err(io::Error::other("made to fail"));
    }
    if fs::symlink_metadata(from)?.file_type().is_dir() {
        return put_folder(from, into, name);
    }
    loop {
        let to = free(into, name, false);
        match put_new(from, &to) {
            Ok(()) => return Ok(to),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
}
