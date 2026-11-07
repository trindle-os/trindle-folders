// What the window knows of shares: those mounted and remembered, pins in them, and putting them back.

use std::path::Path;

use crate::files::app::{Files, Msg};
use crate::files::remembered::{self, Remembered};
use crate::files::remote::Address;
use crate::common::smb::{Login, Mounted};
use crate::toolkit::t;
use super::commands::mounted_now;
use super::system::nfs_export_on;
use super::types::Reach;
use super::kept::{forget_server, kept_rows};
use super::opening::typed_login;
use super::types::{QUIET, ShareMsg, Shares};

impl Shares {
    pub fn refresh(&mut self) {
        if let Some(now) = mounted_now() {
            self.mounted = now;
        }
    }

    pub fn open_at(&self, server: &str, share: &str) -> Option<&Mounted> {
        self.mounted.iter().find(|m| m.server.eq_ignore_ascii_case(server) && m.share.eq_ignore_ascii_case(share))
    }

    pub fn open_nfs(&self, server: &str, export: &str) -> Option<&Mounted> {
        self.mounted.iter().find(|m| m.is_nfs() && m.server.eq_ignore_ascii_case(server) && m.share == export)
    }

    pub fn holding(&self, at: &Path) -> Option<&Mounted> {
        self.mounted.iter().find(|m| at.starts_with(&m.at))
    }
}

impl Files {
    pub(in crate::files) fn down_shares(&self) -> Vec<usize> {
        (0..self.shares.remembered.len()).filter(|&i| !self.share_up(&self.shares.remembered[i])).collect()
    }

    pub(super) fn share_up(&self, r: &Remembered) -> bool {
        if r.is_nfs() { self.shares.open_nfs(&r.server, &r.share).is_some() } else { self.shares.open_at(&r.server, &r.share).is_some() }
    }

    pub(in crate::files) fn pin_down(&self, pin: &Path) -> Option<usize> {
        let named = remembered::from_folder(pin);
        self.down_shares().into_iter().find(|&i| {
            let r = &self.shares.remembered[i];
            r.holds(pin) || named.as_ref().is_some_and(|n| r.is(&n.server, &n.share))
        })
    }

    pub(in crate::files) fn remember_pinned_shares(&mut self) {
        let mut changed = false;
        for pin in self.pinned.clone() {
            let Some(n) = remembered::from_folder(&pin) else { continue };
            let known = self.shares.remembered.iter().any(|r| r.holds(&pin) || r.is(&n.server, &n.share));
            if known || (n.at.exists() && self.shares.holding(&n.at).is_none()) {
                continue;
            }
            self.shares.remembered.push(n);
            changed = true;
        }
        if changed {
            self.save_shares();
        }
    }

    pub(in crate::files) fn remember_all_mounted(&mut self) {
        for m in self.shares.mounted.clone() {
            self.remember_mounted(&m.server, &m.share, &m.at);
        }
    }

    pub(super) fn remember_mounted(&mut self, server: &str, share: &str, at: &Path) {
        let (old, changed) = remembered::note(&mut self.shares.remembered, server, share, at);
        if let Some(old) = old {
            let mut moved = false;
            for pin in &mut self.pinned {
                if let Some(now) = remembered::follow(pin, &old, at) {
                    if let Some(name) = self.pin_names.remove(pin) {
                        self.pin_names.insert(now.clone(), name);
                    }
                    *pin = now;
                    moved = true;
                }
            }
            if moved {
                self.save_pins();
            }
        }
        if changed {
            self.save_shares();
        }
    }

    pub(super) fn save_shares(&mut self) {
        let Some(file) = &self.shares.remembered_file else { return };
        if remembered::write(file, &self.shares.remembered).is_err() {
            self.say_help(t!("folders-shares-not-remembered"), crate::files::help::NOT_KEPT);
        }
    }

    pub(in crate::files) fn forget_unpinned_shares(&mut self) {
        let before = self.shares.remembered.len();
        let (pinned, mounted) = (self.pinned.clone(), self.shares.mounted.clone());
        self.shares.remembered.retain(|r| pinned.iter().any(|p| r.holds(p)) || mounted.iter().any(|m| m.at == r.at));
        if self.shares.remembered.len() != before {
            self.save_shares();
        }
    }

    pub(super) fn reconnect(&mut self, r: Remembered, rest: String, quiet: bool) {
        let (t, p) = if quiet { (QUIET, 0) } else { (self.cols[self.side()].tab, self.side()) };
        self.say(t!("folders-shares-reconnecting", name = &r.name()), false);
        if r.is_nfs() {
            let asked = format!("nfs://{}{}", r.server, r.share);
            let (at, which, server, export) = (r.server.clone(), r.share.clone(), r.server, r.share);
            self.share_off(move || nfs_export_on(&at, &which), move |got| ShareMsg::NfsMounted { t, p, asked, server, export, rest, got });
        } else {
            self.smb_mount(t, p, r.server, r.share, rest, None, None, Reach::Look(None));
        }
    }

    pub(super) fn reconnect_next(&mut self, t: usize) {
        if t != QUIET || self.shares.waiting.is_empty() {
            return;
        }
        let (server, share) = self.shares.waiting.remove(0);
        if let Some(r) = self.shares.remembered.iter().find(|r| r.is(&server, &share)).cloned() {
            self.reconnect(r, String::new(), true);
        } else {
            self.reconnect_next(t);
        }
    }

    pub(in crate::files) fn reconnect_pin(&mut self, pin: &Path) -> bool {
        let Some(i) = self.pin_down(pin) else { return false };
        let r = self.shares.remembered[i].clone();
        let root = if r.holds(pin) { r.at.clone() } else { remembered::from_folder(pin).map(|n| n.at).unwrap_or_default() };
        let rest = pin.strip_prefix(&root).map(|x| x.to_string_lossy().into_owned()).unwrap_or_default();
        self.reconnect(r, rest, false);
        true
    }

    pub(in crate::files) fn shares_at_start(&mut self) {
        if self.shares.remembered.is_empty() {
            return;
        }
        let on = self.own_file.as_deref().is_some_and(|f| crate::common::folders_conf::get_in(f, crate::common::folders_conf::RECONNECT) == "yes");
        if on {
            self.share_msg(ShareMsg::ReconnectAll);
        }
    }

    pub(super) fn share_off<T: Send + 'static>(&mut self, job: impl FnOnce() -> T + Send + 'static, done: impl FnOnce(T) -> ShareMsg + Send + 'static) {
        match &self.post {
            Some(post) => {
                let post = post.clone();
                let _ = std::thread::Builder::new().name("folders-shares".into()).spawn(move || post.send(Msg::Share(done(job()))));
            }
            None => {
                let m = done(job());
                self.share_msg(m);
            }
        }
    }

    pub(in crate::files) fn load_kept(&mut self) {
        self.share_off(kept_rows, ShareMsg::KeptRows);
    }

    pub(in crate::files) fn forget_kept(&mut self, server: String) {
        let asked = server.clone();
        self.share_off(move || forget_server(&asked), move |got| ShareMsg::Forgot(server, got));
    }

    pub(super) fn address_of(&self, server: &str) -> Option<String> {
        self.neighbours.iter().find(|c| c.name.eq_ignore_ascii_case(server)).and_then(|c| c.address.clone())
    }

    pub(super) fn typed_for(&mut self, a: &Address) -> Option<Login> {
        let card = self.shares.typed.remove(&a.host.to_lowercase());
        typed_login(a).or(card)
    }
}
