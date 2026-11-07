// Pictures of the main window: its views, tabs, menus and cards, the list and what is picked, and work under way.

use std::path::{Path, PathBuf};

use super::{index_of, pick, shoot, size};
use crate::files::act::Job;
use crate::files::app::{Files, Msg, View};
use crate::files::dialogs::AskMsg;
use crate::files::menu::{Act, MenuMsg};
use crate::files::places::Where;
use crate::toolkit::ui::App;

#[test]
#[ignore]
fn shoot_password_card() {
    use crate::files::archive::{Place, Trouble};
    use crate::files::locked::{Then, UnlockMsg};
    if let Some(home) = std::env::var_os("TRINDLE_SHOT_HOME") {
        std::env::set_var("HOME", home);
    }
    let mut f = Files::new(vec![std::env::temp_dir()]);
    f.room = size();
    let archive = std::env::temp_dir().join("holiday photos.7z");
    f.unlock_ask(archive.clone(), Then::Unpack(Place::Here, std::env::temp_dir()), None);
    shoot("w-password", &f);
    f.update(Msg::Unlock(UnlockMsg::Password("john-smith".into())));
    shoot("w1-password-typed", &f);
    f.dialog = None;
    f.unlock_ask(archive.clone(), Then::List, Some(Trouble::WrongKey));
    shoot("w3-password-wrong", &f);
    f.dialog = None;
    f.unlock_ask(archive, Then::Unpack(Place::Here, std::env::temp_dir()), Some(Trouble::WrongOrBad));
    shoot("w4-password-wrong-or-damaged", &f);
}

#[test]
#[ignore]
fn shoot_files() {
    if let Some(home) = std::env::var_os("TRINDLE_SHOT_HOME") {
        std::env::set_var("HOME", home);
    }
    let home = crate::common::files::home();
    let room = size();
    let mut f = Files::new(Vec::new());
    f.room = room;
    f.pinned = vec![home.join("Downloads"), home.join("Documents")];
    f.update(Msg::PanesAre(false));
    f.go(0, Where::Folder(home.clone()));
    views_dual_and_tabs(&mut f, &home);
    drives_and_a_copy(&mut f, &home);
    menus_on_a_file(&mut f);
    cards_over_the_list(&mut f, &home);
    tool_windows(&mut f, &home);
    empty_trash_and_recent(&mut f, &home);
}

fn views_dual_and_tabs(f: &mut Files, home: &Path) {
    shoot("a-single-list", f);
    f.update(Msg::Look(View::Icons));
    shoot("b-single-icons", f);
    f.update(Msg::Look(View::Compact));
    shoot("c-single-compact", f);
    f.update(Msg::Look(View::List));

    f.update(Msg::PanesAre(true));
    f.go(0, Where::Folder(home.to_path_buf()));
    f.go(1, Where::Folder(home.join("Documents")));
    pick(f, 0, "Photos");
    shoot("d-dual", f);

    f.update(Msg::NewTab(Some(0)));
    f.go(0, Where::Folder(home.join("Photos")));
    f.update(Msg::NewTab(Some(1)));
    f.go(1, Where::Folder(home.join("Downloads")));
    shoot("e-dual-tabs", f);
}

fn drives_and_a_copy(f: &mut Files, home: &Path) {
    f.update(Msg::PanesAre(false));
    f.update(Msg::Drive(crate::files::drives::DriveMsg::Listed(vec![crate::common::volumes::Volume {
        id: "/org/freedesktop/UDisks2/block_devices/sdb1".into(), dev: "sdb1".into(), label: "Holiday stick".into(), size: 32_000_000_000,
        drive: "d".into(), drive_name: "Cruzer".into(), kind: crate::common::volumes::Kind::Usb,
        mounts: vec![home.join("Documents").to_string_lossy().into_owned()], locked: false, container: None, can_eject: true, can_power_off: true, fixed: false, system: false, key: String::new(), drive_key: String::new(),
    }, crate::common::volumes::Volume {
        id: "/org/freedesktop/UDisks2/block_devices/sdc1".into(), dev: "sdc1".into(), label: String::new(), size: 16_000_000_000,
        drive: "e".into(), drive_name: "Stick".into(), kind: crate::common::volumes::Kind::Usb,
        mounts: Vec::new(), locked: true, container: None, can_eject: true, can_power_off: true, fixed: false, system: false, key: String::new(), drive_key: String::new(),
    }])));
    f.go(0, Where::Folder(home.join("Photos")));
    f.jobs.push(Job::sample(1, "Copying 3 items to Documents", 300, 120));
    shoot("f-drives-and-copy", f);
    f.jobs.clear();
    f.notify("Holiday stick can be taken out now");
    shoot("f2-notice", f);
    f.notice = None;
}

fn menus_on_a_file(f: &mut Files) {
    pick(f, 0, "beach.png");
    f.pointer.set((520.0, 240.0));
    f.update(Msg::Menu(MenuMsg::Open));
    shoot("g-menu", f);
    f.menu = None;
    f.update(Msg::Menu(MenuMsg::Side { pinned: true, row: 0, x: 100.0, y: 140.0 }));
    shoot("g2-side-menu", f);
    f.menu = None;
}

fn cards_over_the_list(f: &mut Files, home: &Path) {
    f.ask(Act::Properties, 0, vec![home.join("Photos/beach.png")]);
    shoot("h-properties", f);
    f.update(Msg::Ask(AskMsg::Page(1)));
    shoot("h1-properties-page", f);
    f.ask(Act::Properties, 0, vec![home.join("Documents")]);
    shoot("h3-properties-folder", f);
    f.update(Msg::Ask(AskMsg::Page(1)));
    shoot("h4-properties-folder-share", f);
    f.dialog = None;
    f.ask(Act::SendTo, 0, vec![home.join("Photos/beach.png")]);
    shoot("i-send-to", f);
    f.dialog = None;
    f.ask(Act::Colour, 0, vec![home.join("Photos")]);
    shoot("j-colour", f);
    f.dialog = None;
    f.update(Msg::Ask(AskMsg::Columns));
    shoot("k-columns", f);
    f.dialog = None;
    f.ask(Act::OpenWith, 0, vec![home.join("Documents/report.txt")]);
    shoot("l-open-with", f);
    f.dialog = None;
    f.confirm = Some(vec![home.join("Photos/beach.png")]);
    shoot("m-delete", f);
    f.confirm = None;
}

fn tool_windows(f: &mut Files, home: &Path) {
    f.do_act(Act::Compress, 0, vec![home.join("Photos/beach.png"), home.join("Photos/hills.png")]);
    shoot("q-compress", f);
    f.dialog = None;
    f.do_act(Act::Wipe, 0, vec![home.join("Photos/beach.png")]);
    shoot("r-wipe", f);
    f.dialog = None;
    f.renaming = None;
    f.update(Msg::Pick(0, vec![1, 2]));
    f.update(Msg::Rename);
    shoot("s-rename-many", f);
    f.dialog = None;
    f.do_act(Act::Dupes, 0, vec![home.to_path_buf()]);
    shoot("s2-dupes", f);
    f.dialog = None;
    f.do_act(Act::CompareFiles, 0, vec![home.join("Photos/a.txt"), home.join("Photos/b.txt")]);
    shoot("s3-compare-two", f);
    f.dialog = None;
    f.update(Msg::Far(crate::files::far::FarMsg::Open));
    shoot("t-connect", f);
    f.dialog = None;
    f.do_act(Act::ShareOnNetwork, 0, vec![home.join("Photos/Holiday")]);
    shoot("u-share", f);
    f.dialog = None;
    f.do_act(Act::Preview, 0, vec![home.join("Photos/beach.png")]);
    shoot("v-viewer", f);
    f.dialog = None;
    f.do_act(Act::Preview, 0, vec![home.join("Documents/report.txt")]);
    shoot("v2-viewer-text", f);
    f.dialog = None;
}

fn empty_trash_and_recent(f: &mut Files, home: &Path) {
    f.go(0, Where::Folder(home.join("Music")));
    shoot("n-empty", f);
    f.go(0, Where::Trash);
    shoot("o-trash", f);
    f.go(0, Where::Recent);
    shoot("p-recent", f);
}

#[test]
#[ignore]
fn shoot_rename() {
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-rename-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for sub in ["Albums", "Letters"] {
        std::fs::create_dir_all(dir.join("Papers").join(sub)).unwrap();
    }
    for file in ["holiday plans.txt", "notes.txt", "report.pdf"] {
        std::fs::write(dir.join("Papers").join(file), b"made up").unwrap();
    }
    let mut f = Files::new(vec![dir.join("Papers")]);
    f.room = size();
    f.update(Msg::PanesAre(false));
    for (view, word) in [(View::List, "list"), (View::List, "rich"), (View::Icons, "icons"), (View::Compact, "compact")] {
        f.view = view;
        f.kept.columns = (word == "rich").then(|| vec![crate::files::columns::Col::Size, crate::files::columns::Col::Date]);
        pick(&mut f, 0, "report.pdf");
        shoot(&format!("rename-{word}-before"), &f);
        f.update(Msg::Rename);
        shoot(&format!("rename-{word}-after"), &f);
        f.update(Msg::RenameText("notes.txt".into()));
        f.update(Msg::RenameDone);
        shoot(&format!("rename-{word}-refused"), &f);
        f.update(Msg::RenameStop);
        f.update(Msg::NewFolder);
        shoot(&format!("rename-{word}-new-folder"), &f);
        f.update(Msg::RenameStop);
        std::fs::remove_dir(dir.join("Papers/New folder")).unwrap();
        f.update(Msg::Reload);
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

const VIEWS: [(View, &str); 3] = [(View::List, "list"), (View::Icons, "icons"), (View::Compact, "compact")];

fn close_menu(f: &mut Files) {
    use crate::toolkit::popup::PopupMsg;
    f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Close)));
}

fn win_papers() -> (PathBuf, PathBuf) {
    use std::os::unix::fs::PermissionsExt;
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-win-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let papers = dir.join("Papers");
    for sub in ["Albums", "Letters"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    std::fs::write(papers.join("Albums/list.txt"), vec![b'x'; 4_200]).unwrap();
    let files: [(&str, &[u8]); 10] = [
        ("holiday plans.txt", b"made up"), ("notes.txt", b"made up, and a little longer"), ("report.pdf", b"%PDF-1.7 made up"),
        ("budget.ods", b"PK\x03\x04 made up"), ("photo.jpg.exe", b"MZ made up"), ("old letters.tar.gz", b"\x1f\x8b made up"),
        ("invoice.pdf", b"\x7fELF made up"), ("song.mp3", b"made up"), ("film.mkv", b"made up"), ("empty.txt", b""),
    ];
    for (name, bytes) in files {
        std::fs::write(papers.join(name), bytes).unwrap();
    }
    std::fs::write(papers.join("run me.sh"), b"#!/bin/sh\necho made up\n").unwrap();
    std::fs::set_permissions(papers.join("run me.sh"), std::fs::Permissions::from_mode(0o755)).unwrap();
    for (name, rgb) in [("beach.png", [222, 184, 120]), ("garden.png", [70, 140, 80]), ("hills.png", [90, 110, 170])] {
        image::RgbImage::from_fn(640, 400, |x, y| image::Rgb(if (x / 80 + y / 80) % 2 == 0 { rgb } else { [rgb[0] / 2, rgb[1] / 2, rgb[2] / 2] }))
            .save(papers.join(name)).unwrap();
    }
    (dir, papers)
}

#[test]
#[ignore]
fn shoot_win() {
    let (dir, papers) = win_papers();
    let mut f = Files::new(vec![papers.clone()]);
    f.room = size();
    f.update(Msg::PanesAre(false));
    f.pictures.inline = Some(crate::files::thumbs::Thumbs::at(&dir.join("thumbnails")));
    column_widths(&mut f);
    choosing_columns(&mut f);
    groups(&mut f);
    typing_to_jump(&mut f);
    select_menu_and_invert(&mut f);
    tick_boxes(&mut f);
    endings_hidden(&mut f);
    icon_sizes(&mut f);
    the_line_at_the_foot(&mut f, &papers);
    std::fs::remove_dir_all(&dir).unwrap();
}

fn column_widths(f: &mut Files) {
    use crate::files::columns::Col;
    f.view = View::List;
    shoot("win-columns-usual", f);
    f.kept.set_width(Col::Type, false, 250.0);
    f.kept.set_width(Col::Size, false, 70.0);
    shoot("win-columns-dragged", f);
    f.kept.columns = Some(vec![Col::Size, Col::Date]);
    f.kept.set_width(Col::Date, true, 200.0);
    shoot("win-columns-dragged-rich", f);
    f.kept.columns = None;
    f.kept.set_width(Col::Date, true, Col::Date.width());
    f.kept.set_width(Col::Type, false, Col::Type.width());
    f.kept.set_width(Col::Size, false, Col::Size.width());
}

fn choosing_columns(f: &mut Files) {
    use crate::files::columns::Col;
    use crate::toolkit::popup::PopupMsg;
    use crate::toolkit::widget::Pointer;
    f.pointer.set((620.0, 150.0));
    f.update(Msg::Head(0, Pointer::Menu(400.0, 10.0)));
    shoot("win-columns-menu", f);
    for col in [Col::Made, Col::Owner] {
        f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(Col::ALL.iter().position(|c| *c == col).unwrap()))));
    }
    shoot("win-columns-ticked", f);
    close_menu(f);
    f.update(Msg::SortBy(1));
    f.update(Msg::Flip);
    shoot("win-sorted-by-size", f);
    f.update(Msg::Flip);
    f.update(Msg::SortBy(0));
    for col in [Col::Made, Col::Owner] {
        let _ = f.kept.column(col, false);
    }
}

fn groups(f: &mut Files) {
    use crate::files::columns::Col;
    for (view, word) in VIEWS {
        f.view = view;
        f.update(Msg::GroupBy(1));
        f.update(Msg::Changed);
        shoot(&format!("win-groups-kind-{word}"), f);
    }
    f.view = View::List;
    f.update(Msg::GroupBy(2));
    shoot("win-groups-date-list", f);
    f.update(Msg::GroupBy(3));
    shoot("win-groups-size-list", f);
    f.kept.columns = Some(vec![Col::Size, Col::Date]);
    shoot("win-groups-size-rich", f);
    f.kept.columns = None;
    f.update(Msg::GroupBy(0));
}

fn typing_to_jump(f: &mut Files) {
    for (view, word) in VIEWS {
        f.view = view;
        f.ahead = Default::default();
        f.update(Msg::Typed("r".into()));
        f.update(Msg::Typed("e".into()));
        shoot(&format!("win-jump-{word}"), f);
    }
}

fn select_menu_and_invert(f: &mut Files) {
    use crate::toolkit::widget::Pointer;
    f.view = View::List;
    let _ = crate::files::view::view(f);
    let foot = f.cols[0].grid.get().rows(f.cols[0].entries.len());
    f.pointer.set((640.0, 330.0));
    f.update(Msg::Icon(0, foot, Pointer::Menu(30.0, 6.0)));
    shoot("win-select-menu", f);
    close_menu(f);
    let two = vec![index_of(f, 0, "notes.txt"), index_of(f, 0, "report.pdf")];
    f.update(Msg::Pick(0, two));
    shoot("win-invert-before", f);
    f.update(Msg::Invert);
    shoot("win-invert-after", f);
    f.update(Msg::Unpick);
}

fn tick_boxes(f: &mut Files) {
    use crate::toolkit::widget::Pointer;
    f.ticks = true;
    for (view, word) in VIEWS {
        f.view = view;
        let picked = vec![index_of(f, 0, "Letters"), index_of(f, 0, "beach.png"), index_of(f, 0, "notes.txt")];
        f.update(Msg::Pick(0, picked));
        shoot(&format!("win-ticks-{word}"), f);
        let pane = &f.cols[0];
        let c = pane.grid.get().cell(index_of(f, 0, "garden.png"), pane.width.get());
        f.update(Msg::Icon(0, 0, Pointer::Hover(c.x + c.w / 2.0, c.y + c.h / 2.0)));
        shoot(&format!("win-ticks-pointed-{word}"), f);
        f.update(Msg::Icon(0, 0, Pointer::Leave));
    }
    f.view = View::List;
    f.update(Msg::TickAll(0));
    shoot("win-ticks-all-list", f);
    f.update(Msg::Unpick);
    f.ticks = false;
}

fn endings_hidden(f: &mut Files) {
    f.endings = false;
    for (view, word) in VIEWS {
        f.view = view;
        shoot(&format!("win-endings-hidden-{word}"), f);
    }
    f.view = View::List;
    pick(f, 0, "garden.png");
    f.update(Msg::Rename);
    shoot("win-endings-rename", f);
    f.update(Msg::RenameStop);
    f.update(Msg::Unpick);
    f.endings = true;
}

fn icon_sizes(f: &mut Files) {
    f.view = View::Icons;
    for size in crate::files::icons::ICON_SIZES {
        f.kept.set_icon(size).unwrap();
        f.pictures_resized();
        f.update(Msg::Changed);
        shoot(&format!("win-icons-{size}"), f);
    }
    f.kept.set_icon(crate::files::icons::ICON_USUAL).unwrap();
    f.pictures_resized();
}

fn the_line_at_the_foot(f: &mut Files, papers: &Path) {
    f.view = View::List;
    f.update(Msg::Unpick);
    shoot("win-foot-nothing-picked", f);
    let some = vec![index_of(f, 0, "notes.txt"), index_of(f, 0, "hills.png")];
    f.update(Msg::Pick(0, some));
    shoot("win-foot-files-picked", f);
    let with_folder = vec![index_of(f, 0, "Albums"), index_of(f, 0, "notes.txt"), index_of(f, 0, "hills.png")];
    f.update(Msg::Pick(0, with_folder));
    shoot("win-foot-folders-picked", f);
    f.cache.set_size(&papers.join("Albums"), 1_000, false);
    shoot("win-foot-working-out", f);
}

#[test]
#[ignore]
fn shoot_work() {
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-work-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Papers/Letters")).unwrap();
    for n in 1..=30 {
        std::fs::write(dir.join("Papers").join(format!("report {n:02}.txt")), b"made up").unwrap();
    }
    let mut f = Files::new(vec![dir.join("Papers")]);
    f.room = size();
    f.update(Msg::PanesAre(false));
    f.jobs.push(Job::sample(1, "Copying 3 items to Documents", 300, 120));
    let start = std::time::Instant::now();
    let pace = |job: &mut Job, rate: u64, total: u64| {
        for s in 0..4 {
            job.pace.note(start + std::time::Duration::from_secs(s), s * rate, total);
        }
    };
    pace(&mut f.jobs[0], 24_000_000, 3_000_000_000);
    f.jobs[0].pace.files(1_240, 9_000);
    shoot("win-jobs-one", &f);
    f.jobs.push(Job::sample(2, "Moving holiday.mp4 to Videos", 100, 64));
    f.jobs.push(Job::sample(3, "Extracting photos.zip", 100, 8));
    pace(&mut f.jobs[1], 4_200_000, 900_000_000);
    shoot("win-jobs-three", &f);
    f.jobs[1].paused = true;
    shoot("win-jobs-paused", &f);
    f.jobs[1].paused = false;
    questions_about_clashes(&mut f, &dir);
    where_a_right_drag_was_let_go(&mut f);
    std::fs::remove_dir_all(&dir).unwrap();
}

fn questions_about_clashes(f: &mut Files, dir: &Path) {
    let made = |name: &str, side: u32, shade: u8| {
        let mut pix = tiny_skia::Pixmap::new(side, side * 3 / 4).unwrap();
        pix.fill(tiny_skia::Color::from_rgba8(shade, 120, 255 - shade, 255));
        pix.save_png(dir.join(name)).unwrap();
        dir.join(name)
    };
    std::fs::create_dir_all(dir.join("Other")).unwrap();
    let (coming, there) = (made("beach.png", 320, 230), made("Other/beach.png", 96, 40));
    f.pictures.inline = Some(crate::files::thumbs::Thumbs::at(&dir.join("thumbs")));
    f.update(Msg::Work(1, crate::files::work::News::Clash(crate::files::work::Clash { name: "beach.png".into(), folders: false, sides: Some((coming, there)), replace: true })));
    shoot("win-jobs-asking", f);
    shoot("win-clash-file", f);
    for n in 0..12 {
        std::fs::write(dir.join("Papers/Letters").join(format!("letter {n}.txt")), b"made up").unwrap();
    }
    std::fs::create_dir_all(dir.join("Other/Letters")).unwrap();
    std::fs::write(dir.join("Other/Letters/one.txt"), b"made up").unwrap();
    f.update(Msg::Work(1, crate::files::work::News::Clash(crate::files::work::Clash { name: "Letters".into(), folders: true, replace: true,
        sides: Some((dir.join("Papers/Letters"), dir.join("Other/Letters"))) })));
    f.update(Msg::AllSame(1, true));
    shoot("win-clash-folder", f);
}

fn where_a_right_drag_was_let_go(f: &mut Files) {
    f.jobs.clear();
    f.update(Msg::Look(View::Icons));
    shoot("win-right-drag-held", f);
    f.pointer.set((330.0, 250.0));
    f.update(Msg::Icon(0, 0, crate::toolkit::widget::Pointer::RightDown(150.0, 20.0)));
    f.update(Msg::Icon(0, 0, crate::toolkit::widget::Pointer::RightMove(30.0, 20.0)));
    shoot("win-right-drag-held", f);
    f.update(Msg::Icon(0, 0, crate::toolkit::widget::Pointer::RightUp(30.0, 20.0)));
    shoot("win-right-drag-menu", f);
}
