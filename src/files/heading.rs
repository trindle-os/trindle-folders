// The headings over the List view's columns, drawn so their edges can be dragged.

use std::time::Instant;

use super::app::{Files, Msg, Pane, View};
use super::columns::{Col, NAME_LEAST};
use super::icons::{tick_box, Ticked, LINE_ICON, LINE_PAD, TICK, TICK_GAP};
use super::inplace::TWICE;
use crate::toolkit::input::Cursor;
use crate::toolkit::text::Style;
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::ui::App;
use crate::toolkit::widget::{drawing, Length, Node, Pointer, Rect};

pub const LEAST: f32 = 48.0;
pub const MOST: f32 = 480.0;
const GRAB: f32 = 5.0;
const HEIGHT: f32 = 24.0;
const ARROW: f32 = 8.0;
const ARROW_GAP: f32 = 6.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Head {
    pub cols: Vec<(Col, f32)>,
    pub width: f32,
    pub sorted: Option<(Part, bool)>,
    pub tick: Option<Ticked>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Tick,
    Name,
    Col(usize),
    Edge(usize),
}

impl Head {
    pub fn name_left(&self) -> f32 {
        let lead = if self.tick.is_some() { TICK + TICK_GAP } else { 0.0 };
        2.0 * LINE_PAD + lead + LINE_ICON
    }

    pub fn left(&self, k: usize) -> f32 {
        self.width - LINE_PAD - self.cols[k.min(self.cols.len())..].iter().map(|c| c.1).sum::<f32>()
    }

    pub fn name_room(&self) -> f32 {
        self.left(0) - LINE_PAD - self.name_left()
    }

    pub fn part_at(&self, x: f32) -> Part {
        if let Some(k) = (0..self.cols.len()).find(|&k| (x - self.left(k)).abs() <= GRAB) {
            return Part::Edge(k);
        }
        match (0..self.cols.len()).rev().find(|&k| x >= self.left(k)) {
            Some(k) => Part::Col(k),
            None if self.tick.is_some() && x < LINE_PAD + TICK + TICK_GAP / 2.0 => Part::Tick,
            None => Part::Name,
        }
    }

    pub fn pull(&self, k: usize, dx: f32) -> f32 {
        let Some(&(_, after)) = self.cols.get(k) else { return 0.0 };
        let (mut lo, mut hi) = (after - MOST, after - LEAST);
        match k.checked_sub(1).and_then(|b| self.cols.get(b)) {
            Some(&(_, before)) => {
                lo = lo.max(LEAST - before);
                hi = hi.min(MOST - before);
            }
            None => lo = lo.max(NAME_LEAST - self.name_room()),
        }
        dx.clamp(lo.min(0.0), hi.max(0.0))
    }
}

#[derive(Clone, Debug)]
pub struct Sizing {
    pane: usize,
    edge: usize,
    from: f32,
    was: Head,
}

pub fn view<M: 'static>(head: Head, gutter: f32, on: impl Fn(Pointer) -> M + 'static) -> Node<M> {
    let mut said = vec![t!("folders-heading-name").to_string()];
    said.extend(head.cols.iter().map(|(c, _)| c.word().to_string()));
    let (painted, pointed) = (head.clone(), head);
    drawing((0.0, HEIGHT), t!("folders-heading-spoken", names = &said.join(", ")), move |canvas, pen| {
        let s = pen.scale;
        let head = Head { width: canvas.width() as f32 / s - gutter, ..painted.clone() };
        let (size, weight) = theme::sheet().column_heading;
        let style = Style { weight, ..Style::new((size * s).round()) };
        let dots = pen.fonts.layout("\u{2026}", &style, None);
        let mut word = |part: Part, text: &str, x: f32, room: f32| {
            let sorted = head.sorted.filter(|(by, _)| *by == part);
            let room = if sorted.is_some() { room - ARROW - ARROW_GAP } else { room };
            let laid = pen.fonts.layout(text, &style, None).cut((room * s).max(0.0), &dots);
            let y = ((canvas.height() as f32 - laid.height) / 2.0).round();
            pen.fonts.draw(canvas, &laid, (x * s).round(), y, pen.theme.muted);
            if let Some((_, up)) = sorted {
                arrow(canvas, pen, (x * s).round() + laid.width + ARROW_GAP * s, up);
            }
        };
        word(Part::Name, t!("folders-heading-name"), head.name_left(), head.name_room());
        for (k, (col, room)) in head.cols.iter().enumerate() {
            word(Part::Col(k), col.word(), head.left(k), room - LINE_PAD);
        }
        if let Some(state) = head.tick {
            let (t, h) = (TICK * s, canvas.height() as f32);
            tick_box(canvas, pen, Rect { x: (LINE_PAD * s).round(), y: ((h - t) / 2.0).round(), w: t, h: t }, state);
        }
    })
    .width(Length::Fill)
    .height(Length::Fixed(HEIGHT))
    .on_pointer(on)
    .on_hover()
    .named(true)
    .cursor_with(move |x, _| if matches!(pointed.part_at(x), Part::Edge(_)) { Cursor::ColResize } else { Cursor::Arrow })
}

fn arrow(canvas: &mut tiny_skia::PixmapMut, pen: &crate::toolkit::widget::Pen, x: f32, up: bool) {
    let a = ARROW * pen.scale;
    let (top, foot) = ((canvas.height() as f32 - a * 0.6) / 2.0, (canvas.height() as f32 + a * 0.6) / 2.0);
    let (point, base) = if up { (top, foot) } else { (foot, top) };
    let mut path = tiny_skia::PathBuilder::new();
    path.move_to(x, base);
    path.line_to(x + a, base);
    path.line_to(x + a / 2.0, point);
    path.close();
    if let Some(path) = path.finish() {
        let paint = crate::toolkit::colour::paint(pen.theme.muted);
        canvas.fill_path(&path, &paint, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
    }
}

impl Files {
    pub(super) fn head_of(&self, pane: &Pane, p: usize) -> Option<Head> {
        if self.view_of(p) != View::List {
            return None;
        }
        let cols = super::view::shown_widths(self, pane, p);
        let sorted = match self.sorted_col() {
            None => Some(Part::Name),
            Some(by) => cols.iter().position(|(c, _)| *c == by).map(Part::Col),
        };
        let tick = self.ticks.then_some(match pane.picked.len() {
            0 => Ticked::None,
            n if n == pane.entries.len() => Ticked::All,
            _ => Ticked::Some,
        });
        Some(Head { cols, width: pane.width.get(), sorted: sorted.map(|part| (part, self.up)), tick })
    }

    pub(super) fn head_pointer(&mut self, p: usize, at: Pointer) {
        self.head_pointer_at(p, at, Instant::now());
    }

    pub(super) fn head_pointer_at(&mut self, p: usize, at: Pointer, now: Instant) {
        let Some(head) = self.head_of(&self.cols[p], p) else { return };
        match at {
            Pointer::Down(x, _) => {
                self.sizing = None;
                self.head_down = None;
                let k = match head.part_at(x) {
                    Part::Edge(k) => k,
                    Part::Tick => {
                        self.edge_last = None;
                        return self.update(Msg::TickAll(p));
                    }
                    on => {
                        self.edge_last = None;
                        self.head_down = Some((p, on));
                        return;
                    }
                };
                let twice = self.edge_last.take().is_some_and(|(t, q, j)| q == p && j == k && now.saturating_duration_since(t) < TWICE);
                if twice {
                    return self.fit_column(p, head.cols[k].0);
                }
                self.edge_last = Some((now, p, k));
                self.sizing = Some(Sizing { pane: p, edge: k, from: x, was: head });
            }
            Pointer::Move(x, _) => {
                let Some(s) = self.sizing.as_ref().filter(|s| s.pane == p) else { return };
                let dx = s.was.pull(s.edge, x - s.from);
                let rich = self.widths_rich();
                let mut set = vec![(s.was.cols[s.edge].0, s.was.cols[s.edge].1 - dx)];
                if let Some(&(before, w)) = s.edge.checked_sub(1).and_then(|b| s.was.cols.get(b)) {
                    set.push((before, w + dx));
                }
                for (col, w) in set {
                    self.kept.set_width(col, rich, w);
                }
            }
            Pointer::Up(x, _) => {
                if self.sizing.take().is_some() {
                    self.widths_keep();
                }
                if self.head_down.take() == Some((p, head.part_at(x))) {
                    let col = match head.part_at(x) {
                        Part::Col(k) => head.cols.get(k).map(|c| c.0),
                        _ => None,
                    };
                    if self.sorted_col() == col {
                        self.up = !self.up;
                        self.reload_all();
                    } else {
                        self.up = true;
                        self.sort_on(col);
                    }
                }
            }
            Pointer::Menu(..) => self.columns_menu(p),
            Pointer::Hover(..) | Pointer::Leave | Pointer::Middle(..) | Pointer::RightDown(..) | Pointer::RightMove(..) | Pointer::RightUp(..) => {}
        }
    }

    pub(super) fn widths_rich(&self) -> bool {
        !self.kept.columns().contains(&Col::Type)
    }

    fn widths_keep(&mut self) {
        if let Err(e) = self.kept.save_widths() {
            self.say_help(t!("folders-heading-widths-not-kept", why = &e.to_string()), super::help::NOT_KEPT);
        }
    }

    fn fit_column(&mut self, p: usize, col: Col) {
        let fonts = crate::toolkit::text::Fonts::new();
        let small = Style::new(theme::sheet().note.0.round());
        let (size, weight) = theme::sheet().column_heading;
        let heading = Style { weight, ..Style::new(size.round()) };
        let mut widest = fonts.layout(col.word(), &heading, None).width;
        for e in &self.cols[p].entries {
            let text = self.cache.cells(e, &[col]).pop().unwrap_or_default();
            if !text.is_empty() {
                widest = widest.max(fonts.layout(&text, &small, None).width);
            }
        }
        let rich = self.widths_rich();
        self.kept.set_width(col, rich, (widest.ceil() + LINE_PAD).clamp(LEAST, MOST));
        self.widths_keep();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::kept::Kept;
    use crate::files::places::Where;
    use std::path::PathBuf;
    use std::time::Duration;

    fn head(width: f32) -> Head {
        Head { cols: vec![(Col::Size, 92.0), (Col::Type, 160.0), (Col::Date, 106.0)], width, sorted: None, tick: None }
    }

    #[test]
    fn a_point_on_the_headings_is_on_a_heading_or_on_an_edge() {
        let h = head(800.0);
        assert_eq!((h.left(0), h.left(1), h.left(2)), (430.0, 522.0, 682.0));
        assert_eq!(h.part_at(100.0), Part::Name);
        assert_eq!(h.part_at(430.0 - GRAB), Part::Edge(0));
        assert_eq!(h.part_at(430.0 + GRAB + 1.0), Part::Col(0));
        assert_eq!(h.part_at(522.0), Part::Edge(1));
        assert_eq!(h.part_at(600.0), Part::Col(1));
        assert_eq!(h.part_at(684.0), Part::Edge(2));
        assert_eq!(h.part_at(790.0), Part::Col(2));
        assert_eq!(h.name_left(), 2.0 * LINE_PAD + LINE_ICON);
    }

    #[test]
    fn an_edge_goes_no_further_than_the_least_and_the_most_allow() {
        let h = head(800.0);
        assert_eq!(h.pull(1, 20.0), 20.0);
        assert_eq!(h.pull(1, 500.0), 160.0 - LEAST);
        assert_eq!(h.pull(1, -500.0), LEAST - 92.0);
        assert_eq!(h.pull(0, 500.0), 92.0 - LEAST);
        assert_eq!(h.pull(0, -1000.0), NAME_LEAST - h.name_room());
        assert!(h.name_room() + h.pull(0, -1000.0) >= NAME_LEAST);
        let narrow = Head { width: 300.0, cols: vec![(Col::Size, MOST)], sorted: None, tick: None };
        assert_eq!(narrow.pull(0, -50.0), 0.0);
    }

    fn window(tag: &str, columns: Option<Vec<Col>>) -> (PathBuf, Files) {
        let dir = std::env::temp_dir().join(format!("trindle-files-heading-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("things/Letters")).unwrap();
        for (name, bytes) in [("a.txt", 7), ("b.pdf", 13), ("c.png", 900)] {
            std::fs::write(dir.join("things").join(name), vec![b'x'; bytes]).unwrap();
        }
        let mut kept = Kept::from(Some(dir.join("conf")));
        kept.dual = true;
        kept.columns = columns;
        let at = Where::Folder(dir.join("things"));
        let mut f = Files::with(Vec::new(), dir.clone(), (at.clone(), at), kept, Vec::new());
        f.view = View::List;
        for p in 0..2 {
            f.cols[p].width.set(800.0);
        }
        (dir, f)
    }

    fn widths(f: &Files, p: usize) -> Vec<f32> {
        f.head_of(&f.cols[p], p).unwrap().cols.iter().map(|c| c.1).collect()
    }

    fn lefts(f: &Files, p: usize) -> Vec<f32> {
        let h = f.head_of(&f.cols[p], p).unwrap();
        (0..h.cols.len()).map(|k| h.left(k)).collect()
    }

    #[test]
    fn dragging_an_edge_trades_room_between_two_columns_and_nothing_else_moves() {
        for (kind, columns) in [("plain", Some(vec![Col::Size, Col::Type, Col::Date])), ("rich", Some(vec![Col::Size, Col::Date, Col::Owner]))] {
            let (dir, mut f) = window(&format!("drag-{kind}"), columns);
            for p in 0..2 {
                let what = format!("{kind}, pane {p}");
                let (w0, l0) = (widths(&f, p), lefts(&f, p));
                f.update(Msg::Head(p, Pointer::Down(l0[1], 10.0)));
                f.update(Msg::Head(p, Pointer::Move(l0[1] - 30.0, 10.0)));
                let (w1, l1) = (widths(&f, p), lefts(&f, p));
                assert_eq!((w1[0], w1[1], w1[2]), (w0[0] - 30.0, w0[1] + 30.0, w0[2]), "{what}");
                assert_eq!((l1[0], l1[1], l1[2]), (l0[0], l0[1] - 30.0, l0[2]), "{what}: only the edge moved");
                f.update(Msg::Head(p, Pointer::Move(l0[1] + 10.0, 10.0)));
                assert_eq!(widths(&f, p)[..2], [w0[0] + 10.0, w0[1] - 10.0], "{what}");
                f.update(Msg::Head(p, Pointer::Up(l0[1] + 10.0, 10.0)));
                let (w2, l2) = (widths(&f, p), lefts(&f, p));
                f.update(Msg::Head(p, Pointer::Down(l2[0], 10.0)));
                f.update(Msg::Head(p, Pointer::Move(l2[0] + 20.0, 10.0)));
                f.update(Msg::Head(p, Pointer::Up(l2[0] + 20.0, 10.0)));
                let (w3, l3) = (widths(&f, p), lefts(&f, p));
                assert_eq!((w3[0], w3[1], w3[2]), (w2[0] - 20.0, w2[1], w2[2]), "{what}");
                assert_eq!((l3[0], l3[1], l3[2]), (l2[0] + 20.0, l2[1], l2[2]), "{what}");
                f.update(Msg::Head(p, Pointer::Down(l3[2], 10.0)));
                f.update(Msg::Head(p, Pointer::Move(l3[2] + 2000.0, 10.0)));
                f.update(Msg::Head(p, Pointer::Up(l3[2] + 2000.0, 10.0)));
                assert_eq!(widths(&f, p)[2], LEAST, "{what}");
                let w4 = widths(&f, p);
                f.update(Msg::Head(p, Pointer::Down(l3[1] + 40.0, 10.0)));
                f.update(Msg::Head(p, Pointer::Move(l3[1] + 90.0, 10.0)));
                f.update(Msg::Head(p, Pointer::Up(l3[1] + 90.0, 10.0)));
                assert_eq!(widths(&f, p), w4, "{what}");
                assert_eq!(widths(&f, 1 - p), w4, "{what}");
                let back = Kept::from(Some(dir.join("conf")));
                let rich = kind == "rich";
                let shown = f.head_of(&f.cols[p], p).unwrap();
                for (col, w) in &shown.cols {
                    assert_eq!(back.width(*col, rich), *w, "{what}: {col:?} is kept");
                }
                for (col, _) in &shown.cols {
                    f.kept.set_width(*col, rich, col.width());
                }
            }
            for view in [View::Icons, View::Compact] {
                f.view = view;
                assert!(f.head_of(&f.cols[0], 0).is_none());
                f.update(Msg::Head(0, Pointer::Down(500.0, 10.0)));
                assert!(f.sizing.is_none());
            }
            std::fs::remove_dir_all(&dir).unwrap();
        }
    }

    fn names(f: &Files, p: usize) -> Vec<String> {
        f.cols[p].entries.iter().map(|e| e.name.clone()).collect()
    }

    fn press(f: &mut Files, p: usize, x: f32) {
        f.update(Msg::Head(p, Pointer::Down(x, 10.0)));
        f.update(Msg::Head(p, Pointer::Up(x, 10.0)));
    }

    #[test]
    fn pressing_a_heading_sorts_by_it_and_pressing_it_again_turns_it_round() {
        let (dir, mut f) = window("sort", Some(vec![Col::Size, Col::Items, Col::Owner]));
        std::fs::create_dir_all(dir.join("things/Albums")).unwrap();
        std::fs::write(dir.join("things/Letters/one"), b"x").unwrap();
        f.reload_all();
        for p in 0..2 {
            let l = lefts(&f, p);
            let sorted = |f: &Files| f.head_of(&f.cols[p], p).unwrap().sorted;
            assert_eq!(sorted(&f), Some((Part::Name, true)), "by name to begin with");
            press(&mut f, p, l[0] + 30.0);
            assert_eq!((f.sorted_col(), sorted(&f)), (Some(Col::Size), Some((Part::Col(0), true))));
            assert_eq!(names(&f, p)[2..], ["a.txt", "b.pdf", "c.png"]);
            press(&mut f, p, l[0] + 30.0);
            assert_eq!(sorted(&f), Some((Part::Col(0), false)));
            assert_eq!(names(&f, p)[2..], ["c.png", "b.pdf", "a.txt"]);
            press(&mut f, p, l[1] + 30.0);
            assert_eq!((f.sorted_col(), sorted(&f)), (Some(Col::Items), Some((Part::Col(1), true))));
            assert_eq!(names(&f, p)[..2], ["Albums", "Letters"]);
            press(&mut f, p, l[1] + 30.0);
            assert_eq!(names(&f, p)[..2], ["Letters", "Albums"]);
            assert_eq!(names(&f, 1 - p), names(&f, p));
            f.update(Msg::Head(p, Pointer::Down(l[0] + 30.0, 10.0)));
            f.update(Msg::Head(p, Pointer::Up(60.0, 10.0)));
            assert_eq!(f.sorted_col(), Some(Col::Items));
            press(&mut f, p, 60.0);
            assert_eq!((f.sorted_col(), sorted(&f)), (None, Some((Part::Name, true))));
            assert_eq!(names(&f, p), ["Albums", "Letters", "a.txt", "b.pdf", "c.png"]);
            f.update(Msg::SortBy(1 + Col::ALL.iter().position(|c| *c == Col::Owner).unwrap()));
            assert_eq!((f.sorted_col(), sorted(&f)), (Some(Col::Owner), Some((Part::Col(2), true))));
            f.update(Msg::SortBy(1));
            assert_eq!(f.sorted_col(), Some(Col::Size));
            f.update(Msg::SortBy(0));
            assert_eq!(f.sorted_col(), None);
            f.update(Msg::SortBy(1 + Col::ALL.iter().position(|c| *c == Col::Date).unwrap()));
            assert_eq!(sorted(&f), None);
            f.update(Msg::SortBy(0));
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_right_click_on_the_headings_offers_the_columns_to_tick_and_stays_open() {
        use crate::files::icons::stand_in::TICKED;
        use crate::files::menu::{Act, MenuMsg};
        use crate::toolkit::popup::PopupMsg;
        let (dir, mut f) = window("menu", None);
        for p in 0..2 {
            f.update(Msg::Head(p, Pointer::Menu(500.0, 10.0)));
            let lines = f.menu.as_ref().expect("the headings have a menu").lines.clone();
            assert_eq!(lines.iter().map(|l| l.act).collect::<Vec<_>>(), Col::ALL.iter().map(|c| Act::Column(*c)).collect::<Vec<_>>());
            assert!(lines.iter().all(|l| l.words != "Name"));
            for l in &lines {
                let Act::Column(c) = l.act else { unreachable!() };
                assert_eq!(l.icon == TICKED, Col::USUAL.contains(&c), "{c:?}");
            }
            for word in ["Size", "Type", "Date changed", "Date made", "Permissions", "Owner", "Picture size", "Length", "Artist", "Album"] {
                assert!(lines.iter().any(|l| l.words == word), "{word}");
            }
            let at = |c: Col| Col::ALL.iter().position(|k| *k == c).unwrap();
            f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(at(Col::Owner)))));
            f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(at(Col::Made)))));
            f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(at(Col::Type)))));
            assert_eq!(f.menu.as_ref().unwrap().lines[at(Col::Type)].icon, TICKED);
            assert!(f.kept.columns().contains(&Col::Type));
            f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(at(Col::Type)))));
            let menu = f.menu.as_ref().expect("the menu stays open");
            assert_eq!((menu.lines[at(Col::Owner)].icon, menu.lines[at(Col::Type)].icon, menu.hover), (TICKED, "", Some(at(Col::Type))));
            assert_eq!(f.kept.columns(), [Col::Size, Col::Date, Col::Made, Col::Owner]);
            assert_eq!(Kept::from(Some(dir.join("conf"))).columns(), [Col::Size, Col::Date, Col::Made, Col::Owner], "kept");
            let shown: Vec<Col> = f.head_of(&f.cols[p], p).unwrap().cols.iter().map(|c| c.0).collect();
            assert_eq!(shown, [Col::Size, Col::Date, Col::Made, Col::Owner]);
            let l = lefts(&f, p);
            f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Close)));
            assert!(f.menu.is_none());
            press(&mut f, p, l[3] + 20.0);
            assert_eq!(f.sorted_col(), Some(Col::Owner));
            f.update(Msg::SortBy(0));
            f.update(Msg::Head(p, Pointer::Menu(500.0, 10.0)));
            for c in [Col::Owner, Col::Made] {
                f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Pick(at(c)))));
            }
            f.update(Msg::Menu(MenuMsg::Pop(PopupMsg::Close)));
            assert_eq!(f.kept.columns(), Col::USUAL);
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_tick_box_in_the_name_heading_picks_all_or_none() {
        let (dir, mut f) = window("tick", None);
        let off = f.head_of(&f.cols[0], 0).unwrap();
        assert_eq!((off.tick, off.part_at(LINE_PAD + 4.0)), (None, Part::Name));
        f.ticks = true;
        for p in 0..2 {
            let h = f.head_of(&f.cols[p], p).unwrap();
            assert_eq!(h.tick, Some(Ticked::None));
            assert_eq!(h.name_left() - off.name_left(), TICK + TICK_GAP, "Name starts after the box, as each line's name does");
            assert_eq!(h.part_at(LINE_PAD + 4.0), Part::Tick);
            f.update(Msg::Head(p, Pointer::Down(LINE_PAD + 4.0, 10.0)));
            f.update(Msg::Head(p, Pointer::Up(LINE_PAD + 4.0, 10.0)));
            assert_eq!(f.cols[p].picked, [0, 1, 2, 3]);
            assert_eq!(f.head_of(&f.cols[p], p).unwrap().tick, Some(Ticked::All));
            f.update(Msg::Pick(p, vec![1, 2]));
            assert_eq!(f.head_of(&f.cols[p], p).unwrap().tick, Some(Ticked::Some));
            f.update(Msg::Head(p, Pointer::Down(LINE_PAD + 4.0, 10.0)));
            assert_eq!(f.cols[p].picked.len(), 4);
            f.update(Msg::Head(p, Pointer::Down(LINE_PAD + 4.0, 10.0)));
            assert!(f.cols[p].picked.is_empty());
            assert_eq!(f.sorted_col(), None);
            assert!(f.up);
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn each_kind_of_list_keeps_its_own_widths_and_borrows_the_others_until_it_has_some() {
        let (dir, mut f) = window("kinds", None);
        f.kept.set_width(Col::Size, false, 130.0);
        assert_eq!(f.kept.width(Col::Size, false), 130.0);
        assert_eq!(f.kept.width(Col::Size, true), 130.0, "the rich rows borrow it");
        f.kept.set_width(Col::Size, true, 70.0);
        assert_eq!((f.kept.width(Col::Size, false), f.kept.width(Col::Size, true)), (130.0, 70.0));
        assert_eq!(f.kept.width(Col::Date, true), Col::Date.width(), "the usual width until one is set");
        f.kept.save_widths().unwrap();
        std::fs::write(dir.join("conf").join(crate::files::kept::WIDTHS), "list size 9000\nrich date 3\nlist nonsense 80\nodd size 80\n").unwrap();
        let back = Kept::from(Some(dir.join("conf")));
        assert_eq!((back.width(Col::Size, false), back.width(Col::Date, true)), (MOST, LEAST));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_double_click_on_an_edge_fits_the_column_to_its_widest_entry() {
        let (dir, mut f) = window("fit", Some(vec![Col::Size, Col::Type, Col::Date]));
        for p in 0..2 {
            f.kept.set_width(Col::Size, false, 300.0);
            f.kept.set_width(Col::Date, false, LEAST);
            let l = lefts(&f, p);
            let t = Instant::now();
            f.head_pointer_at(p, Pointer::Down(l[0], 10.0), t);
            f.head_pointer_at(p, Pointer::Up(l[0], 10.0), t);
            assert_eq!(widths(&f, p)[0], 300.0, "one press fits nothing");
            f.head_pointer_at(p, Pointer::Down(l[0], 10.0), t + Duration::from_millis(150));
            let fitted = widths(&f, p)[0];
            assert!(fitted > LEAST && fitted < 140.0, "pane {p}: {fitted}");
            assert!(f.sizing.is_none(), "a double click drags nothing");
            let l = lefts(&f, p);
            f.head_pointer_at(p, Pointer::Down(l[2], 10.0), t + Duration::from_secs(5));
            f.head_pointer_at(p, Pointer::Up(l[2], 10.0), t + Duration::from_secs(5));
            f.head_pointer_at(p, Pointer::Down(l[2], 10.0), t + Duration::from_millis(5100));
            assert!(widths(&f, p)[2] > LEAST, "pane {p}");
            let before = widths(&f, p);
            let l = lefts(&f, p);
            f.head_pointer_at(p, Pointer::Down(l[1], 10.0), t + Duration::from_secs(20));
            f.head_pointer_at(p, Pointer::Up(l[1], 10.0), t + Duration::from_secs(20));
            f.head_pointer_at(p, Pointer::Down(l[1], 10.0), t + Duration::from_secs(22));
            f.head_pointer_at(p, Pointer::Up(l[1], 10.0), t + Duration::from_secs(22));
            assert_eq!(widths(&f, p), before);
            assert_eq!(Kept::from(Some(dir.join("conf"))).width(Col::Size, false), fitted, "kept");
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
