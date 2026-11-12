//! Tests of what Files does to files.

use std::fs;
use std::path::{Path, PathBuf};

use crate::files::app::{Files, Msg};
use crate::files::places::Where;
use crate::files::trash;
use crate::files::app::View;
use crate::toolkit::ui::App;
use crate::toolkit::widget::Pointer;
use super::{arriving, being_moved, cut_elsewhere, panes_left, SOURCE, UNDO_KEPT};
use super::steps::{keep, touched};
use super::types::{Job, Land, Turn, Words};
use super::text::failed_words;

mod busy;
mod drag;
mod pasting;
mod undo;

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-files-act-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("left")).unwrap();
    fs::create_dir_all(dir.join("right")).unwrap();
    dir
}

fn files(dir: &Path) -> Files {
    let mut f = Files::new(vec![dir.join("left")]);
    f.update(Msg::Add(Vec::new()));
    f.cols[1].at = Where::Folder(dir.join("right"));
    f.bin = Some(trash::Bin::at(&dir.join("Trash"), None));
    f.bin.as_ref().unwrap().ready().unwrap();
    f.update(Msg::Changed);
    f.update(Msg::Hidden);
    f.update(Msg::Hidden);
    f
}

fn pick(f: &mut Files, p: usize, name: &str) {
    let i = f.cols[p].entries.iter().position(|e| e.name == name).unwrap();
    f.update(Msg::Choose(p, i));
    f.update(Msg::Pick(p, vec![i]));
}

fn right_drag(f: &mut Files, from: (f32, f32), to: f32, local: (f32, f32)) {
    f.pointer.set((panes_left() + from.0, 100.0));
    f.update(Msg::Icon(0, 0, Pointer::RightDown(from.0, from.1)));
    f.pointer.set((to, 100.0));
    f.update(Msg::Icon(0, 0, Pointer::RightMove(local.0, local.1)));
    f.update(Msg::Icon(0, 0, Pointer::RightUp(local.0, local.1)));
}

fn pick_line(f: &mut Files, act: crate::files::menu::Act) {
    let i = f.menu.as_ref().unwrap().lines.iter().position(|l| l.act == act).unwrap();
    f.update(Msg::Menu(crate::files::menu::MenuMsg::Pop(crate::toolkit::popup::PopupMsg::Pick(i))));
}

fn icons(f: &mut Files) {
    f.view = View::Icons;
    f.update(Msg::Changed);
    for p in 0..2 {
        let pane = &f.cols[p];
        pane.width.set(440.0);
        pane.grid.set(crate::files::icons::Grid::icons(440.0));
    }
}

#[test]
fn the_source_list_names_every_file_of_the_folder() {
    assert_eq!(SOURCE.len(), crate::common::source::files_in("files/act"));
}
