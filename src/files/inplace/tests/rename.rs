// F2, the menu, refused names and new things, named in the row.

use super::*;

fn keys_cases(b: &mut Bench, dir: &Path, p: usize, what: &str) {
    f2_types_and_keeps(b, dir, p, what);
    leaves_and_moves_on(b, dir, p, what);
}

fn f2_types_and_keeps(b: &mut Bench, dir: &Path, p: usize, what: &str) {
    let here = side_dir(dir, p);
    b.pick(p, "report.pdf");
    let before = b.places();
    let cell = b.cell(p, "report.pdf");
    b.key(Keysym::F2);
    assert_eq!(renaming(&b.f).as_deref(), Some("report.pdf"), "{what}: F2");
    let r = b.name_box().unwrap_or_else(|| panic!("{what}: the box is in the list"));
    assert!(r.x >= cell.x && r.x + r.w <= cell.x + cell.w + 0.5 && r.y >= cell.y && r.y + r.h <= cell.y + cell.h + 0.5,
            "{what}: the box {r:?} is inside its row {cell:?}");
    held_still(&before, &b.places(), what);
    assert_eq!(b.typed(), ("report.pdf".to_string(), 0.."report".len()), "{what}");
    b.type_in("plans");
    assert_eq!(b.typed().0, "plans.pdf", "{what}");
    assert!(crate::files::view::view(&b.f).said().contains(&"entry: New name for report.pdf".to_string()), "{what}");
    b.key(Keysym::Return);
    assert!(here.join("plans.pdf").is_file() && !here.join("report.pdf").exists(), "{what}: Enter keeps the name");
    assert!(b.f.renaming.is_none() && b.name_box().is_none());
    assert_eq!(chosen(&b.f, p).as_deref(), Some("plans.pdf"), "{what}");
    b.f.update(Msg::Undo);
    b.show();
    assert!(here.join("report.pdf").is_file() && !here.join("plans.pdf").exists(), "{what}: Undo");
}

fn leaves_and_moves_on(b: &mut Bench, dir: &Path, p: usize, what: &str) {
    let here = side_dir(dir, p);
    b.pick(p, "report.pdf");
    b.key(Keysym::F2);
    b.type_in("gone");
    b.key(Keysym::Escape);
    assert!(b.f.renaming.is_none() && b.name_box().is_none() && here.join("report.pdf").is_file() && !here.join("gone.pdf").exists(),
            "{what}: Escape");

    b.pick(p, "Albums");
    b.f.do_act(crate::files::menu::Act::Rename, p, vec![here.join("Albums")]);
    b.show();
    assert_eq!(renaming(&b.f).as_deref(), Some("Albums"), "{what}: the menu");
    assert_eq!(b.typed(), ("Albums".to_string(), 0.."Albums".len()), "{what}");
    b.key(Keysym::Escape);

    b.pick(p, "notes.txt");
    b.key(Keysym::F2);
    b.type_in("minutes");
    let other = b.cell(p, "Letters");
    let at = if b.f.cols[p].grid.get().lines { (other.x + 20.0, other.y + other.h / 2.0) } else { (other.x + other.w / 2.0, other.y + 30.0) };
    b.press_at(at.0, at.1);
    assert!(here.join("minutes.txt").is_file() && !here.join("notes.txt").exists(), "{what}: a click away keeps the name");
    assert!(b.f.renaming.is_none() && b.name_box().is_none());
    assert_eq!(chosen(&b.f, p).as_deref(), Some("Letters"), "{what}: the row clicked is chosen");
    b.f.update(Msg::Undo);
    b.show();
    assert!(here.join("notes.txt").is_file());

    let list = names(&b.f, p);
    let i = list.iter().position(|n| n == "notes.txt").unwrap();
    b.pick(p, "notes.txt");
    b.key(Keysym::F2);
    b.type_in("memo");
    b.key(Keysym::Tab);
    assert!(here.join("memo.txt").is_file(), "{what}: Tab keeps the name");
    assert_eq!(renaming(&b.f), Some(list[i + 1].clone()), "{what}: Tab goes on to the next");
    assert_eq!(b.typed().0, list[i + 1], "{what}: the next box has the keyboard");
    b.key_with(Keysym::ISO_Left_Tab, Mods { shift: true, ..Mods::default() });
    b.key_with(Keysym::ISO_Left_Tab, Mods { shift: true, ..Mods::default() });
    assert!(b.f.renaming.is_some(), "{what}: Shift+Tab goes back");
    b.key(Keysym::Escape);
    b.f.update(Msg::Undo);
    b.show();
    assert!(here.join("notes.txt").is_file() && !here.join("memo.txt").exists());
}

#[test]
fn f2_and_the_menu_rename_in_the_row_in_every_view() {
    every_pane_and_tab("keys", keys_cases);
}

fn refusal_cases(b: &mut Bench, dir: &Path, p: usize, what: &str) {
    let here = side_dir(dir, p);
    b.pick(p, "report.pdf");
    let before = b.places();
    b.key(Keysym::F2);
    let boxed = b.name_box().expect("open");
    let long = "x".repeat(256);
    for (typed, why) in [("", "A name cannot be empty."), ("a/b", "A name cannot hold a slash."),
                         ("notes.txt", "A file called notes.txt is already here."),
                         ("Albums", "A folder called Albums is already here."), (long.as_str(), "That name is too long.")] {
        b.key_with(Keysym::a, select_all());
        if typed.is_empty() {
            b.key(Keysym::BackSpace);
        } else {
            b.f.update(Msg::RenameText(typed.to_string()));
            b.show();
        }
        b.key(Keysym::Return);
        let r = b.f.renaming.clone().unwrap_or_else(|| panic!("{what}: the box stays open after {why}"));
        assert_eq!((r.text.as_str(), r.why.as_deref()), (typed, Some(why)), "{what}");
        assert!(b.f.bad && b.f.said == why, "{what}: {why} is said as a failure");
        let said = crate::files::view::view(&b.f).said();
        assert!(said.iter().any(|s| s == why), "{what}: {why} is on the page, under the box");
        assert_eq!(b.name_box(), Some(boxed), "{what}: the box keeps its place");
        held_still(&before, &b.places(), &format!("{what}, {why}"));
        assert_eq!(b.typed().0, typed, "{what}: the box still has the keyboard and what was typed");
        assert!(here.join("report.pdf").is_file());
    }
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&here, fs::Permissions::from_mode(0o555)).unwrap();
    if fs::write(here.join("probe"), "").is_ok() {
        let _ = fs::remove_file(here.join("probe"));
        fs::set_permissions(&here, fs::Permissions::from_mode(0o755)).unwrap();
        panic!("{}", crate::common::files::tests::NOT_AS_ROOT);
    }
    b.f.update(Msg::RenameText("locked.pdf".into()));
    b.show();
    b.key(Keysym::Return);
    fs::set_permissions(&here, fs::Permissions::from_mode(0o755)).unwrap();
    let r = b.f.renaming.clone().unwrap_or_else(|| panic!("{what}: the box stays open when the disk refuses"));
    assert!(r.text == "locked.pdf" && r.why.as_deref().is_some_and(|w| w.starts_with("report.pdf could not be renamed")), "{what}: {:?}", r.why);
    held_still(&before, &b.places(), &format!("{what}, the disk"));
    b.f.update(Msg::RenameText("fine.pdf".into()));
    b.show();
    b.key(Keysym::Return);
    assert!(here.join("fine.pdf").is_file() && b.f.renaming.is_none() && !b.f.bad, "{what}: {}", b.f.said);
    b.f.update(Msg::Undo);
    b.show();
    b.pick(p, "report.pdf");
    b.key(Keysym::F2);
    b.key_with(Keysym::a, select_all());
    b.key(Keysym::BackSpace);
    b.key(Keysym::Return);
    assert!(b.f.bad);
    b.key(Keysym::Escape);
    assert!(b.f.renaming.is_none() && !b.f.bad && b.f.said.is_empty() && here.join("report.pdf").is_file(), "{what}");
}

#[test]
fn a_name_that_cannot_be_used_says_why_under_the_box_and_nothing_moves() {
    every_pane_and_tab("refused", refusal_cases);
}

fn new_cases(b: &mut Bench, dir: &Path, p: usize, what: &str) {
    let here = side_dir(dir, p);
    b.f.side = p;
    b.show();
    let top_before: Vec<(String, Rect)> = b.places().into_iter().filter(|(id, _)| !id.starts_with(&format!("pane{p}:"))).collect();
    b.key_with(Keysym::N, Mods { ctrl: true, shift: true, ..Mods::default() });
    assert!(here.join("New folder").is_dir(), "{what}: made at once");
    let r = b.f.renaming.clone().unwrap_or_else(|| panic!("{what}: its name is open"));
    assert!(r.fresh && r.path == here.join("New folder") && r.side == p, "{what}");
    assert_eq!(chosen(&b.f, p).as_deref(), Some("New folder"), "{what}: it is chosen");
    assert_eq!(b.typed(), ("New folder".to_string(), 0.."New folder".len()), "{what}: selected, ready to type over");
    let cell = b.cell(p, "New folder");
    let boxed = b.name_box().expect("the box is in the list");
    assert!(boxed.y >= cell.y && boxed.y + boxed.h <= cell.y + cell.h + 0.5 && boxed.x >= cell.x, "{what}: {boxed:?} in {cell:?}");
    held_still(&top_before, &b.places(), what);
    b.type_in("Trips");
    b.key(Keysym::Return);
    assert!(here.join("Trips").is_dir() && !here.join("New folder").exists(), "{what}");
    assert_eq!(chosen(&b.f, p).as_deref(), Some("Trips"));
    assert_eq!(b.f.undo.last().map(|u| u.0.as_str()), Some("Undo making Trips"), "{what}: one undo, for the making");
    b.f.update(Msg::Undo);
    b.show();
    assert!(!here.join("Trips").exists(), "{what}: Undo removes it");
    b.f.update(Msg::NewFolder);
    b.show();
    b.key(Keysym::Escape);
    b.f.update(Msg::NewFolder);
    b.show();
    assert_eq!(renaming(&b.f).as_deref(), Some("New folder (2)"), "{what}");
    b.key(Keysym::Escape);
    assert!(here.join("New folder").is_dir() && here.join("New folder (2)").is_dir(), "{what}: Escape keeps the starting name");
    fs::remove_dir(here.join("New folder")).unwrap();
    fs::remove_dir(here.join("New folder (2)")).unwrap();
    b.f.update(Msg::Reload);
    b.f.side = p;
    b.f.do_act(crate::files::menu::Act::NewDocument, p, Vec::new());
    b.show();
    assert!(here.join("New document").is_file(), "{what}");
    assert_eq!(b.typed(), ("New document".to_string(), 0.."New document".len()), "{what}");
    assert!(b.name_box().is_some());
    b.type_in("list.txt");
    b.key(Keysym::Return);
    assert!(here.join("list.txt").is_file() && !here.join("New document").exists(), "{what}");
    b.f.update(Msg::Undo);
    b.show();
    assert!(!here.join("list.txt").exists());
}

#[test]
fn a_new_folder_and_a_new_document_are_named_where_they_appear() {
    every_pane_and_tab("new", new_cases);
}

#[test]
fn an_empty_folders_new_folder_is_named_in_the_list() {
    for view in VIEWS {
        let (dir, mut b) = bench(&format!("empty-{view:?}"), view);
        b.f.go(0, Where::Folder(dir.join("left/Albums")));
        b.show();
        assert!(crate::files::view::view(&b.f).said().contains(&"This folder is empty".to_string()));
        b.f.update(Msg::NewFolder);
        b.show();
        assert!(dir.join("left/Albums/New folder").is_dir());
        assert_eq!(b.typed(), ("New folder".to_string(), 0.."New folder".len()), "{view:?}");
        assert!(b.name_box().is_some(), "{view:?}");
        b.key(Keysym::Escape);
        fs::remove_dir_all(&dir).unwrap();
    }
}

#[test]
fn leaving_the_tab_keeps_the_name_typed() {
    let (dir, mut b) = bench("tab", View::List);
    b.f.update(Msg::NewTab(Some(0)));
    b.pick(0, "report.pdf");
    b.key(Keysym::F2);
    b.type_in("kept");
    b.f.update(Msg::Tab(0, 0));
    b.show();
    assert!(b.f.renaming.is_none() && dir.join("left/kept.pdf").is_file(), "{}", b.f.said);
    fs::remove_dir_all(&dir).unwrap();
}
