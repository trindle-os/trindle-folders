// Letting things go: where they may go, and what is done there.

use std::path::{Path, PathBuf};

use crate::files::app::Files;
use crate::files::far;
use crate::files::inside;
use crate::files::places::{self, Where};
use crate::files::work::{self, Kind, Step};
use crate::toolkit::t;
use super::panes_left;
use super::steps::same_drive;
use super::types::{Drag, Land, Way};
use super::text::names;

impl Files {
    pub fn drop_places(&self) -> Vec<(String, Where)> {
        let mut out = vec![(t!("folders-act-place-home").to_string(), Where::Folder(self.home.clone()))];
        for f in &self.pinned {
            out.push((self.pin_names.get(f).cloned().unwrap_or_else(|| Where::Folder(f.clone()).name(&self.home)), Where::Folder(f.clone())));
        }
        out.push((t!("folders-act-place-trash").to_string(), Where::Trash));
        out
    }

    pub(in crate::files) fn land_place(&self, land: Land) -> Option<Where> {
        match land {
            Land::Home => Some(Where::Folder(self.home.clone())),
            Land::Pinned(i) => self.pinned.get(i).cloned().map(Where::Folder),
            Land::Place(i) => self.place_list().into_iter().nth(i),
            Land::Drive(i) => self.drives.list.get(i).and_then(|v| v.mounted()).map(|m| Where::Folder(PathBuf::from(m))),
            Land::Share(i) => self.share_places().get(i).map(|m| Where::Folder(m.at.clone())),
            Land::Tab(p, i) => self.cols.get(p).and_then(|c| c.tabs.get(i)).map(|t| t.at.clone()),
            Land::Part(p, i) => self.pill_place(p, i).filter(open_to_write),
        }
    }

    pub(in crate::files) fn pill_place(&self, p: usize, j: usize) -> Option<Where> {
        let pane = self.cols.get(p)?;
        if matches!(pane.at, Where::Inside(..)) {
            return None;
        }
        let at = places::crumbs(&pane.at, &self.home).into_iter().nth(j)?.1;
        let here = match &at {
            Where::Folder(dir) if !self.earlier_holds(dir) => dir.clone(),
            Where::Remote(a) => PathBuf::from(a),
            _ => return None,
        };
        if let Some(d) = &self.drag {
            if d.items.iter().any(|i| here.starts_with(i)) || (!d.right && d.items.iter().all(|i| i.parent() == Some(here.as_path()))) {
                return None;
            }
        }
        Some(at)
    }

    fn ask_drop(&mut self, items: Vec<PathBuf>, target: Where) {
        let lines = crate::files::menu::drop_lines(&items, &target);
        if lines.is_empty() {
            return self.say(t!("folders-act-drop-only"), false);
        }
        self.said.clear();
        self.menu = Some(crate::files::menu::Menu { at: self.pointer.get(), pane: self.side(), items, lines, hover: None, narrow: false, drop: Some(target), before: Vec::new(), beside: Default::default() });
    }

    pub(in crate::files) fn drop_done(&mut self, act: crate::files::menu::Act, items: Vec<PathBuf>, target: Where) {
        use crate::files::menu::Act;
        match (act, target) {
            (Act::CopyHere, target) => self.drop_into(items, target, Way::Copy),
            (Act::MoveHere, target) => self.drop_into(items, target, Way::Move),
            (Act::LinkHere, Where::Folder(dir)) if self.earlier_holds(&dir) => self.say(crate::files::earlier::shut(), false),
            (Act::LinkHere, Where::Folder(dir)) => self.link_into(&items, &dir),
            _ => {}
        }
    }

    fn link_into(&mut self, items: &[PathBuf], dir: &Path) {
        let (mut made, mut failed) = (Vec::new(), Vec::new());
        if !crate::files::reach::answers(dir) {
            return self.say(crate::files::reach::not_answering(), false);
        }
        for item in items {
            let link = crate::files::menu::link_name(dir, item);
            match std::os::unix::fs::symlink(item, &link) {
                Ok(()) => made.push(link),
                Err(_) => failed.push(work::name_of(item)),
            }
        }
        let what = match made.as_slice() {
            [one] => work::name_of(one),
            many => t!("folders-act-links", n = many.len()),
        };
        self.remember(t!("folders-act-undo-making", name = &what), made.iter().cloned().map(Step::Remove).collect());
        if failed.is_empty() {
            self.say(t!("folders-act-made", what = &what), false);
        } else {
            self.say_help(t!("folders-act-links-failed", n = failed.len(), names = &names(&failed)), crate::files::help::MAKE_FAILED);
        }
        self.reload_all();
    }

    pub(in crate::files) fn drop_into(&mut self, items: Vec<PathBuf>, target: Where, way: Way) {
        if way == Way::Ask {
            return self.ask_drop(items, target);
        }
        if let Some(a) = items.first().and_then(|i| inside::within(i)) {
            return match target {
                Where::Folder(f) => self.inside_take(&a, items, f),
                _ => self.say(t!("folders-act-archive-out"), false),
            };
        }
        match target {
            Where::Folder(f) => {
                let moving = match way {
                    Way::Copy => false,
                    Way::Move => true,
                    Way::Usual | Way::Ask => items.first().is_some_and(|i| same_drive(i, &f) && !self.earlier_holds(i)),
                };
                self.send_to(items, f, if moving { Kind::Move } else { Kind::Copy });
            }
            Where::Trash => self.trash_these(items),
            Where::Remote(a) => {
                let moving = matches!(way, Way::Move);
                self.send_to(items, PathBuf::from(a), if moving { Kind::Move } else { Kind::Copy });
            }
            _ => self.say(t!("folders-act-drop-only"), false),
        }
    }

    pub(in crate::files) fn let_go(&mut self, d: Drag, local: (f32, f32)) {
        let (wx, wy) = self.pointer.get();
        let m = self.mods.get();
        let way = if d.right { Way::Ask } else if m.ctrl { Way::Copy } else if m.shift { Way::Move } else { Way::Usual };
        self.said.clear();
        if let Some(gap) = self.pin_at.take() {
            self.landed = None;
            if d.pins {
                return self.pin_here(d.items, gap);
            }
        }
        if let Some(target) = self.landed.take() {
            return self.drop_into(d.items, target, way);
        }
        if wx < panes_left() && wy >= crate::files::view::top_bar_height(self) {
            self.dropping = Some((d.items, way));
            return;
        }
        let onto = if self.two { self.pane_at(wx) } else { d.pane };
        let target = if onto == d.pane {
            let pane = &*self.cols[d.pane];
            if matches!(pane.at, Where::Inside(..)) {
                return;
            }
            let hit = pane.grid.get().hit(local.0, local.1, pane.width.get(), pane.entries.len());
            match hit.and_then(|i| pane.entries.get(i)) {
                Some(e) if e.folder && !d.items.contains(&e.path) => Where::Folder(e.path.clone()),
                _ if d.right && matches!(pane.at, Where::Folder(_)) => pane.at.clone(),
                _ => return,
            }
        } else {
            dropped_on(&self.cols[onto], &d.items)
        };
        self.drop_into(d.items, target, way);
    }
}

pub(in crate::files) fn open_to_write(at: &Where) -> bool {
    use std::os::unix::ffi::OsStrExt;
    let dir = match at {
        Where::Folder(dir) if !far::is_far(dir) => dir.clone(),
        Where::Folder(_) | Where::Remote(_) => return true,
        _ => return false,
    };
    let Ok(name) = std::ffi::CString::new(dir.as_os_str().as_bytes()) else { return false };
    crate::files::reach::within(&dir, move || unsafe { libc::access(name.as_ptr(), libc::W_OK | libc::X_OK) } == 0).unwrap_or(false)
}

fn dropped_on(pane: &crate::files::app::Pane, items: &[PathBuf]) -> Where {
    let local = matches!(pane.at, Where::Folder(_) | Where::Found(..));
    match pane.hover.and_then(|i| pane.entries.get(i)) {
        Some(e) if local && e.folder && !items.contains(&e.path) => Where::Folder(e.path.clone()),
        _ => pane.at.clone(),
    }
}
