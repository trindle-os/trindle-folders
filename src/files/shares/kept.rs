// Logins kept in the keyring and held in memory, each piece of keyring work done in its turn.

use crate::files::leave::{self, Leave, Reached, Way};
use crate::common::keyring_said;
use crate::common::smb::Login;
use crate::toolkit::t;
use super::types::{Confirmed, Trust};
#[cfg(not(test))]
use super::keyring;
#[cfg(test)]
use super::pretend;
use super::rule::{KeptRow, Sends, choose, mark, now};
use super::types::{Heard, Reach, Tried, Used};

#[cfg(not(test))]
fn keyring_bus() -> Result<zbus::blocking::Connection, String> {
    zbus::blocking::connection::Builder::session()
        .and_then(|b| b.method_timeout(crate::common::keyring_said::WAIT).build())
        .map_err(|e| t!("folders-shares-keyring-unreachable", why = &e.to_string()))
}

#[derive(Clone)]
pub(super) struct Ring {
    #[cfg(test)]
    pub(super) pretend: pretend::Kept,
}

#[cfg(not(test))]
impl Ring {
    fn here() -> Ring {
        Ring {}
    }

    fn find(&self, server: &str, share: Option<&str>) -> Result<Option<Login>, String> {
        keyring::find(&keyring_bus()?, server, share)
    }

    fn seen_for(&self, server: &str, who: &str) -> Result<Vec<(String, u64)>, String> {
        keyring::seen_for(&keyring_bus()?, server, who)
    }

    fn all(&self) -> Result<Vec<KeptRow>, String> {
        keyring::all(&keyring_bus()?, now())
    }

    fn forget(&self, server: &str) -> Result<(), String> {
        keyring::forget(&keyring_bus()?, server)
    }

    fn keep(&self, server: &str, share: Option<&str>, login: &Login, at: Option<&Confirmed>, forget: bool) -> Result<(), String> {
        let c = keyring_bus()?;
        keyring::keep(&c, server, share, login)?;
        match at {
            Some(at) => keyring::saw_at(&c, server, at.address(), now(), forget, &mark(login)),
            None => Ok(()),
        }
    }
}

pub(super) fn kept_login(server: &str, share: Option<&str>) -> Result<Option<Login>, String> {
    let (ring, server, share) = (Ring::here(), server.to_string(), share.map(str::to_string));
    keyring_said::within(move || ring.find(&server, share.as_deref()))
}

pub(super) fn seen_for(server: &str, login: &Login) -> Result<Vec<(String, u64)>, String> {
    let (ring, server, who) = (Ring::here(), server.to_string(), mark(login));
    keyring_said::within(move || ring.seen_for(&server, &who))
}

pub fn kept_rows() -> Result<Vec<KeptRow>, String> {
    let ring = Ring::here();
    keyring_said::within(move || ring.all())
}

pub fn forget_server(server: &str) -> Result<(), String> {
    let (ring, server) = (Ring::here(), server.to_string());
    keyring_said::within(move || ring.forget(&server))
}

pub(super) fn keep_login(server: &str, share: Option<&str>, login: &Login, at: Option<&Confirmed>, forget: bool) -> keyring_said::Waited<()> {
    let (ring, server, share, login, at) = (Ring::here(), server.to_string(), share.map(str::to_string), login.clone(), at.cloned());
    keyring_said::within_late(move || ring.keep(&server, share.as_deref(), &login, at.as_ref(), forget))
}

pub(super) fn settle(server: &str, share: Option<&str>, to: Reach, typed: Option<Login>, used: Option<Used>) -> (Tried, Option<Leave<Option<Login>>>) {
    let (way, trust, heard) = match to {
        Reach::Look(found) => match Reached::now(server, found) {
            Some(reached) => {
                let trust = reached.trust();
                (Way::At(reached), trust, trust == Trust::Heard)
            }
            None => (Way::Nowhere, Trust::Heard, true),
        },
        Reach::Pinned(at, heard) => (Way::At(at.reached()), Trust::Shown, heard),
    };
    let address = match &way {
        Way::At(reached) => Some(reached.address().to_string()),
        Way::Nowhere | Way::Proved => None,
    };
    let unsaid = std::cell::RefCell::new(None);
    let kept = || kept_login(server, share).unwrap_or_else(|why| {
        *unsaid.borrow_mut() = Some(why);
        None
    });
    let seen = |login: &Login| seen_for(server, login).unwrap_or_else(|why| {
        *unsaid.borrow_mut() = Some(why);
        Vec::new()
    });
    let mut tried = Tried { address, heard, ..Tried::default() };
    let (sends, confirmed) = choose(server, tried.address.as_deref(), trust, typed, used, kept, seen, now());
    tried.keyring = unsaid.into_inner();
    match sends {
        Sends::Login(login) => match leave::leave(login, &way) {
            Ok(to) => (Tried { confirmed, ..tried }, Some(to)),
            Err(login) => (Tried { waiting: login, ..tried }, None),
        },
        Sends::Again(login, kept) => (Tried { waiting: Some(login), again: true, kept, ..tried }, None),
        Sends::Held(before) => (Tried { held: Some(before), ..tried }, None),
        Sends::Lapsed(before) => (Tried { held: Some(before), lapsed: true, ..tried }, None),
        Sends::Unknown(before) => (Tried { held: Some(before), unknown: true, ..tried }, None),
        Sends::Unshown(before) => (Tried { held: Some(before), unshown: true, ..tried }, None),
        Sends::Confirm(login) => (Tried { waiting: Some(login), ..tried }, None),
    }
}

pub(super) fn worked(server: &str, share: Option<&str>, login: &Login, at: Option<&Confirmed>, keep: Option<bool>, fresh: bool) -> (Option<String>, Option<Heard>) {
    let (Some(forget), true) = (keep, fresh) else { return (None, None) };
    let waited = keep_login(server, share, login, at, forget);
    match (waited.now, waited.late) {
        (_, Some(late)) => (None, Some(Heard::of(late))),
        (Err(why), None) => (Some(why), None),
        (Ok(()), None) => (None, None),
    }
}

pub(super) fn unkept_words(why: &str) -> String {
    t!("folders-shares-unkept", why = why)
}
