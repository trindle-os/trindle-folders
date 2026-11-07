// Reading a place on this computer on a thread of its own.

use std::collections::HashSet;
use std::os::fd::OwnedFd;
use std::path::PathBuf;
use std::time::Duration;

use super::app::{Files, Msg};
use super::kinds::Mime;
use super::places::{self, Entry, Key, Where};
use crate::common::run::Slots;

pub(super) const LATE: Duration = Duration::from_millis(if cfg!(test) { 150 } else { 3000 });

pub(super) fn not_answering() -> &'static str {
    super::reach::not_answering()
}

static OUT: Slots = Slots::new(8);

#[derive(Clone)]
pub struct Got {
    pub id: u64,
    pub asked: Where,
    pub at: Where,
    pub error: Option<String>,
    pub free: Option<u64>,
    pub entries: Vec<Entry>,
    pub marks: Vec<(PathBuf, i32)>,
    pub trash: Option<Vec<PathBuf>>,
}

pub(super) struct Look {
    id: u64,
    hidden: bool,
    key: Key,
    up: bool,
    mime: std::sync::Arc<Mime>,
    ear: Option<OwnedFd>,
    watched: HashSet<PathBuf>,
}

pub(super) fn read(asked: Where, look: Look) -> Got {
    let Look { id, hidden, key, up, mime, ear, watched } = look;
    #[cfg(test)]
    hold(&asked);
    let mut at = asked.clone();
    while let Where::Folder(f) = &at {
        if f.is_dir() {
            break;
        }
        match at.up() {
            Some(higher) => at = higher,
            None => break,
        }
    }
    let error = match &at {
        Where::Folder(f) => std::fs::read_dir(f).err().map(|e| places::unread_words(&e)),
        _ => None,
    };
    let free = match &at {
        Where::Folder(f) => crate::common::files::room_left(f),
        _ => None,
    };
    let mut marks = Vec::new();
    let mut watch = |folder: &PathBuf| {
        if watched.contains(folder) {
            return;
        }
        if let Some(mark) = ear.as_ref().and_then(|e| crate::toolkit::watch::add(e, folder).ok()) {
            marks.push((folder.clone(), mark));
        }
    };
    let mut trash = None;
    match &at {
        Where::Folder(f) => watch(f),
        Where::Trash => {
            let bins = super::trash::all_files();
            for bin in &bins {
                watch(bin);
            }
            trash = Some(bins);
        }
        _ => {}
    }
    let entries = places::list(&at, hidden, key, up, &mime);
    Got { id, asked, at, error, free, entries, marks, trash }
}

impl Files {
    pub(super) fn read_aside(&mut self, t: usize, p: usize) {
        self.reads += 1;
        let id = self.reads;
        let (hidden, key, up) = (self.hidden, self.sort, self.up);
        let look = Look { id, hidden, key, up, mime: self.mime.clone(), ear: self.watch_ear(), watched: self.watched.clone() };
        let pane = &mut self.cols[p].tabs[t];
        (pane.read, pane.reading, pane.late) = (id, true, false);
        let asked = pane.at.clone();
        let Some(post) = self.post.clone() else {
            let got = read(asked, look);
            return self.read_arrived(got);
        };
        let Some(slot) = OUT.take() else {
            self.cols[p].tabs[t].late = true;
            return;
        };
        let back = post.clone();
        let spawned = std::thread::Builder::new().name("folders-read".into()).spawn(move || {
            let _slot = slot;
            back.send(Msg::Read(Box::new(read(asked, look))));
        });
        if spawned.is_err() {
            self.cols[p].tabs[t].late = true;
            return;
        }
        let _ = std::thread::Builder::new().name("folders-read-wait".into()).spawn(move || {
            std::thread::sleep(LATE);
            post.send(Msg::ReadLate(id));
        });
    }

    fn pane_reading(&self, id: u64) -> Option<(usize, usize)> {
        (0..2).find_map(|p| self.cols[p].tabs.iter().position(|pane| pane.read == id).map(|t| (t, p)))
    }

    pub(super) fn read_late(&mut self, id: u64) {
        if let Some((t, p)) = self.pane_reading(id) {
            let pane = &mut self.cols[p].tabs[t];
            pane.late = pane.reading;
        }
    }

    pub(super) fn read_arrived(&mut self, got: Got) {
        for (folder, mark) in &got.marks {
            if let Some(w) = &mut self.watch {
                w.known(*mark, folder);
            }
            self.watched.insert(folder.clone());
        }
        if let Some(bins) = got.trash.clone() {
            self.trash_dirs = bins;
        }
        let Some((t, p)) = self.pane_reading(got.id) else { return };
        let pane = &mut self.cols[p].tabs[t];
        pane.reading = false;
        pane.late = false;
        if pane.at != got.asked {
            return;
        }
        pane.at = got.at;
        pane.error = got.error;
        pane.free = got.free;
        let mut entries = got.entries;
        if self.cols[p].tabs[t].at == Where::Network {
            entries.extend(self.neighbour_entries());
            places::sort(&mut entries, self.sort, self.up);
        }
        self.fill(t, p, entries);
        if self.cols[p].tab == t {
            if let Some(l) = self.listed.take_if(|l| l.side == p) {
                if let Some(path) = l.choose {
                    self.choose_path(p, &path);
                }
                if let Some((path, fresh)) = l.rename {
                    self.rename_path(p, &path, fresh);
                }
            }
        }
    }

    pub(super) fn when_read(&mut self, p: usize, choose: Option<PathBuf>, rename: Option<(PathBuf, bool)>) {
        let waiting = self.cols[p].tabs.get(self.cols[p].tab).is_some_and(|pane| pane.reading);
        if waiting {
            self.listed = Some(super::far::Listed { side: p, choose, rename });
            return;
        }
        if let Some(path) = choose {
            self.choose_path(p, &path);
        }
        if let Some((path, fresh)) = rename {
            self.rename_path(p, &path, fresh);
        }
    }

    pub(super) fn give_up(&mut self, p: usize) {
        let t = self.cols[p].tab;
        let pane = &mut self.cols[p].tabs[t];
        (pane.read, pane.reading, pane.late) = (0, false, false);
        if !pane.back.is_empty() {
            self.retrace(p, false);
        } else if pane.at != Where::Folder(self.home.clone()) {
            let home = self.home.clone();
            self.go(p, Where::Folder(home));
        }
    }

    pub(super) fn not_answering(pane: &super::app::Pane) -> Option<&'static str> {
        (pane.reading && pane.late).then_some(not_answering())
    }
}

#[cfg(test)]
static STALLED: std::sync::Mutex<Vec<PathBuf>> = std::sync::Mutex::new(Vec::new());

#[cfg(test)]
fn hold(at: &Where) {
    while let Where::Folder(f) = at {
        if !STALLED.lock().unwrap_or_else(|e| e.into_inner()).contains(f) {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::bus::testing::until;
    use crate::toolkit::event::Loop;
    use crate::toolkit::ui::App;
    use std::path::Path;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn stall(folder: &PathBuf, on: bool) {
        let mut all = STALLED.lock().unwrap();
        all.retain(|f| f != folder);
        if on {
            all.push(folder.clone());
        }
    }

    fn folder(name: &str, files: &[&str]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-reading-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for f in files {
            std::fs::write(dir.join(f), "x").unwrap();
        }
        dir
    }

    fn running(lp: &Loop, at: &Path) -> Rc<RefCell<Files>> {
        let files = Rc::new(RefCell::new(Files::new(vec![at.to_path_buf()])));
        let f = files.clone();
        let post = lp.post(move |m: Msg| f.borrow_mut().update(m));
        files.borrow_mut().set_post(post);
        files
    }

    #[test]
    fn a_place_that_is_not_answering_holds_up_nothing() {
        let (live, dead) = (folder("live", &["a.txt", "b.txt"]), folder("dead", &["lost.txt"]));
        let lp = Loop::new();
        let files = running(&lp, &live);
        let f = files.clone();
        assert!(until(&lp, move || f.borrow().cols[0].tabs[0].entries.len() == 2), "the first folder was never shown");

        stall(&dead, true);
        let began = std::time::Instant::now();
        files.borrow_mut().go(0, Where::Folder(dead.clone()));
        assert!(began.elapsed() < Duration::from_millis(100), "going to a folder that is not answering waited for it");
        {
            let f = files.borrow();
            let pane = &f.cols[0].tabs[0];
            assert!(pane.reading && pane.entries.is_empty(), "the pane showed the old folder's rows as the new one's");
            assert_eq!(Files::not_answering(pane), None, "it was said not to answer before it was late");
        }
        let f = files.clone();
        assert!(until(&lp, move || Files::not_answering(&f.borrow().cols[0].tabs[0]).is_some()), "a read that never came back was not said to be not answering");
        let words = super::super::view::status_words(&files.borrow(), &files.borrow().cols[0].tabs[0], 0, true);
        assert!(words.contains(not_answering()), "{words}");

        files.borrow_mut().go(0, Where::Folder(live.clone()));
        let f = files.clone();
        assert!(until(&lp, move || f.borrow().cols[0].tabs[0].entries.len() == 2), "another folder could not be shown while one was not answering");
        assert_eq!(Files::not_answering(&files.borrow().cols[0].tabs[0]), None);

        stall(&dead, false);
        std::thread::sleep(Duration::from_millis(100));
        let f = files.clone();
        let _ = until(&lp, move || { let _ = &f; false });
        let f = files.borrow();
        assert_eq!(f.cols[0].tabs[0].at, Where::Folder(live.clone()));
        assert_eq!(f.cols[0].tabs[0].entries.len(), 2, "a read of the place left showed in the place now shown");
    }

    #[test]
    fn loading_a_place_looks_at_the_disk_only_on_a_thread() {
        let app = crate::common::source::code(crate::files::app::SOURCE);
        let from = app.find("pub(in crate::files) fn load(").expect("load");
        let body = &app[from..];
        let body = &body[..body.find("\n    }\n").expect("the end of load")];
        for look in [".is_dir()", ".exists()", "read_dir(", "room_left(", "places::list(", "metadata(", "all_files(", ".watch_also("] {
            assert!(!body.contains(look), "`load` looks at the disk with {look}, on the thread that draws Folders");
        }
        assert!(body.contains("self.read_aside(t, p)"));
        for (name, text) in [("act", crate::common::source::code_all(crate::files::act::SOURCE)), ("menu.rs", crate::common::source::code(crate::files::menu::SOURCE)), ("inplace.rs", crate::common::source::code(include_str!("inplace.rs")))] {
            assert!(!text.contains("reading::read("), "{name} reads a place on the spot");
        }
    }

    #[test]
    fn cancel_leaves_a_place_that_is_not_answering_and_changes_nothing() {
        let (live, dead) = (folder("cancel-live", &["a.txt"]), folder("cancel-dead", &["lost.txt"]));
        let lp = Loop::new();
        let files = running(&lp, &live);
        let f = files.clone();
        assert!(until(&lp, move || f.borrow().cols[0].tabs[0].entries.len() == 1));
        stall(&dead, true);
        files.borrow_mut().go(0, Where::Folder(dead.clone()));
        let f = files.clone();
        assert!(until(&lp, move || Files::not_answering(&f.borrow().cols[0].tabs[0]).is_some()));
        assert_eq!(not_answering(), "The other computer is not answering.");
        files.borrow_mut().update(Msg::GiveUp);
        {
            let f = files.borrow();
            let pane = &f.cols[0].tabs[0];
            assert!(!pane.late && Files::not_answering(pane).is_none(), "the pane still said the place was not answering");
            assert_eq!(pane.at, Where::Folder(live.clone()));
        }
        let f = files.clone();
        assert!(until(&lp, move || f.borrow().cols[0].tabs[0].entries.len() == 1), "the place gone back to was not read again");
        stall(&dead, false);
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(files.borrow().cols[0].tabs[0].at, Where::Folder(live));
        assert!(dead.join("lost.txt").is_file(), "something in the place was changed by giving up on it");
    }

    #[test]
    fn a_place_that_never_answers_holds_no_look_on_the_window_thread() {
        use super::super::columns::Col;
        let dead = folder("reach-dead", &["a.txt", "b.txt", "c.txt"]);
        let named = |n: &str| Entry { name: n.into(), path: dead.join(n), folder: false, size: 1, when: 7, kind: None };
        let f = Files::new(vec![std::env::temp_dir()]);
        super::super::reach::never(&dead, true);
        let began = std::time::Instant::now();
        for _ in 0..20 {
            for n in ["a.txt", "b.txt", "c.txt"] {
                let e = named(n);
                assert_eq!(f.cache.cells(&e, &[Col::Rights, Col::Owner, Col::Made]), ["", "", ""]);
                assert_eq!(f.shown.hidden_from(&e, false), None, "an ending was hidden by a file that was not opened");
            }
            assert_eq!(super::super::act::make_new(&dead, true), Err(not_answering().to_string()));
            assert!(!super::super::act::pinnable(std::slice::from_ref(&dead)));
        }
        assert!(began.elapsed() < Duration::from_millis(1500), "the looks at a place that never answers took {:?}", began.elapsed());
        super::super::reach::never(&dead, false);
        std::thread::sleep(super::super::reach::SILENT_FOR + Duration::from_millis(50));
        assert_eq!(std::fs::read_dir(&dead).unwrap().count(), 3, "something was made in a place that did not answer");
        let e = named("a.txt");
        assert_ne!(f.cache.cells(&e, &[Col::Rights])[0], "", "a place that answers again was still left alone");
    }

    #[test]
    fn the_window_thread_looks_at_places_only_through_reach() {
        let files = [
            ("menu.rs", crate::common::source::code(crate::files::menu::SOURCE)),
            ("tools.rs", crate::common::source::code(include_str!("tools.rs"))),
            ("clip.rs", crate::common::source::code(include_str!("clip.rs"))),
            ("view.rs", crate::common::source::code(include_str!("view.rs"))),
        ];
        for (name, text) in files {
            let code = &text[..text.find("#[cfg(test)]").unwrap_or(text.len())];
            for look in [".is_dir()", ".exists()", ".is_file()", "fs::metadata(", "symlink_metadata(", "read_dir("] {
                assert!(!code.contains(look), "{name} looks at a place on the window's thread with {look}, not through `reach`");
            }
        }
    }

    #[test]
    fn nothing_in_the_folders_window_touches_its_neighbour() {
        use crate::toolkit::{theme::Theme, ui::Ui};
        let dir = folder("touching", &["a.txt", "b.txt"]).join("Papers");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.txt"), "made up").unwrap();
        let mut f = Files::new(vec![dir]);
        let mut wrong = Vec::new();
        for late in [false, true] {
            if late {
                f.cols[0].tabs[0].entries.clear();
                (f.cols[0].tabs[0].reading, f.cols[0].tabs[0].late) = (true, true);
            }
            for w in [900, 1278, 1920] {
                f.resized(w as f32, 692.0);
                for _ in 0..2 {
                    let mut ui = Ui::new(super::super::view::view(&f), Theme::new(true, "#fbc711"));
                    let mut pix = tiny_skia::Pixmap::new(w, 692).unwrap();
                    ui.paint(&mut pix.as_mut(), 1.0);
                }
                let mut ui = Ui::new(super::super::view::view(&f), Theme::new(true, "#fbc711"));
                ui.lay_at(w, 692);
                wrong.extend(ui.squeezed_or_touching().into_iter().map(|found| format!("late {late} at {w}: {found}")));
            }
        }
        wrong.dedup();
        assert!(wrong.is_empty(), "{} things touch or are squeezed:\n{}", wrong.len(), wrong.join("\n"));
    }

}
