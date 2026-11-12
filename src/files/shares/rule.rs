//! The one rule for what is sent to a computer at an address, and the words the card says of it.

use crate::common::smb::Login;
use crate::toolkit::t;
use super::types::Trust;
use super::types::Used;

pub const LAPSE: u64 = 90 * 24 * 60 * 60;
pub const SEEN_MOST: usize = 8;

pub fn mark(login: &Login) -> String {
    use sha2::{Digest, Sha256};
    let mut sha = Sha256::new();
    for part in ["trindle smb login", login.domain.as_str(), login.user.as_str(), login.password.as_str()] {
        sha.update(part.as_bytes());
        sha.update([0]);
    }
    sha.finalize()[..8].iter().map(|b| format!("{b:02x}")).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeptRow {
    pub server: String,
    pub users: String,
    pub left: u64,
}

pub fn left_words(left: u64) -> String {
    match left {
        0 => t!("folders-shares-left-none").to_string(),
        l if l < 24 * 60 * 60 => t!("folders-shares-left-under-day").to_string(),
        l => t!("folders-shares-left-days", n = l.div_ceil(24 * 60 * 60)),
    }
}

pub(super) fn time_left(times: &[u64], now: u64) -> u64 {
    times.iter().copied().filter(|w| *w > 0).max().map_or(0, |when| (when + LAPSE).saturating_sub(now))
}

pub(super) fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

#[derive(Debug, PartialEq, Eq)]
pub enum Sends {
    Login(Option<Login>),
    Held(Vec<String>),
    Lapsed(Vec<String>),
    Unknown(Vec<String>),
    Unshown(Vec<String>),
    Confirm(Login),
    Again(Login, bool),
}

pub(super) fn same_address(a: &str, b: &str) -> bool {
    matches!((a.parse::<std::net::IpAddr>(), b.parse::<std::net::IpAddr>()), (Ok(a), Ok(b)) if a == b)
}

pub fn choose(address: Option<&str>, trust: Trust, typed: Option<Login>, used: Option<Used>,
              kept: impl FnOnce() -> Option<Login>, seen: impl FnOnce(&Login) -> Vec<(String, u64)>, now: u64) -> (Sends, bool) {
    if let Some(login) = typed {
        return match (address, trust) {
            (None, _) => (Sends::Held(Vec::new()), false),
            (Some(_), Trust::Heard) => (Sends::Confirm(login), false),
            (Some(_), Trust::Given) => (Sends::Login(Some(login)), false),
            (Some(_), Trust::Shown) => (Sends::Login(Some(login)), true),
        };
    }
    let here = |at: &str| address.is_some_and(|a| same_address(a, at));
    let mut unshown = false;
    if let Some(u) = used.as_ref().filter(|u| here(&u.at)) {
        match (trust, u.confirmed) {
            (Trust::Heard, true) => {
                let also_kept = kept().is_some_and(|k| k == u.login);
                return (Sends::Again(u.login.clone(), also_kept), false);
            }
            (Trust::Heard, false) => unshown = true,
            (Trust::Given | Trust::Shown, confirmed) => return (Sends::Login(Some(u.login.clone())), confirmed),
        }
    }
    let before: Vec<String> = used.map(|u| u.at).into_iter().collect();
    match kept() {
        Some(login) => {
            let seen = seen(&login);
            kept_sends(login, seen, address, trust, now, (before, unshown))
        }
        None if before.is_empty() => (Sends::Login(None), false),
        None if unshown => (Sends::Unshown(before), false),
        None => (Sends::Held(before), false),
    }
}

fn kept_sends(login: Login, seen: Vec<(String, u64)>, address: Option<&str>, trust: Trust, now: u64, (mut before, unshown): (Vec<String>, bool)) -> (Sends, bool) {
    let here = |at: &str| address.is_some_and(|a| same_address(a, at));
    let (mut lapsed, mut unknown) = (false, false);
    for (at, when) in &seen {
        if here(at) {
            if *when == 0 {
                unknown = true;
            } else if now.saturating_sub(*when) <= LAPSE {
                return match trust {
                    Trust::Heard => (Sends::Again(login, true), false),
                    Trust::Given | Trust::Shown => (Sends::Login(Some(login)), true),
                };
            } else {
                lapsed = true;
            }
        }
    }
    for (at, _) in seen {
        if !before.iter().any(|b| same_address(b, &at)) {
            before.push(at);
        }
    }
    let held = if lapsed {
        Sends::Lapsed(before)
    } else if unknown {
        Sends::Unknown(before)
    } else if unshown {
        Sends::Unshown(before)
    } else {
        Sends::Held(before)
    };
    (held, false)
}

pub fn held_words(server: &str, now: Option<&str>, before: &[String]) -> String {
    let Some(now) = now else { return nowhere_words(server) };
    let (past, full) = (if before.is_empty() { "no" } else { "yes" }, if before.len() >= SEEN_MOST { "yes" } else { "no" });
    t!("folders-shares-held", server = server, now = now, past = past, before = &before.join(", "), full = full, most = SEEN_MOST)
}

pub fn nowhere_words(server: &str) -> String {
    t!("folders-shares-nowhere", server = server)
}

pub fn lapsed_words(server: &str, now: &str) -> String {
    t!("folders-shares-lapsed", server = server, now = now, days = LAPSE / (24 * 60 * 60))
}

pub fn unknown_words(server: &str, now: &str) -> String {
    t!("folders-shares-unknown", server = server, now = now)
}

pub fn unshown_words(server: &str, now: &str) -> String {
    t!("folders-shares-unshown", server = server, now = now)
}

pub fn asking_words(server: &str, heard: Option<&str>) -> String {
    t!("folders-shares-asking", server = server, heard = if heard.is_some() { "yes" } else { "no" }, at = heard.unwrap_or(""))
}

pub fn where_words(address: &str, heard: bool) -> String {
    t!("folders-shares-where", address = address, heard = if heard { "yes" } else { "no" })
}

pub fn again_words(server: &str, at: &str) -> String {
    t!("folders-shares-again", server = server, at = at, port = crate::files::nearby::SMB_PORT)
}
