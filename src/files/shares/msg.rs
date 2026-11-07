// What the window does with each message about shares.

use std::path::PathBuf;

use zeroize::Zeroizing;

use crate::files::app::Files;
use crate::files::help;
use crate::files::places::{self, Entry, Where};
use crate::files::dialogs::Dialog;
use crate::files::remote::{Address, Scheme};
use crate::common::keyring_said;
use crate::common::smb::{Login, Mounted, Problem};
use crate::toolkit::t;
use super::types::Confirmed;
use super::commands::nfs_words;
use super::rule::mark;
use super::types::{Reach, ShareMsg, Tried, QUIET};
use super::system::unmount_on;

impl Files {
    pub(in crate::files) fn share_msg(&mut self, m: ShareMsg) {
        match m {
            ShareMsg::KeptRows(got) => self.own_logins(got),
            ShareMsg::Forgot(server, got) => self.own_forgot(&server, got),
            ShareMsg::Listed { t, p, asked, tried, got } => self.shares_listed(t, p, asked, tried, got),
            ShareMsg::NfsMounted { t, p, asked, server, export, rest, got } => self.nfs_mounted(t, p, (asked, server, export, rest), got),
            ShareMsg::Mounted { t, p, server, share, rest, tried, got } => self.share_mounted(t, p, (server, share, rest), tried, got),
            ShareMsg::User(_) | ShareMsg::Password(_) | ShareMsg::Remember(_) | ShareMsg::Forget(_) | ShareMsg::Close => self.share_card_edit(m),
            ShareMsg::Go => self.share_go(),
            ShareMsg::KeptLate(how) => {
                let said = keyring_said::late("login", false, &how);
                self.say(said.words, said.failed);
            }
            ShareMsg::Reconnect(i) => {
                if let Some(r) = self.shares.remembered.get(i).cloned() {
                    self.reconnect(r, String::new(), false);
                }
            }
            ShareMsg::ForgetShare(i) => {
                let Some(r) = self.shares.remembered.get(i) else { return };
                if self.share_up(r) || self.pinned.iter().any(|p| r.holds(p)) {
                    return;
                }
                self.shares.remembered.remove(i);
                self.save_shares();
            }
            ShareMsg::ReconnectAll => {
                self.shares.refresh();
                self.shares.waiting = self.down_shares().into_iter().map(|i| (self.shares.remembered[i].server.clone(), self.shares.remembered[i].share.clone())).collect();
                self.reconnect_next(QUIET);
            }
            ShareMsg::Show(i) => {
                if let Some(m) = self.shares.mounted.get(i) {
                    let at = m.at.clone();
                    self.go(self.side(), Where::Folder(at));
                }
            }
            ShareMsg::Eject(i) => self.share_eject(i),
            ShareMsg::Ejected(at, got) => self.share_ejected(at, got),
        }
    }

    fn shares_listed(&mut self, t: usize, p: usize, asked: String, tried: Tried, got: Result<(Vec<String>, Option<Login>), Problem>) {
        self.far.waiting.retain(|w| *w != asked);
        let here = Where::Remote(asked.clone());
        let Ok(a) = Address::parse(&asked) else { return };
        let server = a.host.clone();
        let still = self.cols[p].tabs.get(t).is_some_and(|pane| pane.at == here);
        match got {
            Ok((names, login)) => {
                self.share_used(&server, login, &tried);
                self.share_card_done(&server);
                let said = self.share_kept_said(&tried);
                if !still {
                    return;
                }
                if !said && names.is_empty() {
                    self.say(t!("folders-shares-none", server = &server), false);
                }
                let mut entries: Vec<Entry> = names.into_iter().map(|n| Entry {
                    path: PathBuf::from(Address { path: format!("/{n}"), user: None, password: None, ..a.clone() }.to_string()),
                    name: n, folder: true, size: 0, when: 0, kind: None,
                }).collect();
                places::arrange(&mut entries, &here, self.hidden, self.sort, self.up);
                self.fill(t, p, entries);
            }
            Err(problem) => {
                if still {
                    self.fill(t, p, Vec::new());
                }
                if problem.ask() {
                    self.share_ask(t, p, &server, None, "", &problem, tried);
                } else {
                    self.share_card_done(&server);
                    self.say_help(problem.words(&server, None), help::share_page(&problem, false));
                }
            }
        }
    }

    fn nfs_mounted(&mut self, t: usize, p: usize, named: (String, String, String, String), got: Result<PathBuf, Problem>) {
        let (asked, server, export, rest) = named;
        match got {
            Ok(at) => {
                self.shares.refresh();
                if self.shares.open_nfs(&server, &export).is_none() {
                    self.shares.mounted.push(Mounted { server: server.clone(), share: export.clone(), at: at.clone() });
                }
                self.remember_mounted(&server, &export, &at);
                self.say(String::new(), false);
                if let Ok(a) = Address::parse(&asked) {
                    self.share_arrived(t, p, &a, at.join(&rest));
                }
                self.reconnect_next(t);
            }
            Err(problem) => {
                self.say_help(nfs_words(&problem, &server, &export), help::share_page(&problem, true));
                self.reconnect_next(t);
            }
        }
    }

    fn share_mounted(&mut self, t: usize, p: usize, named: (String, String, String), tried: Tried, got: Result<(PathBuf, Option<Login>), Problem>) {
        let (server, share, rest) = named;
        match got {
            Ok((at, login)) => {
                self.share_used(&server, login, &tried);
                self.share_card_done(&server);
                self.shares.refresh();
                if self.shares.open_at(&server, &share).is_none() {
                    self.shares.mounted.push(Mounted { server: server.clone(), share: share.clone(), at: at.clone() });
                }
                self.remember_mounted(&server, &share, &at);
                self.say(String::new(), false);
                let a = Address { scheme: Scheme::Smb, user: None, password: None, host: server, port: None,
                                  path: if rest.is_empty() { format!("/{share}") } else { format!("/{share}/{rest}") } };
                self.share_arrived(t, p, &a, at.join(&rest));
                self.share_kept_said(&tried);
                self.reconnect_next(t);
            }
            Err(problem) if problem.ask() && t == QUIET => {
                self.say_help(problem.words(&server, Some(&share)), help::share_page(&problem, false));
                self.reconnect_next(t);
            }
            Err(problem) if problem.ask() => {
                self.say(String::new(), false);
                self.share_ask(t, p, &server, Some(&share), &rest, &problem, tried);
            }
            Err(problem) => {
                self.share_card_done(&server);
                self.say_help(problem.words(&server, Some(&share)), help::share_page(&problem, false));
                self.reconnect_next(t);
            }
        }
    }

    fn share_card_edit(&mut self, m: ShareMsg) {
        match m {
            ShareMsg::User(text) => {
                if let Some(Dialog::Share(a)) = &mut self.dialog {
                    a.user = text;
                }
            }
            ShareMsg::Password(text) => {
                if let Some(Dialog::Share(a)) = &mut self.dialog {
                    a.password = Zeroizing::new(text);
                }
            }
            ShareMsg::Remember(on) => {
                if let Some(Dialog::Share(a)) = &mut self.dialog {
                    a.remember = on;
                }
            }
            ShareMsg::Forget(on) => {
                if let Some(Dialog::Share(a)) = &mut self.dialog {
                    a.forget = on;
                }
            }
            ShareMsg::Close => {
                if matches!(self.dialog, Some(Dialog::Share(_))) {
                    self.dialog = None;
                }
            }
            _ => {}
        }
    }

    fn share_go(&mut self) {
        let Some(Dialog::Share(a)) = &mut self.dialog else { return };
        if a.busy || a.user.trim().is_empty() {
            return;
        }
        let Some(at) = Confirmed::pressed(a) else { return };
        let login = Login::typed(&a.user, &a.password);
        if let Err(why) = login.check() {
            a.why = why;
            a.failed = true;
            return;
        }
        a.busy = true;
        let (t, p, server, share, rest) = (a.tab, a.pane, a.server.clone(), a.share.clone(), a.rest.clone());
        let as_filled = a.filled.as_ref().is_some_and(|(whose, kept)| *whose == mark(&login) && *kept == a.remember);
        let keep = if as_filled { None } else { a.remember.then_some(a.forget && a.held) };
        let to = Reach::Pinned(at, a.heard);
        match share {
            Some(share) => self.smb_mount(t, p, server, share, rest, Some(login), keep, to),
            None => {
                let asked = Address { scheme: Scheme::Smb, user: None, password: None, host: server.clone(), port: None, path: "/".into() }.to_string();
                let asked = self.cols[p].tabs.get(t).and_then(|pane| match &pane.at {
                    Where::Remote(at) if Address::parse(at).is_ok_and(|x| x.scheme == Scheme::Smb && x.host == server) => Some(at.clone()),
                    _ => None,
                }).unwrap_or(asked);
                self.smb_list(t, p, server, asked, Some(login), keep, to);
            }
        }
    }

    fn share_eject(&mut self, i: usize) {
        let Some(m) = self.shares.mounted.get(i).cloned() else { return };
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                if self.cols[p].tabs[t].at.folder().is_some_and(|f| f.starts_with(&m.at)) {
                    if t == self.cols[p].tab {
                        self.go(p, Where::Network);
                    } else {
                        self.cols[p].tabs[t].at = Where::Network;
                    }
                }
            }
        }
        self.say(t!("folders-shares-closing", name = &m.name()), false);
        let at = m.at.clone();
        self.share_off(move || unmount_on(&at), move |got| ShareMsg::Ejected(m.at, got));
    }

    fn share_ejected(&mut self, at: PathBuf, got: Result<(), String>) {
        let name = self.shares.holding(&at).map(Mounted::name).unwrap_or_default();
        match got {
            Ok(()) => {
                self.shares.mounted.retain(|m| m.at != at);
                self.shares.refresh();
                self.forget_unpinned_shares();
                self.say(String::new(), false);
            }
            Err(why) => self.say_help(t!("folders-shares-close-failed", name = &name, why = why.trim_end_matches('.')), help::SHARE_NOT_OPENED),
        }
        self.reload_all();
    }
}
