//! Renaming many files at once by a list of rules.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::common::files::rename_new;
use crate::toolkit::t;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Case {
    Lower,
    Upper,
    Title,
    Sentence,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ending {
    Set(String),
    Lower,
    Upper,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum At {
    Start,
    End,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rule {
    Replace { find: String, with: String, wildcard: bool, match_case: bool },
    Before(String),
    After(String),
    Number { start: u64, step: u64, pad: usize, at: At, sep: String },
    Case(Case),
    Ending(Ending),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clash {
    Empty,
    Bad,
    Long,
    Twice,
    Taken,
    NotText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Planned {
    pub from: PathBuf,
    pub to: PathBuf,
    pub clash: Option<Clash>,
}

impl Planned {
    pub fn moves(&self) -> bool {
        self.from != self.to
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Undo {
    pub moves: Vec<(PathBuf, PathBuf)>,
}

pub fn new_name(name: &str, folder: bool, index: usize, rules: &[Rule]) -> String {
    let (mut stem, mut end) = split(name, folder);
    for rule in rules {
        match rule {
            Rule::Replace { find, with, wildcard, match_case } => {
                stem = replace(&stem, find, with, *wildcard, *match_case);
            }
            Rule::Before(t) => stem.insert_str(0, t),
            Rule::After(t) => stem.push_str(t),
            Rule::Number { start, step, pad, at, sep } => {
                let n = start.saturating_add(step.saturating_mul(index as u64));
                let n = format!("{n:0pad$}", pad = *pad);
                stem = match at {
                    At::Start => format!("{n}{sep}{stem}"),
                    At::End => format!("{stem}{sep}{n}"),
                };
            }
            Rule::Case(c) => stem = case(&stem, *c),
            Rule::Ending(e) if !folder => match e {
                Ending::Set(s) => end = Some(s.trim_start_matches('.').to_string()).filter(|s| !s.is_empty()),
                Ending::Lower => end = end.map(|e| e.to_lowercase()),
                Ending::Upper => end = end.map(|e| upper(&e)),
            },
            Rule::Ending(_) => {}
        }
    }
    match end {
        Some(e) => format!("{stem}.{e}"),
        None => stem,
    }
}

pub fn preview(items: &[PathBuf], rules: &[Rule]) -> Vec<Planned> {
    let mut plan: Vec<Planned> = items
        .iter()
        .enumerate()
        .map(|(i, from)| {
            let dir = from.parent().unwrap_or(Path::new(""));
            let Some(name) = from.file_name().and_then(|n| n.to_str()) else {
                return Planned { from: from.clone(), to: from.clone(), clash: Some(Clash::NotText) };
            };
            let folder = fs::symlink_metadata(from).is_ok_and(|m| m.is_dir());
            let new = new_name(name, folder, i, rules);
            let clash = if new.is_empty() {
                Some(Clash::Empty)
            } else if new.contains('/') || new.contains('\0') || new == "." || new == ".." {
                Some(Clash::Bad)
            } else if new.len() > 255 {
                Some(Clash::Long)
            } else {
                None
            };
            let to = if clash.is_some() { from.clone() } else { dir.join(&new) };
            Planned { from: from.clone(), to, clash }
        })
        .collect();
    let mut count: HashMap<PathBuf, usize> = HashMap::new();
    for p in &plan {
        *count.entry(p.to.clone()).or_default() += 1;
    }
    let leaving: HashSet<&PathBuf> = plan.iter().map(|p| &p.from).collect();
    let mut marks = Vec::with_capacity(plan.len());
    for p in &plan {
        let mark = if p.clash.is_some() {
            p.clash
        } else if count[&p.to] > 1 {
            Some(Clash::Twice)
        } else if p.moves() && !leaving.contains(&p.to) && taken(&p.from, &p.to) {
            Some(Clash::Taken)
        } else {
            None
        };
        marks.push(mark);
    }
    for (p, m) in plan.iter_mut().zip(marks) {
        p.clash = m;
    }
    plan
}

fn taken(from: &Path, to: &Path) -> bool {
    let Ok(there) = fs::symlink_metadata(to) else { return false };
    match fs::symlink_metadata(from) {
        Ok(me) => (me.dev(), me.ino()) != (there.dev(), there.ino()),
        Err(_) => true,
    }
}

pub fn apply(plan: &[Planned]) -> io::Result<Undo> {
    if plan.iter().any(|p| p.clash.is_some()) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, t!("folders-rename-clash")));
    }
    let moves: Vec<(PathBuf, PathBuf)> = plan.iter().filter(|p| p.moves()).map(|p| (p.from.clone(), p.to.clone())).collect();
    carry(&moves)?;
    Ok(Undo { moves })
}

pub fn undo(done: &Undo) -> io::Result<Undo> {
    let back: Vec<(PathBuf, PathBuf)> = done.moves.iter().rev().map(|(a, b)| (b.clone(), a.clone())).collect();
    carry(&back)?;
    Ok(Undo { moves: back })
}

fn carry(moves: &[(PathBuf, PathBuf)]) -> io::Result<()> {
    let mut pending: Vec<(PathBuf, PathBuf)> = moves.to_vec();
    let mut landed: Vec<(PathBuf, PathBuf)> = Vec::new();
    let mut parked: Vec<(PathBuf, PathBuf)> = Vec::new();
    #[cfg(test)]
    let mut steps = 0;
    let result = (|| {
        while !pending.is_empty() {
            #[cfg(test)]
            {
                if DIE_AFTER.with(|d| d.get() == Some(steps)) {
                    return Err(io::Error::other("stopped"));
                }
                steps += 1;
            }
            let free = (0..pending.len()).find(|&i| !pending.iter().any(|(from, _)| *from == pending[i].1));
            let Some(i) = free else {
                let (from, _) = &pending[0];
                let dir = from.parent().unwrap_or(Path::new(""));
                let hold = (0..)
                    .map(|n| dir.join(format!(".trindle-rename-{}-{n}", std::process::id())))
                    .find(|p| fs::symlink_metadata(p).is_err())
                    .expect("a free name");
                parked.push((from.clone(), hold.clone()));
                write_journal(&parked)?;
                rename_new(from, &hold)?;
                pending[0].0 = hold;
                continue;
            };
            let (from, to) = pending.remove(i);
            if fs::symlink_metadata(&to).is_ok() {
                return Err(io::Error::new(io::ErrorKind::AlreadyExists, t!("folders-rename-already-there", path = &to.display().to_string())));
            }
            rename_new(&from, &to)?;
            landed.push((from, to));
        }
        Ok(())
    })();
    #[cfg(test)]
    if DIE_AFTER.with(|d| d.get().is_some()) {
        return result;
    }
    if result.is_err() {
        for (from, to) in landed.iter().rev() {
            let _ = rename_new(to, from);
        }
        for (from, hold) in parked.iter().rev() {
            give_back(hold, from);
        }
    }
    if !parked.is_empty() {
        let _ = journal().map(fs::remove_file);
    }
    result
}

#[cfg(test)]
thread_local! {
    static DIE_AFTER: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}

fn journal() -> Option<PathBuf> {
    #[cfg(test)]
    return JOURNAL.with(|j| j.borrow().clone()).or_else(|| Some(std::env::temp_dir().join(format!("trindle-rename-journal-{}", std::process::id()))));
    #[cfg(not(test))]
    Some(crate::common::files::data_home().join("trindle-folders").join("renaming"))
}

#[cfg(test)]
thread_local! {
    static JOURNAL: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

fn write_journal(parked: &[(PathBuf, PathBuf)]) -> io::Result<()> {
    use std::os::unix::ffi::OsStrExt;
    let Some(path) = journal() else { return Err(io::Error::other(t!("folders-rename-no-journal"))) };
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut bytes = Vec::new();
    for (from, hold) in parked {
        for p in [from, hold] {
            bytes.extend_from_slice(p.as_os_str().as_bytes());
            bytes.push(0);
        }
    }
    crate::common::files::write_whole_private(&path, &bytes)
}

pub fn recover() {
    use std::os::unix::ffi::OsStrExt;
    let Some(path) = journal() else { return };
    let Ok(bytes) = crate::common::files::read_file(&path) else { return };
    let paths: Vec<PathBuf> = bytes.split(|b| *b == 0).filter(|p| !p.is_empty()).map(|p| PathBuf::from(std::ffi::OsStr::from_bytes(p))).collect();
    for pair in paths.chunks_exact(2) {
        if fs::symlink_metadata(&pair[1]).is_ok() {
            give_back(&pair[1], &pair[0]);
        }
    }
    let _ = fs::remove_file(path);
}

fn give_back(hold: &Path, from: &Path) {
    let (Some(dir), Some(name)) = (from.parent(), from.file_name()) else { return };
    let folder = fs::symlink_metadata(hold).is_ok_and(|m| m.is_dir());
    super::proof::named_back(hold, dir, name, folder);
}

fn split(name: &str, folder: bool) -> (String, Option<String>) {
    if folder {
        return (name.to_string(), None);
    }
    match name.rfind('.') {
        Some(i) if i > 0 && i + 1 < name.len() => (name[..i].to_string(), Some(name[i + 1..].to_string())),
        _ => (name.to_string(), None),
    }
}

fn upper(s: &str) -> String {
    s.chars().flat_map(char::to_uppercase).collect()
}

fn case(s: &str, c: Case) -> String {
    match c {
        Case::Lower => s.to_lowercase(),
        Case::Upper => upper(s),
        Case::Title | Case::Sentence => {
            let mut out = String::with_capacity(s.len());
            let mut start = true;
            for ch in s.chars() {
                if start && ch.is_alphanumeric() {
                    out.extend(char::to_uppercase(ch));
                    start = false;
                } else {
                    out.extend(ch.to_lowercase());
                }
                if c == Case::Title && !ch.is_alphanumeric() && ch != '\'' {
                    start = true;
                }
            }
            out
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Piece {
    Letter(char),
    One,
    Any,
}

fn same(a: char, b: char, match_case: bool) -> bool {
    a == b || (!match_case && a.to_lowercase().eq(b.to_lowercase()))
}

fn replace(s: &str, find: &str, with: &str, wildcard: bool, match_case: bool) -> String {
    if find.is_empty() {
        return s.to_string();
    }
    let pattern: Vec<Piece> = find
        .chars()
        .map(|c| match c {
            '*' if wildcard => Piece::Any,
            '?' if wildcard => Piece::One,
            c => Piece::Letter(c),
        })
        .collect();
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        match longest(&pattern, &chars[i..], match_case).filter(|&n| n > 0) {
            Some(n) => {
                out.push_str(with);
                i += n;
            }
            None => {
                out.push(chars[i]);
                i += 1;
            }
        }
    }
    out
}

fn longest(pattern: &[Piece], s: &[char], match_case: bool) -> Option<usize> {
    let mut reach = vec![false; s.len() + 1];
    reach[0] = true;
    for piece in pattern {
        let mut next = vec![false; s.len() + 1];
        match piece {
            Piece::Any => {
                let first = reach.iter().position(|&r| r)?;
                next[first..].iter_mut().for_each(|r| *r = true);
            }
            Piece::One => next[1..].copy_from_slice(&reach[..s.len()]),
            Piece::Letter(c) => {
                for j in 0..s.len() {
                    next[j + 1] = reach[j] && same(s[j], *c, match_case);
                }
            }
        }
        reach = next;
    }
    reach.iter().rposition(|&r| r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-rename-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn rep(find: &str, with: &str, wildcard: bool) -> Rule {
        Rule::Replace { find: find.into(), with: with.into(), wildcard, match_case: false }
    }

    #[test]
    fn rules_make_new_names() {
        assert_eq!(new_name("IMG_0042.JPG", false, 0, &[rep("img_*", "Holiday", true)]), "Holiday.JPG");
        assert_eq!(new_name("a.b.c.txt", false, 0, &[rep(".", "-", false)]), "a-b-c.txt");
        assert_eq!(new_name("a*b?c", false, 0, &[rep("*", "x", false), rep("?", "y", false)]), "axbyc");
        assert_eq!(new_name("file21x", false, 0, &[rep("??x", "", true)]), "file");
        assert_eq!(new_name("cat cat", false, 0, &[rep("CAT", "dog", false)]), "dog dog");
        let exact = Rule::Replace { find: "CAT".into(), with: "dog".into(), wildcard: false, match_case: true };
        assert_eq!(new_name("cat CAT", false, 0, &[exact]), "cat dog");
        let number = Rule::Number { start: 1, step: 2, pad: 3, at: At::End, sep: " ".into() };
        assert_eq!(new_name("photo.jpg", false, 4, &[number]), "photo 009.jpg");
        let first = Rule::Number { start: 7, step: 1, pad: 0, at: At::Start, sep: "-".into() };
        assert_eq!(new_name("photo.jpg", false, 0, &[first, Rule::Before("x".into())]), "x7-photo.jpg");
        assert_eq!(new_name("notes", false, 0, &[Rule::After(" (old)".into())]), "notes (old)");
        assert_eq!(new_name("the CAT's hat.TXT", false, 0, &[Rule::Case(Case::Title)]), "The Cat's Hat.TXT");
        assert_eq!(new_name("THE cat.txt", false, 0, &[Rule::Case(Case::Sentence)]), "The cat.txt");
        assert_eq!(new_name("Über.txt", false, 0, &[Rule::Case(Case::Upper)]), "ÜBER.txt");
        assert_eq!(new_name("a.JPEG", false, 0, &[Rule::Ending(Ending::Set(".jpg".into()))]), "a.jpg");
        assert_eq!(new_name("a.JPEG", false, 0, &[Rule::Ending(Ending::Lower)]), "a.jpeg");
        assert_eq!(new_name("a.txt", false, 0, &[Rule::Ending(Ending::Set(String::new()))]), "a");
        assert_eq!(new_name("README", false, 0, &[Rule::Ending(Ending::Set("md".into()))]), "README.md");
        assert_eq!(new_name(".bashrc", false, 0, &[Rule::After("2".into())]), ".bashrc2");
        assert_eq!(new_name("v1.2", true, 0, &[Rule::After("x".into()), Rule::Ending(Ending::Set("y".into()))]), "v1.2x");
    }

    #[test]
    fn clashes_are_marked() {
        let dir = place("clash");
        for n in ["a.txt", "b.txt", "keep.txt"] {
            fs::write(dir.join(n), n).unwrap();
        }
        let items = [dir.join("a.txt"), dir.join("b.txt")];
        let plan = preview(&items, &[rep("*", "same", true)]);
        assert!(plan.iter().all(|p| p.clash == Some(Clash::Twice)));
        let plan = preview(&items[..1], &[rep("a", "keep", false)]);
        assert_eq!(plan[0].clash, Some(Clash::Taken));
        let plan = preview(&items, &[Rule::Number { start: 0, step: 1, pad: 0, at: At::End, sep: String::new() }, rep("a0", "b", false)]);
        assert_eq!(plan[0].to, dir.join("b.txt"));
        assert_eq!(plan[0].clash, None);
        let plan = preview(&items, &[rep("*", "", true)]);
        assert!(plan.iter().all(|p| p.clash == Some(Clash::Twice) || p.clash.is_none()));
        let plan = preview(&items[..1], &[rep("a", "x/y", false)]);
        assert_eq!(plan[0].clash, Some(Clash::Bad));
        let plan = preview(&items[..1], &[Rule::After("x".repeat(300))]);
        assert_eq!(plan[0].clash, Some(Clash::Long));
        let plan = preview(&items[..1], &[rep("a", "", false), Rule::Ending(Ending::Set(String::new()))]);
        assert_eq!(plan[0].clash, Some(Clash::Empty));
        assert!(apply(&plan).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    fn journal_in(dir: &Path) {
        JOURNAL.with(|j| *j.borrow_mut() = Some(dir.join("journal")));
    }

    #[test]
    fn a_rename_stopped_part_way_loses_no_name() {
        let dir = place("killed");
        let names = |d: &Path| { let mut n: Vec<String> = fs::read_dir(d).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).filter(|n| n != "journal").collect(); n.sort(); n };
        for n in ["one", "two", "three"] {
            fs::write(dir.join(n), n).unwrap();
        }
        journal_in(&dir);
        let plan: Vec<Planned> = ["one", "two", "three"].iter().map(|n| Planned { from: dir.join(n), to: dir.join(format!("{n}.txt")), clash: None }).collect();
        DIE_AFTER.with(|d| d.set(Some(1)));
        assert!(apply(&plan).is_err());
        DIE_AFTER.with(|d| d.set(None));
        assert_eq!(names(&dir), ["one.txt", "three", "two"], "a file was left under a name nobody chose");
        fs::write(dir.join("a"), "a").unwrap();
        fs::write(dir.join("b"), "b").unwrap();
        let plan = vec![Planned { from: dir.join("a"), to: dir.join("b"), clash: None }, Planned { from: dir.join("b"), to: dir.join("a"), clash: None }];
        DIE_AFTER.with(|d| d.set(Some(1)));
        assert!(apply(&plan).is_err());
        DIE_AFTER.with(|d| d.set(None));
        assert!(names(&dir).iter().any(|n| n.starts_with(".trindle-rename-")));
        recover();
        assert_eq!(names(&dir), ["a", "b", "one.txt", "three", "two"]);
        assert_eq!(fs::read_to_string(dir.join("a")).unwrap(), "a");
        assert!(!dir.join("journal").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn swaps_apply_and_undo() {
        let dir = place("swap");
        journal_in(&dir);
        for n in ["a", "b", "c"] {
            fs::write(dir.join(n), n).unwrap();
        }
        let items = [dir.join("a"), dir.join("b"), dir.join("c")];
        let plan: Vec<Planned> = items
            .iter()
            .zip(["b", "c", "a"])
            .map(|(from, to)| Planned { from: from.clone(), to: dir.join(to), clash: None })
            .collect();
        let done = apply(&plan).unwrap();
        assert_eq!(fs::read_to_string(dir.join("b")).unwrap(), "a");
        assert_eq!(fs::read_to_string(dir.join("c")).unwrap(), "b");
        assert_eq!(fs::read_to_string(dir.join("a")).unwrap(), "c");
        let again = undo(&done).unwrap();
        for n in ["a", "b", "c"] {
            assert_eq!(fs::read_to_string(dir.join(n)).unwrap(), n);
        }
        undo(&again).unwrap();
        assert_eq!(fs::read_to_string(dir.join("b")).unwrap(), "a");
        assert!(!journal().unwrap().exists(), "the journal outlived the rename");
        let plan = preview(&[dir.join("a")], &[Rule::Case(Case::Upper)]);
        assert_eq!(plan[0].clash, None);
        apply(&plan).unwrap();
        assert_eq!(fs::read_to_string(dir.join("A")).unwrap(), "c");
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 3);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_failed_rename_puts_everything_back() {
        let dir = place("fail");
        fs::write(dir.join("one"), "1").unwrap();
        fs::write(dir.join("two"), "2").unwrap();
        let plan = vec![
            Planned { from: dir.join("one"), to: dir.join("uno"), clash: None },
            Planned { from: dir.join("two"), to: dir.join("late"), clash: None },
        ];
        fs::write(dir.join("late"), "late").unwrap();
        assert!(apply(&plan).is_err());
        assert_eq!(fs::read_to_string(dir.join("one")).unwrap(), "1");
        assert_eq!(fs::read_to_string(dir.join("two")).unwrap(), "2");
        assert_eq!(fs::read_to_string(dir.join("late")).unwrap(), "late");
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 3);
        fs::remove_dir_all(&dir).unwrap();
    }
}
