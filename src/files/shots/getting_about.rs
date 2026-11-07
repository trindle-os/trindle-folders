// Pictures of getting about and of the windows beside the list.

use std::path::{Path, PathBuf};

use super::{index_of, nav_papers, pick, shoot, shoot_at, size, Live};
use crate::files::app::{Files, Msg, View};
use crate::files::dialogs::AskMsg;
use crate::files::menu::{Act, MenuMsg};
use crate::files::places::Where;
use crate::toolkit::input::{Button, Input, Keysym};
use crate::toolkit::ui::App;

#[test]
#[ignore]
fn shoot_nav() {
    let (dir, papers) = nav_papers("a");
    let mut f = Files::new(vec![papers.clone()]);
    f.room = size();
    f.home = dir.clone();
    f.update(Msg::PanesAre(false));
    f.pinned = vec![papers.join("Letters"), papers.join("Recipes")];
    a_middle_click_opens_a_tab_behind(&mut f);
    find_with_its_three_choices(&mut f, &papers);
    the_path_bars_arrows(&mut f, &papers);
    pin_to_the_ring(&mut f, &dir);
    with_no_other_part_of_trindle(&mut f, &papers);
    the_sidebar_by_dragging(&mut f);
    std::fs::remove_dir_all(&dir).unwrap();
}

fn a_middle_click_opens_a_tab_behind(f: &mut Files) {
    use crate::toolkit::widget::Pointer;
    let _ = crate::files::view::view(f);
    let pane = &f.cols[0];
    let c = pane.grid.get().cell(index_of(f, 0, "Albums"), pane.width.get());
    f.update(Msg::Icon(0, 0, Pointer::Middle(c.x + 40.0, c.y + c.h / 2.0)));
    shoot("nav-tab-behind", f);
    f.update(Msg::Middle(crate::files::act::Land::Tab(0, 1)));
}

fn find_with_its_three_choices(f: &mut Files, papers: &Path) {
    use crate::files::found::FindMsg;
    f.update(Msg::Find(FindMsg::Open(Some(0))));
    shoot("nav-find-choices", f);
    f.update(Msg::Find(FindMsg::Kind(2)));
    shoot("nav-find-pictures", f);
    f.update(Msg::Find(FindMsg::Kind(1)));
    f.update(Msg::Find(FindMsg::Changed(3)));
    f.update(Msg::Find(FindMsg::Search(false)));
    f.update(Msg::Find(FindMsg::Open(Some(0))));
    shoot("nav-find-folders-below", f);
    f.update(Msg::Find(FindMsg::Close));
    f.go(0, Where::Folder(papers.to_path_buf()));
    f.update(Msg::PanesAre(true));
    f.go(1, Where::Folder(papers.join("Albums")));
    f.update(Msg::Find(FindMsg::Open(Some(0))));
    f.update(Msg::Find(FindMsg::Size(1)));
    shoot("nav-find-two-panes", f);
    f.update(Msg::Find(FindMsg::Close));
    f.update(Msg::PanesAre(false));
}

fn the_path_bars_arrows(f: &mut Files, papers: &Path) {
    f.go(0, Where::Folder(papers.join("Albums/2025")));
    let mut live = Live::new(f);
    let key = |sym| Input::Key { sym, text: None, mods: Default::default(), repeat: false };
    let pills = live.ui.pill_boxes("path0");
    let (x, y) = (pills[1].x + pills[1].w - 6.0, pills[1].y + pills[1].h / 2.0);
    live.send(f, Input::PointerMove { x, y });
    live.send(f, Input::Press { button: Button::Left, x, y });
    live.send(f, Input::Release { button: Button::Left, x, y });
    live.snap("nav-path-list", f);
    live.send(f, key(Keysym::Escape));
    live.ui.keyboard_to("path0");
    live.send(f, key(Keysym::Left));
    live.send(f, key(Keysym::Left));
    live.snap("nav-path-keys", f);
    live.send(f, key(Keysym::Down));
    live.snap("nav-path-keys-open", f);
    live.send(f, key(Keysym::Escape));
    live.send(f, key(Keysym::End));
    live.send(f, key(Keysym::Down));
    live.snap("nav-path-last", f);
    live.send(f, key(Keysym::Escape));
    f.update(Msg::PanesAre(true));
    f.go(1, Where::Folder(papers.join("Albums/2025/Summer")));
    f.side = 1;
    let mut live = Live::new(f);
    let pills = live.ui.pill_boxes("path1");
    let at = pills.len() - 2;
    let (x, y) = (pills[at].x + pills[at].w - 6.0, pills[at].y + pills[at].h / 2.0);
    live.send(f, Input::PointerMove { x, y });
    live.send(f, Input::Press { button: Button::Left, x, y });
    live.send(f, Input::Release { button: Button::Left, x, y });
    live.snap("nav-path-two-panes", f);
    live.send(f, key(Keysym::Escape));
    f.update(Msg::PanesAre(false));
    f.go(0, Where::Folder(papers.to_path_buf()));
}

fn pin_to_the_ring(f: &mut Files, dir: &Path) {
    f.ring = Some(dir.join("ring-favourites"));
    pick(f, 0, "Albums");
    f.pointer.set((520.0, 150.0));
    f.update(Msg::Menu(MenuMsg::Open));
    f.menu_show(Act::RingPin);
    shoot("nav-ring-menu-folder", f);
    assert!(f.menu_pick(Act::RingPin));
    shoot("nav-ring-pinned-said", f);
    f.update(Msg::Menu(MenuMsg::Open));
    shoot("nav-ring-menu-unpin", f);
    f.menu = None;
    pick(f, 0, "report.pdf");
    f.pointer.set((520.0, 60.0));
    f.update(Msg::Menu(MenuMsg::Open));
    shoot("nav-ring-menu-file", f);
    f.menu = None;
    f.update(Msg::Unpick);
    f.ring = None;
}

fn with_no_other_part_of_trindle(f: &mut Files, papers: &Path) {
    f.parts = crate::files::alone::Parts::none();
    pick(f, 0, "Albums");
    f.pointer.set((520.0, 150.0));
    f.update(Msg::Menu(MenuMsg::Open));
    shoot("nav-alone-menu-folder", f);
    f.menu = None;
    f.ask(Act::Properties, 0, vec![papers.join("Albums")]);
    f.update(Msg::Ask(AskMsg::Page(1)));
    shoot("nav-alone-share", f);
    f.dialog = None;
    f.update(Msg::Unpick);
    f.parts = crate::files::alone::Parts::all();
}

fn the_sidebar_by_dragging(f: &mut Files) {
    let mut live = Live::new(f);
    let (x, y) = live.item(f, 0, "Albums");
    let pins = live.place("pinned");
    let row = pins.h / f.pinned.len() as f32;
    live.send(f, Input::PointerMove { x, y });
    live.send(f, Input::Press { button: Button::Left, x, y });
    live.send(f, Input::PointerMove { x: x - 80.0, y: y - 30.0 });
    live.send(f, Input::PointerMove { x: pins.x + 70.0, y: pins.y + row + 2.0 });
    live.snap("nav-pin-line", f);
    live.send(f, Input::Release { button: Button::Left, x: pins.x + 70.0, y: pins.y + row + 2.0 });
    live.snap("nav-pin-dropped", f);
    let pins = live.place("pinned");
    live.send(f, Input::PointerMove { x: pins.x + 70.0, y: pins.y + row * 2.5 });
    live.send(f, Input::Press { button: Button::Left, x: pins.x + 70.0, y: pins.y + row * 2.5 });
    live.send(f, Input::PointerMove { x: pins.x + 70.0, y: pins.y + 3.0 });
    live.snap("nav-pin-reorder-line", f);
    live.send(f, Input::Release { button: Button::Left, x: pins.x + 70.0, y: pins.y + 3.0 });
    live.snap("nav-pin-reordered", f);
    f.update(Msg::Menu(MenuMsg::Side { pinned: true, row: 1, x: pins.x + 70.0, y: pins.y + row * 1.5 }));
    shoot("nav-pin-menu", f);
    f.menu = None;
}

fn panels_papers() -> (PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-panels-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("HOME", &dir);
    let papers = dir.join("Papers");
    for sub in ["Albums", "Letters/Old"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for n in 1..=12 {
        std::fs::write(papers.join("Letters").join(format!("letter {n:02}.txt")), vec![b'x'; 2_400]).unwrap();
    }
    std::fs::write(papers.join("Letters/Old/first.txt"), vec![b'x'; 9_000]).unwrap();
    let notes = "Things to take\n\nTent, pegs and the mallet\nTwo sleeping bags\nThe small stove and a full gas bottle\nMatches\nTea, milk, bread, eggs\n\n\
                 Ring John Smith on 07700 900123 about the keys.\nWrite to jane@example.com with the dates.\n\nLeave by nine on Saturday.\n";
    std::fs::write(papers.join("notes.txt"), notes.repeat(3)).unwrap();
    std::fs::write(papers.join("budget.ods"), b"PK\x03\x04 made up").unwrap();
    std::fs::write(papers.join("report.pdf"), crate::files::walled::tests::a4_pdf("0.2 0.4 0.8 rg 60 600 475 180 re f 0.85 g 60 120 475 440 re f")).unwrap();
    image::RgbImage::from_fn(640, 400, |x, y| image::Rgb(if (x / 80 + y / 80) % 2 == 0 { [222, 184, 120] } else { [111, 92, 60] }))
        .save(papers.join("beach.png")).unwrap();
    image::RgbImage::from_fn(300, 900, |_, y| image::Rgb([40, (y / 4) as u8, 160])).save(papers.join("tall.png")).unwrap();
    (dir, papers)
}

#[test]
#[ignore]
fn shoot_panels() {
    let (dir, papers) = panels_papers();
    let mut f = Files::new(vec![papers.clone()]);
    f.room = size();
    f.update(Msg::PanesAre(false));
    f.pictures.inline = Some(crate::files::thumbs::Thumbs::at(&dir.join("thumbnails")));
    the_preview_panel(&mut f, &papers);
    storage(&mut f);
    the_top_of_the_disk(&mut f);
    std::fs::remove_dir_all(&dir).unwrap();
}

fn choose(f: &mut Files, names: &[&str]) {
    let picked: Vec<usize> = names.iter().map(|n| index_of(f, 0, n)).collect();
    f.update(Msg::Choose(0, picked[0]));
    f.update(Msg::Pick(0, picked));
}

fn the_preview_panel(f: &mut Files, papers: &Path) {
    use crate::files::panel::PanelMsg;
    shoot("panel-off", f);
    f.update(Msg::Panel(PanelMsg::Switch));
    shoot("panel-nothing-chosen", f);
    choose(f, &["beach.png"]);
    shoot("panel-picture", f);
    f.panel_waits();
    shoot("panel-reading", f);
    choose(f, &["tall.png"]);
    shoot("panel-picture-tall", f);
    choose(f, &["report.pdf"]);
    shoot("panel-pdf", f);
    choose(f, &["notes.txt"]);
    shoot("panel-text", f);
    choose(f, &["Letters"]);
    shoot("panel-folder", f);
    choose(f, &["beach.png", "notes.txt", "Letters"]);
    f.update(Msg::Changed);
    shoot("panel-several", f);
    f.update(Msg::PanesAre(true));
    f.go(1, Where::Folder(papers.join("Letters")));
    f.update(Msg::OtherPane);
    choose(f, &["beach.png"]);
    shoot("panel-dual", f);
    f.update(Msg::Look(View::Icons));
    shoot("panel-dual-icons", f);
    f.update(Msg::Look(View::List));
    f.update(Msg::PanesAre(false));
    f.kept.set_panel(true, Some(crate::files::panel::MOST));
    shoot("panel-widest", f);
    f.kept.set_panel(true, Some(300.0));
    let narrow = (crate::files::panel::least_window(false) - 20.0, 700.0);
    f.resized(narrow.0, narrow.1);
    shoot_at("panel-too-narrow", f, narrow);
    f.resized(size().0, size().1);
    f.update(Msg::Panel(PanelMsg::Switch));
}

fn storage(f: &mut Files) {
    use crate::common::volumes::{Kind, Volume};
    use crate::files::panel::PanelMsg;
    let gb = 1_000_000_000u64;
    let drive = |id: &str, label: &str, at: Option<&str>, kind: Kind, fixed: bool, size: u64| Volume {
        id: id.into(), dev: id.into(), label: label.into(), size: size * gb, drive: format!("/drive/{id}"), drive_name: "Acme".into(), kind,
        mounts: at.iter().map(|m| m.to_string()).collect(), locked: false, container: None, can_eject: !fixed, can_power_off: !fixed,
        fixed, system: fixed && at == Some("/"), key: String::new(), drive_key: String::new(),
    };
    let mut locked = drive("sdd1", "", None, Kind::Usb, false, 16);
    locked.locked = true;
    let listed = vec![
        drive("nvme0n1p3", "", Some("/"), Kind::Other, true, 256), drive("sda1", "Films", None, Kind::Other, true, 2000),
        drive("sdb1", "HOLIDAY", Some("/run/media/john/HOLIDAY"), Kind::Usb, false, 32),
        drive("mmc0p1", "PHOTOS", Some("/run/media/john/PHOTOS"), Kind::Card, false, 64), locked,
    ];
    f.computer.reader = Some(std::sync::Arc::new(move |at: &std::path::Path| match at.to_str() {
        Some("/") => Some((100 * gb, 156 * gb)),
        Some("/run/media/john/HOLIDAY") => Some((28 * gb, 4 * gb)),
        Some("/run/media/john/PHOTOS") => Some((62 * gb, 2 * gb)),
        Some("/run/media/john/Photos on nas") => Some((1_200 * gb, 2_800 * gb)),
        _ => None,
    }));
    f.update(Msg::Drive(crate::files::drives::DriveMsg::Listed(listed.clone())));
    f.far.phones = vec![crate::files::remote::Phone { id: "1-2".into(), name: "Acme Phone 7".into() }];
    f.shares.mounted = vec![crate::common::smb::Mounted { server: "nas".into(), share: "Photos".into(), at: "/run/media/john/Photos on nas".into() }];
    f.go(0, Where::Computer);
    shoot("panel-storage", f);
    let mut changed = listed.clone();
    changed.retain(|v| v.id != "sdb1");
    changed.insert(0, drive("sde1", "BACKUP", None, Kind::Usb, false, 500));
    f.update(Msg::Drive(crate::files::drives::DriveMsg::Listed(changed)));
    shoot("panel-storage-changed", f);
    f.update(Msg::Drive(crate::files::drives::DriveMsg::Listed(listed)));
    f.update(Msg::PanesAre(true));
    shoot("panel-storage-dual", f);
    f.update(Msg::PanesAre(false));
    f.update(Msg::Panel(PanelMsg::Switch));
    shoot("panel-storage-with-preview", f);
    f.update(Msg::Panel(PanelMsg::Switch));
}

fn the_top_of_the_disk(f: &mut Files) {
    let outside = std::env::temp_dir().join(format!("trindle-files-shoot-outside-{}", std::process::id()));
    std::fs::create_dir_all(outside.join("Shared/Maps")).unwrap();
    std::fs::write(outside.join("Shared/readme.txt"), b"made up").unwrap();
    f.go(0, Where::Folder(outside.join("Shared")));
    shoot("panel-storage-path", f);
    std::fs::remove_dir_all(&outside).unwrap();
}

fn choose_one(f: &mut Files, name: &str) {
    let i = index_of(f, 0, name);
    f.update(Msg::Choose(0, i));
    f.update(Msg::Pick(0, vec![i]));
}

#[test]
#[ignore]
fn shoot_earlier() {
    use crate::common::clock::Local;
    use crate::common::home_snapshots::tests::add_snapshot;
    use crate::common::home_snapshots::DESCRIPTION;
    use crate::files::earlier::tests::{write, World};
    use crate::files::earlier::{EarlierMsg, What};
    let w = World::new("shots");
    std::env::set_var("HOME", w.live(""));
    let now = Local::now().unix;
    let day = 86_400;
    let copy = |number: u32, ago: i64, files: &[(&str, &str, i64)]| {
        let date = Local::at(now - ago * day).format("%Y-%m-%d 03:00:00");
        let tree = add_snapshot(&w.root.join("snaps"), number, &date, DESCRIPTION).join("john");
        for (below, words, changed) in files {
            write(&tree.join(below), words, (now - changed) as u64);
        }
        tree
    };
    let long = "The quarter's figures, with the notes from the meeting.\n".repeat(40);
    write(&w.live("Documents/report.odt"), &long.repeat(3), (now - 3_600) as u64);
    write(&w.live("Documents/budget.ods"), "made up", (now - 9 * day) as u64);
    write(&w.live("Documents/Letters/to the council.odt"), "made up", (now - 5_000) as u64);
    copy(1, 5, &[("Documents/report.odt", "An outline.", 6 * day), ("Documents/Letters/first.odt", "x", 6 * day),
                 ("Documents/holiday list.txt", "Tent, pegs, mallet", 7 * day)]);
    copy(2, 3, &[("Documents/report.odt", &long, 3 * day + 7_000), ("Documents/Letters/first.odt", "x", 6 * day),
                 ("Documents/holiday list.txt", "Tent, pegs, mallet, stove", 4 * day), ("Documents/Photos/beach.png", "made up", 4 * day)]);
    copy(3, 2, &[("Documents/report.odt", &long, 3 * day + 7_000), ("Documents/Letters/first.odt", "x", 6 * day)]);
    let then = copy(4, 1, &[("Documents/report.odt", &long.repeat(2), day + 20_000), ("Documents/Letters/first.odt", "x", 6 * day),
                            ("Documents/Letters/second.odt", "xx", 2 * day)]);
    let mut f = w.files("Documents");
    f.room = size();
    choose_one(&mut f, "report.odt");
    f.pointer.set((520.0, 200.0));
    f.update(Msg::Menu(MenuMsg::Open));
    shoot("panel-earlier-menu", &f);
    f.menu = None;
    f.do_act(Act::Earlier, 0, vec![w.live("Documents/report.odt")]);
    shoot("panel-earlier-versions", &f);
    f.update(Msg::Earlier(EarlierMsg::Compare(1)));
    f.update(Msg::Earlier(EarlierMsg::PutBack(0)));
    shoot("panel-earlier-put-back", &f);
    f.update(Msg::Undo);
    f.do_act(Act::Earlier, 0, vec![w.live("Documents/Letters")]);
    shoot("panel-earlier-folder", &f);
    f.do_act(Act::Earlier, 0, vec![w.live("Documents/budget.ods")]);
    shoot("panel-earlier-none", &f);
    f.earlier_open(0, What::Deleted(w.live("Documents")));
    shoot("panel-earlier-deleted", &f);
    f.update(Msg::Earlier(EarlierMsg::PutBack(1)));
    shoot("panel-earlier-deleted-put-back", &f);
    that_days_folder_in_a_tab(&mut f, &w, &then);
}

fn that_days_folder_in_a_tab(f: &mut Files, w: &crate::files::earlier::tests::World, then: &Path) {
    use crate::files::earlier::EarlierMsg;
    f.do_act(Act::Earlier, 0, vec![w.live("Documents/Letters")]);
    f.update(Msg::Earlier(EarlierMsg::Open(0)));
    assert_eq!(f.cols[0].at, Where::Folder(then.join("Documents/Letters")));
    shoot("panel-earlier-tab", f);
    choose_one(f, "second.odt");
    f.update(Msg::Trash);
    shoot("panel-earlier-tab-refused", f);
    f.pointer.set((520.0, 260.0));
    f.update(Msg::Menu(MenuMsg::Open));
    shoot("panel-earlier-tab-menu", f);
}

fn as_it_was(lines: &[crate::files::menu::Line]) -> Vec<crate::files::menu::Line> {
    use crate::files::menu::Line;
    if lines.iter().all(|l| l.sub.is_empty()) {
        return lines.to_vec();
    }
    let rows: Vec<Line> = crate::files::menu::every(lines).into_iter().cloned().collect();
    let has = |act: Act| rows.iter().any(|l| l.act == act);
    let (with_pin, alone) = if has(Act::Pin) { (vec![Act::Pin, Act::RingPin], Vec::new()) } else { (Vec::new(), vec![Act::RingPin]) };
    let groups: Vec<Vec<Act>> = if has(Act::Trash) {
        let mut more = with_pin;
        more.extend([Act::Colour, Act::Terminal, Act::AsRoot, Act::ShareOnNetwork, Act::ShareWithAi, Act::Dupes, Act::Wallpaper, Act::TurnLeft, Act::TurnRight, Act::TurnOver, Act::Resize,
                     Act::InstallFont, Act::CompareFiles]);
        vec![vec![Act::NewFolder, Act::NewDocument], vec![Act::Open, Act::OpenTab, Act::OpenWindow, Act::OpenWith, Act::Preview],
             vec![Act::Cut, Act::Copy, Act::CopyPath, Act::Rename, Act::Link, Act::SendTo], vec![Act::Compress, Act::ExtractHere, Act::ExtractTo],
             vec![Act::Trash, Act::Delete, Act::Wipe], more, alone, vec![Act::Earlier, Act::Properties]]
    } else {
        vec![vec![Act::NewFolder, Act::NewDocument], vec![Act::Paste], vec![Act::SelectAll, Act::SelectNone, Act::Invert], vec![Act::Dupes],
             vec![Act::Terminal, Act::AsRoot], vec![Act::Pin, Act::RingPin, Act::Colour], vec![Act::DeletedHere, Act::Properties]]
    };
    let mut out: Vec<Line> = Vec::new();
    for group in &groups {
        let mut first = !out.is_empty();
        for act in group {
            out.extend(rows.iter().filter(|l| l.act == *act).map(|l| Line { rule: std::mem::take(&mut first), ..l.clone() }));
        }
    }
    assert_eq!(out.len(), rows.len(), "every row has its old place");
    out
}

#[test]
#[ignore]
fn shoot_menu_levels() {
    let (dir, papers) = nav_papers("menus");
    std::env::set_var("HOME", &dir);
    std::env::set_var("XDG_DATA_HOME", dir.join("share"));
    let bin = crate::files::trash::home_bin();
    bin.ready().unwrap();
    std::fs::write(bin.files.join("old notes.txt"), b"made up").unwrap();
    std::fs::write(bin.info.join("old notes.txt.trashinfo"), format!("[Trash Info]\nPath={}\nDeletionDate=2026-10-10T10:00:00\n", papers.join("old notes.txt").display())).unwrap();
    let mut f = Files::new(vec![papers.clone()]);
    f.room = size();
    f.home = dir.clone();
    f.update(Msg::PanesAre(false));
    f.ring = Some(dir.join("ring-favourites"));
    f.parts = crate::files::alone::Parts { wallpaper: true, as_root: true, helper: true, ai: true, terminal: Some(true) };
    let mut counts = String::from("case\tlines before\tfirst level after\tsub-menus\n");
    menu_levels(&mut f, "empty-room", &[], &mut counts);
    menu_levels(&mut f, "one-file", &["notes.txt"], &mut counts);
    menu_levels(&mut f, "a-folder", &["Albums"], &mut counts);
    menu_levels(&mut f, "an-archive", &["old letters.tar.gz"], &mut counts);
    menu_levels(&mut f, "a-picture", &["beach.png"], &mut counts);
    menu_levels(&mut f, "several-files", &["notes.txt", "report.pdf", "budget.ods"], &mut counts);
    f.go(0, Where::Trash);
    f.cols[0].entries.retain(|e| e.path.starts_with(&dir));
    menu_levels(&mut f, "a-trash-item", &["old notes.txt"], &mut counts);
    let out = PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-c".into()));
    std::fs::write(out.join("counts.tsv"), counts).unwrap();
    let _ = std::fs::remove_dir_all(dir);
}

fn menu_levels(f: &mut Files, name: &str, names: &[&str], counts: &mut String) {
    use crate::toolkit::popup::PopupMsg;
    let picked: Vec<usize> = names.iter().map(|n| index_of(f, 0, n)).collect();
    match picked.first() {
        Some(&first) => {
            f.update(Msg::Choose(0, first));
            f.update(Msg::Pick(0, picked.clone()));
        }
        None => {
            f.update(Msg::Unpick);
            f.cols[0].chosen = None;
        }
    }
    f.pointer.set((470.0, 40.0));
    f.update(Msg::Menu(MenuMsg::Open));
    let menu = f.menu.as_mut().expect("a menu");
    let folded = menu.lines.clone();
    menu.hover = None;
    menu.lines = as_it_was(&folded);
    let before = menu.lines.len();
    shoot(&format!("{name}-1-before"), f);
    f.menu.as_mut().unwrap().lines = folded.clone();
    shoot(&format!("{name}-2-after"), f);
    let mut subs = Vec::new();
    for (i, top) in folded.iter().enumerate().filter(|(_, l)| !l.sub.is_empty()) {
        f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(i))));
        shoot(&format!("{name}-3-after-{}", top.words.to_lowercase().replace(", ", "-").replace(' ', "-")), f);
        f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Out)));
        subs.push(format!("{}: {}", top.words, top.sub.iter().map(|l| l.words).collect::<Vec<_>>().join(", ")));
    }
    counts.push_str(&format!("{name}\t{before}\t{}\t{}\n", folded.len(), subs.join(" | ")));
    f.menu = None;
}

#[test]
#[ignore]
fn shoot_choose() {
    use crate::files::choose::{Ask, ChooseMsg, Choosing, Filter, Mode, Pick};
    let (dir, papers) = nav_papers("choose");
    let start = |ask: Ask| {
        let mut f = Files::new(vec![papers.clone()]);
        f.room = size();
        f.home = dir.clone();
        f.update(Msg::PanesAre(false));
        f.choosing_for(Choosing::new(ask, |_| {}));
        f
    };
    let filters = vec![Filter { name: "Documents".into(), rules: vec![(0, "*.txt".into()), (0, "*.pdf".into())] }, Filter { name: "All files".into(), rules: vec![(0, "*".into())] }];
    let choices = vec![Pick { id: "enc".into(), label: "Encoding".into(), options: vec![("u".into(), "UTF-8".into()), ("l".into(), "Latin-1".into())], value: "u".into() }];
    let mut f = start(Ask { filters: filters.clone(), choices, ..Ask::default() });
    pick(&mut f, 0, "notes.txt");
    shoot("choose-open", &f);
    let mut f = start(Ask { mode: Mode::Save, name: "letter.txt".into(), filters, ..Ask::default() });
    shoot("choose-save", &f);
    pick(&mut f, 0, "notes.txt");
    f.update(Msg::Chooser(ChooseMsg::Accept));
    shoot("choose-save-taken", &f);
    f.update(Msg::Chooser(ChooseMsg::Name("a/b.txt".into())));
    f.update(Msg::Chooser(ChooseMsg::Accept));
    shoot("choose-save-slash", &f);
    let f = start(Ask { mode: Mode::Open { many: false, folder: true }, ..Ask::default() });
    shoot("choose-folder", &f);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
#[ignore]
fn shoot_layout() {
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "layout".into());
    let (dir, papers) = nav_papers("layout");
    let mut f = Files::new(vec![papers.join("Albums/2025")]);
    f.room = size();
    f.home = dir.clone();
    f.update(Msg::PanesAre(false));
    std::fs::create_dir_all(papers.join("Backup-Sync")).unwrap();
    for (file, bytes) in [("beach.txt", 1_800), ("sandcastle.txt", 640), ("picnic list.txt", 320), ("tide times.ods", 12_000)] {
        std::fs::write(papers.join("Albums/2025/Summer").join(file), vec![b'x'; bytes]).unwrap();
    }
    f.pinned = vec![papers.join("Backup-Sync"), papers.join("Letters"), papers.join("Recipes")];
    f.pins_file = Some(dir.join("pins"));
    one_row_at_the_top(&mut f, &name, &papers);
    a_pinned_folder_under_a_name_of_its_own(&mut f, &name, &papers);
    a_file_held_over_the_path(&mut f, &name, &papers);
    std::fs::remove_dir_all(&dir).unwrap();
}

fn one_row_at_the_top(f: &mut Files, name: &str, papers: &Path) {
    use crate::files::found::FindMsg;
    f.go(0, Where::Folder(papers.join("Albums/2025/Summer")));
    shoot(&format!("{name}-one-pane"), f);
    f.update(Msg::Find(FindMsg::Open(Some(0))));
    f.update(Msg::Find(FindMsg::Text("beach".into())));
    shoot(&format!("{name}-search"), f);
    f.update(Msg::Find(FindMsg::Close));
    f.update(Msg::Locate);
    shoot(&format!("{name}-typing-place"), f);
    f.update(Msg::LocateStop);
    f.update(Msg::LocateLine(0));
    shoot(&format!("{name}-one-pane-typing-line"), f);
    f.update(Msg::LocateStop);
    f.update(Msg::NewTab(Some(0)));
    f.go(0, Where::Folder(papers.join("Albums/2025/Summer")));
    shoot(&format!("{name}-one-pane-tabs"), f);
    f.update(Msg::LocateLine(0));
    shoot(&format!("{name}-one-pane-tabs-typing-line"), f);
    f.update(Msg::LocateStop);
    let long = papers.join("Albums/2025").join("A folder with a very long name that goes on and on for the whole width of the card and then some more");
    std::fs::create_dir_all(long.join("Another folder with a very long name that goes on beyond the end of the card")).unwrap();
    f.go(0, Where::Folder(long.join("Another folder with a very long name that goes on beyond the end of the card")));
    shoot(&format!("{name}-one-pane-long"), f);
    f.update(Msg::CloseTab(Some(0)));
    f.go(0, Where::Computer);
    shoot(&format!("{name}-one-pane-computer"), f);
    f.go(0, Where::Folder(papers.join("Albums/2025/Summer")));
    f.room = (640.0, 500.0);
    shoot_at(&format!("{name}-narrow"), f, f.room);
    f.room = size();
    f.update(Msg::PanesAre(true));
    f.go(1, Where::Folder(papers.to_path_buf()));
    f.side = 0;
    shoot(&format!("{name}-two-panes"), f);
    f.update(Msg::LocateLine(1));
    shoot(&format!("{name}-two-panes-typing-line"), f);
    f.update(Msg::LocateStop);
    f.update(Msg::PanesAre(false));
}

fn a_pinned_folder_under_a_name_of_its_own(f: &mut Files, name: &str, papers: &Path) {
    f.update(Msg::Menu(MenuMsg::Side { pinned: true, row: 0, x: 60.0, y: 250.0 }));
    shoot(&format!("{name}-pin-menu"), f);
    f.menu = None;
    f.pin_rename_start(papers.join("Backup-Sync"));
    shoot(&format!("{name}-pin-typing"), f);
    f.update(Msg::PinNameText("C-sync".into()));
    f.update(Msg::PinNameDone);
    shoot(&format!("{name}-pin-renamed"), f);
}

fn a_file_held_over_the_path(f: &mut Files, name: &str, papers: &Path) {
    f.go(0, Where::Folder(papers.join("Albums/2025/Summer")));
    f.update(Msg::Unpick);
    let mut live = Live::new(f);
    let (x, y) = live.item(f, 0, "beach.txt");
    live.send(f, Input::PointerMove { x, y });
    live.send(f, Input::Press { button: Button::Left, x, y });
    live.send(f, Input::PointerMove { x: x + 40.0, y: y - 40.0 });
    for (tag, i) in [("pill-lit-parent", 3), ("pill-lit-further", 2)] {
        let r = live.ui.pill_boxes("path0")[i];
        live.send(f, Input::PointerMove { x: r.x + r.w / 2.0, y: r.y + r.h / 2.0 });
        live.snap(&format!("{name}-{tag}"), f);
    }
    let r = live.ui.pill_boxes("path0")[3];
    live.send(f, Input::PointerMove { x: r.x + r.w / 2.0, y: r.y + r.h / 2.0 });
    live.send(f, Input::Release { button: Button::Left, x: r.x + r.w / 2.0, y: r.y + r.h / 2.0 });
    live.snap(&format!("{name}-pill-dropped"), f);
    f.update(Msg::Undo);
    live.snap(&format!("{name}-pill-undone"), f);
}
