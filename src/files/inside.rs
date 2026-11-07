// Archives opened like folders.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::SystemTime;

use super::app::{Files, Msg};
use super::archive::{self, Key};
use super::locked::Then;
use super::far;
use super::reach;
use super::places::{self, Entry, Where};
use super::work;
use crate::toolkit::t;

const KEEP: usize = 4;

#[derive(Default)]
pub struct Opened {
    read: HashMap<PathBuf, (Option<SystemTime>, Vec<archive::Entry>)>,
    pub waiting: Vec<PathBuf>,
    pub(super) names_locked: Vec<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct Refusal {
    pub key: Option<archive::Trouble>,
    pub why: String,
    pub help: &'static str,
}

impl Refusal {
    pub fn of(e: &std::io::Error) -> Refusal {
        Refusal { key: archive::wants_key(e), why: archive::why(e), help: super::help::archive_page(e) }
    }
}

#[derive(Clone, Debug)]
pub enum InsideMsg {
    Listed { archive: PathBuf, stamp: Option<SystemTime>, got: Result<Vec<archive::Entry>, Refusal> },
    Out { got: Result<(Vec<PathBuf>, usize), String>, sealed: bool },
}

pub(super) fn stamp(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

pub fn opens(path: &Path) -> bool {
    let package = path.extension().is_some_and(|e| e.eq_ignore_ascii_case("deb") || e.eq_ignore_ascii_case("udeb"));
    !package && archive::Format::of(path).is_some() && reach::meta(path, true).is_some_and(|m| m.is_file())
}

pub fn place(path: PathBuf) -> Where {
    if opens(&path) { Where::Inside(path, String::new()) } else { Where::Folder(path) }
}

pub fn inner(archive: &Path, path: &Path) -> Option<String> {
    let rest = path.strip_prefix(archive).ok()?;
    let parts: Vec<String> = rest.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
    (!parts.is_empty()).then(|| parts.join("/"))
}

pub fn within(path: &Path) -> Option<PathBuf> {
    path.ancestors().skip(1).find(|a| opens(a)).map(Path::to_path_buf)
}

fn taken_out_home() -> PathBuf {
    crate::common::files::cache_home().join("trindle-files").join("inside")
}

pub(super) fn open_dir(archive: &Path) -> PathBuf {
    let name = archive.file_name().map(|n| n.to_os_string()).unwrap_or_default();
    #[cfg(test)]
    let base = archive.parent().unwrap_or(Path::new("/")).join(".taken-out");
    #[cfg(not(test))]
    let base = taken_out_home();
    base.join(name)
}

pub(super) fn forget_taken_out() {
    std::thread::spawn(|| forget_under(&taken_out_home()));
}

fn forget_under(home: &Path) {
    if std::fs::symlink_metadata(home).is_ok_and(|m| m.is_dir()) {
        let _ = std::fs::remove_dir_all(home);
    }
}

impl Files {
    fn inside_off<T: Send + 'static>(&mut self, job: impl FnOnce() -> T + Send + 'static, done: impl FnOnce(T) -> InsideMsg + Send + 'static) {
        match &self.post {
            Some(post) => {
                let post = post.clone();
                let _ = std::thread::Builder::new().name("folders-inside".into()).spawn(move || post.send(Msg::Inside(done(job()))));
            }
            None => {
                let m = done(job());
                self.inside_msg(m);
            }
        }
    }

    pub(super) fn inside_list(&mut self, t: usize, p: usize) {
        let Where::Inside(a, f) = self.cols[p].tabs[t].at.clone() else { return };
        let now = stamp(&a);
        if let Some((when, all)) = self.inside.read.get(&a) {
            if *when == now {
                let entries = self.inside_entries(&a, &f, all);
                return self.fill(t, p, entries);
            }
        }
        if self.inside.waiting.contains(&a) {
            return;
        }
        self.inside.waiting.push(a.clone());
        let asked = a.clone();
        self.inside_off(move || archive::list(&asked, None).map_err(|e| Refusal::of(&e)),
                        move |got| InsideMsg::Listed { archive: a, stamp: now, got });
    }

    fn inside_entries(&self, a: &Path, folder: &str, all: &[archive::Entry]) -> Vec<Entry> {
        let mut out: Vec<Entry> = archive::inside(all, folder).into_iter().map(|e| {
            let name = e.name.rsplit('/').next().unwrap_or(&e.name).to_string();
            let kind = if e.folder { None } else { self.mime.of(&name) };
            Entry { path: a.join(&e.name), folder: e.folder, size: if e.folder { 0 } else { e.size }, when: e.modified.unwrap_or(0), kind, name }
        }).collect();
        let at = Where::Inside(a.to_path_buf(), folder.to_string());
        places::arrange(&mut out, &at, self.hidden, self.sort, self.up);
        out
    }

    pub(super) fn inside_open(&mut self, p: usize, items: Vec<PathBuf>) {
        let Where::Inside(a, _) = self.cols[p].at.clone() else { return };
        let entries = &self.cols[p].entries;
        let folder = |i: &PathBuf| entries.iter().any(|e| e.path == *i && e.folder);
        if let [one] = items.as_slice() {
            if folder(one) {
                let Some(name) = inner(&a, one) else { return };
                return self.go(p, Where::Inside(a, name));
            }
        }
        let files: Vec<PathBuf> = items.iter().filter(|i| !folder(i)).cloned().collect();
        if files.is_empty() {
            return;
        }
        self.inside_take_open(&a, files, None);
    }

    pub(super) fn inside_locked(&self, a: &Path, names: &[String]) -> bool {
        let Some((_, all)) = self.inside.read.get(a) else { return false };
        self.inside.names_locked.iter().any(|n| n == a) || all.iter().any(|e| e.locked && names.iter().any(|n| e.name == *n || e.name.strip_prefix(n.as_str()).is_some_and(|rest| rest.starts_with('/'))))
    }

    pub(super) fn inside_take(&mut self, a: &Path, items: Vec<PathBuf>, into: PathBuf) {
        let items: Vec<PathBuf> = items.into_iter().filter(|i| inner(a, i).is_some()).collect();
        if items.is_empty() {
            return self.say(t!("folders-inside-pick-first"), false);
        }
        if far::is_far(&into) {
            return self.say(t!("folders-inside-only-here"), false);
        }
        let names: Vec<String> = items.iter().filter_map(|i| inner(a, i)).collect();
        if self.inside_locked(a, &names) && self.unlock_free() {
            return self.unlock_ask(a.to_path_buf(), Then::Take(items, into), None);
        }
        self.send_to(items, into, work::Kind::Other(work::Extra::TakeOut(a.to_path_buf())));
    }

    pub(super) fn inside_take_open(&mut self, a: &Path, items: Vec<PathBuf>, key: Option<Key>) {
        let names: Vec<String> = items.iter().filter_map(|i| inner(a, i)).collect();
        if names.is_empty() {
            return;
        }
        if key.is_none() && self.inside_locked(a, &names) && self.unlock_free() {
            return self.unlock_ask(a.to_path_buf(), Then::Open(items), None);
        }
        let into = open_dir(a);
        let a = a.to_path_buf();
        let sealed = key.is_some();
        self.inside_off(move || {
            std::fs::create_dir_all(&into).map_err(|e| e.to_string())?;
            for n in &names {
                let old = into.join(n.rsplit('/').next().unwrap_or(n));
                let _ = if old.is_dir() { std::fs::remove_dir_all(&old) } else { std::fs::remove_file(&old) };
            }
            let stop = AtomicBool::new(false);
            archive::take_out(&a, key.as_ref(), &names, &into, &stop, &|_, _| {}).map(|u| (u.made, u.refused.len())).map_err(|e| archive::why(&e))
        }, move |got| InsideMsg::Out { got, sealed });
    }

    pub(super) fn inside_extract(&mut self, p: usize, items: Vec<PathBuf>) {
        let Where::Inside(a, _) = self.cols[p].at.clone() else { return };
        let Some(to) = a.parent().map(Path::to_path_buf) else { return };
        self.inside_take(&a, items, to);
    }

    pub(super) fn inside_msg(&mut self, m: InsideMsg) {
        match m {
            InsideMsg::Listed { archive: a, stamp, got } => {
                self.inside.waiting.retain(|w| *w != a);
                match got {
                    Ok(all) => {
                        if self.inside.read.len() >= KEEP && !self.inside.read.contains_key(&a) {
                            self.inside.read.clear();
                            self.inside.names_locked.clear();
                        }
                        self.inside.read.insert(a.clone(), (stamp, all));
                        for p in 0..2 {
                            for t in 0..self.cols[p].tabs.len() {
                                if matches!(&self.cols[p].tabs[t].at, Where::Inside(b, _) if *b == a) {
                                    self.inside_list(t, p);
                                }
                            }
                        }
                    }
                    Err(no) => {
                        for pane in self.cols.iter_mut().flat_map(|c| c.tabs.iter_mut()) {
                            if matches!(&pane.at, Where::Inside(b, _) if *b == a) {
                                pane.entries.clear();
                                pane.all.clear();
                            }
                        }
                        if no.key.is_some() && self.unlock_free() {
                            return self.unlock_ask(a, Then::List, None);
                        }
                        self.say_help(t!("folders-inside-unread", name = work::name_of(&a), why = &no.why), no.help);
                    }
                }
            }
            InsideMsg::Out { got, sealed } => match got {
                Ok((made, refused)) => {
                    if sealed {
                        self.unlocks.taken(&made);
                    }
                    if refused > 0 {
                        self.say_help(t!("folders-inside-refused", n = refused), super::help::ARCHIVE_NO_ROOM);
                    }
                    self.open_taken(&made);
                }
                Err(e) => self.say_help(t!("folders-inside-failed", why = &e), super::help::ARCHIVE_NOT_OPENED),
            },
        }
    }

    #[cfg(not(test))]
    fn open_taken(&mut self, made: &[PathBuf]) {
        for m in made {
            let _ = self.open_file(m);
        }
    }

    #[cfg(test)]
    fn open_taken(&mut self, _made: &[PathBuf]) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::trash::Bin;
    use crate::toolkit::ui::App;

    #[test]
    fn a_zip_opens_like_a_folder() {
        let dir = std::env::temp_dir().join(format!("trindle-files-inside-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src/docs")).unwrap();
        std::fs::write(dir.join("src/docs/note.txt"), b"hello").unwrap();
        std::fs::write(dir.join("src/top.txt"), b"top").unwrap();
        let zip = dir.join("box.zip");
        let stop = AtomicBool::new(false);
        archive::compress(&[dir.join("src/docs"), dir.join("src/top.txt")], &zip, archive::Format::Zip, &stop, &|_, _| {}).unwrap();
        assert!(opens(&zip));

        let mut f = Files::new(vec![dir.clone()]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.go(0, Where::Folder(dir.clone()));
        let i = f.cols[0].tabs[0].entries.iter().position(|e| e.name == "box.zip").unwrap();
        f.open(0, i);
        assert_eq!(f.cols[0].tabs[0].at, Where::Inside(zip.clone(), String::new()));
        let names: Vec<&str> = f.cols[0].tabs[0].entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["docs", "top.txt"]);

        f.inside_open(0, vec![zip.join("docs")]);
        assert_eq!(f.cols[0].tabs[0].at, Where::Inside(zip.clone(), "docs".into()));
        assert_eq!(f.cols[0].tabs[0].entries[0].name, "note.txt");
        let crumbs = places::crumbs(&f.cols[0].tabs[0].at, &f.home);
        assert_eq!(crumbs.last().unwrap().0, "docs");
        assert_eq!(f.cols[0].tabs[0].at.up(), Some(Where::Inside(zip.clone(), String::new())));
        assert_eq!(Where::Inside(zip.clone(), String::new()).up(), Some(Where::Folder(dir.clone())));

        f.inside_extract(0, vec![zip.join("docs/note.txt")]);
        assert_eq!(std::fs::read(dir.join("note.txt")).unwrap(), b"hello");
        assert!(!f.bad, "{}", f.said);
        f.inside_open(0, vec![zip.join("docs/note.txt")]);
        assert_eq!(std::fs::read(open_dir(&zip).join("note.txt")).unwrap(), b"hello");
        f.update(Msg::Choose(0, 0));
        f.update(Msg::Preview(crate::files::preview::PreviewMsg::Toggle));
        let Some(crate::files::dialogs::Dialog::Preview(pv)) = &f.dialog else { panic!("no viewer") };
        assert!(matches!(&pv.seen, crate::files::preview::Seen::Text { blocks, .. } if blocks[0] == "hello"));
        f.update(Msg::Preview(crate::files::preview::PreviewMsg::Toggle));
        f.cols[0].tabs[0].picked = vec![0];
        f.update(Msg::Trash);
        assert!(f.bin.as_ref().map(|b| b.files.read_dir().map(|d| d.count()).unwrap_or(0)).unwrap_or(0) == 0);

        std::fs::create_dir_all(dir.join("out")).unwrap();
        assert_eq!(within(&zip.join("docs/note.txt")), Some(zip.clone()));
        assert_eq!(within(&dir.join("out")), None);
        f.drop_into(vec![zip.join("docs")], Where::Folder(dir.join("out")), crate::files::act::Way::Move);
        assert_eq!(std::fs::read(dir.join("out/docs/note.txt")).unwrap(), b"hello");
        assert!(zip.is_file());
        f.bin.as_ref().unwrap().ready().unwrap();
        f.update(Msg::Undo);
        assert!(!dir.join("out/docs").exists());
        f.drop_into(vec![zip.join("top.txt")], Where::Trash, crate::files::act::Way::Usual);
        assert!(f.said.contains("only be taken out"), "{}", f.said);

        let _ = std::fs::remove_dir_all(open_dir(&zip).parent().unwrap());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_rar_opens_like_a_folder_and_a_locked_one_says_why() {
        use crate::files::unarchive::tests::{ar, rar5, Thing};
        let dir = std::env::temp_dir().join(format!("trindle-files-inside-rar-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("out")).unwrap();
        let things: [Thing; 3] = [(b"docs", 0o040_755, b""), (b"docs/note.txt", 0o100_644, b"hello"), (b"top.txt", 0o100_644, b"top")];
        let rar = dir.join("box.rar");
        std::fs::write(&rar, rar5(&things)).unwrap();
        assert!(opens(&rar));

        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        let i = f.cols[0].tabs[0].entries.iter().position(|e| e.name == "box.rar").unwrap();
        f.open(0, i);
        assert_eq!(f.cols[0].tabs[0].at, Where::Inside(rar.clone(), String::new()));
        let names: Vec<&str> = f.cols[0].tabs[0].entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["docs", "top.txt"], "{}", f.said);
        f.inside_extract(0, vec![rar.join("docs/note.txt")]);
        assert_eq!(std::fs::read(dir.join("note.txt")).unwrap(), b"hello");
        assert!(!f.bad, "{}", f.said);
        let plan = work::Plan { kind: work::Kind::Other(work::Extra::Unpack(archive::Place::Here)), items: vec![rar.clone()], to: dir.join("out"), key: None };
        let over = work::run(plan, &work::Ctl::default(), &|_| {});
        assert!(over.failed.is_empty() && !over.stopped, "{:?}", over.failed);
        assert_eq!(over.undo, vec![work::Step::Remove(dir.join("out/box"))]);
        assert_eq!(std::fs::read(dir.join("out/box/docs/note.txt")).unwrap(), b"hello");

        let f = Files::new(vec![rar.clone()]);
        assert_eq!(f.cols[0].tabs[0].at, Where::Inside(rar.clone(), String::new()));

        let locked = dir.join("locked.rar");
        std::fs::write(&locked, include_bytes!("../../tests/fixtures/archives/rar5-locked.rar")).unwrap();
        let plan = work::Plan { kind: work::Kind::Other(work::Extra::Unpack(archive::Place::Folder)), items: vec![locked], to: dir.join("out"), key: None };
        let over = work::run(plan, &work::Ctl::default(), &|_| {});
        assert_eq!(over.failed, ["locked.rar (it is a RAR locked with a password, which Folders cannot unlock; nothing is wrong with the file)"]);
        assert!(over.undo.is_empty() && !dir.join("out/locked").exists());
        std::fs::write(dir.join("fake.rar"), b"not a rar").unwrap();
        let mut f = Files::new(vec![dir.join("fake.rar")]);
        f.load(0, 0);
        assert!(f.bad && f.said == "fake.rar could not be read: it is not an archive Folders can open.", "{}", f.said);

        std::fs::write(dir.join("some.deb"), ar(&[("debian-binary", b"2.0\n")])).unwrap();
        assert!(!opens(&dir.join("some.deb")) && archive::Format::of(&dir.join("some.deb")).is_some());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn taking_out_tells_how_far_it_has_got() {
        let dir = std::env::temp_dir().join(format!("trindle-files-inside-told-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();
        let mut x = 0x2545_f491_u32;
        let big: Vec<u8> = (0..3 << 20).map(|_| { x ^= x << 13; x ^= x >> 17; x ^= x << 5; x as u8 }).collect();
        std::fs::write(dir.join("src/big"), big).unwrap();
        let zip = dir.join("box.zip");
        let stop = AtomicBool::new(false);
        archive::compress(&[dir.join("src/big")], &zip, archive::Format::Zip, &stop, &|_, _| {}).unwrap();
        std::fs::create_dir_all(dir.join("out")).unwrap();

        let told = std::cell::RefCell::new(Vec::new());
        let plan = work::Plan { kind: work::Kind::Other(work::Extra::TakeOut(zip.clone())), items: vec![zip.join("big")], to: dir.join("out"), key: None };
        let over = work::run(plan, &work::Ctl::default(), &|n| told.borrow_mut().push(n));
        assert!(over.failed.is_empty() && !over.stopped);
        assert_eq!(over.undo, vec![work::Step::Remove(dir.join("out/big"))]);
        let told = told.into_inner();
        assert!(told.iter().any(|n| matches!(n, work::News::Total(t) if *t > 0)));
        assert!(told.iter().filter(|n| matches!(n, work::News::Done(_))).count() > 1);
        assert_eq!(std::fs::metadata(dir.join("out/big")).unwrap().len(), 3 << 20);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn what_an_earlier_folders_took_out_is_forgotten_and_a_link_is_not_followed() {
        let dir = std::env::temp_dir().join(format!("trindle-taken-out-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (home, kept) = (dir.join("inside"), dir.join("kept"));
        std::fs::create_dir_all(home.join("notes.zip")).unwrap();
        std::fs::create_dir_all(&kept).unwrap();
        std::fs::write(home.join("notes.zip").join("secret.txt"), "unlocked").unwrap();
        std::fs::write(kept.join("mine.txt"), "mine").unwrap();
        super::forget_under(&home);
        assert!(!home.exists());
        std::os::unix::fs::symlink(&kept, &home).unwrap();
        super::forget_under(&home);
        assert!(kept.join("mine.txt").is_file());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
