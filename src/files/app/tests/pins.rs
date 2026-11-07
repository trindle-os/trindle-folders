// Pinned folders: let go in the sidebar, put in another order, and shown under names of their own.

use crate::files::menu::{Act, MenuMsg};
use crate::toolkit::popup::PopupMsg;

use super::*;

fn pinned_names(f: &Files) -> Vec<String> {
    f.pinned.iter().map(|p| crate::files::work::name_of(p)).collect()
}

fn pins_on_disk(kept: &Path) -> Vec<String> {
    places::read_pins(kept).iter().map(|p| crate::files::work::name_of(p)).collect()
}

fn drag_left(f: &mut Files, name: &str, gap: Option<usize>) {
    let (x, y) = item_centre(f, 1, name);
    f.pointer.set((600.0, 300.0));
    f.update(Msg::Icon(1, 0, Pointer::Down(x, y)));
    f.pointer.set((40.0, 120.0));
    f.update(Msg::Icon(1, 0, Pointer::Move(x - 300.0, y)));
    if let Some(gap) = gap {
        f.update(Msg::PinAt(gap));
    }
    f.update(Msg::Icon(1, 0, Pointer::Up(x - 300.0, y)));
}

#[test]
fn folders_are_pinned_where_they_are_let_go_and_put_in_another_order() {
    let (dir, mut f) = two_panes("pins");
    for d in ["Albums", "Letters", "Recipes", "Work"] {
        std::fs::create_dir_all(dir.join(d)).unwrap();
    }
    let kept = dir.join("pins-kept");
    f.pins_file = Some(kept.clone());
    f.pinned = vec![dir.join("Letters")];
    f.reload_all();
    pinned_by_the_left_button(&mut f, &dir, &kept);
    pinned_by_the_right_button(&mut f);
    pinned_folders_dragged_up_and_down(&mut f, &kept);
    pinned_folders_moved_from_their_menu(&mut f, &kept);
    let _ = std::fs::remove_dir_all(dir);
}

fn pinned_by_the_left_button(f: &mut Files, dir: &Path, kept: &Path) {
    drag_left(f, "Albums", Some(0));
    assert_eq!(pinned_names(f), ["Albums", "Letters"]);
    assert_eq!(pins_on_disk(kept), ["Albums", "Letters"], "kept in that order");
    assert!(dir.join("Albums").is_dir() && f.jobs.is_empty() && f.dropping.is_none(), "nothing is moved");
    assert_eq!(f.said, "Pinned Albums to the sidebar.");
    drag_left(f, "Work", Some(1));
    assert_eq!(pinned_names(f), ["Albums", "Work", "Letters"]);
    drag_left(f, "Albums", Some(3));
    assert_eq!(pinned_names(f), ["Albums", "Work", "Letters"]);
    assert!(f.said.contains("already"), "{}", f.said);
    drag_left(f, "apple.txt", Some(1));
    assert_eq!(pinned_names(f), ["Albums", "Work", "Letters"]);
    assert!(f.dropping.is_some(), "the usual question");
    f.update(Msg::DropStop);
    drag_left(f, "Recipes", None);
    assert!(f.dropping.is_some() && pinned_names(f).len() == 3);
    f.update(Msg::DropStop);
}

fn pinned_by_the_right_button(f: &mut Files) {
    let (x, y) = item_centre(f, 1, "Recipes");
    f.pointer.set((40.0, 120.0));
    f.update(Msg::Icon(1, 0, Pointer::RightDown(x, y)));
    f.update(Msg::Icon(1, 0, Pointer::RightMove(x - 300.0, y)));
    f.update(Msg::PinAt(3));
    f.update(Msg::Icon(1, 0, Pointer::RightUp(x - 300.0, y)));
    assert_eq!(pinned_names(f), ["Albums", "Work", "Letters", "Recipes"]);
    assert!(f.menu.is_none() && f.drag.is_none() && f.pin_at.is_none());
}

fn pinned_folders_dragged_up_and_down(f: &mut Files, kept: &Path) {
    f.update(Msg::PinMove(3, 0));
    assert_eq!(pinned_names(f), ["Recipes", "Albums", "Work", "Letters"]);
    f.update(Msg::PinMove(0, 4));
    assert_eq!(pinned_names(f), ["Albums", "Work", "Letters", "Recipes"]);
    f.update(Msg::PinMove(1, 2));
    f.update(Msg::PinMove(1, 1));
    f.update(Msg::PinMove(9, 0));
    assert_eq!(pinned_names(f), ["Albums", "Work", "Letters", "Recipes"]);
    assert_eq!(pins_on_disk(kept), ["Albums", "Work", "Letters", "Recipes"]);
}

fn pinned_menu(f: &mut Files, row: usize) -> Vec<Act> {
    f.update(Msg::Menu(MenuMsg::Side { pinned: true, row, x: 100.0, y: 140.0 }));
    f.menu.as_ref().unwrap().lines.iter().map(|l| l.act).collect()
}

fn pinned_folders_moved_from_their_menu(f: &mut Files, kept: &Path) {
    let first = pinned_menu(f, 0);
    assert!(first.contains(&Act::PinDown) && !first.contains(&Act::PinUp));
    let last = pinned_menu(f, 3);
    assert!(last.contains(&Act::PinUp) && !last.contains(&Act::PinDown));
    let words: Vec<&str> = f.menu.as_ref().unwrap().lines.iter().map(|l| l.words).collect();
    assert!(words.contains(&"Move up") && words.contains(&"Unpin"), "{words:?}");
    let trash = f.place_list().iter().position(|w| *w == Where::Trash).unwrap();
    f.update(Msg::Menu(MenuMsg::Side { pinned: false, row: trash, x: 100.0, y: 140.0 }));
    assert!(!f.menu.as_ref().unwrap().lines.iter().any(|l| matches!(l.act, Act::PinUp | Act::PinDown)));
    pick_from_pinned_menu(f, 3, Act::PinUp);
    assert_eq!(pinned_names(f), ["Albums", "Work", "Recipes", "Letters"]);
    pick_from_pinned_menu(f, 0, Act::PinDown);
    assert_eq!(pinned_names(f), ["Work", "Albums", "Recipes", "Letters"]);
    assert_eq!(pins_on_disk(kept), ["Work", "Albums", "Recipes", "Letters"]);
}

fn pick_from_pinned_menu(f: &mut Files, row: usize, act: Act) {
    let at = pinned_menu(f, row).iter().position(|a| *a == act).unwrap();
    f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(at))));
}

#[test]
fn a_pinned_folder_can_be_shown_under_a_name_of_its_own() {
    let (dir, mut f) = two_panes("pin-names");
    for d in ["Backup-Sync", "Work"] {
        std::fs::create_dir_all(dir.join(d)).unwrap();
    }
    let kept = dir.join("pins-kept");
    let names = places::names_beside(&kept);
    f.pins_file = Some(kept.clone());
    f.pinned = vec![dir.join("Backup-Sync"), dir.join("Work")];
    f.save_pins();
    let sync = dir.join("Backup-Sync");
    let acts = |f: &mut Files, pinned: bool, row: usize| {
        f.update(Msg::Menu(MenuMsg::Side { pinned, row, x: 100.0, y: 140.0 }));
        f.menu.as_ref().map(|m| m.lines.iter().map(|l| l.act).collect::<Vec<_>>()).unwrap_or_default()
    };
    for row in 0..f.place_list().len() {
        assert!(!acts(&mut f, false, row).contains(&Act::RenamePin), "place {row}");
        f.menu = None;
    }
    assert!(acts(&mut f, true, 0).contains(&Act::RenamePin));
    let at = acts(&mut f, true, 0).iter().position(|a| *a == Act::RenamePin).unwrap();
    f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(at))));
    assert_eq!(f.pin_rename, Some((sync.clone(), "Backup-Sync".to_string())), "the box opens on the name shown");
    f.update(Msg::PinNameText("nothing".into()));
    f.update(Msg::PinNameStop);
    assert!(f.pin_rename.is_none() && f.pin_names.is_empty() && !names.exists());
    f.pin_rename_start(sync.clone());
    f.update(Msg::PinNameText("  C-sync ".into()));
    f.update(Msg::PinNameDone);
    assert_eq!(f.pin_name(&sync), "C-sync");
    assert_eq!(f.pin_name(&dir.join("Work")), "Work");
    assert!(f.drop_places().iter().any(|(n, _)| n == "C-sync"));
    assert!(f.pins_named().contains(&(sync.clone(), "C-sync".to_string())));
    assert!(sync.is_dir() && !dir.join("C-sync").exists());
    assert_eq!(places::read_pins(&kept), [sync.clone(), dir.join("Work")]);
    assert_eq!(f.cols[0].tabs.iter().chain(f.cols[1].tabs.iter()).filter(|t| t.at == Where::Folder(sync.clone())).count(), 0);
    assert_eq!(places::read_names(&names).get(&sync).map(String::as_str), Some("C-sync"));
    f.go(0, Where::Folder(sync.clone()));
    assert_eq!(f.tab_name(&f.cols[0].at.clone()), "Backup-Sync", "the tab keeps the real name");
    f.pin_rename_start(sync.clone());
    f.update(Msg::PinNameText("  ".into()));
    f.update(Msg::PinNameDone);
    assert_eq!(f.pin_name(&sync), "Backup-Sync");
    assert!(places::read_names(&names).is_empty());
    assert!(acts(&mut f, true, 0).contains(&Act::RenamePin));
    f.pin_rename_start(sync.clone());
    f.update(Msg::PinNameText("C-sync".into()));
    f.update(Msg::PinNameDone);
    assert_eq!(places::read_names(&names).len(), 1);
    f.toggle_pin(sync.clone());
    assert!(f.pin_names.is_empty() && places::read_names(&names).is_empty());
    f.toggle_pin(sync.clone());
    assert_eq!(f.pin_name(&sync), "Backup-Sync", "pinned again, it has its own name");
    let _ = std::fs::remove_dir_all(dir);
}
