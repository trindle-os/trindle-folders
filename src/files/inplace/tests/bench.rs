// The window as a person meets it, for tests that press keys and click.

use super::*;

pub(super) struct Bench {
    pub(super) f: Files,
    pub(super) ui: Ui<Msg>,
}

impl Bench {
    fn new(f: Files) -> Bench {
        let ui = Ui::new(crate::files::view::view(&f), Theme::new(false, "#fbc711"));
        let mut b = Bench { f, ui };
        b.show();
        b
    }

    pub(super) fn show(&mut self) {
        for _ in 0..2 {
            self.ui.set_view(crate::files::view::view(&self.f));
            let mut pix = tiny_skia::Pixmap::new(ROOM.0, ROOM.1).unwrap();
            self.ui.paint(&mut pix.as_mut(), 1.0);
        }
    }

    pub(super) fn send(&mut self, input: Input) {
        let sent = match self.f.intercept_with(&input, self.ui.keyboard()) {
            Some(m) => vec![m],
            None => self.ui.input(&input, Instant::now()).messages,
        };
        for m in sent {
            self.f.update(m);
        }
        self.show();
    }

    pub(super) fn key(&mut self, sym: Keysym) {
        self.send(Input::Key { sym, text: None, mods: Mods::default(), repeat: false });
    }

    pub(super) fn key_with(&mut self, sym: Keysym, mods: Mods) {
        self.send(Input::Key { sym, text: None, mods, repeat: false });
        self.send(Input::Mods(Mods::default()));
    }

    pub(super) fn type_in(&mut self, words: &str) {
        for c in words.chars() {
            self.send(Input::Key { sym: Keysym::NoSymbol, text: Some(c.to_string()), mods: Mods::default(), repeat: false });
        }
    }

    pub(super) fn press_at(&mut self, x: f32, y: f32) {
        self.send(Input::PointerMove { x, y });
        self.send(Input::Press { button: Button::Left, x, y });
        self.send(Input::Release { button: Button::Left, x, y });
    }

    pub(super) fn pick(&mut self, p: usize, name: &str) {
        let i = index(&self.f, p, name);
        self.f.update(Msg::Choose(p, i));
        self.f.update(Msg::Pick(p, vec![i]));
        self.show();
    }

    pub(super) fn places(&mut self) -> Vec<(String, Rect)> {
        self.ui.places_at(ROOM)
    }

    pub(super) fn name_box(&mut self) -> Option<Rect> {
        self.places().into_iter().find(|(id, _)| id.starts_with("rename")).map(|(_, r)| r)
    }

    pub(super) fn items_at(&mut self, p: usize) -> Rect {
        let area = format!("pane{p}:");
        let strips: Vec<Rect> = self.places().into_iter().filter(|(id, _)| id.starts_with(&area) && id.contains('/')).map(|(_, r)| r).collect();
        *strips.first().expect("the pane's items are laid out")
    }

    pub(super) fn cell(&mut self, p: usize, name: &str) -> Rect {
        let origin = self.items_at(p);
        let pane = &self.f.cols[p];
        let c = pane.grid.get().cell(index(&self.f, p, name), pane.width.get());
        Rect { x: origin.x + c.x, y: origin.y + c.y, ..c }
    }

    pub(super) fn typed(&mut self) -> (String, std::ops::Range<usize>) {
        let t = self.ui.typing().expect("a typing box has the keyboard");
        assert!(t.id.starts_with("rename"), "{}", t.id);
        (t.text, t.caret.min(t.anchor)..t.caret.max(t.anchor))
    }
}

pub(super) fn held_still(before: &[(String, Rect)], now: &[(String, Rect)], what: &str) {
    for (id, was) in before {
        let is = now.iter().find(|(n, _)| n == id).map(|(_, r)| *r);
        assert_eq!(is, Some(*was), "{what}: {id} moved");
    }
}

pub(super) fn bench(tag: &str, view: View) -> (PathBuf, Bench) {
    let dir = folder(tag);
    let f = files(&dir, view);
    (dir, Bench::new(f))
}

pub(super) fn select_all() -> Mods {
    Mods { ctrl: true, ..Mods::default() }
}

pub(super) fn every_pane_and_tab(tag: &str, cases: fn(&mut Bench, &Path, usize, &str)) {
    for view in VIEWS {
        let (dir, mut b) = bench(&format!("{tag}-{view:?}"), view);
        for p in [0, 1] {
            cases(&mut b, &dir, p, &format!("{view:?}, pane {p}"));
            b.f.update(Msg::NewTab(Some(p)));
            b.show();
            cases(&mut b, &dir, p, &format!("{view:?}, pane {p}, second tab"));
            b.f.update(Msg::Tab(p, 0));
            b.show();
        }
        fs::remove_dir_all(&dir).unwrap();
    }
}
