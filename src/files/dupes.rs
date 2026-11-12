//! Finding duplicate files: files are grouped by size first, then by a hash

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Read};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::time::Instant;

use blake2::{Blake2b512, Digest};

use super::meter::{stopped, GAP};
use super::work::threads;
use crate::common::gentle;

const PEEK: u64 = 4096;

#[derive(Clone, Debug)]
pub struct Options {
    pub min_size: u64,
    pub hidden: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options { min_size: 1, hidden: false }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub size: u64,
    pub files: Vec<PathBuf>,
}

impl Group {
    pub fn spare(&self) -> u64 {
        self.size * (self.files.len() as u64 - 1)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Found {
    pub groups: Vec<Group>,
    pub unread: Vec<PathBuf>,
}

pub fn find(roots: &[PathBuf], options: &Options, stop: &AtomicBool, tell: &dyn Fn(u64, u64)) -> io::Result<Found> {
    let mut unread = Vec::new();
    let mut seen = HashSet::new();
    let mut by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    let mut last = Instant::now();
    tell(0, 0);
    for root in roots {
        walk(root, options, stop, &mut seen, &mut by_size, &mut unread, &mut || {
            if last.elapsed() >= GAP {
                tell(0, 0);
                last = Instant::now();
            }
        })?;
    }
    let alike: Vec<(u64, Vec<PathBuf>)> = by_size.into_iter().filter(|(_, v)| v.len() > 1).collect();
    let peeks: u64 = alike.iter().map(|(s, v)| (*s).min(PEEK) * v.len() as u64).sum();
    let fulls: u64 = alike.iter().filter(|(s, _)| *s > PEEK).map(|(s, v)| s * v.len() as u64).sum();
    let done = AtomicU64::new(0);
    let total = AtomicU64::new(peeks + fulls);
    let workers = threads(&roots.iter().map(PathBuf::as_path).collect::<Vec<_>>());

    let jobs: Vec<(u64, PathBuf)> = alike.into_iter().flat_map(|(s, v)| v.into_iter().map(move |p| (s, p))).collect();
    let peeked = hash_all(&jobs, PEEK, workers, stop, &done, &total, tell)?;
    let mut by_peek: HashMap<(u64, [u8; 64]), Vec<PathBuf>> = HashMap::new();
    for ((size, path), hash) in jobs.into_iter().zip(peeked) {
        match hash {
            Some(h) => by_peek.entry((size, h)).or_default().push(path),
            None => unread.push(path),
        }
    }

    let mut groups = Vec::new();
    let mut jobs = Vec::new();
    let mut need = 0;
    for ((size, _), files) in by_peek {
        if files.len() < 2 {
            continue;
        }
        if size <= PEEK {
            groups.push(Group { size, files });
        } else {
            need += size * files.len() as u64;
            jobs.extend(files.into_iter().map(|p| (size, p)));
        }
    }
    total.fetch_sub(fulls - need, Ordering::Relaxed);
    let hashed = hash_all(&jobs, u64::MAX, workers, stop, &done, &total, tell)?;
    let mut by_hash: HashMap<(u64, [u8; 64]), Vec<PathBuf>> = HashMap::new();
    for ((size, path), hash) in jobs.into_iter().zip(hashed) {
        match hash {
            Some(h) => by_hash.entry((size, h)).or_default().push(path),
            None => unread.push(path),
        }
    }
    groups.extend(by_hash.into_iter().filter(|(_, v)| v.len() > 1).map(|((size, _), files)| Group { size, files }));
    for g in &mut groups {
        g.files.sort();
    }
    groups.sort_by(|a, b| b.spare().cmp(&a.spare()).then_with(|| a.files.cmp(&b.files)));
    tell(done.load(Ordering::Relaxed), total.load(Ordering::Relaxed));
    Ok(Found { groups, unread })
}

fn walk(
    path: &Path,
    options: &Options,
    stop: &AtomicBool,
    seen: &mut HashSet<(u64, u64)>,
    by_size: &mut HashMap<u64, Vec<PathBuf>>,
    unread: &mut Vec<PathBuf>,
    tick: &mut dyn FnMut(),
) -> io::Result<()> {
    let mut stack = vec![path.to_path_buf()];
    while let Some(p) = stack.pop() {
        if stop.load(Ordering::Relaxed) {
            return Err(stopped());
        }
        let Ok(meta) = fs::symlink_metadata(&p) else {
            unread.push(p);
            continue;
        };
        if meta.is_dir() {
            let Ok(list) = fs::read_dir(&p) else {
                unread.push(p);
                continue;
            };
            for entry in list.flatten() {
                if options.hidden || !entry.file_name().as_encoded_bytes().starts_with(b".") {
                    stack.push(entry.path());
                }
            }
        } else if meta.is_file() && meta.len() >= options.min_size.max(1) && seen.insert((meta.dev(), meta.ino())) {
            by_size.entry(meta.len()).or_default().push(p);
            tick();
        }
    }
    Ok(())
}

fn hash_all(
    jobs: &[(u64, PathBuf)],
    limit: u64,
    workers: usize,
    stop: &AtomicBool,
    done: &AtomicU64,
    total: &AtomicU64,
    tell: &dyn Fn(u64, u64),
) -> io::Result<Vec<Option<[u8; 64]>>> {
    let mut out = vec![None; jobs.len()];
    if jobs.is_empty() {
        return Ok(out);
    }
    let next = AtomicUsize::new(0);
    let (send, take) = mpsc::channel();
    std::thread::scope(|s| {
        for _ in 0..workers.clamp(1, jobs.len()) {
            let send = send.clone();
            let next = &next;
            gentle::scoped(s, move || loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= jobs.len() || stop.load(Ordering::Relaxed) {
                    break;
                }
                let (size, path) = &jobs[i];
                let want = (*size).min(limit);
                let (hash, read) = hash(path, want, stop, done);
                total.fetch_sub(want - read, Ordering::Relaxed);
                if send.send((i, hash)).is_err() {
                    break;
                }
            });
        }
        drop(send);
        let mut last = Instant::now();
        loop {
            match take.recv_timeout(GAP) {
                Ok((i, h)) => out[i] = h,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
            if last.elapsed() >= GAP {
                tell(done.load(Ordering::Relaxed), total.load(Ordering::Relaxed));
                last = Instant::now();
            }
        }
    });
    if stop.load(Ordering::Relaxed) {
        return Err(stopped());
    }
    Ok(out)
}

fn hash(path: &Path, want: u64, stop: &AtomicBool, done: &AtomicU64) -> (Option<[u8; 64]>, u64) {
    let Ok(file) = crate::common::files::open_file(path) else { return (None, 0) };
    let mut file = gentle::In::new(file);
    let mut hasher = Blake2b512::new();
    let mut buf = vec![0u8; want.clamp(1, 64 * 1024) as usize];
    let mut read = 0;
    while read < want {
        if stop.load(Ordering::Relaxed) {
            return (None, read);
        }
        let room = (want - read).min(buf.len() as u64) as usize;
        let n = match file.read(&mut buf[..room]) {
            Ok(0) | Err(_) => return (None, read),
            Ok(n) => n,
        };
        hasher.update(&buf[..n]);
        read += n as u64;
        done.fetch_add(n as u64, Ordering::Relaxed);
    }
    (Some(hasher.finalize().into()), read)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-dupes-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("deep/er")).unwrap();
        fs::create_dir_all(dir.join(".hidden")).unwrap();
        dir
    }

    #[test]
    fn duplicates_are_found_and_lookalikes_are_not() {
        let dir = place("find");
        let big: Vec<u8> = (0..100_000u32).map(|i| (i % 251) as u8).collect();
        let mut late = big.clone();
        *late.last_mut().unwrap() ^= 1;
        fs::write(dir.join("big1"), &big).unwrap();
        fs::write(dir.join("deep/big2"), &big).unwrap();
        fs::write(dir.join("deep/er/big3"), &big).unwrap();
        fs::write(dir.join("deep/late"), &late).unwrap();
        fs::write(dir.join("small1"), b"John Smith").unwrap();
        fs::write(dir.join("deep/small2"), b"John Smith").unwrap();
        fs::write(dir.join("other"), b"Jane Smith").unwrap();
        fs::write(dir.join("empty1"), b"").unwrap();
        fs::write(dir.join("empty2"), b"").unwrap();
        fs::write(dir.join(".hidden/small3"), b"John Smith").unwrap();
        fs::hard_link(dir.join("big1"), dir.join("big1-link")).unwrap();
        std::os::unix::fs::symlink(dir.join("big1"), dir.join("big1-sym")).unwrap();

        let stop = AtomicBool::new(false);
        let told = std::cell::Cell::new((0, 0));
        let tell = |d, t| told.set((d, t));
        let found = find(std::slice::from_ref(&dir), &Options::default(), &stop, &tell).unwrap();
        assert_eq!(found.groups.len(), 2);
        let g = &found.groups[0];
        assert_eq!(g.size, 100_000);
        assert_eq!(g.files.len(), 3);
        assert!(g.files.contains(&dir.join("deep/big2")) && g.files.contains(&dir.join("deep/er/big3")));
        assert_eq!(g.spare(), 200_000);
        assert_eq!(found.groups[1].files, [dir.join("deep/small2"), dir.join("small1")]);
        let (d, t) = told.get();
        assert_eq!(d, t);
        assert!(found.unread.is_empty());

        let all = find(std::slice::from_ref(&dir), &Options { min_size: 1, hidden: true }, &stop, &tell).unwrap();
        assert_eq!(all.groups[1].files.len(), 3);
        let only_big = find(std::slice::from_ref(&dir), &Options { min_size: 1000, hidden: true }, &stop, &tell).unwrap();
        assert_eq!(only_big.groups.len(), 1);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn stopping_stops() {
        let dir = place("stop");
        let big = vec![7u8; 300_000];
        for n in 0..4 {
            fs::write(dir.join(format!("f{n}")), &big).unwrap();
        }
        let stop = AtomicBool::new(true);
        let e = find(std::slice::from_ref(&dir), &Options::default(), &stop, &|_, _| {}).unwrap_err();
        assert!(super::super::meter::is_stopped(&e));
        fs::remove_dir_all(&dir).unwrap();
    }
}
