// Comparing two folders: every name is put in one of four lists (left only, right only, the same, different).

use std::cell::Cell;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{self, File, Metadata};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, SystemTime};

use super::meter::Meter;

#[derive(Clone, Debug)]
pub struct Options {
    pub content: bool,
    pub deep: bool,
    pub slack: Duration,
    pub hidden: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options { content: false, deep: true, slack: Duration::from_secs(2), hidden: true }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    File,
    Folder,
    Link,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Side {
    pub kind: Kind,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Why {
    Kind,
    Size,
    Date,
    Content,
    Target,
    Inside,
    Unread,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    OnlyLeft,
    OnlyRight,
    Same,
    Different(Why),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Newer {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub name: PathBuf,
    pub state: State,
    pub left: Option<Side>,
    pub right: Option<Side>,
    pub newer: Option<Newer>,
}

pub fn compare(left: &Path, right: &Path, options: &Options, stop: &AtomicBool, tell: &dyn Fn(u64, u64)) -> io::Result<Vec<Row>> {
    let meter = Meter::new(stop, tell);
    let walk = Walk { left, right, options, meter: &meter, total: Cell::new(0) };
    let mut rows = Vec::new();
    meter.total(0);
    walk.folder(Path::new(""), &mut rows)?;
    meter.end();
    Ok(rows)
}

struct Walk<'a> {
    left: &'a Path,
    right: &'a Path,
    options: &'a Options,
    meter: &'a Meter<'a>,
    total: Cell<u64>,
}

type Names = BTreeMap<OsString, (Option<Metadata>, Option<Metadata>)>;

impl Walk<'_> {
    fn folder(&self, under: &Path, rows: &mut Vec<Row>) -> io::Result<bool> {
        let (options, meter) = (self.options, self.meter);
        let names = self.names_in(under)?;
        self.count(&names);
        let mut all_same = true;
        for (name, (l, r)) in names {
            meter.check()?;
            let rel = under.join(&name);
            let (ls, rs) = (l.as_ref().map(side), r.as_ref().map(side));
            let newer = match (&ls, &rs) {
                (Some(a), Some(b)) => newer(a, b, options.slack),
                _ => None,
            };
            let at = rows.len();
            rows.push(Row { name: rel.clone(), state: State::Same, left: ls.clone(), right: rs.clone(), newer });
            let state = self.state_of(&rel, (&ls, &rs), rows)?;
            if !options.content {
                meter.add(1)?;
            }
            rows[at].state = state;
            all_same &= state == State::Same;
        }
        Ok(all_same)
    }

    fn names_in(&self, under: &Path) -> io::Result<Names> {
        let mut names = Names::new();
        for (i, dir) in [self.left, self.right].into_iter().enumerate() {
            for entry in fs::read_dir(dir.join(under))? {
                let entry = entry?;
                let name = entry.file_name();
                if !self.options.hidden && name.as_encoded_bytes().starts_with(b".") {
                    continue;
                }
                let meta = entry.metadata().ok();
                let slot = names.entry(name).or_default();
                if i == 0 {
                    slot.0 = meta;
                } else {
                    slot.1 = meta;
                }
            }
        }
        Ok(names)
    }

    fn count(&self, names: &Names) {
        let here: u64 = if self.options.content {
            names
                .values()
                .filter_map(|(l, r)| l.as_ref().zip(r.as_ref()))
                .filter(|(l, r)| l.is_file() && r.is_file() && l.len() == r.len())
                .map(|(l, r)| l.len() + r.len())
                .sum()
        } else {
            names.len() as u64
        };
        self.total.set(self.total.get() + here);
        self.meter.total(self.total.get());
    }

    fn state_of(&self, rel: &Path, sides: (&Option<Side>, &Option<Side>), rows: &mut Vec<Row>) -> io::Result<State> {
        let (left, right, options, meter) = (self.left, self.right, self.options, self.meter);
        let (ls, rs) = sides;
        Ok(match (ls, rs) {
            (Some(_), None) => State::OnlyLeft,
            (None, Some(_)) => State::OnlyRight,
            (Some(a), Some(b)) if a.kind != b.kind => State::Different(Why::Kind),
            (Some(a), Some(_)) => match a.kind {
                Kind::Folder if options.deep => match self.folder(rel, rows) {
                    Ok(true) => State::Same,
                    Ok(false) => State::Different(Why::Inside),
                    Err(e) if super::meter::is_stopped(&e) => return Err(e),
                    Err(_) => State::Different(Why::Unread),
                },
                Kind::Folder => State::Same,
                Kind::Link => match (fs::read_link(left.join(rel)), fs::read_link(right.join(rel))) {
                    (Ok(a), Ok(b)) if a == b => State::Same,
                    (Ok(_), Ok(_)) => State::Different(Why::Target),
                    _ => State::Different(Why::Unread),
                },
                Kind::File => file(&left.join(rel), &right.join(rel), ls.as_ref().zip(rs.as_ref()), options, meter)?,
                Kind::Other => State::Same,
            },
            (None, None) => unreachable!("every name was seen on one side"),
        })
    }
}

fn side(m: &Metadata) -> Side {
    let t = m.file_type();
    let kind = if t.is_symlink() {
        Kind::Link
    } else if t.is_dir() {
        Kind::Folder
    } else if t.is_file() {
        Kind::File
    } else {
        Kind::Other
    };
    Side { kind, size: if kind == Kind::File { m.len() } else { 0 }, modified: m.modified().ok() }
}

fn newer(a: &Side, b: &Side, slack: Duration) -> Option<Newer> {
    let (x, y) = (a.modified?, b.modified?);
    match x.duration_since(y) {
        Ok(d) if d > slack => Some(Newer::Left),
        Ok(_) => None,
        Err(e) if e.duration() > slack => Some(Newer::Right),
        Err(_) => None,
    }
}

fn file(l: &Path, r: &Path, sides: Option<(&Side, &Side)>, options: &Options, meter: &Meter) -> io::Result<State> {
    let Some((a, b)) = sides else { return Ok(State::Different(Why::Unread)) };
    if a.size != b.size {
        return Ok(State::Different(Why::Size));
    }
    if !options.content {
        return Ok(if newer(a, b, options.slack).is_some() { State::Different(Why::Date) } else { State::Same });
    }
    match same_bytes(l, r, a.size, meter) {
        Ok(true) => Ok(State::Same),
        Ok(false) => Ok(State::Different(Why::Content)),
        Err(e) if super::meter::is_stopped(&e) => Err(e),
        Err(_) => Ok(State::Different(Why::Unread)),
    }
}

fn same_bytes(l: &Path, r: &Path, size: u64, meter: &Meter) -> io::Result<bool> {
    let read = Cell::new(0u64);
    let result = (|| {
        let (mut a, mut b) = (crate::common::files::open_file(l)?, crate::common::files::open_file(r)?);
        let mut x = vec![0u8; 64 * 1024];
        let mut y = vec![0u8; 64 * 1024];
        loop {
            let n = fill(&mut a, &mut x)?;
            let m = fill(&mut b, &mut y)?;
            read.set(read.get() + (n + m) as u64);
            meter.add((n + m) as u64)?;
            if n != m || x[..n] != y[..m] {
                return Ok(false);
            }
            if n == 0 {
                return Ok(true);
            }
        }
    })();
    match result {
        Err(e) if super::meter::is_stopped(&e) => Err(e),
        other => {
            meter.add((2 * size).saturating_sub(read.get()))?;
            other
        }
    }
}

fn fill(f: &mut File, buf: &mut [u8]) -> io::Result<usize> {
    let mut n = 0;
    while n < buf.len() {
        match f.read(&mut buf[n..])? {
            0 => break,
            k => n += k,
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::FileTimes;

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-compare-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        for side in ["l", "r"] {
            fs::create_dir_all(dir.join(side).join("both/inner")).unwrap();
        }
        dir
    }

    fn at(path: &Path, secs: u64) {
        let t = SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        File::options().write(true).open(path).unwrap().set_times(FileTimes::new().set_modified(t)).unwrap();
    }

    fn state(rows: &[Row], name: &str) -> State {
        rows.iter().find(|r| r.name == Path::new(name)).unwrap_or_else(|| panic!("no row {name}")).state
    }

    fn made_up_pair(dir: &Path) -> (PathBuf, PathBuf) {
        let (l, r) = (dir.join("l"), dir.join("r"));
        for (name, a, b) in [
            ("same.txt", "John Smith", "John Smith"),
            ("size.txt", "John", "John Smith"),
            ("dated.txt", "John Smith", "John Smith"),
            ("bytes.txt", "John Smith", "Jane Smith"),
            ("both/inner/deep.txt", "a", "b"),
            ("both/fine.txt", "a", "a"),
        ] {
            fs::write(l.join(name), a).unwrap();
            fs::write(r.join(name), b).unwrap();
            at(&l.join(name), 1_000_000);
            at(&r.join(name), 1_000_001);
        }
        at(&r.join("dated.txt"), 2_000_000);
        at(&r.join("bytes.txt"), 1_000_000);
        fs::write(l.join("left-only"), "x").unwrap();
        fs::create_dir(r.join("right-folder")).unwrap();
        fs::write(r.join("right-folder/hidden-inside"), "x").unwrap();
        fs::write(l.join("kind"), "x").unwrap();
        fs::create_dir(r.join("kind")).unwrap();
        std::os::unix::fs::symlink("same.txt", l.join("link")).unwrap();
        std::os::unix::fs::symlink("size.txt", r.join("link")).unwrap();
        (l, r)
    }

    fn checks_by_size_and_date(rows: &[Row], told: (u64, u64)) {
        assert_eq!(state(rows, "same.txt"), State::Same);
        assert_eq!(state(rows, "size.txt"), State::Different(Why::Size));
        assert_eq!(state(rows, "dated.txt"), State::Different(Why::Date));
        assert_eq!(state(rows, "bytes.txt"), State::Same);
        assert_eq!(state(rows, "left-only"), State::OnlyLeft);
        assert_eq!(state(rows, "right-folder"), State::OnlyRight);
        assert!(!rows.iter().any(|r| r.name == Path::new("right-folder/hidden-inside")));
        assert_eq!(state(rows, "kind"), State::Different(Why::Kind));
        assert_eq!(state(rows, "link"), State::Different(Why::Target));
        assert_eq!(state(rows, "both/inner/deep.txt"), State::Same);
        assert_eq!(state(rows, "both"), State::Same);
        let names: Vec<_> = rows.iter().map(|r| r.name.to_string_lossy().into_owned()).collect();
        let b = names.iter().position(|n| n == "both").unwrap();
        assert_eq!(names[b + 1], "both/fine.txt");
        assert_eq!(rows.iter().find(|r| r.name == Path::new("dated.txt")).unwrap().newer, Some(Newer::Right));
        assert_eq!(told, (rows.len() as u64, rows.len() as u64));
    }

    #[test]
    fn folders_compare() {
        let dir = place("cmp");
        let (l, r) = made_up_pair(&dir);
        let stop = AtomicBool::new(false);
        let told = std::cell::Cell::new((0, 0));
        let tell = |d, t| told.set((d, t));
        let rows = compare(&l, &r, &Options::default(), &stop, &tell).unwrap();
        checks_by_size_and_date(&rows, told.get());

        let content = Options { content: true, ..Options::default() };
        let rows = compare(&l, &r, &content, &stop, &tell).unwrap();
        assert_eq!(state(&rows, "bytes.txt"), State::Different(Why::Content));
        assert_eq!(state(&rows, "dated.txt"), State::Same);
        assert_eq!(state(&rows, "both/inner/deep.txt"), State::Different(Why::Content));
        assert_eq!(state(&rows, "both/inner"), State::Different(Why::Inside));
        assert_eq!(state(&rows, "both"), State::Different(Why::Inside));
        let (d, t) = told.get();
        assert_eq!(d, t);

        let shallow = Options { deep: false, ..content };
        let rows = compare(&l, &r, &shallow, &stop, &tell).unwrap();
        assert_eq!(state(&rows, "both"), State::Same);
        assert!(!rows.iter().any(|r| r.name.starts_with("both") && r.name != Path::new("both")));

        stop.store(true, std::sync::atomic::Ordering::Relaxed);
        assert!(super::super::meter::is_stopped(&compare(&l, &r, &Options::default(), &stop, &tell).unwrap_err()));
        fs::remove_dir_all(&dir).unwrap();
    }
}
