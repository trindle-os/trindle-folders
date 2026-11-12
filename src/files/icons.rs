//! The Icons view: big icons in a grid, one drawing a row, so a long folder

use crate::toolkit::colour::paint;
use std::cell::Cell;
use std::rc::Rc;

use tiny_skia::{FillRule, FilterQuality, Pixmap, PixmapMut, PixmapPaint, Stroke, Transform};

use crate::toolkit::colour::Colour;
use crate::toolkit::text::{Align, Style};
use crate::toolkit::theme::{self, alpha, Theme};
use crate::toolkit::ui::rounded;
use crate::toolkit::widget::{drawing, Length, Node, Pen, Pointer, Rect};

pub const CELL_W: f32 = 108.0;
pub const CELL_H: f32 = 100.0;
pub const ICON_SIZES: [f32; 7] = [32.0, 48.0, 64.0, 96.0, 128.0, 192.0, 256.0];
pub const ICON_USUAL: f32 = 48.0;
const ICON_ROOM: (f32, f32) = (CELL_W - ICON_USUAL, CELL_H - ICON_USUAL);
const INSET: f32 = 4.0;
const DIM: f32 = 0.45;
pub(super) const LINE_GAP: f32 = 2.0;
pub const LINE_ICON: f32 = 20.0;
pub const LINE_PAD: f32 = 12.0;
pub const TICK: f32 = 18.0;
pub const TICK_GAP: f32 = 8.0;
const TICK_CORNER: f32 = 6.0;
const TICK_REACH: f32 = 4.0;
pub const HEAD_H: f32 = 32.0;
pub const COLUMN_MOST: f32 = 360.0;
const NAME_SLACK: f32 = 2.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub cols: usize,
    pub h: f32,
    pub lines: bool,
    pub ticks: bool,
    pub icon: f32,
    groups: Rc<[Section]>,
    flow: Option<Rc<Flow>>,
}

#[derive(Debug, PartialEq)]
struct Flow {
    rows: usize,
    top: f32,
    cols: Vec<Run>,
    heads: Vec<(usize, usize, String)>,
}

#[derive(Debug, PartialEq)]
struct Run {
    start: usize,
    len: usize,
    x: f32,
    w: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct Section {
    start: usize,
    len: usize,
    words: String,
    first_row: usize,
    top: f32,
}

pub struct Held(std::cell::RefCell<Grid>);

impl Held {
    pub fn new(grid: Grid) -> Held {
        Held(std::cell::RefCell::new(grid))
    }

    pub fn get(&self) -> Grid {
        self.0.borrow().clone()
    }

    pub fn set(&self, grid: Grid) {
        *self.0.borrow_mut() = grid;
    }
}

impl Grid {
    pub fn icons(width: f32) -> Grid {
        Grid::icons_at(width, ICON_USUAL)
    }

    pub fn icons_at(width: f32, icon: f32) -> Grid {
        Grid::icons_in(width, icon, ICON_ROOM)
    }

    pub fn icons_in(width: f32, icon: f32, room: (f32, f32)) -> Grid {
        let cols = ((width / (icon + room.0)).floor() as usize).max(1);
        Grid { cols, h: icon + room.1, lines: false, ticks: false, icon, groups: Rc::from(Vec::new()), flow: None }
    }

    pub fn lines(h: f32) -> Grid {
        Grid { cols: 1, h, lines: true, ticks: false, icon: LINE_ICON, groups: Rc::from(Vec::new()), flow: None }
    }

    pub fn columns(h: f32, tall: f32, names: &[f32], groups: &[(usize, usize, String)], ticks: bool) -> Grid {
        let grid = Grid { cols: 1, h, lines: true, ticks, icon: LINE_ICON, groups: Rc::from(Vec::new()), flow: None };
        let top = if groups.is_empty() { 0.0 } else { HEAD_H };
        let rows = (((tall - top) / h).floor() as usize).max(1);
        let around = 4.0 * LINE_PAD + LINE_ICON + grid.lead() + NAME_SLACK;
        let runs: Vec<(usize, usize, String)> = match groups.is_empty() {
            true => vec![(0, names.len(), String::new())],
            false => groups.iter().filter(|g| g.1 > 0).cloned().collect(),
        };
        let (mut cols, mut heads, mut x) = (Vec::new(), Vec::new(), 0.0);
        for (start, len, words) in runs {
            let first = cols.len();
            for at in (start..start + len).step_by(rows) {
                let end = (at + rows).min(start + len).min(names.len());
                let widest = names.get(at..end).unwrap_or(&[]).iter().fold(0.0f32, |a, b| a.max(*b));
                let w = (widest + around).ceil().min(COLUMN_MOST);
                cols.push(Run { start: at, len: end.saturating_sub(at), x, w });
                x += w;
            }
            if !words.is_empty() {
                heads.push((first, cols.len(), words));
            }
        }
        Grid { flow: Some(Rc::new(Flow { rows, top, cols, heads })), ..grid }
    }

    pub fn across(&self) -> bool {
        self.flow.is_some()
    }

    pub fn breadth(&self) -> f32 {
        self.flow.as_ref().and_then(|f| f.cols.last()).map_or(0.0, |c| c.x + c.w)
    }

    pub fn in_sight(&self, from: f32, to: f32) -> std::ops::Range<usize> {
        let Some(f) = &self.flow else { return 0..0 };
        let (a, b) = (f.cols.partition_point(|c| c.x + c.w <= from), f.cols.partition_point(|c| c.x < to));
        match (f.cols.get(a), b.checked_sub(1).and_then(|b| f.cols.get(b))) {
            (Some(a), Some(b)) if a.start <= b.start => a.start..b.start + b.len,
            _ => 0..0,
        }
    }

    pub fn runs_across(&self) -> Vec<(usize, usize, f32, f32)> {
        self.flow.as_ref().map_or(Vec::new(), |f| f.cols.iter().map(|c| (c.start, c.len, c.x, c.w)).collect())
    }

    fn run_of(f: &Flow, i: usize) -> usize {
        f.cols.partition_point(|c| c.start <= i).saturating_sub(1)
    }

    pub fn heads_across(&self) -> Vec<(f32, f32, &str)> {
        let Some(f) = &self.flow else { return Vec::new() };
        f.heads.iter().map(|(a, b, words)| {
            let (first, last) = (&f.cols[*a], &f.cols[b - 1]);
            (first.x, last.x + last.w - first.x, words.as_str())
        }).collect()
    }

    pub fn grouped(self, groups: &[(usize, usize, String)]) -> Grid {
        let (mut row, mut top) = (0, 0.0);
        let sections: Vec<Section> = groups.iter().filter(|g| g.1 > 0).map(|(start, len, words)| {
            let s = Section { start: *start, len: *len, words: words.clone(), first_row: row, top };
            let rows = len.div_ceil(self.cols);
            row += rows;
            top += HEAD_H + rows as f32 * self.h;
            s
        }).collect();
        Grid { groups: Rc::from(sections), ..self }
    }

    pub fn rows(&self, n: usize) -> usize {
        if let Some(f) = &self.flow {
            return f.rows;
        }
        match self.groups.last() {
            Some(s) => s.first_row + s.len.div_ceil(self.cols),
            None => n.div_ceil(self.cols),
        }
    }

    fn section_at(&self, row: usize) -> Option<&Section> {
        self.groups.iter().rev().find(|s| s.first_row <= row)
    }

    pub fn place(&self, i: usize) -> (usize, usize) {
        if let Some(f) = &self.flow {
            let c = Grid::run_of(f, i);
            return (i - f.cols.get(c).map_or(0, |r| r.start), c);
        }
        match self.groups.iter().rev().find(|s| s.start <= i) {
            Some(s) => (s.first_row + (i - s.start) / self.cols, (i - s.start) % self.cols),
            None => (i / self.cols, i % self.cols),
        }
    }

    pub fn row_top(&self, row: usize) -> f32 {
        if let Some(f) = &self.flow {
            return f.top + row as f32 * self.h;
        }
        match self.section_at(row) {
            Some(s) => s.top + HEAD_H + (row - s.first_row) as f32 * self.h,
            None => row as f32 * self.h,
        }
    }

    pub fn strip_top(&self, row: usize) -> f32 {
        if self.flow.is_some() {
            return 0.0;
        }
        match self.head_at(row) {
            Some(_) => self.row_top(row) - HEAD_H,
            None => self.row_top(row),
        }
    }

    pub fn head_at(&self, row: usize) -> Option<&str> {
        self.groups.iter().find(|s| s.first_row == row).map(|s| s.words.as_str())
    }

    pub fn runs(&self, n: usize) -> Vec<(usize, usize, usize)> {
        match self.groups.is_empty() {
            true => vec![(0, n, 0)],
            false => self.groups.iter().map(|s| (s.start, s.len, s.first_row)).collect(),
        }
    }

    pub fn row_under(&self, y: f32, n: usize) -> Option<usize> {
        (0..self.rows(n)).find(|r| y >= self.row_top(*r) && y < self.row_top(*r) + self.h)
    }

    pub fn row_items(&self, row: usize, n: usize) -> (usize, usize) {
        let (start, len, first_row) = match self.section_at(row) {
            Some(s) => (s.start, s.len, s.first_row),
            None => (0, n, 0),
        };
        let before = (row - first_row) * self.cols;
        (start + before, len.saturating_sub(before).min(self.cols))
    }

    pub fn step(&self, i: usize, step: super::app::Step, n: usize) -> usize {
        use super::app::Step;
        if let Some(f) = &self.flow {
            return Grid::step_across(f, i, step, n);
        }
        let (row, col) = self.place(i);
        let to = match step {
            Step::First => return 0,
            Step::Last => return n.saturating_sub(1),
            Step::Left => return i.saturating_sub(1),
            Step::Right => return (i + 1).min(n.saturating_sub(1)),
            Step::Up => match row.checked_sub(1) {
                Some(up) => up,
                None => return i,
            },
            Step::Down => row + 1,
        };
        if to >= self.rows(n) {
            return i;
        }
        let (first, count) = self.row_items(to, n);
        if count == 0 { i } else { first + col.min(count - 1) }
    }

    fn step_across(f: &Flow, i: usize, step: super::app::Step, n: usize) -> usize {
        use super::app::Step;
        let last = n.saturating_sub(1);
        let c = Grid::run_of(f, i);
        let to = match step {
            Step::First => return 0,
            Step::Last => return last,
            Step::Up => return i.saturating_sub(1),
            Step::Down => return (i + 1).min(last),
            Step::Left => match c.checked_sub(1) {
                Some(c) => c,
                None => return i,
            },
            Step::Right => c + 1,
        };
        let row = i - f.cols[c].start;
        match f.cols.get(to).filter(|r| r.len > 0) {
            Some(r) => (r.start + row.min(r.len - 1)).min(last),
            None => i,
        }
    }

    pub fn ticked(self, ticks: bool) -> Grid {
        Grid { ticks, ..self }
    }

    pub fn lead(&self) -> f32 {
        if self.ticks && self.lines { TICK + TICK_GAP } else { 0.0 }
    }

    pub fn tick_rect(&self, i: usize, width: f32) -> Rect {
        let r = self.cell(i, width);
        if self.lines {
            Rect { x: r.x + LINE_PAD, y: r.y + (r.h - TICK) / 2.0, w: TICK, h: TICK }
        } else {
            Rect { x: r.x + TICK_CORNER, y: r.y + TICK_CORNER, w: TICK, h: TICK }
        }
    }

    pub fn tick_hit(&self, x: f32, y: f32, width: f32, n: usize) -> Option<usize> {
        let i = self.hit(x, y, width, n).filter(|_| self.ticks)?;
        self.tick_rect(i, width).inset(-TICK_REACH).contains(x, y).then_some(i)
    }

    pub fn cell(&self, i: usize, width: f32) -> Rect {
        if let Some(f) = &self.flow {
            let (row, c) = self.place(i);
            let Some(r) = f.cols.get(c) else { return Rect { x: 0.0, y: 0.0, w: 0.0, h: 0.0 } };
            return Rect { x: r.x, y: f.top + row as f32 * self.h + LINE_GAP, w: r.w, h: self.h - 2.0 * LINE_GAP };
        }
        let (row, col) = self.place(i);
        let top = self.row_top(row);
        if self.lines {
            return Rect { x: 0.0, y: top + LINE_GAP, w: width, h: self.h - 2.0 * LINE_GAP };
        }
        let w = width / self.cols as f32;
        Rect { x: col as f32 * w, y: top, w, h: self.h }.inset(INSET)
    }

    pub fn hit(&self, x: f32, y: f32, width: f32, n: usize) -> Option<usize> {
        if let Some(f) = &self.flow {
            let c = f.cols.partition_point(|r| r.x <= x).checked_sub(1)?;
            let row = ((y - f.top) / self.h).floor();
            let r = &f.cols[c];
            if y < f.top || row as usize >= r.len {
                return None;
            }
            let i = r.start + row as usize;
            return (i < n && self.cell(i, width).contains(x, y)).then_some(i);
        }
        if x < 0.0 || y < 0.0 || x >= width {
            return None;
        }
        let col = (x / (width / self.cols as f32)).floor() as usize;
        let i = match self.groups.iter().rev().find(|s| s.top <= y) {
            Some(s) => {
                let down = y - s.top - HEAD_H;
                if down < 0.0 {
                    return None;
                }
                let k = (down / self.h).floor() as usize * self.cols + col;
                if k >= s.len {
                    return None;
                }
                s.start + k
            }
            None if self.groups.is_empty() => (y / self.h).floor() as usize * self.cols + col,
            None => return None,
        };
        (col < self.cols && i < n && self.cell(i, width).contains(x, y)).then_some(i)
    }

    pub fn in_band(&self, band: Rect, width: f32, n: usize) -> Vec<usize> {
        let all = match self.flow.is_some() {
            true => self.in_sight(band.x, band.x + band.w),
            false => 0..n,
        };
        all.take_while(|i| *i < n).filter(|&i| {
            let c = self.cell(i, width);
            c.x < band.x + band.w && band.x < c.x + c.w && c.y < band.y + band.h && band.y < c.y + c.h
        }).collect()
    }
}

pub fn band_rect(a: (f32, f32), b: (f32, f32)) -> Rect {
    Rect { x: a.0.min(b.0), y: a.1.min(b.1), w: (a.0 - b.0).abs(), h: (a.1 - b.1).abs() }
}

#[derive(Clone)]
pub struct Cellular {
    pub name: String,
    pub icon: &'static str,
    pub folder: bool,
    pub picked: bool,
    pub chosen: bool,
    pub hover: bool,
    pub dim: bool,
    pub tint: Option<Colour>,
    pub cols: Vec<(String, f32)>,
    pub picture: Option<Rc<Pixmap>>,
    pub emblems: Vec<String>,
    pub editing: bool,
    pub tick: bool,
}

#[derive(Clone, Default)]
pub struct Noted {
    pub name: Rc<Cell<Option<(usize, Rect)>>>,
    pub tall: Rc<Cell<Option<f32>>>,
}

impl Noted {
    const GUESS: f32 = 1.21;

    pub fn line(&self) -> f32 {
        self.tall.get().unwrap_or(theme::sheet().text.0 * Self::GUESS)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NameRoom {
    pub x: f32,
    pub w: f32,
    pub top: f32,
    pub tall: f32,
    pub above: f32,
    pub below: f32,
}

pub fn name_room(grid: &Grid, r: Rect, cols: f32, line: f32, scale: f32) -> NameRoom {
    let s = scale;
    let line = line * s;
    if !grid.lines {
        let top = r.y + 8.0 * s + (grid.icon * s).round();
        return NameRoom { x: r.x + INSET * s, w: (r.w - 2.0 * INSET * s).max(0.0), top: top + 6.0 * s, tall: line, above: top, below: r.y + r.h };
    }
    let (pad, px) = (LINE_PAD * s, (LINE_ICON * s).round());
    let x = r.x + 2.0 * pad + px + grid.lead() * s;
    let end = r.x + r.w - pad - cols * s;
    let w = (end - pad - x).max(0.0);
    NameRoom { x, w, top: (r.y + (r.h - line) / 2.0).round(), tall: line, above: r.y, below: r.y + r.h }
}

const BOX_EDGE: f32 = 2.0;

impl NameRoom {
    pub fn typing_box(&self, lines: bool) -> Rect {
        let sheet = theme::sheet();
        let mid = self.top + self.tall / 2.0;
        let h = sheet.entry_height.min(2.0 * (mid - self.above - BOX_EDGE)).min(2.0 * (self.below - BOX_EDGE - mid)).max(self.tall);
        let lead = if lines { sheet.snug_padding } else { INSET - BOX_EDGE };
        Rect { x: self.x - lead, y: mid - h / 2.0, w: self.w + 2.0 * lead, h }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn strip<M: 'static>(first: usize, cells: Vec<Cellular>, grid: Grid, height: f32, band: Option<Rect>,
                         width: Rc<Cell<f32>>, noted: Noted, tip: String, on: impl Fn(Pointer) -> M + 'static,
                         shown: impl Fn() + 'static) -> Node<M> {
    drawing((CELL_W, height), tip, move |canvas, pen| {
        shown();
        let s = pen.scale;
        let w = canvas.width() as f32 / s;
        width.set(w);
        let top = grid.strip_top(first);
        if let Some(words) = grid.head_at(first) {
            heading(canvas, pen, words, 0.0, w);
        }
        let from = grid.row_items(first, usize::MAX).0;
        paint_items(canvas, pen, &grid, w, (0.0, top), cells.iter().enumerate().map(|(k, c)| (from + k, c)), band, &noted);
    })
    .width(Length::Fill)
    .height(Length::Fixed(height))
    .on_pointer(on)
    .on_hover()
    .on_right_drag()
    .only_when_seen()
}

#[allow(clippy::too_many_arguments)]
pub fn column<M: 'static>(cells: Vec<(usize, Cellular)>, grid: Grid, place: (f32, f32), band: Option<Rect>, width: Rc<Cell<f32>>,
                          noted: Noted, tip: String, on: impl Fn(Pointer) -> M + 'static, shown: impl Fn() + 'static) -> Node<M> {
    let (x, w) = place;
    drawing((w, CELL_H), tip, move |canvas, pen| {
        shown();
        for (at, room, words) in grid.heads_across().into_iter().filter(|(at, room, _)| *at < x + w && at + room > x) {
            heading(canvas, pen, words, at - x, room);
        }
        paint_items(canvas, pen, &grid, width.get(), (x, 0.0), cells.iter().map(|(i, c)| (*i, c)), band, &noted);
    })
    .width(Length::Fixed(w))
    .height(Length::Fill)
    .on_pointer(on)
    .on_hover()
    .on_right_drag()
    .only_when_seen()
}

pub fn along(at: Pointer, by: f32) -> Pointer {
    match at {
        Pointer::Down(x, y) => Pointer::Down(x + by, y),
        Pointer::Move(x, y) => Pointer::Move(x + by, y),
        Pointer::Up(x, y) => Pointer::Up(x + by, y),
        Pointer::Hover(x, y) => Pointer::Hover(x + by, y),
        Pointer::Leave => Pointer::Leave,
        Pointer::Menu(x, y) => Pointer::Menu(x + by, y),
        Pointer::Middle(x, y) => Pointer::Middle(x + by, y),
        Pointer::RightDown(x, y) => Pointer::RightDown(x + by, y),
        Pointer::RightMove(x, y) => Pointer::RightMove(x + by, y),
        Pointer::RightUp(x, y) => Pointer::RightUp(x + by, y),
    }
}

fn heading(canvas: &mut PixmapMut, pen: &Pen, words: &str, x: f32, room: f32) {
    let s = pen.scale;
    let (size, weight) = theme::sheet().column_heading;
    let style = Style { weight, ..Style::new((size * s).round()) };
    let dots = pen.fonts.layout("\u{2026}", &style, None);
    let laid = pen.fonts.layout(words, &style, None).cut(((room - 2.0 * LINE_PAD) * s).max(0.0), &dots);
    pen.fonts.draw(canvas, &laid, ((x + LINE_PAD) * s).round(), ((HEAD_H * s - laid.height) / 2.0).round(), pen.theme.muted);
}

#[allow(clippy::too_many_arguments)]
fn paint_items<'a>(canvas: &mut PixmapMut, pen: &Pen, grid: &Grid, w: f32, from: (f32, f32), items: impl Iterator<Item = (usize, &'a Cellular)>,
                   band: Option<Rect>, noted: &Noted) {
    let s = pen.scale;
    let (left, top) = from;
    let px = |r: Rect| Rect { x: (r.x - left) * s, y: (r.y - top) * s, w: r.w * s, h: r.h * s };
    let plain = Style::new((line_size() * s).round());
    let line = pen.fonts.layout("\u{2026}", &plain, None).height / s;
    noted.tall.set(Some(line));
    for (i, c) in items {
        let r = px(grid.cell(i, w));
        lit(canvas, pen, r, c);
        let room = name_room(grid, r, c.cols.iter().map(|(_, w)| w).sum(), line, s);
        let drawn = if grid.lines { paint_line(canvas, pen, r, c, room, grid.lead()) } else { paint_cell(canvas, pen, r, c, room, grid.icon) };
        if grid.ticks && c.tick {
            let state = if c.picked { Ticked::All } else { Ticked::None };
            tick_box(canvas, pen, px(grid.tick_rect(i, w)), state);
        }
        if c.chosen {
            let at = Rect { x: drawn.x / s + left, y: drawn.y / s + top, w: drawn.w / s, h: drawn.h / s };
            noted.name.set(Some((i, at)));
        }
    }
    if let Some(b) = band {
        paint_band(canvas, pen, px(b));
    }
}

fn fill_of(t: &Theme, c: &Cellular) -> Option<Colour> {
    if c.picked { Some(t.picked) } else if c.hover || c.chosen { Some(t.hover) } else { None }
}

fn seen_on(t: &Theme, c: &Cellular) -> [Colour; 2] {
    [t.bg, t.panel].map(|ground| fill_of(t, c).map_or(ground, |fill| theme::over(fill, ground)))
}

fn lit(canvas: &mut PixmapMut, pen: &Pen, r: Rect, c: &Cellular) {
    let s = pen.scale;
    if let (Some(fill), Some(path)) = (fill_of(pen.theme, c), rounded(r, theme::corner_px(theme::Part::Row, r.w.min(r.h), s))) {
        canvas.fill_path(&path, &paint(fill), FillRule::Winding, Transform::identity(), None);
    }
}

fn inks(pen: &Pen, c: &Cellular) -> (Colour, Colour, Colour) {
    let t = pen.theme;
    let on = seen_on(t, c);
    let (words, detail) = if c.picked { (t.words_on(&on), t.quiet_on(&on)) } else { (t.fg, t.muted) };
    let ink = if c.folder { t.folder_ink(c.tint, &on) } else { words };
    match c.dim {
        true => (theme::dimmed(ink, DIM, &on, theme::AIM_EDGES), theme::dimmed(words, DIM, &on, theme::AIM_WORDS), theme::dimmed(detail, DIM, &on, theme::AIM_WORDS)),
        false => (ink, words, detail),
    }
}

pub(super) fn icon(canvas: &mut PixmapMut, pen: &Pen, name: &str, x: f32, y: f32, px: f32, ink: Colour) {
    let (x, y) = (x.round() as i32, y.round() as i32);
    if pen.icons.draw(canvas, name, x, y, px as u32, 1, ink).is_none() {
        pen.icons.draw(canvas, "text-x-generic-symbolic", x, y, px as u32, 1, ink);
    }
}

pub(super) fn picture(canvas: &mut PixmapMut, pic: &Pixmap, x: f32, y: f32, px: f32, dim: bool) {
    let (w, h) = (pic.width() as f32, pic.height() as f32);
    let k = (px / w).min(px / h);
    let (dx, dy) = (x + (px - w * k) / 2.0, y + (px - h * k) / 2.0);
    let paint = PixmapPaint { quality: FilterQuality::Bicubic, opacity: if dim { DIM } else { 1.0 }, ..PixmapPaint::default() };
    canvas.draw_pixmap(0, 0, pic.as_ref(), &paint, Transform::from_row(k, 0.0, 0.0, k, dx.round(), dy.round()), None);
}

pub(super) fn line_size() -> f32 {
    theme::sheet().text.0
}

pub fn name_width(name: &str) -> f32 {
    thread_local! {
        static FONTS: crate::toolkit::text::Fonts = crate::toolkit::text::Fonts::new();
        static KEPT: std::cell::RefCell<(f32, std::collections::HashMap<char, f32>)> = Default::default();
    }
    let size = line_size().round();
    KEPT.with_borrow_mut(|(at, kept)| {
        if *at != size {
            *at = size;
            kept.clear();
        }
        let style = Style::new(size);
        name.chars().map(|c| *kept.entry(c).or_insert_with(|| {
            FONTS.with(|f| f.layout(&format!("|{c}|"), &style, None).width - f.layout("||", &style, None).width)
        })).sum::<f32>().min(COLUMN_MOST)
    })
}

fn paint_cell(canvas: &mut PixmapMut, pen: &Pen, r: Rect, c: &Cellular, room: NameRoom, size: f32) -> Rect {
    let s = pen.scale;
    let plain = Style { align: Align::Centre, ..Style::new((theme::sheet().text.0 * s).round()) };
    let dots = pen.fonts.layout("\u{2026}", &plain, None);
    let px = (size * s).round();
    let (ink, words_ink, _) = inks(pen, c);
    let (ix, iy) = (r.x + (r.w - px) / 2.0, r.y + 8.0 * s);
    match &c.picture {
        Some(pic) => picture(canvas, pic, ix, iy, px, c.dim),
        None => icon(canvas, pen, c.icon, ix, iy, px, ink),
    }
    emblems(canvas, pen, &c.emblems, ix, iy, px);
    let words = pen.fonts.layout(&c.name, &plain, None).cut(room.w, &dots);
    let x = r.x + (r.w - words.width) / 2.0;
    if !c.editing {
        pen.fonts.draw(canvas, &words, x, room.top, words_ink);
    }
    Rect { x, y: room.top, w: words.width, h: words.height }
}

fn paint_line(canvas: &mut PixmapMut, pen: &Pen, r: Rect, c: &Cellular, name: NameRoom, lead: f32) -> Rect {
    let s = pen.scale;
    let plain = Style::new((line_size() * s).round());
    let dots = pen.fonts.layout("\u{2026}", &plain, None);
    let (ink, words_ink, detail_ink) = inks(pen, c);
    let (pad, px) = (LINE_PAD * s, (LINE_ICON * s).round());
    let ix = r.x + pad + lead * s;
    icon(canvas, pen, c.icon, ix, r.y + (r.h - px) / 2.0, px, ink);
    emblems(canvas, pen, &c.emblems, ix, r.y + (r.h - px) / 2.0, px);
    let mut end = r.x + r.w - pad;
    for (text, room) in c.cols.iter().rev() {
        end -= room * s;
        if !text.is_empty() {
            let words = pen.fonts.layout(text, &plain, None).cut((room * s - pad).max(0.0), &dots);
            pen.fonts.draw(canvas, &words, end.round(), (r.y + (r.h - words.height) / 2.0).round(), detail_ink);
        }
    }
    let (x, room) = (name.x, name.w);
    let words = pen.fonts.layout(&c.name, &plain, None).cut(room, &dots);
    if !c.editing {
        pen.fonts.draw(canvas, &words, x, name.top, words_ink);
    }
    Rect { x, y: name.top, w: words.width, h: words.height }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ticked {
    None,
    Some,
    All,
}

pub(super) fn tick_box(canvas: &mut PixmapMut, pen: &Pen, r: Rect, state: Ticked) {
    let (t, s) = (pen.theme, pen.scale);
    let edge = (1.5 * s).max(1.0);
    let inner = r.inset(edge / 2.0);
    let Some(shape) = rounded(inner, theme::corner_px(theme::Part::Tick, inner.w.min(inner.h), s)) else { return };
    if state == Ticked::None {
        canvas.fill_path(&shape, &paint(t.bg), FillRule::Winding, Transform::identity(), None);
        let stroke = Stroke { width: edge, ..Stroke::default() };
        canvas.stroke_path(&shape, &paint(t.muted), &stroke, Transform::identity(), None);
        return;
    }
    canvas.fill_path(&shape, &paint(t.accent), FillRule::Winding, Transform::identity(), None);
    let at = |fx: f32, fy: f32| (r.x + r.w * fx, r.y + r.h * fy);
    let mut mark = tiny_skia::PathBuilder::new();
    let points: &[(f32, f32)] = if state == Ticked::All { &[(0.24, 0.52), (0.43, 0.70), (0.76, 0.32)] } else { &[(0.26, 0.5), (0.74, 0.5)] };
    for (k, (fx, fy)) in points.iter().enumerate() {
        let (x, y) = at(*fx, *fy);
        if k == 0 { mark.move_to(x, y) } else { mark.line_to(x, y) }
    }
    if let Some(mark) = mark.finish() {
        let stroke = Stroke { width: (2.0 * s).max(1.5), line_cap: tiny_skia::LineCap::Round, line_join: tiny_skia::LineJoin::Round, ..Stroke::default() };
        canvas.stroke_path(&mark, &paint(t.on_accent), &stroke, Transform::identity(), None);
    }
}

fn paint_band(canvas: &mut PixmapMut, pen: &Pen, r: Rect) {
    let Some(rect) = tiny_skia::Rect::from_xywh(r.x, r.y, r.w.max(1.0), r.h.max(1.0)) else { return };
    canvas.fill_rect(rect, &paint(alpha(pen.theme.accent, 0.25)), Transform::identity(), None);
}

fn emblems(canvas: &mut PixmapMut, pen: &Pen, names: &[String], x: f32, y: f32, px: f32) {
    let e = (px * 0.45).round().max(8.0);
    for (k, name) in names.iter().take(2).enumerate() {
        let ex = x + px - e * (k as f32 + 1.0) + e * 0.25;
        pen.icons.draw(canvas, name, ex.round() as i32, (y + px - e * 0.75).round() as i32, e as u32, 1, pen.theme.fg);
    }
}

pub mod stand_in {
    pub const SUB_PACK: &str = "send-to-symbolic";
    pub const SUB_PICTURE: &str = "image-x-generic-symbolic";
    pub const SUB_TOOLS: &str = "applications-utilities-symbolic";
    pub const SETTINGS: &str = "preferences-system-symbolic";
    pub const MERGE: &str = "folder-drag-accept-symbolic";
    pub const SHRED: &str = "edit-delete-symbolic";
    pub const WIPE: &str = "edit-clear-all-symbolic";
    pub const SHARE_FOLDER: &str = "folder-publicshare-symbolic";
    pub const SHARED_MARK: &str = "emblem-shared-symbolic";
    pub const LET_CONNECT: &str = "network-transmit-receive-symbolic";
    pub const LET_THROUGH: &str = "changes-allow-symbolic";
    pub const LINK: &str = "insert-link-symbolic";
    pub const LINK_PANES: &str = "insert-link-symbolic";
    pub const AS_ROOT: &str = "dialog-password-symbolic";
    pub const FOLDER_COLOUR: &str = "color-select-symbolic";
    pub const DUPES: &str = "edit-find-symbolic";
    pub const COMPARE: &str = "view-dual-symbolic";
    pub const SAME_FOLDER: &str = "go-jump-symbolic";
    pub const SWAP_PANES: &str = "object-flip-horizontal-symbolic";
    pub const PICK_DIFFERING: &str = "edit-select-all-symbolic";
    pub const RENAME_MANY: &str = "document-edit-symbolic";
    pub const SERVER: &str = "network-server-symbolic";
    pub const EJECT: &str = "media-eject-symbolic";
    pub const PHONE: &str = "phone-symbolic";
    pub const KEY_FILE: &str = "dialog-password-symbolic";
    pub const SECURITY: &str = "security-high-symbolic";
    pub const COLUMNS: &str = "view-column-symbolic";
    pub const SEARCH_INSIDE: &str = "edit-find-replace-symbolic";
    pub const PREVIEW: &str = "view-reveal-symbolic";
    pub const UNLOCK: &str = "dialog-password-symbolic";
    pub const CHECKSUM: &str = "trindle-folders-checksum-symbolic";
    pub const TURN_LEFT: &str = "trindle-folders-rotate-left-symbolic";
    pub const TURN_RIGHT: &str = "trindle-folders-rotate-right-symbolic";
    pub const TURN_OVER: &str = "trindle-folders-rotate-half-symbolic";
    pub const FONT: &str = "trindle-folders-install-font-symbolic";
    pub const TICKED: &str = crate::common::no_tick::MARK_ICON;
    pub const SELECT_NONE: &str = "edit-clear-symbolic";
    pub const INVERT: &str = "media-playlist-repeat-symbolic";
    pub const RESIZE: &str = "trindle-folders-resize-symbolic";
    pub const PREVIEW_PANEL: &str = "sidebar-show-right-symbolic";
    pub const TWO_PANES: &str = "view-dual-symbolic";
    pub const MOUNT: &str = "media-playback-start-symbolic";
    pub const RECONNECT: &str = "view-refresh-symbolic";
    pub const FORGET_SHARE: &str = "trindle-folders-forget-symbolic";
    pub const SEVERAL: &str = "edit-select-all-symbolic";
}

#[cfg(test)]
pub fn folders_actions() -> [(&'static str, &'static str); 6] {
    use crate::toolkit::t;
    [
        (t!("folders-icons-action-font"), stand_in::FONT),
        (t!("folders-icons-action-checksum"), stand_in::CHECKSUM),
        (t!("folders-icons-action-turn-left"), stand_in::TURN_LEFT),
        (t!("folders-icons-action-turn-right"), stand_in::TURN_RIGHT),
        (t!("folders-icons-action-turn-over"), stand_in::TURN_OVER),
        (t!("folders-icons-action-resize"), stand_in::RESIZE),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_folders_action_has_its_own_trindle_icon() {
        assert_eq!(folders_actions().len(), 6);
        for (action, icon) in folders_actions() {
            assert!(icon.starts_with("trindle-folders-"), "{action} uses {icon}");
        }
    }

    #[test]
    fn compact_names_run_down_and_then_along() {
        use crate::files::app::Step;
        let around = 4.0 * LINE_PAD + LINE_ICON + NAME_SLACK;
        let names = [10.0, 50.0, 20.0, 30.0, 40.0, 60.0, 900.0];
        let g = Grid::columns(20.0, 100.0, &names, &[], false);
        assert!(g.across() && g.lines);
        assert_eq!(g.rows(7), 5);
        let (a, b) = (g.cell(0, 300.0), g.cell(5, 300.0));
        assert_eq!((a.x, a.y, a.w), (0.0, LINE_GAP, 50.0 + around), "as wide as its longest");
        assert_eq!((b.x, b.y, b.w), (a.w, LINE_GAP, COLUMN_MOST), "the next column, never wider than the most");
        assert_eq!(g.cell(4, 300.0).y, 80.0 + LINE_GAP);
        assert_eq!(g.breadth(), a.w + COLUMN_MOST);
        assert_eq!(g.hit(a.w + 5.0, 25.0, 300.0, 7), Some(6));
        assert_eq!(g.hit(a.w + 5.0, 45.0, 300.0, 7), None, "under the last name");
        assert_eq!(g.hit(g.breadth() + 5.0, 5.0, 300.0, 7), None, "after the last column");
        assert_eq!((g.step(1, Step::Down, 7), g.step(5, Step::Up, 7), g.step(4, Step::Down, 7)), (2, 4, 5));
        assert_eq!((g.step(1, Step::Right, 7), g.step(3, Step::Right, 7), g.step(6, Step::Left, 7)), (6, 6, 1));
        assert_eq!((g.step(0, Step::Left, 7), g.step(6, Step::Right, 7)), (0, 6), "nowhere further");
        assert_eq!(g.in_sight(0.0, 10.0), 0..5);
        assert_eq!(g.in_sight(a.w - 1.0, a.w + 1.0), 0..7);
        assert_eq!(g.in_sight(g.breadth(), g.breadth() + 50.0), 0..0);
        assert_eq!(g.in_band(band_rect((5.0, 30.0), (a.w + 5.0, 50.0)), 300.0, 7), [1, 2, 6]);
        let groups = [(0, 2, "Folders, 2".to_string()), (2, 5, "Files, 5".to_string())];
        let g = Grid::columns(20.0, 100.0 + HEAD_H, &names, &groups, false);
        assert_eq!((g.place(1), g.place(2), g.place(6), g.place(7 - 1)), ((1, 0), (0, 1), (4, 1), (4, 1)));
        assert_eq!(g.cell(2, 300.0).y, HEAD_H + LINE_GAP);
        let heads = g.heads_across();
        assert_eq!(heads.iter().map(|h| h.2).collect::<Vec<_>>(), ["Folders, 2", "Files, 5"]);
        assert_eq!((heads[1].0, heads[1].1), (g.cell(2, 300.0).x, COLUMN_MOST));
        assert_eq!(g.hit(5.0, 10.0, 300.0, 7), None, "a heading is no item");
        let t = Grid::columns(20.0, 100.0, &names, &[], true);
        assert_eq!(t.cell(0, 300.0).w, 50.0 + around + TICK + TICK_GAP);
    }

    #[test]
    fn a_name_is_never_wider_than_its_letters_added_up() {
        let fonts = crate::toolkit::text::Fonts::new();
        let style = Style::new(line_size().round());
        for name in ["AVATAR To Wa.txt", "office fi fl ffi.pdf", "Ünïcödé naïve café.odt", "日本語のファイル.txt", "مرحبا بالعالم.txt", "Привет мир.doc", "a b  c", "Wwwwwwwwww"] {
            let whole = fonts.layout(name, &style, None).width;
            assert!(name_width(name) >= whole, "{name}: {} against {whole}", name_width(name));
        }
    }

    #[test]
    fn cells_fill_the_width() {
        assert_eq!(Grid::icons(50.0).cols, 1);
        assert_eq!(Grid::icons(440.0).cols, 4);
        let big = Grid::icons_at(440.0, 128.0);
        assert_eq!((big.cols, big.h), (2, 128.0 + CELL_H - ICON_USUAL));
        let small = Grid::icons_at(440.0, 32.0);
        assert_eq!((small.cols, small.h), (4, 32.0 + CELL_H - ICON_USUAL));
        let usual = Grid::icons(440.0);
        assert_eq!((usual.h, usual.icon), (CELL_H, ICON_USUAL));
        let name = |g: &Grid| name_room(g, g.cell(0, 440.0), 0.0, 19.0, 1.0);
        assert_eq!(name(&big).top - name(&usual).top, 128.0 - ICON_USUAL);
        assert!(ICON_SIZES.windows(2).all(|w| w[0] < w[1]) && ICON_SIZES.contains(&ICON_USUAL));
        let c = Grid::icons(440.0).cell(5, 440.0);
        assert_eq!((c.x, c.y), (110.0 + INSET, CELL_H + INSET));
        let l = Grid::lines(32.0).cell(2, 300.0);
        assert_eq!((l.x, l.w, l.y), (0.0, 300.0, 64.0 + LINE_GAP));
    }

    #[test]
    fn a_point_finds_its_item() {
        let g = Grid::icons(440.0);
        assert_eq!(g.hit(20.0, 20.0, 440.0, 10), Some(0));
        assert_eq!(g.hit(120.0, 120.0, 440.0, 10), Some(5));
        assert_eq!(g.hit(110.0, 20.0, 440.0, 10), None);
        assert_eq!(g.hit(250.0, 220.0, 440.0, 10), None);
        let l = Grid::lines(32.0);
        assert_eq!(l.hit(290.0, 40.0, 300.0, 3), Some(1));
        assert_eq!(l.hit(10.0, 100.0, 300.0, 3), None);
    }

    #[test]
    fn a_tick_box_sits_at_a_lines_start_and_on_an_icons_corner() {
        let l = Grid::lines(32.0).ticked(true);
        assert_eq!(l.lead(), TICK + TICK_GAP);
        assert_eq!(Grid::lines(32.0).lead(), 0.0);
        assert_eq!(Grid::icons(440.0).ticked(true).lead(), 0.0);
        let t = l.tick_rect(2, 300.0);
        assert_eq!((t.x, t.y, t.w), (LINE_PAD, 64.0 + (32.0 - TICK) / 2.0, TICK));
        assert_eq!(l.tick_hit(LINE_PAD + 5.0, 70.0, 300.0, 5), Some(2));
        assert_eq!(l.tick_hit(LINE_PAD + TICK + TICK_REACH + 2.0, 70.0, 300.0, 5), None, "the rest of the line is not the box");
        assert_eq!(Grid::lines(32.0).tick_hit(LINE_PAD + 5.0, 70.0, 300.0, 5), None, "no boxes, nothing to hit");
        let g = Grid::icons(440.0).ticked(true);
        let c = g.cell(5, 440.0);
        assert_eq!(g.tick_hit(c.x + TICK_CORNER + 4.0, c.y + TICK_CORNER + 4.0, 440.0, 10), Some(5));
        assert_eq!(g.tick_hit(c.x + c.w / 2.0, c.y + c.h / 2.0, 440.0, 10), None);
        let plain = name_room(&Grid::lines(32.0), Grid::lines(32.0).cell(0, 300.0), 0.0, 19.0, 1.0);
        let ticked = name_room(&l, l.cell(0, 300.0), 0.0, 19.0, 1.0);
        assert_eq!((ticked.x - plain.x, plain.w - ticked.w), (TICK + TICK_GAP, TICK + TICK_GAP));
    }

    #[test]
    fn groups_have_headings_that_no_click_box_or_arrow_lands_on() {
        use super::super::app::Step;
        let groups = [(0, 5, "Pictures, 5".to_string()), (5, 2, "Music, 2".to_string())];
        let g = Grid::icons(440.0).grouped(&groups);
        assert_eq!(g.rows(7), 3);
        assert_eq!((g.place(4), g.place(5), g.place(6)), ((1, 0), (2, 0), (2, 1)));
        assert_eq!((g.head_at(0), g.head_at(1), g.head_at(2)), (Some("Pictures, 5"), None, Some("Music, 2")));
        assert_eq!((g.row_top(0), g.row_top(1), g.row_top(2)), (HEAD_H, HEAD_H + CELL_H, 2.0 * HEAD_H + 2.0 * CELL_H));
        assert_eq!((g.strip_top(0), g.strip_top(1), g.strip_top(2)), (0.0, HEAD_H + CELL_H, HEAD_H + 2.0 * CELL_H));
        assert_eq!(g.row_top(3), 2.0 * HEAD_H + 3.0 * CELL_H, "where the items end");
        let c = g.cell(5, 440.0);
        assert_eq!((c.x, c.y), (INSET, 2.0 * HEAD_H + 2.0 * CELL_H + INSET));
        assert_eq!(g.hit(20.0, 10.0, 440.0, 7), None);
        assert_eq!(g.hit(20.0, HEAD_H + 20.0, 440.0, 7), Some(0));
        assert_eq!(g.hit(20.0, HEAD_H + 2.0 * CELL_H + 10.0, 440.0, 7), None, "the second heading");
        assert_eq!(g.hit(130.0, 2.0 * HEAD_H + 2.0 * CELL_H + 20.0, 440.0, 7), Some(6));
        assert_eq!(g.hit(130.0, HEAD_H + CELL_H + 20.0, 440.0, 7), None, "the first group's last row is one short");
        assert_eq!(g.in_band(band_rect((10.0, HEAD_H + CELL_H + 30.0), (60.0, 2.0 * HEAD_H + 2.0 * CELL_H + 30.0)), 440.0, 7), [4, 5]);
        assert_eq!(g.step(1, Step::Down, 7), 4, "the row under has one thing");
        assert_eq!(g.step(4, Step::Down, 7), 5);
        assert_eq!(g.step(6, Step::Up, 7), 4);
        assert_eq!(g.step(5, Step::Up, 7), 4);
        assert_eq!(g.step(6, Step::Down, 7), 6);
        assert_eq!(g.step(0, Step::Up, 7), 0);
        assert_eq!((g.step(4, Step::Right, 7), g.step(5, Step::Left, 7)), (5, 4));
        assert_eq!((g.step(3, Step::First, 7), g.step(3, Step::Last, 7)), (0, 6));
        let l = Grid::lines(30.0).grouped(&groups);
        assert_eq!((l.cell(0, 300.0).y, l.cell(5, 300.0).y), (HEAD_H + LINE_GAP, 2.0 * HEAD_H + 150.0 + LINE_GAP));
        assert_eq!((l.hit(9.0, 5.0, 300.0, 7), l.hit(9.0, HEAD_H + 155.0, 300.0, 7), l.hit(9.0, 2.0 * HEAD_H + 155.0, 300.0, 7)), (None, None, Some(5)));
        assert_eq!((l.step(4, Step::Down, 7), l.step(5, Step::Up, 7)), (5, 4));
        assert_eq!((g.row_under(HEAD_H + 1.0, 7), g.row_under(HEAD_H + CELL_H + 99.0, 7), g.row_under(10.0, 7), g.row_under(5000.0, 7)), (Some(0), Some(1), None, None));
        assert_eq!((g.row_items(1, 7), g.row_items(2, 7)), ((4, 1), (5, 2)));
        let plain = Grid::icons(440.0);
        assert_eq!(plain.clone().grouped(&[]), plain);
        assert_eq!((plain.rows(6), plain.row_top(2), plain.strip_top(2), plain.head_at(0)), (2, 2.0 * CELL_H, 2.0 * CELL_H, None));
        assert_eq!((plain.step(0, Step::Down, 6), plain.step(4, Step::Up, 6), plain.step(5, Step::Down, 6)), (4, 0, 5));
    }

    #[test]
    fn a_box_picks_what_it_touches() {
        let b = band_rect((200.0, 150.0), (20.0, 50.0));
        assert_eq!((b.x, b.y, b.w, b.h), (20.0, 50.0, 180.0, 100.0));
        assert_eq!(Grid::icons(440.0).in_band(b, 440.0, 10), [0, 1, 4, 5]);
        assert_eq!(Grid::icons(440.0).in_band(band_rect((1.0, 1.0), (2.0, 2.0)), 440.0, 10), Vec::<usize>::new());
        assert_eq!(Grid::lines(32.0).in_band(band_rect((5.0, 100.0), (8.0, 40.0)), 300.0, 10), [1, 2, 3]);
    }
}
