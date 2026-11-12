//! The size of the icons: the keys, the wheel, and what the pointer is over staying where it is.

use crate::files::icons::{ICON_SIZES, ICON_USUAL};
use crate::files::zoom::Zoom;
use crate::toolkit::theme::Theme;

use super::*;

fn many_icons(name: &str, dual: bool) -> (PathBuf, Files) {
    let dir = scratch(name);
    std::fs::create_dir_all(dir.join("many")).unwrap();
    for i in 0..120 {
        std::fs::write(dir.join(format!("many/p{i:03}.txt")), b"x").unwrap();
    }
    let mut kept = crate::files::kept::Kept::from(Some(dir.join("conf")));
    kept.dual = dual;
    let at = Where::Folder(dir.join("many"));
    let mut f = Files::with(Vec::new(), dir.clone(), (at.clone(), at), kept, Vec::new());
    f.view = View::Icons;
    (dir, f)
}

const CTRL: Mods = Mods { ctrl: true, shift: false, alt: false, logo: false };

fn ctrl_key(f: &Files, sym: Keysym) -> Msg {
    f.intercept_with(&Input::Key { sym, text: None, mods: CTRL, repeat: false }, Keyboard::default()).unwrap()
}

fn wheel(f: &Files, y: f32, steps: bool) -> Option<Msg> {
    f.intercept_with(&Input::Scroll { x: 0.0, y, steps }, Keyboard::default())
}

#[test]
fn the_icons_size_steps_by_keys_and_by_ctrl_and_the_wheel_and_is_kept() {
    let (dir, mut f) = many_icons("zoom", true);
    let _ = f.view();
    assert_eq!((f.kept.icon(), f.cols[0].grid.get().cols), (ICON_USUAL, 4));
    sizes_step_by_keys(&mut f, &dir);
    the_wheel_changes_the_size(&mut f);
    f.view = View::Icons;
    the_thing_under_the_pointer_keeps_its_place(&mut f);
    std::fs::remove_dir_all(&dir).unwrap();
}

fn sizes_step_by_keys(f: &mut Files, dir: &Path) {
    f.update(ctrl_key(f, Keysym::plus));
    f.update(ctrl_key(f, Keysym::equal));
    f.update(ctrl_key(f, Keysym::KP_Add));
    assert_eq!(f.kept.icon(), 128.0);
    let _ = f.view();
    for p in 0..2 {
        let g = f.cols[p].grid.get();
        assert_eq!((g.icon, g.cols, g.h), (128.0, 2, 128.0 + f.room_now().icon_room.1), "pane {p}: both show the same size");
    }
    assert_eq!(crate::files::kept::Kept::from(Some(dir.join("conf"))).icon(), 128.0, "kept");
    for _ in 0..5 {
        f.update(ctrl_key(f, Keysym::plus));
    }
    assert_eq!(f.kept.icon(), *ICON_SIZES.last().unwrap());
    f.update(ctrl_key(f, Keysym::minus));
    assert_eq!(f.kept.icon(), 192.0);
    for _ in 0..9 {
        f.update(ctrl_key(f, Keysym::KP_Subtract));
    }
    assert_eq!(f.kept.icon(), ICON_SIZES[0]);
    f.update(ctrl_key(f, Keysym::_0));
    assert_eq!(f.kept.icon(), ICON_USUAL);
}

fn the_wheel_changes_the_size(f: &mut Files) {
    f.intercept(&Input::Mods(CTRL), false);
    f.pointer.set((crate::files::act::panes_left() + 50.0, 300.0));
    assert!(matches!(wheel(f, -1.0, true), Some(Msg::Zoom(Zoom::In, Some(0)))));
    assert!(matches!(wheel(f, 1.0, true), Some(Msg::Zoom(Zoom::Out, Some(0)))));
    for _ in 0..3 {
        assert!(matches!(wheel(f, 12.0, false), Some(Msg::Resized)), "taken, so the list does not scroll, but not yet a step");
    }
    assert!(matches!(wheel(f, 12.0, false), Some(Msg::Zoom(Zoom::Out, Some(0)))));
    assert!(matches!(wheel(f, 12.0, false), Some(Msg::Resized)), "and counting starts again");
    f.intercept(&Input::Mods(Mods::default()), false);
    assert!(wheel(f, -1.0, true).is_none());
    for view in [View::List, View::Compact] {
        f.view = view;
        f.update(Msg::Zoom(Zoom::In, None));
        assert_eq!(f.kept.icon(), ICON_USUAL, "{view:?}");
        f.intercept(&Input::Mods(CTRL), false);
        assert!(wheel(f, -1.0, true).is_none(), "{view:?}");
        f.intercept(&Input::Mods(Mods::default()), false);
    }
}

fn the_thing_under_the_pointer_keeps_its_place(f: &mut Files) {
    for p in 0..2 {
        let _ = f.view();
        let (g, w) = (f.cols[p].grid.get(), f.cols[p].width.get());
        let c = g.cell(41, w);
        f.update(Msg::Icon(p, 0, Pointer::Hover(c.x + c.w / 2.0, c.y + c.h * 0.25)));
        f.update(Msg::Zoom(Zoom::In, Some(p)));
        let _ = f.view();
        let (g2, share) = (f.cols[p].grid.get(), (c.y + c.h * 0.25 - g.row_top(g.place(41).0)) / g.h);
        let by = (g2.row_top(g2.place(41).0) + share * g2.h) - (c.y + c.h * 0.25);
        assert!((f.cols[p].nudge.1 - by).abs() < 0.01 && by > 0.0, "pane {p}: {} for {by}", f.cols[p].nudge.1);
        let mut told = Vec::new();
        f.view().nudges(0, &mut Vec::new(), &mut told);
        assert!(told.iter().any(|(turn, by, _)| (*turn, *by) == f.cols[p].nudge), "pane {p}: {told:?}");
        f.update(Msg::Choose(p, 77));
        f.side = p;
        let before = f.cols[p].grid.get().cell(77, w).y;
        f.update(Msg::Zoom(Zoom::Usual, None));
        let _ = f.view();
        assert!((f.cols[p].nudge.1 - (f.cols[p].grid.get().cell(77, w).y - before)).abs() < 0.01, "pane {p}");
    }
}

#[test]
fn on_a_painted_window_the_row_under_the_pointer_stays_under_it_as_the_icons_change_size() {
    fn row_under(f: &Files) -> (usize, usize) {
        let pane = &f.cols[0];
        let (g, n) = (pane.grid.get(), pane.entries.len());
        let (_, y) = pane.pointed.expect("the pointer is over the items");
        let row = (0..g.rows(n)).find(|r| y >= g.row_top(*r) && y < g.row_top(*r) + g.h).expect("the pointer is in a row");
        let first = (0..n).find(|i| g.place(*i) == (row, 0)).unwrap();
        (row, first)
    }
    let ctrl = Mods { ctrl: true, ..Mods::default() };
    for at in [384.0, 391.0, 420.0, 482.0, 500.0, 567.0] {
        let (dir, mut f) = many_icons(&format!("zoomed{at}"), false);
        let mut ui = Ui::new(f.view(), Theme::new(false, "#fbc711"));
        show(&f, &mut ui);
        send(&mut f, &mut ui, Input::PointerMove { x: 700.0, y: at });
        let items_top = at - f.cols[0].pointed.expect("the pointer is over the items").1;
        send(&mut f, &mut ui, Input::Scroll { x: 0.0, y: 4.0, steps: true });
        send(&mut f, &mut ui, Input::PointerMove { x: 700.0, y: at + 1.0 });
        for (y, size) in [(-1.0, 64.0), (-1.0, 96.0), (1.0, 64.0), (1.0, 48.0), (1.0, 32.0)] {
            let (_, first) = row_under(&f);
            let held = f.cols[0].hover.unwrap_or(first);
            send(&mut f, &mut ui, Input::Mods(ctrl));
            send(&mut f, &mut ui, Input::Scroll { x: 0.0, y, steps: true });
            send(&mut f, &mut ui, Input::Mods(Mods::default()));
            assert_eq!(f.kept.icon(), size);
            send(&mut f, &mut ui, Input::PointerMove { x: 700.0, y: at + 0.5 });
            send(&mut f, &mut ui, Input::PointerMove { x: 700.0, y: at + 1.0 });
            let (row, _) = row_under(&f);
            let now = f.cols[0].grid.get().place(held).0;
            let at_top = f.cols[0].pointed.unwrap().1 - (at + 1.0 - items_top) < 0.5;
            assert!(now == row || at_top && now < row, "at {at}, size {size}: the row of the thing held, {now}, is the row under the pointer, {row}");
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
