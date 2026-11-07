// Tabs, the two panes, and opening places in them.

use std::path::{Path, PathBuf};

use super::*;

impl Files {
    pub(super) fn tab_msg(&mut self, m: Msg) {
        match m {
            Msg::Tab(p, i) => {
                self.side = p;
                self.cols[p].tab = i.min(self.cols[p].tabs.len() - 1);
            }
            Msg::NewTab(p) => self.new_tab(p.unwrap_or(self.side), None),
            Msg::CloseTab(p) => self.close_tab(p),
            Msg::NewWindow => self.new_window(),
            Msg::Middle(crate::files::act::Land::Tab(p, i)) => self.shut_tab(p, i),
            Msg::Middle(land) => {
                if let Some(at) = self.land_place(land) {
                    self.tab_for(self.side(), at, self.mods.get().shift);
                }
            }
            Msg::PartTab(p, i) => {
                if let Some((_, at)) = places::crumbs(&self.cols[p].at, &self.home).into_iter().nth(i) {
                    self.tab_for(p, at, self.mods.get().shift);
                }
            }
            Msg::NextTab(on) => {
                let col = &mut self.cols[self.side];
                let n = col.tabs.len();
                col.tab = if on { (col.tab + 1) % n } else { (col.tab + n - 1) % n };
            }
            Msg::OtherPane => {
                if self.two {
                    self.side = 1 - self.side();
                }
            }
            Msg::Panes => self.show_panes(!self.two),
            Msg::PanesAre(two) => self.show_panes(two),
            Msg::Link => self.link_panes(),
            _ => unreachable!("not about tabs"),
        }
    }

    fn close_tab(&mut self, p: Option<usize>) {
        let keyed = p.is_none();
        let p = p.unwrap_or(self.side);
        let tab = self.cols[p].tab;
        if self.cols[p].tabs.len() > 1 {
            self.shut_tab(p, tab);
            self.side = p;
        } else if keyed && (!self.two || self.cols[1 - p].tabs.len() == 1) {
            self.closes += 1;
            if let Some(hook) = &self.close_hook {
                hook();
            }
        } else if keyed {
            self.say(t!("folders-app-only-tab"), false);
        }
    }

    fn link_panes(&mut self) {
        self.link_note = None;
        let on = !self.kept.linked;
        if self.kept.link(on).is_err() {
            self.say_help(t!("folders-app-link-not-kept"), crate::files::help::NOT_KEPT);
        }
    }

    pub(in crate::files) fn new_tab(&mut self, p: usize, at: Option<Where>) {
        let at = at.unwrap_or_else(|| self.cols[p].at.clone());
        self.cols[p].open(at);
        self.side = p;
        self.locate = None;
        let t = self.cols[p].tab;
        self.load(t, p);
    }

    pub(in crate::files) fn tab_for(&mut self, p: usize, at: Where, front: bool) {
        if front {
            return self.new_tab(p, Some(at));
        }
        if !self.rename_keep() {
            return;
        }
        let name = at.name(&self.home);
        self.cols[p].open_behind(at);
        let t = self.cols[p].tab + 1;
        self.load(t, p);
        self.say(t!("folders-app-opened-tab", name = &name), false);
    }

    pub(in crate::files) fn tab_place(&self, p: usize, i: usize) -> Option<Where> {
        let pane = &self.cols[p];
        crate::files::below::place_of(&pane.at, pane.entries.get(i)?)
    }

    pub(in crate::files) fn shut_tab(&mut self, p: usize, t: usize) {
        let col = &mut self.cols[p];
        if col.tabs.len() < 2 || t >= col.tabs.len() {
            return;
        }
        col.tabs.remove(t);
        if t < col.tab {
            col.tab -= 1;
        }
        col.tab = col.tab.min(col.tabs.len() - 1);
        self.locate = None;
        self.renaming = None;
    }

    fn new_window(&mut self) {
        let here = self.pane().at.folder().map(|f| f.to_string_lossy().into_owned());
        let mut args = vec![crate::files::NEW_WINDOW];
        args.extend(here.as_deref());
        if self.post.is_some() && !crate::files::menu::run_self("folders", &args) {
            self.say_help(t!("folders-app-no-window"), crate::files::help::NOT_STARTED);
        }
    }

    pub(in crate::files) fn show_panes(&mut self, two: bool) {
        if two == self.two {
            return;
        }
        self.two = two;
        let guess = if two { GUESS_TWO } else { GUESS_ONE };
        for pane in self.cols.iter_mut().flat_map(|c| c.tabs.iter_mut()) {
            pane.width.set(guess);
        }
        if self.kept.panes(two).is_err() {
            self.say_help(t!("folders-app-panes-not-kept"), crate::files::help::NOT_KEPT);
        }
    }

    pub(in crate::files) fn shown_pane(&self, p: usize) -> bool {
        self.two || p == self.side()
    }

    pub(in crate::files) fn presses_activate(&self) -> bool {
        self.two && self.menu.is_none() && self.dialog.is_none() && self.confirm.is_none() && self.dropping.is_none()
    }

    pub(in crate::files) fn pane_at(&self, wx: f32) -> usize {
        if !self.two || wx < crate::files::act::panes_left() {
            self.side
        } else if wx > crate::files::act::panes_left() + self.cols[0].width.get() + crate::files::act::between() / 2.0 {
            1
        } else {
            0
        }
    }

    pub(super) fn give_way(&mut self) -> bool {
        std::mem::take(&mut self.for_a_call) && self.cols[0].tabs.len() == 1
    }

    fn open_asked(&mut self, at: Where, replace: &mut bool) {
        if std::mem::take(replace) {
            self.arrive(0, at);
        } else {
            self.new_tab(0, Some(at));
        }
    }

    pub(super) fn reveal(&mut self, items: Vec<PathBuf>, props: bool) {
        let mut folders: Vec<(PathBuf, Vec<PathBuf>)> = Vec::new();
        for item in items {
            let Some(folder) = item.parent().map(Path::to_path_buf) else { continue };
            match folders.iter_mut().find(|(f, _)| *f == folder) {
                Some((_, them)) => them.push(item),
                None => folders.push((folder, vec![item])),
            }
        }
        if folders.is_empty() {
            return;
        }
        let mut replace = self.give_way();
        self.side = 0;
        for (folder, them) in folders {
            self.open_asked(Where::Folder(folder), &mut replace);
            let pane = self.pane_mut(0);
            let rows = places::among(&pane.entries, &them);
            if let Some(&first) = rows.first() {
                pane.pick(first, false);
                pane.picked = rows;
                self.seek(0);
            }
            if props {
                self.do_act(crate::files::menu::Act::Properties, 0, them);
            }
        }
    }
}
