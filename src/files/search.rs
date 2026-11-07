// Finding files by name or by what is written inside them, and the quick filter that narrows a folder as the person types.

use std::borrow::Cow;
use std::collections::VecDeque;
use std::fs;
use std::io::{self, Read};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use super::index::{self, list, Index, Item, Sort};
use super::kinds::glob_matches;
use super::meter::{stopped, GAP};
use super::narrow::Narrow;
use super::viewer::{decode, is_text, PEEK};

const LINES_KEPT: usize = 20;
const LINE_LETTERS: usize = 160;

#[derive(Clone, Debug)]
pub struct Options {
    pub text: String,
    pub hidden: bool,
    pub other_disks: bool,
    pub most: u64,
    pub narrow: Narrow,
}

impl Options {
    pub fn new(text: &str) -> Options {
        Options { text: text.to_string(), hidden: false, other_disks: false, most: 16 << 20, narrow: Narrow::default() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hit {
    pub path: PathBuf,
    pub lines: Vec<(u64, String)>,
    pub count: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Over {
    pub looked: u64,
    pub unread: u64,
    pub skipped: u64,
}

pub enum News {
    Found(Vec<Hit>),
    Over(Over),
}

pub use crate::toolkit::ahead::{fold, fold_into};

#[derive(Clone, Debug, Default)]
pub struct Filter {
    words: Vec<String>,
    pattern: Option<String>,
}

impl Filter {
    pub fn new(typed: &str) -> Filter {
        let folded = fold(typed.trim());
        if folded.contains(['*', '?']) {
            return Filter { words: Vec::new(), pattern: Some(folded) };
        }
        Filter { words: folded.split_whitespace().map(str::to_string).collect(), pattern: None }
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty() && self.pattern.is_none()
    }

    pub fn matches(&self, name: &str) -> bool {
        let name = fold(name);
        match &self.pattern {
            Some(p) => glob_matches(p, &name),
            None => self.words.iter().all(|w| name.contains(w.as_str())),
        }
    }
}

pub fn names(
    folder: &Path,
    options: &Options,
    index: Option<&Index>,
    stop: &AtomicBool,
    found: &mut dyn FnMut(Hit),
) -> io::Result<Over> {
    let filter = Filter::new(&options.text);
    let narrow = options.narrow;
    let mime = narrow.wants_kinds().then(super::kinds::Mime::load);
    let now = crate::common::clock::Local::now();
    walk(folder, options, index, stop, &mut |path, item, over| {
        let named = if filter.is_empty() { !narrow.is_any() } else { filter.matches(&item.name.to_string_lossy()) };
        if named && (narrow.is_any() || narrow.fits_found(path, item.sort == Sort::Folder, item.size, mime.as_ref(), now)) {
            found(Hit { path: path.to_path_buf(), lines: Vec::new(), count: 0 });
        }
        over.looked += 1;
    })
}

pub fn inside(folder: &Path, options: &Options, stop: &AtomicBool, found: &mut dyn FnMut(Hit)) -> io::Result<Over> {
    let phrase = fold(options.text.trim());
    if phrase.is_empty() {
        return Ok(Over::default());
    }
    let narrow = options.narrow;
    let mime = narrow.wants_kinds().then(super::kinds::Mime::load);
    let now = crate::common::clock::Local::now();
    walk(folder, options, None, stop, &mut |path, item, over| {
        over.looked += 1;
        if item.sort != Sort::File {
            return;
        }
        if !narrow.is_any() && !narrow.fits_found(path, false, item.size, mime.as_ref(), now) {
            return;
        }
        if item.size > options.most {
            over.skipped += 1;
            return;
        }
        match look_inside(path, &phrase, stop) {
            Ok(Some(hit)) => found(hit),
            Ok(None) => {}
            Err(e) if e.kind() == io::ErrorKind::InvalidData => over.skipped += 1,
            Err(_) => over.unread += 1,
        }
    })
}

fn walk(
    folder: &Path,
    options: &Options,
    index: Option<&Index>,
    stop: &AtomicBool,
    each: &mut dyn FnMut(&Path, &Item, &mut Over),
) -> io::Result<Over> {
    let disk = fs::symlink_metadata(folder)?.dev();
    let mut over = Over::default();
    let mut todo = VecDeque::from([folder.to_path_buf()]);
    while let Some(dir) = todo.pop_front() {
        let items = match index.and_then(|i| i.listing(&dir)) {
            Some(items) => Cow::Borrowed(items),
            None => match list(&dir, disk) {
                Ok(items) => Cow::Owned(items),
                Err(_) => {
                    over.unread += 1;
                    continue;
                }
            },
        };
        if stop.load(Ordering::Relaxed) {
            return Err(stopped());
        }
        for item in items.iter() {
            if !options.hidden && item.name.as_encoded_bytes().starts_with(b".") {
                continue;
            }
            let path = dir.join(&item.name);
            if item.sort == Sort::Folder && (options.other_disks || !item.away) {
                todo.push_back(path.clone());
            }
            each(&path, item, &mut over);
        }
    }
    Ok(over)
}

fn look_inside(path: &Path, phrase: &str, stop: &AtomicBool) -> io::Result<Option<Hit>> {
    let mut bytes = Vec::new();
    crate::common::files::open_file(path)?.read_to_end(&mut bytes)?;
    if !is_text(&bytes[..bytes.len().min(PEEK)]) {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let (text, _) = decode(&bytes);
    let mut hit = Hit { path: path.to_path_buf(), lines: Vec::new(), count: 0 };
    for (n, line) in text.lines().enumerate() {
        if n % 4096 == 0 && stop.load(Ordering::Relaxed) {
            return Err(stopped());
        }
        let Some(at) = find_folded(line, phrase) else { continue };
        hit.count += 1;
        if hit.lines.len() < LINES_KEPT {
            hit.lines.push((n as u64 + 1, around(line, at)));
        }
    }
    Ok((hit.count > 0).then_some(hit))
}

fn find_folded(line: &str, phrase: &str) -> Option<usize> {
    let mut folded = String::with_capacity(line.len());
    let mut starts = Vec::with_capacity(line.len());
    for c in line.chars() {
        starts.push(folded.len());
        fold_into(c, &mut folded);
    }
    let at = folded.find(phrase)?;
    Some(starts.partition_point(|&s| s <= at).saturating_sub(1))
}

fn around(line: &str, at: usize) -> String {
    let letters: Vec<char> = line.chars().collect();
    let lead = letters.iter().take_while(|c| c.is_whitespace()).count();
    let tail = letters.iter().rev().take_while(|c| c.is_whitespace()).count();
    let (lo, hi) = (lead, letters.len().saturating_sub(tail).max(lead));
    if hi - lo <= LINE_LETTERS {
        return letters[lo..hi].iter().collect();
    }
    let start = at.saturating_sub(LINE_LETTERS / 3).clamp(lo, hi - LINE_LETTERS);
    let end = start + LINE_LETTERS;
    let mut out = String::new();
    if start > lo {
        out.push_str("...");
    }
    out.extend(&letters[start..end]);
    if end < hi {
        out.push_str("...");
    }
    out
}

pub struct Search {
    stop: Arc<AtomicBool>,
}

impl Search {
    pub fn names(folder: &Path, options: Options, tell: impl Fn(News) + Send + 'static) -> Search {
        Search::start("folders-search", folder, options, tell, |folder, options, stop, found| {
            let index = index::for_folder(folder);
            names(folder, options, index.as_ref(), stop, found)
        })
    }

    pub fn inside(folder: &Path, options: Options, tell: impl Fn(News) + Send + 'static) -> Search {
        Search::start("folders-search-inside", folder, options, tell, inside)
    }

    fn start(
        name: &str,
        folder: &Path,
        options: Options,
        tell: impl Fn(News) + Send + 'static,
        how: impl FnOnce(&Path, &Options, &AtomicBool, &mut dyn FnMut(Hit)) -> io::Result<Over> + Send + 'static,
    ) -> Search {
        let stop = Arc::new(AtomicBool::new(false));
        let (s, folder) = (stop.clone(), folder.to_path_buf());
        let spawned = crate::common::gentle::spawn(name, move || {
            let mut batch = Vec::new();
            let mut last = Instant::now();
            let result = how(&folder, &options, &s, &mut |hit| {
                batch.push(hit);
                if last.elapsed() >= GAP {
                    tell(News::Found(std::mem::take(&mut batch)));
                    last = Instant::now();
                }
            });
            if !batch.is_empty() {
                tell(News::Found(batch));
            }
            tell(News::Over(match result {
                Ok(over) => over,
                Err(_) => Over { unread: 1, ..Over::default() },
            }));
        });
        if let Err(e) = spawned {
            eprintln!("trindle: files: {name}: {e}");
        }
        Search { stop }
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for Search {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-search-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("deep/er")).unwrap();
        fs::create_dir_all(dir.join(".hidden")).unwrap();
        fs::write(dir.join("Café Menu.txt"), "Starters\n  Soup of the day  \nCRÈME brûlée, twice: crème brulee\n").unwrap();
        fs::write(dir.join("deep/cafe-notes.md"), "nothing here\n").unwrap();
        fs::write(dir.join("deep/er/old.txt"), b"Cr\xe8me br\xfbl\xe9e\n").unwrap();
        fs::write(dir.join("deep/er/photo.bin"), [0u8, 1, 2, 0, b'c', b'r', b'e', b'm', b'e']).unwrap();
        fs::write(dir.join(".hidden/cafe.txt"), "creme brulee\n").unwrap();
        std::os::unix::fs::symlink(dir.join("deep"), dir.join("cafe-link")).unwrap();
        dir
    }

    #[test]
    fn folding_takes_off_capitals_and_accents() {
        assert_eq!(fold("Crème Brûlée"), "creme brulee");
        assert_eq!(fold("Straße ÆØÅ Łódź Œuvre"), "strasse aeoa lodz oeuvre");
        assert_eq!(fold("Cafe\u{301}"), "cafe");
        assert_eq!(fold("Ĳssel Ŧ ſ"), "ijssel t s");
    }

    #[test]
    fn the_quick_filter_matches_words_and_patterns() {
        let f = Filter::new("  cafe MENU ");
        assert!(f.matches("Café Menu.txt"));
        assert!(f.matches("menu for the café"));
        assert!(!f.matches("Café.txt"));
        assert!(Filter::new("*.TXT").matches("Café Menu.txt"));
        assert!(!Filter::new("*.txt").matches("notes.md"));
        assert!(Filter::new("caf?").matches("CAFÉ"));
        assert!(Filter::new("").is_empty());
        assert!(Filter::new("").matches("anything"));
    }

    #[test]
    fn names_are_found_below_but_not_hidden_or_through_links() {
        let dir = place("names");
        let stop = AtomicBool::new(false);
        let mut hits = Vec::new();
        let over = names(&dir, &Options::new("cafe"), None, &stop, &mut |h| hits.push(h.path)).unwrap();
        hits.sort();
        assert_eq!(hits, [dir.join("Café Menu.txt"), dir.join("cafe-link"), dir.join("deep/cafe-notes.md")]);
        assert!(over.looked >= 6);
        let mut all = Vec::new();
        let options = Options { hidden: true, ..Options::new("cafe") };
        names(&dir, &options, None, &stop, &mut |h| all.push(h.path)).unwrap();
        assert_eq!(all.len(), 4);
        stop.store(true, Ordering::Relaxed);
        assert!(names(&dir, &options, None, &stop, &mut |_| {}).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_search_goes_by_the_choices_with_words_or_with_none() {
        use crate::files::narrow::{Kind, Narrow};
        let dir = place("narrow");
        let stop = AtomicBool::new(false);
        let found = |options: &Options| {
            let mut hits = Vec::new();
            names(&dir, options, None, &stop, &mut |h| hits.push(h.path)).unwrap();
            hits.sort();
            hits
        };
        assert!(found(&Options::new("")).is_empty());
        let folders = Options { narrow: Narrow { kind: Kind::Folders, ..Narrow::default() }, ..Options::new("") };
        assert_eq!(found(&folders), [dir.join("deep"), dir.join("deep/er")]);
        let docs = Options { narrow: Narrow { kind: Kind::Documents, ..Narrow::default() }, ..Options::new("cafe") };
        assert_eq!(found(&docs), [dir.join("Café Menu.txt"), dir.join("deep/cafe-notes.md")]);
        let mut hits = Vec::new();
        let pictures = Options { narrow: Narrow { kind: Kind::Pictures, ..Narrow::default() }, ..Options::new("creme") };
        inside(&dir, &pictures, &stop, &mut |h| hits.push(h.path)).unwrap();
        assert!(hits.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_index_gives_the_same_finds_and_misses_nothing_new() {
        let dir = place("index");
        let stop = AtomicBool::new(false);
        let mut idx = Index::new(&dir);
        index::age(&dir);
        idx.refresh(&[], &stop, &mut || true).unwrap();
        let search = |idx: Option<&Index>| {
            let mut hits = Vec::new();
            names(&dir, &Options::new("cafe"), idx, &stop, &mut |h| hits.push(h.path)).unwrap();
            hits.sort();
            hits
        };
        assert!(idx.listing(&dir.join("deep/er")).is_some());
        assert_eq!(search(Some(&idx)), search(None));
        fs::write(dir.join("deep/er/cafe-late.txt"), "").unwrap();
        assert!(search(Some(&idx)).contains(&dir.join("deep/er/cafe-late.txt")));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn text_is_found_inside_files_but_not_binaries() {
        let dir = place("inside");
        let stop = AtomicBool::new(false);
        let mut hits = Vec::new();
        let over = inside(&dir, &Options::new("Creme Brulee"), &stop, &mut |h| hits.push(h)).unwrap();
        hits.sort_by(|a, b| a.path.cmp(&b.path));
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].path, dir.join("Café Menu.txt"));
        assert_eq!(hits[0].count, 1);
        assert_eq!(hits[0].lines, [(3, "CRÈME brûlée, twice: crème brulee".to_string())]);
        assert_eq!(hits[1].path, dir.join("deep/er/old.txt"));
        assert_eq!(over.skipped, 1);
        let small = Options { most: 10, ..Options::new("creme") };
        let mut none = 0;
        inside(&dir, &small, &stop, &mut |_| none += 1).unwrap();
        assert_eq!(none, 0);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn long_lines_are_cut_around_the_match() {
        let line = format!("{}needle{}", "a".repeat(300), "b".repeat(300));
        let at = find_folded(&line, "needle").unwrap();
        assert_eq!(at, 300);
        let shown = around(&line, at);
        assert!(shown.starts_with("...") && shown.ends_with("..."));
        assert!(shown.contains("needle"));
        assert_eq!(find_folded("Straße x", "x"), Some(7));
    }

    #[test]
    fn a_search_on_its_own_thread_streams_and_ends() {
        let dir = place("thread");
        let (tx, rx) = mpsc::channel();
        let _search = Search::names(&dir, Options::new("cafe"), move |n| {
            let _ = tx.send(n);
        });
        let mut found = 0;
        while let News::Found(h) = rx.recv_timeout(Duration::from_secs(5)).unwrap() {
            found += h.len();
        }
        assert_eq!(found, 3);
        fs::remove_dir_all(&dir).unwrap();
    }
}
