//! Middle clicks and double clicks that open tabs, and a middle click that closes one.

use crate::files::act::Land;

use super::*;

struct Case<'a> {
    dir: &'a Path,
    p: usize,
    what: String,
    before: usize,
    here: Where,
    now: Instant,
}

#[test]
fn a_middle_click_opens_a_tab_behind_and_closes_a_tab() {
    let (dir, mut f) = two_panes("middle");
    for d in ["Albums", "Letters"] {
        std::fs::create_dir_all(dir.join(d).join("inner")).unwrap();
    }
    f.pinned = vec![dir.join("Letters")];
    f.reload_all();
    let now = Instant::now();
    for view in VIEWS {
        f.view = view;
        for p in 0..2 {
            let case = Case { dir: &dir, p, what: format!("{view:?}, pane {p}"), before: f.cols[p].tabs.len(), here: f.cols[p].at.clone(), now };
            f.update(Msg::Pick(p, vec![2]));
            a_middle_click_on_an_item(&mut f, &case);
            a_middle_click_on_a_tab(&mut f, &case);
            ctrl_and_a_double_click(&mut f, &case);
        }
    }
    a_pinned_folder_a_place_and_a_name_in_the_path(&mut f, &dir);
    let _ = std::fs::remove_dir_all(dir);
}

fn middle_click(f: &mut Files, c: &Case, name: &str) {
    let (x, y) = item_centre(f, c.p, name);
    f.icon_pointer_at(c.p, 0, Pointer::Middle(x, y), c.now);
}

fn a_middle_click_on_an_item(f: &mut Files, c: &Case) {
    let (p, what) = (c.p, &c.what);
    middle_click(f, c, "Albums");
    assert_eq!(f.cols[p].tabs.len(), c.before + 1, "{what}");
    let tab = f.cols[p].tab;
    assert_eq!((&f.cols[p].at, &f.cols[p].tabs[tab + 1].at), (&c.here, &Where::Folder(c.dir.join("Albums"))), "{what}");
    assert_eq!(f.cols[p].tabs[tab + 1].entries.len(), 1, "{what}: the new tab is read");
    assert_eq!(f.cols[p].picked, [2], "{what}: what was picked stays");
    middle_click(f, c, "apple.txt");
    assert_eq!(f.cols[p].tabs.len(), c.before + 1, "{what}");
    f.mods.set(Mods { shift: true, ..Mods::default() });
    middle_click(f, c, "Letters");
    f.mods.set(Mods::default());
    assert_eq!((f.cols[p].tabs.len(), &f.cols[p].at), (c.before + 2, &Where::Folder(c.dir.join("Letters"))), "{what}");
}

fn a_middle_click_on_a_tab(f: &mut Files, c: &Case) {
    let (p, what) = (c.p, &c.what);
    let tab = f.cols[p].tab;
    f.update(Msg::Middle(Land::Tab(p, tab + 1)));
    assert_eq!((f.cols[p].tabs.len(), &f.cols[p].at), (c.before + 1, &Where::Folder(c.dir.join("Letters"))), "{what}");
    f.update(Msg::Middle(Land::Tab(p, tab - 1)));
    assert_eq!((f.cols[p].tabs.len(), &f.cols[p].at, f.cols[p].tab), (c.before, &Where::Folder(c.dir.join("Letters")), tab - 1), "{what}");
    f.go(p, c.here.clone());
}

fn ctrl_and_a_double_click(f: &mut Files, c: &Case) {
    let (p, what, now) = (c.p, &c.what, c.now);
    f.update(Msg::Pick(p, vec![2]));
    let (x, y) = item_centre(f, p, "Albums");
    f.mods.set(Mods { ctrl: true, ..Mods::default() });
    f.icon_pointer_at(p, 0, Pointer::Down(x, y), now);
    f.icon_pointer_at(p, 0, Pointer::Up(x, y), now);
    f.icon_pointer_at(p, 0, Pointer::Down(x, y), now + std::time::Duration::from_millis(120));
    f.icon_pointer_at(p, 0, Pointer::Up(x, y), now + std::time::Duration::from_millis(130));
    f.mods.set(Mods::default());
    assert_eq!((f.cols[p].tabs.len(), &f.cols[p].at), (c.before + 1, &c.here), "{what}");
    assert_eq!(f.cols[p].picked, [2], "{what}: picked as it began");
    assert!(f.drag.is_none(), "{what}");
    let behind = f.cols[p].tab + 1;
    assert_eq!(f.cols[p].tabs[behind].at, Where::Folder(c.dir.join("Albums")), "{what}");
    f.update(Msg::Middle(Land::Tab(p, behind)));
    if f.cols[p].tabs.len() == 1 {
        f.update(Msg::Middle(Land::Tab(p, 0)));
        assert_eq!(f.cols[p].tabs.len(), 1, "{what}");
    }
}

fn a_pinned_folder_a_place_and_a_name_in_the_path(f: &mut Files, dir: &Path) {
    f.side = 1;
    let before = f.cols[1].tabs.len();
    f.update(Msg::Middle(Land::Pinned(0)));
    let trash = f.place_list().iter().position(|w| *w == Where::Trash).unwrap();
    f.update(Msg::Middle(Land::Place(trash)));
    let tabs: Vec<&Where> = f.cols[1].tabs.iter().map(|t| &t.at).collect();
    assert_eq!(tabs.len(), before + 2);
    assert!(tabs.contains(&&Where::Folder(dir.join("Letters"))) && tabs.contains(&&Where::Trash), "{tabs:?}");
    assert_eq!(f.cols[1].tab, 0, "the tab showing stays");
    f.go(0, Where::Folder(dir.join("Albums/inner")));
    let parts = places::crumbs(&f.cols[0].at, &f.home);
    let i = parts.iter().position(|(_, at)| *at == Where::Folder(dir.join("Albums"))).unwrap();
    let before = f.cols[0].tabs.len();
    f.update(Msg::PartTab(0, i));
    let tab = f.cols[0].tab;
    assert_eq!((f.cols[0].tabs.len(), &f.cols[0].tabs[tab + 1].at), (before + 1, &Where::Folder(dir.join("Albums"))));
    assert_eq!(f.cols[0].at, Where::Folder(dir.join("Albums/inner")));
}
