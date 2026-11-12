//! Opening the menu for what is picked, a button of the top bar or a sidebar

use std::path::{Path, PathBuf};

use super::act::view_msg;
use super::{column_lines, fold, lines_for, more_lines, pinned_lines, side_lines, view_lines, with_ring, Act, Line, Menu, MenuMsg, Sel};
use crate::files::app::{Files, Msg};
use crate::files::archive::Format;
use crate::files::places::{self, Where};
use crate::toolkit::popup::{self, Beside, Done, PopupMsg};
use crate::toolkit::t;
use crate::toolkit::ui::App;

impl Files {
    pub(in crate::files) fn open_menu(&mut self, p: usize) {
        if !self.rename_keep() {
            return;
        }
        let pane = &self.cols[p];
        let picked: Vec<usize> = if pane.picked.is_empty() { pane.chosen.into_iter().collect() } else { pane.picked.clone() };
        let entries: Vec<&places::Entry> = picked.iter().filter_map(|&i| pane.entries.get(i)).collect();
        let sel: Vec<Sel> = entries.iter().map(|e| Sel {
            folder: e.folder,
            image: e.kind.as_deref().is_some_and(|k| k.starts_with("image/")),
            archive: !e.folder && Format::of(&e.path).is_some(),
            turnable: !e.folder && crate::files::imaging::turnable(&e.path),
            resizable: !e.folder && crate::files::imaging::resizable(&e.path),
            font: !e.folder && crate::files::fonts::is_font(&e.path),
        }).collect();
        let items: Vec<PathBuf> = entries.iter().map(|e| e.path.clone()).collect();
        let target = match items.as_slice() {
            [one] if crate::files::reach::is_dir(one) => Some(one.clone()),
            [] => pane.at.folder().map(Path::to_path_buf),
            _ => None,
        };
        let pinned = target.is_some_and(|t| self.pinned.contains(&t));
        let lines = self.parts.fit(lines_for(&pane.at, &sel, pinned));
        let mut lines = self.earlier_lines(&pane.at, &items, lines);
        if lines.is_empty() {
            return;
        }
        let for_ring = match items.as_slice() {
            [one] => Some(one.clone()),
            [] => pane.at.folder().map(Path::to_path_buf),
            _ => None,
        };
        if let Some(on_ring) = for_ring.and_then(|path| self.ring_has(&pane.at, &path)) {
            lines = with_ring(lines, on_ring);
        }
        let lines = fold(lines);
        self.side = p;
        let pane = &mut self.cols[p];
        let before = std::mem::take(&mut pane.before).into_iter().filter_map(|i| pane.entries.get(i)).map(|e| e.path.clone()).collect();
        self.menu = Some(Menu { at: self.pointer.get(), pane: p, items, lines, hover: None, narrow: false, before, drop: None, beside: Beside::default() });
    }

    pub(in crate::files) fn columns_menu(&mut self, p: usize) {
        if !self.rename_keep() {
            return;
        }
        self.side = p;
        let lines = column_lines(&self.kept.columns());
        self.menu = Some(Menu { at: self.pointer.get(), pane: p, items: Vec::new(), lines, hover: None, narrow: false, before: Vec::new(), drop: None, beside: Beside::default() });
    }

    fn top_menu(&mut self, lines: Vec<Line>) {
        let p = self.side();
        let below = crate::files::view::top_bar_height(self);
        self.menu = Some(Menu { at: (self.room.0, below), pane: p, items: Vec::new(), lines, hover: None, narrow: false, before: Vec::new(), drop: None, beside: Beside::default() });
        self.menu_said();
    }

    pub(in crate::files) fn menu_msg(&mut self, m: MenuMsg) {
        if matches!(m, MenuMsg::Open | MenuMsg::Side { .. } | MenuMsg::View | MenuMsg::New | MenuMsg::More) {
            self.menu_told.clear();
        }
        match m {
            MenuMsg::Open => self.menu_from_keyboard(),
            MenuMsg::View => self.view_menu(),
            MenuMsg::New => {
                let at = self.pane().at.clone();
                let lines = lines_for(&at, &[], false).into_iter().filter(|l| matches!(l.act, Act::NewFolder | Act::NewDocument)).collect::<Vec<_>>();
                if !lines.is_empty() {
                    self.top_menu(lines);
                }
            }
            MenuMsg::More => {
                let lines = more_lines(!self.undo.is_empty(), !self.redo.is_empty());
                self.top_menu(lines);
            }
            MenuMsg::Side { pinned, row, x, y } => self.side_menu(pinned, row, (x, y)),
            MenuMsg::Pop(m) => self.pop_msg(m),
        }
    }

    fn menu_from_keyboard(&mut self) {
        let p = self.side();
        self.open_menu(p);
        if let Some(menu) = &mut self.menu {
            menu.hover = menu.popup().lit().hover;
        }
        self.menu_said();
    }

    fn view_menu(&mut self) {
        let p = self.side();
        let sort = self.sorted_col().and_then(|c| crate::files::columns::Col::ALL.iter().position(|k| *k == c)).map_or(0, |k| k + 1);
        let group = crate::files::groups::GroupBy::ALL.iter().position(|g| *g == self.kept.group(&self.pane().at)).unwrap_or(0);
        let lines = view_lines(self.view_of(p), sort, group, self.up);
        self.top_menu(lines);
    }

    fn side_menu(&mut self, pinned: bool, row: usize, (x, y): (f32, f32)) {
        let at = if pinned {
            self.pinned.get(row).cloned().map(Where::Folder)
        } else {
            self.place_list().into_iter().nth(row)
        };
        let Some(at) = at else { return };
        let pin = at.folder().is_some_and(|f| self.pinned.iter().any(|p| p == f));
        let mut lines = self.parts.fit(side_lines(&at, pin));
        if pinned {
            lines = pinned_lines(lines, row, self.pinned.len());
        }
        if let Some(on_ring) = at.folder().and_then(|f| self.ring_has(&at, f)) {
            lines = with_ring(lines, on_ring);
        }
        if lines.is_empty() {
            return;
        }
        let items = at.folder().map(Path::to_path_buf).into_iter().collect();
        if !self.rename_keep() {
            return;
        }
        self.menu = Some(Menu { at: (x, y), pane: self.side(), items, lines, hover: None, narrow: true, before: Vec::new(), drop: None, beside: Beside::default() });
    }

    fn pop_msg(&mut self, m: PopupMsg) {
        let Some(menu) = &mut self.menu else { return };
        let mut pop = menu.popup();
        let waited = pop.beside.due;
        let done = pop.update(m);
        let (i, j) = match done {
            Done::Open => {
                menu.hover = pop.hover;
                menu.beside = pop.beside;
                if let Some((number, _)) = pop.beside.due.filter(|due| Some(*due) != waited) {
                    self.menu_wait(number);
                }
                return self.menu_said();
            }
            Done::Closed => return self.menu = None,
            Done::Picked(i) => (i, None),
            Done::PickedIn(i, j) => (i, Some(j)),
        };
        if let (Some(Act::Column(col)), None) = (menu.lines.get(i).map(|l| l.act), j) {
            return self.tick_column(col, i);
        }
        self.row_chosen(i, j);
    }

    fn tick_column(&mut self, col: crate::files::columns::Col, i: usize) {
        let on = !self.kept.columns().contains(&col);
        if self.kept.column(col, on).is_err() {
            self.say_help(t!("folders-menu-columns-not-saved"), crate::files::help::NOT_KEPT);
        }
        let lines = column_lines(&self.kept.columns());
        if let Some(menu) = &mut self.menu {
            menu.lines = lines;
            menu.hover = Some(i);
        }
    }

    fn row_chosen(&mut self, i: usize, j: Option<usize>) {
        let Some(menu) = self.menu.take() else { return };
        let Some(act) = menu.chosen(i, j).map(|l| l.act) else { return };
        if let Some(msg) = view_msg(act) {
            return self.update(msg);
        }
        if act == Act::Invert && menu.items.is_empty() && menu.drop.is_none() {
            let pane = &mut self.cols[menu.pane];
            pane.picked = places::among(&pane.entries, &menu.before);
        }
        match menu.drop {
            Some(target) => self.drop_done(act, menu.items, target),
            None => self.do_act(act, menu.pane, menu.items),
        }
    }

    fn menu_wait(&self, number: u32) {
        let Some(post) = self.post.clone() else { return };
        let _ = std::thread::Builder::new().name("folders-menu".into()).spawn(move || {
            std::thread::sleep(popup::sub_pause());
            post.send(Msg::Menu(MenuMsg::Pop(PopupMsg::Due(number))));
        });
    }

    fn menu_said(&mut self) {
        let said = self.menu.as_ref().map(|m| m.popup().said()).unwrap_or_default();
        if said.is_empty() || said == self.menu_told {
            return;
        }
        if let Some(speak) = &self.speak {
            speak.send(said.clone());
        }
        self.menu_told = said;
    }
}
