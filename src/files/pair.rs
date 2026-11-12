//! What the two panes do together, from the strip between them: showing

use crate::toolkit::t;

use super::app::Files;
use super::places::{Entry, Where};

const SLACK: i64 = 2;

pub fn differing(here: &[Entry], there: &[Entry]) -> Vec<usize> {
    let other: std::collections::HashMap<&str, &Entry> = there.iter().map(|e| (e.name.as_str(), e)).collect();
    here.iter().enumerate().filter(|(_, e)| match other.get(e.name.as_str()) {
        None => true,
        Some(o) if e.folder || o.folder => e.folder != o.folder,
        Some(o) => e.size != o.size || (e.when - o.when).abs() > SLACK,
    }).map(|(i, _)| i).collect()
}

fn differing_words(n: usize) -> String {
    t!("folders-pair-differing", n = n)
}

impl Files {
    fn paired(&mut self) -> bool {
        if !self.two {
            self.say(t!("folders-pair-needs-both"), false);
        }
        self.two
    }

    pub(super) fn same_in_other(&mut self) {
        if !self.paired() {
            return;
        }
        let here = self.side();
        let at = self.cols[here].at.clone();
        if let Where::Found(..) = at {
            return self.say(t!("folders-pair-no-search"), false);
        }
        self.go(1 - here, at);
        self.side = here;
    }

    pub(super) fn swap_panes(&mut self) {
        if !self.paired() {
            return;
        }
        self.cols.swap(0, 1);
        self.side = 1 - self.side();
        self.menu = None;
        self.drag = None;
        if let Some(open) = &mut self.find.open {
            *open = 1 - *open;
        }
        if let Some((p, _)) = &mut self.locate {
            *p = 1 - *p;
        }
        if let Some(r) = &mut self.renaming {
            r.side = 1 - r.side;
        }
        if let Some(s) = &mut self.slow {
            s.side = 1 - s.side;
        }
        if let Some(l) = &mut self.listed {
            l.side = 1 - l.side;
        }
        for p in 0..2 {
            if matches!(self.cols[p].at, Where::Remote(_) | Where::Inside(..)) && self.cols[p].entries.is_empty() {
                let t = self.cols[p].tab;
                self.load(t, p);
            }
        }
    }

    pub(super) fn pick_differing(&mut self) {
        if !self.paired() {
            return;
        }
        let here = self.side();
        let other = &self.cols[1 - here];
        let there = if other.all.is_empty() { &other.entries } else { &other.all };
        let picked = differing(&self.cols[here].entries, there);
        let n = picked.len();
        let pane = self.pane_mut(here);
        pane.picked = picked;
        self.say(differing_words(n), false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::app::Msg;
    use crate::toolkit::ui::App as _;
    use std::path::PathBuf;

    fn entry(name: &str, folder: bool, size: u64, when: i64) -> Entry {
        Entry { name: name.into(), path: PathBuf::from(name), folder, size, when, kind: None }
    }

    #[test]
    fn what_differs_is_judged_by_name_size_and_date() {
        let here = [entry("only here.txt", false, 5, 100), entry("same.txt", false, 5, 100), entry("bigger.txt", false, 9, 100),
                    entry("newer.txt", false, 5, 900), entry("fat date.txt", false, 5, 102), entry("Letters", true, 0, 100),
                    entry("was a file", true, 0, 100), entry("Only folder", true, 0, 100)];
        let there = [entry("same.txt", false, 5, 100), entry("bigger.txt", false, 5, 100), entry("newer.txt", false, 5, 100),
                     entry("fat date.txt", false, 5, 100), entry("Letters", true, 0, 700), entry("was a file", false, 3, 100),
                     entry("only there.txt", false, 1, 1)];
        let names: Vec<&str> = differing(&here, &there).into_iter().map(|i| here[i].name.as_str()).collect();
        assert_eq!(names, ["only here.txt", "bigger.txt", "newer.txt", "was a file", "Only folder"]);
        assert!(differing(&there, &there).is_empty());
        assert_eq!(differing(&here, &[]).len(), here.len());
        assert!(differing_words(0).starts_with("Nothing here differs"));
        assert!(differing_words(1).starts_with("1 thing here is missing from the other pane") && differing_words(4).starts_with("4 things here are missing from the other pane"));
        for n in [0, 1, 4] {
            assert!(differing_words(n).ends_with("Subfolders are only compared by name."));
        }
    }

    fn two(tag: &str) -> (PathBuf, Files) {
        let dir = std::env::temp_dir().join(format!("trindle-files-pair-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for (sub, files) in [("left", &["apple.txt", "both.txt", "pear.txt"][..]), ("right", &["both.txt", "pear.txt"][..])] {
            std::fs::create_dir_all(dir.join(sub).join("Letters")).unwrap();
            for name in files {
                std::fs::write(dir.join(sub).join(name), if sub == "right" && *name == "pear.txt" { &b"longer"[..] } else { &b"x"[..] }).unwrap();
            }
        }
        let then = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000);
        for sub in ["left", "right"] {
            for e in std::fs::read_dir(dir.join(sub)).unwrap().flatten() {
                std::fs::File::open(e.path()).unwrap().set_modified(then).unwrap();
            }
        }
        let mut kept = super::super::kept::Kept::default();
        kept.dual = true;
        let start = (Where::Folder(dir.join("left")), Where::Folder(dir.join("right")));
        let f = Files::with(Vec::new(), dir.clone(), start, kept, Vec::new());
        (dir, f)
    }

    #[test]
    fn the_strip_shows_the_same_folder_swaps_and_picks_what_differs() {
        let (dir, mut f) = two("strip");
        let at = |f: &Files, p: usize| f.cols[p].at.clone();
        let (left, right) = (Where::Folder(dir.join("left")), Where::Folder(dir.join("right")));
        f.update(Msg::PickDiffering);
        let names = |f: &Files, p: usize| f.cols[p].picked.iter().map(|i| f.cols[p].entries[*i].name.clone()).collect::<Vec<_>>();
        assert_eq!(names(&f, 0), ["apple.txt", "pear.txt"]);
        assert!(f.cols[1].picked.is_empty());
        assert!(f.said.starts_with("2 things here are missing from the other pane"), "{}", f.said);
        f.update(Msg::OtherPane);
        f.update(Msg::PickDiffering);
        assert_eq!(names(&f, 1), ["pear.txt"]);
        assert!(f.cols[0].picked.is_empty(), "things are picked in one pane at a time");
        f.update(Msg::SwapPanes);
        assert_eq!((at(&f, 0), at(&f, 1), f.side()), (right.clone(), left.clone(), 0));
        assert_eq!(names(&f, 0), ["pear.txt"]);
        f.update(Msg::SwapPanes);
        assert_eq!((at(&f, 0), at(&f, 1), f.side()), (left.clone(), right.clone(), 1));
        f.update(Msg::SameInOther);
        assert_eq!((at(&f, 0), at(&f, 1), f.side()), (right.clone(), right.clone(), 1));
        assert_eq!(f.cols[0].back.last(), Some(&left));
        f.update(Msg::PickDiffering);
        assert!(f.cols[1].picked.is_empty() && f.said.starts_with("Nothing here differs"), "{}", f.said);
        f.update(Msg::PanesAre(false));
        for msg in [Msg::SameInOther, Msg::SwapPanes, Msg::PickDiffering] {
            f.said.clear();
            f.update(msg);
            assert_eq!((at(&f, 0), at(&f, 1), f.side()), (right.clone(), right.clone(), 1));
            assert!(f.said.contains("both panes"), "{}", f.said);
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
