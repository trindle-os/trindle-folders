//! The folders inside each folder named in the path bar, for the small arrow after each name.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::app::{Files, Msg};
use super::far::is_far;
use super::places::{self, Entry, Where};
use crate::toolkit::t;
use crate::toolkit::widget::Sub;

pub fn reading() -> &'static str {
    t!("folders-below-reading")
}
pub fn none() -> &'static str {
    t!("folders-below-none")
}

pub enum Listing {
    Reading,
    Got(Rc<Vec<String>>, Vec<Where>),
    Failed(String),
}

#[derive(Default)]
pub struct Below {
    read: HashMap<Where, Listing>,
}

#[derive(Clone, Debug)]
pub enum BelowMsg {
    Open(usize, Where),
    Got(PathBuf, Result<Vec<PathBuf>, String>),
    Pick(usize, Where, usize),
}

pub fn folders_in(dir: &Path, hidden: bool) -> Result<Vec<PathBuf>, String> {
    let read = std::fs::read_dir(dir).map_err(|e| places::unread_words(&e))?;
    let mut out: Vec<PathBuf> = read.flatten()
        .filter(|e| hidden || !e.file_name().as_encoded_bytes().starts_with(b"."))
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    out.sort_by_cached_key(|p| crate::toolkit::ahead::fold(&super::kinds::shown_name(p)));
    Ok(out)
}

fn listing(got: Result<Vec<PathBuf>, String>) -> Listing {
    match got {
        Ok(folders) => Listing::Got(Rc::new(folders.iter().map(|f| super::kinds::shown_name(f)).collect()), folders.into_iter().map(Where::Folder).collect()),
        Err(why) => Listing::Failed(why),
    }
}

fn offered(at: &Where, last: bool) -> bool {
    match at {
        Where::Folder(dir) => !is_far(dir),
        Where::Remote(_) | Where::Inside(..) => last,
        _ => false,
    }
}

pub fn subs(f: &Files, crumbs: &[(String, Where)]) -> Vec<Sub> {
    crumbs.iter().enumerate().map(|(i, (_, at))| {
        if !offered(at, i + 1 == crumbs.len()) {
            return Sub::None;
        }
        match f.below.read.get(at) {
            None | Some(Listing::Reading) => Sub::Words(reading().into()),
            Some(Listing::Failed(why)) => Sub::Words(why.clone()),
            Some(Listing::Got(names, _)) if names.is_empty() => Sub::Words(none().into()),
            Some(Listing::Got(names, wheres)) => {
                let next = crumbs.get(i + 1).map(|c| &c.1);
                Sub::Rows(names.clone(), next.and_then(|n| wheres.iter().position(|w| w == n)))
            }
        }
    }).collect()
}

impl Files {
    pub(super) fn below_msg(&mut self, m: BelowMsg) {
        match m {
            BelowMsg::Open(p, at) => self.below_read(p, at),
            BelowMsg::Got(dir, got) => {
                self.below.read.insert(Where::Folder(dir), listing(got));
            }
            BelowMsg::Pick(p, at, row) => {
                let to = match self.below.read.get(&at) {
                    Some(Listing::Got(_, wheres)) => wheres.get(row).cloned(),
                    _ => None,
                };
                if let Some(to) = to {
                    self.go(p, to);
                }
            }
        }
    }

    fn below_read(&mut self, p: usize, at: Where) {
        match &at {
            Where::Folder(dir) if !is_far(dir) => self.below_folders(vec![dir.clone()]),
            _ if self.cols[p].at == at => {
                let pane = &self.cols[p];
                let folders: Vec<(String, Where)> = pane.all.iter().filter(|e| e.folder)
                    .filter_map(|e| Some((e.name.clone(), place_of(&pane.at, e)?))).collect();
                let (names, wheres) = folders.into_iter().unzip();
                self.below.read.insert(at, Listing::Got(Rc::new(names), wheres));
            }
            _ => {}
        }
    }

    fn below_folders(&mut self, dirs: Vec<PathBuf>) {
        let hidden = self.hidden;
        let Some(post) = self.post.clone() else {
            for dir in dirs {
                let got = listing(folders_in(&dir, hidden));
                self.below.read.insert(Where::Folder(dir), got);
            }
            return;
        };
        for dir in &dirs {
            self.below.read.entry(Where::Folder(dir.clone())).or_insert(Listing::Reading);
        }
        let spawned = std::thread::Builder::new().name("folders-below".into()).spawn(move || {
            for dir in dirs {
                let got = folders_in(&dir, hidden);
                post.send(Msg::Below(BelowMsg::Got(dir, got)));
            }
        });
        if let Err(e) = spawned {
            eprintln!("trindle: files: files-below: {e}");
        }
    }

    pub(super) fn below_fetch(&mut self, p: usize) {
        let home = self.home.clone();
        let shown: HashSet<Where> = self.cols.iter().flat_map(|c| c.tabs.iter())
            .flat_map(|t| places::crumbs(&t.at, &home)).map(|(_, at)| at).collect();
        self.below.read.retain(|at, _| shown.contains(at));
        if self.post.is_none() {
            return;
        }
        let dirs: Vec<PathBuf> = places::crumbs(&self.cols[p].at, &home).into_iter().filter_map(|(_, at)| match at {
            Where::Folder(dir) if !is_far(&dir) && !self.below.read.contains_key(&Where::Folder(dir.clone())) => Some(dir),
            _ => None,
        }).collect();
        if !dirs.is_empty() {
            self.below_folders(dirs);
        }
    }

    pub(super) fn below_forget(&mut self) {
        self.below.read.clear();
    }
}

pub fn place_of(at: &Where, e: &Entry) -> Option<Where> {
    match at {
        Where::Inside(a, _) => e.folder.then(|| super::inside::inner(a, &e.path)).flatten().map(|name| Where::Inside(a.clone(), name)),
        _ if !e.folder => super::inside::opens(&e.path).then(|| Where::Inside(e.path.clone(), String::new())),
        _ if is_far(&e.path) => Some(Where::Remote(e.path.to_string_lossy().into_owned())),
        _ => Some(Where::Folder(e.path.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::app::View;
    use crate::toolkit::input::{Button, Input, Keysym, Mods};
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App, Ui};
    use std::os::unix::fs::PermissionsExt;
    use std::time::Instant;

    fn made_up(tag: &str) -> (PathBuf, Files) {
        let dir = std::env::temp_dir().join(format!("trindle-files-below-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for sub in ["Papers/Albums/2025", "Papers/Albums/2026", "Papers/letters", "Papers/Recipes", "Papers/.hidden", "Papers/\u{c9}clairs", "Music"] {
            std::fs::create_dir_all(dir.join(sub)).unwrap();
        }
        std::fs::write(dir.join("Papers/notes.txt"), b"made up").unwrap();
        std::fs::write(dir.join("Papers/Albums/list.txt"), b"made up").unwrap();
        let kept = crate::files::kept::Kept::default();
        let start = (Where::Folder(dir.join("Papers/Albums")), Where::Folder(dir.clone()));
        let f = Files::with(Vec::new(), dir.clone(), start, kept, Vec::new());
        (dir, f)
    }

    fn rows_of(sub: &Sub) -> (Vec<String>, Option<usize>) {
        match sub {
            Sub::Rows(rows, marked) => (rows.to_vec(), *marked),
            other => panic!("not rows: {other:?}"),
        }
    }

    #[test]
    fn each_name_of_the_path_lists_the_folders_inside_it_with_the_way_marked() {
        let (dir, mut f) = made_up("lists");
        let crumbs = places::crumbs(&f.cols[0].at, &f.home);
        assert_eq!(crumbs.iter().map(|c| c.0.as_str()).collect::<Vec<_>>(), ["Home", "Papers", "Albums"]);
        assert_eq!(subs(&f, &crumbs), vec![Sub::Words(reading().into()); 3]);
        for (_, at) in &crumbs {
            f.update(Msg::Below(BelowMsg::Open(0, at.clone())));
        }
        let got = subs(&f, &crumbs);
        assert_eq!(rows_of(&got[0]), (vec!["Music".to_string(), "Papers".to_string()], Some(1)));
        assert_eq!(rows_of(&got[1]), (["Albums", "\u{c9}clairs", "letters", "Recipes"].map(String::from).to_vec(), Some(0)));
        assert_eq!(rows_of(&got[2]), (vec!["2025".to_string(), "2026".to_string()], None));
        f.update(Msg::Hidden);
        assert_eq!(subs(&f, &crumbs)[1], Sub::Words(reading().into()), "read afresh");
        f.update(Msg::Below(BelowMsg::Open(0, crumbs[1].1.clone())));
        assert_eq!(rows_of(&subs(&f, &crumbs)[1]).0[0], ".hidden");
        f.update(Msg::Hidden);
        f.update(Msg::Below(BelowMsg::Open(0, crumbs[1].1.clone())));
        let row = rows_of(&subs(&f, &crumbs)[1]).0.iter().position(|n| n == "Recipes").unwrap();
        f.update(Msg::Below(BelowMsg::Pick(0, crumbs[1].1.clone(), row)));
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers/Recipes")));
        assert_eq!(f.cols[1].at, Where::Folder(dir.clone()), "the other pane stays");
        f.update(Msg::Back);
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers/Albums")));
        f.go(0, Where::Folder(dir.join("Papers/Recipes")));
        let crumbs = places::crumbs(&f.cols[0].at, &f.home);
        f.update(Msg::Below(BelowMsg::Open(0, crumbs[2].1.clone())));
        assert_eq!(subs(&f, &crumbs)[2], Sub::Words(none().into()));
        f.update(Msg::Below(BelowMsg::Pick(0, crumbs[2].1.clone(), 0)));
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers/Recipes")));
        std::fs::set_permissions(dir.join("Papers/Recipes"), std::fs::Permissions::from_mode(0o000)).unwrap();
        let shut = std::fs::read_dir(dir.join("Papers/Recipes")).is_err();
        f.update(Msg::Below(BelowMsg::Open(0, crumbs[2].1.clone())));
        let refused = subs(&f, &crumbs)[2] == Sub::Words("You don't have permission to open this folder.".into());
        std::fs::set_permissions(dir.join("Papers/Recipes"), std::fs::Permissions::from_mode(0o755)).unwrap();
        assert!(shut, "{}", crate::common::files::tests::NOT_AS_ROOT);
        assert!(refused, "a folder that may not be opened is said to be one");
        std::fs::remove_dir(dir.join("Papers/Recipes")).unwrap();
        f.update(Msg::Below(BelowMsg::Open(0, crumbs[2].1.clone())));
        assert_eq!(subs(&f, &crumbs)[2], Sub::Words("This folder isn't there any more.".into()));
        f.go(0, Where::Folder(dir.join("Music")));
        assert!(!f.below.read.contains_key(&Where::Folder(dir.join("Papers"))));
        for at in [Where::Recent, Where::Trash, Where::Network] {
            let crumbs = places::crumbs(&at, &f.home);
            assert_eq!(subs(&f, &crumbs), vec![Sub::None], "{at:?}");
        }
        let found = places::crumbs(&Where::Found(dir.join("Papers"), "a".into(), false), &f.home);
        let got = subs(&f, &found);
        assert_eq!(got.last(), Some(&Sub::None));
        assert!(got[..got.len() - 1].iter().all(|s| *s != Sub::None));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_servers_folder_lists_what_the_pane_already_read() {
        let (dir, mut f) = made_up("server");
        let at = Where::Remote("sftp://files.example.com/home/john".into());
        let entry = |name: &str, folder: bool| Entry {
            name: name.into(), path: PathBuf::from(format!("sftp://files.example.com/home/john/{name}")), folder, size: 0, when: 0, kind: None,
        };
        f.cols[0].at = at.clone();
        f.cols[0].all = vec![entry("Documents", true), entry("Photos", true), entry("notes.txt", false)];
        let crumbs = places::crumbs(&at, &f.home);
        let before = subs(&f, &crumbs);
        assert!(before[..before.len() - 1].iter().all(|s| *s == Sub::None), "the folders above have no arrow: {before:?}");
        assert_eq!(before.last(), Some(&Sub::Words(reading().into())));
        f.below_msg(BelowMsg::Open(0, at.clone()));
        assert_eq!(rows_of(subs(&f, &crumbs).last().unwrap()), (vec!["Documents".to_string(), "Photos".to_string()], None));
        match f.below.read.get(&at) {
            Some(Listing::Got(_, wheres)) => assert_eq!(wheres[1], Where::Remote("sftp://files.example.com/home/john/Photos".into())),
            _ => panic!("not read"),
        }
        f.below_msg(BelowMsg::Open(0, crumbs[0].1.clone()));
        assert!(!f.below.read.contains_key(&crumbs[0].1));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_arrow_after_a_name_opens_its_list_in_the_window_by_pointer_and_by_keys() {
        let (dir, mut f) = made_up("window");
        f.update(Msg::PanesAre(false));
        f.side = 0;
        f.view = View::List;
        let mut ui = Ui::new(f.view(), Theme::new(false, "#fbc711"));
        show(&f, &mut ui);
        press_the_arrow_after_a_name(&mut f, &mut ui, &dir);
        press_the_name_itself(&mut f, &mut ui, &dir);
        keys_along_the_bar(&mut f, &mut ui, &dir);
        escape_puts_a_list_away(&mut f, &mut ui, &dir);
        typing_a_place_takes_the_names_place(&mut f, &mut ui);
        let _ = std::fs::remove_dir_all(dir);
    }

    fn show(f: &Files, ui: &mut Ui<Msg>) {
        for _ in 0..2 {
            ui.set_view(f.view());
            let mut pix = tiny_skia::Pixmap::new(1200, 760).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
        }
    }

    fn send(f: &mut Files, ui: &mut Ui<Msg>, input: Input) {
        let sent = match f.intercept_with(&input, ui.keyboard()) {
            Some(m) => vec![m],
            None => ui.input(&input, Instant::now()).messages,
        };
        for m in sent {
            f.update(m);
        }
        show(f, ui);
    }

    fn key(sym: Keysym) -> Input {
        Input::Key { sym, text: None, mods: Mods::default(), repeat: false }
    }

    fn press_the_arrow_after_a_name(f: &mut Files, ui: &mut Ui<Msg>, dir: &Path) {
        let pills = ui.pill_boxes("path0");
        assert_eq!(pills.len(), 3);
        let before = ui.boxes();
        let (x, y) = (pills[1].x + pills[1].w - 6.0, pills[1].y + pills[1].h / 2.0);
        send(f, ui, Input::PointerMove { x, y });
        send(f, ui, Input::Press { button: Button::Left, x, y });
        send(f, ui, Input::Release { button: Button::Left, x, y });
        assert_eq!(ui.open_rows(), ["Albums", "\u{c9}clairs", "letters", "Recipes"], "read as it opened");
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers/Albums")), "the arrow goes nowhere by itself");
        let (list, own) = ui.open_list().unwrap();
        assert!((own.x - pills[1].x).abs() < 1.0 && list.y >= own.y + own.h, "against its own name, below it");
        assert_eq!(ui.boxes(), before, "nothing moved for it");
        send(f, ui, key(Keysym::Down));
        send(f, ui, key(Keysym::Return));
        assert!(ui.open_list().is_none());
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers/\u{c9}clairs")));
    }

    fn press_the_name_itself(f: &mut Files, ui: &mut Ui<Msg>, dir: &Path) {
        let pills = ui.pill_boxes("path0");
        let (x, y) = (pills[1].x + 8.0, pills[1].y + pills[1].h / 2.0);
        send(f, ui, Input::PointerMove { x, y });
        send(f, ui, Input::Press { button: Button::Left, x, y });
        send(f, ui, Input::Release { button: Button::Left, x, y });
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers")));
    }

    fn keys_along_the_bar(f: &mut Files, ui: &mut Ui<Msg>, dir: &Path) {
        ui.keyboard_to("path0");
        send(f, ui, key(Keysym::Left));
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers")), "moving along goes nowhere");
        send(f, ui, key(Keysym::Down));
        assert_eq!(ui.open_rows(), ["Music", "Papers"]);
        send(f, ui, Input::Key { sym: Keysym::m, text: Some("m".into()), mods: Mods::default(), repeat: false });
        send(f, ui, key(Keysym::Return));
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Music")));
    }

    fn escape_puts_a_list_away(f: &mut Files, ui: &mut Ui<Msg>, dir: &Path) {
        ui.keyboard_to("path0");
        send(f, ui, key(Keysym::Right));
        send(f, ui, key(Keysym::Return));
        assert_eq!(ui.open_rows(), [none()], "an empty folder says so");
        send(f, ui, key(Keysym::Escape));
        assert!(ui.open_list().is_none());
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Music")));
    }

    fn typing_a_place_takes_the_names_place(f: &mut Files, ui: &mut Ui<Msg>) {
        f.update(Msg::Locate);
        show(f, ui);
        assert!(ui.pill_boxes("path0").is_empty() && f.locate.is_some());
        f.update(Msg::LocateStop);
        show(f, ui);
        assert!(!ui.pill_boxes("path0").is_empty());
    }
}
