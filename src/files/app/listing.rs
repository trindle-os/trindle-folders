//! Reading each pane's place and putting its items in order.

use std::path::{Path, PathBuf};

use super::*;

impl Files {
    pub(in crate::files) fn first_reads(&mut self) {
        #[cfg(test)]
        {
            let bins = crate::files::trash::all_files();
            for bin in &bins {
                self.watch_also(bin);
            }
            self.trash_dirs = bins;
        }
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                self.load(t, p);
            }
        }
    }

    pub(in crate::files) fn watch_also(&mut self, folder: &Path) {
        if self.watched.contains(folder) {
            return;
        }
        if let Some(w) = &mut self.watch {
            if w.folder(folder).is_ok() {
                self.watched.insert(folder.to_path_buf());
            }
        }
    }

    pub(in crate::files) fn load(&mut self, t: usize, p: usize) {
        if self.post.is_some() {
            self.sharing_refresh(false);
        }
        if self.cols[p].tabs[t].at == Where::Computer {
            self.computer_again();
        }
        let pane = &mut self.cols[p].tabs[t];
        if matches!(pane.at, Where::Remote(_) | Where::Inside(..) | Where::Found(..)) {
            (pane.error, pane.free, pane.reading, pane.late) = (None, None, false, false);
            pane.read = 0;
        }
        if let Where::Remote(_) = pane.at {
            return self.far_list(t, p);
        }
        if let Where::Inside(..) = pane.at {
            return self.inside_list(t, p);
        }
        if let Where::Found(..) = pane.at {
            return self.found_again(t, p);
        }
        self.read_aside(t, p);
    }

    pub(in crate::files) fn fill(&mut self, t: usize, p: usize, entries: Vec<Entry>) {
        self.cache.forget(&entries);
        self.place(t, p, entries);
        let pane = &mut self.cols[p].tabs[t];
        pane.band = None;
        pane.hover = None;
        pane.name_at.set(None);
    }

    pub(in crate::files) fn place(&mut self, t: usize, p: usize, mut entries: Vec<Entry>) {
        if let Some(c) = &self.choosing {
            entries.retain(|e| c.shows(e, &self.mime));
        }
        for e in entries.iter_mut().filter(|e| e.folder) {
            if let Some((bytes, _)) = self.cache.size(&e.path) {
                e.size = bytes;
            }
        }
        if self.sort == Key::Size && self.cols[p].tabs[t].at != Where::Recent {
            places::sort(&mut entries, self.sort, self.up);
        }
        if let Some(col) = self.by.filter(|_| self.cols[p].tabs[t].at != Where::Recent) {
            let up = self.up;
            entries.sort_by(|a, b| b.folder.cmp(&a.folder).then_with(|| {
                let by = self.cache.order(a, b, col);
                if up { by } else { by.reverse() }
            }));
        }
        let by = self.kept.group(&self.cols[p].tabs[t].at);
        let now = crate::common::clock::Local::now();
        crate::files::groups::arrange(&mut entries, by, now);
        let pane = &mut self.cols[p].tabs[t];
        let keep: Vec<PathBuf> = pane.picked.iter().filter_map(|&i| pane.entries.get(i)).map(|e| e.path.clone()).collect();
        let chosen = pane.chosen.and_then(|i| pane.entries.get(i)).map(|e| e.path.clone());
        let filter = crate::files::search::Filter::new(&pane.filter);
        let narrow = pane.narrow;
        pane.entries = if filter.is_empty() && narrow.is_any() {
            entries.clone()
        } else {
            entries.iter().filter(|e| filter.matches(&e.name) && narrow.fits(e, now)).cloned().collect()
        };
        pane.all = entries;
        pane.picked = places::among(&pane.entries, &keep);
        pane.chosen = chosen.and_then(|c| pane.entries.iter().position(|e| e.path == c));
        pane.anchor = pane.chosen;
        pane.groups = crate::files::groups::runs(&pane.entries, by, now);
    }

    pub(in crate::files) fn resort_round(&mut self, folder: &Path) {
        if self.sort != Key::Size {
            return;
        }
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                if self.cols[p].tabs[t].at.folder() == folder.parent() {
                    let all = std::mem::take(&mut self.cols[p].tabs[t].all);
                    self.place(t, p, all);
                }
            }
        }
    }

    pub fn sorted_col(&self) -> Option<crate::files::columns::Col> {
        use crate::files::columns::Col;
        self.by.or(match self.sort {
            Key::Name => None,
            Key::Size => Some(Col::Size),
            Key::Date => Some(Col::Date),
            Key::Type => Some(Col::Type),
        })
    }

    pub(in crate::files) fn sort_on(&mut self, col: Option<crate::files::columns::Col>) {
        use crate::files::columns::Col;
        (self.sort, self.by) = match col {
            None => (Key::Name, None),
            Some(Col::Size) => (Key::Size, None),
            Some(Col::Date) => (Key::Date, None),
            Some(Col::Type) => (Key::Type, None),
            other => (Key::Name, other),
        };
        self.reload_all();
    }

    pub(in crate::files) fn resort_all(&mut self) {
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                let all = std::mem::take(&mut self.cols[p].tabs[t].all);
                self.place(t, p, all);
            }
        }
    }

    pub(in crate::files) fn reload_all(&mut self) {
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                self.load(t, p);
            }
        }
    }
}

pub(super) fn touches(at: &Where, path: &Path, trash: &[PathBuf]) -> bool {
    match at {
        Where::Folder(f) => path.parent() == Some(f.as_path()) || path == f,
        Where::Trash => path.parent().is_some_and(|d| trash.iter().any(|f| f == d)),
        Where::Recent => path == places::recent_file(),
        Where::Network | Where::Remote(_) => false,
        Where::Computer => false,
        Where::Inside(a, _) => path == a,
        Where::Found(..) => false,
    }
}
