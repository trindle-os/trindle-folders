//! A very big folder stays quick under every act a hand does in it.

use std::time::{Duration, Instant};

use super::app::{Files, Msg};
use crate::toolkit::input::{Button, Input};
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::{App, Ui};

const MANY: usize = 50_000;
const SIZE: (u32, u32) = (1200, 760);
const LIMIT: Duration = Duration::from_millis(150);
const TRIES: usize = 3;

fn best(mut act: impl FnMut() -> Duration) -> Duration {
    (0..TRIES).map(|_| act()).min().unwrap_or_default()
}

fn shown(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap, began: Instant) -> Duration {
    for _ in 0..3 {
        ui.set_view(f.view());
        let _tree = ui.access("Folders");
        ui.paint(&mut pix.as_mut(), 1.0);
        if !f.cols[0].tabs[0].seen.asked() {
            break;
        }
        f.update(Msg::Resized);
    }
    began.elapsed()
}

fn turn(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap, input: Input) -> Duration {
    let began = Instant::now();
    let reply = ui.input(&input, began);
    for m in reply.messages {
        f.update(m);
    }
    shown(f, ui, pix, began)
}

fn click(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap, button: Button, (x, y): (f32, f32)) -> Duration {
    turn(f, ui, pix, Input::Press { button, x, y }) + turn(f, ui, pix, Input::Release { button, x, y })
}

fn pass(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap, (x, y): (f32, f32)) -> Duration {
    (0..40).map(|k| turn(f, ui, pix, Input::PointerMove { x: x + (k % 7) as f32, y: y + (k * 5) as f32 })).max().unwrap_or_default()
}

type Took = Vec<(&'static str, Duration)>;

fn made_folder() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-big-folder-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for n in 0..MANY {
        std::fs::File::create(dir.join(format!("file{n:05}.txt"))).unwrap();
    }
    dir
}

fn opened(dir: &std::path::Path) -> (Files, Ui<Msg>, tiny_skia::Pixmap) {
    let mut f = Files::new(vec![dir.to_path_buf()]);
    assert_eq!(f.cols[0].tabs[0].entries.len(), MANY);
    f.resized(SIZE.0 as f32, SIZE.1 as f32);
    let mut pix = tiny_skia::Pixmap::new(SIZE.0, SIZE.1).unwrap();
    let mut ui = Ui::new(f.view(), Theme::new(true, "#fbc711"));
    ui.skip_audit();
    for _ in 0..3 {
        ui.paint(&mut pix.as_mut(), 1.0);
        f.update(Msg::Resized);
        ui.set_view(f.view());
    }
    (f, ui, pix)
}

fn where_the_items_are(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap) -> (f32, f32) {
    for y in (150..600).step_by(20) {
        for x in (220..600).step_by(40) {
            let _ = turn(f, ui, pix, Input::PointerMove { x: x as f32, y: y as f32 });
            if f.cols[0].tabs[0].hover.is_some() {
                return (x as f32, y as f32);
            }
        }
    }
    panic!("the pointer found no item in the left pane");
}

fn pointing_and_picking(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap, at: (f32, f32), took: &mut Took) {
    let below = (at.0, at.1 + 40.0);
    took.push(("a move of the pointer", best(|| pass(f, ui, pix, at))));
    took.push(("a click", best(|| click(f, ui, pix, Button::Left, at))));
    assert_eq!(f.cols[0].tabs[0].picked.len(), 1, "the click picked the item under it");
    took.push(("a right-click", best(|| {
        f.menu = None;
        click(f, ui, pix, Button::Right, below)
    })));
    assert!(f.menu.is_some(), "the right-click opened the menu");
    took.push(("a move over the open menu", best(|| pass(f, ui, pix, at))));
    f.menu = None;
    took.push(("picking everything", best(|| {
        let began = Instant::now();
        f.update(Msg::All);
        shown(f, ui, pix, began)
    })));
    assert_eq!(f.cols[0].tabs[0].picked.len(), MANY);
    took.push(("a move with everything picked", best(|| pass(f, ui, pix, at))));
    took.push(("a right-click with everything picked", best(|| {
        f.menu = None;
        click(f, ui, pix, Button::Right, below)
    })));
    assert!(f.menu.is_some());
    f.menu = None;
    took.push(("turning the choice round", best(|| {
        f.update(Msg::All);
        let began = Instant::now();
        f.update(Msg::Invert);
        shown(f, ui, pix, began)
    })));
    assert!(f.cols[0].tabs[0].picked.is_empty());
    took.push(("a click after that", best(|| click(f, ui, pix, Button::Left, at))));
}

fn scrolling_the_list(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap, took: &mut Took) {
    let step = best(|| {
        f.update(Msg::Step(super::app::Step::First, false));
        shown(f, ui, pix, Instant::now());
        (0..400).map(|_| {
            let t = turn(f, ui, pix, Input::Scroll { x: 0.0, y: 3.0, steps: true });
            assert!(f.cols[0].tabs[0].seen.covered(), "a scroll reached rows that were not made");
            t
        }).max().unwrap_or_default()
    });
    took.push(("a step of the wheel", step));
    let top = f.cols[0].tabs[0].seen.top().unwrap_or(0);
    assert!(top > 1000, "400 steps of the wheel reached only item {top}");
    took.push(("a leap to the last item", best(|| leap_to_the_last(f, ui, pix))));
    let pane = &f.cols[0].tabs[0];
    assert_eq!(pane.chosen, Some(MANY - 1));
    assert!(pane.seen.top().is_some_and(|t| t > MANY - 200) && pane.seen.covered(), "the last items are not in sight with their rows: {:?}", pane.seen.top());
}

fn leap_to_the_last(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap) -> Duration {
    f.update(Msg::Step(super::app::Step::First, false));
    shown(f, ui, pix, Instant::now());
    let began = Instant::now();
    f.update(Msg::Step(super::app::Step::Last, false));
    shown(f, ui, pix, began)
}

fn the_compact_view(f: &mut Files, ui: &mut Ui<Msg>, pix: &mut tiny_skia::Pixmap, at: (f32, f32), took: &mut Took) {
    let began = Instant::now();
    f.update(Msg::Look(super::app::View::Compact));
    took.push(("the Compact view the first time", shown(f, ui, pix, began)));
    took.push(("the Compact view", best(|| {
        f.update(Msg::Look(super::app::View::List));
        shown(f, ui, pix, Instant::now());
        let began = Instant::now();
        f.update(Msg::Look(super::app::View::Compact));
        shown(f, ui, pix, began)
    })));
    took.push(("a move of the pointer over the columns", best(|| pass(f, ui, pix, at))));
    took.push(("a click in the columns", best(|| click(f, ui, pix, Button::Left, at))));
    took.push(("a step of the wheel along the columns", best(|| turn(f, ui, pix, Input::Scroll { x: 0.0, y: 3.0, steps: true }))));
    assert!(f.cols[0].tabs[0].seen.top().is_some_and(|t| t > 0), "the wheel moved the columns along");
    took.push(("a leap to the last column", best(|| leap_to_the_last(f, ui, pix))));
    let pane = &f.cols[0].tabs[0];
    assert!(pane.seen.top().is_some_and(|t| t > MANY - 200) && pane.seen.covered(), "the last columns are not in sight with their names: {:?}", pane.seen.top());
    turn(f, ui, pix, Input::Scroll { x: -1.0e8, y: 0.0, steps: false });
    turn(f, ui, pix, Input::Scroll { x: 1.0e8, y: 0.0, steps: false });
    let pane = &f.cols[0].tabs[0];
    assert!(pane.seen.top().is_some_and(|t| t > MANY - 200) && pane.seen.covered(), "the end of the bar is not the last column: {:?}", pane.seen.top());
}

#[test]
#[ignore = "a speed test: run alone on a quiet machine with --ignored"]
fn a_folder_of_fifty_thousand_files_stays_quick() {
    let dir = made_folder();
    let (mut f, mut ui, mut pix) = opened(&dir);
    let at = where_the_items_are(&mut f, &mut ui, &mut pix);
    let mut took: Took = Vec::new();
    pointing_and_picking(&mut f, &mut ui, &mut pix, at, &mut took);
    scrolling_the_list(&mut f, &mut ui, &mut pix, &mut took);
    the_compact_view(&mut f, &mut ui, &mut pix, at, &mut took);
    let _ = std::fs::remove_dir_all(&dir);

    eprintln!("big folder: {took:?}");
    for (what, t) in &took {
        assert!(*t <= LIMIT, "{what} took {t:?} in a folder of {MANY} files: {took:?}");
    }
}
