// A pretend server for tests: a folder under the temporary one.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::files::remote::{Entry, Error, Kind, Progress, Remote};

pub struct Fake(pub PathBuf);

static DIALS: std::sync::Mutex<Vec<(String, usize, usize)>> = std::sync::Mutex::new(Vec::new());

pub fn links_most(user: &str, most: usize) {
    DIALS.lock().unwrap().push((user.to_string(), 0, most));
}

pub fn links_given(user: &str) -> usize {
    DIALS.lock().unwrap().iter().find(|d| d.0 == user).map_or(0, |d| d.1)
}

static ASKED: std::sync::Mutex<Vec<(String, usize)>> = std::sync::Mutex::new(Vec::new());

pub fn count_asked(user: &str) {
    ASKED.lock().unwrap().push((user.to_string(), 0));
}

pub fn asked(user: &str) -> usize {
    ASKED.lock().unwrap().iter().find(|a| a.0 == user).map_or(0, |a| a.1)
}

pub fn gives_link(user: &str) -> bool {
    let counted = match ASKED.lock().unwrap().iter_mut().find(|a| a.0 == user) {
        Some(a) => {
            a.1 += 1;
            true
        }
        None => false,
    };
    if counted {
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
    match DIALS.lock().unwrap().iter_mut().find(|d| d.0 == user) {
        Some(d) if d.1 >= d.2 => false,
        Some(d) => {
            d.1 += 1;
            true
        }
        None => true,
    }
}

static GOING: std::sync::Mutex<Vec<(PathBuf, usize, usize)>> = std::sync::Mutex::new(Vec::new());

pub fn most_at_once(server: &std::path::Path) -> usize {
    GOING.lock().unwrap().iter().find(|g| g.0 == server).map_or(0, |g| g.2)
}

struct Counted(PathBuf);

impl Counted {
    fn new(server: &std::path::Path) -> Counted {
        {
            let mut going = GOING.lock().unwrap();
            match going.iter_mut().find(|g| g.0 == server) {
                Some(g) => {
                    g.1 += 1;
                    g.2 = g.2.max(g.1);
                }
                None => going.push((server.to_path_buf(), 1, 1)),
            }
        }
        if server.to_string_lossy().contains("slow") {
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
        Counted(server.to_path_buf())
    }
}

impl Drop for Counted {
    fn drop(&mut self) {
        if let Some(g) = GOING.lock().unwrap().iter_mut().find(|g| g.0 == self.0) {
            g.1 -= 1;
        }
    }
}

type Then = Box<dyn FnOnce() + Send>;

static MEANWHILE: std::sync::Mutex<Vec<(PathBuf, Then)>> = std::sync::Mutex::new(Vec::new());

pub fn meanwhile(server: &std::path::Path, then: impl FnOnce() + Send + 'static) {
    MEANWHILE.lock().unwrap().push((server.to_path_buf(), Box::new(then)));
}

static AS_NAMED: std::sync::Mutex<Vec<(PathBuf, Then)>> = std::sync::Mutex::new(Vec::new());

pub fn as_named(server: &std::path::Path, then: impl FnOnce() + Send + 'static) {
    AS_NAMED.lock().unwrap().push((server.to_path_buf(), Box::new(then)));
}

pub const STROKE: char = '\u{2215}';

impl Fake {
    fn at(&self, p: &str) -> PathBuf {
        let plain = self.0.join(p.trim_start_matches('/'));
        if fs::symlink_metadata(&plain).is_ok() {
            return plain;
        }
        let kept = |i: usize| self.0.join(format!("{}{}", &p[..=i], p[i + 1..].replace('/', &STROKE.to_string())).trim_start_matches('/'));
        p.match_indices('/').map(|(i, _)| kept(i)).find(|kept| fs::symlink_metadata(kept).is_ok()).unwrap_or(plain)
    }

    fn careless(&self) -> bool {
        self.0.to_string_lossy().contains("careless")
    }

    fn meanwhile(&self) {
        let mut all = MEANWHILE.lock().unwrap();
        let (mine, others): (Vec<_>, Vec<_>) = std::mem::take(&mut *all).into_iter().partition(|(server, _)| *server == self.0);
        *all = others;
        drop(all);
        let mine: Vec<Then> = mine.into_iter().map(|(_, then)| then).collect();
        for then in mine {
            then();
        }
    }
}

fn entry(name: String, m: &fs::Metadata) -> Entry {
    let kind = if m.is_dir() { Kind::Folder } else if m.is_symlink() { Kind::Link } else { Kind::File };
    Entry { name, kind, size: m.len(), modified: None, mode: Some(m.permissions().mode()) }
}

fn pump(from: &mut dyn Read, to: &mut dyn Write, progress: Progress) -> Result<u64, Error> {
    let mut buf = [0u8; 4096];
    let mut done = 0;
    loop {
        let n = from.read(&mut buf)?;
        if n == 0 {
            return Ok(done);
        }
        to.write_all(&buf[..n])?;
        done += n as u64;
        if !progress(done) {
            return Err(Error::Cancelled);
        }
    }
}

impl Remote for Fake {
    fn home(&mut self) -> Result<String, Error> {
        Ok("/".into())
    }
    fn list(&mut self, path: &str) -> Result<Vec<Entry>, Error> {
        let mut out = Vec::new();
        for d in fs::read_dir(self.at(path))?.flatten() {
            out.push(entry(d.file_name().to_string_lossy().replace(STROKE, "/"), &d.metadata()?));
        }
        Ok(out)
    }
    fn stat(&mut self, path: &str) -> Result<Entry, Error> {
        if path.contains("nostat") {
            return Err(Error::Failed("cannot be looked at".into()));
        }
        let m = fs::symlink_metadata(self.at(path)).map_err(|_| Error::NotFound(path.into()))?;
        Ok(entry(crate::files::remote::name_of(path).into(), &m))
    }
    fn read(&mut self, path: &str, to: &mut dyn Write, progress: Progress) -> Result<u64, Error> {
        let mut file = File::open(self.at(path))?;
        let _going = Counted::new(&self.0);
        self.meanwhile();
        if path.contains("endless") {
            return pump(&mut file.chain(std::io::repeat(7).take(24 << 20)), to, progress);
        }
        if path.contains("torn") {
            let half = file.metadata()?.len() / 2;
            return pump(&mut file.take(half), to, progress);
        }
        pump(&mut file, to, progress)
    }
    fn write(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        let _going = Counted::new(&self.0);
        self.meanwhile();
        let mut file = File::create(self.at(path))?;
        if path.contains("short") {
            let all = pump(from, &mut std::io::sink(), progress)?;
            file.write_all(&vec![0u8; (all / 2) as usize])?;
            return Ok(all);
        }
        if path.contains("fails") {
            pump(&mut from.take(size / 2), &mut file, progress)?;
            return Err(Error::Failed("The made-up server stopped part way.".into()));
        }
        pump(from, &mut file, progress)
    }
    fn write_new(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        self.meanwhile();
        match File::options().write(true).create_new(true).open(self.at(path)) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => return Err(Error::Exists(crate::files::remote::name_of(path).into())),
            Err(e) => return Err(e.into()),
        }
        self.write(path, from, size, progress)
    }
    fn refuses_taken(&self) -> bool {
        !self.careless()
    }
    fn mkdir(&mut self, path: &str) -> Result<(), Error> {
        Ok(fs::create_dir(self.at(path))?)
    }
    fn rmdir(&mut self, path: &str) -> Result<(), Error> {
        if self.0.to_string_lossy().contains("greedy") {
            return Ok(fs::remove_dir_all(self.at(path))?);
        }
        Ok(fs::remove_dir(self.at(path))?)
    }
    fn remove(&mut self, path: &str) -> Result<(), Error> {
        Ok(fs::remove_file(self.at(path))?)
    }
    fn rename(&mut self, from: &str, to: &str) -> Result<(), Error> {
        let as_named = std::mem::take(&mut *AS_NAMED.lock().unwrap());
        let (mine, others): (Vec<_>, Vec<_>) = as_named.into_iter().partition(|(server, _)| *server == self.0);
        AS_NAMED.lock().unwrap().extend(others);
        for (_, then) in mine {
            then();
        }
        let large = fs::symlink_metadata(self.at(from)).is_ok_and(|m| m.len() >= 9000);
        if (to.contains("stuck") && large) || (to.contains("wedged") && from.contains(super::SPARE)) {
            return Err(Error::Failed("The made-up server would not give it that name.".into()));
        }
        if !self.careless() && fs::symlink_metadata(self.at(to)).is_ok() {
            return Err(Error::Exists(crate::files::remote::name_of(to).into()));
        }
        Ok(fs::rename(self.at(from), self.at(to))?)
    }
}
