// The acts the keys and buttons start: copy, move, cut, paste, trash, delete, restore and make.

use std::path::{Path, PathBuf};

use crate::files::app::{Files, Msg};
use crate::files::clip;
use crate::files::far;
use crate::files::places::{self, Key, Where};
use crate::files::work::{self, Extra, Kind, Plan, Step};
use crate::common::selection;
use crate::toolkit::t;
use super::{CLIP_LIMIT, cut_elsewhere, far_clip, shut};
use super::steps::{make_new, trashed};
use super::types::{Clip, Turn};
use super::text::{what, words};

impl Files {
    pub(in crate::files) fn send_to(&mut self, items: Vec<PathBuf>, to: PathBuf, kind: Kind) {
        self.send_with(items, to, kind, None);
    }

    pub(in crate::files) fn send_with(&mut self, items: Vec<PathBuf>, to: PathBuf, kind: Kind, key: Option<crate::files::archive::Key>) {
        if items.is_empty() {
            return self.say(t!("folders-act-pick-first"), false);
        }
        let words = words(&kind, &items, &to, &self.home);
        self.start(Plan { kind, items, to, key }, words, Turn::New);
    }

    pub(in crate::files) fn trash_these(&mut self, items: Vec<PathBuf>) {
        if items.is_empty() {
            return self.say(t!("folders-act-pick-first"), false);
        }
        if items.iter().any(|p| trashed(p)) {
            return self.say(t!("folders-act-in-trash"), false);
        }
        if items.iter().any(|p| far::is_far(p)) {
            return self.say(t!("folders-act-far-trash"), false);
        }
        let words = words(&Kind::Trash, &items, Path::new(""), &self.home);
        self.start(Plan { kind: Kind::Trash, items, to: PathBuf::new(), key: None }, words, Turn::New);
    }

    pub(super) fn across(&mut self, moving: bool) {
        let from = self.side();
        let items = self.picked_paths(from);
        if items.is_empty() {
            return self.say(t!("folders-act-pick-copy-move"), false);
        }
        if let Where::Inside(a, _) = self.cols[from].at.clone() {
            return match self.cols[1 - from].at.folder() {
                Some(f) => self.inside_take(&a, items, f.to_path_buf()),
                None => self.say(t!("folders-act-archive-out"), false),
            };
        }
        match self.cols[1 - from].at.clone() {
            Where::Folder(f) => self.send_to(items, f, if moving { Kind::Move } else { Kind::Copy }),
            Where::Trash if moving => self.trash_these(items),
            Where::Trash => self.say(t!("folders-act-copy-trash"), false),
            Where::Remote(a) => self.send_to(items, PathBuf::from(a), if moving { Kind::Move } else { Kind::Copy }),
            _ => self.say(t!("folders-act-other-pane"), false),
        }
    }

    pub(super) fn copy_clip(&mut self, cut: bool) {
        let paths = self.picked_paths(self.side());
        if paths.is_empty() {
            return self.say(t!("folders-act-pick-copy"), false);
        }
        if paths.iter().any(|p| far::is_far(p)) {
            return self.say(far_clip(), false);
        }
        if matches!(self.pane().at, Where::Inside(..)) {
            return self.say(shut(), false);
        }
        let said = what(&paths);
        self.say(if cut { t!("folders-act-cut-said", what = &said) } else { t!("folders-act-copied-said", what = &said) }, false);
        let note = self.cut_note();
        match (&note, cut) {
            (Some(at), true) => clip::note_cut(at, &paths),
            (Some(at), false) => clip::forget_cut(at, None),
            (None, _) => {}
        }
        if self.post.is_some() {
            let kinds = clip::kinds_for(&paths, cut);
            let cut_paths = cut.then(|| paths.clone());
            let _ = std::thread::Builder::new().name("folders-clip".into()).spawn(move || {
                selection::offer_kinds(kinds, || {});
                if let (Some(at), Some(paths)) = (note, cut_paths) {
                    clip::forget_cut(&at, Some(&paths));
                }
            });
        }
        self.clip = Some(Clip { paths, cut });
    }

    fn cut_note(&self) -> Option<PathBuf> {
        self.cut_note.clone().or_else(|| self.post.is_some().then(clip::cut_note))
    }

    pub(super) fn paste(&mut self) {
        if let Where::Remote(_) = self.pane().at {
            return self.say(far_clip(), false);
        }
        if self.pane().at.folder().is_none() {
            return self.say(t!("folders-act-paste-folder"), false);
        }
        let Some(post) = self.post.clone() else { return self.pasted(None) };
        let _ = std::thread::Builder::new().name("folders-paste".into()).spawn(move || {
            let read = selection::paste_kinds(CLIP_LIMIT, &clip::READ);
            post.send(Msg::Pasted((!read.is_empty()).then(|| clip::parse(&read))));
        });
    }

    pub(super) fn pasted(&mut self, got: Option<clip::Pasted>) {
        let Some(to) = self.pane().at.folder().map(Path::to_path_buf) else { return };
        let note = self.cut_note();
        let mut elsewhere = false;
        let (items, cut) = match (got, &self.clip) {
            (Some(g), own) if !g.paths.is_empty() => {
                let cut = clip::moves(&g.paths, g.cut, own.as_ref().map(|c| (&c.paths[..], c.cut)), note.as_deref());
                elsewhere = g.cut && !cut;
                (g.paths, cut)
            }
            (Some(_), _) => return self.say(t!("folders-act-no-clip"), false),
            (None, Some(c)) => (c.paths.clone(), c.cut),
            (None, None) => return self.say(t!("folders-act-nothing-cut"), false),
        };
        if cut {
            self.clip = None;
            if let Some(at) = &note {
                clip::forget_cut(at, Some(&items));
            }
        }
        if elsewhere && !items.is_empty() {
            let mut words = words(&Kind::Copy, &items, &to, &self.home);
            words.done = cut_elsewhere().into();
            return self.start(Plan { kind: Kind::Copy, items, to, key: None }, words, Turn::New);
        }
        self.send_to(items, to, if cut { Kind::Move } else { Kind::Copy });
    }

    pub(super) fn trash_key(&mut self) {
        let items = self.picked_paths(self.side());
        match self.pane().at {
            Where::Trash | Where::Remote(_) => self.ask_delete(items),
            Where::Folder(_) | Where::Recent | Where::Found(..) => self.trash_these(items),
            Where::Network | Where::Computer => {}
            Where::Inside(..) => self.say(shut(), false),
        }
    }

    pub(super) fn ask_delete(&mut self, items: Vec<PathBuf>) {
        if items.is_empty() {
            return self.say(t!("folders-act-pick-first"), false);
        }
        if self.pane().at == Where::Network {
            return self.say(t!("folders-act-network-delete"), false);
        }
        if matches!(self.pane().at, Where::Inside(..)) {
            return self.say(shut(), false);
        }
        self.emptying = false;
        self.ask_about(items);
    }

    pub(super) fn asked_for(&mut self, plan: &Plan) -> Vec<crate::files::proof::Asked> {
        if matches!(plan.kind, Kind::Delete | Kind::Other(Extra::Wipe(_))) { std::mem::take(&mut self.asked) } else { Vec::new() }
    }

    pub(super) fn ask_about(&mut self, items: Vec<PathBuf>) {
        self.asked = crate::files::proof::asked(&items);
        self.confirm = Some(items);
    }

    pub(super) fn empty_trash(&mut self) {
        let bins = match &self.bin {
            Some(b) => vec![b.files.clone()],
            None => crate::files::trash::all_files(),
        };
        let all: Vec<PathBuf> = bins.into_iter()
            .flat_map(|files| places::list(&Where::Folder(files), true, Key::Name, true, &self.mime))
            .map(|e| e.path).collect();
        if all.is_empty() {
            return self.say(t!("folders-act-trash-empty"), false);
        }
        self.ask_about(all);
        self.emptying = true;
    }

    pub(super) fn restore(&mut self) {
        if self.pane().at != Where::Trash {
            return;
        }
        let items = self.picked_paths(self.side());
        if items.is_empty() {
            return self.say(t!("folders-act-pick-trash"), false);
        }
        let words = words(&Kind::Restore, &items, Path::new(""), &self.home);
        self.start(Plan { kind: Kind::Restore, items, to: PathBuf::new(), key: None }, words, Turn::New);
    }

    fn far_make(&mut self, p: usize, at: &str, folder: bool) {
        let Ok(a) = crate::files::remote::Address::parse(at) else { return };
        let names: Vec<String> = self.cols[p].entries.iter().map(|e| e.name.clone()).collect();
        let base = if folder { t!("folders-act-new-folder") } else { t!("folders-act-new-document") };
        let name = std::iter::once(base.to_string()).chain((2..).map(|n| t!("folders-act-new-numbered", base = base, n = n)))
            .find(|n| !names.contains(n)).unwrap_or_default();
        let made = a.join(&name);
        let (path, shown) = (made.path.clone(), PathBuf::from(made.to_string()));
        self.say(t!("folders-act-making", name = &name), false);
        let done = move |got| far::FarMsg::Made { side: p, path: shown, got };
        if folder {
            self.far_do(a, move |link, _| link.mkdir(&path), done);
        } else {
            self.far_do(a, move |link, _| link.write_new(&path, &mut std::io::empty(), 0, &mut |_| true).map(drop), done);
        }
    }

    pub(super) fn make(&mut self, folder: bool) {
        let p = self.side();
        if !crate::files::inplace::can_rename(&self.pane().at) {
            return self.say(t!("folders-act-make-folder-only"), false);
        }
        if let Where::Remote(a) = self.pane().at.clone() {
            return self.far_make(p, &a, folder);
        }
        let Some(dir) = self.pane().at.folder().map(Path::to_path_buf) else { return };
        let path = match make_new(&dir, folder) {
            Ok(path) => path,
            Err(why) => return self.say(why.clone(), why != crate::files::reach::not_answering()),
        };
        let name = work::name_of(&path);
        self.remember(t!("folders-act-undo-making", name = &name), vec![Step::Remove(path.clone())]);
        self.say("", false);
        self.load(self.cols[p].tab, p);
        self.when_read(p, None, Some((path, true)));
    }
}
