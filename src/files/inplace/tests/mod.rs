//! Tests for renaming in place.

mod bench;
mod endings;
#[cfg(test)]
mod naming_tests;
mod presses;
mod rename;
mod slow;
mod taken;

use crate::files::{icons, trash};
use crate::files::app::View;
use crate::toolkit::input::{Button, Input, Keysym, Mods};
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::{App, Ui};
use crate::toolkit::widget::{Pointer, Rect};
use self::bench::{bench, every_pane_and_tab, held_still, select_all, Bench};
use super::*;

#[test]
fn a_name_in_other_case_of_something_else_is_still_taken() {
    let dir = folder("case");
    let big = dir.join("left/Notes.txt");
    fs::write(&big, b"one").unwrap();
    let got = rename_here(&dir.join("left/Albums"), "albums");
    assert!(got.is_ok() && dir.join("left/albums").is_dir());
    fs::hard_link(&big, dir.join("left/NOTES.txt")).unwrap();
    assert!(rename_here(&big, "NOTES.txt").is_err() && big.exists());
    let _ = fs::remove_dir_all(&dir);
}

const ROOM: (u32, u32) = (1200, 760);
const VIEWS: [View; 3] = [View::List, View::Icons, View::Compact];

fn folder(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-files-inplace-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    for side in ["left", "right"] {
        for sub in ["Albums", "Letters"] {
            fs::create_dir_all(dir.join(side).join(sub)).unwrap();
        }
        for file in ["archive.tar.gz", "notes.txt", "report.pdf"] {
            fs::write(dir.join(side).join(file), b"made up").unwrap();
        }
    }
    dir
}

fn files(dir: &Path, view: View) -> Files {
    let mut f = Files::new(vec![dir.join("left")]);
    f.update(Msg::Add(Vec::new()));
    f.cols[1].at = Where::Folder(dir.join("right"));
    f.bin = Some(trash::Bin::at(&dir.join("Trash"), None));
    f.bin.as_ref().unwrap().ready().unwrap();
    f.view = view;
    f.update(Msg::Changed);
    f.update(Msg::Hidden);
    f.update(Msg::Hidden);
    f
}

fn side_dir(dir: &Path, p: usize) -> PathBuf {
    dir.join(if p == 0 { "left" } else { "right" })
}

fn index(f: &Files, p: usize, name: &str) -> usize {
    f.cols[p].entries.iter().position(|e| e.name == name).unwrap_or_else(|| panic!("{name} is not in pane {p}"))
}

fn names(f: &Files, p: usize) -> Vec<String> {
    f.cols[p].entries.iter().map(|e| e.name.clone()).collect()
}

fn chosen(f: &Files, p: usize) -> Option<String> {
    f.cols[p].chosen.and_then(|i| f.cols[p].entries.get(i)).map(|e| e.name.clone())
}

fn renaming(f: &Files) -> Option<String> {
    f.renaming.as_ref().map(Renaming::was)
}

fn paint(f: &Files) {
    for _ in 0..2 {
        let mut ui = Ui::new(crate::files::view::view(f), Theme::new(false, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new(ROOM.0, ROOM.1).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
    }
}

fn on_name(f: &Files, p: usize, name: &str) -> (f32, f32) {
    let (i, r) = f.cols[p].name_at.get().expect("the chosen item's name was painted");
    assert_eq!(i, index(f, p, name), "the name painted is the chosen one's");
    (r.x + r.w / 2.0, r.y + r.h / 2.0)
}

fn on_icon(f: &Files, p: usize, name: &str) -> (f32, f32) {
    let pane = &f.cols[p];
    let grid = pane.grid.get();
    let c = grid.cell(index(f, p, name), pane.width.get());
    if grid.lines { (c.x + icons::LINE_PAD + 4.0, c.y + c.h / 2.0) } else { (c.x + c.w / 2.0, c.y + 24.0) }
}

fn down(f: &mut Files, p: usize, at: (f32, f32), when: Instant) {
    f.icon_pointer_at(p, 0, Pointer::Down(at.0, at.1), when);
}

fn up(f: &mut Files, p: usize, at: (f32, f32), when: Instant) {
    f.icon_pointer_at(p, 0, Pointer::Up(at.0, at.1), when);
}

fn click(f: &mut Files, p: usize, at: (f32, f32), when: Instant) {
    down(f, p, at, when);
    up(f, p, at, when);
}

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

fn choose(f: &mut Files, p: usize, name: &str, when: Instant) -> (f32, f32) {
    let at = on_icon(f, p, name);
    click(f, p, at, when);
    assert_eq!(chosen(f, p).as_deref(), Some(name));
    assert!(f.slow.is_none() && f.renaming.is_none(), "the click that chooses starts nothing");
    paint(f);
    on_name(f, p, name)
}

#[test]
fn a_name_is_selected_up_to_its_last_ending() {
    assert_eq!(stem_len("report.pdf", false), "report".len());
    assert_eq!(stem_len("archive.tar.gz", false), "archive.tar".len());
    assert_eq!(stem_len("Makefile", false), "Makefile".len());
    assert_eq!(stem_len("Photos.2024", true), "Photos.2024".len());
    assert_eq!(stem_len(".profile", false), ".profile".len());
    assert_eq!(stem_len(".config.bak", false), ".config".len());
    assert_eq!(stem_len("caf\u{e9}.txt", false), "caf\u{e9}".len());
}

#[test]
fn a_second_click_is_slow_only_between_the_double_click_time_and_a_few_seconds() {
    let t = Instant::now();
    assert_eq!(TWICE, crate::toolkit::ui::MULTI_CLICK, "the toolkit's own double-click time");
    assert!(!slow_second(None, 3, t));
    assert!(!slow_second(Some((t, 3)), 3, t + TWICE - ms(1)), "a quick second click is a double click");
    assert!(slow_second(Some((t, 3)), 3, t + TWICE));
    assert!(slow_second(Some((t, 3)), 3, t + SLOW_WITHIN));
    assert!(!slow_second(Some((t, 3)), 3, t + SLOW_WITHIN + ms(1)), "too long after");
    assert!(!slow_second(Some((t, 2)), 3, t + ms(900)), "another item");
}

#[test]
fn names_change_in_folders_and_on_servers_but_not_in_other_places() {
    assert!(can_rename(&Where::Folder(PathBuf::from("/tmp/made-up"))));
    assert!(can_rename(&Where::Remote("sftp://john@example.com/files".into())));
    assert!(can_rename(&Where::Remote("mtp://phone/Internal".into())));
    assert!(!can_rename(&Where::Remote("smb://office.example.com".into())));
    for at in [Where::Trash, Where::Recent, Where::Network, Where::Inside(PathBuf::from("/tmp/a.zip"), String::new())] {
        assert!(!can_rename(&at), "{at:?}");
    }
}
