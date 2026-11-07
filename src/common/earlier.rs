// Earlier versions of a file or folder, read from the snapshots of the home folder (common/home_snapshots.rs), for Folders to offer.

use std::fs::Metadata;
use std::io::Read;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Component, Path, PathBuf};

use crate::common::home_snapshots::{self, Kept};

pub const BEFORE: &str = " (before putting back)";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    File,
    Folder,
    Link,
}

fn kind_of(m: &Metadata) -> Option<Kind> {
    let t = m.file_type();
    if t.is_symlink() {
        Some(Kind::Link)
    } else if t.is_dir() {
        Some(Kind::Folder)
    } else if t.is_file() {
        Some(Kind::File)
    } else {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub number: u32,
    pub taken: i64,
    pub path: PathBuf,
    pub kind: Kind,
    pub size: u64,
    pub modified: i64,
    pub differs: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gone {
    pub name: String,
    pub last: Version,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PutBack {
    pub put: PathBuf,
    pub kept: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct Store {
    live: PathBuf,
    snapshots: PathBuf,
}

#[derive(PartialEq, Eq)]
enum Mark {
    File(u64, i64, i64),
    Link(PathBuf),
    Folder,
}

fn mark(path: &Path, m: &Metadata) -> Option<Mark> {
    Some(match kind_of(m)? {
        Kind::File => Mark::File(m.len(), m.mtime(), m.mtime_nsec()),
        Kind::Link => Mark::Link(std::fs::read_link(path).ok()?),
        Kind::Folder => Mark::Folder,
    })
}

impl Store {
    #[cfg(not(test))]
    pub fn here() -> Store {
        Store::at(home_snapshots::HOME, home_snapshots::SNAPSHOTS)
    }

    pub fn at(live: impl Into<PathBuf>, snapshots: impl Into<PathBuf>) -> Store {
        Store { live: live.into(), snapshots: snapshots.into() }
    }

    pub fn any(&self) -> bool {
        !home_snapshots::kept_in(&self.snapshots).is_empty()
    }

    pub fn covers(&self, path: &Path) -> bool {
        self.place(path).is_some()
    }

    pub fn holds(&self, path: &Path) -> Option<u32> {
        if path.components().any(|c| matches!(c, Component::ParentDir)) {
            return None;
        }
        let mut below = path.strip_prefix(&self.snapshots).ok()?.components();
        let number = below.next()?.as_os_str().to_str()?.parse().ok()?;
        (below.next()?.as_os_str() == "snapshot").then_some(number)
    }

    fn place(&self, path: &Path) -> Option<PathBuf> {
        if !path.is_absolute() || path.components().any(|c| matches!(c, Component::ParentDir)) {
            return None;
        }
        let name = path.file_name()?;
        let parent = path.parent()?;
        let real = std::fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf()).join(name);
        let live = std::fs::canonicalize(&self.live).unwrap_or_else(|_| self.live.clone());
        let below = real.strip_prefix(&live).ok()?;
        (!below.as_os_str().is_empty()).then(|| below.to_path_buf())
    }

    fn inside(&self, number: u32, below: &Path) -> Option<(PathBuf, Metadata)> {
        let mut at = home_snapshots::tree(&self.snapshots, number);
        let mut parts = below.components().peekable();
        while let Some(part) = parts.next() {
            let Component::Normal(name) = part else { return None };
            at.push(name);
            let m = std::fs::symlink_metadata(&at).ok()?;
            if parts.peek().is_some() && !m.is_dir() {
                return None;
            }
            if parts.peek().is_none() {
                return Some((at, m));
            }
        }
        None
    }

    fn version(&self, k: &Kept, below: &Path) -> Option<(Version, Mark)> {
        let (path, m) = self.inside(k.number, below)?;
        let kind = kind_of(&m)?;
        let mark = mark(&path, &m)?;
        let size = if kind == Kind::File { m.len() } else { 0 };
        Some((Version { number: k.number, taken: k.taken, path, kind, size, modified: m.mtime(), differs: true }, mark))
    }

    pub fn versions(&self, path: &Path) -> Vec<Version> {
        let Some(below) = self.place(path) else { return Vec::new() };
        let mut kept = home_snapshots::kept_in(&self.snapshots);
        kept.sort_by_key(|k| std::cmp::Reverse((k.taken, k.number)));
        let now = self.live.join(&below);
        let mut newer = std::fs::symlink_metadata(&now).ok().and_then(|m| mark(&now, &m));
        let mut out = Vec::new();
        for k in &kept {
            let Some((mut v, mark)) = self.version(k, &below) else { continue };
            v.differs = mark == Mark::Folder || newer.as_ref() != Some(&mark);
            newer = Some(mark);
            out.push(v);
        }
        out
    }

    pub fn changes(&self, path: &Path) -> Vec<Version> {
        self.versions(path).into_iter().filter(|v| v.differs).collect()
    }

    pub fn gone_from(&self, folder: &Path) -> Vec<Gone> {
        let Some(below) = self.place(&folder.join("x")).and_then(|p| p.parent().map(Path::to_path_buf)) else {
            return Vec::new();
        };
        let mut kept = home_snapshots::kept_in(&self.snapshots);
        kept.sort_by_key(|k| std::cmp::Reverse((k.taken, k.number)));
        let now = self.live.join(&below);
        let mut out: Vec<Gone> = Vec::new();
        for k in &kept {
            let dir = if below.as_os_str().is_empty() {
                Some(home_snapshots::tree(&self.snapshots, k.number))
            } else {
                self.inside(k.number, &below).filter(|(_, m)| m.is_dir()).map(|(p, _)| p)
            };
            let Some(dir) = dir else { continue };
            let Ok(list) = std::fs::read_dir(&dir) else { continue };
            for entry in list.flatten() {
                let name = entry.file_name();
                let Some(text) = name.to_str() else { continue };
                if out.iter().any(|g| g.name == text) || std::fs::symlink_metadata(now.join(&name)).is_ok() {
                    continue;
                }
                if let Some((last, _)) = self.version(k, &below.join(&name)) {
                    out.push(Gone { name: text.to_string(), last });
                }
            }
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        out
    }

    pub fn put_back(&self, version: &Version, to: &Path) -> Result<PutBack, String> {
        if !version.path.starts_with(&self.snapshots) || version.path.components().any(|c| matches!(c, Component::ParentDir)) {
            return Err("That is not an earlier version.".into());
        }
        if self.place(to).is_none() {
            return Err("An earlier version can only be put back inside the home folder.".into());
        }
        let (Some(parent), Some(name)) = (to.parent(), to.file_name().and_then(|n| n.to_str())) else {
            return Err("That is not a place a file can go.".into());
        };
        std::fs::create_dir_all(parent).map_err(|e| format!("The folder it was in could not be made again: {e}"))?;
        let coming = parent.join(format!(".{name}.putting-back-{}", std::process::id()));
        remove(&coming);
        if let Err(e) = copy(&version.path, &coming) {
            remove(&coming);
            return Err(format!("The earlier version could not be copied: {e}"));
        }
        let kept = if std::fs::symlink_metadata(to).is_ok() {
            let aside = free_name(parent, name);
            if let Err(e) = crate::common::files::rename_new(to, &aside) {
                remove(&coming);
                return Err(format!("What is there now could not be set aside: {e}"));
            }
            Some(aside)
        } else {
            None
        };
        if let Err(e) = crate::common::files::rename_new(&coming, to) {
            if let Some(aside) = &kept {
                let _ = crate::common::files::rename_new(aside, to);
            }
            remove(&coming);
            return Err(format!("The earlier version could not be put in place: {e}"));
        }
        Ok(PutBack { put: to.to_path_buf(), kept })
    }
}

fn free_name(parent: &Path, name: &str) -> PathBuf {
    let whole = Path::new(name);
    let (stem, ext) = match (whole.file_stem().and_then(|s| s.to_str()), whole.extension().and_then(|e| e.to_str())) {
        (Some(stem), Some(ext)) => (stem.to_string(), format!(".{ext}")),
        _ => (name.to_string(), String::new()),
    };
    let first = parent.join(format!("{stem}{BEFORE}{ext}"));
    if std::fs::symlink_metadata(&first).is_err() {
        return first;
    }
    (2u32..).map(|n| parent.join(format!("{stem}{BEFORE} {n}{ext}")))
        .find(|p| std::fs::symlink_metadata(p).is_err()).expect("some number is free")
}

fn remove(path: &Path) {
    match std::fs::symlink_metadata(path) {
        Ok(m) if m.is_dir() => drop(std::fs::remove_dir_all(path)),
        Ok(_) => drop(std::fs::remove_file(path)),
        Err(_) => {}
    }
}

fn open(path: &Path) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path)
}

fn copy(from: &Path, to: &Path) -> std::io::Result<()> {
    let m = std::fs::symlink_metadata(from)?;
    match kind_of(&m) {
        Some(Kind::Link) => std::os::unix::fs::symlink(std::fs::read_link(from)?, to),
        Some(Kind::File) => {
            let mut source = open(from)?;
            if !source.metadata()?.is_file() {
                return Ok(());
            }
            let made = std::fs::OpenOptions::new().write(true).create_new(true).mode(m.mode() & 0o777).open(to)?;
            let made = super::gentle::copy_into(&mut source, made)?;
            made.set_permissions(std::fs::Permissions::from_mode(m.mode() & 0o777))?;
            made.set_modified(m.modified()?)
        }
        Some(Kind::Folder) => {
            std::fs::create_dir(to)?;
            std::fs::set_permissions(to, std::fs::Permissions::from_mode(0o700))?;
            for entry in std::fs::read_dir(from)? {
                let entry = entry?;
                copy(&entry.path(), &to.join(entry.file_name()))?;
            }
            std::fs::set_permissions(to, std::fs::Permissions::from_mode(m.mode() & 0o777))?;
            std::fs::File::open(to).and_then(|d| d.set_modified(m.modified()?)).or(Ok(()))
        }
        None => Ok(()),
    }
}

pub fn same_content(a: &Path, b: &Path) -> std::io::Result<bool> {
    let (ma, mb) = (std::fs::symlink_metadata(a)?, std::fs::symlink_metadata(b)?);
    match (kind_of(&ma), kind_of(&mb)) {
        (Some(Kind::Link), Some(Kind::Link)) => Ok(std::fs::read_link(a)? == std::fs::read_link(b)?),
        (Some(Kind::File), Some(Kind::File)) => {
            if ma.len() != mb.len() {
                return Ok(false);
            }
            let (mut fa, mut fb) = (open(a)?, open(b)?);
            let (mut ba, mut bb) = (vec![0u8; 65_536], vec![0u8; 65_536]);
            loop {
                let n = fa.read(&mut ba)?;
                if n == 0 {
                    return Ok(fb.read(&mut bb)? == 0);
                }
                fb.read_exact(&mut bb[..n])?;
                if ba[..n] != bb[..n] {
                    return Ok(false);
                }
            }
        }
        (Some(Kind::Folder), Some(Kind::Folder)) => {
            let names = |p: &Path| -> std::io::Result<Vec<std::ffi::OsString>> {
                let mut all = std::fs::read_dir(p)?.map(|e| e.map(|e| e.file_name())).collect::<Result<Vec<_>, _>>()?;
                all.sort();
                Ok(all)
            };
            let (na, nb) = (names(a)?, names(b)?);
            if na != nb {
                return Ok(false);
            }
            for name in na {
                if !same_content(&a.join(&name), &b.join(&name))? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::home_snapshots::tests::{add_snapshot, made_up_snapshots};
    use crate::common::home_snapshots::DESCRIPTION;
    use std::time::{Duration, UNIX_EPOCH};

    struct World {
        root: PathBuf,
        store: Store,
    }

    impl World {
        fn new(name: &str) -> World {
            let root = made_up_snapshots(&format!("earlier-{name}"));
            let root = std::fs::canonicalize(&root).unwrap();
            std::fs::create_dir_all(root.join("home/john")).unwrap();
            std::fs::create_dir_all(root.join("snaps")).unwrap();
            World { store: Store::at(root.join("home"), root.join("snaps")), root }
        }

        fn live(&self, below: &str) -> PathBuf {
            self.root.join("home").join(below)
        }

        fn snapshot(&self, number: u32, day: u32, files: &[(&str, &str, u64)]) -> PathBuf {
            let tree = add_snapshot(&self.root.join("snaps"), number, &format!("2026-10-{day:02} 03:00:00"), DESCRIPTION);
            for (below, words, changed) in files {
                write(&tree.join(below), words, *changed);
            }
            tree
        }
    }

    impl Drop for World {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn write(path: &Path, words: &str, changed: u64) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, words).unwrap();
        std::fs::File::options().write(true).open(path).unwrap()
            .set_modified(UNIX_EPOCH + Duration::from_secs(changed)).unwrap();
    }

    #[test]
    fn a_files_versions_are_listed_newest_first_and_marked_where_they_differ() {
        let w = World::new("versions");
        write(&w.live("john/report.txt"), "third draft", 3000);
        w.snapshot(1, 1, &[("john/report.txt", "first", 1000)]);
        w.snapshot(2, 2, &[("john/report.txt", "first", 1000)]);
        w.snapshot(3, 3, &[("john/report.txt", "second draft", 2000)]);
        w.snapshot(4, 4, &[("john/report.txt", "third draft", 3000)]);
        w.snapshot(5, 5, &[("john/other.txt", "x", 1)]);
        let all = w.store.versions(&w.live("john/report.txt"));
        assert_eq!(all.iter().map(|v| v.number).collect::<Vec<_>>(), [4, 3, 2, 1]);
        assert_eq!(all.iter().map(|v| v.differs).collect::<Vec<_>>(), [false, true, true, false]);
        assert_eq!(all[1].size, 12);
        assert_eq!(all[1].modified, 2000);
        assert_eq!(all[1].kind, Kind::File);
        assert_eq!(all[1].path, w.root.join("snaps/3/snapshot/john/report.txt"));
        assert_eq!(all[1].taken, home_snapshots::unix_of("2026-10-03 03:00:00").unwrap());
        assert_eq!(w.store.changes(&w.live("john/report.txt")).iter().map(|v| v.number).collect::<Vec<_>>(), [3, 2]);
        assert!(w.store.any());
    }

    #[test]
    fn the_same_length_at_another_time_is_another_version() {
        let w = World::new("time");
        write(&w.live("john/a.txt"), "abcd", 2000);
        w.snapshot(1, 1, &[("john/a.txt", "wxyz", 1000)]);
        let all = w.store.versions(&w.live("john/a.txt"));
        assert!(all[0].differs);
        assert!(!same_content(&all[0].path, &w.live("john/a.txt")).unwrap());
    }

    #[test]
    fn a_place_inside_a_snapshot_is_known_from_its_path_and_the_home_folders_are_covered() {
        let w = World::new("holds");
        let tree = w.snapshot(7, 3, &[("john/report.txt", "first", 1000)]);
        assert_eq!(w.store.holds(&tree), Some(7));
        assert_eq!(w.store.holds(&tree.join("john/report.txt")), Some(7));
        assert_eq!(w.store.holds(&w.root.join("snaps")), None);
        assert_eq!(w.store.holds(&w.root.join("snaps/7/info.xml")), None);
        assert_eq!(w.store.holds(&w.root.join("snaps/seven/snapshot/john")), None);
        assert_eq!(w.store.holds(&tree.join("../../8/snapshot")), None);
        assert_eq!(w.store.holds(&w.live("john/report.txt")), None);
        assert_eq!(w.store.holds(Path::new("/etc/passwd")), None);
        assert!(w.store.covers(&w.live("john/report.txt")) && w.store.covers(&w.live("john/gone/long ago.txt")));
        assert!(!w.store.covers(&w.root.join("home")) && !w.store.covers(Path::new("/etc/passwd")) && !w.store.covers(&tree.join("john")));
    }

    #[test]
    fn a_deleted_file_is_found_where_it_was() {
        let w = World::new("deleted");
        w.snapshot(1, 1, &[("john/Documents/letter.odt", "dear sir", 1000), ("john/Documents/kept.txt", "k", 1)]);
        w.snapshot(2, 2, &[("john/Documents/letter.odt", "dear sir or madam", 2000), ("john/Documents/kept.txt", "k", 1)]);
        write(&w.live("john/Documents/kept.txt"), "k", 1);
        let all = w.store.versions(&w.live("john/Documents/letter.odt"));
        assert_eq!(all.iter().map(|v| (v.number, v.differs)).collect::<Vec<_>>(), [(2, true), (1, true)]);
        let gone = w.store.gone_from(&w.live("john/Documents"));
        assert_eq!(gone.len(), 1);
        assert_eq!(gone[0].name, "letter.odt");
        assert_eq!(gone[0].last.number, 2);
        std::fs::remove_dir_all(w.live("john/Documents")).unwrap();
        assert_eq!(w.store.gone_from(&w.live("john")).iter().map(|g| g.name.as_str()).collect::<Vec<_>>(), ["Documents"]);
        assert_eq!(w.store.versions(&w.live("john/Documents/letter.odt")).len(), 2);
        assert_eq!(w.store.gone_from(&w.live("john/Documents")).len(), 2);
    }

    #[test]
    fn a_folder_is_listed_from_every_snapshot_and_compared_only_when_asked() {
        let w = World::new("folder");
        write(&w.live("john/Pictures/a.png"), "aaa", 10);
        w.snapshot(1, 1, &[("john/Pictures/a.png", "aaa", 10)]);
        w.snapshot(2, 2, &[("john/Pictures/a.png", "aaa", 10), ("john/Pictures/sub/b.png", "b", 20)]);
        let all = w.store.versions(&w.live("john/Pictures"));
        assert_eq!(all.iter().map(|v| (v.number, v.kind, v.differs)).collect::<Vec<_>>(),
                   [(2, Kind::Folder, true), (1, Kind::Folder, true)]);
        assert!(same_content(&all[1].path, &w.live("john/Pictures")).unwrap());
        assert!(!same_content(&all[0].path, &w.live("john/Pictures")).unwrap());
    }

    #[test]
    fn a_link_in_a_snapshot_is_never_followed() {
        let w = World::new("links");
        std::fs::write(w.root.join("secret.txt"), "not yours").unwrap();
        std::fs::create_dir_all(w.root.join("elsewhere")).unwrap();
        std::fs::write(w.root.join("elsewhere/passwd"), "not yours either").unwrap();
        let tree = w.snapshot(1, 1, &[("john/real.txt", "mine", 5)]);
        std::os::unix::fs::symlink(w.root.join("secret.txt"), tree.join("john/out.txt")).unwrap();
        std::os::unix::fs::symlink(w.root.join("elsewhere"), tree.join("john/way-out")).unwrap();
        assert!(w.store.versions(&w.live("john/way-out/passwd")).is_empty());
        let link = w.store.versions(&w.live("john/out.txt"));
        assert_eq!(link.len(), 1);
        assert_eq!(link[0].kind, Kind::Link);
        let done = w.store.put_back(&link[0], &w.live("john/out.txt")).unwrap();
        assert!(std::fs::symlink_metadata(&done.put).unwrap().file_type().is_symlink());
        let john = w.store.versions(&w.live("john"));
        let done = w.store.put_back(&john[0], &w.live("john-as-it-was")).unwrap();
        assert!(std::fs::symlink_metadata(done.put.join("way-out")).unwrap().file_type().is_symlink());
        assert!(std::fs::symlink_metadata(done.put.join("out.txt")).unwrap().file_type().is_symlink());
        assert_eq!(std::fs::read_to_string(done.put.join("real.txt")).unwrap(), "mine");
        assert!(same_content(&tree.join("john/out.txt"), &w.live("john/out.txt")).unwrap());
        assert!(open(&tree.join("john/out.txt")).is_err());
    }

    #[test]
    fn only_a_place_inside_the_home_folders_is_looked_up() {
        let w = World::new("places");
        w.snapshot(1, 1, &[("john/a.txt", "a", 1)]);
        std::fs::write(w.root.join("outside.txt"), "x").unwrap();
        assert!(w.store.versions(&w.root.join("outside.txt")).is_empty());
        assert!(w.store.versions(Path::new("john/a.txt")).is_empty());
        assert!(w.store.versions(&w.live("john/../../outside.txt")).is_empty());
        assert!(w.store.versions(&w.live("")).is_empty());
        assert!(w.store.versions(Path::new("/")).is_empty());
        std::os::unix::fs::symlink(w.live("john"), w.live("me")).unwrap();
        assert_eq!(w.store.versions(&w.live("me/a.txt")).len(), 1);
        std::os::unix::fs::symlink(&w.root, w.live("john/up")).unwrap();
        assert!(w.store.versions(&w.live("john/up/outside.txt")).is_empty());
    }

    #[test]
    fn putting_back_keeps_what_was_there_under_a_new_name() {
        let w = World::new("putback");
        write(&w.live("john/report.odt"), "spoilt", 3000);
        let tree = w.snapshot(1, 1, &[("john/report.odt", "good", 1000)]);
        std::fs::set_permissions(tree.join("john/report.odt"), std::fs::Permissions::from_mode(0o640)).unwrap();
        let v = w.store.changes(&w.live("john/report.odt")).remove(0);
        let done = w.store.put_back(&v, &w.live("john/report.odt")).unwrap();
        assert_eq!(done.put, w.live("john/report.odt"));
        assert_eq!(done.kept, Some(w.live("john/report (before putting back).odt")));
        assert_eq!(std::fs::read_to_string(&done.put).unwrap(), "good");
        assert_eq!(std::fs::read_to_string(done.kept.as_ref().unwrap()).unwrap(), "spoilt");
        let m = std::fs::metadata(&done.put).unwrap();
        assert_eq!(m.mode() & 0o777, 0o640);
        assert_eq!(m.mtime(), 1000);
        assert!(!w.store.versions(&w.live("john/report.odt"))[0].differs);
        assert_eq!(std::fs::read_to_string(&v.path).unwrap(), "good");
        write(&w.live("john/report.odt"), "spoilt again", 4000);
        let again = w.store.put_back(&v, &w.live("john/report.odt")).unwrap();
        assert_eq!(again.kept, Some(w.live("john/report (before putting back) 2.odt")));
        assert_eq!(std::fs::read_to_string(w.live("john/report (before putting back).odt")).unwrap(), "spoilt");
        let names: Vec<String> = std::fs::read_dir(w.live("john")).unwrap().flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        assert!(names.iter().all(|n| !n.contains("putting-back")), "{names:?}");
    }

    #[test]
    fn a_deleted_file_or_folder_is_put_back_with_nothing_set_aside() {
        let w = World::new("putgone");
        w.snapshot(1, 1, &[("john/Work/plan/notes.txt", "notes", 100), ("john/Work/plan/deep/more.txt", "more", 200)]);
        let v = w.store.versions(&w.live("john/Work/plan/notes.txt")).remove(0);
        let done = w.store.put_back(&v, &w.live("john/Work/plan/notes.txt")).unwrap();
        assert_eq!(done.kept, None);
        assert_eq!(std::fs::read_to_string(&done.put).unwrap(), "notes");
        let folder = w.store.versions(&w.live("john/Work/plan")).remove(0);
        let done = w.store.put_back(&folder, &w.live("john/Work/plan")).unwrap();
        assert_eq!(done.kept, Some(w.live("john/Work/plan (before putting back)")));
        assert_eq!(std::fs::read_to_string(done.put.join("deep/more.txt")).unwrap(), "more");
        assert!(same_content(&folder.path, &done.put).unwrap());
        assert_eq!(free_name(Path::new("/nowhere"), "Makefile"), Path::new("/nowhere/Makefile (before putting back)"));
        assert_eq!(free_name(Path::new("/nowhere"), ".bashrc"), Path::new("/nowhere/.bashrc (before putting back)"));
    }

    #[test]
    fn putting_back_refuses_what_is_not_a_version_or_not_in_the_home() {
        let w = World::new("refuse");
        w.snapshot(1, 1, &[("john/a.txt", "a", 1)]);
        std::fs::write(w.root.join("outside.txt"), "x").unwrap();
        let v = w.store.versions(&w.live("john/a.txt")).remove(0);
        assert!(w.store.put_back(&v, &w.root.join("outside.txt")).is_err());
        assert_eq!(std::fs::read_to_string(w.root.join("outside.txt")).unwrap(), "x");
        let forged = Version { path: w.root.join("outside.txt"), ..v.clone() };
        assert!(w.store.put_back(&forged, &w.live("john/a.txt")).is_err());
        assert!(!w.live("john/a.txt").exists());
        write(&w.live("john/a.txt"), "now", 9);
        std::fs::remove_file(&v.path).unwrap();
        assert!(w.store.put_back(&v, &w.live("john/a.txt")).is_err());
        assert_eq!(std::fs::read_to_string(w.live("john/a.txt")).unwrap(), "now");
        assert_eq!(std::fs::read_dir(w.live("john")).unwrap().count(), 1);
    }

    #[test]
    fn with_no_snapshots_there_is_nothing_to_offer() {
        let w = World::new("none");
        write(&w.live("john/a.txt"), "a", 1);
        assert!(!w.store.any());
        assert!(w.store.versions(&w.live("john/a.txt")).is_empty());
        assert!(w.store.gone_from(&w.live("john")).is_empty());
    }
}
