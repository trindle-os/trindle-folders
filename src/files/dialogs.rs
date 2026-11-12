//! The small windows the right-click menu opens inside Files: Open with,

use tiny_skia::{FillRule, PixmapMut, Transform};

use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::app::{Files, Msg};
use super::columns::Col;
use super::icons;
use super::kinds::when_text;
use super::menu::Act;
use super::sizes::{self, Tally};
use super::view::{items_words, size_words};
use super::work::{self, Kind};
use crate::common::apps::{self, Entry};
use crate::common::style::ACCENTS;
use crate::toolkit::colour::Colour;
use crate::toolkit::t;
use crate::toolkit::theme::Palette;
use crate::toolkit::tiles::standard_ring;
use crate::toolkit::widget::{button, card, choice, column, entry, form_label, heading, icon_circle, item, label, list, main_action, note, row, scroll, space,
                             switch, Length, Node, Rect, Tone};

const WIDTH: f32 = 420.0;
const PROPS_ROOM: f32 = 200.0;
const PROPS_LEAST: f32 = 240.0;
const PROPS_LINE: f32 = 40.0;
const LIST_H: f32 = 300.0;
pub(super) const KEY_W: f32 = 110.0;
const SWATCH: f32 = 22.0;
const SWATCH_ROW: usize = 5;
const RING: f32 = 32.0;

pub enum Dialog {
    OpenWith(OpenWith),
    Properties(Box<Props>),
    SendTo { items: Vec<PathBuf>, places: Vec<(String, PathBuf)>, find: String },
    Colour { folders: Vec<PathBuf>, now: Option<String> },
    Tool(super::tools::Tool),
    Connect(super::far::Connect),
    Share(super::shares::Ask),
    Sharing(Box<super::sharing::Form>),
    Columns(Vec<Col>),
    Preview(super::preview::Preview),
    Unlock(super::locked::Unlock),
    Earlier(Box<super::earlier::Card>),
    Settings(Box<super::settings::Card>),
    Dropped(Box<super::outside::Asked>),
}

pub struct OpenWith {
    pub file: PathBuf,
    pub kind: String,
    pub suggested: Vec<Entry>,
    pub every: Vec<Entry>,
    pub all: bool,
    pub chosen: Option<usize>,
    pub default: Option<String>,
    pub always: bool,
    pub find: String,
}

const FIND_FROM: usize = 8;

impl OpenWith {
    fn whole(&self) -> &[Entry] {
        if self.all { &self.every } else { &self.suggested }
    }

    fn shown(&self) -> Vec<Entry> {
        let words = self.find.trim().to_lowercase();
        self.whole().iter().filter(|e| words.is_empty() || e.name.to_lowercase().contains(&words)).cloned().collect()
    }
}

pub struct Props {
    pub items: Vec<PathBuf>,
    pub rows: Vec<(&'static str, String)>,
    pub tally: Option<Tally>,
    pub done: bool,
    pub id: u64,
    pub stop: Arc<AtomicBool>,
    pub details: Option<super::details::Details>,
    pub sums: Sums,
    pub check: String,
    pub share: Option<super::sharing::Form>,
    pub tall: f32,
    pub page: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sums {
    None,
    Unasked,
    Working(f32),
    Done(Vec<(&'static str, String)>),
    Failed,
}

impl Drop for Props {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub enum AskMsg {
    Choose(usize),
    Every(bool),
    Always(bool),
    OpenIt,
    OpenWithRow(usize),
    Find(String),
    FindPlace(String),
    Send(usize),
    Colour(Option<usize>),
    Measured(u64, Tally, bool),
    Sums,
    SumsFar(u64, f32),
    SumsGot(u64, Option<Vec<(&'static str, String)>>),
    SumsCheck(String),
    Detail(super::details::DetailMsg),
    Page(usize),
    Columns,
    Column(usize, bool),
    Close,
}

impl Files {
    pub(super) fn ask(&mut self, act: Act, p: usize, items: Vec<PathBuf>) {
        let here = self.cols[p].at.folder().map(Path::to_path_buf);
        self.dialog = match act {
            Act::OpenWith => items.first().map(|f| Dialog::OpenWith(self.open_with(f))),
            Act::Properties => {
                let items = if items.is_empty() { here.into_iter().collect() } else { items };
                (!items.is_empty()).then(|| Dialog::Properties(Box::new(self.props(items))))
            }
            Act::SendTo => {
                let other = self.cols[1 - p].at.folder().map(Path::to_path_buf);
                Some(Dialog::SendTo { places: send_places(&self.home, other.as_deref(), &self.pins_named()), items, find: String::new() })
            }
            Act::Colour => {
                let folders = if items.is_empty() { here.into_iter().collect() } else { items };
                let now = folders.first().and_then(|f| self.kept.colours.get(f).cloned());
                (!folders.is_empty()).then_some(Dialog::Colour { folders, now })
            }
            Act::ShareOnNetwork => {
                let folder = items.into_iter().next().or(here);
                folder.and_then(|f| self.share_form(&f)).map(|f| Dialog::Sharing(Box::new(f)))
            }
            _ => None,
        };
    }

    pub(super) fn open_with(&self, file: &Path) -> OpenWith {
        let kind = self.mime.of_path(file);
        let all = apps::entries();
        let default = apps::chosen_of(&all, &kind).map(|e| e.id);
        let mut suggested = apps::openers_of(&all, &kind);
        if let Some(i) = suggested.iter().position(|e| Some(&e.id) == default.as_ref()) {
            let d = suggested.remove(i);
            suggested.insert(0, d);
        }
        let mut every: Vec<Entry> = all.into_iter().filter(|e| !e.exec.is_empty() && !e.terminal).collect();
        every.sort_by_key(|e| e.name.to_lowercase());
        let none = suggested.is_empty();
        OpenWith { file: file.to_path_buf(), kind, chosen: (!none).then_some(0), suggested, every, all: none, default, always: false, find: String::new() }
    }

    fn sums_start(&mut self) {
        let Some(Dialog::Properties(p)) = &mut self.dialog else { return };
        if p.sums != Sums::Unasked && p.sums != Sums::Failed {
            return;
        }
        let (path, id, stop) = (p.items[0].clone(), p.id, p.stop.clone());
        p.sums = Sums::Working(0.0);
        match &self.post {
            Some(post) => {
                let post = post.clone();
                let _ = crate::common::gentle::spawn("folders-checksums", move || {
                    let mut shown = 0.0;
                    let got = super::checksum::work_out(&path, &stop, &mut |far| {
                        if far - shown >= 0.01 {
                            shown = far;
                            post.send(Msg::Ask(AskMsg::SumsFar(id, far)));
                        }
                    });
                    post.send(Msg::Ask(AskMsg::SumsGot(id, got.ok())));
                });
            }
            None => p.sums = super::checksum::work_out(&path, &stop, &mut |_| {}).map_or(Sums::Failed, Sums::Done),
        }
    }

    fn props(&mut self, items: Vec<PathBuf>) -> Props {
        let details = match items.as_slice() {
            [one] if !self.earlier_holds(one) && super::reach::answers(one) => super::details::Details::load(one),
            _ => None,
        };
        let mut rows = describe(&items, &self.mime);
        if details.is_some() {
            rows.retain(|(k, _)| *k != t!("folders-dialogs-row-changed"));
        }
        self.next_job += 1;
        let id = self.next_job;
        let stop = Arc::new(AtomicBool::new(false));
        let sums = match items.as_slice() {
            [one] if std::fs::symlink_metadata(one).is_ok_and(|m| m.is_file()) => Sums::Unasked,
            _ => Sums::None,
        };
        let share = match items.as_slice() {
            [one] => self.share_form(one),
            _ => None,
        };
        let mut props = Props { items: items.clone(), rows, tally: None, done: false, id, stop: stop.clone(), details, sums,
                                check: String::new(), share, tall: (self.room.1 - PROPS_ROOM).max(PROPS_LEAST), page: 0 };
        match &self.post {
            Some(post) => {
                let post = post.clone();
                let _ = crate::common::gentle::spawn("folders-measure", move || {
                    let tally = add_up(&items, &stop, &mut |t| {
                        post.send(Msg::Ask(AskMsg::Measured(id, *t, false)));
                    });
                    post.send(Msg::Ask(AskMsg::Measured(id, tally, true)));
                });
            }
            None => {
                props.tally = Some(add_up(&items, &stop, &mut |_| {}));
                props.done = true;
            }
        }
        props
    }

    fn open_with_chosen(&mut self) {
        let Some(Dialog::OpenWith(o)) = self.dialog.take() else { return };
        let Some(app) = o.chosen.and_then(|i| o.shown().get(i).cloned()) else {
            self.dialog = Some(Dialog::OpenWith(o));
            self.say(t!("folders-dialogs-choose-app"), false);
            return;
        };
        if o.always && apps::choose_opener(&app.id, &[&o.kind]).is_err() {
            self.say_help(t!("folders-dialogs-usual-failed", name = &app.name), super::help::NOT_KEPT);
        }
        if !apps::launch_boxed(&app, &[o.file.to_string_lossy().into_owned()]) {
            self.say_help(t!("folders-dialogs-start-failed", name = &app.name), super::help::NOT_STARTED);
        } else {
            self.used(&o.file);
        }
    }

    pub(super) fn ask_msg(&mut self, m: AskMsg) {
        match m {
            AskMsg::Close if matches!(self.dialog, Some(Dialog::Dropped(_))) => self.outside_answer(false),
            AskMsg::Close => self.dialog = None,
            AskMsg::Choose(_) | AskMsg::Every(_) | AskMsg::Find(_) | AskMsg::Always(_) | AskMsg::OpenIt | AskMsg::OpenWithRow(_) => self.open_with_msg(m),
            AskMsg::FindPlace(words) => {
                if let Some(Dialog::SendTo { find, .. }) = &mut self.dialog {
                    *find = words;
                }
            }
            AskMsg::Send(i) => {
                let Some(Dialog::SendTo { items, places, .. }) = self.dialog.take() else { return };
                if let Some((_, to)) = places.into_iter().nth(i) {
                    self.send_to(items, to, Kind::Copy);
                }
            }
            AskMsg::Columns => self.dialog = Some(Dialog::Columns(self.kept.columns())),
            AskMsg::Column(i, on) => {
                let Some(col) = Col::ALL.get(i).copied() else { return };
                if self.kept.column(col, on).is_err() {
                    self.say_help(t!("folders-dialogs-columns-failed"), super::help::NOT_KEPT);
                }
                if let Some(Dialog::Columns(now)) = &mut self.dialog {
                    *now = self.kept.columns();
                }
            }
            AskMsg::Colour(c) => {
                let Some(Dialog::Colour { folders, .. }) = self.dialog.take() else { return };
                let name = c.and_then(|i| ACCENTS.get(i)).map(|(_, n)| *n);
                let failed = folders.iter().any(|f| self.kept.colour(f, name).is_err());
                if failed {
                    self.say_help(t!("folders-dialogs-colour-failed"), super::help::NOT_KEPT);
                }
            }
            AskMsg::Page(_) | AskMsg::Detail(_) | AskMsg::Sums | AskMsg::SumsFar(..) | AskMsg::SumsGot(..) | AskMsg::SumsCheck(_) | AskMsg::Measured(..) => self.props_msg(m),
        }
    }

    fn open_with_msg(&mut self, m: AskMsg) {
        match m {
            AskMsg::Choose(i) => {
                if let Some(Dialog::OpenWith(o)) = &mut self.dialog {
                    o.chosen = Some(i);
                }
            }
            AskMsg::Every(on) => {
                if let Some(Dialog::OpenWith(o)) = &mut self.dialog {
                    let id = o.chosen.and_then(|i| o.shown().get(i).map(|e| e.id.clone()));
                    o.all = on || o.suggested.is_empty();
                    o.chosen = id.and_then(|id| o.shown().iter().position(|e| e.id == id));
                }
            }
            AskMsg::Find(words) => {
                if let Some(Dialog::OpenWith(o)) = &mut self.dialog {
                    let id = o.chosen.and_then(|i| o.shown().get(i).map(|e| e.id.clone()));
                    o.find = words;
                    let now = o.shown();
                    o.chosen = id.and_then(|id| now.iter().position(|e| e.id == id)).or((!now.is_empty()).then_some(0));
                }
            }
            AskMsg::Always(on) => {
                if let Some(Dialog::OpenWith(o)) = &mut self.dialog {
                    o.always = on;
                }
            }
            AskMsg::OpenIt => self.open_with_chosen(),
            AskMsg::OpenWithRow(i) => {
                if let Some(Dialog::OpenWith(o)) = &mut self.dialog {
                    o.chosen = Some(i);
                }
                self.open_with_chosen();
            }
            _ => {}
        }
    }

    fn props_msg(&mut self, m: AskMsg) {
        match m {
            AskMsg::Page(i) => {
                if let Some(Dialog::Properties(p)) = &mut self.dialog {
                    p.page = i;
                }
            }
            AskMsg::Detail(m) => {
                let Some(Dialog::Properties(p)) = &mut self.dialog else { return };
                let Some(d) = &mut p.details else { return };
                if d.update(m) {
                    self.reload_all();
                }
            }
            AskMsg::Sums => self.sums_start(),
            AskMsg::SumsFar(id, far) => {
                if let Some(Dialog::Properties(p)) = &mut self.dialog {
                    if p.id == id && matches!(p.sums, Sums::Working(_)) {
                        p.sums = Sums::Working(far);
                    }
                }
            }
            AskMsg::SumsGot(id, got) => {
                if let Some(Dialog::Properties(p)) = &mut self.dialog {
                    if p.id == id {
                        p.sums = got.map_or(Sums::Failed, Sums::Done);
                    }
                }
            }
            AskMsg::SumsCheck(text) => {
                if let Some(Dialog::Properties(p)) = &mut self.dialog {
                    p.check = text;
                }
            }
            AskMsg::Measured(id, tally, done) => {
                if let Some(Dialog::Properties(p)) = &mut self.dialog {
                    if p.id == id && !p.done {
                        p.tally = Some(tally);
                        p.done = done;
                    }
                }
            }
            _ => {}
        }
    }
}

fn swatch(canvas: &mut PixmapMut, r: Rect, ink: Colour) {
    let spot = Rect { x: r.x + (r.w - SWATCH) / 2.0, y: r.y + (r.h - SWATCH) / 2.0, w: SWATCH, h: SWATCH };
    if let Some(path) = crate::toolkit::ui::rounded(spot, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Box, SWATCH, 1.0)) {
        canvas.fill_path(&path, &crate::toolkit::colour::paint(ink), FillRule::Winding, Transform::identity(), None);
    }
}

fn sums_view(p: &Props) -> Vec<Node<Msg>> {
    let line = |key: &str, value: Node<Msg>| row(vec![note(key.to_string()).width(Length::Fixed(KEY_W)), value]).gap(8.0).width(Length::Fill);
    match &p.sums {
        Sums::None => Vec::new(),
        Sums::Unasked => vec![line(t!("folders-dialogs-checksums"), button(icons::stand_in::CHECKSUM, t!("folders-dialogs-sums-work-out")).on_press(Msg::Ask(AskMsg::Sums)))],
        Sums::Working(far) => vec![line(t!("folders-dialogs-checksums"), row(vec![note(t!("folders-dialogs-sums-working")), standard_ring(Some(*far), RING).width(Length::Shrink)]).gap(12.0).centred())],
        Sums::Failed => vec![line(t!("folders-dialogs-checksums"), row(vec![label(t!("folders-dialogs-sums-failed")).tone(Tone::Problem), crate::common::more::button(Msg::Help(super::help::FILE_UNREAD))]).gap(8.0))],
        Sums::Done(sums) => {
            let mut out: Vec<Node<Msg>> = sums.iter().map(|(kind, sum)| line(kind, label(sum.clone()).width(Length::Fill))).collect();
            let found = if p.check.trim().is_empty() { None } else { Some(super::checksum::matching(sums, &p.check)) };
            let mut check = vec![entry(p.check.clone(), t!("folders-dialogs-sums-paste"), |t| Msg::Ask(AskMsg::SumsCheck(t))).width(Length::Fill)];
            match found {
                Some(Some(kind)) => check.push(label(t!("folders-dialogs-sums-matches", kind = kind))),
                Some(None) => check.push(label(t!("folders-dialogs-sums-no-match")).tone(Tone::Problem)),
                None => {}
            }
            out.push(line(t!("folders-dialogs-sums-check"), row(check).gap(8.0).width(Length::Fill)));
            out
        }
    }
}

pub(super) fn dialog_list(name: &str, list: Node<Msg>, rows: usize) -> Node<Msg> {
    scroll(name, list).height(Length::Fixed((rows as f32 * crate::toolkit::theme::sheet().row).min(LIST_H)))
}

fn count_words(t: &Tally, items: &[PathBuf]) -> String {
    match items {
        [one] if super::reach::is_dir(one) => items_words(t.items().saturating_sub(1)),
        [_] => String::new(),
        _ => items_words(t.items()),
    }
}

fn add_up(items: &[PathBuf], stop: &AtomicBool, tell: &mut dyn FnMut(&Tally)) -> Tally {
    let mut sum = Tally::default();
    for item in items {
        let before = sum;
        let got = sizes::measure(item, stop, &mut |t| tell(&plus(before, t)));
        match got {
            Ok(t) => sum = plus(before, &t),
            Err(_) => sum.unread += 1,
        }
        if stop.load(Ordering::Relaxed) {
            break;
        }
        if item.is_dir() { sum.folders += 1 } else if items.len() > 1 { sum.files = sum.files.max(1) }
    }
    sum
}

fn plus(a: Tally, b: &Tally) -> Tally {
    Tally { bytes: a.bytes + b.bytes, files: a.files + b.files, folders: a.folders + b.folders, unread: a.unread + b.unread }
}

fn describe(items: &[PathBuf], mime: &super::kinds::Mime) -> Vec<(&'static str, String)> {
    let mut rows = Vec::new();
    let [one] = items else {
        rows.push((t!("folders-dialogs-row-name"), items_words(items.len() as u64)));
        if let Some(at) = items.first().and_then(|i| i.parent()) {
            rows.push((t!("folders-dialogs-row-where"), home_path(at, &crate::common::files::home())));
        }
        return rows;
    };
    rows.push((t!("folders-dialogs-row-name"), work::name_of(one)));
    if !super::reach::answers(one) {
        rows.push((t!("folders-dialogs-row-kind"), super::reach::not_answering().to_string()));
        return rows;
    }
    let link = std::fs::read_link(one).ok();
    let meta = std::fs::metadata(one).or_else(|_| std::fs::symlink_metadata(one));
    if !matches!(&meta, Ok(m) if m.is_dir()) {
        let kind = mime.of(&work::name_of(one)).map_or_else(|| t!("folders-dialogs-kind-unknown").into(), |k| kind_words(&k));
        rows.push((t!("folders-dialogs-row-kind"), kind));
    }
    if let Some(to) = link {
        rows.push((t!("folders-dialogs-row-link-to"), to.to_string_lossy().into_owned()));
    }
    if let Some(at) = one.parent() {
        rows.push((t!("folders-dialogs-row-where"), home_path(at, &crate::common::files::home())));
    }
    if let Ok(m) = meta {
        rows.push((t!("folders-dialogs-row-changed"), when_text(m.mtime())));
        rows.push((t!("folders-dialogs-row-owner"), super::columns::owner_name(m.uid())));
        rows.push((t!("folders-dialogs-row-rights"), rights(m.permissions().mode(), m.uid())));
    }
    rows
}

fn kind_words(kind: &str) -> String {
    match super::kinds::kind_name(kind) {
        words if words != kind => t!("folders-dialogs-kind-with-name", words = words, kind = kind),
        _ => kind.to_string(),
    }
}

fn rights(mode: u32, owner: u32) -> String {
    let letters = super::columns::letters(mode);
    let me = unsafe { libc::getuid() };
    let shift = if me == owner { 6 } else { 0 };
    let (read, write) = ((mode >> shift) & 4 != 0, (mode >> shift) & 2 != 0);
    match (read, write) {
        (true, true) => t!("folders-dialogs-rights-open-change", letters = letters),
        (true, false) => t!("folders-dialogs-rights-open-only", letters = letters),
        (false, true) => t!("folders-dialogs-rights-change-only", letters = letters),
        (false, false) => t!("folders-dialogs-rights-none", letters = letters),
    }
}

pub fn send_places(home: &Path, other: Option<&Path>, pinned: &[(PathBuf, String)]) -> Vec<(String, PathBuf)> {
    let mut out: Vec<(String, PathBuf)> = Vec::new();
    let mut add = |name: String, path: PathBuf| {
        if super::reach::is_dir(&path) && !out.iter().any(|(_, p)| *p == path) {
            out.push((name, path));
        }
    };
    if let Some(o) = other {
        add(t!("folders-dialogs-send-other", name = if o == home { t!("folders-dialogs-home").to_string() } else { work::name_of(o) }), o.to_path_buf());
    }
    add(t!("folders-dialogs-home").into(), home.to_path_buf());
    for (p, name) in pinned {
        add(name.clone(), p.clone());
    }
    let user = home.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    for base in [PathBuf::from("/media").join(&user), PathBuf::from("/run/media").join(&user)] {
        let Ok(rd) = std::fs::read_dir(&base) else { continue };
        let mut disks: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
        disks.sort();
        for d in disks {
            add(work::name_of(&d), d);
        }
    }
    out
}

pub(super) fn home_path(path: &Path, home: &Path) -> String {
    match path.strip_prefix(home) {
        Ok(rest) if rest.as_os_str().is_empty() => "~".to_string(),
        Ok(rest) => format!("~/{}", rest.to_string_lossy()),
        Err(_) => path.to_string_lossy().into_owned(),
    }
}

pub(super) fn short_path(path: &Path, home: &Path) -> String {
    let text = home_path(path, home);
    let letters: Vec<char> = text.chars().collect();
    if letters.len() <= SHORT_PATH { text } else { format!("\u{2026}{}", letters[letters.len() - SHORT_PATH + 1..].iter().collect::<String>()) }
}

const SHORT_PATH: usize = 32;

pub(super) fn head(icon: &str, name: impl Into<String>) -> Node<Msg> {
    head_to(icon, name, t!("folders-dialogs-close"), Msg::Ask(AskMsg::Close))
}

pub(super) fn head_to(icon: &str, name: impl Into<String>, tip: &str, close: Msg) -> Node<Msg> {
    row(vec![
        icon_circle(icon),
        heading(name).cut().width(Length::Fill),
        button("window-close-symbolic", tip.to_string()).on_press(close).first(),
    ]).gap(12.0).width(Length::Fill).centred()
}

pub(super) fn named(name: &str, sub: &str) -> Vec<Node<Msg>> {
    let mut words = vec![form_label(name.to_string())];
    if !sub.is_empty() {
        words.push(note(sub.to_string()));
    }
    words
}

pub(super) fn toggle(name: &str, sub: &str, on: bool, tip: &str, change: impl Fn(bool) -> Msg + 'static) -> Node<Msg> {
    row(vec![
        column(named(name, sub)).gap(4.0).width(Length::Fill),
        switch(on, tip.to_string(), change),
    ]).gap(12.0).width(Length::Fill).centred()
}

pub(super) fn foot(mut left: Vec<Node<Msg>>, main: Node<Msg>) -> Node<Msg> {
    left.push(space().width(Length::Fill));
    left.push(main);
    row(left).gap(8.0).width(Length::Fill).centred()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    General,
    Details,
    Sharing,
}

fn sections(p: &Props) -> Vec<(&'static str, Section)> {
    let mut out = vec![(t!("folders-dialogs-page-general"), Section::General)];
    if p.details.as_ref().is_some_and(|d| d.has_media()) {
        out.push((t!("folders-dialogs-page-details"), Section::Details));
    }
    if p.share.is_some() {
        out.push((t!("folders-dialogs-page-sharing"), Section::Sharing));
    }
    out
}

pub fn view(d: &Dialog, turn: u32) -> Node<Msg> {
    let body = match d {
        Dialog::Tool(t) => return super::tools::view(t, turn),
        Dialog::Connect(c) => return super::far::view(c, turn),
        Dialog::Share(a) => return super::shares::view(a, turn),
        Dialog::Sharing(f) => super::sharing::card_view(f),
        Dialog::Preview(pv) => return super::preview::view(pv),
        Dialog::Unlock(u) => return super::locked::view(u, turn),
        Dialog::Earlier(c) => return super::earlier::view(c),
        Dialog::Settings(c) => return super::settings::view(c),
        Dialog::Dropped(a) => return super::outside::view(a),
        Dialog::OpenWith(o) => open_with_view(o, turn),
        Dialog::Properties(p) => properties_view(p),
        Dialog::SendTo { items, places, find } => send_to_view(items, places, find, turn),
        Dialog::Columns(now) => columns_view(now),
        Dialog::Colour { folders, now } => colour_view(folders, now.as_deref()),
    };
    card(body.gap(12.0).width(Length::Fill)).width(Length::Fixed(WIDTH)).on_escape(Msg::Ask(AskMsg::Close))
}

fn open_with_view(o: &OpenWith, turn: u32) -> Node<Msg> {
    let name = work::name_of(&o.file);
    let shown = o.shown();
    let n = shown.len();
    let rows = shown.iter().map(|e| {
        let mut it = item(e.name.clone()).icon(e.icon.clone().unwrap_or_else(|| crate::common::apps::generic_icon_for(&e.id, &e.name)));
        if Some(&e.id) == o.default.as_ref() {
            it = it.detail(t!("folders-dialogs-opens-now"));
        }
        it
    }).collect();
    let apps = if n == 0 {
        note(if o.find.trim().is_empty() { t!("folders-dialogs-no-apps") } else { t!("folders-dialogs-no-match") })
    } else {
        dialog_list("open-with", list(rows, o.chosen, |i| Msg::Ask(AskMsg::Choose(i))).on_open(|i| Msg::Ask(AskMsg::OpenWithRow(i))).grab(turn), n)
    };
    let mut top = vec![head("document-open-symbolic", t!("folders-dialogs-open-with-title", name = &name))];
    if o.whole().len() > FIND_FROM {
        top.push(entry(o.find.clone(), t!("folders-dialogs-find-app"), |w| Msg::Ask(AskMsg::Find(w))));
    }
    top.push(apps);
    top.push(toggle(t!("folders-dialogs-always-use"), &t!("folders-dialogs-always-sub", kind = &o.kind), o.always, t!("folders-dialogs-always-tip"),
                    |on| Msg::Ask(AskMsg::Always(on))));
    top.push(foot(vec![
        button("view-app-grid-symbolic", if o.all { t!("folders-dialogs-show-suggested") } else { t!("folders-dialogs-show-every") })
            .on(o.all).on_press(Msg::Ask(AskMsg::Every(!o.all))).off(o.suggested.is_empty()),
    ], main_action("document-open-symbolic", t!("folders-dialogs-open")).on_press(Msg::Ask(AskMsg::OpenIt)).off(o.chosen.is_none())));
    column(top)
}

fn properties_view(p: &Props) -> Node<Msg> {
    let parts = sections(p);
    let at = p.page.min(parts.len() - 1);
    let lines = match parts[at].1 {
        Section::General => general_page(p),
        Section::Details => p.details.iter().flat_map(|d| super::details::view(d, true)).collect(),
        Section::Sharing => p.share.iter().flat_map(super::sharing::view).collect(),
    };
    let n = lines.len();
    let middle = column(lines).gap(12.0).width(Length::Fill);
    let middle = if n as f32 * PROPS_LINE > p.tall { scroll(format!("properties:{at}"), middle).height(Length::Fixed(p.tall)) } else { middle };
    let mut all = vec![head("document-properties-symbolic", t!("folders-dialogs-properties"))];
    if parts.len() > 1 {
        let names = parts.iter().map(|(name, _)| name.to_string()).collect();
        all.push(choice(names, at, t!("folders-dialogs-properties-pages"), |i| Msg::Ask(AskMsg::Page(i))));
    }
    all.push(middle);
    column(all)
}

fn general_page(p: &Props) -> Vec<Node<Msg>> {
    let mut lines: Vec<Node<Msg>> = Vec::new();
    let mut rows = p.rows.clone();
    let size = match &p.tally {
        Some(t) => {
            let items = count_words(t, &p.items);
            let yes = |on: bool| if on { "yes" } else { "no" };
            let (counted, unread, adding) = (yes(!items.is_empty()), yes(t.unread > 0), yes(!p.done));
            t!("folders-dialogs-size", size = size_words(t.bytes), counted = counted, items = items, unread = unread, adding = adding)
        }
        None => t!("folders-dialogs-size-adding").into(),
    };
    rows.insert(rows.len().min(2), (t!("folders-dialogs-row-size"), size));
    for (key, value) in rows {
        lines.push(row(vec![note(key).width(Length::Fixed(KEY_W)), label(value).width(Length::Fill)]).gap(8.0).width(Length::Fill));
    }
    if let Some(d) = &p.details {
        lines.extend(super::details::view(d, false));
    }
    lines.extend(sums_view(p));
    lines
}

fn send_to_view(items: &[PathBuf], places: &[(String, PathBuf)], find: &str, turn: u32) -> Node<Msg> {
    let words = find.trim().to_lowercase();
    let kept: Vec<(usize, &(String, PathBuf))> = places.iter().enumerate()
        .filter(|(_, (name, _))| words.is_empty() || name.to_lowercase().contains(&words)).collect();
    let what = t!("folders-dialogs-send-title", n = items.len(), name = items.first().map(|i| work::name_of(i)).unwrap_or_default());
    let rows = kept.iter().map(|(_, (name, path))| {
        let disk = path.starts_with("/media") || path.starts_with("/run/media");
        let row = if disk { item(name.clone()).icon("drive-removable-media-symbolic") } else { item(name.clone()).icon("folder-symbolic").folder() };
        row.detail(short_path(path, &crate::common::files::home()))
    }).collect();
    let mut top = vec![head("send-to-symbolic", what), note(t!("folders-dialogs-send-note"))];
    if places.len() > FIND_FROM {
        top.push(entry(find.to_string(), t!("folders-dialogs-find-place"), |w| Msg::Ask(AskMsg::FindPlace(w))));
    }
    if kept.is_empty() {
        top.push(note(t!("folders-dialogs-no-match-place")));
    } else {
        let at: Vec<usize> = kept.iter().map(|(i, _)| *i).collect();
        top.push(dialog_list("send-to", list(rows, None, move |i| Msg::Ask(AskMsg::Send(at[i]))).grab(turn), kept.len()));
    }
    column(top)
}

fn columns_view(now: &[Col]) -> Node<Msg> {
    let mut lines = vec![head(super::icons::stand_in::COLUMNS, t!("folders-dialogs-columns-title")),
                         note(t!("folders-dialogs-columns-note"))];
    let group = |name: &'static str, media: bool| {
        let mut g = vec![form_label(name)];
        g.extend(Col::ALL.iter().enumerate().filter(|(_, c)| c.media() == media).map(|(i, c)| {
            row(vec![label(c.word()).width(Length::Fill), switch(now.contains(c), t!("folders-dialogs-columns-show", name = c.word()), move |on| Msg::Ask(AskMsg::Column(i, on)))])
                .width(Length::Fill)
        }));
        column(g).width(Length::Fill).height(Length::Fill)
    };
    lines.push(row(vec![group(t!("folders-dialogs-columns-files"), false), group(t!("folders-dialogs-columns-media"), true)]).gap(16.0).width(Length::Fill));
    column(lines)
}

fn colour_view(folders: &[PathBuf], now: Option<&str>) -> Node<Msg> {
    let what = t!("folders-dialogs-colour-title", n = folders.len(), name = folders.first().map(|i| work::name_of(i)).unwrap_or_default());
    let mut swatches: Vec<Node<Msg>> = ACCENTS.iter().enumerate().map(|(i, (hex, name))| {
        let ink = Colour::parse(hex).unwrap_or_else(|| Palette::Grey.swatch());
        button(super::icons::stand_in::FOLDER_COLOUR, *name)
            .on(now.is_some_and(|n| n.eq_ignore_ascii_case(name)))
            .on_press(Msg::Ask(AskMsg::Colour(Some(i))))
            .face(move |canvas, r, _, _| swatch(canvas, r, ink))
    }).collect();
    let mut rows: Vec<Node<Msg>> = Vec::new();
    while !swatches.is_empty() {
        let rest = swatches.split_off(swatches.len().min(SWATCH_ROW));
        let mut one = vec![space()];
        one.extend(std::mem::replace(&mut swatches, rest));
        one.push(space());
        rows.push(row(one).gap(8.0).width(Length::Fill));
    }
    let none = row(vec![
        button("edit-clear-symbolic", t!("folders-dialogs-colour-none")).on(now.is_none()).on_press(Msg::Ask(AskMsg::Colour(None))),
        note(t!("folders-dialogs-colour-none")),
    ]).gap(12.0).centred();
    column(vec![head(super::icons::stand_in::FOLDER_COLOUR, what), column(rows).gap(8.0).width(Length::Fill), none])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_control_with_no_line_under_its_name_draws_none() {
        assert_eq!(named("Read only", "").len(), 1);
        assert_eq!(named("Read only", "Apps can open it but not change it.").len(), 2);
    }
    use crate::files::places::Where;
    use crate::toolkit::ui::App;

    fn open_with_dialog(file: &Path) -> Dialog {
        let app = |id: &str| Entry { id: id.into(), name: id.into(), icon: None, exec: id.into(), kinds: Vec::new(), path: PathBuf::from("/nonexistent").join(id), flatpak: None, appimage: None, terminal: false };
        Dialog::OpenWith(OpenWith { file: file.to_path_buf(), kind: "text/plain".into(), suggested: vec![app("one.desktop"), app("two.desktop")],
                                    every: Vec::new(), all: false, chosen: Some(0), default: None, always: false, find: String::new() })
    }

    #[test]
    fn a_double_click_on_an_app_opens_the_file_with_it_and_the_tick_still_does() {
        let mut f = Files::new(Vec::new());
        f.dialog = Some(open_with_dialog(Path::new("/tmp/a.txt")));
        f.update(Msg::Ask(AskMsg::OpenWithRow(1)));
        assert!(f.dialog.is_none(), "the window shuts as the app starts");
        assert!(f.bad && f.said.contains("two.desktop"), "the app that was double clicked was the one started: {}", f.said);
        f.said.clear();
        f.dialog = Some(open_with_dialog(Path::new("/tmp/a.txt")));
        f.update(Msg::Ask(AskMsg::Choose(1)));
        assert!(f.dialog.is_some());
        f.update(Msg::Ask(AskMsg::OpenIt));
        assert!(f.dialog.is_none() && f.said.contains("two.desktop"), "{}", f.said);
        f.said.clear();
        f.dialog = Some(open_with_dialog(Path::new("/tmp/a.txt")));
        if let Some(Dialog::OpenWith(o)) = &mut f.dialog {
            o.chosen = None;
        }
        f.update(Msg::Ask(AskMsg::OpenIt));
        assert!(f.dialog.is_some() && f.said == "Choose an app first.", "{}", f.said);
    }

    #[test]
    fn checksums_are_worked_out_for_one_file_and_checked() {
        let dir = std::env::temp_dir().join(format!("trindle-files-sums-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("abc.txt"), "abc").unwrap();
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        let sums = |f: &Files| match &f.dialog { Some(Dialog::Properties(p)) => p.sums.clone(), _ => Sums::None };
        f.ask(Act::Properties, 0, vec![dir.clone()]);
        assert_eq!(sums(&f), Sums::None);
        f.ask(Act::Properties, 0, vec![dir.join("abc.txt")]);
        assert_eq!(sums(&f), Sums::Unasked);
        f.update(Msg::Ask(AskMsg::Sums));
        assert!(matches!(sums(&f), Sums::Done(s) if s[2].1.starts_with("ba7816bf")));
        f.update(Msg::Ask(AskMsg::SumsCheck("900150983cd24fb0d6963f7d28e17f72".into())));
        let Some(Dialog::Properties(p)) = &f.dialog else { panic!() };
        let Sums::Done(s) = &p.sums else { panic!() };
        assert_eq!(super::super::checksum::matching(s, &p.check), Some("MD5"));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_colour_follows_a_rename_and_its_undoing() {
        let dir = std::env::temp_dir().join(format!("trindle-files-colour-rename-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("a.txt")).unwrap();
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        f.kept.colour(&dir.join("a.txt"), Some("Teal")).unwrap();
        let i = f.cols[0].tabs[0].entries.iter().position(|e| e.name == "a.txt").unwrap();
        f.update(Msg::Ask(AskMsg::Close));
        f.update(Msg::Choose(0, i));
        f.update(Msg::Pick(0, vec![i]));
        f.update(Msg::Rename);
        f.update(Msg::RenameText("b.txt".into()));
        f.update(Msg::RenameDone);
        assert!(dir.join("b.txt").exists());
        assert!(f.kept.colours.contains_key(&dir.join("b.txt")) && !f.kept.colours.contains_key(&dir.join("a.txt")));
        f.update(Msg::Undo);
        assert!(f.kept.colours.contains_key(&dir.join("a.txt")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_folder_is_described_by_what_is_inside_it() {
        let dir = std::env::temp_dir().join(format!("trindle-files-count-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("empty")).unwrap();
        std::fs::create_dir_all(dir.join("one")).unwrap();
        std::fs::write(dir.join("one/a.txt"), "a").unwrap();
        let stop = AtomicBool::new(false);
        let words = |items: Vec<PathBuf>| count_words(&add_up(&items, &stop, &mut |_| {}), &items);
        assert_eq!(words(vec![dir.join("empty")]), "0 items");
        assert_eq!(words(vec![dir.join("one")]), "1 item");
        assert_eq!(words(vec![dir.join("one/a.txt")]), "");
        assert_eq!(words(vec![dir.join("empty"), dir.join("one")]), "3 items");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_long_open_with_list_narrows_as_you_type() {
        let app = |name: &str| Entry { id: format!("{name}.desktop"), name: name.into(), icon: None, exec: name.into(), kinds: Vec::new(), path: PathBuf::from("/nonexistent").join(name), flatpak: None, appimage: None, terminal: false };
        let with = |n: usize| {
            let names = ["Alpha", "Bravo", "Cedar", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliet", "Kilo", "Lima"];
            let every: Vec<Entry> = names[..n].iter().map(|n| app(n)).collect();
            Dialog::OpenWith(OpenWith { file: PathBuf::from("/tmp/a.txt"), kind: "text/plain".into(), suggested: every.clone(), every,
                                        all: false, chosen: Some(0), default: None, always: false, find: String::new() })
        };
        let box_is_there = |d: &Dialog| view(d, 1).said().join("\n").contains(crate::toolkit::t!("folders-dialogs-find-app"));
        assert!(!box_is_there(&with(FIND_FROM)), "a list that fits has a find box");
        assert!(box_is_there(&with(FIND_FROM + 1)), "a list that is too long has none");
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.dialog = Some(with(12));
        let shown = |f: &Files| match &f.dialog { Some(Dialog::OpenWith(o)) => o.shown().iter().map(|e| e.name.clone()).collect::<Vec<_>>(), _ => panic!() };
        assert_eq!(shown(&f).len(), 12);
        f.update(Msg::Ask(AskMsg::Find("ho".into())));
        assert_eq!(shown(&f), ["Echo", "Hotel"].map(String::from), "the list was not narrowed to the names that match");
        f.update(Msg::Ask(AskMsg::Find("HOT".into())));
        assert_eq!(shown(&f), ["Hotel".to_string()], "the words are matched whatever their case");
        f.update(Msg::Ask(AskMsg::Find("zzz".into())));
        assert!(shown(&f).is_empty());
        f.update(Msg::Ask(AskMsg::Find(String::new())));
        assert_eq!(shown(&f).len(), 12, "clearing the box did not bring the whole list back");
    }

    #[test]
    fn a_long_send_to_list_scrolls() {
        let places = (0..40).map(|i| (format!("Folder {i}"), PathBuf::from(format!("/tmp/{i}")))).collect();
        let d = Dialog::SendTo { items: vec![PathBuf::from("/tmp/a.txt")], places, find: String::new() };
        assert!(view(&d, 1).lists_scroll());
        let places = (0..40).map(|i| (format!("Folder {i}"), PathBuf::from(format!("/tmp/{i}")))).collect();
        let t = super::super::tools::Tool::Unpack { archive: PathBuf::from("/tmp/a.zip"), places };
        assert!(super::super::tools::view(&t, 1).lists_scroll());
    }

    #[test]
    fn paths_are_kept_short() {
        let home = Path::new("/home/john");
        assert_eq!(short_path(home, home), "~");
        assert_eq!(short_path(Path::new("/home/john/Work"), home), "~/Work");
        let long = short_path(Path::new("/data/a/very/long/folder/name/that/goes/on/and/on"), home);
        assert_eq!(long.chars().count(), SHORT_PATH);
        assert!(long.starts_with('\u{2026}') && long.ends_with("and/on"));
    }

    #[test]
    fn rights_read_in_words() {
        let me = unsafe { libc::getuid() };
        assert_eq!(rights(0o644, me), "rw-r--r--, you can open and change it");
        assert_eq!(rights(0o444, me), "r--r--r--, you can open it but not change it");
        assert_eq!(rights(0o700, me + 1), "rwx------, you can't open it");
    }

    #[test]
    fn send_to_offers_each_place_once() {
        let dir = std::env::temp_dir().join(format!("trindle-files-send-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("a")).unwrap();
        let got = send_places(&dir, Some(&dir.join("a")), &[(dir.join("a"), "a".to_string()), (dir.join("gone"), "gone".to_string())]);
        assert_eq!(got.iter().map(|(_, p)| p.clone()).collect::<Vec<_>>(), [dir.join("a"), dir.clone()]);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn many_items_are_added_up() {
        let dir = std::env::temp_dir().join(format!("trindle-files-addup-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("in")).unwrap();
        std::fs::write(dir.join("in/x"), b"12345").unwrap();
        std::fs::write(dir.join("y"), b"123").unwrap();
        let t = add_up(&[dir.join("in"), dir.join("y")], &AtomicBool::new(false), &mut |_| {});
        assert_eq!((t.bytes, t.folders), (8, 1));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
