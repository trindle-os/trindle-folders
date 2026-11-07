// The links open in a window: kept logins, the work links and the reading link of each server.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::toolkit::t;

use crate::files::leave::{self, Confirmed, Leave, Reached, Way};
use crate::files::remote::{self, Address, Error, Login, Scheme, Screened};
use crate::common::gentle;
#[cfg(test)]
use super::fake;
use super::{lock, root};

type Shared = Arc<Mutex<Screened>>;

#[derive(Clone)]
pub(super) struct Work {
    pub(super) link: Shared,
    pub(super) busy: Arc<AtomicBool>,
}

struct Free<'a>(&'a AtomicBool);

impl Drop for Free<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

const LINK_WAIT: std::time::Duration = std::time::Duration::from_millis(2);

#[derive(Default)]
pub(super) struct Inner {
    pub(super) open: HashMap<String, Vec<Work>>,
    pub(super) opening: HashMap<String, usize>,
    pub(super) most: HashMap<String, usize>,
    pub(super) look: HashMap<String, Shared>,
    pub(super) logins: HashMap<String, Kept>,
}

#[derive(Clone)]
pub(super) struct Kept {
    pub(super) login: Login,
    pub(super) at: Option<String>,
}

#[derive(Clone, Default)]
pub struct Links(pub(super) Arc<Mutex<Inner>>);

pub(super) fn links_most(a: &Address) -> usize {
    if a.scheme == Scheme::Mtp { 1 } else { gentle::AT_ONCE }
}

pub(super) fn dropped<T>(out: &Result<T, Error>) -> bool {
    matches!(out, Err(Error::Lost(_) | Error::Unreachable(_) | Error::Io(_)))
}

impl Links {
    pub fn open(&self, a: &Address, login: &Login, shown: Option<&Confirmed>) -> Result<String, Error> {
        let (mut r, used) = self.dial(a, Some(login), shown)?;
        let home = if a.path.is_empty() { r.home()? } else { a.path.clone() };
        let heard = shown.is_some() && a.scheme != Scheme::Mtp;
        let work = if heard { self.dial(a, Some(&used.login), shown).ok().map(|d| d.0) } else { None };
        let key = root(a);
        let link = Arc::new(Mutex::new(r));
        let mut inner = lock(&self.0);
        inner.logins.insert(key.clone(), used);
        match work {
            Some(work) => {
                inner.open.insert(key.clone(), vec![Work { link: Arc::new(Mutex::new(work)), busy: Arc::default() }]);
            }
            None if heard => {
                inner.open.insert(key.clone(), vec![Work { link: link.clone(), busy: Arc::default() }]);
                inner.most.insert(key.clone(), 1);
            }
            None => {}
        }
        if a.scheme == Scheme::Mtp {
            inner.open.insert(key, vec![Work { link, busy: Arc::default() }]);
        } else {
            inner.look.insert(key, link);
        }
        Ok(home)
    }

    fn dial(&self, a: &Address, typed: Option<&Login>, shown: Option<&Confirmed>) -> Result<(Screened, Kept), Error> {
        let held = lock(&self.0).logins.get(&root(a)).cloned();
        let was = if typed.is_none() && shown.is_none() { held.as_ref().and_then(|h| h.at.clone()) } else { None };
        let mut login = match (typed, held) {
            (Some(typed), held) => Login { password: typed.password.clone().or_else(|| held.and_then(|h| h.login.password)), ..typed.clone() },
            (None, Some(held)) => held.login,
            (None, None) => Login::default(),
        };
        if login.password.is_none() {
            login.password = a.password.clone();
        }
        let to = cleared(a, login, shown)?;
        if let (Some(was), Some(now), true) = (&was, to.address(), to.login().password.is_some()) {
            if was != now {
                return Err(Error::Heard { host: a.host.clone(), at: now.to_string(), port: a.port() });
            }
        }
        let r = dial(a, &to)?;
        let at = to.address().map(str::to_string);
        Ok((r, Kept { login: to.login().clone(), at }))
    }

    pub(super) fn claim(&self, a: &Address) -> Result<Work, Error> {
        let key = root(a);
        loop {
            let open_one = {
                let mut inner = lock(&self.0);
                let most = inner.most.get(&key).copied().unwrap_or(links_most(a));
                let opening = inner.opening.get(&key).copied().unwrap_or(0);
                let list = inner.open.entry(key.clone()).or_default();
                if let Some(w) = list.iter().find(|w| w.busy.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok()) {
                    return Ok(w.clone());
                }
                if list.len() + opening >= most {
                    false
                } else {
                    *inner.opening.entry(key.clone()).or_default() += 1;
                    true
                }
            };
            if !open_one {
                std::thread::sleep(LINK_WAIT);
                continue;
            }
            let dialled = self.dial(a, None, None).map(|d| d.0);
            let mut inner = lock(&self.0);
            if let Some(n) = inner.opening.get_mut(&key) {
                *n = n.saturating_sub(1);
            }
            let others = inner.opening.get(&key).copied().unwrap_or(0);
            match dialled {
                Ok(r) => {
                    let w = Work { link: Arc::new(Mutex::new(r)), busy: Arc::new(AtomicBool::new(true)) };
                    inner.open.entry(key).or_default().push(w.clone());
                    return Ok(w);
                }
                Err(e) => {
                    let (mut have, mut others) = (inner.open.get(&key).map_or(0, Vec::len), others);
                    while have == 0 && others > 0 {
                        drop(inner);
                        std::thread::sleep(LINK_WAIT);
                        inner = lock(&self.0);
                        have = inner.open.get(&key).map_or(0, Vec::len);
                        others = inner.opening.get(&key).copied().unwrap_or(0);
                    }
                    if have == 0 {
                        return Err(e);
                    }
                    inner.most.insert(key.clone(), have);
                }
            }
        }
    }

    pub fn with<T>(&self, a: &Address, f: impl FnOnce(&mut Screened) -> Result<T, Error>) -> Result<T, Error> {
        let work = self.claim(a)?;
        let _free = Free(&work.busy);
        let out = {
            let mut r = lock(&work.link);
            f(&mut r)
        };
        if dropped(&out) {
            let key = root(a);
            let mut inner = lock(&self.0);
            if let Some(list) = inner.open.get_mut(&key) {
                list.retain(|w| !Arc::ptr_eq(&w.link, &work.link));
                if list.is_empty() {
                    inner.most.remove(&key);
                }
            }
        }
        out
    }

    pub fn look<T>(&self, a: &Address, f: impl FnOnce(&mut Screened) -> Result<T, Error>) -> Result<T, Error> {
        if a.scheme == Scheme::Mtp {
            return self.with(a, f);
        }
        let key = root(a);
        let found = lock(&self.0).look.get(&key).cloned();
        let link = match found {
            Some(link) => link,
            None => {
                let r: Shared = Arc::new(Mutex::new(self.dial(a, None, None)?.0));
                lock(&self.0).look.entry(key.clone()).or_insert(r).clone()
            }
        };
        let out = {
            let mut r = lock(&link);
            f(&mut r)
        };
        if dropped(&out) {
            lock(&self.0).look.remove(&key);
        }
        out
    }

    pub(super) fn rest(&self) {
        for list in lock(&self.0).open.values_mut() {
            let mut kept = 0;
            list.retain(|w| {
                let keep = kept == 0 || w.busy.load(Ordering::SeqCst);
                kept += 1;
                keep
            });
        }
    }
}

fn cleared(a: &Address, mut login: Login, shown: Option<&Confirmed>) -> Result<Leave<Login>, Error> {
    let to = match a.scheme {
        Scheme::Sftp | Scheme::Mtp | Scheme::Smb | Scheme::Nfs => {
            login.password = None;
            Way::Nowhere
        }
        Scheme::Ftps | Scheme::Ftpes | Scheme::Davs => Way::Proved,
        Scheme::Ftp | Scheme::Dav => match shown {
            Some(at) => Way::At(at.reached()),
            None => Reached::now(&a.host, None).map_or(Way::Nowhere, Way::At),
        },
    };
    leave::leave(login, &to).map_err(|_| match &to {
        Way::At(reached) => Error::Heard { host: a.host.clone(), at: reached.address().to_string(), port: a.port() },
        Way::Nowhere | Way::Proved => Error::Unreachable(t!("folders-far-no-server", host = &a.host)),
    })
}

#[cfg(not(test))]
fn dial(a: &Address, to: &Leave<Login>) -> Result<Screened, Error> {
    remote::connect(a, to)
}

#[cfg(test)]
fn dial(a: &Address, to: &Leave<Login>) -> Result<Screened, Error> {
    DIALLED.with_borrow_mut(|d| d.push(Dialled { host: a.host.clone(), at: to.address().map(str::to_string), password: to.login().password.clone() }));
    match (a.host.as_str(), &a.user) {
        ("test.invalid" | "nas.local" | "10.0.0.9" | "files.example.com", Some(user)) if fake::gives_link(user) => Ok(Screened::over(Box::new(fake::Fake(std::env::temp_dir().join(user))))),
        ("127.0.0.1", _) if std::env::var_os("TRINDLE_TEST_SFTP").is_some() => remote::connect(a, to),
        _ => Err(Error::Unreachable(a.host.clone())),
    }
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Dialled {
    pub(super) host: String,
    pub(super) at: Option<String>,
    pub(super) password: Option<String>,
}

#[cfg(test)]
thread_local! {
    pub(super) static DIALLED: std::cell::RefCell<Vec<Dialled>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
impl Links {
    pub(super) fn dropped(&self) {
        let mut inner = lock(&self.0);
        inner.open.clear();
        inner.look.clear();
    }
}
