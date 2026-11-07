// Asking for the password of a locked zip or 7z.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use zeroize::Zeroizing;

use super::app::{Files, Msg};
use super::archive::{self, Key, Place, Trouble};
use super::dialogs::{foot, head_to, Dialog};
use super::icons::stand_in;
use super::inside::{InsideMsg, Refusal};
use super::places::Where;
use super::work::{self, Extra, Kind};
use crate::toolkit::t;
use crate::toolkit::tiles::standard_ring;
use crate::toolkit::widget::{card, column, entry, main_action, note, row, Length, Node, Tone};

const WIDTH: f32 = 440.0;

#[derive(Clone)]
pub enum Then {
    List,
    Open(Vec<PathBuf>),
    Take(Vec<PathBuf>, PathBuf),
    View(usize, PathBuf),
    Unpack(Place, PathBuf),
}

pub struct Unlock {
    pub archive: PathBuf,
    pub password: Zeroizing<String>,
    pub wrong: Option<String>,
    pub busy: bool,
    pub then: Then,
    id: u64,
}

#[derive(Default)]
pub struct Waiting {
    queue: VecDeque<Unlock>,
    taken: Vec<PathBuf>,
    kept: Vec<(PathBuf, Option<SystemTime>, Key)>,
}

impl Waiting {
    pub(super) fn taken(&mut self, made: &[PathBuf]) {
        self.taken.extend_from_slice(made);
    }

    pub(super) fn tidy(&self) {
        for made in &self.taken {
            if std::fs::remove_file(made).is_err() {
                let _ = std::fs::remove_dir_all(made);
            }
        }
    }
}

impl Drop for Waiting {
    fn drop(&mut self) {
        self.tidy();
    }
}

type Listing = (Option<SystemTime>, Vec<archive::Entry>);

#[derive(Clone)]
pub enum UnlockMsg {
    Password(String),
    Go,
    Tried(u64, Result<Option<Listing>, Refusal>),
    Relisted(PathBuf, Result<Listing, Refusal>),
    Close,
}

fn msg(m: UnlockMsg) -> Msg {
    Msg::Unlock(m)
}

pub(super) fn sentence(t: Trouble) -> String {
    let words = t.to_string();
    let mut letters = words.chars();
    match letters.next() {
        Some(first) => format!("{}{}.", first.to_uppercase(), letters.as_str()),
        None => words,
    }
}

pub fn view(u: &Unlock, turn: u32) -> Node<Msg> {
    let name = work::name_of(&u.archive);
    let (icon, action) = match u.then {
        Then::Unpack(..) | Then::Take(..) => ("archive-extract-symbolic", t!("folders-locked-extract")),
        _ => ("document-open-symbolic", t!("folders-locked-open")),
    };
    let typed = entry(u.password.to_string(), t!("folders-locked-password"), |t| msg(UnlockMsg::Password(t)))
        .key("unlockpassword").secret().grab(turn).on_submit(msg(UnlockMsg::Go)).on_escape(msg(UnlockMsg::Close)).width(Length::Fill);
    let under = match &u.wrong {
        Some(why) => note(why.clone()).tone(Tone::Problem),
        None => note(t!("folders-locked-kept-note")),
    };
    let mut left = Vec::new();
    if u.wrong.is_some() {
        left.push(crate::common::more::button(Msg::Help(super::help::ARCHIVE_PASSWORD)));
    }
    if u.busy {
        left.push(row(vec![note(t!("folders-locked-trying")), standard_ring(None, 32.0)]).gap(12.0).centred());
    }
    let body = vec![
        head_to(stand_in::UNLOCK, t!("folders-locked-title", name = &name), t!("folders-locked-close"), msg(UnlockMsg::Close)),
        note(t!("folders-locked-is-locked", name = &name)),
        typed,
        under.key("unlockwhy"),
        foot(left, main_action(icon, action).on_press(msg(UnlockMsg::Go)).off(u.password.is_empty()).off_because(crate::common::ui::wait_while(u.busy))),
    ];
    card(column(body).gap(12.0).width(Length::Fill)).width(Length::Fixed(WIDTH)).on_escape(msg(UnlockMsg::Close))
}

impl Files {
    fn unlock_off<T: Send + 'static>(&mut self, job: impl FnOnce() -> T + Send + 'static, done: impl FnOnce(T) -> UnlockMsg + Send + 'static) {
        match &self.post {
            Some(post) => {
                let post = post.clone();
                let _ = std::thread::Builder::new().name("folders-unlock".into()).spawn(move || post.send(Msg::Unlock(done(job()))));
            }
            None => {
                let m = done(job());
                self.unlock_msg(m);
            }
        }
    }

    pub(super) fn unlock_free(&self) -> bool {
        matches!(self.dialog, None | Some(Dialog::Unlock(_)))
    }

    pub(super) fn unlock_ask(&mut self, archive: PathBuf, then: Then, wrong: Option<Trouble>) {
        self.unlock_forget_closed();
        if wrong.is_some() {
            self.unlocks.kept.retain(|(a, ..)| *a != archive);
        } else if let Some(key) = self.unlock_kept(&archive) {
            return self.unlock_with_kept(archive, then, key);
        }
        let listing = |u: &Unlock| matches!(u.then, Then::List) && matches!(then, Then::List) && u.archive == archive;
        let showing = match &self.dialog {
            None => false,
            Some(Dialog::Unlock(u)) => {
                if listing(u) || self.unlocks.queue.iter().any(listing) {
                    return;
                }
                true
            }
            Some(_) => return self.say_help(t!("folders-locked-open-again", name = &work::name_of(&archive)), super::help::ARCHIVE_PASSWORD),
        };
        self.next_job += 1;
        let ask = Unlock { archive, password: Zeroizing::new(String::new()), wrong: wrong.map(sentence), busy: false, then, id: self.next_job };
        if showing {
            self.unlocks.queue.push_back(ask);
        } else {
            self.unlock_show(ask);
        }
    }

    fn unlock_show(&mut self, ask: Unlock) {
        self.dialog = Some(Dialog::Unlock(ask));
        self.turn += 1;
    }

    fn unlock_next(&mut self) {
        if self.dialog.is_none() {
            if let Some(next) = self.unlocks.queue.pop_front() {
                self.unlock_show(next);
            }
        }
    }

    fn unlock_leave(&mut self, archive: &Path) {
        let side = self.side;
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                let front = t == self.cols[p].tab;
                let pane = &mut self.cols[p].tabs[t];
                if !matches!(&pane.at, Where::Inside(a, _) if a == archive) {
                    continue;
                }
                let Some(up) = Where::Inside(archive.to_path_buf(), String::new()).up() else { continue };
                if pane.back.last() == Some(&up) {
                    pane.back.pop();
                }
                if front {
                    self.arrive(p, up);
                } else {
                    pane.at = up;
                    self.load(t, p);
                }
            }
        }
        self.side = side;
    }

    fn unlock_open(&self, archive: &Path) -> bool {
        self.cols.iter().any(|c| c.tabs.iter().any(|t| matches!(&t.at, Where::Inside(a, _) if a == archive)))
    }

    pub(super) fn unlock_forget_closed(&mut self) {
        if self.unlocks.kept.is_empty() {
            return;
        }
        let mut kept = std::mem::take(&mut self.unlocks.kept);
        kept.retain(|(a, stamp, _)| self.unlock_open(a) && super::inside::stamp(a) == *stamp);
        self.unlocks.kept = kept;
    }

    fn unlock_kept(&self, archive: &Path) -> Option<Key> {
        self.unlocks.kept.iter().find(|(a, ..)| a == archive).map(|(_, _, k)| k.clone())
    }

    fn unlock_with_kept(&mut self, archive: PathBuf, then: Then, key: Key) {
        if !matches!(then, Then::List) {
            return self.unlock_do(archive, then, key, None);
        }
        let (a, stamp) = (archive.clone(), super::inside::stamp(&archive));
        self.unlock_off(move || archive::list(&a, Some(&key)).map(|all| (stamp, all)).map_err(|e| Refusal::of(&e)),
                        move |got| UnlockMsg::Relisted(archive, got));
    }

    fn unlock_then(&mut self, u: Unlock, listing: Option<Listing>) {
        let key = Key::new(&u.password);
        if self.unlock_open(&u.archive) {
            self.unlocks.kept.retain(|(a, ..)| *a != u.archive);
            self.unlocks.kept.push((u.archive.clone(), super::inside::stamp(&u.archive), key.clone()));
        }
        self.unlock_do(u.archive.clone(), u.then.clone(), key, listing);
    }

    fn unlock_do(&mut self, archive: PathBuf, then: Then, key: Key, listing: Option<Listing>) {
        match then {
            Then::List => {
                if let Some((stamp, all)) = listing {
                    if !self.inside.names_locked.contains(&archive) {
                        self.inside.names_locked.push(archive.clone());
                    }
                    self.inside_msg(InsideMsg::Listed { archive, stamp, got: Ok(all) });
                }
            }
            Then::Open(items) => self.inside_take_open(&archive, items, Some(key)),
            Then::Take(items, into) => self.send_with(items, into, Kind::Other(Extra::TakeOut(archive)), Some(key)),
            Then::View(p, file) => self.preview_unlocked(p, &file, key),
            Then::Unpack(place, to) => self.send_with(vec![archive], to, Kind::Other(Extra::Unpack(place)), Some(key)),
        }
    }

    pub(super) fn unlock_msg(&mut self, m: UnlockMsg) {
        match m {
            UnlockMsg::Password(text) => {
                if let Some(Dialog::Unlock(u)) = &mut self.dialog {
                    u.password = Zeroizing::new(text);
                }
            }
            UnlockMsg::Go => self.unlock_go(),
            UnlockMsg::Tried(id, got) => self.unlock_tried(id, got),
            UnlockMsg::Relisted(archive, got) => self.unlock_relisted(archive, got),
            UnlockMsg::Close => {
                let Some(Dialog::Unlock(u)) = self.dialog.take() else { return };
                if matches!(u.then, Then::List) {
                    self.unlock_leave(&u.archive);
                }
                self.unlock_next();
            }
        }
    }

    fn unlock_go(&mut self) {
        let Some(Dialog::Unlock(u)) = &mut self.dialog else { return };
        if u.busy || u.password.is_empty() {
            return;
        }
        u.busy = true;
        let (id, a, key, list) = (u.id, u.archive.clone(), Key::new(&u.password), matches!(u.then, Then::List));
        let stamp = super::inside::stamp(&a);
        self.unlock_off(move || {
            let got = if list { archive::list(&a, Some(&key)).map(|all| Some((stamp, all))) } else { archive::unlocks(&a, &key).map(|()| None) };
            got.map_err(|e| Refusal::of(&e))
        }, move |got| UnlockMsg::Tried(id, got));
    }

    fn unlock_tried(&mut self, id: u64, got: Result<Option<Listing>, Refusal>) {
        let Some(Dialog::Unlock(u)) = &mut self.dialog else { return };
        if u.id != id {
            return;
        }
        u.busy = false;
        match got {
            Err(Refusal { key: Some(t), .. }) => u.wrong = Some(sentence(t)),
            Err(no) => {
                let Some(Dialog::Unlock(u)) = self.dialog.take() else { return };
                if matches!(u.then, Then::List) {
                    self.inside_msg(InsideMsg::Listed { archive: u.archive, stamp: None, got: Err(no) });
                } else {
                    self.say_help(t!("folders-locked-could-not-read", name = &work::name_of(&u.archive), why = &no.why), no.help);
                }
                self.unlock_next();
            }
            Ok(listing) => {
                let Some(Dialog::Unlock(u)) = self.dialog.take() else { return };
                self.unlock_then(u, listing);
                self.unlock_next();
            }
        }
    }

    fn unlock_relisted(&mut self, archive: PathBuf, got: Result<Listing, Refusal>) {
        match got {
            Ok(listing) => {
                if let Some(key) = self.unlock_kept(&archive) {
                    self.unlock_do(archive, Then::List, key, Some(listing));
                }
            }
            Err(Refusal { key: Some(t), .. }) => self.unlock_ask(archive, Then::List, Some(t)),
            Err(no) => self.inside_msg(InsideMsg::Listed { archive, stamp: None, got: Err(no) }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::inside::open_dir;
    use crate::files::preview::{PreviewMsg, Seen};
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App, Ui};

    const KEY: &str = "john-smith-test";
    const HELLO: &[u8] = b"Hello from John Smith.\n";
    const WRONG: &str = "That password does not open it.";
    const WRONG_OR_BAD: &str = "That password does not open it, or the archive is damaged.";

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-locked-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn kept(dir: &Path, name: &str) -> PathBuf {
        let bytes: &[u8] = match name {
            "locked-old.zip" => include_bytes!("../../tests/fixtures/archives/locked-old.zip"),
            "locked-aes.zip" => include_bytes!("../../tests/fixtures/archives/locked-aes.zip"),
            "locked.7z" => include_bytes!("../../tests/fixtures/archives/locked.7z"),
            "locked-names.7z" => include_bytes!("../../tests/fixtures/archives/locked-names.7z"),
            "locked-odd.7z" => include_bytes!("../../tests/fixtures/archives/locked-odd.7z"),
            _ => include_bytes!("../../tests/fixtures/archives/rar5-locked.rar"),
        };
        let path = dir.join(name);
        std::fs::write(&path, bytes).unwrap();
        path
    }

    fn card(f: &Files) -> &Unlock {
        match &f.dialog {
            Some(Dialog::Unlock(u)) => u,
            _ => panic!("no card asks for a password: {}", f.said),
        }
    }

    fn try_with(f: &mut Files, password: &str) {
        f.update(Msg::Unlock(UnlockMsg::Password(password.into())));
        f.update(Msg::Unlock(UnlockMsg::Go));
    }

    fn count(dir: &Path) -> usize {
        std::fs::read_dir(dir).unwrap().count()
    }

    fn pressable(f: &Files, tip: &str) -> Option<bool> {
        view(card(f), f.turn).buttons().into_iter().find(|(t, _)| t == tip).map(|(_, on)| on)
    }

    #[test]
    fn extracting_a_locked_zip_or_7z_asks_for_its_password_and_a_wrong_one_is_said_under_the_box() {
        let dir = place("extract");
        for (name, stem, wrong) in [("locked-old.zip", "locked-old", WRONG), ("locked-aes.zip", "locked-aes", WRONG), ("locked.7z", "locked", WRONG_OR_BAD)] {
            let arc = kept(&dir, name);
            let out = dir.join(format!("out-{stem}"));
            std::fs::create_dir(&out).unwrap();
            let mut f = Files::new(vec![dir.clone()]);
            f.send_to(vec![arc.clone()], out.clone(), Kind::Other(Extra::Unpack(Place::Folder)));
            let u = card(&f);
            assert!(u.wrong.is_none() && !u.busy && u.password.is_empty() && matches!(u.then, Then::Unpack(..)), "{name}");
            assert!(!f.bad && f.said.is_empty(), "{name}: {}", f.said);
            assert_eq!(count(&out), 0, "{name}");
            assert_eq!(pressable(&f, "Extract"), Some(false), "{name}: nothing to try until something is typed");
            try_with(&mut f, "wrong one");
            let u = card(&f);
            assert_eq!(u.wrong.as_deref(), Some(wrong), "{name}");
            assert!(!u.busy && u.password.as_str() == "wrong one", "{name}");
            assert!(view(u, f.turn).said().contains(&wrong.to_string()), "{name}");
            assert!(!f.bad && count(&out) == 0, "{name}: {}", f.said);
            assert_eq!(pressable(&f, "Extract"), Some(true), "{name}");
            try_with(&mut f, KEY);
            assert!(f.dialog.is_none() && !f.bad, "{name}: {}", f.said);
            assert!(f.said.starts_with("Extracted "), "{name}: {}", f.said);
            assert_eq!(std::fs::read(out.join(stem).join("hello.txt")).unwrap(), HELLO, "{name}");
            assert_eq!(std::fs::read(out.join(stem).join("docs/note.txt")).unwrap().len(), 98, "{name}");
            let other = dir.join(format!("other-{stem}"));
            std::fs::create_dir(&other).unwrap();
            f.send_to(vec![arc], other.clone(), Kind::Other(Extra::Unpack(Place::Here)));
            f.update(Msg::Unlock(UnlockMsg::Password("john".into())));
            f.update(Msg::Unlock(UnlockMsg::Close));
            assert!(f.dialog.is_none() && !f.bad && f.said.is_empty(), "{name}: {}", f.said);
            assert_eq!(count(&other), 0, "{name}");
        }
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_7z_with_its_names_locked_asks_before_it_shows_anything_and_keeps_the_password_only_while_it_is_open() {
        let dir = place("names");
        let arc = kept(&dir, "locked-names.7z");
        let inside = Where::Inside(arc.clone(), String::new());
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        names_are_asked_for_first(&mut f, &dir, &inside);
        let taken = the_kept_password_serves_what_is_inside(&mut f, &dir, &arc, &inside);
        leaving_wipes_the_password_and_a_changed_archive_asks_again(&mut f, &dir, &arc);

        assert!(taken.exists());
        drop(f);
        assert!(!taken.exists());
        let _ = std::fs::remove_dir_all(dir);
    }

    fn at(f: &Files, name: &str) -> usize {
        f.cols[0].entries.iter().position(|e| e.name == name).unwrap()
    }

    fn names_are_asked_for_first(f: &mut Files, dir: &Path, inside: &Where) {
        let i = at(f, "locked-names.7z");
        f.open(0, i);
        assert!(matches!(card(f).then, Then::List));
        assert!(f.cols[0].at == *inside && f.cols[0].entries.is_empty() && !f.bad, "{}", f.said);
        assert_eq!(pressable(f, "Open"), Some(false));
        f.update(Msg::Unlock(UnlockMsg::Close));
        assert!(f.dialog.is_none() && !f.bad, "{}", f.said);
        assert_eq!(f.cols[0].at, Where::Folder(dir.to_path_buf()));
        assert!(!f.cols[0].back.contains(inside));
        let i = at(f, "locked-names.7z");
        f.open(0, i);
        try_with(f, "nope");
        assert_eq!(card(f).wrong.as_deref(), Some(WRONG));
        assert!(f.cols[0].entries.is_empty() && !f.bad);
        try_with(f, KEY);
        assert!(f.dialog.is_none() && !f.bad, "{}", f.said);
        let names: Vec<&str> = f.cols[0].entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["docs", "hello.txt"]);
    }

    fn the_kept_password_serves_what_is_inside(f: &mut Files, dir: &Path, arc: &Path, inside: &Where) -> PathBuf {
        let taken = open_dir(arc).join("hello.txt");
        f.inside_open(0, vec![arc.join("hello.txt")]);
        assert!(f.dialog.is_none() && !f.bad, "{}", f.said);
        assert_eq!(std::fs::read(&taken).unwrap(), HELLO);
        assert_eq!(f.cols[0].at, *inside, "the archive stays open");

        let i = at(f, "hello.txt");
        f.update(Msg::Choose(0, i));
        f.update(Msg::Preview(PreviewMsg::Toggle));
        let Some(Dialog::Preview(pv)) = &f.dialog else { panic!("no viewer: {}", f.said) };
        assert!(matches!(&pv.seen, Seen::Text { blocks, .. } if blocks[0].starts_with("Hello from John Smith.")));
        let copy = pv.copy().unwrap().to_path_buf();
        assert_eq!(std::fs::read(&copy).unwrap(), HELLO);
        f.update(Msg::Preview(PreviewMsg::Toggle));
        assert!(f.dialog.is_none() && !copy.exists());

        f.inside_extract(0, vec![arc.join("docs")]);
        assert!(f.dialog.is_none() && !f.bad, "{}", f.said);
        assert_eq!(std::fs::read(dir.join("docs/note.txt")).unwrap().len(), 98);
        f.inside_open(0, vec![arc.join("docs")]);
        f.inside_open(0, vec![arc.join("docs/blank.txt")]);
        assert!(f.dialog.is_none() && open_dir(arc).join("blank.txt").exists(), "{}", f.said);
        taken
    }

    fn leaving_wipes_the_password_and_a_changed_archive_asks_again(f: &mut Files, dir: &Path, arc: &Path) {
        assert_eq!(f.unlocks.kept.len(), 1);
        f.go(0, Where::Folder(dir.to_path_buf()));
        f.update(Msg::Changed);
        assert!(f.unlocks.kept.is_empty(), "nothing is kept once no pane shows the archive");
        let i = at(f, "locked-names.7z");
        f.open(0, i);
        if f.dialog.is_some() {
            assert!(matches!(card(f).then, Then::List));
            f.update(Msg::Unlock(UnlockMsg::Close));
            let i = at(f, "locked-names.7z");
            f.open(0, i);
            try_with(f, KEY);
        } else {
            assert!(f.unlocks.kept.is_empty());
            f.inside_open(0, vec![arc.join("hello.txt")]);
            assert!(matches!(card(f).then, Then::Open(_)), "what is in it asks again");
            try_with(f, KEY);
        }
        assert!(f.dialog.is_none() && f.unlocks.kept.len() == 1, "{}", f.said);
        let later = std::time::SystemTime::now() + std::time::Duration::from_secs(90);
        std::fs::File::options().write(true).open(arc).unwrap().set_modified(later).unwrap();
        f.inside_open(0, vec![arc.join("hello.txt")]);
        assert!(matches!(card(f).then, Then::Open(_)), "a changed archive asks again");
        assert!(f.unlocks.kept.is_empty());
        try_with(f, KEY);
        assert!(f.dialog.is_none() && !f.bad, "{}", f.said);
    }

    #[test]
    fn a_lock_that_cannot_be_opened_is_said_in_red_and_never_asked_about() {
        let dir = place("cannot");
        let out = dir.join("out");
        std::fs::create_dir(&out).unwrap();
        let mut f = Files::new(vec![dir.clone()]);
        f.send_to(vec![kept(&dir, "locked.rar")], out.clone(), Kind::Other(Extra::Unpack(Place::Folder)));
        assert!(f.dialog.is_none() && f.bad);
        assert_eq!(f.said, "One item could not be extracted: locked.rar (it is a RAR locked with a password, which Folders cannot unlock; nothing is wrong with the file).");
        f.send_to(vec![kept(&dir, "locked-odd.7z")], out.clone(), Kind::Other(Extra::Unpack(Place::Folder)));
        assert!(f.dialog.is_none() && f.bad);
        assert_eq!(f.said, "One item could not be extracted: locked-odd.7z (it is locked with a password and packed a way Folders cannot unlock).");
        assert_eq!(count(&out), 0);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn several_locked_archives_are_asked_about_one_at_a_time() {
        let dir = place("several");
        let out = dir.join("out");
        std::fs::create_dir(&out).unwrap();
        let (zip, seven) = (kept(&dir, "locked-old.zip"), kept(&dir, "locked.7z"));
        let mut f = Files::new(vec![dir.clone()]);
        f.send_to(vec![zip.clone(), seven.clone()], out.clone(), Kind::Other(Extra::Unpack(Place::Folder)));
        assert_eq!(card(&f).archive, zip);
        f.update(Msg::Unlock(UnlockMsg::Close));
        assert_eq!(card(&f).archive, seven);
        try_with(&mut f, KEY);
        assert!(f.dialog.is_none() && !f.bad, "{}", f.said);
        assert!(out.join("locked/hello.txt").is_file() && !out.join("locked-old").exists());
        f.dialog = Some(Dialog::Columns(Vec::new()));
        f.send_to(vec![zip.clone()], out.clone(), Kind::Other(Extra::Unpack(Place::Folder)));
        assert!(matches!(f.dialog, Some(Dialog::Columns(_))) && f.bad);
        assert_eq!(f.said, "One item could not be extracted: locked-old.zip (it is locked with a password).");
        f.dialog = None;
        f.send_to(vec![zip.clone()], out.clone(), Kind::Other(Extra::Unpack(Place::Folder)));
        try_with(&mut f, KEY);
        assert!(f.dialog.is_none() && out.join("locked-old/hello.txt").is_file(), "{}", f.said);
        f.go(0, Where::Inside(zip.clone(), String::new()));
        assert!(f.dialog.is_none() && f.cols[0].entries.iter().any(|e| e.name == "open.txt"), "{}", f.said);
        f.inside_open(0, vec![zip.join("open.txt")]);
        assert!(f.dialog.is_none() && open_dir(&zip).join("open.txt").is_file(), "{}", f.said);
        f.inside_open(0, vec![zip.join("hello.txt")]);
        assert!(matches!(card(&f).then, Then::Open(_)));
        try_with(&mut f, "no");
        assert_eq!(card(&f).wrong.as_deref(), Some(WRONG));
        try_with(&mut f, KEY);
        assert_eq!(std::fs::read(open_dir(&zip).join("hello.txt")).unwrap(), HELLO);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_card_hides_what_is_typed_and_is_the_same_size_when_it_says_a_password_is_wrong() {
        let mut f = Files::new(Vec::new());
        f.unlock_ask(PathBuf::from("/tmp/photos.zip"), Then::Unpack(Place::Here, PathBuf::from("/tmp")), None);
        let said = |f: &Files| view(card(f), f.turn).said();
        let tall = |f: &Files| Ui::new(view(card(f), f.turn), Theme::new(true, "#fbc711")).natural_height(WIDTH);
        let plain = tall(&f);
        assert!(said(&f).contains(&"secret: Password".to_string()) && !said(&f).contains(&"entry: Password".to_string()));
        assert!(said(&f).contains(&"Kept only while this archive is open, never saved.".to_string()));
        assert!(view(card(&f), f.turn).buttons().iter().all(|(tip, _)| !tip.contains("password")));
        f.update(Msg::Unlock(UnlockMsg::Go));
        assert!(!card(&f).busy && card(&f).wrong.is_none());
        f.update(Msg::Unlock(UnlockMsg::Password("john".into())));
        let mut ui = Ui::new(view(card(&f), f.turn), Theme::new(true, "#fbc711"));
        ui.press_eye("unlockpassword");
        assert!(ui.shows_secret("unlockpassword"));
        assert_eq!(ui.natural_height(WIDTH), plain);
        ui.press_eye("unlockpassword");
        assert!(!ui.shows_secret("unlockpassword"));
        assert!(said(&f).contains(&"secret: Password".to_string()));
        for t in [Trouble::WrongKey, Trouble::WrongOrBad] {
            if let Some(Dialog::Unlock(u)) = &mut f.dialog {
                u.wrong = Some(sentence(t));
            }
            assert!(said(&f).contains(&sentence(t)));
            assert_eq!(tall(&f), plain, "{t:?}");
        }
        assert_eq!((sentence(Trouble::WrongKey), sentence(Trouble::WrongOrBad)), (WRONG.to_string(), WRONG_OR_BAD.to_string()));
        f.update(Msg::Unlock(UnlockMsg::Close));
        assert!(f.dialog.is_none() && f.unlocks.queue.is_empty());
    }
}
