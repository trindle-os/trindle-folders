//! A slow second click on a name renames it.

use super::*;

struct Minutes(Instant);

impl Minutes {
    fn next(&mut self) -> Instant {
        self.0 += Duration::from_secs(60);
        self.0
    }
}

fn slow_click_cases(f: &mut Files, dir: &Path, p: usize, what: &str) {
    let here = f.cols[p].at.clone();
    let mut clock = Minutes(Instant::now());
    slow_click_renames(f, dir, p, what, &mut clock);
    slow_click_opens(f, dir, p, what, &mut clock, &here);
    slow_click_misses(f, p, what, &mut clock);
    slow_click_picks(f, p, what, &mut clock);
    slow_click_holds(f, p, what, &mut clock);
    slow_click_in_recent(f, p, what, &mut clock, here);
}

fn slow_click_renames(f: &mut Files, dir: &Path, p: usize, what: &str, clock: &mut Minutes) {
    let t0 = clock.next();
    let name = choose(f, p, "report.pdf", t0);
    click(f, p, name, t0 + ms(900));
    let n = f.slow.as_ref().map(|s| s.n).unwrap_or_else(|| panic!("{what}: a slow click on the name waits to rename"));
    assert!(f.renaming.is_none(), "{what}: not before the double-click time has passed");
    f.slow_due(n, t0 + ms(900) + TWICE - ms(1));
    assert!(f.renaming.is_none(), "{what}: a moment too soon");
    f.slow_due(n, t0 + ms(900) + TWICE);
    assert_eq!(renaming(f).as_deref(), Some("report.pdf"), "{what}");
    assert_eq!(f.renaming.as_ref().unwrap().selected(), 0.."report".len(), "{what}");
    assert_eq!(f.side(), p);
    f.update(Msg::RenameStop);
    assert!(f.renaming.is_none() && side_dir(dir, p).join("report.pdf").is_file());
}

fn slow_click_opens(f: &mut Files, dir: &Path, p: usize, what: &str, clock: &mut Minutes, here: &Where) {
    let t0 = clock.next();
    let name = choose(f, p, "Albums", t0);
    click(f, p, name, t0 + TWICE - ms(1));
    assert_eq!(f.cols[p].at, Where::Folder(side_dir(dir, p).join("Albums")), "{what}: a double click opens");
    assert!(f.slow.is_none() && f.renaming.is_none(), "{what}: a double click starts no rename");
    f.go(p, here.clone());

    let t0 = clock.next();
    let name = choose(f, p, "Albums", t0);
    click(f, p, name, t0 + ms(900));
    let n = f.slow.as_ref().map(|s| s.n).expect("waiting");
    click(f, p, name, t0 + ms(1100));
    assert_eq!(f.cols[p].at, Where::Folder(side_dir(dir, p).join("Albums")), "{what}: slow then quick opens");
    assert!(f.slow.is_none());
    f.slow_due(n, t0 + ms(5000));
    assert!(f.renaming.is_none(), "{what}: the forgotten rename never starts");
    f.go(p, here.clone());
}

fn slow_click_misses(f: &mut Files, p: usize, what: &str, clock: &mut Minutes) {
    let t0 = clock.next();
    choose(f, p, "report.pdf", t0);
    let icon = on_icon(f, p, "report.pdf");
    click(f, p, icon, t0 + ms(900));
    assert!(f.slow.is_none(), "{what}: the icon is not the name");
    if f.cols[p].grid.get().lines {
        let c = f.cols[p].grid.get().cell(index(f, p, "report.pdf"), f.cols[p].width.get());
        let t0 = clock.next();
        choose(f, p, "report.pdf", t0);
        click(f, p, (c.x + c.w - 6.0, c.y + c.h / 2.0), t0 + ms(900));
        assert!(f.slow.is_none(), "{what}: the far end of the row is not the name");
    }

    let t0 = clock.next();
    let name = choose(f, p, "report.pdf", t0);
    click(f, p, name, t0 + SLOW_WITHIN + ms(1));
    assert!(f.slow.is_none(), "{what}: too late");
}

fn slow_click_picks(f: &mut Files, p: usize, what: &str, clock: &mut Minutes) {
    for mods in [Mods { ctrl: true, ..Mods::default() }, Mods { shift: true, ..Mods::default() },
                 Mods { alt: true, ..Mods::default() }, Mods { logo: true, ..Mods::default() }] {
        let t0 = clock.next();
        let name = choose(f, p, "report.pdf", t0);
        f.mods.set(mods);
        click(f, p, name, t0 + ms(900));
        f.mods.set(Mods::default());
        assert!(f.slow.is_none(), "{what}: {mods:?}");
    }

    let t0 = clock.next();
    let name = choose(f, p, "report.pdf", t0);
    let other = on_icon(f, p, "notes.txt");
    f.icon_pointer_at(p, 0, Pointer::Hover(other.0, other.1), t0 + ms(300));
    f.icon_pointer_at(p, 0, Pointer::Hover(name.0, name.1), t0 + ms(600));
    click(f, p, name, t0 + ms(900));
    assert!(f.slow.is_none(), "{what}: the pointer moved off");
    let t0 = clock.next();
    let name = choose(f, p, "report.pdf", t0);
    f.icon_pointer_at(p, 0, Pointer::Leave, t0 + ms(300));
    click(f, p, name, t0 + ms(900));
    assert!(f.slow.is_none(), "{what}: the pointer left the pane");

    let t0 = clock.next();
    let name = choose(f, p, "report.pdf", t0);
    let both = vec![index(f, p, "notes.txt"), index(f, p, "report.pdf")];
    f.update(Msg::Pick(p, both.clone()));
    click(f, p, name, t0 + ms(900));
    assert!(f.slow.is_none(), "{what}: several are picked");
}

fn slow_click_holds(f: &mut Files, p: usize, what: &str, clock: &mut Minutes) {
    let t0 = clock.next();
    let name = choose(f, p, "report.pdf", t0);
    down(f, p, name, t0 + ms(900));
    assert!(f.slow.is_some());
    let (wx, wy) = f.pointer.get();
    f.pointer.set((wx + 40.0, wy + 40.0));
    f.icon_pointer_at(p, 0, Pointer::Move(name.0 + 40.0, name.1 + 40.0), t0 + ms(950));
    assert!(f.slow.is_none() && f.drag.as_ref().is_some_and(|d| d.moving), "{what}: dragging");
    f.pointer.set((wx, wy));
    f.update(Msg::Escape);
    assert!(f.drag.is_none() && f.renaming.is_none());

    let t0 = clock.next();
    let name = choose(f, p, "report.pdf", t0);
    down(f, p, name, t0 + ms(900));
    let n = f.slow.as_ref().map(|s| s.n).expect("waiting");
    f.slow_due(n, t0 + ms(900) + TWICE);
    assert!(f.renaming.is_none(), "{what}: not while the button is down");
    up(f, p, name, t0 + ms(1500));
    assert_eq!(renaming(f).as_deref(), Some("report.pdf"), "{what}: on letting go");
    f.update(Msg::RenameStop);
}

fn slow_click_in_recent(f: &mut Files, p: usize, what: &str, clock: &mut Minutes, here: Where) {
    let t0 = clock.next();
    let name = choose(f, p, "report.pdf", t0);
    f.cols[p].at = Where::Recent;
    click(f, p, name, t0 + ms(900));
    assert!(f.slow.is_none(), "{what}: not in Recent");
    f.cols[p].at = here;
}

#[test]
fn a_slow_second_click_on_a_name_renames_and_a_double_click_opens_in_every_view() {
    for view in VIEWS {
        let dir = folder(&format!("slow-{view:?}"));
        let mut f = files(&dir, view);
        for p in [0, 1] {
            slow_click_cases(&mut f, &dir, p, &format!("{view:?}, pane {p}"));
            f.update(Msg::NewTab(Some(p)));
            assert_eq!(f.cols[p].tab, 1);
            slow_click_cases(&mut f, &dir, p, &format!("{view:?}, pane {p}, second tab"));
            f.update(Msg::Tab(p, 0));
        }
        fs::remove_dir_all(&dir).unwrap();
    }
}
