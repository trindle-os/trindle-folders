// Opening a computer or a share: listing it, mounting it, and the sign-in card when it asks.

use std::path::PathBuf;

use zeroize::Zeroizing;

use crate::files::app::Files;
use crate::files::dialogs::Dialog;
use crate::files::leave::Leave;
use crate::files::places::Where;
use crate::files::remote::Address;
use crate::common::keyring_said;
use crate::common::smb::{self, Login, Mounted, Problem};
use crate::toolkit::t;
use super::commands::parts;
use super::kept::{settle, unkept_words, worked};
use super::rule::{again_words, asking_words, held_words, lapsed_words, mark, nowhere_words, same_address, unknown_words, unshown_words};
use super::types::{Ask, Reach, ShareMsg, Tried, Used};
use super::system::list_on;
use super::system::nfs_export_on;
use super::system::mount_on;

impl Files {
    pub(in crate::files) fn smb_load(&mut self, t: usize, p: usize, a: Address) {
        let (server, share, _) = parts(&a);
        if share.is_some() {
            self.fill(t, p, Vec::new());
            return self.smb_open(p, &a);
        }
        let asked = a.to_string();
        if !self.far.waiting.contains(&asked) {
            self.far.waiting.push(asked.clone());
        }
        let typed = self.typed_for(&a);
        self.smb_list(t, p, server, asked, typed, None, Reach::Look(None));
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn smb_list(&mut self, t: usize, p: usize, server: String, asked: String, typed: Option<Login>, keep: Option<bool>, to: Reach) {
        let at = server.clone();
        self.share_send(server, None, typed, keep, to, move |to| list_on(&at, to), move |tried, got| ShareMsg::Listed { t, p, asked, tried, got });
    }

    fn held_for(&self, server: &str) -> Option<Used> {
        self.shares.logins.get(&server.to_lowercase()).cloned()
    }

    #[allow(clippy::too_many_arguments)]
    fn share_send<T: Send + 'static>(&mut self, server: String, share: Option<String>, typed: Option<Login>, keep: Option<bool>, to: Reach,
                                     send: impl FnOnce(&Leave<Option<Login>>) -> Result<T, Problem> + Send + 'static,
                                     done: impl FnOnce(Tried, Result<(T, Option<Login>), Problem>) -> ShareMsg + Send + 'static) {
        let to = match to {
            Reach::Look(_) => Reach::Look(self.address_of(&server)),
            pinned => pinned,
        };
        let used = self.held_for(&server);
        self.share_off(move || {
            let fresh = typed.is_some();
            let confirmed = match &to {
                Reach::Pinned(at, _) => Some(at.clone()),
                Reach::Look(_) => None,
            };
            let (mut tried, go) = settle(&server, share.as_deref(), to, typed, used);
            let Some(to) = go else { return (tried, Err(Problem::NeedsLogin)) };
            let got = send(&to);
            if let (Ok(_), Some(l)) = (&got, to.login()) {
                (tried.keyring, tried.late) = worked(&server, share.as_deref(), l, confirmed.as_ref(), keep, fresh);
            }
            (tried, got.map(|found| (found, to.login().clone())))
        }, move |(tried, got)| done(tried, got));
    }

    pub(in crate::files) fn nfs_load(&mut self, t: usize, p: usize, a: Address) {
        self.fill(t, p, Vec::new());
        let export = match a.path.trim_end_matches('/') {
            "" => "/".to_string(),
            path => path.to_string(),
        };
        if !smb::valid_server(&a.host) {
            return self.say_help(t!("folders-shares-bad-computer"), crate::files::help::SHARE_NOT_OPENED);
        }
        if !smb::valid_export(&export) {
            return self.say_help(t!("folders-shares-bad-path"), crate::files::help::SHARE_NOT_OPENED);
        }
        self.shares.refresh();
        if let Some(m) = self.shares.open_nfs(&a.host, &export) {
            let at = m.at.clone();
            self.remember_mounted(&a.host, &export, &at);
            return self.share_arrived(t, p, &a, at);
        }
        self.say(t!("folders-shares-opening-export", export = &export, host = &a.host), false);
        let (asked, server) = (a.to_string(), a.host.clone());
        let (at, which) = (server.clone(), export.clone());
        self.share_off(move || nfs_export_on(&at, &which), move |got| ShareMsg::NfsMounted { t, p, asked, server, export, rest: String::new(), got });
    }

    pub(in crate::files) fn smb_open(&mut self, p: usize, a: &Address) {
        let (server, Some(share), rest) = parts(a) else { return };
        self.shares.refresh();
        if let Some(m) = self.shares.open_at(&server, &share) {
            let at = m.at.clone();
            self.remember_mounted(&server, &share, &at);
            return self.share_arrived(self.cols[p].tab, p, a, at.join(&rest));
        }
        let typed = self.typed_for(a);
        self.say(t!("folders-shares-opening-share", share = &share, server = &server), false);
        self.smb_mount(self.cols[p].tab, p, server, share, rest, typed, None, Reach::Look(None));
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn smb_mount(&mut self, t: usize, p: usize, server: String, share: String, rest: String, typed: Option<Login>, keep: Option<bool>, to: Reach) {
        let (at, which) = (server.clone(), share.clone());
        self.share_send(server.clone(), Some(share.clone()), typed, keep, to, move |to| mount_on(&at, &which, to),
                        move |tried, got| ShareMsg::Mounted { t, p, server, share, rest, tried, got });
    }

    pub(super) fn share_arrived(&mut self, t: usize, p: usize, a: &Address, to: PathBuf) {
        if t != self.cols[p].tab {
            return;
        }
        let here = Where::Remote(a.to_string());
        if self.cols[p].tabs[t].at == here {
            self.arrive(p, Where::Folder(to));
        } else {
            self.go(p, Where::Folder(to));
        }
    }

    fn share_card_for(&self, server: &str) -> bool {
        matches!(&self.dialog, Some(Dialog::Share(a)) if a.server.eq_ignore_ascii_case(server))
    }

    pub(super) fn share_card_done(&mut self, server: &str) {
        if self.share_card_for(server) {
            self.dialog = None;
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn share_ask(&mut self, t: usize, p: usize, server: &str, share: Option<&str>, rest: &str, problem: &Problem, tried: Tried) {
        let failed = !matches!(problem, Problem::NeedsLogin);
        if let Some(Dialog::Share(a)) = &mut self.dialog {
            if !(a.server.eq_ignore_ascii_case(server) && a.share.as_deref() == share && a.address == tried.address) {
                let it = a.server.clone();
                return self.say(t!("folders-shares-answer-card", it = &it), false);
            }
            if !a.busy && !failed {
                return;
            }
            if failed {
                a.why = problem.words(server, share);
            }
            if tried.keyring.is_some() {
                a.keyring = tried.keyring;
            }
            a.failed = failed;
            a.busy = false;
            a.password = Zeroizing::new(String::new());
            return;
        }
        let nowhere = tried.address.is_none();
        let held = tried.held.is_some() || nowhere;
        let why = match (&tried.address, &tried.held) {
            (None, _) => nowhere_words(server),
            (Some(at), Some(_)) if tried.lapsed => lapsed_words(server, at),
            (Some(at), Some(_)) if tried.unknown => unknown_words(server, at),
            (Some(at), Some(_)) if tried.unshown => unshown_words(server, at),
            (at, Some(before)) => held_words(server, at.as_deref(), before),
            (_, None) if failed => problem.words(server, share),
            (Some(at), None) if tried.again => again_words(server, at),
            (at, None) => asking_words(server, at.as_deref().filter(|_| tried.heard)),
        };
        let waiting = tried.waiting.filter(|_| !nowhere);
        let filled = waiting.as_ref().filter(|_| tried.again).map(|l| (mark(l), tried.kept));
        let user = waiting.as_ref().map(Login::shown_user).or_else(|| self.held_for(server)
            .filter(|u| !held && tried.address.as_deref().is_some_and(|a| same_address(a, &u.at))).map(|u| u.login.shown_user()));
        let remember = match &filled {
            Some((_, kept)) => *kept,
            None => waiting.is_none(),
        };
        let password = waiting.map_or_else(|| Zeroizing::new(String::new()), |l| l.password);
        self.dialog = Some(Dialog::Share(Ask {
            server: server.to_string(), share: share.map(str::to_string), rest: rest.to_string(),
            user: user.unwrap_or_default(), password, remember,
            why, keyring: tried.keyring, failed, busy: false, tab: t, pane: p, address: tried.address, held, heard: tried.heard,
            before: tried.held.unwrap_or_default(), forget: false, filled,
        }));
    }

    pub(in crate::files) fn share_places(&self) -> &[Mounted] {
        &self.shares.mounted
    }

    pub(in crate::files) fn shares_login(&mut self, server: &str, login: Login) {
        self.shares.typed.insert(server.to_lowercase(), login);
    }

    pub(super) fn share_kept_said(&mut self, tried: &Tried) -> bool {
        match (&tried.keyring, &tried.late) {
            (_, Some(heard)) => {
                let said = keyring_said::waiting("login", false);
                self.say(t!("folders-shares-open-waiting", words = &said.words), said.failed);
                let heard = heard.clone();
                self.share_off(move || heard.wait(), ShareMsg::KeptLate);
                true
            }
            (Some(why), None) => {
                self.say_help(unkept_words(why), crate::files::help::SHARE_LOGIN);
                true
            }
            (None, None) => false,
        }
    }

    pub(super) fn share_used(&mut self, server: &str, login: Option<Login>, tried: &Tried) {
        if let (Some(login), Some(at)) = (login, &tried.address) {
            self.shares.logins.insert(server.to_lowercase(), Used { login, at: at.clone(), confirmed: tried.confirmed });
        }
    }
}

pub(super) fn typed_login(a: &Address) -> Option<Login> {
    let user = a.user.as_ref()?;
    let (domain, user) = user.split_once(';').unwrap_or(("", user));
    let password = a.password.clone()?;
    let l = Login { user: user.to_string(), domain: domain.to_string(), password: Zeroizing::new(password) };
    l.check().is_ok().then_some(l)
}
