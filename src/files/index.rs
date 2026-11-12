//! The index: every name in the home folder, kept in a small file so a search by name is near instant. It is built on a thread of its own at the lowest priority and only while the computer is left alone.

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, OnceLock};
use std::time::{Duration, SystemTime};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::{wl_registry, wl_seat};
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1::{self, ExtIdleNotificationV1},
    ext_idle_notifier_v1::ExtIdleNotifierV1,
};

use super::meter::stopped;
use crate::common::files::{cache_home, home};

const MAGIC: &[u8] = b"trindle-files-index 1\n";
const LEFT_ALONE_MS: u32 = 2 * 60 * 1000;
const LOOK: Duration = Duration::from_secs(5);
const EVERY: Duration = Duration::from_secs(15 * 60);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sort {
    File,
    Folder,
    Link,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub name: OsString,
    pub sort: Sort,
    pub size: u64,
    pub away: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Folder {
    stamp: Stamp,
    items: Vec<Item>,
}

type Stamp = (i64, i64, u64);

fn stamp(meta: &fs::Metadata) -> Stamp {
    (meta.mtime(), meta.mtime_nsec(), meta.ino())
}

const UNSETTLED: Stamp = (i64::MIN, 0, 0);

const FRESH: Duration = Duration::from_secs(2);

pub fn settled(modified: SystemTime) -> bool {
    SystemTime::now().duration_since(modified).is_ok_and(|d| d >= FRESH)
}

#[cfg(test)]
pub fn age(dir: &Path) {
    let ago = SystemTime::now() - Duration::from_secs(60);
    for entry in fs::read_dir(dir).unwrap().flatten() {
        if entry.file_type().unwrap().is_dir() {
            age(&entry.path());
        }
    }
    fs::File::open(dir).unwrap().set_modified(ago).unwrap();
}

pub fn list(dir: &Path, disk: u64) -> io::Result<Vec<Item>> {
    let mut items = Vec::new();
    for entry in fs::read_dir(dir)? {
        let Ok((name, meta)) = entry.and_then(|e| e.metadata().map(|m| (e.file_name(), m))) else { continue };
        let ft = meta.file_type();
        let sort = if ft.is_dir() {
            Sort::Folder
        } else if ft.is_file() {
            Sort::File
        } else if ft.is_symlink() {
            Sort::Link
        } else {
            Sort::Other
        };
        items.push(Item { name, sort, size: meta.len(), away: ft.is_dir() && meta.dev() != disk });
    }
    Ok(items)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Index {
    root: PathBuf,
    folders: HashMap<PathBuf, Folder>,
}

impl Index {
    pub fn new(root: &Path) -> Index {
        Index { root: root.to_path_buf(), folders: HashMap::new() }
    }

    #[cfg(test)]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[cfg(test)]
    pub fn folders(&self) -> usize {
        self.folders.len()
    }

    pub fn listing(&self, dir: &Path) -> Option<&[Item]> {
        let folder = self.folders.get(dir)?;
        let meta = fs::symlink_metadata(dir).ok()?;
        (stamp(&meta) == folder.stamp).then_some(&folder.items[..])
    }

    pub fn refresh(&mut self, skip: &[PathBuf], stop: &AtomicBool, pause: &mut dyn FnMut() -> bool) -> io::Result<u64> {
        let disk = fs::symlink_metadata(&self.root)?.dev();
        let mut old = std::mem::take(&mut self.folders);
        let mut read = 0;
        let mut todo = vec![self.root.clone()];
        while let Some(dir) = todo.pop() {
            if stop.load(Ordering::Relaxed) {
                self.folders.extend(old);
                return Err(stopped());
            }
            let Ok(meta) = fs::symlink_metadata(&dir) else { continue };
            let now = stamp(&meta);
            let folder = match old.remove(&dir) {
                Some(f) if f.stamp == now => f,
                was => {
                    if !pause() {
                        if let Some(f) = was {
                            old.insert(dir, f);
                        }
                        self.folders.extend(old);
                        return Err(stopped());
                    }
                    read += 1;
                    let Ok(items) = list(&dir, disk) else { continue };
                    let trusted = meta.modified().is_ok_and(settled);
                    Folder { stamp: if trusted { now } else { UNSETTLED }, items }
                }
            };
            for item in &folder.items {
                if item.sort == Sort::Folder && !item.away {
                    let path = dir.join(&item.name);
                    if !skip.iter().any(|s| path.starts_with(s)) {
                        todo.push(path);
                    }
                }
            }
            self.folders.insert(dir, folder);
        }
        Ok(read)
    }

    pub fn open(file: &Path) -> io::Result<Index> {
        let mut r = BufReader::new(GzDecoder::new(crate::common::files::open_file(file)?));
        let mut magic = [0; MAGIC.len()];
        r.read_exact(&mut magic)?;
        if magic != MAGIC {
            return Err(io::ErrorKind::InvalidData.into());
        }
        let root = PathBuf::from(OsString::from_vec(bytes(&mut r)?));
        let mut folders = HashMap::new();
        for _ in 0..number(&mut r)? {
            let dir = PathBuf::from(OsString::from_vec(bytes(&mut r)?));
            let stamp = (number(&mut r)? as i64, number(&mut r)? as i64, number(&mut r)?);
            let count = number(&mut r)?;
            let mut items = Vec::with_capacity(count.min(1 << 16) as usize);
            for _ in 0..count {
                let name = OsString::from_vec(bytes(&mut r)?);
                let mut flags = [0];
                r.read_exact(&mut flags)?;
                let sort = match flags[0] & 3 {
                    0 => Sort::File,
                    1 => Sort::Folder,
                    2 => Sort::Link,
                    _ => Sort::Other,
                };
                items.push(Item { name, sort, size: number(&mut r)?, away: flags[0] & 4 != 0 });
            }
            folders.insert(dir, Folder { stamp, items });
        }
        Ok(Index { root, folders })
    }

    pub fn save(&self, file: &Path) -> io::Result<()> {
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir)?;
        }
        let part = file.with_extension("part");
        let out = fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(&part)?;
        let mut w = BufWriter::new(GzEncoder::new(out, flate2::Compression::fast()));
        w.write_all(MAGIC)?;
        put_bytes(&mut w, self.root.as_os_str().as_bytes())?;
        put(&mut w, self.folders.len() as u64)?;
        for (dir, f) in &self.folders {
            put_bytes(&mut w, dir.as_os_str().as_bytes())?;
            put(&mut w, f.stamp.0 as u64)?;
            put(&mut w, f.stamp.1 as u64)?;
            put(&mut w, f.stamp.2)?;
            put(&mut w, f.items.len() as u64)?;
            for item in &f.items {
                put_bytes(&mut w, item.name.as_bytes())?;
                let sort = match item.sort {
                    Sort::File => 0,
                    Sort::Folder => 1,
                    Sort::Link => 2,
                    Sort::Other => 3,
                };
                w.write_all(&[sort | if item.away { 4 } else { 0 }])?;
                put(&mut w, item.size)?;
            }
        }
        w.into_inner().map_err(|e| e.into_error())?.finish()?.sync_all()?;
        fs::rename(&part, file)
    }
}

fn put(w: &mut impl Write, mut n: u64) -> io::Result<()> {
    loop {
        let low = (n & 0x7f) as u8;
        n >>= 7;
        if n == 0 {
            return w.write_all(&[low]);
        }
        w.write_all(&[low | 0x80])?;
    }
}

fn number(r: &mut impl Read) -> io::Result<u64> {
    let mut n = 0u64;
    for shift in (0..64).step_by(7) {
        let mut b = [0];
        r.read_exact(&mut b)?;
        n |= u64::from(b[0] & 0x7f) << shift;
        if b[0] & 0x80 == 0 {
            return Ok(n);
        }
    }
    Err(io::ErrorKind::InvalidData.into())
}

fn put_bytes(w: &mut impl Write, b: &[u8]) -> io::Result<()> {
    put(w, b.len() as u64)?;
    w.write_all(b)
}

fn bytes(r: &mut impl Read) -> io::Result<Vec<u8>> {
    let len = number(r)?;
    if len > 1 << 16 {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let mut b = vec![0; len as usize];
    r.read_exact(&mut b)?;
    Ok(b)
}

pub fn file() -> PathBuf {
    cache_home().join("trindle/files-index")
}

pub fn for_folder(folder: &Path) -> Option<Index> {
    let root = home();
    if !folder.starts_with(&root) {
        return None;
    }
    Index::open(&file()).ok().filter(|i| i.root == root)
}

pub fn left_alone() -> bool {
    if crate::common::charge::on_battery() || touched() == Some(true) {
        return false;
    }
    let cpus = std::thread::available_parallelism().map_or(1, |n| n.get()) as f64;
    let load = fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|l| l.split_whitespace().next()?.parse::<f64>().ok())
        .unwrap_or(f64::MAX);
    load < (cpus * 0.5).max(1.0)
}

fn touched() -> Option<bool> {
    static WATCH: OnceLock<Option<Arc<AtomicBool>>> = OnceLock::new();
    let alone = WATCH.get_or_init(watch_touches).as_ref()?;
    Some(!alone.load(Ordering::Relaxed))
}

fn watch_touches() -> Option<Arc<AtomicBool>> {
    let alone = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel();
    let a = alone.clone();
    let spawned = std::thread::Builder::new().name("folders-index-idle".into()).spawn(move || {
        let Ok(conn) = Connection::connect_to_env() else {
            let _ = tx.send(false);
            return;
        };
        let Ok((globals, mut queue)) = registry_queue_init::<Watch>(&conn) else {
            let _ = tx.send(false);
            return;
        };
        let qh = queue.handle();
        let notifier = globals.bind::<ExtIdleNotifierV1, _, _>(&qh, 1..=1, ());
        let seat = globals.bind::<wl_seat::WlSeat, _, _>(&qh, 1..=1, ());
        let (Ok(notifier), Ok(seat)) = (notifier, seat) else {
            let _ = tx.send(false);
            return;
        };
        notifier.get_idle_notification(LEFT_ALONE_MS, &seat, &qh, ());
        let _ = tx.send(true);
        let mut watch = Watch { alone: a };
        while queue.blocking_dispatch(&mut watch).is_ok() {}
        watch.alone.store(true, Ordering::Relaxed);
    });
    spawned.ok()?;
    rx.recv_timeout(Duration::from_secs(2)).unwrap_or(false).then_some(alone)
}

struct Watch {
    alone: Arc<AtomicBool>,
}

impl Dispatch<ExtIdleNotificationV1, ()> for Watch {
    fn event(watch: &mut Self, _: &ExtIdleNotificationV1, event: ext_idle_notification_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        match event {
            ext_idle_notification_v1::Event::Idled => watch.alone.store(true, Ordering::Relaxed),
            ext_idle_notification_v1::Event::Resumed => watch.alone.store(false, Ordering::Relaxed),
            _ => {}
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for Watch {
    fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<ExtIdleNotifierV1, ()> for Watch {
    fn event(_: &mut Self, _: &ExtIdleNotifierV1, _: <ExtIdleNotifierV1 as wayland_client::Proxy>::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<wl_seat::WlSeat, ()> for Watch {
    fn event(_: &mut Self, _: &wl_seat::WlSeat, _: wl_seat::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

pub struct Indexer {
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::Thread>,
}

impl Indexer {
    pub fn home() -> Indexer {
        let root = home();
        let skip = vec![cache_home(), root.join(".local/share/Trash")];
        Indexer::start(root, file(), skip, left_alone, EVERY)
    }

    pub fn start(
        root: PathBuf,
        file: PathBuf,
        skip: Vec<PathBuf>,
        idle: impl Fn() -> bool + Send + 'static,
        every: Duration,
    ) -> Indexer {
        let stop = Arc::new(AtomicBool::new(false));
        let s = stop.clone();
        let spawned = crate::common::gentle::spawn("folders-index", move || {
            let wait = |s: &AtomicBool| loop {
                if s.load(Ordering::Relaxed) {
                    return false;
                }
                if idle() {
                    return true;
                }
                nap(s, LOOK);
            };
            while wait(&s) {
                let mut index = Index::open(&file).ok().filter(|i| i.root == root).unwrap_or_else(|| Index::new(&root));
                match index.refresh(&skip, &s, &mut || wait(&s)) {
                    Ok(0) if file.exists() => {}
                    Ok(_) => {
                        if let Err(e) = index.save(&file) {
                            eprintln!("trindle: files: index: {e}");
                        }
                    }
                    Err(_) => {}
                }
                drop(index);
                crate::common::memory::give_back();
                nap(&s, every);
            }
        });
        let thread = match spawned {
            Ok(handle) => Some(handle.thread().clone()),
            Err(e) => {
                eprintln!("trindle: files: index: {e}");
                None
            }
        };
        Indexer { stop, thread }
    }
}

impl Drop for Indexer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = &self.thread {
            thread.unpark();
        }
    }
}

fn nap(stop: &AtomicBool, long: Duration) {
    let until = std::time::Instant::now() + long;
    while !stop.load(Ordering::Relaxed) {
        let left = until.saturating_duration_since(std::time::Instant::now());
        if left.is_zero() {
            return;
        }
        std::thread::park_timeout(left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-index-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("home/a/b")).unwrap();
        fs::create_dir_all(dir.join("home/.cache/big")).unwrap();
        fs::write(dir.join("home/one.txt"), "1").unwrap();
        fs::write(dir.join("home/a/two.txt"), "22").unwrap();
        fs::write(dir.join("home/a/b/Café.txt"), "333").unwrap();
        fs::write(dir.join("home/.cache/big/skip.txt"), "4").unwrap();
        dir
    }

    #[test]
    fn only_changed_folders_are_read_again_and_it_keeps_in_a_file() {
        let dir = place("refresh");
        let home = dir.join("home");
        let skip = [home.join(".cache")];
        let stop = AtomicBool::new(false);
        let mut index = Index::new(&home);
        age(&home);
        assert_eq!(index.refresh(&skip, &stop, &mut || true).unwrap(), 3);
        assert_eq!(index.folders(), 3);
        assert!(index.listing(&home.join(".cache/big")).is_none());
        let b = index.listing(&home.join("a/b")).unwrap();
        assert_eq!(b, [Item { name: "Café.txt".into(), sort: Sort::File, size: 3, away: false }]);
        assert_eq!(index.refresh(&skip, &stop, &mut || true).unwrap(), 0);

        fs::write(home.join("a/new.txt"), "").unwrap();
        assert!(index.listing(&home.join("a")).is_none());
        assert_eq!(index.refresh(&skip, &stop, &mut || true).unwrap(), 1);
        assert!(index.listing(&home.join("a")).is_none());
        fs::File::open(home.join("a")).unwrap().set_modified(SystemTime::now() - FRESH * 2).unwrap();
        assert_eq!(index.refresh(&skip, &stop, &mut || true).unwrap(), 1);
        assert_eq!(index.listing(&home.join("a")).unwrap().len(), 3);

        let file = dir.join("cache/trindle/files-index");
        index.save(&file).unwrap();
        assert_eq!(Index::open(&file).unwrap(), index);
        fs::remove_dir_all(home.join("a/b")).unwrap();
        index.refresh(&skip, &stop, &mut || true).unwrap();
        assert_eq!(index.folders(), 2);
        fs::write(home.join("three.txt"), "").unwrap();
        assert!(index.refresh(&skip, &stop, &mut || false).is_err());
        assert_eq!(index.folders(), 2);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_indexer_works_only_while_left_alone() {
        let dir = place("indexer");
        let home = dir.join("home");
        let file = dir.join("cache/files-index");
        let alone = Arc::new(AtomicBool::new(false));
        let a = alone.clone();
        let indexer = Indexer::start(home.clone(), file.clone(), Vec::new(), move || a.load(Ordering::Relaxed), Duration::from_secs(60));
        std::thread::sleep(Duration::from_millis(300));
        assert!(!file.exists());
        alone.store(true, Ordering::Relaxed);
        let start = Instant::now();
        while !file.exists() && start.elapsed() < Duration::from_secs(15) {
            std::thread::sleep(Duration::from_millis(50));
        }
        drop(indexer);
        let index = Index::open(&file).unwrap();
        assert_eq!(index.root(), home);
        assert_eq!(index.folders(), 5);
        fs::remove_dir_all(&dir).unwrap();
    }
}
