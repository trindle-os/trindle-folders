// What the shares code is told and tells: the sign-in card, the messages, and what an opening reports.

use std::collections::HashMap;
use std::path::PathBuf;

use zeroize::Zeroizing;

use crate::files::app::Msg;
use crate::files::leave::Card;
use crate::files::remembered::Remembered;
use crate::common::keyring_said;
use crate::common::smb::{Login, Mounted, Problem};
use super::rule::KeptRow;

pub struct Ask {
    pub server: String,
    pub share: Option<String>,
    pub rest: String,
    pub user: String,
    pub password: Zeroizing<String>,
    pub remember: bool,
    pub why: String,
    pub keyring: Option<String>,
    pub failed: bool,
    pub busy: bool,
    pub tab: usize,
    pub pane: usize,
    pub address: Option<String>,
    pub held: bool,
    pub heard: bool,
    pub before: Vec<String>,
    pub forget: bool,
    pub filled: Option<(String, bool)>,
}

impl Card for Ask {
    fn shows(&self) -> Option<String> {
        self.address.clone()
    }
}

#[derive(Default)]
pub struct Shares {
    pub(super) logins: HashMap<String, Used>,
    pub(super) typed: HashMap<String, Login>,
    pub mounted: Vec<Mounted>,
    pub remembered: Vec<Remembered>,
    pub remembered_file: Option<PathBuf>,
    pub(super) waiting: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
pub enum ShareMsg {
    Listed { t: usize, p: usize, asked: String, tried: Tried, got: Result<(Vec<String>, Option<Login>), Problem> },
    NfsMounted { t: usize, p: usize, asked: String, server: String, export: String, rest: String, got: Result<PathBuf, Problem> },
    Mounted { t: usize, p: usize, server: String, share: String, rest: String, tried: Tried, got: Result<(PathBuf, Option<Login>), Problem> },
    User(String),
    Password(String),
    Remember(bool),
    Forget(bool),
    Go,
    Close,
    KeptLate(Result<(), String>),
    Show(usize),
    Eject(usize),
    Ejected(PathBuf, Result<(), String>),
    KeptRows(Result<Vec<KeptRow>, String>),
    Forgot(String, Result<(), String>),
    Reconnect(usize),
    ForgetShare(usize),
    ReconnectAll,
}

pub(super) const QUIET: usize = usize::MAX;

pub(super) fn share_msg(m: ShareMsg) -> Msg {
    Msg::Share(m)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tried {
    pub address: Option<String>,
    pub held: Option<Vec<String>>,
    pub heard: bool,
    pub lapsed: bool,
    pub unknown: bool,
    pub unshown: bool,
    pub confirmed: bool,
    pub waiting: Option<Login>,
    pub again: bool,
    pub kept: bool,
    pub keyring: Option<String>,
    pub late: Option<Heard>,
}

#[derive(Clone)]
pub struct Heard(std::sync::Arc<std::sync::Mutex<Option<keyring_said::Late<()>>>>);

impl Heard {
    pub(super) fn of(late: keyring_said::Late<()>) -> Heard {
        Heard(std::sync::Arc::new(std::sync::Mutex::new(Some(late))))
    }

    pub(super) fn wait(&self) -> Result<(), String> {
        let late = self.0.lock().unwrap_or_else(|e| e.into_inner()).take();
        late.map_or_else(|| Err(keyring_said::never_words().to_string()), keyring_said::Late::wait)
    }
}

impl PartialEq for Heard {
    fn eq(&self, other: &Heard) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Heard {}

impl std::fmt::Debug for Heard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Heard")
    }
}

pub use crate::files::leave::{Confirmed, Trust};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Used {
    pub login: Login,
    pub at: String,
    pub confirmed: bool,
}

pub(super) enum Reach {
    Look(Option<String>),
    Pinned(Confirmed, bool),
}
