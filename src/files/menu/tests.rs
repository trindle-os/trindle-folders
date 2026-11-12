//! Tests of the menu: its rows for each place and pick, its sub-menus, and
use std::path::{Path, PathBuf};

use super::fold::{fold, FOR_GOOD, WHOLE};
use super::lines::without;
use super::*;
use crate::files::app::{Files, Msg};
use crate::files::icons::stand_in;
use crate::toolkit::popup::{Beside, Done, Place, PopupMsg};
use crate::toolkit::ui::App;

pub fn every(lines: &[Line]) -> Vec<&Line> {
    lines.iter().flat_map(|l| if l.sub.is_empty() { vec![l] } else { l.sub.iter().collect() }).collect()
}

pub fn find(lines: &[Line], act: Act) -> Option<(usize, Option<usize>)> {
    lines.iter().enumerate().find_map(|(i, l)| {
        if l.sub.is_empty() {
            (l.act == act).then_some((i, None))
        } else {
            l.sub.iter().position(|s| s.act == act).map(|j| (i, Some(j)))
        }
    })
}

pub const AIM: usize = 14;

impl Files {
        pub(in crate::files) fn menu_show(&mut self, act: Act) {
        if let Some((i, Some(j))) = self.menu.as_ref().and_then(|m| find(&m.lines, act)) {
            self.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(i))));
            self.update(Msg::Menu(MenuMsg::Pop(PopupMsg::SubHover(Some(j)))));
        }
    }

        pub(in crate::files) fn menu_pick(&mut self, act: Act) -> bool {
        let Some(at) = self.menu.as_ref().and_then(|m| find(&m.lines, act)) else { return false };
        match at {
            (i, None) => self.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(i)))),
            (i, Some(j)) => {
                self.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(i))));
                self.update(Msg::Menu(MenuMsg::Pop(PopupMsg::SubPick(j))));
            }
        }
        true
    }
}

fn without_terminal(lines: Vec<Line>, terminal: bool) -> Vec<Line> {
    if terminal {
        return lines;
    }
    without(lines, |act| act == Act::Terminal)
}

fn acts(at: &Where, sel: &[Sel]) -> Vec<Act> {
    lines_for(at, sel, false).into_iter().map(|l| l.act).collect()
}

#[test]
fn each_place_offers_its_own_rows() {
    let home = Where::Folder(PathBuf::from("/home/someone"));
    let folder = Sel { folder: true, ..Sel::default() };
    let image = Sel { image: true, turnable: true, resizable: true, ..Sel::default() };
    let zip = Sel { archive: true, ..Sel::default() };
    assert!(acts(&Where::Network, &[folder]).is_empty());
    assert_eq!(acts(&Where::Trash, &[]), [Act::SelectAll, Act::SelectNone, Act::Invert, Act::EmptyTrash]);
    assert_eq!(acts(&Where::Trash, &[image]), [Act::Restore, Act::Delete, Act::Properties]);
    let one = acts(&home, &[folder]);
    for a in [Act::OpenTab, Act::OpenWindow, Act::Terminal, Act::AsRoot, Act::Pin, Act::Colour, Act::Link, Act::ShareOnNetwork] {
        assert!(one.contains(&a), "{a:?}");
    }
    assert!(!acts(&home, &[folder, folder]).contains(&Act::ShareOnNetwork));
    assert!(!acts(&home, &[image]).contains(&Act::ShareOnNetwork));
    assert!(acts(&home, &[]).contains(&Act::Terminal));
    assert!(acts(&home, &[image]).contains(&Act::Terminal));
    assert!(acts(&home, &[image, zip]).contains(&Act::Terminal));
    assert!(acts(&home, &[folder, folder]).contains(&Act::Terminal));
    assert!(!one.contains(&Act::OpenWith) && !one.contains(&Act::Wallpaper));
    let pic = acts(&home, &[image]);
    assert!(pic.contains(&Act::Wallpaper) && pic.contains(&Act::OpenWith) && pic.contains(&Act::Preview) && !pic.contains(&Act::AsRoot));
    assert!(pic.contains(&Act::TurnLeft) && pic.contains(&Act::Resize) && !one.contains(&Act::Resize));
    let gif = acts(&home, &[Sel { image: true, resizable: true, ..Sel::default() }]);
    assert!(!gif.contains(&Act::TurnRight) && gif.contains(&Act::Resize));
    let font = Sel { font: true, ..Sel::default() };
    assert!(acts(&home, &[font, font]).contains(&Act::InstallFont) && !acts(&home, &[font, Sel::default()]).contains(&Act::InstallFont));
    assert!(acts(&home, &[zip]).contains(&Act::ExtractHere));
    let two = acts(&home, &[image, zip]);
    assert!(two.contains(&Act::Rename) && two.contains(&Act::Wipe) && !two.contains(&Act::Link) && !two.contains(&Act::ExtractHere));
    let zips = acts(&home, &[zip, zip]);
    assert!(zips.contains(&Act::ExtractTo) && !zips.contains(&Act::ExtractHere));
    assert!(!one.contains(&Act::Wipe) && one.contains(&Act::Dupes));
    let file = Sel::default();
    assert!(acts(&home, &[file, file]).contains(&Act::CompareFiles));
    assert!(!acts(&home, &[file]).contains(&Act::CompareFiles) && !acts(&home, &[file, folder]).contains(&Act::CompareFiles));
    assert_eq!(acts(&home, &[])[..3], [Act::NewFolder, Act::NewDocument, Act::Paste]);
    for sel in [vec![image], vec![folder], vec![zip, zip], vec![file, folder]] {
        assert_eq!(acts(&home, &sel)[..2], [Act::NewFolder, Act::NewDocument], "{}", sel.len());
    }
    let lines = lines_for(&home, &[image], false);
    let words = |a: Act| lines.iter().find(|l| l.act == a).map(|l| l.words);
    assert_eq!(words(Act::Trash), Some("Move to Trash"));
    assert_eq!(words(Act::Delete), Some("Delete permanently"));
    assert!(!lines.iter().any(|l| l.words.contains("for good") || l.words.contains("Put in")));
    assert_eq!(words(Act::Wipe), Some("Wipe securely"));
    assert!(!lines[0].rule && lines.iter().filter(|l| l.rule).count() >= 4);
}

struct Ringed {
    f: Files,
    dir: PathBuf,
}

impl Ringed {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!("trindle-files-ring-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Photos")).unwrap();
        std::fs::write(dir.join("letter.odt"), b"made up").unwrap();
        std::fs::write(dir.join("notes.txt"), b"made up").unwrap();
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        f.pinned = vec![dir.join("Photos")];
        Ringed { f, dir }
    }

    fn rows(&mut self, names: &[&str]) -> Vec<Line> {
        let f = &mut self.f;
        let picked: Vec<usize> = names.iter().map(|n| f.cols[0].entries.iter().position(|e| e.name == *n).unwrap()).collect();
        f.cols[0].chosen = picked.first().copied();
        f.cols[0].picked = picked;
        f.menu = None;
        f.open_menu(0);
        f.menu.take().map(|m| m.lines).unwrap_or_default()
    }

    fn side(&mut self) -> Vec<Line> {
        self.f.menu = None;
        self.f.update(Msg::Menu(MenuMsg::Side { pinned: true, row: 0, x: 100.0, y: 140.0 }));
        self.f.menu.take().map(|m| m.lines).unwrap_or_default()
    }

    fn ring_pin(&mut self, name: &str) {
        let path = self.dir.join(name);
        self.f.do_act(Act::RingPin, 0, vec![path]);
    }
}

fn ring_row(lines: &[Line]) -> Option<&'static str> {
    every(lines).into_iter().find(|l| l.act == Act::RingPin).map(|l| l.words)
}

fn tools(lines: &[Line]) -> Vec<Line> {
    lines.iter().find(|l| l.act == Act::Sub(Kind::Tools)).map(|l| l.sub.clone()).expect("the menu has Tools")
}

#[test]
fn pin_to_the_ring_is_offered_only_where_there_is_a_ring() {
    let mut r = Ringed::new();
    assert!(r.f.ring.is_none(), "a test has no ring unless it makes one up");
    for names in [&["Photos"][..], &["letter.odt"], &[]] {
        assert_eq!(ring_row(&r.rows(names)), None, "{names:?}");
    }
    assert_eq!(ring_row(&r.side()), None);
    r.ring_pin("Photos");
    assert!(r.f.said.is_empty() && !r.f.bad);

    let ring = r.dir.join("ring-favourites");
    r.f.ring = Some(ring.clone());
    the_ring_row_is_kept_in_tools(&mut r);
    the_ring_is_not_offered_everywhere(&r);
    pinning_is_kept_said_and_undone(&mut r, &ring);
    let _ = std::fs::remove_dir_all(&r.dir);
}

fn the_ring_row_is_kept_in_tools(r: &mut Ringed) {
    let folder = r.rows(&["Photos"]);
    assert_eq!(ring_row(&folder), Some("Pin to the ring"));
    let kept = tools(&folder);
    let at = kept.iter().position(|l| l.act == Act::RingPin).unwrap();
    assert_eq!(kept[at - 1].act, Act::Pin, "after Pin to the sidebar");
    assert!(!kept[at].rule);
    assert!(!folder.iter().any(|l| matches!(l.act, Act::Pin | Act::RingPin)), "and nowhere else");
    let file = r.rows(&["letter.odt"]);
    let kept = tools(&file);
    let at = kept.iter().position(|l| l.act == Act::RingPin).unwrap();
    assert!(at > 0 && kept[at].rule && kept[..at].iter().all(|l| matches!(l.act, Act::Preview | Act::Terminal)), "{kept:?}");
    assert!(!file.iter().any(|l| l.act == Act::RingPin));
    assert_eq!(file.last().map(|l| l.act), Some(Act::Properties));
    assert!(file[file.len() - 1].rule && !file[0].rule);
    assert_eq!(ring_row(&r.rows(&[])), Some("Pin to the ring"));
    assert_eq!(ring_row(&r.side()), Some("Pin to the ring"));
    assert_eq!(ring_row(&r.rows(&["Photos", "letter.odt"])), None);
}

fn the_ring_is_not_offered_everywhere(r: &Ringed) {
    let dir = &r.dir;
    assert_eq!(r.f.ring_has(&Where::Trash, &dir.join("Photos")), None);
    assert_eq!(r.f.ring_has(&Where::Remote("sftp://files.example.com/home".into()), Path::new("sftp://files.example.com/home/john")), None);
    assert_eq!(r.f.ring_has(&Where::Inside(dir.join("old.zip"), String::new()), &dir.join("old.zip/inner")), None);
    assert_eq!(r.f.ring_has(&Where::Folder(dir.clone()), &dir.join("two\nlines")), None);
    assert_eq!(r.f.ring_has(&Where::Recent, &dir.join("letter.odt")), Some(false));
}

fn pinning_is_kept_said_and_undone(r: &mut Ringed, ring: &Path) {
    use crate::common::favourites;
    let dir = r.dir.clone();
    r.ring_pin("Photos");
    assert_eq!(r.f.said, "Pinned Photos to the ring, next to your favourite apps.");
    r.ring_pin("letter.odt");
    let ids = favourites::read(ring);
    assert_eq!(ids[ids.len() - 2..], [dir.join("Photos").to_string_lossy().into_owned(), dir.join("letter.odt").to_string_lossy().into_owned()]);
    assert_eq!(&ids[..favourites::STARTING.len()], favourites::STARTING, "the apps stay");
    assert_eq!(ring_row(&r.rows(&["Photos"])), Some("Unpin from the ring"));
    assert_eq!(ring_row(&r.side()), Some("Unpin from the ring"));
    assert_eq!(ring_row(&r.rows(&["notes.txt"])), Some("Pin to the ring"));
    r.ring_pin("Photos");
    assert_eq!(r.f.said, "Unpinned Photos from the ring.");
    assert!(!favourites::has(ring, &dir.join("Photos")) && favourites::has(ring, &dir.join("letter.odt")));
    let full: Vec<String> = (0..favourites::MAX).map(|n| format!("app{n}.desktop")).collect();
    favourites::write(ring, &full).unwrap();
    r.ring_pin("Photos");
    assert!(r.f.bad && r.f.said.contains("full"), "{}", r.f.said);
    assert_eq!(favourites::read(ring), full);
}

#[test]
fn the_menu_is_the_toolkits_with_its_rows() {
    let menu = Menu { at: (5.0, 6.0), pane: 0, items: Vec::new(), lines: lines_for(&Where::Trash, &[], false), hover: None, narrow: false, before: Vec::new(), drop: None, beside: Beside::default() };
    let p = menu.popup();
    assert_eq!(p.place, Place::At(5.0, 6.0));
    assert_eq!(p.rows.len(), 4);
    assert!(p.rows[3].rule && p.rows[3].icon == stand_in::SHRED);
    assert_eq!(menu.popup().moved(1), Some(0));
    assert_eq!(menu.popup().moved(-1), Some(3));
    let menu = Menu { hover: Some(3), ..menu };
    assert_eq!(menu.popup().moved(1), Some(0));
    assert_eq!(menu.popup().lit().hover, Some(0));
}

#[test]
fn the_sidebar_menu_offers_what_suits_a_place() {
    let home = Where::Folder(PathBuf::from("/home/jo/Music"));
    let lines = side_lines(&home, true);
    let acts: Vec<Act> = lines.iter().map(|l| l.act).collect();
    assert_eq!(acts[0], Act::Open);
    assert!(acts.contains(&Act::OpenTab) && acts.contains(&Act::Pin) && acts.contains(&Act::Colour));
    assert!(!acts.contains(&Act::Rename) && !acts.contains(&Act::Trash) && !acts.contains(&Act::Delete));
    assert!(lines.iter().any(|l| l.words == "Unpin"));
    assert!(!lines[0].rule);
    let plain = without_terminal(lines_for(&home, &[], false), false);
    assert!(!plain.iter().any(|l| l.act == Act::Terminal) && plain.iter().any(|l| l.act == Act::AsRoot && l.rule));
    assert!(!plain[0].rule);
    let side = without_terminal(lines.clone(), false);
    assert!(!side.iter().any(|l| l.act == Act::Terminal) && side.len() + 1 == lines.len() && !side[0].rule);
    let trash = side_lines(&Where::Trash, false);
    assert_eq!(trash.iter().map(|l| l.act).collect::<Vec<_>>(), [Act::EmptyTrash]);
    assert!(!trash[0].rule);
    assert!(side_lines(&Where::Recent, false).is_empty());
    let menu = Menu { at: (1.0, 2.0), pane: 0, items: Vec::new(), lines, hover: None, narrow: true, before: Vec::new(), drop: None, beside: Beside::default() };
    assert_eq!(menu.popup().width, SIDE_WIDTH);
}

fn cases() -> Vec<(String, Vec<Line>)> {
    use crate::files::alone::Parts;
    use crate::files::earlier::{looks_only, with_row};
    let file = Sel::default();
    let folder = Sel { folder: true, ..Sel::default() };
    let zip = Sel { archive: true, ..Sel::default() };
    let image = Sel { image: true, turnable: true, resizable: true, ..Sel::default() };
    let gif = Sel { image: true, resizable: true, ..Sel::default() };
    let font = Sel { font: true, ..Sel::default() };
    let places = [("a folder", Where::Folder(PathBuf::from("/home/john/Documents"))), ("the Trash", Where::Trash), ("Recent", Where::Recent),
                  ("a search", Where::Found(PathBuf::from("/home/john"), "letter".into(), false)), ("a server or phone", Where::Remote("sftp://files.example.com/home/john".into())),
                  ("inside an archive", Where::Inside(PathBuf::from("/home/john/old.zip"), String::new())), ("Network", Where::Network), ("Storage", Where::Computer)];
    let picked = [("empty room", vec![]), ("one file", vec![file]), ("two files", vec![file, file]), ("three files", vec![file, file, file]),
                  ("a folder", vec![folder]), ("two folders", vec![folder, folder]), ("a file and a folder", vec![file, folder]),
                  ("an archive", vec![zip]), ("two archives", vec![zip, zip]), ("three archives", vec![zip, zip, zip]), ("a picture", vec![image]), ("two pictures", vec![image, image]),
                  ("three pictures", vec![image, image, image]), ("a GIF", vec![gif]), ("a font", vec![font]), ("two fonts", vec![font, font]), ("three fonts", vec![font, font, font])];
    let parts = [("every part", Parts { wallpaper: true, as_root: true, helper: true, ai: true, terminal: Some(true) }), ("no other part", Parts::none()),
                 ("no terminal", Parts { wallpaper: true, as_root: true, helper: true, ai: true, terminal: Some(false) })];
    let mut out = Vec::new();
    for (place, at) in &places {
        for (what, sel) in &picked {
            for pinned in [false, true] {
                for (has, parts) in &parts {
                    let lines = parts.fit(lines_for(at, sel, pinned));
                    if lines.is_empty() {
                        continue;
                    }
                    let name = format!("{what} in {place}, {has}{}", if pinned { ", pinned" } else { "" });
                    out.push((name.clone(), lines.clone()));
                    let ring = sel.len() <= 1 && matches!(at, Where::Folder(_) | Where::Recent | Where::Found(..));
                    if ring {
                        out.push((format!("{name}, a ring"), with_ring(lines.clone(), false)));
                        out.push((format!("{name}, on the ring"), with_ring(lines.clone(), true)));
                    }
                    if matches!(at, Where::Folder(_)) {
                        if sel.len() <= 1 {
                            out.push((format!("{name}, earlier copies kept"), with_row(lines.clone(), sel.len() == 1)));
                            out.push((format!("{name}, earlier copies kept, a ring"), with_ring(with_row(lines.clone(), sel.len() == 1), false)));
                        }
                        out.push((format!("{name}, in an earlier copy"), looks_only(lines.clone())));
                    }
                }
            }
        }
    }
    let side = Where::Folder(PathBuf::from("/home/john/Music"));
    for (has, parts) in &parts {
        for pinned in [false, true] {
            let lines = parts.fit(side_lines(&side, pinned));
            out.push((format!("a sidebar place, {has}"), lines.clone()));
            for (row, count) in [(0, 3), (1, 3), (2, 3)] {
                out.push((format!("a pinned folder in the sidebar, {has}, row {row}"), with_ring(pinned_lines(lines.clone(), row, count), true)));
            }
        }
    }
    out.push(("the Trash in the sidebar".into(), side_lines(&Where::Trash, false)));
    out
}

fn by_name(mut acts: Vec<(Act, &'static str, &'static str, &'static str)>) -> Vec<(Act, &'static str, &'static str, &'static str)> {
    acts.sort_by_key(|a| format!("{:?} {}", a.0, a.1));
    acts
}

#[test]
fn every_case_is_short_on_its_first_level_and_loses_no_row() {
    let all = cases();
    assert!(all.len() > 400, "the walk is over every case: {}", all.len());
    let mut homes: Vec<(Act, Kind)> = Vec::new();
    let mut longest = (0, String::new());
    for (name, flat) in &all {
        let sidebar = name.contains("sidebar");
        let folded = if sidebar { flat.clone() } else { fold(flat.clone()) };
        assert!(folded.len() <= AIM, "{name}: {} rows on the first level: {:?}", folded.len(), folded.iter().map(|l| l.words).collect::<Vec<_>>());
        if folded.len() > longest.0 {
            longest = (folded.len(), name.clone());
        }
        let whole = |lines: &[&Line]| by_name(lines.iter().map(|l| (l.act, l.words, l.icon, l.keys)).collect());
        assert_eq!(whole(&every(&folded)), whole(&flat.iter().collect::<Vec<_>>()), "{name}");
        assert!(!every(&folded).iter().any(|l| matches!(l.act, Act::Sub(_)) || !l.sub.is_empty()), "{name}: sub-menus are one deep");
        if flat.len() <= WHOLE {
            assert_eq!(&folded, flat, "{name}");
        }
        let subs: Vec<&Line> = folded.iter().filter(|l| !l.sub.is_empty()).collect();
        check_sub_menus(name, &subs, &mut homes);
        check_first_level(name, flat, &folded, subs.len(), sidebar);
    }
    assert!(longest.0 >= AIM - 1, "the longest first level is {} rows: {}", longest.0, longest.1);
    for (act, kind) in &homes {
        assert_eq!(homes.iter().filter(|(a, _)| a == act).count(), 1, "{act:?} is in {kind:?} and elsewhere");
    }
    for kind in [Kind::Select, Kind::Pack, Kind::Picture, Kind::Tools] {
        assert!(homes.iter().any(|(_, k)| *k == kind), "{kind:?}");
    }
    the_named_cases_stand();
}

fn check_sub_menus(name: &str, subs: &[&Line], homes: &mut Vec<(Act, Kind)>) {
    assert!(subs.len() <= 3, "{name}: {} sub-menus", subs.len());
    for s in subs {
        let Act::Sub(kind) = s.act else { panic!("{name}: {:?} holds rows", s.act) };
        assert!(s.sub.len() >= 2, "{name}: {} holds one row", s.words);
        assert!(!FOR_GOOD.contains(&s.sub[0].act) && !s.sub[0].rule, "{name}: {} opens on {}", s.words, s.sub[0].words);
        assert!(s.keys.is_empty() && !s.words.is_empty() && s.words != "More");
        let gone: Vec<usize> = s.sub.iter().enumerate().filter(|(_, l)| FOR_GOOD.contains(&l.act)).map(|(i, _)| i).collect();
        if let Some(&first) = gone.first() {
            assert!(s.sub[first].rule && gone == (first..s.sub.len()).collect::<Vec<_>>(), "{name}: {:?}", s.sub.iter().map(|l| l.words).collect::<Vec<_>>());
        }
        for l in &s.sub {
            if !homes.contains(&(l.act, kind)) {
                homes.push((l.act, kind));
            }
        }
    }
}

fn check_first_level(name: &str, flat: &[Line], folded: &[Line], subs: usize, sidebar: bool) {
    if let Some(first) = folded.iter().position(|l| !l.sub.is_empty()) {
        assert!(folded[first..first + subs].iter().all(|l| !l.sub.is_empty()), "{name}");
        assert!(first == 0 || folded[first].rule, "{name}");
        assert!(folded.get(first + subs).is_none_or(|l| l.rule), "{name}");
    }
    assert!(folded.first().is_none_or(|l| !l.rule), "{name}");
    if flat.iter().any(|l| l.act == Act::NewFolder) {
        assert_eq!([folded[0].act, folded[1].act], [Act::NewFolder, Act::NewDocument], "{name}");
    }
    for rows in std::iter::once(folded).chain(folded.iter().filter(|l| !l.sub.is_empty()).map(|s| s.sub.as_slice())) {
        if let Some(at) = rows.iter().position(|l| l.act == Act::SelectAll) {
            assert_eq!(rows[at..].iter().take(3).map(|l| l.act).collect::<Vec<_>>(), [Act::SelectAll, Act::SelectNone, Act::Invert], "{name}");
        }
    }
    assert_eq!(every(folded).iter().filter(|l| l.act == Act::Invert).count(), every(folded).iter().filter(|l| l.act == Act::SelectAll).count(), "{name}");
    if !sidebar {
        if let Some(at) = folded.iter().position(|l| FOR_GOOD.contains(&l.act)) {
            let open = folded.iter().position(|l| l.act == Act::Open);
            assert!(open.is_none_or(|o| o < at), "{name}");
            assert!(folded[at..].iter().all(|l| FOR_GOOD.contains(&l.act) || matches!(l.act, Act::Properties | Act::RingPin | Act::Earlier)), "{name}: {:?}", folded.iter().map(|l| l.words).collect::<Vec<_>>());
        }
    }
}

fn the_named_cases_stand() {
    let level = |at: &Where, sel: &[Sel]| -> Vec<&'static str> {
        let all = crate::files::alone::Parts { wallpaper: true, as_root: true, helper: true, ai: true, terminal: Some(true) };
        let lines = all.fit(lines_for(at, sel, false));
        let lines = if matches!(at, Where::Folder(_)) { with_ring(lines, false) } else { lines };
        fold(lines).iter().map(|l| l.words).collect()
    };
    let home = Where::Folder(PathBuf::from("/home/john/Documents"));
    assert_eq!(level(&home, &[Sel::default()]),
               ["New folder", "New document", "Open", "Open with another app", "Cut", "Copy", "Copy location", "Rename", "Compress, send and share", "Tools", "Move to Trash", "Delete permanently", "Properties"]);
    assert_eq!(level(&home, &[]), ["New folder", "New document", "Paste", "Select", "Tools", "Properties"]);
    assert_eq!(level(&home, &[Sel { archive: true, ..Sel::default() }])[8..], ["Extract here", "Compress, send and share", "Tools", "Move to Trash", "Delete permanently", "Properties"]);
    assert_eq!(level(&home, &[Sel { folder: true, ..Sel::default() }]),
               ["New folder", "New document", "Open", "Open in a new tab", "Cut", "Copy", "Copy location", "Rename", "Compress, send and share", "Tools", "Move to Trash", "Delete permanently", "Properties"]);
    assert_eq!(level(&Where::Trash, &[Sel::default()]), ["Put back", "Delete permanently", "Properties"]);
}

#[test]
fn the_keyboard_reaches_every_row_of_every_case() {
    use crate::toolkit::input::Keysym;
    use crate::toolkit::popup::key;
    for (name, flat) in cases() {
        let lines = if name.contains("sidebar") { flat } else { fold(flat) };
        let menu = Menu { at: (300.0, 200.0), pane: 0, items: Vec::new(), lines, hover: None, narrow: false, before: Vec::new(), drop: None, beside: Beside::default() };
        let mut pop = menu.popup().lit();
        let mut reached: Vec<Act> = Vec::new();
        for i in 0..menu.lines.len() {
            assert_eq!(pop.hover, Some(i), "{name}");
            let top = &menu.lines[i];
            if top.sub.is_empty() {
                assert_eq!(pop.said(), if top.keys.is_empty() { top.words.to_string() } else { format!("{}, {}", top.words, top.keys) }, "{name}");
                assert_eq!(pop.clone().update(key(Keysym::Return)), Done::Picked(i), "{name}");
                reached.push(menu.chosen(i, None).unwrap().act);
            } else {
                assert_eq!(pop.said(), format!("{}, sub-menu, {} lines", top.words, top.sub.len()), "{name}");
                assert_eq!(pop.update(key(Keysym::Right)), Done::Open, "{name}");
                for (j, inner) in top.sub.iter().enumerate() {
                    assert_eq!((pop.beside.open, pop.beside.hover), (Some(i), Some(j)), "{name}");
                    assert!(pop.said().starts_with(inner.words), "{name}: {}", pop.said());
                    assert_eq!(pop.clone().update(key(Keysym::Return)), Done::PickedIn(i, j), "{name}");
                    reached.push(menu.chosen(i, Some(j)).unwrap().act);
                    pop.update(key(Keysym::Down));
                }
                assert_eq!(pop.beside.hover, Some(0), "{name}: round to its first row");
                assert_eq!(pop.update(key(Keysym::Left)), Done::Open, "{name}");
                assert_eq!((pop.beside.open, pop.hover), (None, Some(i)), "{name}");
            }
            pop.update(key(Keysym::Down));
        }
        assert_eq!(pop.hover, Some(0), "{name}: round to the first row");
        assert_eq!(reached, every(&menu.lines).iter().map(|l| l.act).collect::<Vec<_>>(), "{name}");
    }
}

#[test]
fn every_sub_menu_stays_inside_the_window_at_each_corner() {
    let mut opened = 0;
    for (name, flat) in cases() {
        if name.contains("sidebar") || !name.contains("every part") {
            continue;
        }
        let lines = fold(flat);
        for room in [(1200.0, 760.0), (700.0, 520.0)] {
            for at in [(0.0, 0.0), (room.0, 0.0), (0.0, room.1), (room.0, room.1), (room.0 / 2.0, room.1 / 2.0), (room.0 - 300.0, room.1 - 120.0)] {
                let menu = Menu { at, pane: 0, items: Vec::new(), lines: lines.clone(), hover: None, narrow: false, before: Vec::new(), drop: None, beside: Beside::default() };
                let inside = |r: crate::toolkit::widget::Rect| r.x >= 0.0 && r.y >= 0.0 && r.x + r.w <= room.0 && r.y + r.h <= room.1;
                let pop = menu.popup().fits(room.1);
                let (whole, none) = pop.spots(room);
                assert!(inside(whole) && none.is_none(), "{name} at {at:?} in {room:?}: {whole:?}");
                for (i, _) in lines.iter().enumerate().filter(|(_, l)| !l.sub.is_empty()) {
                    let mut pop = pop.clone();
                    assert_eq!(pop.update(PopupMsg::Pick(i)), Done::Open, "{name}");
                    let (whole, sub) = pop.spots(room);
                    let sub = sub.expect("an open sub-menu has a place");
                    assert!(inside(sub), "{name}, {} at {at:?} in {room:?}: {sub:?}", lines[i].words);
                    assert!(sub.x >= whole.x + whole.w - 12.0 || sub.x + sub.w <= whole.x + 12.0 || room.0 < whole.w + sub.w, "{name}: {whole:?} {sub:?}");
                    opened += 1;
                }
            }
        }
    }
    assert!(opened > 500, "{opened}");
}

#[test]
fn a_sub_menu_works_in_folders_by_pointer_keys_and_screen_reader() {
    use crate::toolkit::input::Keysym;
    use crate::toolkit::popup::key;
    let dir = std::env::temp_dir().join(format!("trindle-files-sub-menu-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Photos")).unwrap();
    std::fs::write(dir.join("letter.odt"), b"made up").unwrap();
    let mut f = Files::new(vec![dir.clone()]);
    f.go(0, Where::Folder(dir.clone()));
    let file = f.cols[0].entries.iter().position(|e| e.name == "letter.odt").unwrap();
    let pop = |m: PopupMsg| Msg::Menu(MenuMsg::Pop(m));
    f.cols[0].chosen = Some(file);
    f.cols[0].picked = vec![file];
    f.update(Msg::Menu(MenuMsg::Open));
    assert_eq!(f.menu_told, "New folder, Ctrl+Shift+N");
    let lines = f.menu.as_ref().unwrap().lines.clone();
    assert!(lines.len() <= AIM, "{}", lines.len());
    assert!(matches!(find(&lines, Act::CopyPath), Some((_, None))) && matches!(find(&lines, Act::Delete), Some((_, None))));
    let (pack, Some(place)) = find(&lines, Act::Link).expect("Make a link is in the menu") else { panic!("Make a link is not in a sub-menu") };
    assert_eq!(lines[pack].act, Act::Sub(Kind::Pack));
    f.update(pop(PopupMsg::Point { row: Some(pack), x: 40, y: 300, toward: false }));
    let menu = f.menu.as_ref().unwrap();
    let Some((number, row)) = menu.beside.due else { panic!("pointing begins the pause") };
    assert_eq!((row, menu.beside.open, menu.hover), (pack, None, Some(pack)));
    assert_eq!(f.menu_told, format!("Compress, send and share, sub-menu, {} lines", lines[pack].sub.len()));
    f.update(pop(PopupMsg::Due(number)));
    assert_eq!(f.menu.as_ref().unwrap().beside.open, Some(pack));
    f.update(pop(PopupMsg::SubHover(Some(place))));
    assert_eq!(f.menu_told, "Make a link");
    f.update(pop(PopupMsg::SubPick(place)));
    assert!(f.menu.is_none());
    assert!(f.said.starts_with("Made "), "{}", f.said);
    f.update(Msg::Menu(MenuMsg::Open));
    for _ in 0..pack {
        f.update(pop(key(Keysym::Down)));
    }
    f.update(pop(key(Keysym::Right)));
    let menu = f.menu.as_ref().unwrap();
    assert_eq!((menu.beside.open, menu.beside.hover), (Some(pack), Some(0)));
    assert_eq!(f.menu_told, lines[pack].sub[0].words);
    f.update(pop(key(Keysym::Escape)));
    let menu = f.menu.as_ref().expect("Escape shuts only the sub-menu");
    assert_eq!((menu.beside.open, menu.hover), (None, Some(pack)));
    f.update(pop(key(Keysym::Escape)));
    assert!(f.menu.is_none());
    f.do_act(Act::Sub(Kind::Tools), 0, vec![dir.join("letter.odt")]);
    assert!(f.menu.is_none() && f.dialog.is_none());
    f.update(Msg::Menu(MenuMsg::Open));
    f.menu_show(Act::Wipe);
    let menu = f.menu.as_ref().unwrap();
    let tools = &menu.lines[menu.beside.open.expect("Tools is open")];
    assert_eq!((tools.act, tools.sub.last().map(|l| l.act)), (Act::Sub(Kind::Tools), Some(Act::Wipe)));
    let mut ui = crate::toolkit::ui::Ui::new(crate::toolkit::ui::App::view(&f), crate::toolkit::theme::Theme::new(true, "#fbc711"));
    let mut pix = tiny_skia::Pixmap::new(1200, 760).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    let _ = std::fs::remove_dir_all(dir);
}
