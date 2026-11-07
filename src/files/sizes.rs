// A folder's real size: everything it holds, added up on a thread of its own.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime};

use super::index::settled;
use super::meter::{is_stopped, stopped, GAP};
use super::queue::Queue;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub bytes: u64,
    pub files: u64,
    pub folders: u64,
    pub unread: u64,
}

impl Tally {
    pub fn items(&self) -> u64 {
        self.files + self.folders
    }
}

pub fn measure(folder: &Path, stop: &AtomicBool, tell: &mut dyn FnMut(&Tally)) -> io::Result<Tally> {
    let top = fs::symlink_metadata(folder)?;
    let mut tally = Tally::default();
    if !top.is_dir() {
        tally.bytes = top.len();
        tally.files = 1;
        return Ok(tally);
    }
    let disk = top.dev();
    let mut seen = HashSet::new();
    let mut stack = vec![folder.to_path_buf()];
    let mut last = Instant::now();
    while let Some(dir) = stack.pop() {
        let Ok(list) = fs::read_dir(&dir) else {
            tally.unread += 1;
            continue;
        };
        for entry in list {
            if stop.load(Ordering::Relaxed) {
                return Err(stopped());
            }
            let Ok((path, meta)) = entry.and_then(|e| e.metadata().map(|m| (e.path(), m))) else {
                tally.unread += 1;
                continue;
            };
            if meta.is_dir() {
                if meta.dev() == disk {
                    tally.folders += 1;
                    stack.push(path);
                }
            } else if meta.nlink() < 2 || seen.insert((meta.dev(), meta.ino())) {
                tally.files += 1;
                tally.bytes += meta.len();
            }
        }
        if last.elapsed() >= GAP {
            tell(&tally);
            last = Instant::now();
        }
    }
    Ok(tally)
}

type Stamp = (u64, u64, Option<SystemTime>);

fn stamp(folder: &Path) -> Option<Stamp> {
    fs::symlink_metadata(folder).ok().map(|m| (m.dev(), m.ino(), m.modified().ok()))
}

type Memory = Arc<Mutex<HashMap<PathBuf, (Stamp, Tally)>>>;

pub struct Sizes {
    queue: Queue<PathBuf>,
    memory: Memory,
}

impl Sizes {
    pub fn new(tell: impl Fn(&Path, &Tally, bool) + Send + Sync + 'static) -> Sizes {
        let memory: Memory = Arc::default();
        let mem = memory.clone();
        let queue = Queue::new("folders-sizes", 1, move |folder: PathBuf, stop: &AtomicBool| {
            if let Some(t) = recall(&mem, &folder) {
                tell(&folder, &t, true);
                return;
            }
            let before = stamp(&folder);
            match measure(&folder, stop, &mut |t| tell(&folder, t, false)) {
                Ok(t) => {
                    let trusted = |s: &Stamp| stamp(&folder).as_ref() == Some(s) && s.2.is_some_and(settled);
                    if let Some(s) = before.filter(trusted) {
                        if let Ok(mut m) = mem.lock() {
                            m.insert(folder.clone(), (s, t));
                        }
                    }
                    tell(&folder, &t, true);
                }
                Err(e) if is_stopped(&e) => {}
                Err(_) => tell(&folder, &Tally { unread: 1, ..Tally::default() }, true),
            }
        });
        Sizes { queue, memory }
    }

    pub fn ask(&self, folder: &Path) {
        self.queue.ask(folder.to_path_buf());
    }

    #[cfg(test)]
    pub fn cached(&self, folder: &Path) -> Option<Tally> {
        recall(&self.memory, folder)
    }

    pub fn forget(&self, folder: &Path) {
        if let Ok(mut m) = self.memory.lock() {
            for up in folder.ancestors() {
                m.remove(up);
            }
        }
    }

    pub fn forget_round(&self, folder: &Path) {
        if let Ok(mut m) = self.memory.lock() {
            m.retain(|k, _| !k.starts_with(folder) && !folder.starts_with(k));
        }
    }

    pub fn cancel(&self) {
        self.queue.cancel();
    }
}

fn recall(memory: &Memory, folder: &Path) -> Option<Tally> {
    let now = stamp(folder)?;
    let m = memory.lock().ok()?;
    m.get(folder).filter(|(s, _)| *s == now).map(|(_, t)| *t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-sizes-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("a/b")).unwrap();
        fs::write(dir.join("one"), vec![1u8; 100]).unwrap();
        fs::write(dir.join("a/two"), vec![2u8; 200]).unwrap();
        fs::write(dir.join("a/b/three"), vec![3u8; 300]).unwrap();
        fs::hard_link(dir.join("one"), dir.join("a/one-again")).unwrap();
        std::os::unix::fs::symlink("/usr", dir.join("a/link")).unwrap();
        dir
    }

    #[test]
    fn a_folder_is_added_up_without_following_links() {
        let dir = place("measure");
        let stop = AtomicBool::new(false);
        let t = measure(&dir, &stop, &mut |_| {}).unwrap();
        let link = fs::symlink_metadata(dir.join("a/link")).unwrap().len();
        assert_eq!(t.bytes, 600 + link);
        assert_eq!(t.files, 4);
        assert_eq!(t.folders, 2);
        assert_eq!(t.unread, 0);
        assert_eq!(t.items(), 6);
        assert_eq!(measure(&dir.join("one"), &stop, &mut |_| {}).unwrap().bytes, 100);
        stop.store(true, Ordering::Relaxed);
        assert!(is_stopped(&measure(&dir, &stop, &mut |_| {}).unwrap_err()));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn sizes_are_remembered_until_the_folder_changes() {
        let dir = place("cache");
        super::super::index::age(&dir);
        let (tx, rx) = mpsc::channel();
        let tx = Mutex::new(tx);
        let sizes = Sizes::new(move |p, t, done| {
            if done {
                tx.lock().unwrap().send((p.to_path_buf(), *t)).unwrap();
            }
        });
        sizes.ask(&dir);
        let (p, t) = rx.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(p, dir);
        assert_eq!(sizes.cached(&dir), Some(t));
        fs::write(dir.join("four"), vec![4u8; 400]).unwrap();
        sizes.forget(&dir.join("a"));
        assert_eq!(sizes.cached(&dir), None);
        sizes.ask(&dir);
        let (_, t2) = rx.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(t2.bytes, t.bytes + 400);
        fs::remove_dir_all(&dir).unwrap();
    }
}
