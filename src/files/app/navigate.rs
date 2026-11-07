// Taking a pane somewhere: Back and Forward, folders opened, files started, and the box to type a place in.

use std::path::{Path, PathBuf};

use super::*;

#[derive(Clone, Copy)]
enum Move<'a> {
    Into(&'a std::ffi::OsStr),
    Up(&'a std::ffi::OsStr),
}

impl Files {
    pub(super) fn go_msg(&mut self, m: Msg) {
        match m {
            Msg::Part(p, i) => {
                let parts = places::crumbs(&self.cols[p].at, &self.home);
                if let Some((_, at)) = parts.into_iter().nth(i) {
                    self.go(p, at);
                }
            }
            Msg::Place(i) => {
                if let Some(at) = self.place_list().into_iter().nth(i) {
                    self.go(self.side(), at);
                }
            }
            Msg::Pinned(i) => {
                if let Some(f) = self.pinned.get(i).cloned() {
                    if !self.reconnect_pin(&f) {
                        self.go(self.side(), Where::Folder(f));
                    }
                }
            }
            Msg::Travel(p, forward) => {
                self.side = p;
                self.travel(p, forward);
            }
            Msg::Back => self.travel(self.side(), false),
            Msg::Forward => self.travel(self.side(), true),
            Msg::Chooser(m) => {
                if self.choose(m) {
                    for p in 0..2 {
                        self.load(self.cols[p].tab, p);
                    }
                }
            }
            Msg::Reload => {
                let p = self.side();
                self.load(self.cols[p].tab, p);
            }
            Msg::GiveUp => self.give_up(self.side()),
            Msg::Home => {
                let home = self.home.clone();
                self.go(self.side(), Where::Folder(home));
            }
            Msg::Up => self.go_up(),
            Msg::Enter => {
                let p = self.side();
                if let Some(i) = self.pane().chosen {
                    self.open(p, i);
                }
            }
            Msg::Add(folders) => self.add_folders(folders),
            Msg::Reveal(items, props) => self.reveal(items, props),
            Msg::Props => self.act_on_picked(crate::files::menu::Act::Properties),
            Msg::Terminal => self.act_on_picked(crate::files::menu::Act::Terminal),
            _ => unreachable!("not about going places"),
        }
    }

    fn go_up(&mut self) {
        let p = self.side();
        if let Some(up) = self.pane().at.up() {
            let was = self.pane().at.folder().map(Path::to_path_buf);
            let left = was.as_ref().and_then(|d| d.file_name()).map(|n| n.to_os_string());
            self.go(p, up);
            if let Some(was) = was.as_ref().filter(|_| !self.choose_gives_folders()) {
                self.choose_path(p, was);
            }
            if let Some(left) = left {
                self.follow(p, Move::Up(&left));
            }
        }
    }

    fn add_folders(&mut self, folders: Vec<PathBuf>) {
        let mut replace = !folders.is_empty() && self.give_way();
        for f in folders {
            if std::mem::take(&mut replace) {
                self.arrive(self.side, crate::files::inside::place(f));
            } else {
                self.new_tab(self.side, Some(crate::files::inside::place(f)));
            }
        }
    }

    fn act_on_picked(&mut self, act: crate::files::menu::Act) {
        let p = self.side();
        let items = self.picked_paths(p);
        self.do_act(act, p, items);
    }

    pub(super) fn locate_msg(&mut self, m: Msg) {
        match m {
            Msg::Locate => self.locate_start(self.side(), false),
            Msg::LocateLine(p) => {
                if p < self.cols.len() && (self.two || p == self.side) {
                    self.side = p;
                    self.locate_start(p, true);
                }
            }
            Msg::LocateText(t) => {
                if let Some((_, text)) = &mut self.locate {
                    *text = t;
                }
            }
            Msg::LocateDone => self.located(),
            Msg::LocateStop => self.locate = None,
            _ => unreachable!("not about the place box"),
        }
    }

    pub(in crate::files) fn go(&mut self, p: usize, at: Where) {
        let pane = self.pane_mut(p);
        if pane.at != at {
            let was = pane.at.clone();
            pane.back.push(was);
            if pane.back.len() > HISTORY {
                pane.back.remove(0);
            }
            pane.ahead.clear();
        }
        self.arrive(p, at);
    }

    fn travel(&mut self, p: usize, forward: bool) {
        let Some((was, now)) = self.retrace(p, forward) else { return };
        if !self.kept.linked || !self.two {
            return;
        }
        let o = 1 - p;
        let (Where::Folder(from), Where::Folder(to), Where::Folder(there)) = (&was, &now, &self.cols[o].at) else { return };
        match places::mirrored(there, from, to) {
            Some(to) => {
                self.link_note = None;
                let other = &self.cols[o];
                let next = if forward { other.ahead.last() } else { other.back.last() };
                if next == Some(&Where::Folder(to.clone())) {
                    self.retrace(o, forward);
                } else {
                    self.go(o, Where::Folder(to));
                }
            }
            None => self.link_note = Some(t!("folders-app-link-no-folder").into()),
        }
        self.side = p;
    }

    pub(in crate::files) fn retrace(&mut self, p: usize, forward: bool) -> Option<(Where, Where)> {
        let pane = self.pane_mut(p);
        let (from, to) = if forward { (&mut pane.ahead, &mut pane.back) } else { (&mut pane.back, &mut pane.ahead) };
        let at = from.pop()?;
        let was = pane.at.clone();
        to.push(was.clone());
        self.arrive(p, at.clone());
        Some((was, at))
    }

    pub(in crate::files) fn arrive(&mut self, p: usize, at: Where) {
        let tab = self.cols[p].tab;
        self.locate = None;
        if at == Where::Network {
            self.find_neighbours();
        }
        let carried = self.find.carry.take().unwrap_or_default();
        let pane = self.pane_mut(p);
        if pane.at != at || matches!(at, Where::Remote(_) | Where::Inside(..) | Where::Found(..)) {
            pane.entries.clear();
            pane.all.clear();
        }
        pane.at = at;
        pane.clear();
        pane.filter.clear();
        pane.narrow = carried;
        pane.hunt = None;
        self.side = p;
        if self.find.open == Some(p) {
            self.find.open = None;
        }
        self.sizes_moved();
        self.said.clear();
        self.bad = false;
        self.load(tab, p);
        self.pictures_moved();
        self.hunt_start(p);
        self.renaming = None;
        self.shown.forget();
        self.below_fetch(p);
        self.computer_wanted();
    }

    fn follow(&mut self, p: usize, way: Move) {
        if !self.kept.linked || !self.two {
            return;
        }
        let o = 1 - p;
        let Where::Folder(there) = &self.cols[o].at else { return };
        let to = match way {
            Move::Into(name) if crate::files::reach::is_dir(&there.join(name)) => there.join(name),
            Move::Into(name) => {
                self.link_note = Some(t!("folders-app-link-no-folder-named", name = &name.to_string_lossy().to_string()));
                return;
            }
            Move::Up(left) => match there.parent() {
                Some(up) if there.file_name() == Some(left) => up.to_path_buf(),
                _ => return,
            },
        };
        self.link_note = None;
        self.go(o, Where::Folder(to));
        self.side = p;
    }

    pub(in crate::files) fn open(&mut self, p: usize, i: usize) {
        let Some(e) = self.cols[p].entries.get(i).cloned() else { return };
        if let Where::Inside(..) = self.cols[p].at {
            self.inside_open(p, vec![e.path]);
        } else if !e.folder && crate::files::inside::opens(&e.path) {
            self.go(p, Where::Inside(e.path, String::new()));
        } else if e.folder && crate::files::far::is_far(&e.path) {
            self.go(p, Where::Remote(e.path.to_string_lossy().into_owned()));
        } else if e.folder {
            let name = e.path.file_name().map(|n| n.to_os_string());
            self.go(p, Where::Folder(e.path));
            if let Some(name) = name {
                self.follow(p, Move::Into(&name));
            }
        } else if crate::files::far::is_far(&e.path) {
            self.far_open(&e.path);
        } else if !self.choose_opened(&e.path) {
            let _ = self.open_file(&e.path);
        }
    }

    pub(in crate::files) fn open_file(&mut self, path: &Path) -> bool {
        let kind = self.mime.of_path(path);
        let opener = crate::common::apps::opener_for(&kind)
            .or_else(|| kind.starts_with("text/").then(|| crate::common::apps::opener_for("text/plain")).flatten());
        match opener {
            Some(app) if start(&app, path) => {
                self.used(path);
                true
            }
            Some(app) => {
                self.say_help(t!("folders-app-not-started", name = &app.name), crate::files::help::NOT_STARTED);
                false
            }
            None => {
                self.dialog = Some(crate::files::dialogs::Dialog::OpenWith(self.open_with(path)));
                false
            }
        }
    }

    pub(in crate::files) fn used(&mut self, file: &Path) {
        let kind = self.mime.of_path(file);
        if places::note_recent(file, &kind).is_err() {
            return;
        }
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                if self.cols[p].tabs[t].at == Where::Recent {
                    self.load(t, p);
                }
            }
        }
    }

    fn locate_start(&mut self, p: usize, in_line: bool) {
        let now = match &self.cols[p].at {
            Where::Folder(f) => {
                let f = f.to_string_lossy();
                if f.ends_with('/') { f.into_owned() } else { format!("{f}/") }
            }
            Where::Remote(a) => a.clone(),
            _ => String::new(),
        };
        self.locate = Some((p, now));
        self.locate_line = in_line;
    }

    pub(super) fn locate_follow(&mut self) {
        let Some((p, _)) = &self.locate else { return };
        let shown = if self.locate_line { self.shown_pane(*p) } else { *p == self.side() };
        if !shown {
            self.locate = None;
        }
    }

    fn located(&mut self) {
        let Some((p, text)) = self.locate.take() else { return };
        let here = self.cols[p].at.folder().map_or_else(|| self.home.clone(), Path::to_path_buf);
        match places::typed_place(&text, &here, &self.home) {
            Some((at, file)) => {
                self.go(p, at);
                if let Some(file) = file {
                    self.choose_path(p, &file);
                }
            }
            None => self.say_help(t!("folders-app-nothing-at", text = text.trim()), crate::files::help::PLACE_UNREAD),
        }
    }
}

fn start(app: &crate::common::apps::Entry, path: &Path) -> bool {
    #[cfg(not(test))]
    return crate::common::apps::launch_boxed(app, &[path.to_string_lossy().into_owned()]);
    #[cfg(test)]
    {
        let _ = (app, path);
        true
    }
}
