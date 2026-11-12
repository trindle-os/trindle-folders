//! Pictures of Files drawn without a screen, to look at: run with

mod getting_about;
mod looks;
mod window;

use std::path::PathBuf;

use super::app::{Files, Msg};
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::{App, Ui};
use crate::toolkit::widget::Node;

pub(super) fn size() -> (f32, f32) {
    let size = std::env::var("TRINDLE_SHOT_SIZE").unwrap_or_else(|_| "1200x760".into());
    size.split_once('x').and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?))).unwrap_or((1200.0, 760.0))
}

fn draw(name: &str, tree: impl Fn() -> Node<Msg>, (w, h): (f32, f32)) {
    let out = PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-c".into()));
    std::fs::create_dir_all(&out).unwrap();
    for _ in 0..2 {
        let mut ui = Ui::new(tree(), Theme::new(std::env::var_os("TRINDLE_SHOT_LIGHT").is_some(), &crate::common::style::accent()));
        let mut pix = tiny_skia::Pixmap::new(w as u32, h as u32).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join(format!("{name}.png"))).unwrap();
    }
}

fn shoot(name: &str, f: &Files) {
    draw(name, || super::view::view(f), size());
}

pub(super) fn index_of(f: &Files, p: usize, name: &str) -> usize {
    f.cols[p].entries.iter().position(|e| e.name == name).unwrap_or(0)
}

fn pick(f: &mut Files, p: usize, name: &str) -> usize {
    let i = index_of(f, p, name);
    f.update(Msg::Choose(p, i));
    f.update(Msg::Pick(p, vec![i]));
    i
}

pub(super) fn nav_papers(tag: &str) -> (PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-nav-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let papers = dir.join("Papers");
    for sub in ["Albums/2025/Summer", "Albums/2026", "Letters", "Recipes", "Work/Invoices"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for name in ["holiday plans.txt", "notes.txt", "report.pdf", "budget.ods", "song.mp3", "film.mkv", "old letters.tar.gz"] {
        std::fs::write(papers.join(name), b"made up").unwrap();
    }
    std::fs::write(papers.join("Albums/list.txt"), b"made up").unwrap();
    for (name, rgb) in [("beach.png", [222, 184, 120]), ("garden.png", [70, 140, 80])] {
        image::RgbImage::from_fn(320, 200, |x, y| image::Rgb(if (x / 40 + y / 40) % 2 == 0 { rgb } else { [rgb[0] / 2, rgb[1] / 2, rgb[2] / 2] }))
            .save(papers.join(name)).unwrap();
    }
    (dir, papers)
}

pub(super) struct Live {
    pub(super) ui: Ui<Msg>,
}

impl Live {
    pub(super) fn new(f: &Files) -> Live {
        let mut live = Live { ui: Ui::new(super::view::view(f), Theme::new(false, &crate::common::style::accent())) };
        live.show(f);
        live
    }

    pub(super) fn show(&mut self, f: &Files) -> tiny_skia::Pixmap {
        let (w, h) = size();
        let mut pix = tiny_skia::Pixmap::new(w as u32, h as u32).unwrap();
        for _ in 0..2 {
            self.ui.set_view(super::view::view(f));
            self.ui.paint(&mut pix.as_mut(), 1.0);
        }
        pix
    }

    pub(super) fn send(&mut self, f: &mut Files, input: crate::toolkit::input::Input) {
        let sent = match f.intercept_with(&input, self.ui.keyboard()) {
            Some(m) => vec![m],
            None => self.ui.input(&input, std::time::Instant::now()).messages,
        };
        for m in sent {
            f.update(m);
        }
        self.show(f);
    }

    pub(super) fn snap(&mut self, name: &str, f: &Files) {
        let out = PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-c".into()));
        std::fs::create_dir_all(&out).unwrap();
        self.show(f).save_png(out.join(format!("{name}.png"))).unwrap();
    }

    pub(super) fn place(&self, name: &str) -> crate::toolkit::widget::Rect {
        self.ui.boxes().into_iter().find(|b| b.0.starts_with(name)).map(|b| b.1).unwrap_or_else(|| panic!("nothing called {name}"))
    }

    pub(super) fn item(&self, f: &Files, p: usize, name: &str) -> (f32, f32) {
        let area = self.place(&format!("pane{p}:"));
        let pane = &f.cols[p];
        let c = pane.grid.get().cell(index_of(f, p, name), pane.width.get());
        (area.x + c.x + 60.0, area.y + c.y + c.h / 2.0)
    }
}

fn shoot_at(name: &str, f: &Files, room: (f32, f32)) {
    draw(name, || super::view::view(f), room);
}
