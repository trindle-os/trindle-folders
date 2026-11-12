//! What the sidebar lists: the pinned folders and the places, and keeping them.

use std::path::{Path, PathBuf};

use super::*;

impl Files {
    pub(super) fn sidebar_msg(&mut self, m: Msg) {
        match m {
            Msg::PinAt(gap) => {
                if self.drag.as_ref().is_some_and(|d| d.pins) {
                    self.pin_at = Some(gap);
                }
            }
            Msg::PinMove(from, gap) => self.move_pin(from, gap),
            Msg::PinNameText(t) => {
                if let Some((_, text)) = &mut self.pin_rename {
                    *text = t;
                }
            }
            Msg::PinNameDone => self.pin_rename_done(),
            Msg::PinNameStop => self.pin_rename = None,
            Msg::Pin => {
                let Some(f) = self.pane().at.folder().map(Path::to_path_buf) else { return };
                self.toggle_pin(f);
            }
            _ => unreachable!("not about the sidebar"),
        }
    }

    pub(in crate::files) fn toggle_pin(&mut self, folder: PathBuf) {
        match self.pinned.iter().position(|p| *p == folder) {
            Some(i) => {
                self.pinned.remove(i);
                self.pin_names.remove(&folder);
                if self.pin_rename.as_ref().is_some_and(|(p, _)| *p == folder) {
                    self.pin_rename = None;
                }
            }
            None => self.pinned.push(folder),
        }
        self.save_pins();
        self.forget_unpinned_shares();
    }

    pub(in crate::files) fn save_pins(&mut self) {
        let Some(file) = &self.pins_file else { return };
        let names = places::names_beside(file);
        if places::write_pins(file, &self.pinned).is_err() || places::write_names(&names, &self.pin_names, &self.pinned).is_err() {
            self.say_help(t!("folders-app-pins-not-saved"), crate::files::help::NOT_KEPT);
        }
    }

    pub(in crate::files) fn pin_name(&self, folder: &Path) -> String {
        self.pin_names.get(folder).cloned().unwrap_or_else(|| crate::files::kinds::shown_name(folder))
    }

    pub(in crate::files) fn pins_named(&self) -> Vec<(PathBuf, String)> {
        self.pinned.iter().map(|p| (p.clone(), self.pin_names.get(p).cloned().unwrap_or_else(|| crate::files::work::name_of(p)))).collect()
    }

    pub(in crate::files) fn pin_rename_start(&mut self, folder: PathBuf) {
        if self.pinned.contains(&folder) && self.rename_keep() {
            let now = self.pin_name(&folder);
            self.pin_rename = Some((folder, now));
        }
    }

    pub(in crate::files) fn pin_rename_done(&mut self) {
        let Some((folder, text)) = self.pin_rename.take() else { return };
        let name = places::clean_pin_name(&text);
        if name.is_empty() || name == crate::files::kinds::shown_name(&folder) {
            self.pin_names.remove(&folder);
        } else {
            self.pin_names.insert(folder, name);
        }
        self.save_pins();
    }

    pub(in crate::files) fn pin_here(&mut self, folders: Vec<PathBuf>, gap: usize) {
        let mut at = gap.min(self.pinned.len());
        let mut added = Vec::new();
        for folder in folders {
            if self.pinned.contains(&folder) {
                continue;
            }
            added.push(crate::files::kinds::shown_name(&folder));
            self.pinned.insert(at, folder);
            at += 1;
        }
        let words = match added.as_slice() {
            [] => t!("folders-app-pinned-already").to_string(),
            [one] => t!("folders-app-pinned-one", name = one),
            many => t!("folders-app-pinned-many", n = many.len()),
        };
        self.say(words, false);
        self.save_pins();
    }

    pub(in crate::files) fn move_pin(&mut self, from: usize, gap: usize) {
        if from >= self.pinned.len() || gap == from || gap == from + 1 {
            return;
        }
        let folder = self.pinned.remove(from);
        let to = if gap > from { gap - 1 } else { gap };
        self.pinned.insert(to.min(self.pinned.len()), folder);
        self.save_pins();
    }

    pub fn place_list(&self) -> Vec<Where> {
        let mut out = vec![Where::Computer, Where::Recent, Where::Trash];
        out.extend(self.far.places());
        out.push(Where::Network);
        out
    }

    pub fn place_name(&self, w: &Where) -> String {
        self.far.name(w).unwrap_or_else(|| w.name(&self.home))
    }
}
