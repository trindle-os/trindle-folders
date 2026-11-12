//! The quick filter and the search. A box over a pane narrows what it

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::app::{Files, Msg, Pane};
use super::far;
use super::narrow::{Changed, Kind, Narrow, Size};
use super::places::{self, Entry, Where};
use crate::toolkit::t;
use super::search::{self, Hit, News, Options, Over, Search};

const MOST: usize = 10_000;

static NEXT: AtomicU64 = AtomicU64::new(1);

#[derive(Default)]
pub struct Finding {
    pub open: Option<usize>,
    pub typing: bool,
    pub(super) carry: Option<Narrow>,
}

#[derive(Default)]
pub struct Hunt {
    id: u64,
    search: Option<Search>,
    over: Option<Over>,
    full: bool,
    lines: HashMap<PathBuf, (u64, String, u64)>,
}

#[derive(Clone, Debug)]
pub enum FindMsg {
    Open(Option<usize>),
    Text(String),
    Search(bool),
    Kind(usize),
    Size(usize),
    Changed(usize),
    Close,
    Found(u64, Vec<Hit>, Option<Over>),
}

impl Files {
    pub(super) fn find_msg(&mut self, m: FindMsg) {
        match m {
            FindMsg::Open(p) => {
                let p = p.unwrap_or(self.side);
                if self.find.open == Some(p) {
                    return self.find_close();
                }
                if self.cols[p].at == Where::Computer {
                    return;
                }
                if !self.rename_keep() {
                    return;
                }
                self.locate = None;
                self.find.open = Some(p);
                self.side = p;
            }
            FindMsg::Text(text) => {
                let Some(p) = self.find.open else { return };
                self.cols[p].filter = text;
                self.refilter(self.cols[p].tab, p);
            }
            FindMsg::Search(inside) => {
                let Some(p) = self.find.open else { return };
                self.hunt(p, inside);
            }
            FindMsg::Kind(i) => self.narrow_to(|n| n.kind = Kind::ALL[i.min(Kind::ALL.len() - 1)]),
            FindMsg::Size(i) => self.narrow_to(|n| n.size = Size::ALL[i.min(Size::ALL.len() - 1)]),
            FindMsg::Changed(i) => self.narrow_to(|n| n.changed = Changed::ALL[i.min(Changed::ALL.len() - 1)]),
            FindMsg::Close => self.find_close(),
            FindMsg::Found(id, hits, over) => self.found(id, hits, over),
        }
    }

    fn find_close(&mut self) {
        let Some(p) = self.find.open.take() else { return };
        self.cols[p].filter.clear();
        let chosen = !std::mem::take(&mut self.cols[p].narrow).is_any();
        if chosen && matches!(self.cols[p].at, Where::Found(..)) {
            return self.hunt_again(p);
        }
        self.refilter(self.cols[p].tab, p);
    }

    fn narrow_to(&mut self, change: impl FnOnce(&mut Narrow)) {
        let Some(p) = self.find.open else { return };
        let was = self.cols[p].narrow;
        change(&mut self.cols[p].narrow);
        if self.cols[p].narrow == was {
            return;
        }
        if matches!(self.cols[p].at, Where::Found(..)) {
            return self.hunt_again(p);
        }
        self.refilter(self.cols[p].tab, p);
    }

    fn hunt_again(&mut self, p: usize) {
        let pane = &mut *self.cols[p];
        pane.all.clear();
        pane.entries.clear();
        pane.clear();
        pane.hunt = None;
        self.hunt_start(p);
    }

    fn refilter(&mut self, t: usize, p: usize) {
        let all = std::mem::take(&mut self.cols[p].tabs[t].all);
        self.fill(t, p, all);
    }

    pub(super) fn searchable(&self, p: usize) -> bool {
        match &self.cols[p].at {
            Where::Folder(f) => !far::is_far(f),
            Where::Found(..) => true,
            _ => false,
        }
    }

    fn hunt(&mut self, p: usize, inside: bool) {
        let pane = &self.cols[p];
        let text = pane.filter.trim().to_string();
        let narrow = pane.narrow;
        if text.is_empty() && (inside || narrow.is_any()) {
            let words = if inside { t!("folders-found-need-inside-words") } else { t!("folders-found-need-something") };
            return self.say(words, false);
        }
        let from = match &pane.at {
            Where::Folder(f) if !far::is_far(f) => f.clone(),
            Where::Found(f, ..) => f.clone(),
            _ => return self.say(t!("folders-found-only-here"), false),
        };
        self.find.open = None;
        self.find.carry = Some(narrow);
        self.go(p, Where::Found(from, text, inside));
        self.find.carry = None;
        self.cols[p].narrow = narrow;
    }

    pub(super) fn hunt_start(&mut self, p: usize) {
        let tab = self.cols[p].tab;
        let Where::Found(from, text, inside) = self.cols[p].tabs[tab].at.clone() else { return };
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut options = Options::new(&text);
        options.hidden = self.hidden;
        options.narrow = self.cols[p].tabs[tab].narrow;
        self.cols[p].tabs[tab].hunt = Some(Hunt { id, ..Hunt::default() });
        match self.post.clone() {
            Some(post) => {
                let tell = move |news: News| {
                    let m = match news {
                        News::Found(hits) => FindMsg::Found(id, hits, None),
                        News::Over(over) => FindMsg::Found(id, Vec::new(), Some(over)),
                    };
                    post.send(Msg::Find(m));
                };
                let s = if inside { Search::inside(&from, options, tell) } else { Search::names(&from, options, tell) };
                if let Some(h) = self.cols[p].tabs[tab].hunt.as_mut() {
                    h.search = Some(s);
                }
            }
            None => {
                let stop = AtomicBool::new(false);
                let mut hits = Vec::new();
                let got = if inside {
                    search::inside(&from, &options, &stop, &mut |h| hits.push(h))
                } else {
                    search::names(&from, &options, None, &stop, &mut |h| hits.push(h))
                };
                self.found(id, hits, Some(got.unwrap_or(Over { unread: 1, ..Over::default() })));
            }
        }
    }

    fn found(&mut self, id: u64, hits: Vec<Hit>, over: Option<Over>) {
        let at = self.cols.iter().enumerate().find_map(|(p, col)| {
            col.tabs.iter().position(|pane| pane.hunt.as_ref().is_some_and(|h| h.id == id)).map(|t| (t, p))
        });
        let Some((t, p)) = at else { return };
        let (hidden, sort, up) = (self.hidden, self.sort, self.up);
        let pane = &mut self.cols[p].tabs[t];
        let Some(hunt) = pane.hunt.as_mut() else { return };
        let mut all = std::mem::take(&mut pane.all);
        for hit in hits {
            if all.len() >= MOST {
                hunt.full = true;
                break;
            }
            let Some(e) = Entry::read(hit.path.clone(), &self.mime) else { continue };
            if let Some((n, line)) = hit.lines.first() {
                hunt.lines.insert(hit.path, (*n, line.clone(), hit.count));
            }
            all.push(e);
        }
        if hunt.full {
            hunt.search = None;
            hunt.over.get_or_insert(Over::default());
        }
        if let Some(o) = over {
            hunt.over = Some(o);
            hunt.search = None;
        }
        places::arrange(&mut all, &pane.at, hidden, sort, up);
        self.fill(t, p, all);
    }

    pub(super) fn found_again(&mut self, t: usize, p: usize) {
        let pane = &self.cols[p].tabs[t];
        let mut all: Vec<Entry> = pane.all.iter().filter_map(|e| Entry::read(e.path.clone(), &self.mime)).collect();
        places::arrange(&mut all, &pane.at, self.hidden, self.sort, self.up);
        self.fill(t, p, all);
    }

    pub(super) fn show_in_folder(&mut self, p: usize, item: &Path) {
        let Some(folder) = item.parent() else { return };
        self.go(p, Where::Folder(folder.to_path_buf()));
        let pane = &mut self.cols[p];
        if let Some(i) = pane.entries.iter().position(|e| e.path == item) {
            pane.pick(i, false);
            self.seek(p);
        }
    }
}

pub fn looking(pane: &Pane) -> bool {
    pane.hunt.as_ref().is_some_and(|h| h.search.is_some())
}

pub fn words(pane: &Pane, picked: usize) -> Option<String> {
    let Where::Found(from, _, inside) = &pane.at else {
        return ((!pane.filter.is_empty() || !pane.narrow.is_any()) && picked == 0).then(|| t!("folders-found-shown", shown = pane.entries.len(), all = pane.all.len()));
    };
    let hunt = pane.hunt.as_ref()?;
    if picked == 1 {
        let e = pane.picked.first().and_then(|&i| pane.entries.get(i))?;
        let folder = e.path.parent().unwrap_or(from);
        let rest = folder.strip_prefix(from).ok().filter(|r| !r.as_os_str().is_empty());
        if let Some((n, line, count)) = hunt.lines.get(&e.path) {
            let more = count.saturating_sub(1);
            return Some(match rest {
                Some(r) => t!("folders-found-in-folder-line", folder = r.display().to_string(), n = *n, line = line.trim(), more = more),
                None => t!("folders-found-in-root-line", n = *n, line = line.trim(), more = more),
            });
        }
        return Some(match rest {
            Some(r) => t!("folders-found-in-folder", folder = r.display().to_string()),
            None => t!("folders-found-in-root").to_string(),
        });
    }
    if picked > 1 {
        return None;
    }
    let n = pane.entries.len();
    let looking = hunt.search.is_some();
    let full = !looking && hunt.full;
    let (unread, skipped) = match hunt.over {
        Some(o) if !looking => (o.unread, if *inside { o.skipped } else { 0 }),
        _ => (0, 0),
    };
    let yes = |on: bool| if on { "yes" } else { "no" };
    let (filtered, all, most) = (yes(!pane.filter.is_empty()), pane.all.len(), format!("{},{:03}", MOST / 1000, MOST % 1000));
    Some(t!("folders-found-status", n = n, filtered = filtered, all = all, looking = yes(looking), full = yes(full), most = most, unread = unread, skipped = skipped))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::trash::Bin;
    use crate::toolkit::ui::App;

    #[test]
    fn the_choices_beside_the_box_narrow_a_folder_and_a_search_and_go_with_it() {
        let dir = std::env::temp_dir().join(format!("trindle-files-narrow-{}", std::process::id()));
        made_up_to_narrow(&dir);
        let mut f = Files::new(vec![dir.clone()]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.go(0, Where::Folder(dir.clone()));
        choices_narrow_the_folder_shown(&mut f);
        choices_narrow_a_search_below(&mut f, &dir);
        words_and_choices_together_below(&mut f, &dir);
        closing_the_box_and_leaving_the_search_let_the_choices_go(&mut f, &dir);
        let _ = std::fs::remove_dir_all(dir);
    }

    fn made_up_to_narrow(dir: &Path) {
        let _ = std::fs::remove_dir_all(dir);
        std::fs::create_dir_all(dir.join("Albums/2025")).unwrap();
        std::fs::create_dir_all(dir.join("Letters")).unwrap();
        std::fs::write(dir.join("notes.txt"), b"made up").unwrap();
        std::fs::write(dir.join("beach.png"), b"made up").unwrap();
        std::fs::write(dir.join("big film.mkv"), vec![0u8; 1_200_000]).unwrap();
        std::fs::write(dir.join("Albums/garden.png"), b"made up").unwrap();
        std::fs::write(dir.join("Albums/2025/hills.jpg"), vec![0u8; 1_500_000]).unwrap();
        std::fs::write(dir.join("Albums/2025/old.txt"), b"made up").unwrap();
        let long_ago = std::fs::File::options().write(true).open(dir.join("Albums/2025/old.txt")).unwrap();
        long_ago.set_modified(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_000_000_000)).unwrap();
        drop(long_ago);
    }

    fn narrow_names(f: &Files) -> Vec<String> {
        let mut n: Vec<String> = f.cols[0].tabs[f.cols[0].tab].entries.iter().map(|e| e.name.clone()).collect();
        n.sort();
        n
    }

    fn pick_kind(k: Kind) -> Msg {
        Msg::Find(FindMsg::Kind(Kind::ALL.iter().position(|x| *x == k).unwrap()))
    }

    fn pick_size(s: Size) -> Msg {
        Msg::Find(FindMsg::Size(Size::ALL.iter().position(|x| *x == s).unwrap()))
    }

    fn pick_changed(c: Changed) -> Msg {
        Msg::Find(FindMsg::Changed(Changed::ALL.iter().position(|x| *x == c).unwrap()))
    }

    fn choices_narrow_the_folder_shown(f: &mut Files) {
        f.update(pick_kind(Kind::Pictures));
        assert_eq!(narrow_names(f).len(), 5);

        f.update(Msg::Find(FindMsg::Open(Some(0))));
        f.update(pick_kind(Kind::Pictures));
        assert_eq!(narrow_names(f), ["beach.png"]);
        assert_eq!(words(&f.cols[0].tabs[0], 0).unwrap(), "1 of 5 shown");
        f.update(pick_kind(Kind::Folders));
        assert_eq!(narrow_names(f), ["Albums", "Letters"]);
        f.update(Msg::Find(FindMsg::Text("alb".into())));
        assert_eq!(narrow_names(f), ["Albums"]);
        f.update(Msg::Find(FindMsg::Text(String::new())));
        f.update(pick_kind(Kind::Anything));
        f.update(pick_size(Size::Over1Mb));
        assert_eq!(narrow_names(f), ["big film.mkv"]);
        f.update(pick_size(Size::Any));
        f.update(pick_changed(Changed::Today));
        assert_eq!(narrow_names(f).len(), 5, "all made just now");
    }

    fn choices_narrow_a_search_below(f: &mut Files, dir: &Path) {
        f.update(pick_changed(Changed::Any));
        f.update(Msg::Find(FindMsg::Search(false)));
        assert!(f.said.contains("choose a kind"), "nothing to look by: {}", f.said);
        assert_eq!(f.cols[0].at, Where::Folder(dir.to_path_buf()));
        f.update(pick_kind(Kind::Pictures));
        f.update(Msg::Find(FindMsg::Search(false)));
        assert_eq!(f.cols[0].at, Where::Found(dir.to_path_buf(), String::new(), false));
        assert_eq!(f.cols[0].at.name(&f.home), "Search results");
        assert_eq!(narrow_names(f), ["beach.png", "garden.png", "hills.jpg"]);
        assert_eq!(f.cols[0].narrow.kind, Kind::Pictures, "the search took its choices with it");
        f.update(Msg::Find(FindMsg::Open(Some(0))));
        f.update(pick_size(Size::Over1Mb));
        assert_eq!(narrow_names(f), ["hills.jpg"]);
        f.update(pick_kind(Kind::Anything));
        assert_eq!(narrow_names(f), ["big film.mkv", "hills.jpg"]);
        f.update(pick_size(Size::Any));
        f.update(pick_changed(Changed::Year));
        assert_eq!(narrow_names(f).len(), 8, "everything but the one changed long ago: {:?}", narrow_names(f));
        assert!(!narrow_names(f).contains(&"old.txt".to_string()));
    }

    fn words_and_choices_together_below(f: &mut Files, dir: &Path) {
        f.update(Msg::Find(FindMsg::Text("a".into())));
        f.update(pick_kind(Kind::Pictures));
        f.update(Msg::Find(FindMsg::Search(false)));
        assert_eq!(f.cols[0].at, Where::Found(dir.to_path_buf(), "a".into(), false));
        assert_eq!(narrow_names(f), ["beach.png", "garden.png"], "pictures changed this year with an a in their names");
        f.update(Msg::Find(FindMsg::Open(Some(0))));
        f.update(Msg::Find(FindMsg::Close));
        assert!(f.cols[0].narrow.is_any());
        assert_eq!(narrow_names(f), ["Albums", "beach.png", "garden.png"]);
    }

    fn closing_the_box_and_leaving_the_search_let_the_choices_go(f: &mut Files, dir: &Path) {
        f.go(0, Where::Folder(dir.to_path_buf()));
        assert!(f.cols[0].narrow.is_any(), "a new place starts with none");
        f.update(Msg::Find(FindMsg::Open(Some(0))));
        f.update(pick_changed(Changed::Year));
        f.update(Msg::Find(FindMsg::Search(true)));
        assert!(f.said.contains("inside files"), "{}", f.said);
        f.update(Msg::Find(FindMsg::Text("made".into())));
        f.update(Msg::Find(FindMsg::Search(true)));
        assert_eq!(narrow_names(f), ["beach.png", "garden.png", "notes.txt"], "not the one changed long ago");
        f.go(0, Where::Folder(dir.join("Albums")));
        assert!(f.cols[0].narrow.is_any());
        assert_eq!(narrow_names(f), ["2025", "garden.png"]);
    }

    #[test]
    fn the_filter_narrows_and_a_search_looks_below() {
        let dir = std::env::temp_dir().join(format!("trindle-files-found-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("deep/deeper")).unwrap();
        std::fs::write(dir.join("tax 2025.pdf"), b"x").unwrap();
        std::fs::write(dir.join("notes.txt"), b"buy milk\nring the bank\n").unwrap();
        std::fs::write(dir.join("deep/deeper/Tax return.txt"), b"bank details").unwrap();

        let mut f = Files::new(vec![dir.clone()]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.go(0, Where::Folder(dir.clone()));
        f.update(Msg::Find(FindMsg::Open(Some(0))));
        assert_eq!(f.find.open, Some(0));
        f.update(Msg::Find(FindMsg::Text("TAX".into())));
        let names = |f: &Files| f.cols[0].tabs[0].entries.iter().map(|e| e.name.clone()).collect::<Vec<_>>();
        assert_eq!(names(&f), ["tax 2025.pdf"]);
        assert_eq!(words(&f.cols[0].tabs[0], 0).unwrap(), "1 of 3 shown");

        f.update(Msg::Find(FindMsg::Search(false)));
        assert_eq!(f.cols[0].tabs[0].at, Where::Found(dir.clone(), "TAX".into(), false));
        assert_eq!(f.find.open, None);
        assert_eq!(names(&f), ["tax 2025.pdf", "Tax return.txt"]);
        assert!(!looking(&f.cols[0].tabs[0]));
        assert_eq!(words(&f.cols[0].tabs[0], 0).unwrap(), "2 found");
        let i = f.cols[0].tabs[0].entries.iter().position(|e| e.name == "Tax return.txt").unwrap();
        f.update(Msg::Pick(0, vec![i]));
        assert_eq!(words(&f.cols[0].tabs[0], 1).unwrap(), "In deep/deeper");

        f.update(Msg::Find(FindMsg::Open(None)));
        f.update(Msg::Find(FindMsg::Text("bank".into())));
        assert!(f.cols[0].tabs[0].entries.is_empty(), "the box filters what was found");
        f.update(Msg::Find(FindMsg::Search(true)));
        assert_eq!(f.cols[0].tabs[0].at, Where::Found(dir.clone(), "bank".into(), true));
        assert_eq!(names(&f), ["notes.txt", "Tax return.txt"]);
        let i = f.cols[0].tabs[0].entries.iter().position(|e| e.name == "notes.txt").unwrap();
        f.update(Msg::Pick(0, vec![i]));
        assert_eq!(words(&f.cols[0].tabs[0], 1).unwrap(), "In the folder you searched. Line 2: ring the bank");

        std::fs::remove_file(dir.join("notes.txt")).unwrap();
        f.reload_all();
        assert_eq!(names(&f), ["Tax return.txt"]);

        f.show_in_folder(0, &dir.join("deep/deeper/Tax return.txt"));
        assert_eq!(f.cols[0].tabs[0].at, Where::Folder(dir.join("deep/deeper")));
        assert_eq!(f.cols[0].tabs[0].picked.len(), 1);
        let _ = std::fs::remove_dir_all(dir);
    }
}
