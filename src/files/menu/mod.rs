// The right-click menu: what it offers for what is picked, which of its rows are kept in sub-menus, and what each row does.

mod act;
mod fold;
mod lines;
mod open;
mod ring;

use std::path::PathBuf;

use super::app::{Msg, View};
use super::places::Where;
use crate::toolkit::popup::{row, Beside, Place, Popup, PopupMsg};
use crate::toolkit::widget::Node;

pub use fold::fold;
pub use lines::{column_lines, drop_lines, lines_for, more_lines, pinned_lines, side_lines, view_lines, with_ring, without};
pub(super) use act::{have_terminal, link_name, run_self};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    Open,
    OpenTab,
    OpenWindow,
    OpenWith,
    Preview,
    CopyPath,
    Cut,
    Copy,
    Paste,
    Trash,
    Delete,
    Rename,
    Pin,
    ShareOnNetwork,
    ShareWithAi,
    Properties,
    Terminal,
    AsRoot,
    Wallpaper,
    Link,
    SendTo,
    Colour,
    NewDocument,
    NewFolder,
    SelectAll,
    SelectNone,
    Invert,
    Restore,
    EmptyTrash,
    Compress,
    ExtractHere,
    ExtractTo,
    Wipe,
    Dupes,
    ShowFolder,
    TurnLeft,
    TurnRight,
    TurnOver,
    Resize,
    CompareFiles,
    InstallFont,
    Column(super::columns::Col),
    CopyHere,
    MoveHere,
    LinkHere,
    LeaveThem,
    PinUp,
    PinDown,
    RenamePin,
    RingPin,
    Earlier,
    DeletedHere,
    Sub(Kind),
    Look(View),
    Columns,
    SortBy(usize),
    GroupBy(usize),
    Flip,
    Hidden,
    Undo,
    Redo,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Select,
    Pack,
    Picture,
    Tools,
    Sort,
    Group,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub act: Act,
    pub words: &'static str,
    pub icon: &'static str,
    pub keys: &'static str,
    pub rule: bool,
    pub sub: Vec<Line>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Sel {
    pub folder: bool,
    pub image: bool,
    pub archive: bool,
    pub turnable: bool,
    pub resizable: bool,
    pub font: bool,
}

pub struct Menu {
    pub at: (f32, f32),
    pub pane: usize,
    pub items: Vec<PathBuf>,
    pub lines: Vec<Line>,
    pub hover: Option<usize>,
    pub narrow: bool,
    pub before: Vec<PathBuf>,
    pub drop: Option<Where>,
    pub beside: Beside,
}

#[derive(Clone, Debug)]
pub enum MenuMsg {
    Open,
    Pop(PopupMsg),
    Side { pinned: bool, row: usize, x: f32, y: f32 },
    View,
    New,
    More,
}

fn line(act: Act, words: &'static str, icon: &'static str, keys: &'static str) -> Line {
    Line { act, words, icon, keys, rule: false, sub: Vec::new() }
}

const SIDE_WIDTH: f32 = 232.0;

impl Menu {
    pub fn popup(&self) -> Popup {
        let longest = |lines: &[Line]| lines.iter().map(|l| (l.words.chars().count() + l.keys.chars().count()) as f32 * 7.0 + 100.0).fold(0.0, f32::max);
        let plain = |l: &Line| {
            let r = row(l.icon, l.words).keys(l.keys);
            if l.rule { r.rule() } else { r }
        };
        let rows = self.lines.iter().map(|l| {
            let r = plain(l);
            if l.sub.is_empty() { r } else { r.sub(l.sub.iter().map(plain).collect(), longest(&l.sub)) }
        });
        let pop = Popup::new(rows.collect(), Place::At(self.at.0, self.at.1));
        let wide = pop.width.max(longest(&self.lines));
        let least = pop.width;
        let mut pop = if self.narrow { pop.width(SIDE_WIDTH) } else { pop.width(wide) };
        for r in pop.rows.iter_mut().filter(|r| !r.sub.is_empty()) {
            r.sub_width = r.sub_width.max(least);
        }
        Popup { hover: self.hover, beside: self.beside, ..pop }
    }

    fn chosen(&self, i: usize, j: Option<usize>) -> Option<&Line> {
        let top = self.lines.get(i)?;
        match j {
            Some(j) => top.sub.get(j),
            None => Some(top),
        }
    }
}

pub fn over(all: Node<Msg>, menu: &Menu, room: f32) -> Node<Msg> {
    menu.popup().fits(room).over(all, |m| Msg::Menu(MenuMsg::Pop(m)))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub use tests::{every, find};

#[cfg(test)]
pub const SOURCE: &str = concat!(
    include_str!("act.rs"), include_str!("fold.rs"), include_str!("lines.rs"), include_str!("open.rs"), include_str!("ring.rs"), include_str!("mod.rs"),
);
