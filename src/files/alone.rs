// Folders is an app of its own: it needs nothing but the toolkit and the shared code beside it to run.

use std::path::Path;

use crate::toolkit::t;

use super::menu::{Act, Line};

pub fn no_helper_share() -> &'static str {
    t!("folders-alone-no-share")
}
#[cfg_attr(test, allow(dead_code))]
pub fn no_helper_mount() -> &'static str {
    t!("folders-alone-no-mount")
}
#[cfg_attr(test, allow(dead_code))]
pub fn no_helper_nfs() -> &'static str {
    t!("folders-alone-no-nfs")
}
pub fn no_wallpaper() -> &'static str {
    t!("folders-alone-no-wallpaper")
}
pub fn no_root() -> &'static str {
    t!("folders-alone-no-root")
}
pub fn no_ai_share() -> &'static str {
    t!("folders-alone-no-ai")
}
pub fn no_terminal() -> &'static str {
    t!("folders-alone-no-terminal")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parts {
    pub wallpaper: bool,
    pub as_root: bool,
    pub helper: bool,
    pub ai: bool,
    pub terminal: Option<bool>,
}

impl Parts {
    #[cfg_attr(test, allow(dead_code))]
    pub fn look() -> Parts {
        Parts { wallpaper: part_here("wallpaper"), as_root: crate::common::run::find("pkexec").is_some(), helper: helper_here(), ai: part_here("boxed"), terminal: None }
    }

    pub fn all() -> Parts {
        Parts { wallpaper: true, as_root: true, helper: true, ai: true, terminal: None }
    }

    #[cfg(test)]
    pub fn none() -> Parts {
        Parts { wallpaper: false, as_root: false, helper: false, ai: false, terminal: Some(false) }
    }

    pub fn has_terminal(&self) -> bool {
        self.terminal.unwrap_or_else(super::menu::have_terminal)
    }

    pub fn has(&self, act: Act) -> bool {
        match act {
            Act::Wallpaper => self.wallpaper,
            Act::AsRoot => self.as_root,
            Act::ShareOnNetwork => self.helper,
            Act::ShareWithAi => self.ai,
            Act::Terminal => self.has_terminal(),
            _ => true,
        }
    }

    pub fn fit(&self, lines: Vec<Line>) -> Vec<Line> {
        super::menu::without(lines, |act| !self.has(act))
    }
}

pub fn part_here(part: &str) -> bool {
    let whole = std::env::current_exe().ok().is_some_and(|e| e.file_name().is_some_and(|n| n == "trindle"));
    whole || crate::common::run::find(&format!("trindle-{part}")).is_some()
}

pub fn helper_here() -> bool {
    Path::new(&crate::common::run::admin_helper()).is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::app::{Files, Msg, View};
    use crate::files::found::FindMsg;
    use crate::files::kept::Kept;
    use crate::files::menu::MenuMsg;
    use crate::files::places::Where;
    use crate::files::trash::Bin;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App, Ui};
    use std::path::PathBuf;

    const OUTSIDE: [Act; 6] = [Act::Wallpaper, Act::AsRoot, Act::ShareOnNetwork, Act::ShareWithAi, Act::Terminal, Act::RingPin];

    fn made_up(tag: &str) -> (PathBuf, Files) {
        let dir = std::env::temp_dir().join(format!("trindle-files-alone-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Papers/Albums")).unwrap();
        std::fs::create_dir_all(dir.join("Other")).unwrap();
        std::fs::write(dir.join("Papers/notes.txt"), b"made up").unwrap();
        std::fs::write(dir.join("Papers/report.txt"), b"made up too").unwrap();
        image::RgbImage::from_pixel(32, 24, image::Rgb([222, 184, 120])).save(dir.join("Papers/beach.png")).unwrap();
        let mut kept = Kept::default();
        kept.dual = true;
        let start = (Where::Folder(dir.join("Papers")), Where::Folder(dir.join("Other")));
        let mut f = Files::with(Vec::new(), dir.clone(), start, kept, vec![dir.join("Papers/Albums")]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.bin.as_ref().unwrap().ready().unwrap();
        (dir, f)
    }

    fn names(f: &Files, p: usize) -> Vec<String> {
        f.cols[p].entries.iter().map(|e| e.name.clone()).collect()
    }

    fn pick(f: &mut Files, p: usize, name: &str) {
        let i = f.cols[p].entries.iter().position(|e| e.name == name).unwrap();
        f.update(Msg::Choose(p, i));
        f.update(Msg::Pick(p, vec![i]));
    }

    fn offered(f: &mut Files, picked: &[&str]) -> Vec<Act> {
        let rows: Vec<usize> = picked.iter().map(|n| f.cols[0].entries.iter().position(|e| e.name == *n).unwrap()).collect();
        f.cols[0].chosen = rows.first().copied();
        f.cols[0].picked = rows;
        f.side = 0;
        f.menu = None;
        f.open_menu(0);
        f.menu.take().map(|m| crate::files::menu::every(&m.lines).iter().map(|l| l.act).collect()).unwrap_or_default()
    }

    fn paint(f: &Files) {
        let mut ui = Ui::new(f.view(), Theme::new(false, "#fbc711"));
        for _ in 0..2 {
            ui.set_view(f.view());
            let mut pix = tiny_skia::Pixmap::new(1200, 760).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
        }
    }

    #[test]
    fn folders_runs_alone_with_no_other_part_of_trindle() {
        let (dir, mut f) = made_up("none");
        f.parts = Parts::none();
        f.ring = None;
        assert!(f.post.is_none(), "nothing behind it: no loop, no display, no bus");
        for view in [View::List, View::Icons, View::Compact] {
            f.view = view;
            paint(&f);
        }
        f.view = View::List;
        works_on_folders_and_files(&mut f, &dir);
        offers_no_row_that_reaches_outside(&mut f);
        says_plainly_that_a_missing_part_is_missing(&mut f, &dir);
        let _ = std::fs::remove_dir_all(dir);
    }

    fn works_on_folders_and_files(f: &mut Files, dir: &Path) {
        assert_eq!(names(f, 0), ["Albums", "beach.png", "notes.txt", "report.txt"]);
        assert!(names(f, 1).is_empty());
        pick(f, 0, "Albums");
        f.update(Msg::Enter);
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers/Albums")));
        f.update(Msg::Back);
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Papers")));
        pick(f, 0, "notes.txt");
        f.update(Msg::Rename);
        f.update(Msg::RenameText("jottings.txt".into()));
        f.update(Msg::RenameDone);
        assert!(dir.join("Papers/jottings.txt").is_file() && !dir.join("Papers/notes.txt").exists());
        f.side = 0;
        pick(f, 0, "report.txt");
        f.update(Msg::Copy);
        assert_eq!(std::fs::read(dir.join("Other/report.txt")).unwrap(), b"made up too");
        f.update(Msg::Undo);
        assert!(!dir.join("Other/report.txt").exists());
        f.update(Msg::Find(FindMsg::Open(Some(0))));
        f.update(Msg::Find(FindMsg::Text("beach".into())));
        f.update(Msg::Find(FindMsg::Search(false)));
        assert_eq!(names(f, 0), ["beach.png"]);
        paint(f);
        f.go(0, Where::Folder(dir.join("Papers")));
        f.update(Msg::Below(crate::files::below::BelowMsg::Open(0, Where::Folder(dir.to_path_buf()))));
    }

    fn offers_no_row_that_reaches_outside(f: &mut Files) {
        for picked in [&["Albums"][..], &["beach.png"], &["jottings.txt"], &["beach.png", "jottings.txt"], &[]] {
            let acts = offered(f, picked);
            assert!(!acts.iter().any(|a| OUTSIDE.contains(a)), "{picked:?}: {acts:?}");
            assert!(acts.contains(&Act::NewFolder) && acts.contains(&Act::Properties), "{picked:?}: {acts:?}");
            if !picked.is_empty() {
                assert!(acts.contains(&Act::Open) && acts.contains(&Act::Copy) && acts.contains(&Act::Rename) && acts.contains(&Act::Trash), "{picked:?}");
            }
        }
        f.update(Msg::Menu(MenuMsg::Side { pinned: true, row: 0, x: 100.0, y: 140.0 }));
        let side: Vec<Act> = f.menu.take().unwrap().lines.iter().map(|l| l.act).collect();
        assert!(!side.iter().any(|a| OUTSIDE.contains(a)) && side.contains(&Act::Open) && side.contains(&Act::Pin), "{side:?}");
        f.update(Msg::Menu(MenuMsg::Open));
        paint(f);
        f.menu = None;
    }

    fn says_plainly_that_a_missing_part_is_missing(f: &mut Files, dir: &Path) {
        f.update(Msg::Terminal);
        assert_eq!(f.said, no_terminal());
        for (act, on, why) in [(Act::Wallpaper, "Papers/beach.png", no_wallpaper()), (Act::AsRoot, "Papers/Albums", no_root())] {
            f.do_act(act, 0, vec![dir.join(on)]);
            assert_eq!((f.said.as_str(), f.bad), (why, true), "{act:?}");
        }
        f.do_act(Act::RingPin, 0, vec![dir.join("Papers/Albums")]);
        assert_eq!(f.said, no_root(), "with no ring, nothing is said or written for it");
        f.parts = Parts::all();
        image::RgbImage::new(3841, 16).save(dir.join("Papers/huge.png")).unwrap();
        f.do_act(Act::Wallpaper, 0, vec![dir.join("Papers/huge.png")]);
        assert_eq!((f.said.as_str(), f.bad), ("huge.png cannot be the wallpaper: larger than 4K.", true));
        f.parts = Parts::none();
        let share = f.share_form(&dir.join("Papers/Albums")).expect("the Share part is there");
        assert_eq!(share.cannot.as_deref(), Some(no_helper_share()));
        f.ask(Act::Properties, 0, vec![dir.join("Papers/Albums")]);
        assert!(f.dialog.is_some());
        paint(f);
        f.dialog = None;
    }

    #[test]
    fn each_row_that_reaches_outside_goes_with_its_own_part() {
        let (dir, mut f) = made_up("each");
        f.parts = Parts { terminal: Some(true), ..Parts::all() };
        f.ring = Some(dir.join("ring-favourites"));
        let folder = offered(&mut f, &["Albums"]);
        for act in [Act::AsRoot, Act::ShareOnNetwork, Act::ShareWithAi, Act::Terminal, Act::RingPin] {
            assert!(folder.contains(&act), "{act:?}");
        }
        assert!(offered(&mut f, &["beach.png"]).contains(&Act::Wallpaper));
        for (parts, gone) in [
            (Parts { wallpaper: false, terminal: Some(true), ..Parts::all() }, Act::Wallpaper),
            (Parts { as_root: false, terminal: Some(true), ..Parts::all() }, Act::AsRoot),
            (Parts { helper: false, terminal: Some(true), ..Parts::all() }, Act::ShareOnNetwork),
            (Parts { ai: false, terminal: Some(true), ..Parts::all() }, Act::ShareWithAi),
            (Parts { terminal: Some(false), ..Parts::all() }, Act::Terminal),
        ] {
            f.parts = parts;
            let mut acts = offered(&mut f, &["Albums"]);
            acts.extend(offered(&mut f, &["beach.png"]));
            assert!(!acts.contains(&gone), "{gone:?}");
            assert_eq!(OUTSIDE.iter().filter(|a| acts.contains(a)).count(), OUTSIDE.len() - 1, "only {gone:?} goes: {acts:?}");
        }
        f.parts = Parts { terminal: Some(true), ..Parts::all() };
        f.ring = None;
        assert!(!offered(&mut f, &["Albums"]).contains(&Act::RingPin));
        f.parts = Parts::none();
        f.menu = None;
        f.open_menu(0);
        assert!(!f.menu.as_ref().unwrap().lines[0].rule);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn folders_code_reaches_no_other_part_of_trindle_but_these() {
        const ALLOWED: [&str; 3] = ["crate::common", "crate::files", "crate::toolkit"];
        fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() { walk(&p, out) } else if p.extension().is_some_and(|x| x == "rs") { out.push(p) }
            }
        }
        let mut files = Vec::new();
        walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src/files"), &mut files);
        assert!(files.len() > 10);
        let mut bad = Vec::new();
        for f in files.into_iter().filter(|f| !f.ends_with("alone.rs")) {
            let text = std::fs::read_to_string(&f).unwrap();
            for (n, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                let mut rest = code;
                while let Some(i) = rest.find("crate::") {
                    let tail = &rest[i..];
                    let end = tail[7..].find(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':')).map_or(tail.len(), |e| e + 7);
                    let path = &tail[..end];
                    if !ALLOWED.iter().any(|a| path.starts_with(a)) {
                        bad.push(format!("{}:{}: {path}", f.display(), n + 1));
                    }
                    rest = &tail[7..];
                }
            }
        }
        assert!(bad.is_empty(), "Folders reaches another part of Trindle: {bad:#?}");
    }
}
