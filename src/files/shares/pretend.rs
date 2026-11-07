// A pretend network for tests: nas.invalid wants jo's password "right" for everything but its Public share; asleep.invalid never answers.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::files::leave::Leave;
use crate::common::keyring_said;
use crate::common::smb::{self, Login, Problem};
use super::kept::Ring;
use super::types::Confirmed;
use super::rule::{KeptRow, SEEN_MOST, mark, now, time_left};

type Call = (String, Option<String>, Option<String>);

type Line = (String, u64, String);

#[derive(Default)]
pub struct Inside {
    kept: HashMap<String, (Login, Vec<Line>)>,
    unsaid: Option<String>,
    late: Option<Result<(), String>>,
    slow: std::time::Duration,
    with_it: usize,
    most: usize,
}

impl Inside {
    fn unsaid(&self) -> Result<(), String> {
        self.unsaid.clone().map_or(Ok(()), Err)
    }
}

pub type Kept = std::sync::Arc<std::sync::Mutex<Inside>>;

thread_local! {
    static RING: std::cell::RefCell<Kept> = std::cell::RefCell::new(Kept::default());
    static SENT: std::cell::RefCell<Vec<Call>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub fn ring() -> Kept {
    RING.with_borrow(Clone::clone)
}

pub fn share_ring(ring: Kept) {
    RING.with_borrow_mut(|r| *r = ring);
}

fn inside<T>(ring: &Kept, f: impl FnOnce(&mut Inside) -> T) -> T {
    f(&mut ring.lock().unwrap_or_else(|e| e.into_inner()))
}

fn mine<T>(f: impl FnOnce(&mut Inside) -> T) -> T {
    inside(&ring(), f)
}

pub fn slow(long: std::time::Duration) {
    mine(|k| k.slow = long);
}

pub fn most_at_once() -> usize {
    mine(|k| k.most)
}

pub fn kept(server: &str, login: Login, seen: &[&str]) {
    let seen: Vec<(&str, u64)> = seen.iter().map(|s| (*s, now())).collect();
    kept_on(server, login, &seen);
}

pub fn kept_on(server: &str, login: Login, seen: &[(&str, u64)]) {
    let who = mark(&login);
    mine(|k| k.kept.insert(server.to_lowercase(), (login, seen.iter().map(|(s, when)| (s.to_string(), *when, who.clone())).collect())));
}

pub use crate::files::leave::pretend::{dns, dns_forgets};

pub fn keyring_cannot_say(why: Option<&str>) {
    mine(|k| k.unsaid = why.map(str::to_string));
}

pub fn nothing_kept() {
    mine(|k| k.kept.clear());
}

pub fn keeps_late(how: Result<(), String>) {
    mine(|k| k.late = Some(how));
}

pub fn kept_login(server: &str, share: Option<&str>) -> Result<Option<Login>, String> {
    Ring::here().find(server, share)
}

pub fn kept_lines(server: &str, login: Login, lines: &[(&str, u64, &str)]) {
    mine(|k| k.kept.insert(server.to_lowercase(), (login, lines.iter().map(|(at, when, who)| (at.to_string(), *when, who.to_string())).collect())));
}

pub fn seen_when(server: &str) -> Vec<(String, u64)> {
    mine(|k| k.kept.get(&server.to_lowercase()).map(|(_, seen)| seen.iter().map(|(at, when, _)| (at.clone(), *when)).collect()).unwrap_or_default())
}

pub fn seen_for(server: &str, login: &Login) -> Result<Vec<(String, u64)>, String> {
    Ring::here().seen_for(server, &mark(login))
}

pub fn seen_at(server: &str) -> Vec<String> {
    seen_when(server).into_iter().map(|(at, _)| at).collect()
}

impl Ring {
    pub(super)fn here() -> Ring {
        Ring { pretend: ring() }
    }

    pub(super)fn all(&self) -> Result<Vec<KeptRow>, String> {
        inside(&self.pretend, |k| {
            k.unsaid()?;
            let mut rows: Vec<KeptRow> = k.kept.iter().map(|(server, (login, seen))| KeptRow {
                server: server.clone(),
                users: login.user.clone(),
                left: time_left(&seen.iter().map(|l| l.1).collect::<Vec<_>>(), now()),
            }).collect();
            rows.sort_by(|a, b| a.server.cmp(&b.server));
            Ok(rows)
        })
    }

    pub(super)fn forget(&self, server: &str) -> Result<(), String> {
        inside(&self.pretend, |k| {
            k.unsaid()?;
            k.kept.remove(&server.to_lowercase());
            Ok(())
        })
    }

    pub(super)fn find(&self, server: &str, _share: Option<&str>) -> Result<Option<Login>, String> {
        inside(&self.pretend, |k| {
            k.unsaid()?;
            Ok(k.kept.get(&server.to_lowercase()).map(|(l, _)| l.clone()))
        })
    }

    pub(super)fn seen_for(&self, server: &str, who: &str) -> Result<Vec<(String, u64)>, String> {
        inside(&self.pretend, |k| {
            k.unsaid()?;
            Ok(k.kept.get(&server.to_lowercase()).map(|(_, seen)| seen.iter().filter(|s| s.2 == who || s.2.is_empty())
                .map(|(at, when, whose)| (at.clone(), if whose.is_empty() { 0 } else { *when })).take(SEEN_MOST).collect()).unwrap_or_default())
        })
    }

    pub(super)fn keep(&self, server: &str, _share: Option<&str>, login: &Login, at: Option<&Confirmed>, forget: bool) -> Result<(), String> {
        let (late, slow) = inside(&self.pretend, |k| {
            k.unsaid()?;
            k.with_it += 1;
            k.most = k.most.max(k.with_it);
            Ok::<_, String>((k.late.take(), k.slow))
        })?;
        if late.is_some() {
            std::thread::sleep(keyring_said::DEADLINE + std::time::Duration::from_millis(300));
        }
        if !matches!(late, Some(Err(_))) {
            let mut lines = inside(&self.pretend, |k| k.kept.get(&server.to_lowercase()).map(|(_, seen)| seen.clone()).unwrap_or_default());
            std::thread::sleep(slow);
            if let Some(at) = at.map(Confirmed::address) {
                let who = mark(login);
                if forget {
                    lines.retain(|s| s.2 != who || s.0 == at);
                }
                lines.retain(|s| !(s.0 == at && s.2 == who));
                lines.insert(0, (at.to_string(), now(), who));
            }
            inside(&self.pretend, |k| k.kept.insert(server.to_lowercase(), (login.clone(), lines)));
        }
        inside(&self.pretend, |k| k.with_it -= 1);
        late.unwrap_or(Ok(()))
    }
}

pub fn sent() -> Vec<Call> {
    SENT.with_borrow(|s| s.clone())
}

fn note(server: &str, ip: Option<&str>, login: Option<&Login>) {
    SENT.with_borrow_mut(|s| s.push((server.to_string(), ip.map(str::to_string), login.map(|l| l.user.clone()))));
}

pub fn folder(share: &str) -> PathBuf {
    let test: String = format!("{:?}", std::thread::current().id()).chars().filter(char::is_ascii_digit).collect();
    std::env::temp_dir().join(format!("trindle-files-shares-{}-{test}", std::process::id())).join(smb::folder_name("nas.invalid", share))
}

fn check(server: &str, login: Option<&Login>, guest_ok: bool) -> Result<(), Problem> {
    match (server, login) {
        ("asleep.invalid", _) => Err(Problem::Asleep),
        ("nas.invalid", None) if guest_ok => Ok(()),
        ("nas.invalid", None) => Err(Problem::NeedsLogin),
        ("nas.invalid", Some(l)) if l.user == "jo" && l.password.as_str() == "right" => Ok(()),
        ("nas.invalid", Some(_)) => Err(Problem::WrongLogin),
        _ => Err(Problem::NoComputer),
    }
}

pub fn list_on(server: &str, to: &Leave<Option<Login>>) -> Result<Vec<String>, Problem> {
    let (ip, login) = (to.address(), to.login().as_ref());
    note(server, ip, login);
    check(server, login, false)?;
    Ok(vec!["Photos".into(), "Public".into()])
}

pub fn mount_on(server: &str, share: &str, to: &Leave<Option<Login>>) -> Result<PathBuf, Problem> {
    let (ip, login) = (to.address(), to.login().as_ref());
    note(server, ip, login);
    check(server, login, share == "Public")?;
    if !["Photos", "Public"].contains(&share) {
        return Err(Problem::NoShare);
    }
    let at = folder(share);
    std::fs::create_dir_all(&at).map_err(|e| Problem::Other(e.to_string()))?;
    Ok(at)
}

pub fn unmount_on(at: &Path) -> Result<(), String> {
    std::fs::remove_dir_all(at).map_err(|e| e.to_string())
}

pub fn nfs_export_on(server: &str, export: &str) -> Result<PathBuf, Problem> {
    match (server, export) {
        ("asleep.invalid", _) => Err(Problem::Asleep),
        ("nas.invalid", "/srv/media" | "/") => {
            let test: String = format!("{:?}", std::thread::current().id()).chars().filter(char::is_ascii_digit).collect();
            let at = std::env::temp_dir().join(format!("trindle-files-nfs-{}-{test}", std::process::id())).join(smb::nfs_folder_name(server, export));
            std::fs::create_dir_all(&at).map_err(|e| Problem::Other(e.to_string()))?;
            Ok(at)
        }
        ("nas.invalid", _) => Err(Problem::NoShare),
        _ => Err(Problem::NoComputer),
    }
}
