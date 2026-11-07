// What the window does with each message about a server or phone.

use std::path::PathBuf;

use crate::toolkit::t;

use crate::files::act::Ask;
use crate::files::app::Files;
use crate::files::{askpass, help};
use crate::files::inplace::Renaming;
use crate::files::dialogs::Dialog;
use crate::files::places::{self, Entry, Where};
use crate::files::leave::Confirmed;
use crate::files::remote::{self, Address, Login, Scheme};
use crate::files::work;
use super::card::{Question, Why, far, tilde, typed, why};
use super::root;
use super::window::{Copied, FarMsg, Listed, Server, Then, changed_time};

impl Files {
    pub(in crate::files) fn far_msg(&mut self, m: FarMsg) {
        match m {
            FarMsg::Listed { t, p, asked, got } => self.far_listed(t, p, asked, got),
            FarMsg::Open => {
                self.far.look_for_phones();
                self.connect_card(String::new(), None, self.side());
            }
            FarMsg::Address(_) | FarMsg::Password(_) | FarMsg::Key(_) | FarMsg::PickKey | FarMsg::Trust | FarMsg::Close => self.connect_edit(m),
            FarMsg::Go => self.connect_go(),
            FarMsg::Connected { p, root, name, at } => {
                let at = Where::Remote(at);
                if !self.far.servers.iter().any(|s| s.root == root) {
                    self.far.servers.push(Server { root, name, at: at.clone() });
                }
                if matches!(self.dialog, Some(Dialog::Connect(_))) {
                    self.dialog = None;
                }
                self.go(p, at);
            }
            FarMsg::Refused(w) => {
                let Some(Dialog::Connect(c)) = &mut self.dialog else {
                    if let Why::Words(w, page) = w {
                        self.say_help(w, page);
                    }
                    return;
                };
                c.busy = false;
                match w {
                    Why::Ask(q) => c.question = Some(q),
                    Why::Words(w, page) => c.bad = Some((w, page)),
                }
            }
            FarMsg::Plugged => self.far.look_for_phones(),
            FarMsg::Fetched { far, then: Then::Open, got: Ok(path) } => self.far_opened_copy(far, path),
            FarMsg::Fetched { then: Then::View(id), got, .. } => self.preview_fetched(id, got),
            FarMsg::Fetched { far, got: Err(w), .. } => self.say_help(w, help::server_page(&far)),
            FarMsg::SentBack(local, got) => self.far_sent_back(local, got),
            FarMsg::Renamed { was, now, next, got } => self.far_renamed(*was, now, next, got),
            FarMsg::Made { side, path, got } => {
                match got {
                    Err(w) => self.say_help(w, help::server_page(&path)),
                    Ok(()) => self.listed = Some(Listed { side, choose: None, rename: Some((path, true)) }),
                }
                self.far_relist();
            }
        }
    }

    fn far_listed(&mut self, t: usize, p: usize, asked: String, got: Result<remote::Listing, Why>) {
        self.far.waiting.retain(|w| *w != asked);
        let here = Where::Remote(asked.clone());
        if self.cols[p].tabs.get(t).is_none_or(|pane| pane.at != here) {
            return;
        }
        match got {
            Ok(list) => {
                let Ok(a) = Address::parse(&asked) else { return };
                match list.refused.as_slice() {
                    [] => {}
                    many => self.say_help(t!("folders-far-unsafe-names", n = many.len(), names = &many.join(", ")), help::NAME_REFUSED),
                }
                let mut entries: Vec<Entry> = list.entries.into_iter().map(|e| {
                    let folder = e.is_folder();
                    let kind = if folder { None } else { self.mime.of(&e.name) };
                    let path = PathBuf::from(a.join(&e.name).to_string());
                    Entry { path, folder, size: if folder { 0 } else { e.size }, when: e.modified.unwrap_or(0), kind, name: e.name }
                }).collect();
                places::arrange(&mut entries, &here, self.hidden, self.sort, self.up);
                self.fill(t, p, entries);
                if self.cols[p].tab == t {
                    if let Some(l) = self.listed.take_if(|l| l.side == p) {
                        if let Some(path) = l.choose {
                            self.choose_path(p, &path);
                        }
                        if let Some((path, fresh)) = l.rename {
                            self.rename_path(p, &path, fresh);
                        }
                    }
                }
            }
            Err(Why::Words(w, page)) => {
                self.fill(t, p, Vec::new());
                self.say_help(w, page);
            }
            Err(Why::Ask(q)) => {
                self.fill(t, p, Vec::new());
                let top = typed(&asked).map(|a| Address { path: String::new(), ..a }.to_string()).unwrap_or(asked);
                self.connect_card(top, Some(q), p);
            }
        }
    }

    fn connect_edit(&mut self, m: FarMsg) {
        match m {
            FarMsg::Address(t) => {
                if let Some(Dialog::Connect(c)) = &mut self.dialog {
                    c.address = t;
                    c.bad = None;
                    c.question = None;
                    c.trust = false;
                    c.trust_cert = None;
                }
            }
            FarMsg::Password(t) => {
                if let Some(Dialog::Connect(c)) = &mut self.dialog {
                    c.password = t;
                }
            }
            FarMsg::Key(t) => {
                if let Some(Dialog::Connect(c)) = &mut self.dialog {
                    c.key = t;
                }
            }
            FarMsg::PickKey => {
                let ask = Ask {
                    title: t!("folders-far-connect-pick-key").into(),
                    folder: Some(crate::common::files::home()),
                    directory: false,
                };
                self.pick(ask, |paths| paths.into_iter().next().map(|p| far(FarMsg::Key(p.to_string_lossy().into_owned()))));
            }
            FarMsg::Trust => {
                if let Some(Dialog::Connect(c)) = &mut self.dialog {
                    match &c.question {
                        Some(Question::NewCert { fingerprint, .. }) | Some(Question::CertChanged { fingerprint, .. }) => {
                            c.trust_cert = Some(fingerprint.clone());
                        }
                        _ => c.trust = true,
                    }
                }
                self.far_msg(FarMsg::Go);
            }
            FarMsg::Close => {
                if matches!(self.dialog, Some(Dialog::Connect(_))) {
                    self.dialog = None;
                }
            }
            _ => {}
        }
    }

    fn connect_go(&mut self) {
        let Some(Dialog::Connect(c)) = &mut self.dialog else { return };
        if c.busy {
            return;
        }
        let a = match typed(&c.address) {
            Ok(a) => a,
            Err(e) => {
                c.bad = Some((e.to_string(), help::remote_page(&e, false)));
                return;
            }
        };
        if a.scheme == Scheme::Mtp {
            c.bad = Some((t!("folders-far-phones").into(), help::PHONE));
            return;
        }
        if a.scheme == Scheme::Nfs {
            let p = c.pane;
            self.dialog = None;
            return self.go(p, Where::Remote(Address { user: None, password: None, ..a }.to_string()));
        }
        if a.scheme == Scheme::Smb {
            return self.connect_share(a);
        }
        let password = if c.password.is_empty() { a.password.clone() } else { Some(c.password.clone()) };
        let key = (!c.key.trim().is_empty()).then(|| tilde(c.key.trim()));
        let askpass = if a.scheme == Scheme::Sftp { askpass::program() } else { None };
        let login = Login { password, key, askpass, trust_new_host: c.trust, trust_cert: c.trust_cert.clone() };
        let shown = Confirmed::pressed(c);
        c.busy = true;
        c.bad = None;
        let p = c.pane;
        let links = self.far.links.clone();
        let phone = a.scheme == Scheme::Mtp;
        self.far_off(move || match links.open(&a, &login, shown.as_ref()) {
            Ok(home) => FarMsg::Connected { p, root: root(&a), name: a.host.clone(), at: Address { path: home, password: None, ..a }.to_string() },
            Err(e) => FarMsg::Refused(why(e, phone)),
        }, |m| m);
    }

    fn connect_share(&mut self, a: Address) {
        let Some(Dialog::Connect(c)) = &mut self.dialog else { return };
        let p = c.pane;
        let user = a.user.clone().unwrap_or_default();
        let password = if c.password.is_empty() { a.password.clone().unwrap_or_default() } else { c.password.clone() };
        if !user.is_empty() && !password.is_empty() {
            let login = crate::common::smb::Login::typed(&user.replacen(';', "\\", 1), &password);
            if login.check().is_ok() {
                self.shares_login(&a.host, login);
            }
        }
        self.dialog = None;
        self.go(p, Where::Remote(Address { password: None, ..a }.to_string()));
    }

    fn far_opened_copy(&mut self, far: PathBuf, path: PathBuf) {
        let name = work::name_of(&path);
        if self.open_file(&path) {
            if let Some(dir) = path.parent() {
                self.watch_also(dir);
            }
            let seen = changed_time(&path);
            self.far.fetched.insert(path, Copied { far, seen, sending: false, again: false });
            self.say(t!("folders-far-opened-copy", name = &name), false);
        }
    }

    fn far_sent_back(&mut self, local: PathBuf, got: Result<(), String>) {
        let name = work::name_of(&local);
        match got {
            Ok(()) => self.say(t!("folders-far-saved", name = &name), false),
            Err(w) => {
                let page = self.far.fetched.get(&local).map_or(help::SERVER_REFUSED, |c| help::server_page(&c.far));
                self.say_help(t!("folders-far-not-saved", name = &name, why = w.trim_end_matches('.')), page)
            }
        }
        let again = match self.far.fetched.get_mut(&local) {
            Some(c) => {
                c.sending = false;
                std::mem::take(&mut c.again)
            }
            None => false,
        };
        if again {
            self.far_saved(&[local]);
        }
    }

    fn far_renamed(&mut self, was: Renaming, now: Option<PathBuf>, next: Option<PathBuf>, got: Result<(), String>) {
        let side = was.side;
        match got {
            Err(w) => {
                let page = help::server_page(&was.path);
                return self.rename_refused(was, w, page);
            }
            Ok(()) => {
                self.say(t!("folders-far-renamed", was = was.was(), now = was.name()), false);
                self.listed = Some(Listed { side, choose: now, rename: next.map(|n| (n, false)) });
            }
        }
        self.far_relist();
    }
}
