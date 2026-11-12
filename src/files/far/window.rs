//! The window's side of servers and phones: what it knows of them, the messages about them and the work they start.

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use crate::toolkit::t;

use crate::files::inplace;
use crate::files::help;
use crate::files::app::{Files, Msg};
use crate::files::dialogs::Dialog;
use crate::files::places::Where;
use crate::files::remote::{self, Address, Error, Scheme, Screened};
use crate::files::work;
use crate::common::files::is_one_name;
use super::card::{Connect, Question, Why, why};
use super::copy::{room_in, take_in};
use super::hands::save_back;
use super::links::Links;
use super::{address, path_of};

pub struct Server {
    pub root: String,
    pub name: String,
    pub at: Where,
}

#[derive(Default)]
pub struct Far {
    pub links: Links,
    pub servers: Vec<Server>,
    pub phones: Vec<remote::Phone>,
    pub waiting: Vec<String>,
    pub fetched: HashMap<PathBuf, Copied>,
    pub(super) made: std::collections::HashSet<PathBuf>,
}

impl Far {
    pub(in crate::files) fn forget_copies(&self) {
        for dir in &self.made {
            let _ = fs::remove_dir_all(dir);
        }
    }
}

pub(in crate::files) fn forget_fetched() {
    let base = fetch_dir("");
    std::thread::spawn(move || {
        if fs::symlink_metadata(&base).is_ok_and(|m| m.is_dir()) {
            let _ = fs::remove_dir_all(&base);
        }
    });
}

pub struct Copied {
    pub far: PathBuf,
    pub(super) seen: Option<std::time::SystemTime>,
    pub(super) sending: bool,
    pub(super) again: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Then {
    Open,
    View(u64),
}

const SETTLE: std::time::Duration = std::time::Duration::from_millis(if cfg!(test) { 0 } else { 400 });

pub(super) fn fetch_dir(host: &str) -> PathBuf {
    #[cfg(test)]
    let base = std::env::temp_dir().join(format!("trindle-files-fetched-{}", std::process::id()));
    #[cfg(not(test))]
    let base = crate::common::files::cache_home().join("trindle-files").join("remote");
    base.join(host)
}

fn copy_dir(a: &Address) -> PathBuf {
    use sha2::{Digest, Sha256};
    let whole = Address { password: None, ..a.clone() }.to_string();
    let sum = Sha256::digest(whole.as_bytes());
    fetch_dir(&a.host).join(sum[..12].iter().map(|b| format!("{b:02x}")).collect::<String>())
}

pub(super) fn changed_time(p: &Path) -> Option<std::time::SystemTime> {
    fs::metadata(p).and_then(|m| m.modified()).ok()
}

impl Far {
    pub fn new() -> Far {
        let mut far = Far::default();
        far.look_for_phones();
        far
    }

    pub fn look_for_phones(&mut self) {
        if !cfg!(test) {
            self.phones = remote::phones();
        }
    }

    pub fn places(&self) -> Vec<Where> {
        let phones = self.phones.iter().map(|p| Where::Remote(format!("mtp://{}/", p.id)));
        phones.chain(self.servers.iter().map(|s| s.at.clone())).collect()
    }

    pub fn name(&self, w: &Where) -> Option<String> {
        let Where::Remote(at) = w else { return None };
        if let Some(p) = self.phones.iter().find(|p| *at == format!("mtp://{}/", p.id)) {
            return Some(p.name.clone());
        }
        self.servers.iter().find(|s| s.at == *w).map(|s| s.name.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Listed {
    pub side: usize,
    pub choose: Option<PathBuf>,
    pub rename: Option<(PathBuf, bool)>,
}

#[derive(Clone, Debug)]
pub enum FarMsg {
    Listed { t: usize, p: usize, asked: String, got: Result<remote::Listing, Why> },
    Connected { p: usize, root: String, name: String, at: String },
    Refused(Why),
    Fetched { far: PathBuf, then: Then, got: Result<PathBuf, String> },
    SentBack(PathBuf, Result<(), String>),
    Renamed { was: Box<inplace::Renaming>, now: Option<PathBuf>, next: Option<PathBuf>, got: Result<(), String> },
    Made { side: usize, path: PathBuf, got: Result<(), String> },
    Open,
    Address(String),
    Password(crate::files::remote::Secret),
    Key(String),
    PickKey,
    Trust,
    Go,
    Close,
    Plugged,
}

impl Files {
    pub(super) fn far_off<T: Send + 'static>(&mut self, job: impl FnOnce() -> T + Send + 'static, done: impl FnOnce(T) -> FarMsg + Send + 'static) {
        self.off_thread("folders-far", job, move |got| Msg::Far(done(got)));
    }

    pub(in crate::files) fn far_list(&mut self, t: usize, p: usize) {
        let Where::Remote(at) = self.cols[p].tabs[t].at.clone() else { return };
        let a = match Address::parse(&at) {
            Ok(a) => a,
            Err(e) => return self.say_help(e.to_string(), help::remote_page(&e, false)),
        };
        if a.scheme == Scheme::Smb {
            return self.smb_load(t, p, a);
        }
        if a.scheme == Scheme::Nfs {
            return self.nfs_load(t, p, a);
        }
        if !self.far.waiting.contains(&at) {
            self.far.waiting.push(at.clone());
        }
        let links = self.far.links.clone();
        let phone = a.scheme == Scheme::Mtp;
        self.far_off(move || links.look(&a, |r| {
            let mut list = r.list(path_of(&a))?;
            for e in list.entries.iter_mut().filter(|e| e.kind == remote::Kind::Link) {
                if let Ok(t) = r.follow(path_of(&a.join(&e.name))) {
                    (e.kind, e.size, e.modified) = (t.kind, t.size, t.modified.or(e.modified));
                }
            }
            Ok(list)
        }).map_err(move |e| why(e, phone)),
                     move |got| FarMsg::Listed { t, p, asked: at, got });
    }

    pub(in crate::files) fn far_open(&mut self, path: &Path) {
        self.far_fetch(path, Then::Open);
    }

    pub(in crate::files) fn far_fetch(&mut self, path: &Path, then: Then) {
        let Some(a) = address(path) else { return };
        let name = remote::name_of(&a.path).to_string();
        if !is_one_name(name.as_bytes()) || !is_one_name(a.host.as_bytes()) {
            return self.say_help(t!("folders-far-unsafe-name"), help::NAME_REFUSED);
        }
        let dir = if then == Then::Open { copy_dir(&a) } else { copy_dir(&a).with_extension("view") };
        let to = dir.join(&name);
        if self.far.fetched.contains_key(&to) {
            return self.far_msg(FarMsg::Fetched { far: path.to_path_buf(), then, got: Ok(to) });
        }
        self.far.made.insert(dir.clone());
        let links = self.far.links.clone();
        if then == Then::Open {
            self.say(t!("folders-far-fetching", name = &name), false);
        }
        let far = path.to_path_buf();
        self.far_off(move || {
            fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let listed = links.with(&a, |r| {
                let e = r.stat(path_of(&a))?;
                if e.kind == remote::Kind::Link { r.follow(path_of(&a)) } else { Ok(e) }
            }).map_err(|e| e.to_string())?.size;
            let mut file = File::create(&to).map_err(|e| e.to_string())?;
            match take_in(&links, &a, listed, room_in(&dir), &mut file, &mut || true) {
                Ok(_) => Ok(to),
                Err(e) => {
                    let _ = fs::remove_file(&to);
                    Err(match e {
                        Error::Failed(why) if why == work::changed_why() => t!("folders-far-cannot-fetch", name = &name, why = &why),
                        e => e.to_string(),
                    })
                }
            }
        }, move |got| FarMsg::Fetched { far, then, got });
    }

    pub(in crate::files) fn far_saved(&mut self, news: &[PathBuf]) {
        for n in news {
            let Some(c) = self.far.fetched.get_mut(n) else { continue };
            let now = changed_time(n);
            if now.is_none() || now == c.seen {
                continue;
            }
            if c.sending {
                c.again = true;
                continue;
            }
            c.seen = now;
            c.sending = true;
            let (local, far, links, back) = (n.clone(), c.far.clone(), self.far.links.clone(), n.clone());
            self.far_off(move || {
                std::thread::sleep(SETTLE);
                let a = address(&far).ok_or_else(|| t!("folders-far-address-unknown").to_string())?;
                save_back(&links, &a, &local)
            }, move |got| FarMsg::SentBack(back, got));
        }
    }

    pub(in crate::files) fn far_do(&mut self, a: Address, job: impl FnOnce(&mut Screened, &str) -> Result<(), Error> + Send + 'static,
                         done: impl FnOnce(Result<(), String>) -> FarMsg + Send + 'static) {
        let links = self.far.links.clone();
        self.far_off(move || links.with(&a, |r| job(r, path_of(&a))).map_err(|e| e.to_string()), done);
    }

    pub(super) fn far_relist(&mut self) {
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                if matches!(self.cols[p].tabs[t].at, Where::Remote(_)) {
                    self.far_list(t, p);
                }
            }
        }
    }

    pub(in crate::files) fn connect_card(&mut self, address: String, question: Option<Question>, p: usize) {
        let mut c = Connect::new(address, p);
        c.question = question;
        self.dialog = Some(Dialog::Connect(c));
    }
}
