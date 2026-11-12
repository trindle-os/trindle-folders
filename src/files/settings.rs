//! Folders' own settings, which live here in Folders and not in the

use std::path::{Path, PathBuf};

use super::app::{Files, Msg};
use super::dialogs::{head, Dialog};
use super::icons::stand_in;
use super::shares::{left_words, KeptRow};
use super::density::Density;
use crate::common::folders_conf::{self as conf, DENSITY, ENDINGS, LEFT, OPEN_DRIVES, RECONNECT, RIGHT, START, TICKS};
use crate::toolkit::t;
use crate::toolkit::tiles::radio_group;
use crate::toolkit::widget::{button, card, choice, column, form_label, note, one_of, row, switch, Length, Node, Tone};

pub fn name() -> &'static str {
    t!("folders-settings-name")
}
pub const OPEN_FLAG: &str = "--settings";

const WIDTH: f32 = 520.0;
const PLACE_W: f32 = 180.0;

pub(super) fn problem(why: String, page: &'static str) -> Node<Msg> {
    row(vec![note(why).tone(Tone::Problem).width(Length::Fill), crate::common::more::button(Msg::Help(page))]).gap(8.0).width(Length::Fill)
}

fn not_saved() -> &'static str {
    t!("folders-settings-not-saved")
}
fn not_read(why: &str) -> String {
    t!("folders-settings-not-read", why = why)
}
pub fn no_ring() -> &'static str {
    t!("folders-settings-no-ring")
}

fn pages() -> [&'static str; 5] {
    [t!("folders-settings-page-start"), t!("folders-settings-page-list"), t!("folders-settings-page-drives"), t!("folders-settings-page-logins"), t!("folders-settings-page-shares")]
}

const LOGINS: usize = 3;
const SHARES: usize = 4;
fn logins_about() -> &'static str {
    t!("folders-settings-logins-about")
}
fn no_logins() -> &'static str {
    t!("folders-settings-no-logins")
}
fn logins_wait() -> &'static str {
    t!("folders-settings-logins-wait")
}

fn starts() -> [(&'static str, &'static str, &'static str); 3] {
    [
        ("", t!("folders-settings-start-usual"), t!("folders-settings-start-usual-sub")),
        ("last", t!("folders-settings-start-last"), t!("folders-settings-start-last-sub")),
        ("chosen", t!("folders-settings-start-chosen"), ""),
    ]
}

fn places() -> [(&'static str, &'static str); 6] {
    [
        ("recent:", t!("folders-settings-place-recent")),
        ("~", t!("folders-settings-place-home")),
        ("~/Documents", t!("folders-settings-place-documents")),
        ("~/Downloads", t!("folders-settings-place-downloads")),
        ("~/Pictures", t!("folders-settings-place-pictures")),
        ("trash:", t!("folders-settings-place-trash")),
    ]
}

fn side_words() -> [(&'static str, &'static str, &'static str); 2] {
    [(LEFT, t!("folders-settings-left-pane"), ""), (RIGHT, t!("folders-settings-right-pane"), "")]
}

#[derive(Clone)]
pub enum OwnMsg {
    Open,
    Page(usize),
    Start(usize),
    Place(usize, usize),
    Choose(usize),
    Chosen(usize, PathBuf),
    Endings(bool),
    Ticks(bool),
    OpenDrives(bool),
    Reconnect(bool),
    Density(usize),
    Forget(String),
}

pub struct Card {
    pub page: usize,
    start: String,
    sides: [String; 2],
    endings: bool,
    ticks: bool,
    density: Density,
    open_drives: bool,
    reconnect: bool,
    ring: bool,
    kept: Option<(&'static str, u32)>,
    failed: Option<(&'static str, String)>,
    unread: Option<String>,
    logins: Option<Result<Vec<KeptRow>, String>>,
    forget_failed: Option<(String, String)>,
}

impl Card {
    fn read(file: Option<&Path>, f: &Files) -> Card {
        let get = |key: &str| file.map_or_else(|| conf::usual(key).to_string(), |file| conf::get_in(file, key));
        Card {
            page: 0,
            start: get(START),
            sides: [get(LEFT), get(RIGHT)],
            endings: f.endings,
            ticks: f.ticks,
            density: f.density,
            open_drives: get(OPEN_DRIVES) == "yes",
            reconnect: get(RECONNECT) == "yes",
            ring: f.ring.is_some(),
            kept: None,
            failed: None,
            unread: file.and_then(conf::unreadable),
            logins: None,
            forget_failed: None,
        }
    }

    fn glow(&self, key: &str) -> u32 {
        self.kept.filter(|(k, _)| *k == key).map_or(0, |(_, n)| n)
    }
}

fn written(p: &Path, home: &Path) -> String {
    match p.strip_prefix(home) {
        Ok(rest) if rest.as_os_str().is_empty() => "~".into(),
        Ok(rest) => format!("~/{}", rest.to_string_lossy()),
        Err(_) => p.to_string_lossy().into_owned(),
    }
}

fn place_path(text: &str, home: &Path) -> Option<PathBuf> {
    match text {
        "recent:" | "trash:" | "" => None,
        "~" => Some(home.to_path_buf()),
        t => Some(t.strip_prefix("~/").map_or_else(|| PathBuf::from(t), |r| home.join(r))),
    }
}

impl Files {
    pub(super) fn read_own(&mut self) {
        let Some(file) = &self.own_file else { return };
        self.endings = conf::get_in(file, ENDINGS) != "no";
        self.ticks = conf::get_in(file, TICKS) == "yes";
        self.set_density(Density::named(&conf::get_in(file, DENSITY)));
    }

    fn own_keep(&mut self, key: &'static str, value: &str) -> bool {
        let done = match &self.own_file {
            Some(file) => conf::set_in(file, key, value),
            None => Ok(()),
        };
        let Some(Dialog::Settings(c)) = &mut self.dialog else { return done.is_ok() };
        match &done {
            Ok(()) => {
                c.kept = Some((key, c.kept.map_or(1, |(_, n)| n + 1)));
                if c.failed.as_ref().is_some_and(|(k, _)| *k == key) {
                    c.failed = None;
                }
            }
            Err(e) => {
                let file_bad = self.own_file.as_deref().and_then(conf::unreadable);
                c.failed = Some((key, match (&file_bad, e.kind()) {
                    (Some(why), _) => t!("folders-settings-not-saved-because", why = why),
                    (None, std::io::ErrorKind::InvalidInput) => t!("folders-settings-not-saved-because", why = &e.to_string()),
                    _ => not_saved().to_string(),
                }));
                c.unread = file_bad;
            }
        }
        done.is_ok()
    }

    pub(super) fn own_msg(&mut self, m: OwnMsg) {
        if let OwnMsg::Open = m {
            if self.dialog.is_none() && self.confirm.is_none() && self.dropping.is_none() {
                self.menu = None;
                self.dialog = Some(Dialog::Settings(Box::new(Card::read(self.own_file.as_deref(), self))));
            }
            return;
        }
        if !matches!(self.dialog, Some(Dialog::Settings(_))) {
            return;
        }
        match m {
            OwnMsg::Open => {}
            OwnMsg::Page(i) => {
                let page = i.min(pages().len() - 1);
                self.own_card(|c| c.page = page);
                if page == LOGINS {
                    self.own_card(|c| c.logins = None);
                    self.load_kept();
                }
            }
            OwnMsg::Forget(server) => {
                self.own_card(|c| c.forget_failed = None);
                self.forget_kept(server);
            }
            OwnMsg::Start(_) | OwnMsg::Place(..) | OwnMsg::Choose(_) | OwnMsg::Chosen(..) => self.own_places_msg(m),
            OwnMsg::Endings(_) | OwnMsg::Ticks(_) | OwnMsg::Density(_) | OwnMsg::Reconnect(_) | OwnMsg::OpenDrives(_) => self.own_switch_msg(m),
        }
    }

    fn own_places_msg(&mut self, m: OwnMsg) {
        match m {
            OwnMsg::Start(i) => {
                let Some(&(value, ..)) = starts().get(i) else { return };
                if self.own_keep(START, value) {
                    self.own_card(|c| c.start = value.to_string());
                }
            }
            OwnMsg::Place(side, i) => {
                let (Some(&(key, ..)), Some(&(value, _))) = (side_words().get(side), places().get(i)) else { return };
                if self.own_keep(key, value) {
                    self.own_card(|c| c.sides[side] = value.to_string());
                }
            }
            OwnMsg::Choose(side) => {
                let Some(now) = (match &self.dialog {
                    Some(Dialog::Settings(c)) => c.sides.get(side).cloned(),
                    _ => None,
                }) else { return };
                let ask = super::act::Ask {
                    title: t!("folders-settings-choose-title").into(),
                    folder: Some(place_path(&now, &self.home).unwrap_or_else(|| self.home.clone())),
                    directory: true,
                };
                self.pick(ask, move |paths| paths.into_iter().next().map(|p| Msg::Own(OwnMsg::Chosen(side, p))));
            }
            OwnMsg::Chosen(side, path) => {
                let Some(&(key, ..)) = side_words().get(side) else { return };
                let value = written(&path, &self.home);
                if self.own_keep(key, &value) {
                    self.own_card(|c| c.sides[side] = value);
                }
            }
            _ => {}
        }
    }

    fn own_switch_msg(&mut self, m: OwnMsg) {
        let yes_no = |on: bool| if on { "yes" } else { "no" };
        match m {
            OwnMsg::Endings(on) => {
                if self.own_keep(ENDINGS, yes_no(on)) {
                    self.endings = on;
                    self.own_card(|c| c.endings = on);
                }
            }
            OwnMsg::Ticks(on) => {
                if self.own_keep(TICKS, yes_no(on)) {
                    self.ticks = on;
                    self.own_card(|c| c.ticks = on);
                }
            }
            OwnMsg::Density(i) => {
                let Some(level) = Density::ALL.get(i).copied() else { return };
                if self.own_keep(DENSITY, level.key()) {
                    self.set_density(level);
                    self.own_card(|c| c.density = level);
                }
            }
            OwnMsg::Reconnect(on) => {
                if self.own_keep(RECONNECT, yes_no(on)) {
                    self.own_card(|c| c.reconnect = on);
                }
            }
            OwnMsg::OpenDrives(on) if self.own_keep(OPEN_DRIVES, yes_no(on)) => self.own_card(|c| c.open_drives = on),
            _ => {}
        }
    }

    pub(super) fn own_logins(&mut self, got: Result<Vec<KeptRow>, String>) {
        self.own_card(|c| c.logins = Some(got));
    }

    pub(super) fn own_forgot(&mut self, server: &str, got: Result<(), String>) {
        let again = matches!(self.dialog, Some(Dialog::Settings(_)));
        match got {
            Ok(()) => self.own_card(|c| {
                c.forget_failed = None;
                if let Some(Ok(rows)) = &mut c.logins {
                    rows.retain(|r| r.server != server);
                }
            }),
            Err(why) => {
                let said = t!("folders-settings-forget-failed", server = server, why = why.trim_end_matches('.'));
                self.own_card(|c| c.forget_failed = Some((server.to_string(), said)));
            }
        }
        if again {
            self.load_kept();
        }
    }

    fn own_card(&mut self, change: impl FnOnce(&mut Card)) {
        if let Some(Dialog::Settings(c)) = &mut self.dialog {
            change(c);
        }
    }
}

fn own(m: OwnMsg) -> Msg {
    Msg::Own(m)
}

fn line(c: &Card, key: &'static str, name: &str, sub: &str, controls: Vec<Node<Msg>>) -> Node<Msg> {
    let mut words = super::dialogs::named(name, sub);
    if let Some((_, why)) = c.failed.as_ref().filter(|(k, _)| *k == key) {
        words.push(problem(why.clone(), super::help::NOT_KEPT));
    }
    let turn = c.glow(key);
    let buttons = !controls.iter().any(Node::holds_box) && controls.iter().all(Node::only_buttons);
    let mut items = vec![column(words).gap(4.0).width(Length::Fill)];
    items.extend(controls.into_iter().map(|n| if turn > 0 { n.glow_kept(turn, buttons) } else { n }));
    row(items).gap(8.0).width(Length::Fill).centred()
}

fn side_line(c: &Card, side: usize) -> Node<Msg> {
    let (key, heading, sub) = side_words()[side];
    let now = &c.sides[side];
    let all = places();
    let mut options: Vec<String> = all.iter().map(|(_, w)| w.to_string()).collect();
    let mut chosen = all.iter().position(|(v, _)| v == now);
    if chosen.is_none() {
        options.push(now.clone());
        chosen = Some(options.len() - 1);
    }
    let usual = all.len();
    let pick = one_of(options, chosen, heading, move |i| own(if i < usual { OwnMsg::Place(side, i) } else { OwnMsg::Page(0) }))
        .key(format!("own:{key}")).width(Length::Fixed(PLACE_W));
    let other = button("folder-open-symbolic", t!("folders-settings-choose-another")).on_press(own(OwnMsg::Choose(side)));
    line(c, key, heading, sub, vec![pick, other])
}

pub fn view(c: &Card) -> Node<Msg> {
    let page = c.page.min(pages().len() - 1);
    let mut lines = vec![
        head(stand_in::SETTINGS, name()),
        choice(pages().iter().map(|p| p.to_string()).collect(), page, t!("folders-settings-pages-tip"), |i| own(OwnMsg::Page(i))),
    ];
    if let Some(why) = &c.unread {
        lines.push(problem(not_read(why), super::help::NOT_KEPT));
    }
    let more = match page {
        0 => start_page(c),
        1 => list_page(c),
        LOGINS => logins_page(c),
        SHARES => vec![line(c, RECONNECT, t!("folders-settings-reconnect"), t!("folders-settings-reconnect-sub"),
                            vec![switch(c.reconnect, t!("folders-settings-reconnect"), |on| own(OwnMsg::Reconnect(on)))])],
        _ => drives_page(c),
    };
    lines.extend(more);
    card(column(lines).gap(12.0).width(Length::Fill)).width(Length::Fixed(WIDTH)).on_escape(Msg::Ask(super::dialogs::AskMsg::Close))
}

fn start_page(c: &Card) -> Vec<Node<Msg>> {
    let mut lines = Vec::new();
    let all = starts();
    let chosen = all.iter().position(|(v, _, _)| *v == c.start).unwrap_or(0);
    let options = all.iter().map(|(_, name, sub)| (name.to_string(), sub.to_string())).collect();
    lines.push(radio_group(options, chosen, |i| own(OwnMsg::Start(i))));
    if let Some((_, why)) = c.failed.as_ref().filter(|(k, _)| *k == START) {
        lines.push(problem(why.clone(), super::help::NOT_KEPT));
    }
    lines.push(note(t!("folders-settings-start-note")).width(Length::Fill));
    let own_places = all[chosen].0 == "chosen";
    lines.push(side_line(c, 0).off(!own_places));
    lines.push(side_line(c, 1).off(!own_places));
    lines
}

fn list_page(c: &Card) -> Vec<Node<Msg>> {
    let level = Density::ALL.iter().position(|d| *d == c.density).unwrap_or(1);
    let strip = choice(Density::ALL.iter().map(|d| d.word().to_string()).collect(), level, t!("folders-settings-density-tip"), |i| own(OwnMsg::Density(i)))
        .key("own:density");
    vec![
        line(c, DENSITY, t!("folders-settings-density"), t!("folders-settings-density-sub"), vec![strip]),
        line(c, TICKS, t!("folders-settings-ticks"), "",
             vec![switch(c.ticks, t!("folders-settings-ticks"), |on| own(OwnMsg::Ticks(on)))]),
        line(c, ENDINGS, t!("folders-settings-endings"), t!("folders-settings-endings-sub"),
             vec![switch(c.endings, t!("folders-settings-endings"), |on| own(OwnMsg::Endings(on)))]),
        note(t!("folders-settings-list-note")).width(Length::Fill),
    ]
}

fn logins_page(c: &Card) -> Vec<Node<Msg>> {
    let mut lines = vec![note(logins_about()).width(Length::Fill)];
    match &c.logins {
        None => lines.push(note(logins_wait()).width(Length::Fill)),
        Some(Err(why)) => lines.push(problem(why.clone(), super::help::SHARE_LOGIN)),
        Some(Ok(rows)) if rows.is_empty() => lines.push(note(no_logins()).width(Length::Fill)),
        Some(Ok(rows)) => {
            for r in rows {
                let words = column(vec![form_label(r.server.clone()), note(t!("folders-settings-login-line", users = &r.users, left = &left_words(r.left)))]).gap(4.0).width(Length::Fill);
                let forget = button(stand_in::SHRED, t!("folders-settings-forget", server = &r.server)).cannot_be_undone().on_press(own(OwnMsg::Forget(r.server.clone())));
                lines.push(row(vec![words, forget]).gap(8.0).width(Length::Fill).centred());
            }
        }
    }
    if let Some((_, why)) = &c.forget_failed {
        lines.push(problem(why.clone(), super::help::SHARE_LOGIN));
    }
    lines
}

fn drives_page(c: &Card) -> Vec<Node<Msg>> {
    let drives = line(c, OPEN_DRIVES, t!("folders-settings-drives"), t!("folders-settings-drives-sub"),
                      vec![switch(c.open_drives, t!("folders-settings-drives"), |on| own(OwnMsg::OpenDrives(on)))]);
    let about = if c.ring { t!("folders-settings-ring-note") } else { no_ring() };
    vec![drives.off(!c.ring), note(about).width(Length::Fill)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::ui::App;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-folders-own-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Work")).unwrap();
        dir
    }

    fn words(f: &Files) -> String {
        match &f.dialog {
            Some(Dialog::Settings(c)) => view(c).said().join("\n"),
            _ => String::new(),
        }
    }

    fn kept(dir: &Path) -> String {
        std::fs::read_to_string(dir.join(conf::FILE)).unwrap_or_default()
    }

    #[test]
    fn every_option_settings_had_is_in_folders_and_kept_in_its_own_file() {
        let dir = scratch("all");
        let mut f = Files::new(vec![dir.clone()]);
        f.own_file = Some(dir.join(conf::FILE));
        f.home = dir.clone();
        f.update(Msg::Own(OwnMsg::Open));
        let first = words(&f);
        for (_, name, sub) in starts() {
            assert!(first.contains(name) && first.contains(sub), "{name} and its line: {first}");
        }
        assert!(starts().iter().all(|(_, _, sub)| sub.chars().count() <= 45), "one short line each");
        assert!(first.contains("Left pane") && first.contains("Right pane") && first.contains("Choose another folder"), "{first}");
        assert!(!first.contains("Folders opens with"), "the usual start needs no sentence of its own: {first}");
        f.update(Msg::Own(OwnMsg::Page(1)));
        let list = words(&f);
        assert!(list.contains("switch: Tick boxes for selecting") && list.contains("switch: Show file endings"), "{list}");
        assert!(list.contains("Spacing") && list.contains("Applies to every view."), "{list}");
        f.update(Msg::Own(OwnMsg::Page(2)));
        assert!(words(&f).contains("switch: Open drives when plugged in"));
        assert_eq!(kept(&dir), "");
        f.update(Msg::Own(OwnMsg::Start(2)));
        f.update(Msg::Own(OwnMsg::Place(0, 2)));
        f.update(Msg::Own(OwnMsg::Chosen(1, dir.join("Work"))));
        f.update(Msg::Own(OwnMsg::Ticks(true)));
        f.update(Msg::Own(OwnMsg::Endings(false)));
        f.update(Msg::Own(OwnMsg::OpenDrives(true)));
        f.update(Msg::Own(OwnMsg::Density(0)));
        assert_eq!(kept(&dir), "start=chosen\nleft=~/Documents\nright=~/Work\nticks=yes\nendings=no\nopen_drives=yes\ndensity=compact\n");
        assert!(f.ticks && !f.endings && f.density == Density::Compact);
        f.update(Msg::Own(OwnMsg::Page(0)));
        let shown = words(&f);
        assert!(!shown.contains("Folders opens with"), "{shown}");
        f.update(Msg::Ask(super::super::dialogs::AskMsg::Close));
        assert!(f.dialog.is_none());
        f.update(Msg::Own(OwnMsg::Open));
        let Some(Dialog::Settings(c)) = &f.dialog else { panic!("the card opens") };
        assert_eq!((c.start.as_str(), c.sides[0].as_str(), c.sides[1].as_str(), c.open_drives), ("chosen", "~/Documents", "~/Work", true));
        let mut again = Files::new(vec![dir.clone()]);
        again.own_file = Some(dir.join(conf::FILE));
        again.update(Msg::Settings);
        assert!(again.ticks && !again.endings && again.density == Density::Compact, "read as the window has the keyboard again");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_setting_that_cannot_be_kept_says_so_in_red_and_stays_as_it_was() {
        let dir = scratch("refused");
        let mut f = Files::new(vec![dir.clone()]);
        std::fs::write(dir.join("blocked"), b"made up").unwrap();
        f.own_file = Some(dir.join("blocked").join(conf::FILE));
        f.update(Msg::Own(OwnMsg::Open));
        f.update(Msg::Own(OwnMsg::Page(1)));
        f.update(Msg::Own(OwnMsg::Ticks(true)));
        assert!(!f.ticks, "not kept, so not shown as on");
        assert!(words(&f).contains("Folders couldn't save this setting"), "{}", words(&f));
        let Some(Dialog::Settings(c)) = &f.dialog else { panic!("the card stays") };
        assert!(!c.ticks && c.glow(TICKS) == 0);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn settings_that_cannot_be_read_are_said_in_red_and_never_written_over() {
        let dir = scratch("unread");
        let mut f = Files::new(vec![dir.clone()]);
        let was = b"density=roomy\nleft=~/Work\nstart=chosen\n\xff\n".to_vec();
        std::fs::write(dir.join(conf::FILE), &was).unwrap();
        f.own_file = Some(dir.join(conf::FILE));
        f.update(Msg::Own(OwnMsg::Open));
        let top = not_read("the file that keeps it is not text and cannot be read");
        assert!(words(&f).contains(&top), "{}", words(&f));
        f.update(Msg::Own(OwnMsg::Page(1)));
        assert!(words(&f).contains(&top), "on every page: {}", words(&f));
        f.update(Msg::Own(OwnMsg::Ticks(true)));
        assert!(!f.ticks, "not kept, so not shown as on");
        assert!(words(&f).contains(&t!("folders-settings-not-saved-because", why = "the file that keeps it is not text and cannot be read")), "{}", words(&f));
        assert_eq!(std::fs::read(dir.join(conf::FILE)).unwrap(), was, "every setting is still in the file");
        std::fs::write(dir.join(conf::FILE), "density=roomy\nleft=~/Work\n").unwrap();
        f.update(Msg::Ask(crate::files::dialogs::AskMsg::Close));
        f.update(Msg::Own(OwnMsg::Open));
        f.update(Msg::Own(OwnMsg::Page(1)));
        assert!(!words(&f).contains("couldn't read its settings"), "{}", words(&f));
        f.update(Msg::Own(OwnMsg::Ticks(true)));
        assert!(f.ticks && !words(&f).contains("Folders couldn't save this setting"));
        assert_eq!(kept(&dir), "density=roomy\nleft=~/Work\nticks=yes\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_change_kept_glows_on_its_own_line_only() {
        let dir = scratch("glow");
        let mut f = Files::new(vec![dir.clone()]);
        f.own_file = Some(dir.join(conf::FILE));
        f.update(Msg::Own(OwnMsg::Open));
        f.update(Msg::Own(OwnMsg::Place(0, 3)));
        let Some(Dialog::Settings(c)) = &f.dialog else { panic!("the card is open") };
        assert!(c.glow(LEFT) > 0 && c.glow(RIGHT) == 0 && c.glow(TICKS) == 0);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn without_the_ring_opening_drives_is_greyed_and_says_why() {
        let dir = scratch("alone");
        let mut f = Files::new(vec![dir.clone()]);
        f.ring = None;
        f.update(Msg::Own(OwnMsg::Open));
        f.update(Msg::Own(OwnMsg::Page(2)));
        assert!(words(&f).contains(no_ring()), "{}", words(&f));
        f.dialog = None;
        f.ring = Some(dir.join("favourites"));
        f.update(Msg::Own(OwnMsg::Open));
        f.update(Msg::Own(OwnMsg::Page(2)));
        assert!(!words(&f).contains(no_ring()));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_cog_and_ctrl_comma_open_the_card_and_escape_closes_it() {
        use crate::toolkit::input::{Input, Keysym, Mods};
        let dir = scratch("keys");
        let mut f = Files::new(vec![dir.clone()]);
        let ctrl = Mods { ctrl: true, ..Mods::default() };
        let press = |f: &Files, sym: Keysym, mods| f.intercept(&Input::Key { sym, mods, text: None, repeat: false }, false);
        let Some(m) = press(&f, Keysym::comma, ctrl) else { panic!("Ctrl and comma is answered") };
        f.update(m);
        assert!(matches!(f.dialog, Some(Dialog::Settings(_))));
        let Some(m) = press(&f, Keysym::Escape, Mods::default()) else { panic!("Escape closes it") };
        f.update(m);
        assert!(f.dialog.is_none());
        f.update(Msg::Menu(super::super::menu::MenuMsg::More));
        assert!(f.menu.as_ref().is_some_and(|m| m.lines.iter().any(|l| l.words == name() && l.act == super::super::menu::Act::Settings)));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn folders_are_written_from_home() {
        let home = Path::new("/home/john");
        assert_eq!(written(Path::new("/home/john"), home), "~");
        assert_eq!(written(Path::new("/home/john/Work/2026"), home), "~/Work/2026");
        assert_eq!(written(Path::new("/srv/share"), home), "/srv/share");
        assert_eq!(place_path("~/Work", home), Some(PathBuf::from("/home/john/Work")));
        assert_eq!(place_path("recent:", home), None);
    }

    #[test]
    fn kept_logins_are_listed_with_time_left_and_forgotten_by_a_red_button() {
        use crate::common::smb::Login;
        use super::super::shares::{self, LAPSE};
        let dir = scratch("logins");
        let mut f = Files::new(vec![dir.clone()]);
        let jo = Login { user: "jo".into(), domain: String::new(), password: zeroize::Zeroizing::new("right".into()) };
        let day = 24 * 60 * 60;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        shares::pretend::kept_on("nas.invalid", jo.clone(), &[("10.0.0.2", now - 10 * day)]);
        shares::pretend::kept_on("old.invalid", jo, &[("10.0.0.3", now - LAPSE - day)]);
        f.update(Msg::Own(OwnMsg::Open));
        f.update(Msg::Own(OwnMsg::Page(LOGINS)));
        let said = words(&f);
        assert!(said.contains("nas.invalid") && said.contains("jo. 80 days left."), "{said}");
        assert!(said.contains("old.invalid") && said.contains("jo. You'll be asked again next time."), "{said}");
        assert!(said.contains("Forget nas.invalid"), "{said}");
        f.update(Msg::Own(OwnMsg::Forget("nas.invalid".into())));
        let said = words(&f);
        assert!(!said.contains("Forget nas.invalid") && said.contains("Forget old.invalid"), "{said}");
        assert_eq!(shares::kept_rows().unwrap().len(), 1);
        shares::pretend::keyring_cannot_say(Some("The keyring is locked."));
        f.update(Msg::Own(OwnMsg::Page(LOGINS)));
        assert!(words(&f).contains("The keyring is locked."), "{}", words(&f));
        shares::pretend::keyring_cannot_say(None);
        shares::pretend::nothing_kept();
        f.update(Msg::Own(OwnMsg::Page(LOGINS)));
        assert!(words(&f).contains(no_logins()));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
