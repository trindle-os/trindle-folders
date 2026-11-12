//! Files' side of servers and phones: copies, moves and deletes that reach them. A far item's path is its address, such as `sftp://jo@example.com/home/jo/notes.txt`.

use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use crate::toolkit::t;

use super::remote::{Address, Scheme};
use super::work::Plan;

mod card;
mod copy;
mod ends;
pub(in crate::files) use ends::missing;
#[cfg(test)]
mod fake;
mod hands;
mod links;
mod msg;
mod put;
#[cfg(test)]
mod tests;
mod walk;
mod window;

#[cfg(test)]
pub(super) const SOURCE: &[&str] = &[include_str!("mod.rs"), include_str!("card.rs"), include_str!("copy.rs"), include_str!("ends.rs"), include_str!("fake.rs"), include_str!("hands.rs"), include_str!("links.rs"), include_str!("msg.rs"), include_str!("put.rs"), include_str!("walk.rs"), include_str!("window.rs")];

pub use card::{view, Connect};
#[cfg(test)]
pub use card::{Question, Why};
pub use links::Links;
pub use walk::run;
pub use window::{Far, FarMsg, Listed, Then};
pub(in crate::files) use window::forget_fetched;

fn lock<T: ?Sized>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn is_far(p: &Path) -> bool {
    let b = p.as_os_str().as_bytes();
    ["sftp://", "ftp://", "ftps://", "ftpes://", "dav://", "davs://", "mtp://", "smb://", "nfs://"].iter().any(|s| b.starts_with(s.as_bytes()))
}

pub fn address(p: &Path) -> Option<Address> {
    if !is_far(p) {
        return None;
    }
    Address::parse(&p.to_string_lossy()).ok()
}

fn path_of(a: &Address) -> &str {
    if a.path.is_empty() { "/" } else { &a.path }
}

pub fn root(a: &Address) -> String {
    Address { path: String::new(), password: None, ..a.clone() }.to_string()
}

pub fn involved(plan: &Plan) -> bool {
    is_far(&plan.to) || plan.items.iter().any(|p| is_far(p))
}

fn not_all_arrived() -> &'static str {
    t!("folders-far-not-all-arrived")
}
#[cfg(test)]
const SPARE: &str = crate::common::files::SPARE_END;
fn not_followed() -> &'static str {
    t!("folders-far-not-followed")
}
fn not_sent() -> &'static str {
    t!("folders-far-not-sent")
}
fn not_told() -> &'static str {
    t!("folders-far-not-told")
}

fn warns_cannot_refuse(scheme: Scheme) -> bool {
    matches!(scheme, Scheme::Ftp | Scheme::Ftps | Scheme::Ftpes | Scheme::Mtp)
}

pub fn ftp_cannot_refuse() -> &'static str {
    t!("folders-far-ftp-cannot-refuse")
}
