// Dragging items onto folders and the menu a right drag ends in.

use std::fs;

use crate::files::app::{Files, Msg};
use crate::files::places::Where;
use crate::files::app::View;
use crate::toolkit::ui::App;
use crate::toolkit::widget::Pointer;
use super::*;

#[test]
fn dragging_onto_a_folder_moves_into_it() {
    let dir = folder("drag");
    fs::write(dir.join("left/a.txt"), b"a").unwrap();
    fs::create_dir(dir.join("left/box")).unwrap();
    let mut f = files(&dir);
    f.view = View::Icons;
    let pane = &f.cols[0];
    pane.width.set(440.0);
    pane.grid.set(crate::files::icons::Grid::icons(440.0));
    assert_eq!(pane.entries[0].name, "box");
    f.pointer.set((panes_left() + 150.0, 100.0));
    f.update(Msg::Icon(0, 0, Pointer::Down(150.0, 20.0)));
    f.pointer.set((panes_left() + 60.0, 100.0));
    f.update(Msg::Icon(0, 0, Pointer::Move(40.0, 20.0)));
    assert!(f.drag.as_ref().is_some_and(|d| d.moving));
    f.update(Msg::Icon(0, 0, Pointer::Up(40.0, 20.0)));
    assert!(dir.join("left/box/a.txt").exists());
    assert!(f.drag.is_none());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_right_drag_ends_in_a_menu_and_does_what_is_picked() {
    use crate::files::menu::Act;
    let dir = folder("right-drag");
    fs::write(dir.join("left/a.txt"), b"a").unwrap();
    fs::create_dir(dir.join("left/box")).unwrap();
    let mut f = files(&dir);
    icons(&mut f);
    let words = |f: &Files| f.menu.as_ref().map(|m| m.lines.iter().map(|l| l.words).collect::<Vec<_>>()).unwrap_or_default();
    right_drag(&mut f, (150.0, 20.0), panes_left() + 40.0, (40.0, 20.0));
    assert!(f.drag.is_none());
    assert_eq!(words(&f), ["Copy here", "Move here", "Make a link here", "Cancel"]);
    assert_eq!(f.menu.as_ref().unwrap().drop, Some(Where::Folder(dir.join("left/box"))));
    assert!(dir.join("left/a.txt").exists() && !dir.join("left/box/a.txt").exists());
    pick_line(&mut f, Act::LeaveThem);
    assert!(f.menu.is_none() && dir.join("left/a.txt").exists() && !dir.join("left/box/a.txt").exists());
    right_drag(&mut f, (150.0, 20.0), panes_left() + 40.0, (40.0, 20.0));
    f.update(Msg::Menu(crate::files::menu::MenuMsg::Pop(crate::toolkit::popup::key(crate::toolkit::input::Keysym::Escape))));
    assert!(f.menu.is_none() && !dir.join("left/box/a.txt").exists());
    right_drag(&mut f, (150.0, 20.0), panes_left() + 40.0, (40.0, 20.0));
    pick_line(&mut f, Act::CopyHere);
    assert!(dir.join("left/a.txt").exists() && dir.join("left/box/a.txt").exists());
    right_drag(&mut f, (150.0, 20.0), panes_left() + 40.0, (40.0, 20.0));
    pick_line(&mut f, Act::LinkHere);
    let link = dir.join("left/box/Link to a.txt");
    assert_eq!(fs::read_link(&link).unwrap(), dir.join("left/a.txt"));
    assert_eq!(f.undo.last().unwrap().0, "Undo making Link to a.txt");
    f.update(Msg::Undo);
    assert!(fs::symlink_metadata(&link).is_err());
    right_drag(&mut f, (150.0, 20.0), panes_left() + 600.0, (600.0, 20.0));
    assert_eq!(f.menu.as_ref().unwrap().drop, Some(Where::Folder(dir.join("right"))));
    pick_line(&mut f, Act::MoveHere);
    assert!(!dir.join("left/a.txt").exists() && dir.join("right/a.txt").exists());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_right_drag_lands_on_a_sidebar_row_a_tab_or_its_own_folder() {
    use crate::files::menu::Act;
    let dir = folder("right-land");
    fs::write(dir.join("left/a.txt"), b"a").unwrap();
    fs::create_dir_all(dir.join("pinned")).unwrap();
    fs::create_dir_all(dir.join("tabbed")).unwrap();
    let mut f = files(&dir);
    f.pinned = vec![dir.join("pinned")];
    icons(&mut f);
    let land = |f: &mut Files, land: Land| {
        f.pointer.set((panes_left() + 20.0, 100.0));
        f.update(Msg::Icon(0, 0, Pointer::RightDown(20.0, 20.0)));
        f.pointer.set((40.0, 100.0));
        f.update(Msg::Icon(0, 0, Pointer::RightMove(-200.0, 20.0)));
        f.update(Msg::Landed(land));
        f.update(Msg::Icon(0, 0, Pointer::RightUp(-200.0, 20.0)));
    };
    land(&mut f, Land::Pinned(0));
    assert_eq!(f.menu.as_ref().unwrap().drop, Some(Where::Folder(dir.join("pinned"))));
    assert!(f.dropping.is_none(), "no asking where: the row said");
    pick_line(&mut f, Act::CopyHere);
    assert!(dir.join("pinned/a.txt").exists());
    let trash = f.place_list().iter().position(|w| *w == Where::Trash).unwrap();
    land(&mut f, Land::Place(trash));
    let words: Vec<&str> = f.menu.as_ref().unwrap().lines.iter().map(|l| l.words).collect();
    assert_eq!(words, ["Move to Trash", "Cancel"]);
    f.menu = None;
    let recent = f.place_list().iter().position(|w| *w == Where::Recent).unwrap();
    land(&mut f, Land::Place(recent));
    assert!(f.menu.is_none() && f.said.contains("only be dropped on a folder"), "{}", f.said);
    f.update(Msg::NewTab(Some(1)));
    f.go(1, Where::Folder(dir.join("tabbed")));
    f.update(Msg::Tab(1, 0));
    f.side = 0;
    land(&mut f, Land::Tab(1, 1));
    assert_eq!(f.menu.as_ref().unwrap().drop, Some(Where::Folder(dir.join("tabbed"))));
    pick_line(&mut f, Act::MoveHere);
    assert!(dir.join("tabbed/a.txt").exists() && !dir.join("left/a.txt").exists());
    fs::write(dir.join("left/b.txt"), b"b").unwrap();
    f.update(Msg::Reload);
    f.update(Msg::Changed);
    right_drag(&mut f, (20.0, 20.0), panes_left() + 300.0, (300.0, 300.0));
    let words: Vec<&str> = f.menu.as_ref().unwrap().lines.iter().map(|l| l.words).collect();
    assert_eq!(words, ["Copy here", "Make a link here", "Cancel"]);
    pick_line(&mut f, Act::CopyHere);
    assert!(dir.join("left/b (2).txt").exists());
    f.update(Msg::Reload);
    f.pointer.set((panes_left() + 20.0, 100.0));
    f.update(Msg::Icon(0, 0, Pointer::Down(20.0, 20.0)));
    f.pointer.set((40.0, 100.0));
    f.update(Msg::Icon(0, 0, Pointer::Move(-200.0, 20.0)));
    f.update(Msg::Landed(Land::Pinned(0)));
    f.update(Msg::Icon(0, 0, Pointer::Up(-200.0, 20.0)));
    assert!(f.menu.is_none() && f.dropping.is_some());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn dragging_onto_a_folder_in_the_other_pane_moves_into_it() {
    let dir = folder("drag-across");
    fs::write(dir.join("left/a.txt"), b"a").unwrap();
    fs::create_dir(dir.join("right/box")).unwrap();
    let mut f = files(&dir);
    f.view = View::Icons;
    f.update(Msg::Changed);
    for p in 0..2 {
        let pane = &f.cols[p];
        pane.width.set(440.0);
        pane.grid.set(crate::files::icons::Grid::icons(440.0));
    }
    assert!(f.two);
    f.pointer.set((panes_left() + 50.0, 100.0));
    f.update(Msg::Icon(0, 0, Pointer::Down(50.0, 20.0)));
    f.pointer.set((panes_left() + 600.0, 100.0));
    f.update(Msg::Icon(0, 0, Pointer::Move(600.0, 20.0)));
    f.update(Msg::Icon(1, 0, Pointer::Hover(50.0, 20.0)));
    assert_eq!(f.cols[1].hover, Some(0));
    assert_eq!(f.side(), 0);
    f.update(Msg::Icon(0, 0, Pointer::Up(600.0, 20.0)));
    assert!(dir.join("right/box/a.txt").exists());
    fs::remove_dir_all(&dir).unwrap();
}
