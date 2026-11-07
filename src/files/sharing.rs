// Sharing one of the person's own folders on the home network.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use super::app::{Files, Msg};
use super::dialogs::Dialog;
use super::tools::keyed;
use super::icons::stand_in;
use crate::common::sharing::{self, Problem, Shared};
use crate::toolkit::t;
use crate::toolkit::widget::{button, column, entry, form_label, label, note, row, Length, Node, Tone};

const FRESH: Duration = Duration::from_secs(10);
const PASSWORD_LEAST: usize = 8;

#[derive(Default)]
pub struct Sharing {
    pub shared: Vec<Shared>,
    pub problem: Option<Problem>,
    asked: Option<Instant>,
}

impl Sharing {
    pub fn of(&self, folder: &Path) -> Option<&Shared> {
        self.shared.iter().find(|s| s.path == folder)
    }

    fn needs_setting_up(&self) -> bool {
        matches!(self.problem, Some(Problem::NoSamba | Problem::NotSetUp))
    }
}

pub struct Form {
    pub folder: PathBuf,
    pub on: bool,
    pub name: String,
    pub was: Option<String>,
    pub write: bool,
    pub guests: bool,
    pub busy: Option<String>,
    pub problem: Option<String>,
    pub cannot: Option<String>,
    pub password: Zeroizing<String>,
    pub changing: bool,
}

#[derive(Clone, Debug)]
pub enum SharingMsg {
    On(bool),
    Name(String),
    NameDone,
    Write(bool),
    Guests(bool),
    Password(String),
    SavePassword,
    ChangePassword,
    LetConnect,
    LetGuestsIn,
    Listed(Result<Vec<Shared>, Problem>),
    SetUp(PathBuf, Result<(), String>),
    Made(PathBuf, Result<(), String>, Result<Vec<Shared>, Problem>),
    PasswordSaved(Result<(), String>),
    Connecting(Result<(), String>),
}

fn sharing_msg(m: SharingMsg) -> Msg {
    Msg::Sharing(m)
}

fn cannot_share(folder: &Path) -> Option<String> {
    use std::os::unix::fs::MetadataExt;
    let Ok(m) = std::fs::symlink_metadata(folder) else { return Some(t!("folders-sharing-cannot-read").into()) };
    if !m.is_dir() {
        return Some(t!("folders-sharing-only-folder").into());
    }
    if m.uid() != unsafe { libc::getuid() } {
        return Some(t!("folders-sharing-only-yours").into());
    }
    None
}

impl Form {
    pub fn new(folder: &Path, s: &Sharing) -> Form {
        let now = s.of(folder);
        Form {
            folder: folder.to_path_buf(),
            on: now.is_some(),
            name: now.map_or_else(|| sharing::name_for(folder), |n| n.name.clone()),
            was: now.map(|n| n.name.clone()),
            write: now.is_some_and(|n| n.write),
            guests: now.is_some_and(|n| n.guests),
            busy: None,
            problem: None,
            cannot: cannot_share(folder),
            password: Zeroizing::new(String::new()),
            changing: false,
        }
    }

    fn follow(&mut self, s: &Sharing) {
        let now = s.of(&self.folder);
        self.on = now.is_some();
        self.was = now.map(|n| n.name.clone());
        if let Some(n) = now {
            self.name = n.name.clone();
            self.write = n.write;
            self.guests = n.guests;
        }
    }

    fn wanted(&self) -> Shared {
        Shared { name: self.name.trim().to_string(), path: self.folder.clone(), write: self.write, guests: self.guests }
    }
}

fn toggle(name: &str, sub: &str, on: bool, tip: &str, (job, off): (bool, bool), change: fn(bool) -> SharingMsg) -> Node<Msg> {
    super::dialogs::toggle(name, sub, on, tip, move |v| sharing_msg(change(v))).off(off).off_because(crate::common::ui::wait_while(job))
}

fn with_button(words: String, icon: &str, tip: &str, press: SharingMsg) -> Node<Msg> {
    row(vec![note(words).width(Length::Fill), button(icon.to_string(), tip.to_string()).on_press(sharing_msg(press))]).gap(8.0).width(Length::Fill)
}

fn closed_words(kept: &[PathBuf], folder: &Path) -> String {
    let home = crate::common::files::home();
    let mut names = kept.iter().map(|p| {
        if *p == home { t!("folders-sharing-your-home").to_string() } else if p == folder { t!("folders-sharing-this-folder").to_string() } else { p.display().to_string() }
    });
    let first = names.next().unwrap_or_default();
    names.fold(first, |all, next| t!("folders-sharing-and", first = &all, second = &next))
}

pub fn view(f: &Form) -> Vec<Node<Msg>> {
    let mut out = Vec::new();
    let shared = f.on && f.was.is_some();
    let busy = f.busy.is_some();
    out.push(toggle(t!("folders-sharing-share"), if shared { t!("folders-sharing-shared") } else { t!("folders-sharing-not-shared") }, f.on,
                    t!("folders-sharing-share-tip"), (busy, f.cannot.is_some() && !f.on), SharingMsg::On));
    out.push(column(vec![
        form_label(t!("folders-sharing-name")),
        entry(f.name.clone(), t!("folders-sharing-name-tip"), |t| sharing_msg(SharingMsg::Name(t))).key("share-name")
            .on_submit(sharing_msg(SharingMsg::NameDone)).on_escape(Msg::Ask(super::dialogs::AskMsg::Close)).width(Length::Fill),
    ]).gap(8.0).width(Length::Fill));
    out.push(toggle(t!("folders-sharing-write"), t!("folders-sharing-write-sub"), f.write,
                    t!("folders-sharing-write"), (busy, false), SharingMsg::Write));
    out.push(toggle(t!("folders-sharing-guests"), t!("folders-sharing-guests-sub"), f.guests,
                    t!("folders-sharing-guests-tip"), (busy, false), SharingMsg::Guests));
    if let Some(b) = &f.busy {
        out.push(note(b.clone()).tone(Tone::Calm));
    }
    if let Some(p) = &f.problem {
        out.push(row(vec![label(p.clone()).tone(Tone::Problem).width(Length::Fill), crate::common::more::button(Msg::Help(super::help::SHARING_FOLDER))]).gap(8.0).width(Length::Fill));
    }
    if let (Some(c), false) = (&f.cannot, f.on) {
        out.push(row(vec![note(c.clone()).width(Length::Fill), crate::common::more::button(Msg::Help(super::help::SHARING_FOLDER))]).gap(8.0).width(Length::Fill));
        return out;
    }
    if !shared {
        if !busy {
            out.push(note(t!("folders-sharing-explain", fetch = if sharing::samba_here() { "no" } else { "yes" })));
        }
        return out;
    }
    shared_lines(f, &mut out);
    out
}

fn shared_lines(f: &Form, out: &mut Vec<Node<Msg>>) {
    let busy = f.busy.is_some();
    let host = sharing::host_name();
    let name = f.was.clone().unwrap_or_default();
    out.push(note(t!("folders-sharing-find-it", host_known = if host.is_empty() { "no" } else { "yes" }, name = &name, host = &host)));
    if !sharing::connect_allowed() {
        out.push(with_button(t!("folders-sharing-cannot-reach").into(),
                             stand_in::LET_CONNECT, t!("folders-sharing-let-connect"), SharingMsg::LetConnect));
    }
    if f.guests {
        let kept = sharing::kept_out(&f.folder, f.write);
        if !kept.is_empty() {
            out.push(with_button(t!("folders-sharing-visitors", n = kept.len(), names = &closed_words(&kept, &f.folder)),
                                 stand_in::LET_THROUGH, t!("folders-sharing-let-through"), SharingMsg::LetGuestsIn));
        }
        return;
    }
    let user = crate::common::files::user();
    if sharing::password_set() && !f.changing {
        out.push(with_button(t!("folders-sharing-sign-in-set", user = &user),
                             "dialog-password-symbolic", t!("folders-sharing-change-password"), SharingMsg::ChangePassword));
        return;
    }
    out.push(note(t!("folders-sharing-set-password", user = &user, n = PASSWORD_LEAST)));
    out.push(keyed(t!("folders-sharing-password"), vec![
        entry(f.password.to_string(), t!("folders-sharing-password-tip"), |t| sharing_msg(SharingMsg::Password(t)))
            .secret().key("share-password").on_submit(sharing_msg(SharingMsg::SavePassword)).width(Length::Fill),
        button(crate::common::ui::SAVE_ICON, t!("folders-sharing-keep-password")).on_press(sharing_msg(SharingMsg::SavePassword))
            .off(f.password.chars().count() < PASSWORD_LEAST).off_because(crate::common::ui::wait_while(busy)),
    ]));
}

pub fn card_view(f: &Form) -> Node<Msg> {
    let mut lines = vec![super::dialogs::head(stand_in::SHARE_FOLDER, t!("folders-sharing-card-title", name = &super::work::name_of(&f.folder)))];
    lines.extend(view(f));
    column(lines)
}

impl Files {
    fn sharing_off(&mut self, job: impl FnOnce() -> SharingMsg + Send + 'static) {
        match &self.post {
            Some(post) => {
                let post = post.clone();
                let _ = std::thread::Builder::new().name("folders-sharing".into()).spawn(move || post.send(Msg::Sharing(job())));
            }
            None => {
                let m = job();
                self.sharing_msg(m);
            }
        }
    }

    pub(super) fn sharing_refresh(&mut self, now: bool) {
        if !now && self.sharing.asked.is_some_and(|at| at.elapsed() < FRESH) {
            return;
        }
        self.sharing.asked = Some(Instant::now());
        self.sharing_off(|| SharingMsg::Listed(sharing::list()));
    }

    pub(super) fn is_shared(&self, path: &Path) -> bool {
        self.sharing.of(path).is_some()
    }

    fn form(&mut self) -> Option<&mut Form> {
        match &mut self.dialog {
            Some(Dialog::Properties(p)) => p.share.as_mut(),
            Some(Dialog::Sharing(f)) => Some(f),
            _ => None,
        }
    }

    pub(super) fn share_form(&mut self, folder: &Path) -> Option<Form> {
        let meta = std::fs::symlink_metadata(folder).ok()?;
        let trash = crate::common::files::data_home().join("Trash");
        if !meta.is_dir() || self.shares.holding(folder).is_some() || folder.starts_with(trash) || super::far::is_far(folder) {
            return None;
        }
        self.sharing_refresh(true);
        let mut form = Form::new(folder, &self.sharing);
        if !self.parts.helper && form.cannot.is_none() && !form.on {
            form.cannot = Some(super::alone::no_helper_share().into());
        }
        Some(form)
    }

    pub(super) fn sharing_msg(&mut self, m: SharingMsg) {
        match m {
            SharingMsg::Listed(got) => self.sharing_listed(got),
            SharingMsg::SetUp(folder, got) => {
                if got.is_ok() {
                    self.sharing.problem = None;
                }
                let Some(f) = self.form().filter(|f| f.folder == folder) else { return };
                f.busy = None;
                match got {
                    Ok(()) => self.share_now(),
                    Err(e) => {
                        f.on = false;
                        f.problem = Some(e);
                    }
                }
            }
            SharingMsg::Made(folder, got, list) => {
                self.sharing_listed(list);
                let s = std::mem::take(&mut self.sharing);
                if let Some(f) = self.form().filter(|f| f.folder == folder) {
                    f.busy = None;
                    let typed = f.name.clone();
                    f.follow(&s);
                    if let Err(e) = got {
                        f.problem = Some(e);
                        f.name = typed;
                    }
                }
                self.sharing = s;
            }
            SharingMsg::PasswordSaved(got) => {
                let Some(f) = self.form() else { return };
                f.busy = None;
                match got {
                    Ok(()) => {
                        f.password = Zeroizing::new(String::new());
                        f.changing = false;
                    }
                    Err(e) => f.problem = Some(e),
                }
            }
            SharingMsg::Connecting(got) => {
                let Some(f) = self.form() else { return };
                f.busy = None;
                if let Err(e) = got {
                    f.problem = Some(e);
                }
            }
            m => self.sharing_asked(m),
        }
    }

    fn sharing_asked(&mut self, m: SharingMsg) {
        let setting_up = self.sharing.needs_setting_up() || !sharing::samba_here();
        let others = self.sharing.shared.clone();
        let Some(f) = self.form() else { return };
        let name_taken = others.iter().any(|x| x.name.eq_ignore_ascii_case(f.name.trim()) && x.path != f.folder);
        if !matches!(m, SharingMsg::Name(_) | SharingMsg::Password(_)) {
            f.problem = None;
        }
        let shared = f.on && f.was.is_some();
        match m {
            SharingMsg::On(true) => self.share_switched_on(setting_up, name_taken),
            SharingMsg::On(false) => self.share_switched_off(),
            SharingMsg::Name(t) => f.name = t,
            SharingMsg::NameDone => self.share_renamed(shared, name_taken),
            SharingMsg::Write(on) | SharingMsg::Guests(on) => {
                if matches!(m, SharingMsg::Write(_)) { f.write = on } else { f.guests = on }
                if shared {
                    self.share_now();
                }
            }
            SharingMsg::Password(t) => f.password = Zeroizing::new(t),
            SharingMsg::ChangePassword => f.changing = !f.changing,
            SharingMsg::SavePassword => self.share_password_saved(),
            SharingMsg::LetConnect => {
                f.busy = Some(t!("folders-sharing-letting-connect").into());
                self.sharing_off(|| SharingMsg::Connecting(sharing::let_connect()));
            }
            SharingMsg::LetGuestsIn => {
                if let Err(e) = sharing::let_guests_in(&f.folder, f.write) {
                    f.problem = Some(e);
                }
            }
            _ => {}
        }
    }

    fn share_switched_on(&mut self, setting_up: bool, name_taken: bool) {
        let Some(f) = self.form() else { return };
        if let Some(c) = &f.cannot {
            f.problem = Some(c.clone());
            return;
        }
        if let Err(e) = sharing::valid_name(&f.name) {
            f.problem = Some(e);
            return;
        }
        if name_taken {
            f.problem = Some(t!("folders-sharing-name-taken").into());
            return;
        }
        f.on = true;
        if setting_up { self.sharing_setup() } else { self.share_now() }
    }

    fn share_switched_off(&mut self) {
        let Some(f) = self.form() else { return };
        f.on = false;
        if let Some(name) = f.was.clone() {
            f.busy = Some(t!("folders-sharing-stopping").into());
            let folder = f.folder.clone();
            self.sharing_off(move || {
                let got = sharing::remove(&name);
                SharingMsg::Made(folder, got, sharing::list())
            });
        }
    }

    fn share_renamed(&mut self, shared: bool, name_taken: bool) {
        let Some(f) = self.form() else { return };
        let changed = f.was.as_deref().is_some_and(|w| !w.eq_ignore_ascii_case(f.name.trim()));
        if !shared || !changed {
            return;
        }
        if let Err(e) = sharing::valid_name(&f.name) {
            f.problem = Some(e);
        } else if name_taken {
            f.problem = Some(t!("folders-sharing-name-taken").into());
        } else {
            self.share_now();
        }
    }

    fn share_password_saved(&mut self) {
        let Some(f) = self.form() else { return };
        if f.password.chars().count() < PASSWORD_LEAST {
            f.problem = Some(t!("folders-sharing-password-short", n = PASSWORD_LEAST));
            return;
        }
        f.busy = Some(t!("folders-sharing-keeping-password").into());
        let secret = Zeroizing::new(f.password.to_string());
        self.sharing_off(move || SharingMsg::PasswordSaved(sharing::set_password(secret)));
    }

    fn sharing_listed(&mut self, got: Result<Vec<Shared>, Problem>) {
        let marks_before: Vec<PathBuf> = self.sharing.shared.iter().map(|s| s.path.clone()).collect();
        match got {
            Ok(list) => {
                self.sharing.shared = list;
                self.sharing.problem = None;
            }
            Err(p) => {
                if matches!(p, Problem::NoSamba | Problem::NotSetUp) {
                    self.sharing.shared.clear();
                }
                self.sharing.problem = Some(p);
            }
        }
        let s = std::mem::take(&mut self.sharing);
        if let Some(f) = self.form() {
            if f.busy.is_none() && s.of(&f.folder).map(|x| &x.name) != f.was.as_ref() {
                f.follow(&s);
            }
        }
        let marks_now: Vec<PathBuf> = s.shared.iter().map(|x| x.path.clone()).collect();
        self.sharing = s;
        if marks_before != marks_now {
            self.reload_all();
        }
    }

    fn sharing_setup(&mut self) {
        let fetch = !sharing::samba_here();
        let Some(f) = self.form() else { return };
        f.busy = Some(if fetch { t!("folders-sharing-fetching") } else { t!("folders-sharing-setting-up") }.into());
        let folder = f.folder.clone();
        self.sharing_off(move || SharingMsg::SetUp(folder, sharing::set_up()));
    }

    fn share_now(&mut self) {
        let Some(f) = self.form() else { return };
        let want = f.wanted();
        let old = f.was.clone().filter(|w| !w.eq_ignore_ascii_case(&want.name));
        f.busy = Some(if f.was.is_some() { t!("folders-sharing-changing") } else { t!("folders-sharing-sharing-it") }.into());
        let folder = f.folder.clone();
        self.sharing_off(move || {
            let got = sharing::add(&want).and_then(|()| old.map_or(Ok(()), |o| sharing::remove(&o)));
            SharingMsg::Made(folder, got, sharing::list())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::sharing::pretend;
    use crate::files::dialogs::AskMsg;
    use crate::files::menu::Act;
    use crate::files::places::Where;
    use crate::toolkit::ui::App;

    fn folder(tag: &str) -> PathBuf {
        pretend::reset();
        let dir = std::env::temp_dir().join(format!("trindle-files-sharing-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Photos")).unwrap();
        dir
    }

    fn said(f: &Files) -> Vec<String> {
        match &f.dialog {
            Some(Dialog::Properties(p)) => p.share.as_ref().map(|s| column(view(s)).said()).unwrap_or_default(),
            Some(Dialog::Sharing(s)) => card_view(s).said(),
            _ => Vec::new(),
        }
    }

    fn form(f: &Files) -> &Form {
        match &f.dialog {
            Some(Dialog::Properties(p)) => p.share.as_ref().unwrap(),
            Some(Dialog::Sharing(s)) => s,
            _ => panic!("no Share part"),
        }
    }

    #[test]
    fn properties_share_a_folder_the_first_time_after_setting_up() {
        let dir = folder("first");
        let photos = dir.join("Photos");
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        f.ask(Act::Properties, 0, vec![photos.clone()]);
        let words = said(&f);
        assert!(words.contains(&"switch: Share this folder with other computers on the home network".to_string()), "{words:?}");
        assert!(words.contains(&"entry: The name other computers see".to_string()));
        assert!(words.contains(&"switch: Others can change files".to_string()));
        assert!(words.contains(&"switch: Let in anyone, without a password".to_string()));
        assert!(words.iter().any(|w| w.contains("fetches Samba")), "{words:?}");
        assert_eq!(form(&f).name, "Photos");
        assert!(!form(&f).write && !form(&f).guests, "read only and with a password at first");

        f.update(Msg::Sharing(SharingMsg::On(true)));
        pretend::with(|c| {
            assert!(c.installed && c.set_up, "set up on first use");
            assert_eq!(c.shares, [Shared { name: "photos".into(), path: photos.clone(), write: false, guests: false }]);
            assert!(c.ran.iter().all(|argv| argv.iter().all(|w| !w.contains("secret"))));
        });
        assert!(f.is_shared(&photos));
        let words = said(&f);
        assert!(words.iter().any(|w| w.starts_with("Other computers find it under Network, as photos")), "{words:?}");
        assert!(words.contains(&"button: Let other computers connect".to_string()), "{words:?}");
        f.update(Msg::Sharing(SharingMsg::LetConnect));
        assert!(!said(&f).contains(&"button: Let other computers connect".to_string()));
        assert!(said(&f).contains(&"secret: Sharing password".to_string()));
        assert!(said(&f).iter().any(|w| w.contains("the password other computers use to open your shared folders")));
        f.update(Msg::Sharing(SharingMsg::Password("short".into())));
        f.update(Msg::Sharing(SharingMsg::SavePassword));
        assert!(form(&f).problem.as_deref().is_some_and(|p| p.contains("at least 8")));
        f.update(Msg::Sharing(SharingMsg::Password("secret-words".into())));
        f.update(Msg::Sharing(SharingMsg::SavePassword));
        assert!(pretend::with(|c| c.password));
        assert!(said(&f).contains(&"button: Change the password other computers use".to_string()));
        pretend::with(|c| assert!(c.ran.iter().flatten().all(|w| !w.contains("secret-words"))));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_share_is_changed_renamed_and_stopped() {
        let dir = folder("change");
        pretend::with(|c| {
            c.installed = true;
            c.set_up = true;
        });
        let photos = dir.join("Photos");
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        f.ask(Act::ShareOnNetwork, 0, vec![photos.clone()]);
        assert!(matches!(f.dialog, Some(Dialog::Sharing(_))));
        assert!(said(&f)[0].starts_with("Share Photos on the network"));
        f.update(Msg::Sharing(SharingMsg::On(true)));
        f.update(Msg::Sharing(SharingMsg::Write(true)));
        f.update(Msg::Sharing(SharingMsg::Guests(true)));
        assert_eq!(pretend::with(|c| c.shares.clone()), [Shared { name: "photos".into(), path: photos.clone(), write: true, guests: true }]);
        assert!(!said(&f).iter().any(|w| w.contains("password other computers")));
        f.update(Msg::Sharing(SharingMsg::Name("Holiday".into())));
        f.update(Msg::Sharing(SharingMsg::NameDone));
        assert_eq!(pretend::with(|c| c.shares.iter().map(|s| s.name.clone()).collect::<Vec<_>>()), ["holiday"]);
        f.update(Msg::Sharing(SharingMsg::Name("a/b".into())));
        f.update(Msg::Sharing(SharingMsg::NameDone));
        assert!(form(&f).problem.is_some(), "a bad name is said, not sent");
        f.update(Msg::Sharing(SharingMsg::On(false)));
        assert!(pretend::with(|c| c.shares.is_empty()));
        assert!(!f.is_shared(&photos));
        f.update(Msg::Ask(AskMsg::Close));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn setting_up_failing_is_said_plainly() {
        let dir = folder("offline");
        pretend::with(|c| c.setup_fails = Some("Samba could not be fetched: this computer seems to be offline.".into()));
        let mut f = Files::new(vec![dir.clone()]);
        f.ask(Act::Properties, 0, vec![dir.join("Photos")]);
        f.update(Msg::Sharing(SharingMsg::On(true)));
        assert!(!form(&f).on);
        assert!(said(&f).iter().any(|w| w.contains("seems to be offline")));
        std::fs::write(dir.join("a.txt"), "a").unwrap();
        f.ask(Act::Properties, 0, vec![dir.join("a.txt")]);
        assert!(matches!(&f.dialog, Some(Dialog::Properties(p)) if p.share.is_none()));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn closed_folders_are_named_plainly() {
        let home = crate::common::files::home();
        let folder = PathBuf::from("/srv/x/Photos");
        assert_eq!(closed_words(std::slice::from_ref(&home), &folder), "your home folder");
        assert_eq!(closed_words(&[home, folder.clone()], &folder), "your home folder and this folder");
    }

    #[test]
    fn a_sharing_problem_has_a_more() {
        let more = format!("button: {}", t!("common-more"));
        let mut form = Form::new(&std::env::temp_dir(), &Sharing::default());
        form.cannot = None;
        assert!(!column(view(&form)).said().contains(&more));
        form.problem = Some(t!("folders-sharing-name-taken").into());
        assert!(column(view(&form)).said().contains(&more));
        form.problem = None;
        form.on = false;
        form.cannot = Some(t!("folders-sharing-only-yours").into());
        assert!(column(view(&form)).said().contains(&more));
    }
}
