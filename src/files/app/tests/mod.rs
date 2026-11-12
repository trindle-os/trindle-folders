//! Tests of Files as a whole: panes, tabs, picking, keys, pins and where it starts. Shared set-up is here.

use crate::toolkit::ui::Ui;

use super::*;

mod history;
mod keys;
mod locate;
mod picking;
mod pins;
mod start;
mod tabs;
mod zoom;

fn press(f: &Files, sym: Keysym, down: bool) {
    let mods = f.mods.get();
    let input = if down { Input::Key { sym, text: None, mods, repeat: false } } else { Input::KeyUp { sym, mods } };
    f.intercept(&input, false);
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-files-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn two_panes(name: &str) -> (PathBuf, Files) {
    let dir = scratch(name);
    for n in ["apple.txt", "Banana.txt", "cherry.txt", "damson.txt", "elder.txt"] {
        std::fs::write(dir.join(n), b"x").unwrap();
    }
    let mut kept = crate::files::kept::Kept::default();
    kept.dual = true;
    let start = (Where::Folder(dir.clone()), Where::Folder(dir.clone()));
    let f = Files::with(Vec::new(), dir.clone(), start, kept, Vec::new());
    (dir, f)
}

const VIEWS: [View; 3] = [View::List, View::Icons, View::Compact];

fn show(f: &Files, ui: &mut Ui<Msg>) {
    for _ in 0..2 {
        ui.set_view(f.view());
        let mut pix = tiny_skia::Pixmap::new(1200, 760).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
    }
}

fn send(f: &mut Files, ui: &mut Ui<Msg>, input: Input) {
    let sent = match f.intercept_with(&input, ui.keyboard()) {
        Some(m) => vec![m],
        None => ui.input(&input, Instant::now()).messages,
    };
    for m in sent {
        f.update(m);
    }
    show(f, ui);
}

fn item_centre(f: &Files, p: usize, name: &str) -> (f32, f32) {
    let _ = f.view();
    let pane = &f.cols[p];
    let i = pane.entries.iter().position(|e| e.name == name).unwrap();
    let c = pane.grid.get().cell(i, pane.width.get());
    (c.x + c.w / 2.0, c.y + c.h / 2.0)
}

fn names(f: &Files, p: usize) -> Vec<String> {
    f.cols[p].entries.iter().map(|e| e.name.clone()).collect()
}
