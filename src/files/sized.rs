// Folder sizes: every folder in a shown Size column, and every folder picked, is measured on the sizes engine's thread.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::app::{Files, Msg};
use super::columns::Col;
use super::far;
use super::places::Where;
use super::sizes::{Sizes, Tally};

#[derive(Default)]
pub struct Measured {
    engine: Option<Sizes>,
    asked: HashSet<PathBuf>,
}

impl Files {
    fn sizes_shown(&self) -> Vec<PathBuf> {
        let mut want = Vec::new();
        for (p, pane) in self.cols.iter().enumerate() {
            if !self.shown_pane(p) {
                continue;
            }
            if matches!(pane.at, Where::Remote(_) | Where::Inside(..) | Where::Network) {
                continue;
            }
            let column = self.post.is_some() && super::view::shown_columns(self, pane, p).contains(&Col::Size);
            let picked = super::app::Picked::of(&pane.picked);
            let folders = pane.entries.iter().enumerate()
                .filter(|(i, e)| e.folder && (column || picked.has(*i)) && !far::is_far(&e.path));
            want.extend(folders.map(|(_, e)| e.path.clone()));
        }
        want
    }

    pub(super) fn sizes_wanted(&mut self) {
        let shown = self.sizes_shown();
        let still: HashSet<&PathBuf> = shown.iter().collect();
        if self.sizes.asked.iter().any(|a| !still.contains(a)) {
            if let Some(engine) = &self.sizes.engine {
                engine.cancel();
            }
            self.sizes.asked.clear();
        }
        let want: Vec<PathBuf> = shown.into_iter()
            .filter(|f| !matches!(self.cache.size(f), Some((_, true))) && !self.sizes.asked.contains(f))
            .collect();
        if want.is_empty() {
            return;
        }
        if self.sizes.engine.is_none() {
            if let Some(post) = self.post.clone() {
                self.sizes.engine = Some(Sizes::new(move |folder: &Path, t: &Tally, done| {
                    post.send(Msg::Sized(folder.to_path_buf(), *t, done));
                }));
            }
        }
        for folder in want {
            if !self.sizes.asked.insert(folder.clone()) {
                continue;
            }
            match &self.sizes.engine {
                Some(engine) => engine.ask(&folder),
                None => {
                    let stop = std::sync::atomic::AtomicBool::new(false);
                    let t = super::sizes::measure(&folder, &stop, &mut |_| {}).unwrap_or(Tally { unread: 1, ..Tally::default() });
                    self.sized(folder, t, true);
                }
            }
        }
    }

    pub(super) fn sizes_moved(&mut self) {
        if let Some(engine) = &self.sizes.engine {
            engine.cancel();
        }
        self.sizes.asked.clear();
    }

    pub(super) fn sized(&mut self, folder: PathBuf, t: Tally, done: bool) {
        self.cache.set_size(&folder, t.bytes, done);
        if done {
            self.sizes.asked.remove(&folder);
            self.resort_round(&folder);
        }
    }

    pub(super) fn sizes_changed(&mut self, folder: &Path) {
        if let Some(engine) = &self.sizes.engine {
            engine.forget(folder);
        }
        self.cache.stale_above(folder);
    }

    pub(super) fn sizes_touched(&mut self, folders: &[PathBuf]) {
        for folder in folders {
            if let Some(engine) = &self.sizes.engine {
                engine.forget_round(folder);
            }
            self.cache.stale_round(folder);
            self.sizes.asked.retain(|k| !k.starts_with(folder) && !folder.starts_with(k));
        }
    }

    pub(super) fn sizes_all_changed(&mut self) {
        self.sizes.asked.clear();
        if let Some(engine) = &self.sizes.engine {
            engine.cancel();
            for folder in self.cache.sized_folders() {
                engine.forget(&folder);
            }
        }
        self.cache.stale_sizes();
    }

    pub(super) fn picked_total(&self, p: usize) -> (usize, u64, bool) {
        let pane = &self.cols[p];
        let (mut bytes, mut short) = (0, false);
        for e in pane.picked.iter().filter_map(|&i| pane.entries.get(i)) {
            if !e.folder {
                bytes += e.size;
                continue;
            }
            match self.cache.size(&e.path) {
                Some((b, done)) => {
                    bytes += b;
                    short |= !done;
                }
                None => short |= !far::is_far(&e.path) && !super::neighbours::is_computer(&e.path),
            }
        }
        (pane.picked.len(), bytes, short)
    }
}

#[cfg(test)]
mod tests {
    use crate::files::app::{Files, Msg};
    use crate::files::places::Where;
    use crate::files::trash::Bin;
    use crate::toolkit::ui::App;

    #[test]
    fn a_picked_folder_is_measured_whole() {
        let dir = std::env::temp_dir().join(format!("trindle-files-sized-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("box/inner")).unwrap();
        std::fs::write(dir.join("box/a"), vec![0u8; 1000]).unwrap();
        std::fs::write(dir.join("box/inner/b"), vec![0u8; 500]).unwrap();
        std::fs::write(dir.join("loose"), vec![0u8; 20]).unwrap();

        let mut f = Files::new(vec![dir.clone()]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.go(0, Where::Folder(dir.clone()));
        let at = |f: &Files, name: &str| f.cols[0].tabs[0].entries.iter().position(|e| e.name == name).unwrap();
        let two = vec![at(&f, "box"), at(&f, "loose")];
        f.update(Msg::Pick(0, two.clone()));
        assert_eq!(f.picked_total(0), (2, 1520, false));
        assert_eq!(f.cache.size(&dir.join("box")), Some((1500, true)));

        std::fs::write(dir.join("box/inner/c"), vec![0u8; 5]).unwrap();
        f.sizes_changed(&dir.join("box/inner"));
        assert!(f.picked_total(0).2);
        f.update(Msg::Pick(0, two));
        assert_eq!(f.picked_total(0), (2, 1525, false));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_line_at_the_foot_counts_what_is_there_and_what_is_picked_in_every_view_and_pane() {
        use crate::files::app::View;
        use crate::files::view::{size_words, status_words};
        let dir = std::env::temp_dir().join(format!("trindle-files-foot-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("box/inner")).unwrap();
        std::fs::create_dir_all(dir.join("crate")).unwrap();
        std::fs::write(dir.join("box/a"), vec![0u8; 1000]).unwrap();
        std::fs::write(dir.join("box/inner/b"), vec![0u8; 500]).unwrap();
        std::fs::write(dir.join("crate/c"), vec![0u8; 40]).unwrap();
        std::fs::write(dir.join("loose"), vec![0u8; 7]).unwrap();
        std::fs::write(dir.join("other"), vec![0u8; 13]).unwrap();

        let mut f = Files::new(vec![dir.clone()]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.go(1, Where::Folder(dir.clone()));
        f.go(0, Where::Folder(dir.clone()));
        f.update(Msg::NewTab(Some(0)));
        let at = |f: &Files, p: usize, name: &str| f.cols[p].entries.iter().position(|e| e.name == name).unwrap();
        let said = |f: &Files, p: usize| status_words(f, &f.cols[p], p, false);
        for view in [View::List, View::Icons, View::Compact] {
            f.view = view;
            for p in 0..2 {
                let what = format!("{view:?}, pane {p}");
                f.update(Msg::Pick(p, Vec::new()));
                let free = f.cols[p].free.expect("a folder on a disk knows the room left");
                assert_eq!(said(&f, p), "4 items", "{what}");
                assert_eq!(super::super::view::free_words(&f.cols[p]), Some(format!("{} free", size_words(free))), "{what}: at the foot's far end");
                f.update(Msg::Pick(p, vec![at(&f, p, "loose")]));
                assert_eq!(said(&f, p), "1 selected, 7 bytes", "{what}");
                f.update(Msg::Pick(p, vec![at(&f, p, "loose"), at(&f, p, "other")]));
                assert_eq!(said(&f, p), "2 selected, 20 bytes", "{what}");
                f.update(Msg::Pick(p, vec![at(&f, p, "box"), at(&f, p, "crate"), at(&f, p, "loose")]));
                assert_eq!(said(&f, p), "3 selected, 1.5 KB", "{what}");
                f.cache.set_size(&dir.join("box"), 300, false);
                assert_eq!(said(&f, p), "3 selected, working out the size\u{2026}", "{what}");
                f.cache.set_size(&dir.join("box"), 1500, true);
                assert_eq!(said(&f, p), "3 selected, 1.5 KB", "{what}");
            }
        }
        f.go(1, Where::Recent);
        assert!(!said(&f, 1).contains("free") && super::super::view::free_words(&f.cols[1]).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn measuring_a_picked_folder_is_given_up_when_it_is_picked_no_longer() {
        let dir = std::env::temp_dir().join(format!("trindle-files-giveup-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for d in ["one", "two"] {
            std::fs::create_dir_all(dir.join(d)).unwrap();
            std::fs::write(dir.join(d).join("x"), vec![0u8; 10]).unwrap();
        }
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        let at = |f: &Files, name: &str| f.cols[0].tabs[0].entries.iter().position(|e| e.name == name).unwrap();
        f.update(Msg::Pick(0, vec![at(&f, "one")]));
        f.cache.set_size(&dir.join("one"), 4, false);
        f.sizes.asked.insert(dir.join("one"));
        f.update(Msg::Resized);
        assert!(f.sizes.asked.contains(&dir.join("one")));
        f.update(Msg::Pick(0, vec![at(&f, "two")]));
        assert!(f.sizes.asked.is_empty());
        assert_eq!(f.cache.size(&dir.join("two")), Some((10, true)));
        f.update(Msg::Pick(0, vec![at(&f, "one")]));
        assert_eq!(f.cache.size(&dir.join("one")), Some((10, true)));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn sorting_by_size_places_folders_by_what_they_hold() {
        let dir = std::env::temp_dir().join(format!("trindle-files-size-sort-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for (folder, bytes) in [("a-small", 10), ("b-big", 5000), ("c-middle", 300)] {
            std::fs::create_dir_all(dir.join(folder)).unwrap();
            std::fs::write(dir.join(folder).join("x"), vec![0u8; bytes]).unwrap();
        }
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        let names = |f: &Files| f.cols[0].tabs[0].entries.iter().map(|e| e.name.clone()).collect::<Vec<_>>();
        let i = names(&f).iter().position(|n| n == "c-middle").unwrap();
        f.update(Msg::Choose(0, i));
        f.update(Msg::SortBy(1));
        f.update(Msg::Flip);
        for (folder, bytes) in [("a-small", 10), ("b-big", 5000), ("c-middle", 300)] {
            f.update(Msg::Sized(dir.join(folder), crate::files::sizes::Tally { bytes, ..Default::default() }, true));
        }
        assert_eq!(names(&f), ["b-big", "c-middle", "a-small"]);
        let pane = &f.cols[0].tabs[0];
        assert_eq!(pane.chosen.map(|i| pane.entries[i].name.as_str()), Some("c-middle"));
        f.update(Msg::Reload);
        assert_eq!(names(&f), ["b-big", "c-middle", "a-small"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_job_measures_again_only_where_it_touched() {
        let dir = std::env::temp_dir().join(format!("trindle-files-touched-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("into/deep")).unwrap();
        std::fs::create_dir_all(dir.join("apart")).unwrap();
        std::fs::write(dir.join("into/deep/a"), vec![0u8; 100]).unwrap();
        std::fs::write(dir.join("apart/b"), vec![0u8; 30]).unwrap();

        let mut f = Files::new(vec![dir.clone()]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.go(0, Where::Folder(dir.clone()));
        let at = |f: &Files, name: &str| f.cols[0].tabs[0].entries.iter().position(|e| e.name == name).unwrap();
        let both = vec![at(&f, "into"), at(&f, "apart")];
        f.update(Msg::Pick(0, both.clone()));
        assert_eq!(f.cache.size(&dir.join("into")), Some((100, true)));
        assert_eq!(f.cache.size(&dir.join("apart")), Some((30, true)));

        std::fs::write(dir.join("into/deep/c"), vec![0u8; 7]).unwrap();
        f.sizes_touched(&[dir.join("into/deep/c")]);
        assert_eq!(f.cache.size(&dir.join("into")).map(|s| s.1), Some(false));
        assert_eq!(f.cache.size(&dir.join("apart")), Some((30, true)));
        f.update(Msg::Pick(0, both));
        assert_eq!(f.cache.size(&dir.join("into")), Some((107, true)));
        let _ = std::fs::remove_dir_all(dir);
    }
}
