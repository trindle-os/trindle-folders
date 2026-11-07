// Picking things: by click, box, tick, key and menu, and how grouping and the other pane change what is picked.

use crate::files::groups::{rank, GroupBy};
use crate::files::menu::Act;

use super::*;

#[test]
fn show_in_folder_opens_the_folder_with_the_items_picked() {
    let dir = std::env::temp_dir().join(format!("trindle-files-reveal-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("b")).unwrap();
    std::fs::create_dir_all(dir.join("a")).unwrap();
    for f in ["a/1.txt", "a/2.txt", "a/3.txt"] {
        std::fs::write(dir.join(f), "").unwrap();
    }
    let mut f = Files::new(Vec::new());
    f.for_a_call = true;
    let tabs = f.cols[0].tabs.len();
    f.update(Msg::Reveal(vec![dir.join("a/2.txt"), dir.join("a/3.txt"), dir.join("b")], false));
    assert_eq!(f.cols[0].tabs.len(), tabs + 1);
    let names = |pane: &Pane| -> Vec<String> { pane.picked.iter().map(|&i| pane.entries[i].name.clone()).collect() };
    assert_eq!(f.cols[0].tabs[0].at, Where::Folder(dir.join("a")));
    assert_eq!(names(&f.cols[0].tabs[0]), ["2.txt", "3.txt"]);
    assert_eq!(f.cols[0].tabs[1].at, Where::Folder(dir.clone()));
    assert_eq!(names(&f.cols[0].tabs[1]), ["b"]);
    f.update(Msg::Add(vec![dir.join("b")]));
    assert_eq!(f.cols[0].tabs.len(), tabs + 2);
    assert!(!f.for_a_call);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn clicks_and_a_box_pick_icons() {
    let dir = std::env::temp_dir().join(format!("trindle-files-pick-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for n in 0..6 {
        std::fs::write(dir.join(format!("file{n}.txt")), b"x").unwrap();
    }
    let mut f = Files::new(vec![dir.clone()]);
    f.view = View::Icons;
    let pane = &f.cols[0].tabs[0];
    assert_eq!(pane.entries.len(), 6);
    pane.width.set(440.0);
    pane.grid.set(icons::Grid::icons(440.0));
    f.update(Msg::Icon(0, 0, Pointer::Down(20.0, 20.0)));
    press(&f, Keysym::Shift_L, true);
    f.update(Msg::Icon(0, 1, Pointer::Down(130.0, 20.0)));
    press(&f, Keysym::Shift_L, false);
    assert_eq!(f.cols[0].tabs[0].picked, [0, 1, 2, 3, 4, 5]);
    press(&f, Keysym::Control_L, true);
    f.update(Msg::Icon(0, 0, Pointer::Down(240.0, 20.0)));
    press(&f, Keysym::Control_L, false);
    assert_eq!(f.cols[0].tabs[0].picked, [0, 1, 3, 4, 5]);
    f.update(Msg::Icon(0, 1, Pointer::Down(400.0, 60.0)));
    assert!(f.cols[0].tabs[0].picked.is_empty());
    f.update(Msg::Icon(0, 1, Pointer::Move(150.0, -50.0)));
    f.update(Msg::Icon(0, 1, Pointer::Up(150.0, -50.0)));
    assert_eq!(f.cols[0].tabs[0].picked, [1, 2, 3, 5]);
    f.update(Msg::Step(Step::Up, false));
    f.update(Msg::Step(Step::Down, false));
    assert_eq!(f.cols[0].tabs[0].chosen, Some(4));
    f.update(Msg::Step(Step::Right, true));
    assert_eq!(f.cols[0].tabs[0].picked, [4, 5]);
    press(&f, Keysym::Control_L, true);
    let all = { let m = f.mods.get(); f.intercept(&Input::Key { sym: Keysym::a, text: None, mods: m, repeat: false }, false) };
    f.update(all.unwrap());
    assert_eq!(f.cols[0].tabs[0].picked.len(), 6);
    let typing = Keyboard { keys: false, text: true, arrows: true };
    let ctrl = Mods { ctrl: true, ..Mods::default() };
    let k = |sym, mods| Input::Key { sym, text: None, mods, repeat: false };
    assert!(f.intercept_with(&k(Keysym::a, ctrl), typing).is_none());
    assert!(f.intercept_with(&k(Keysym::BackSpace, Mods::default()), typing).is_none());
    assert!(f.intercept_with(&k(Keysym::Delete, Mods::default()), typing).is_none());
    assert!(matches!(f.intercept_with(&k(Keysym::t, ctrl), typing), Some(Msg::NewTab(None))));
    let list = Keyboard { keys: false, text: false, arrows: true };
    assert!(f.intercept_with(&k(Keysym::Down, Mods::default()), list).is_none());
    assert!(matches!(f.intercept_with(&k(Keysym::BackSpace, Mods::default()), list), Some(Msg::Up)));
    assert!(matches!(f.intercept_with(&k(Keysym::Down, Mods::default()), Keyboard::default()), Some(Msg::Step(Step::Down, false))));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn picking_in_one_pane_lets_go_of_the_other() {
    let (dir, mut f) = two_panes("one-pane-picks");
    let ctrl = Mods { ctrl: true, ..Mods::default() };
    let k = |sym, mods| Input::Key { sym, text: None, mods, repeat: false };
    for (from, to) in [(0, 1), (1, 0)] {
        f.update(Msg::Pick(from, vec![0, 1]));
        assert_eq!(f.cols[from].picked, [0, 1]);
        f.update(Msg::Pick(to, vec![2]));
        assert_eq!((f.cols[to].picked.clone(), f.cols[from].picked.is_empty()), (vec![2], true), "a pick in pane {to}");
        f.update(Msg::Pick(from, vec![0, 1]));
        f.side = to;
        let _ = f.view();
        f.update(f.intercept_with(&k(Keysym::a, ctrl), Keyboard::default()).unwrap());
        assert!(!f.cols[to].picked.is_empty() && f.cols[from].picked.is_empty(), "Select all in pane {to}");
        f.update(Msg::Pick(to, Vec::new()));
        f.update(Msg::Pick(from, vec![1]));
        f.update(Msg::Choose(to, 0));
        assert_eq!(f.cols[from].picked, [1], "only choosing in pane {to}");
        f.update(Msg::Pick(from, Vec::new()));
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn select_none_and_invert_selection_work_in_every_view_and_both_panes() {
    let (dir, mut f) = two_panes("invert");
    let ctrl = Mods { ctrl: true, ..Mods::default() };
    let both = Mods { ctrl: true, shift: true, ..Mods::default() };
    let k = |sym, mods| Input::Key { sym, text: None, mods, repeat: false };
    for view in VIEWS {
        f.view = view;
        for p in 0..2 {
            let what = format!("{view:?}, pane {p}");
            f.side = p;
            let _ = f.view();
            f.update(f.intercept_with(&k(Keysym::a, ctrl), Keyboard::default()).unwrap());
            assert_eq!(f.cols[p].picked.len(), 5, "{what}");
            f.update(f.intercept_with(&k(Keysym::A, both), Keyboard::default()).unwrap());
            assert!(f.cols[p].picked.is_empty(), "{what}");
            f.update(Msg::Pick(p, vec![1, 3]));
            f.update(f.intercept_with(&k(Keysym::i, ctrl), Keyboard::default()).unwrap());
            assert_eq!(f.cols[p].picked, [0, 2, 4], "{what}");
            let typing = Keyboard { keys: false, text: true, arrows: true };
            assert!(f.intercept_with(&k(Keysym::A, both), typing).is_none(), "{what}");
            f.update(Msg::Pick(p, vec![0, 1]));
            let _ = f.view();
            let grid = f.cols[p].grid.get();
            let (foot, x) = match grid.across() {
                true => (0, grid.breadth() + 30.0),
                false => (f.cols[p].entries.len().div_ceil(grid.cols), 30.0),
            };
            f.update(Msg::Icon(p, foot, Pointer::Menu(x, 6.0)));
            let acts: Vec<Act> = crate::files::menu::every(&f.menu.as_ref().expect("a menu on empty room").lines).iter().map(|l| l.act).collect();
            let at = acts.iter().position(|a| *a == Act::SelectAll).unwrap();
            assert_eq!(acts[at..at + 3], [Act::SelectAll, Act::SelectNone, Act::Invert], "{what}");
            assert!(f.menu_pick(Act::Invert), "{what}");
            assert_eq!(f.cols[p].picked, [2, 3, 4], "{what}");
            f.update(Msg::Icon(p, foot, Pointer::Menu(x, 6.0)));
            assert!(f.menu_pick(Act::SelectNone), "{what}");
            assert!(f.cols[p].picked.is_empty() && f.menu.is_none(), "{what}");
        }
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn invert_in_the_menu_turns_round_the_same_things_after_the_list_is_read_again() {
    let (dir, mut f) = two_panes("invert-read-again");
    f.view = VIEWS[0];
    f.side = 0;
    let _ = f.view();
    f.update(Msg::Pick(0, vec![0, 1]));
    let was: Vec<String> = f.cols[0].picked.iter().map(|&i| f.cols[0].entries[i].name.clone()).collect();
    assert_eq!(was, ["apple.txt", "Banana.txt"]);
    let _ = f.view();
    let foot = f.cols[0].entries.len().div_ceil(f.cols[0].grid.get().cols);
    f.update(Msg::Icon(0, foot, Pointer::Menu(30.0, 6.0)));
    assert!(f.menu.is_some() && f.cols[0].picked.is_empty());
    std::fs::write(dir.join("aardvark.txt"), b"x").unwrap();
    f.reload_all();
    assert!(f.menu.is_some(), "the menu is still open");
    assert!(f.menu_pick(Act::Invert));
    let now: Vec<String> = f.cols[0].picked.iter().map(|&i| f.cols[0].entries[i].name.clone()).collect();
    assert_eq!(now, ["aardvark.txt", "cherry.txt", "damson.txt", "elder.txt"], "everything but the two that were picked");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_tick_box_adds_or_takes_away_one_thing_and_leaves_the_rest_in_every_view() {
    let (dir, mut f) = two_panes("ticks");
    f.update(Msg::NewTab(Some(0)));
    for view in VIEWS {
        f.view = view;
        for p in 0..2 {
            let what = format!("{view:?}, pane {p}");
            f.update(Msg::Pick(p, Vec::new()));
            f.ticks = true;
            let _ = f.view();
            let (grid, width) = (f.cols[p].grid.get(), f.cols[p].width.get());
            assert!(grid.ticks, "{what}");
            let tick = |i: usize| {
                let r = grid.tick_rect(i, width);
                Pointer::Down(r.x + r.w / 2.0, r.y + r.h / 2.0)
            };
            f.update(Msg::Icon(p, 0, tick(1)));
            f.update(Msg::Icon(p, 0, tick(3)));
            f.update(Msg::Icon(p, 0, tick(0)));
            assert_eq!(f.cols[p].picked, [0, 1, 3], "{what}");
            f.update(Msg::Icon(p, 0, tick(1)));
            assert_eq!(f.cols[p].picked, [0, 3], "{what}");
            assert!(f.drag.is_none(), "{what}");
            let at = f.cols[p].at.clone();
            f.update(Msg::Icon(p, 0, tick(2)));
            f.update(Msg::Icon(p, 0, tick(2)));
            assert_eq!((f.cols[p].picked.clone(), &f.cols[p].at), (vec![0, 3], &at), "{what}");
            let c = grid.cell(4, width);
            f.update(Msg::Icon(p, 0, Pointer::Down(c.x + c.w - 30.0, c.y + c.h - 6.0)));
            f.update(Msg::Icon(p, 0, Pointer::Up(c.x + c.w - 30.0, c.y + c.h - 6.0)));
            assert_eq!(f.cols[p].picked, [4], "{what}");
            f.ticks = false;
            let _ = f.view();
            assert!(!f.cols[p].grid.get().ticks, "{what}");
            f.update(Msg::Icon(p, 0, tick(1)));
            f.update(Msg::Icon(p, 0, Pointer::Up(0.0, 0.0)));
            assert_eq!(f.cols[p].picked, [1], "{what}");
        }
    }
    assert!(!f.touch.get());
    f.intercept_with(&Input::TouchDown { id: 1, x: 300.0, y: 300.0 }, Keyboard::default());
    assert!(f.touch.get());
    f.intercept_with(&Input::Press { button: Button::Left, x: 300.0, y: 300.0 }, Keyboard::default());
    assert!(!f.touch.get());
    std::fs::remove_dir_all(&dir).unwrap();
}

fn every_kind_of_group_covers_everything(f: &mut Files, view: View, p: usize, n: usize) {
    for (i, by) in GroupBy::ALL.into_iter().enumerate().skip(1) {
        let what = format!("{view:?}, pane {p}, {by:?}");
        f.update(Msg::GroupBy(i));
        let _ = f.view();
        let pane = &f.cols[p];
        let now = crate::common::clock::Local::now();
        assert_eq!(pane.groups.len() > 1, by != GroupBy::Date, "{what}: everything here was made today");
        let mut next = 0;
        for (start, len, words) in &pane.groups {
            assert_eq!(*start, next, "{what}");
            let name = rank(&pane.entries[*start], by, now).1;
            assert_eq!(words, &format!("{name}, {len}"), "{what}");
            assert!(pane.entries[*start..start + len].iter().all(|e| rank(e, by, now).1 == name), "{what}");
            next += len;
        }
        assert_eq!(next, n, "{what}");
        assert_eq!(pane.groups[0].2, if by == GroupBy::Date { format!("Today, {n}") } else { "Folders, 1".to_string() }, "{what}");
    }
}

fn groups_by_size_keep_the_sort_inside_each(f: &mut Files, p: usize, what: &str) {
    assert_eq!(names(f, p), ["Albums", "apple.txt", "Banana.txt", "cat.png", "cherry.txt", "damson.txt", "elder.txt", "song.mp3", "empty.txt"], "{what}");
    let heads: Vec<&str> = f.cols[p].groups.iter().map(|g| g.2.as_str()).collect();
    assert_eq!(heads, ["Folders, 1", "Small (under 1 MB), 7", "Empty, 1"], "{what}");
    f.update(Msg::Flip);
    assert_eq!(names(f, p), ["Albums", "song.mp3", "elder.txt", "damson.txt", "cherry.txt", "cat.png", "Banana.txt", "apple.txt", "empty.txt"], "{what}: turned round inside each group");
    f.update(Msg::Flip);
}

fn a_heading_is_no_row(f: &mut Files, p: usize, what: &str, n: usize) {
    let _ = f.view();
    let (grid, width) = (f.cols[p].grid.get(), f.cols[p].width.get());
    let first = if grid.across() { grid.heads_across().first().map(|h| h.2) } else { grid.head_at(0) };
    assert_eq!(first, Some("Folders, 1"), "{what}");
    f.update(Msg::Pick(p, Vec::new()));
    f.update(Msg::Icon(p, 0, Pointer::Down(30.0, 6.0)));
    f.update(Msg::Icon(p, 0, Pointer::Up(30.0, 6.0)));
    assert!(f.cols[p].picked.is_empty() && f.cols[p].chosen.is_none(), "{what}");
    let mut seen = Vec::new();
    f.update(Msg::Step(Step::First, false));
    for _ in 0..n + 2 {
        seen.push(f.cols[p].chosen.unwrap());
        f.update(Msg::Step(Step::Right, false));
    }
    assert_eq!(seen[..n], (0..n).collect::<Vec<_>>()[..], "{what}");
    assert_eq!(f.cols[p].chosen, Some(n - 1), "{what}");
    f.update(Msg::Step(Step::First, false));
    let mut downs = 0;
    while f.cols[p].chosen != Some(n - 1) && downs < 2 * n {
        let was = f.cols[p].chosen;
        f.update(Msg::Step(Step::Down, false));
        assert!(f.cols[p].chosen > was || f.cols[p].chosen == Some(n - 1), "{what}: down goes on");
        downs += 1;
    }
    assert_eq!(f.cols[p].chosen, Some(n - 1), "{what}: down reaches the last thing");
    f.update(Msg::Icon(p, 0, Pointer::Down(width.max(grid.breadth()) - 2.0, grid.row_top(grid.rows(n)) + 4.0)));
    f.update(Msg::Icon(p, 0, Pointer::Move(1.0, 2.0)));
    f.update(Msg::Icon(p, 0, Pointer::Up(1.0, 2.0)));
    assert_eq!(f.cols[p].picked, (0..n).collect::<Vec<_>>(), "{what}: a box over everything");
}

#[test]
fn grouping_puts_things_under_headings_in_every_view_and_pane() {
    let (dir, mut f) = two_panes("groups");
    std::fs::create_dir_all(dir.join("Albums")).unwrap();
    std::fs::write(dir.join("cat.png"), b"made up").unwrap();
    std::fs::write(dir.join("song.mp3"), b"made up").unwrap();
    std::fs::write(dir.join("empty.txt"), b"").unwrap();
    f.update(Msg::NewTab(Some(0)));
    f.reload_all();
    let plain = names(&f, 0);
    let n = plain.len();
    assert_eq!(n, 9);
    for view in VIEWS {
        f.view = view;
        for p in 0..2 {
            f.side = p;
            every_kind_of_group_covers_everything(&mut f, view, p, n);
            let what = format!("{view:?}, pane {p}");
            groups_by_size_keep_the_sort_inside_each(&mut f, p, &what);
            a_heading_is_no_row(&mut f, p, &what, n);
            f.update(Msg::Unpick);
            f.update(Msg::All);
            assert_eq!(f.cols[p].picked.len(), n, "{what}");
            assert!(crate::files::view::status_words(&f, &f.cols[p], p, false).starts_with("9 selected"), "{what}");
            assert_eq!(f.cols[1 - p].groups, f.cols[p].groups, "{what}");
            assert_eq!(f.cols[0].tabs[0].groups, f.cols[p].groups, "{what}");
            f.update(Msg::GroupBy(0));
            let _ = f.view();
            assert!(f.cols[p].groups.is_empty() && f.cols[p].grid.get().head_at(0).is_none(), "{what}");
            assert_eq!(names(&f, p), plain, "{what}");
        }
    }
    f.update(Msg::GroupBy(1));
    assert_eq!(f.kept.group(&Where::Recent), GroupBy::None);
    assert_eq!(f.kept.group(&Where::Folder(dir.clone())), GroupBy::Kind);
    std::fs::remove_dir_all(&dir).unwrap();
}
