//! Wiping files: each is written over with random bytes, cut to nothing, renamed and removed; a link is removed as a link and what it points to is never touched.

use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use crate::toolkit::t;

use super::meter::{is_stopped, Counted, Meter};
use crate::common::gentle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    OtherNames,
    NotAFile,
    Changed,
    Failed(io::ErrorKind),
}

#[derive(Debug, Default)]
pub struct Wiped {
    pub gone: Vec<PathBuf>,
    pub refused: Vec<(PathBuf, Refused)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Caveat {
    Flash,
    CopyOnWrite(String),
    Network(String),
}

pub fn caveats(place: &Path) -> Vec<Caveat> {
    let mut out = Vec::new();
    let real = fs::canonicalize(place).unwrap_or_else(|_| place.to_path_buf());
    if let Some(kind) = fs_kind(&real) {
        let base = kind.trim_start_matches("fuse.");
        if ["btrfs", "zfs", "bcachefs", "f2fs", "nilfs2"].contains(&base) {
            out.push(Caveat::CopyOnWrite(kind.clone()));
        } else if super::places::on_another_computer(&kind) {
            out.push(Caveat::Network(kind.clone()));
        }
    }
    if super::work::rotational(&real) == Some(false) {
        out.push(Caveat::Flash);
    }
    out
}

#[cfg(test)]
pub fn wipe(paths: &[PathBuf], passes: u32, stop: &AtomicBool, tell: &dyn Fn(u64, u64)) -> io::Result<Wiped> {
    wipe_asked(&super::proof::asked(paths), paths, passes, stop, tell)
}

pub fn wipe_asked(asked: &[super::proof::Asked], paths: &[PathBuf], passes: u32, stop: &AtomicBool, tell: &dyn Fn(u64, u64)) -> io::Result<Wiped> {
    use crate::common::files::{through, Which};
    let passes = passes.max(1) as u64;
    let meter = Meter::new(stop, tell);
    let mut wiped = Wiped::default();
    let mut todo: Vec<(std::rc::Rc<File>, PathBuf, std::ffi::OsString, Which)> = Vec::new();
    let mut opened: std::collections::HashMap<PathBuf, std::rc::Rc<File>> = std::collections::HashMap::new();
    for p in paths {
        let was = asked.iter().find(|a| a.path() == p).map(|a| a.which());
        let (Some(up), Some(name)) = (p.parent(), p.file_name()) else {
            wiped.refused.push((p.clone(), Refused::Failed(io::ErrorKind::InvalidInput)));
            continue;
        };
        match fs::symlink_metadata(p) {
            Ok(m) if was == Some(Which::of(&m)) => {
                let dir = match opened.get(up) {
                    Some(d) => Ok(d.clone()),
                    None => OpenOptions::new().read(true).custom_flags(libc::O_PATH | libc::O_DIRECTORY).open(up).map(std::rc::Rc::new),
                };
                match dir {
                    Ok(dir) => {
                        opened.insert(up.to_path_buf(), dir.clone());
                        todo.push((dir, up.to_path_buf(), name.to_os_string(), Which::of(&m)));
                    }
                    Err(e) => wiped.refused.push((p.clone(), Refused::Failed(e.kind()))),
                }
            }
            Ok(_) => wiped.refused.push((p.clone(), Refused::Changed)),
            Err(e) => wiped.refused.push((p.clone(), Refused::Failed(e.kind()))),
        }
    }
    meter.total(todo.iter().map(|(dir, _, name, _)| bytes_below(&through(dir).join(name))).sum::<u64>() * passes);
    for (dir, shown, name, which) in &todo {
        wipe_one(dir, shown, name, Some(*which), passes, &meter, &mut wiped)?;
    }
    meter.end();
    Ok(wiped)
}

fn bytes_below(path: &Path) -> u64 {
    let Ok(meta) = fs::symlink_metadata(path) else { return 0 };
    if meta.is_dir() {
        fs::read_dir(path).map_or(0, |list| list.flatten().map(|e| bytes_below(&e.path())).sum())
    } else if meta.is_file() && meta.nlink() == 1 {
        meta.len()
    } else {
        0
    }
}

#[cfg(test)]
type Between = Box<dyn Fn(&Path)>;

#[cfg(test)]
thread_local! {
    static BETWEEN: std::cell::RefCell<Option<Between>> = const { std::cell::RefCell::new(None) };
}

fn wipe_one(dir: &File, shown: &Path, name: &std::ffi::OsStr, was: Option<crate::common::files::Which>, passes: u64, meter: &Meter, wiped: &mut Wiped) -> io::Result<()> {
    use crate::common::files::{through, Which};
    let at = through(dir).join(name);
    let here = shown.join(name);
    let meta = match fs::symlink_metadata(&at) {
        Ok(m) if was.is_none_or(|w| w == Which::of(&m)) => m,
        Ok(_) => {
            wiped.refused.push((here, Refused::Changed));
            return Ok(());
        }
        Err(e) => {
            wiped.refused.push((here, Refused::Failed(e.kind())));
            return Ok(());
        }
    };
    meter.check()?;
    let gone = |went: Went| match went {
        Went::Gone => None,
        Went::Left(lies, why) => {
            let lies = lies.strip_prefix(through(dir)).map_or(lies.clone(), |rest| shown.join(rest));
            Some(Some((lies, why)))
        }
    };
    let kind = meta.file_type();
    let stays = |why: Refused| Ok(Some(Some((here.clone(), why))));
    let result = if kind.is_dir() {
        wipe_inside(dir, &here, name, &meta, passes, meter, wiped).and_then(|opened| match opened {
            Some(inner) => match fs::read_dir(through(&inner)).map(|mut d| d.next().is_none()) {
                Ok(true) => hide_and_remove(&at, true, &meta).map(gone),
                Ok(false) => Ok(Some(None)),
                Err(e) => Err(e),
            },
            None => Ok(Some(None)),
        })
    } else if kind.is_symlink() {
        hide_and_remove(&at, false, &meta).map(gone)
    } else if !kind.is_file() {
        stays(Refused::NotAFile)
    } else if meta.nlink() > 1 {
        stays(Refused::OtherNames)
    } else {
        match over(&at, &meta, passes, meter) {
            Ok(true) => hide_and_remove(&at, false, &meta).map(gone),
            Ok(false) => stays(Refused::Changed),
            Err(e) => Err(e),
        }
    };
    match result {
        Ok(None) => wiped.gone.push(here),
        Ok(Some(None)) => {}
        Ok(Some(Some(left))) => wiped.refused.push(left),
        Err(e) if is_stopped(&e) => return Err(e),
        Err(e) => wiped.refused.push((here, Refused::Failed(e.kind()))),
    }
    Ok(())
}

fn wipe_inside(dir: &File, here: &Path, name: &std::ffi::OsStr, meta: &Metadata, passes: u64, meter: &Meter, wiped: &mut Wiped) -> io::Result<Option<File>> {
    use crate::common::files::{is_changed, open_in, through, Which};
    let inner = match open_in(dir, name, Some(Which::of(meta))) {
        Ok((inner, _)) => inner,
        Err(e) => {
            let why = if is_changed(&e) { Refused::Changed } else { Refused::Failed(e.kind()) };
            wiped.refused.push((here.to_path_buf(), why));
            return Ok(None);
        }
    };
    let names: io::Result<Vec<std::ffi::OsString>> = fs::read_dir(through(&inner)).and_then(|list| list.map(|e| e.map(|e| e.file_name())).collect());
    let names = match names {
        Ok(names) => names,
        Err(e) => {
            wiped.refused.push((here.to_path_buf(), Refused::Failed(e.kind())));
            return Ok(None);
        }
    };
    #[cfg(test)]
    BETWEEN.with(|then| then.borrow().iter().for_each(|then| then(here)));
    for each in names {
        wipe_one(&inner, here, &each, None, passes, meter, wiped)?;
    }
    Ok(Some(inner))
}

fn over(path: &Path, meta: &Metadata, passes: u64, meter: &Meter) -> io::Result<bool> {
    let flags = libc::O_NOFOLLOW | libc::O_NONBLOCK;
    let same = |f: &File| f.metadata().is_ok_and(|m| m.is_file() && m.dev() == meta.dev() && m.ino() == meta.ino());
    let file = match OpenOptions::new().write(true).custom_flags(flags).open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            let f = OpenOptions::new().read(true).custom_flags(flags).open(path)?;
            if !same(&f) {
                return Ok(false);
            }
            let mode = f.metadata()?.permissions().mode();
            f.set_permissions(fs::Permissions::from_mode(mode | 0o200))?;
            OpenOptions::new().write(true).custom_flags(flags).open(path)?
        }
        Err(e) => return Err(e),
    };
    if !same(&file) {
        return Ok(false);
    }
    let len = file.metadata()?.len();
    let mut noise = Noise::new();
    let mut buf = vec![0u8; len.clamp(1, 1 << 20) as usize];
    let mut out = Counted::new(&file, meter);
    for _ in 0..passes {
        out.inner.seek(SeekFrom::Start(0))?;
        let mut left = len;
        let mut waiting = 0u64;
        while left > 0 {
            let n = left.min(buf.len() as u64) as usize;
            noise.fill(&mut buf[..n]);
            out.write_all(&buf[..n])?;
            left -= n as u64;
            waiting += n as u64;
            if waiting >= gentle::STEP {
                gentle::written(&file, len - left - waiting, waiting)?;
                waiting = 0;
            }
        }
        file.sync_data()?;
    }
    file.set_len(0)?;
    file.sync_all()?;
    Ok(true)
}

#[cfg(test)]
thread_local! {
    static MEANWHILE: std::cell::RefCell<Option<Box<dyn Fn()>>> = const { std::cell::RefCell::new(None) };
}

fn hide_and_remove(path: &Path, folder: bool, was: &Metadata) -> io::Result<Went> {
    let name = path.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, t!("folders-wipe-no-name")))?;
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut noise = Noise::new();
    let hidden = loop {
        let mut b = [0u8; 8];
        noise.fill(&mut b);
        let p = dir.join(format!("{:016x}", u64::from_le_bytes(b)));
        if fs::symlink_metadata(&p).is_err() {
            break p;
        }
    };
    fs::rename(path, &hidden)?;
    if !fs::symlink_metadata(&hidden).is_ok_and(|now| (now.dev(), now.ino()) == (was.dev(), was.ino())) {
        #[cfg(test)]
        MEANWHILE.with(|then| then.borrow().iter().for_each(|then| then()));
        return Ok(Went::Left(super::proof::named_back(&hidden, dir, name, folder), Refused::Changed));
    }
    let removed = if folder { fs::remove_dir(&hidden) } else { fs::remove_file(&hidden) };
    if let Err(e) = removed {
        return Ok(Went::Left(super::proof::named_back(&hidden, dir, name, folder), Refused::Failed(e.kind())));
    }
    crate::common::files::folder_out(dir)?;
    Ok(Went::Gone)
}

#[derive(Debug, PartialEq)]
enum Went {
    Gone,
    Left(PathBuf, Refused),
}

struct Noise([u64; 4]);

impl Noise {
    fn new() -> Noise {
        let mut seed = [0u8; 32];
        if getrandom::fill(&mut seed).is_err() {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(1);
            seed[..16].copy_from_slice(&t.to_le_bytes());
            let here = &t as *const u128 as usize as u64;
            seed[16..24].copy_from_slice(&here.to_le_bytes());
        }
        let mut s = [0u64; 4];
        for (i, w) in s.iter_mut().enumerate() {
            *w = u64::from_le_bytes(seed[i * 8..i * 8 + 8].try_into().expect("eight bytes")) | 1;
        }
        Noise(s)
    }

    fn next(&mut self) -> u64 {
        let s = &mut self.0;
        let out = s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);
        out
    }

    fn fill(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let x = self.next().to_le_bytes();
            chunk.copy_from_slice(&x[..chunk.len()]);
        }
    }
}

fn fs_kind(path: &Path) -> Option<String> {
    let text = fs::read_to_string("/proc/self/mountinfo").ok()?;
    let mut best: Option<(usize, String)> = None;
    for line in text.lines() {
        let fields: Vec<&str> = line.split(' ').collect();
        let Some(dash) = fields.iter().position(|f| *f == "-") else { continue };
        let (Some(point), Some(kind)) = (fields.get(4), fields.get(dash + 1)) else { continue };
        let point = super::places::unescape(point);
        if path.starts_with(&point) && best.as_ref().is_none_or(|(n, _)| point.len() >= *n) {
            best = Some((point.len(), kind.to_string()));
        }
    }
    best.map(|(_, k)| k)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;
    use std::sync::atomic::Ordering;

    #[test]
    fn only_the_thing_that_was_asked_about_is_wiped() {
        let dir = place("asked");
        let stop = AtomicBool::new(false);
        let paths = [dir.join("secret")];
        fs::write(&paths[0], b"asked about").unwrap();
        let asked = super::super::proof::asked(&paths);
        fs::rename(&paths[0], dir.join("moved")).unwrap();
        fs::write(&paths[0], b"another program's").unwrap();
        let wiped = wipe_asked(&asked, &paths, 1, &stop, &|_, _| {}).unwrap();
        assert_eq!(wiped.refused, [(paths[0].clone(), Refused::Changed)]);
        assert_eq!(fs::read(&paths[0]).unwrap(), b"another program's", "what the person was never asked about was wiped");
        assert_eq!(fs::read(dir.join("moved")).unwrap(), b"asked about");
        let wiped = wipe_asked(&[], &paths, 1, &stop, &|_, _| {}).unwrap();
        assert!(wiped.gone.is_empty() && wiped.refused == [(paths[0].clone(), Refused::Changed)]);
        let wiped = wipe_asked(&super::super::proof::asked(&paths), &paths, 1, &stop, &|_, _| {}).unwrap();
        assert!(wiped.refused.is_empty() && wiped.gone == paths && !paths[0].exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn what_is_not_wiped_is_never_left_under_a_meaningless_name() {
        let dir = place("retaken");
        let (path, other) = (dir.join("report.txt"), dir.join("elsewhere/listed"));
        fs::write(&path, b"another program's").unwrap();
        fs::write(&other, b"the one listed").unwrap();
        let was = fs::symlink_metadata(&other).unwrap();
        assert_eq!(hide_and_remove(&path, false, &was).unwrap(), Went::Left(path.clone(), Refused::Changed));
        assert_eq!(fs::read(&path).unwrap(), b"another program's");
        let again = path.clone();
        MEANWHILE.with(|then| *then.borrow_mut() = Some(Box::new(move || fs::write(&again, b"a third thing").unwrap())));
        let left = hide_and_remove(&path, false, &was);
        MEANWHILE.with(|then| *then.borrow_mut() = None);
        assert_eq!(left.unwrap(), Went::Left(dir.join("report (2).txt"), Refused::Changed));
        assert_eq!(fs::read(&path).unwrap(), b"a third thing", "what took the name was written over");
        let mut names: Vec<String> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
        names.sort();
        assert_eq!(names, ["chosen", "elsewhere", "report (2).txt", "report.txt"], "it lies under a name nobody would look for");
        assert_eq!(fs::read(dir.join("report (2).txt")).unwrap(), b"another program's");
        let folder = dir.join("chosen/inner");
        let was = fs::symlink_metadata(&folder).unwrap();
        fs::write(folder.join("put in since"), b"x").unwrap();
        assert!(matches!(hide_and_remove(&folder, true, &was).unwrap(), Went::Left(lies, Refused::Failed(_)) if lies == folder));
        assert!(folder.join("put in since").is_file());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_folder_inside_swapped_for_a_link_does_not_send_the_wipe_elsewhere() {
        let dir = place("swapped");
        fs::write(dir.join("chosen/inner/b"), b"in the folder").unwrap();
        fs::write(dir.join("elsewhere/b"), b"keep me").unwrap();
        let stop = AtomicBool::new(false);
        let at = dir.clone();
        BETWEEN.with(|then| {
            *then.borrow_mut() = Some(Box::new(move |here: &Path| {
                if here.ends_with("chosen/inner") {
                    fs::rename(at.join("chosen/inner"), at.join("given-away")).unwrap();
                    symlink(at.join("elsewhere"), at.join("chosen/inner")).unwrap();
                }
            }))
        });
        let wiped = wipe(&[dir.join("chosen")], 1, &stop, &|_, _| {});
        BETWEEN.with(|then| *then.borrow_mut() = None);
        let wiped = wiped.unwrap();
        assert_eq!(fs::read(dir.join("elsewhere/b")).unwrap(), b"keep me", "the wipe was sent through a link");
        assert!(!dir.join("given-away/b").exists(), "what was in the folder that was opened was left");
        assert!(wiped.gone.contains(&dir.join("chosen/inner/b")));
        fs::remove_dir_all(&dir).unwrap();
    }

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-wipe-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("chosen/inner")).unwrap();
        fs::create_dir_all(dir.join("elsewhere")).unwrap();
        dir
    }

    #[test]
    fn files_and_folders_go_and_links_lead_nowhere() {
        let dir = place("wipe");
        let secret = b"John Smith, 07700 900123".repeat(1000);
        fs::write(dir.join("chosen/a"), &secret).unwrap();
        fs::write(dir.join("chosen/inner/b"), b"x").unwrap();
        fs::write(dir.join("chosen/inner/empty"), b"").unwrap();
        fs::write(dir.join("elsewhere/target"), b"keep me").unwrap();
        symlink(dir.join("elsewhere/target"), dir.join("chosen/link")).unwrap();
        symlink(dir.join("elsewhere"), dir.join("chosen/folder-link")).unwrap();
        fs::write(dir.join("ro"), b"read only").unwrap();
        fs::set_permissions(dir.join("ro"), fs::Permissions::from_mode(0o400)).unwrap();
        fs::write(dir.join("twice"), b"two names").unwrap();
        fs::hard_link(dir.join("twice"), dir.join("elsewhere/twice")).unwrap();

        let stop = AtomicBool::new(false);
        let told = std::cell::Cell::new((0, 0));
        let tell = |d, t| told.set((d, t));
        let chosen = [dir.join("chosen"), dir.join("ro"), dir.join("twice"), dir.join("missing")];
        let wiped = wipe(&chosen, 2, &stop, &tell).unwrap();
        assert!(!dir.join("chosen").exists());
        assert!(!dir.join("ro").exists());
        assert_eq!(fs::read(dir.join("elsewhere/target")).unwrap(), b"keep me");
        assert_eq!(fs::read(dir.join("twice")).unwrap(), b"two names");
        assert!(wiped.refused.contains(&(dir.join("twice"), Refused::OtherNames)));
        assert!(wiped.refused.contains(&(dir.join("missing"), Refused::Failed(io::ErrorKind::NotFound))));
        assert_eq!(wiped.refused.len(), 2);
        assert_eq!(wiped.gone.len(), 8);
        assert_eq!(told.get(), ((secret.len() as u64 + 1 + 9) * 2, (secret.len() as u64 + 1 + 9) * 2));
        let mut left: Vec<_> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name()).collect();
        left.sort();
        assert_eq!(left, ["elsewhere", "twice"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn writing_over_counts_and_refuses_links() {
        let dir = place("over");
        let p = dir.join("f");
        let secret = vec![0x5au8; 300_000];
        fs::write(&p, &secret).unwrap();
        let meta = fs::symlink_metadata(&p).unwrap();
        let stop = AtomicBool::new(false);
        let meter = Meter::new(&stop, &|_, _| {});
        meter.total(secret.len() as u64 * 3);
        assert!(over(&p, &meta, 3, &meter).unwrap());
        assert_eq!(meter.done(), secret.len() as u64 * 3);
        fs::write(&p, b"short").unwrap();
        let listed = fs::symlink_metadata(&p).unwrap();
        fs::OpenOptions::new().append(true).open(&p).unwrap().write_all(&[7u8; 5000]).unwrap();
        let grew = Meter::new(&stop, &|_, _| {});
        assert!(over(&p, &listed, 1, &grew).unwrap());
        assert_eq!(grew.done(), 5005, "the tail added since the listing was not written over");
        assert_eq!(fs::metadata(&p).unwrap().len(), 0);
        let mut noise = vec![0u8; 4096];
        Noise::new().fill(&mut noise);
        assert!(noise.iter().filter(|b| **b == 0x5a).count() < 100);
        fs::remove_file(&p).unwrap();
        symlink(dir.join("elsewhere"), &p).unwrap();
        assert!(over(&p, &meta, 1, &meter).is_err());
        stop.store(true, Ordering::Relaxed);
        assert!(is_stopped(&wipe(&[dir.join("chosen")], 1, &stop, &|_, _| {}).unwrap_err()));
        assert!(dir.join("chosen/inner").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn caveats_read_the_mounts() {
        assert_eq!(super::super::places::unescape(r"/media/John\040Smith"), "/media/John Smith");
        assert_eq!(super::super::places::unescape(r"/a\\b"), r"/a\\b");
        assert!(fs_kind(Path::new("/proc/self")).is_some_and(|k| k == "proc"));
        let _ = caveats(&std::env::temp_dir());
    }
}
