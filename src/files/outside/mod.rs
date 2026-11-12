//! Files and folders dragged between Folders and other programs.

use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::toolkit::dnd::{self, Given, Way};
use crate::toolkit::t;
use crate::toolkit::ui::{DropEnd, Lifted};
use crate::toolkit::widget::{card, column, label, main_action, note, word_button, Held, Length, Node, Part};

use super::act::{self, Land};
use super::app::{Files, Msg};
use super::dialogs::{foot, head_to, short_path, Dialog};
use super::places::Where;
use super::proof::{not_as_picked, not_looked_through, Counts, Handed, Left, Trouble};
use super::work::{self, Sure, Which};

const QUESTION_W: f32 = 440.0;
const NAMED_MOST: usize = 6;
const LOOK_MOST: usize = 4096;
const LOOK_TIME: Duration = Duration::from_millis(200);
pub const CHECK_WAIT: Duration = Duration::from_secs(5);
const HELD_MOST: usize = 256;
const DRIVE_ROOTS: [&str; 3] = ["/media", "/run/media", "/mnt"];
const SYSTEM_TOPS: [&str; 20] = ["boot", "efi", "usr", "var", "etc", "opt", "snap", "nix", "proc", "sys", "dev", "run", "tmp", "root", "srv", "lib", "lib64",
                                 "bin", "sbin", "lost+found"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wary {
    Hidden,
    Holds,
    Deep,
    System,
    Leads,
    HoldsLink,
}

impl Wary {
    fn words(self) -> &'static str {
        match self {
            Wary::Hidden => t!("folders-outside-wary-hidden"),
            Wary::Holds => t!("folders-outside-wary-holds"),
            Wary::Deep => t!("folders-outside-wary-deep"),
            Wary::System => t!("folders-outside-wary-system"),
            Wary::Leads => t!("folders-outside-wary-leads"),
            Wary::HoldsLink => t!("folders-outside-wary-holds-link"),
        }
    }
}

pub fn drive_roots(mounts: &str, home: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = DRIVE_ROOTS.iter().map(PathBuf::from).collect();
    for line in mounts.lines() {
        let mut fields = line.split(' ');
        let (Some(device), Some(at), Some(kind)) = (fields.next(), fields.next(), fields.next()) else { continue };
        let real = (device.starts_with("/dev/") && !device.starts_with("/dev/loop")) || super::places::on_another_computer(kind);
        let at = PathBuf::from(super::places::unescape(at));
        let top = at.components().nth(1).and_then(|c| c.as_os_str().to_str().map(str::to_string));
        let system = top.as_deref().is_none_or(|t| SYSTEM_TOPS.contains(&t));
        if real && !system && !home.starts_with(&at) && !out.contains(&at) {
            out.push(at);
        }
    }
    out
}

#[cfg(not(test))]
fn drives_now(home: &Path) -> Vec<PathBuf> {
    drive_roots(&std::fs::read_to_string("/proc/self/mounts").unwrap_or_default(), home)
}

#[cfg(test)]
fn drives_now(home: &Path) -> Vec<PathBuf> {
    drive_roots("", home)
}

fn hidden_in(path: &Path, home: &Path) -> bool {
    path.strip_prefix(home).unwrap_or(path).components()
        .any(|c| matches!(c, Component::Normal(n) if n.as_encoded_bytes().first() == Some(&b'.')))
}

fn placed(path: &Path, home: &Path, drives: &[PathBuf]) -> Option<Wary> {
    if hidden_in(path, home) {
        return Some(Wary::Hidden);
    }
    let inside = |top: &Path| path != top && path.starts_with(top);
    (!inside(home) && !drives.iter().any(|d| inside(d))).then_some(Wary::System)
}

#[cfg(test)]
fn looked_through(path: &Path, left: &mut usize, until: Instant) -> Option<Wary> {
    if !std::fs::symlink_metadata(path).is_ok_and(|m| m.is_dir()) {
        return None;
    }
    through_folder(path.to_path_buf(), left, until, &|_| false, &mut HashSet::new()).0
}

fn through_folder(folder: PathBuf, left: &mut usize, until: Instant, away: &dyn Fn(&Path) -> bool, known: &mut HashSet<Which>) -> (Option<Wary>, bool) {
    let mut why = None;
    let mut todo = vec![folder];
    while let Some(dir) = todo.pop() {
        let Ok(read) = std::fs::read_dir(&dir) else { return (why.or(Some(Wary::Deep)), false) };
        for entry in read {
            let Ok(entry) = entry else { return (why.or(Some(Wary::Deep)), false) };
            if *left == 0 || Instant::now() >= until {
                return (why.or(Some(Wary::Deep)), false);
            }
            *left -= 1;
            let Ok(meta) = entry.metadata() else { return (why.or(Some(Wary::Deep)), false) };
            if entry.file_name().as_encoded_bytes().first() == Some(&b'.') {
                why = why.or(Some(Wary::Holds));
                known.insert(Which::of(&meta));
            }
            if meta.file_type().is_symlink() {
                if away(&entry.path()) {
                    why = why.or(Some(Wary::HoldsLink));
                }
                known.insert(Which::of(&meta));
            } else if meta.is_dir() {
                todo.push(entry.path());
            }
        }
    }
    (why, true)
}

fn leads_away(link: &Path, real_home: &Path, drives: &[PathBuf]) -> bool {
    std::fs::canonicalize(link).map_or(true, |real| placed(&real, real_home, drives).is_some())
}

#[allow(clippy::type_complexity)]
fn why_wary(opened: &[(PathBuf, Arc<std::fs::File>, std::fs::Metadata)], home: &Path, drives: &[PathBuf]) -> Vec<(Option<Wary>, Option<Arc<HashSet<Which>>>)> {
    let real_home = work::real_of(home);
    let (mut left, until) = (LOOK_MOST, Instant::now() + LOOK_TIME);
    let away = |link: &Path| leads_away(link, &real_home, drives);
    opened.iter().map(|(path, named, meta)| {
        let real = std::fs::read_link(work::through(named)).unwrap_or_else(|_| path.clone());
        let lies = placed(path, home, drives)
            .or_else(|| if real == *path { None } else { placed(&real, &real_home, drives) })
            .or_else(|| (meta.file_type().is_symlink() && away(path)).then_some(Wary::Leads));
        if !meta.is_dir() {
            return (lies, None);
        }
        let mut known = HashSet::new();
        let (holds, whole) = through_folder(work::through(named), &mut left, until, &away, &mut known);
        (lies.or(holds), Some(if whole { Arc::new(known) } else { work::too_big_known() }))
    }).collect()
}

#[derive(Clone, Debug, Default)]
pub struct Looked {
    gone: Option<PathBuf>,
    inside: bool,
    wary: Vec<(PathBuf, Wary)>,
    which: Vec<Sure>,
    folders: bool,
}

pub fn look(files: &[PathBuf], into: Option<&Path>, home: &Path, pin: bool) -> Looked {
    let mut out = Looked::default();
    if pin {
        out.folders = files.iter().all(|p| p.is_dir());
        return out;
    }
    let mut opened = Vec::with_capacity(files.len());
    for path in files {
        match work::open_same(path, None) {
            Ok((named, meta)) => opened.push((path.clone(), Arc::new(named), meta)),
            Err(_) => {
                out.gone = Some(path.clone());
                return out;
            }
        }
    }
    if files.iter().any(|p| into.is_some_and(|t| work::goes_into(t, p))) {
        out.inside = true;
        return out;
    }
    let found = why_wary(&opened, home, &drives_now(home));
    for (n, ((path, named, meta), (why, known))) in opened.into_iter().zip(found).enumerate() {
        out.wary.extend(why.map(|why| (path.clone(), why)));
        out.which.push(Sure { path, which: Which::of(&meta), held: (n < HELD_MOST).then_some(named), known });
    }
    out
}

pub struct Looking {
    n: u64,
    files: Vec<PathBuf>,
    target: Where,
    how: act::Way,
    moving: bool,
    pin: Option<usize>,
}

pub struct Asked {
    pub files: Vec<PathBuf>,
    pub target: Where,
    how: act::Way,
    pub wary: Vec<(PathBuf, Wary)>,
    into: String,
    home: PathBuf,
    which: Vec<Sure>,
}

pub fn view(a: &Asked) -> Node<Msg> {
    let n = a.files.len();
    let first = a.files.first().map(|f| work::name_of(f)).unwrap_or_default();
    let (title, words, answer) = match a.how {
        act::Way::Move => (t!("folders-outside-title-move", n = n, name = &first), t!("folders-outside-body-move", n = n), t!("folders-outside-do-move", n = n)),
        act::Way::Ask => (t!("folders-outside-title-bring", n = n, name = &first), t!("folders-outside-body-bring", n = n), t!("folders-outside-do-bring", n = n)),
        _ => (t!("folders-outside-title-copy", n = n, name = &first), t!("folders-outside-body-copy", n = n), t!("folders-outside-do-copy", n = n)),
    };
    let leave = t!("folders-outside-leave", n = n);
    let mut body = vec![
        head_to("dialog-warning-symbolic", title, leave.as_str(), Msg::OutsideAnswer(false)),
        label(words).width(Length::Fill),
    ];
    let from = |p: &Path| p.parent().map(|up| short_path(up, &a.home)).unwrap_or_default();
    let ordinary = a.files.iter().filter(|f| !a.wary.iter().any(|(w, _)| w == *f));
    let lines = a.wary.iter().map(|(p, why)| t!("folders-outside-named-why", name = work::name_of(p), from = from(p), why = why.words()))
        .chain(ordinary.map(|p| t!("folders-outside-named", name = work::name_of(p), from = from(p))));
    body.extend(lines.take(NAMED_MOST).map(|l| note(l).width(Length::Fill)));
    if n > NAMED_MOST {
        body.push(note(t!("folders-outside-more", n = n - NAMED_MOST)));
    }
    body.push(label(t!("folders-outside-into", into = &a.into)).width(Length::Fill));
    body.push(foot(vec![word_button(answer).on_press(Msg::OutsideAnswer(true))],
                   main_action("window-close-symbolic", leave).on_press(Msg::OutsideAnswer(false))));
    card(column(body).gap(12.0).width(Length::Fill)).width(Length::Fixed(QUESTION_W)).on_escape(Msg::OutsideAnswer(false))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Onto {
    Icons(usize, usize, f32, f32),
    Land(Land),
    PinGap(usize),
}

enum Last {
    Refused,
    Looking,
    Asking,
    Job(u64),
    Done,
}

#[derive(Default)]
pub struct State {
    last: Option<Last>,
    waiting: Vec<(u64, u64)>,
    asked: Option<(u64, Way)>,
    hook: Option<Rc<dyn Fn(u64, bool)>>,
    looking: Option<Looking>,
    looks: u64,
    sure: Vec<Sure>,
    stayed: Option<(u64, Vec<(PathBuf, String)>)>,
    listed: Option<(u64, Vec<String>)>,
    handed: Option<Vec<Handed>>,
}

fn takes(at: &Where) -> bool {
    matches!(at, Where::Folder(_) | Where::Remote(_))
}

fn would(held: &Held) -> Option<Way> {
    if held.offered.files { held.usual() } else { None }
}

pub(super) fn onto_icons(f: &Files, node: Node<Msg>, p: usize, first: usize, by: f32) -> Node<Msg> {
    if !takes(&f.cols[p].at) {
        return node;
    }
    node.on_drop_from_outside(would, move |held, given, way| {
        let (x, y) = match held.part {
            Part::Point(x, y) => (x, y),
            _ => (0.0, 0.0),
        };
        Msg::Outside(Onto::Icons(p, first, x + by, y), given, way)
    })
}

fn would_pin(held: &Held) -> Option<Way> {
    let ways = held.offered.ways;
    if !held.offered.files {
        return None;
    }
    [Way::Copy, Way::Ask].into_iter().find(|w| ways.has(*w))
}

pub(super) fn onto_rows(f: &Files, node: Node<Msg>, to: Rc<dyn Fn(usize) -> Land>) -> Node<Msg> {
    let rows: Vec<bool> = (0..256).map_while(|i| f.land_place(to(i)).map(|at| takes(&at))).collect();
    let pinned = matches!(to(0), Land::Pinned(0));
    onto_places(node, move |i| rows.get(i).copied().unwrap_or(false), pinned, to)
}

pub(super) fn onto_names(node: Node<Msg>, ok: impl Fn(usize) -> bool + 'static, to: Rc<dyn Fn(usize) -> Land>) -> Node<Msg> {
    onto_places(node, ok, false, to)
}

fn onto_places(node: Node<Msg>, ok: impl Fn(usize) -> bool + 'static, pinned: bool, to: Rc<dyn Fn(usize) -> Land>) -> Node<Msg> {
    node.on_drop_from_outside(
        move |held| match held.part {
            Part::Gap(_) if pinned => would_pin(held),
            Part::Row(i) if ok(i) => would(held),
            _ => None,
        },
        move |held, given, way| match held.part {
            Part::Gap(gap) => Msg::Outside(Onto::PinGap(gap), given, way),
            Part::Row(row) => Msg::Outside(Onto::Land(to(row)), given, way),
            _ => Msg::Outside(Onto::Land(to(usize::MAX)), given, way),
        },
    )
}

fn refused_summary(refused: &[(PathBuf, Vec<Trouble>, Counts)]) -> (Counts, Vec<String>) {
    let (mut counts, mut lines) = (Counts::default(), Vec::new());
    for (_, bad, c) in refused {
        counts.add(*c);
        lines.extend(bad.iter().map(Trouble::line));
        let more = (c.unreadable + c.system).saturating_sub(bad.len());
        if more > 0 {
            lines.push(t!("folders-outside-lines-more", n = crate::common::thousands(more as u64)));
        }
    }
    (counts, lines)
}

fn stayed_words(stayed: &[(PathBuf, String)]) -> Option<String> {
    match stayed {
        [] => None,
        [(one, why)] => Some(t!("folders-outside-stayed-one", name = work::name_of(one), why = why.trim_start_matches("it "))),
        many if many.iter().all(|(_, why)| why == not_as_picked()) => {
            let names: Vec<String> = many.iter().map(|(p, _)| work::name_of(p)).collect();
            Some(t!("folders-outside-stayed-changed", n = many.len(), names = names.join(", ")))
        }
        many => {
            let each: Vec<String> = many.iter().map(|(p, why)| t!("folders-outside-stayed-each", name = work::name_of(p), why = why)).collect();
            Some(t!("folders-outside-stayed-many", n = many.len(), each = each.join(", ")))
        }
    }
}

impl Files {
    pub fn set_drop_hook(&mut self, hook: impl Fn(u64, bool) + 'static) {
        self.outside.hook = Some(Rc::new(hook));
    }

    pub(super) fn outside_drop(&mut self, onto: Onto, given: Given, way: Way) {
        self.outside.last = Some(Last::Refused);
        if matches!(self.dialog, Some(Dialog::Dropped(_))) {
            return self.say(t!("folders-outside-answer-first"), false);
        }
        if self.outside.looking.is_some() {
            return self.say(t!("folders-outside-still-looking"), false);
        }
        let files = match given {
            Given::Files(files) => files,
            Given::Text(_) | Given::Typed(..) => return self.say(t!("folders-outside-only-files"), false),
            Given::Nothing(why) => return self.say_help(why.words(), super::help::DRAG),
        };
        let target = match &onto {
            Onto::PinGap(gap) => {
                if !files.iter().all(|p| dnd::is_plain(p) && !super::far::is_far(p) && super::inside::within(p).is_none()) {
                    return self.say(t!("folders-outside-only-folders-pin"), false);
                }
                return self.outside_look(files, Where::Recent, act::Way::Copy, false, Some(*gap));
            }
            Onto::Icons(p, row, x, y) => {
                let Some(pane) = self.cols.get(*p) else { return };
                let pane = &**pane;
                let top = pane.grid.get().strip_top(*row);
                let hit = pane.grid.get().hit(*x, *y + top, pane.width.get(), pane.entries.len());
                match hit.and_then(|i| pane.entries.get(i)) {
                    Some(e) if e.folder && matches!(pane.at, Where::Folder(_)) => Where::Folder(e.path.clone()),
                    _ => pane.at.clone(),
                }
            }
            Onto::Land(land) => match self.land_place(*land) {
                Some(at) => at,
                None => return,
            },
        };
        if !takes(&target) {
            return self.say(t!("folders-outside-only-on-folder"), false);
        }
        if let Some(gone) = files.iter().find(|p| !dnd::is_plain(p)) {
            return self.say_help(t!("folders-outside-gone", name = work::name_of(gone)), super::help::DRAG);
        }
        let moving = way == Way::Move && self.mods.get().shift;
        let how = match way {
            Way::Ask => act::Way::Ask,
            _ if moving => act::Way::Move,
            _ => act::Way::Copy,
        };
        self.outside_look(files, target, how, moving, None);
    }

    fn outside_look(&mut self, files: Vec<PathBuf>, target: Where, how: act::Way, moving: bool, pin: Option<usize>) {
        let n = self.outside.looks + 1;
        self.outside.looks = n;
        let (home, into) = (self.home.clone(), target.folder().map(Path::to_path_buf));
        self.outside.looking = Some(Looking { n, files: files.clone(), target, how, moving, pin });
        self.outside.last = Some(Last::Looking);
        let Some(post) = self.post.clone() else {
            let looked = look(&files, into.as_deref(), &home, pin.is_some());
            return self.outside_looked(n, looked);
        };
        let late = post.clone();
        let _ = std::thread::Builder::new().name("folders-drop-look".into()).spawn(move || {
            post.send(Msg::OutsideLooked(n, Box::new(look(&files, into.as_deref(), &home, pin.is_some()))));
        });
        let _ = std::thread::Builder::new().name("folders-drop-wait".into()).spawn(move || {
            std::thread::sleep(CHECK_WAIT);
            late.send(Msg::OutsideLate(n));
        });
    }

    pub(super) fn outside_looked(&mut self, n: u64, looked: Looked) {
        let Some(l) = self.outside.looking.take_if(|l| l.n == n) else { return };
        let Looking { files, target, how, moving, pin, .. } = l;
        let into: Option<&Path> = target.folder();
        let last = if let Some(gap) = pin {
            if looked.folders {
                self.pin_here(files, gap);
                Last::Done
            } else {
                self.say(t!("folders-outside-only-folders-pin"), false);
                Last::Refused
            }
        } else if let Some(gone) = &looked.gone {
            self.say_help(t!("folders-outside-gone", name = work::name_of(gone)), super::help::DRAG);
            Last::Refused
        } else if looked.inside {
            self.say_help(t!("folders-outside-inside-itself"), super::help::DRAG);
            Last::Refused
        } else if moving && files.iter().any(|p| p.parent() == into) {
            self.say(t!("folders-outside-already-there"), false);
            Last::Refused
        } else if !looked.wary.is_empty() {
            self.said.clear();
            let (into, home) = (target.name(&self.home), self.home.clone());
            self.dialog = Some(Dialog::Dropped(Box::new(Asked { files, target, how, wary: looked.wary, into, home, which: looked.which })));
            Last::Asking
        } else {
            self.outside_bring(files, target, how, looked.which)
        };
        self.outside_settle(last);
    }

    pub(super) fn outside_late(&mut self, n: u64) {
        let Some(l) = self.outside.looking.take_if(|l| l.n == n) else { return };
        let name = l.files.first().map(|p| work::name_of(p)).unwrap_or_default();
        self.say_help(t!("folders-outside-not-answering", name = &name), super::help::DRAG);
        self.outside_settle(Last::Refused);
    }

    fn outside_settle(&mut self, last: Last) {
        if matches!(last, Last::Asking) && self.outside.asked.is_some() {
            return;
        }
        match self.outside.asked.take() {
            Some((id, way)) => self.outside_tell(id, way, last),
            None => self.outside.last = Some(last),
        }
    }

    pub(super) fn outside_sure(&mut self) -> Vec<Sure> {
        std::mem::take(&mut self.outside.sure)
    }

    fn outside_bring(&mut self, files: Vec<PathBuf>, target: Where, how: act::Way, which: Vec<Sure>) -> Last {
        let next = self.next_job;
        self.said.clear();
        self.outside.sure = which;
        self.drop_into(files, target, how);
        if how != act::Way::Ask {
            self.outside.sure.clear();
        }
        if self.jobs.iter().any(|j| j.id == next) {
            Last::Job(next)
        } else if how == act::Way::Ask || self.next_job > next {
            Last::Done
        } else {
            Last::Refused
        }
    }

    pub(super) fn outside_answer(&mut self, bring: bool) {
        let Some(Dialog::Dropped(_)) = &self.dialog else { return };
        let Some(Dialog::Dropped(a)) = self.dialog.take() else { return };
        let Asked { files, target, how, which, .. } = *a;
        let last = if !bring {
            self.say(t!("folders-outside-nothing-brought"), false);
            Last::Refused
        } else {
            self.outside_bring(files, target, how, which)
        };
        match self.outside.asked.take() {
            Some((id, way)) => self.outside_tell(id, way, last),
            None => self.outside.last = Some(last),
        }
    }

    fn outside_tell(&mut self, id: u64, way: Way, last: Last) {
        let done = match last {
            Last::Job(job) if way == Way::Move => return self.outside.waiting.push((job, id)),
            Last::Job(_) | Last::Done => true,
            Last::Refused | Last::Asking | Last::Looking => false,
        };
        if let Some(hook) = &self.outside.hook {
            hook(id, done);
        }
    }

    pub(super) fn outside_tidy(&mut self) {
        if self.outside.asked.is_some() && self.outside.looking.is_none() && !matches!(self.dialog, Some(Dialog::Dropped(_))) {
            if let Some((id, way)) = self.outside.asked.take() {
                self.outside_tell(id, way, Last::Refused);
            }
        }
    }

    pub(super) fn outside_end(&mut self, id: u64, way: Way) -> DropEnd {
        match self.outside.last.take() {
            Some(Last::Job(job)) if way == Way::Move => {
                self.outside.waiting.push((job, id));
                DropEnd::Later
            }
            Some(Last::Job(_) | Last::Done) => DropEnd::Done,
            Some(Last::Looking) if self.outside.looking.is_some() => {
                self.outside.asked = Some((id, way));
                DropEnd::Later
            }
            Some(Last::Asking) if matches!(self.dialog, Some(Dialog::Dropped(_))) => {
                self.outside.asked = Some((id, way));
                DropEnd::Later
            }
            Some(Last::Refused | Last::Asking | Last::Looking) | None => DropEnd::Failed,
        }
    }

    pub(super) fn outside_finished(&mut self, job: u64, over: &work::Over) {
        let Some(at) = self.outside.waiting.iter().position(|(j, _)| *j == job) else { return };
        let (_, drop) = self.outside.waiting.remove(at);
        let whole = !over.stopped && !over.troubled() && over.skipped == 0 && over.locked.is_empty();
        if let Some(hook) = &self.outside.hook {
            hook(drop, whole);
        }
    }

    pub(super) fn outside_held(&self) -> Option<Lifted> {
        let d = self.drag.as_ref().filter(|d| d.moving && !d.items.is_empty())?;
        if !d.items.iter().all(|p| dnd::is_plain(p) && !super::far::is_far(p) && super::inside::within(p).is_none()) {
            return None;
        }
        let first = self.cols.get(d.pane).and_then(|c| c.entries.iter().find(|e| e.path == d.items[0]));
        let icon = match first {
            Some(e) if e.folder => "folder-symbolic",
            Some(e) => super::kinds::icon_for(e.kind.as_deref()),
            None => super::kinds::icon_for(None),
        };
        let label = match d.items.as_slice() {
            [one] => work::name_of(one),
            many => t!("folders-outside-items-dragged", n = many.len()),
        };
        Some(Lifted { files: d.items.clone(), typed: Vec::new(), right: d.right, label, icon: Some(icon.to_string()), folder: first.is_some_and(|e| e.folder_icon()) })
    }

    pub(super) fn outside_ended(&mut self, took: Option<dnd::Taken>) {
        let Some(d) = self.drag.take() else { return };
        self.landed = None;
        if !self.bad {
            self.said.clear();
        }
        let Some(taken) = took.filter(|t| t.way() == Way::Move) else { return };
        let (items, was) = (d.items, d.was);
        let look = move || -> Vec<Left> { items.into_iter().enumerate().map(|(n, item)| Handed::of(item, was.get(n).cloned().flatten(), &taken)).collect() };
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name("folders-drag-left".into()).spawn(move || post.send(Msg::OutLeft(look(), taken)));
            }
            None => {
                let left = look();
                self.outside_left(left, taken);
            }
        }
    }

    pub(super) fn outside_left(&mut self, left: Vec<Left>, taken: dnd::Taken) {
        if taken.way() != Way::Move {
            return;
        }
        let (mut same, mut stayed, mut refused) = (Vec::new(), Vec::new(), Vec::new());
        for thing in left {
            match thing {
                Left::Same(handed) => same.push(handed),
                Left::Changed(path) => stayed.push((path, not_as_picked().to_string())),
                Left::Unsure(path) => stayed.push((path, not_looked_through().to_string())),
                Left::Refused(path, bad, counts) => refused.push((path, bad, counts)),
                Left::Gone => {}
            }
        }
        let (counts, lines) = refused_summary(&refused);
        let not_moved = counts.said();
        if same.is_empty() {
            let plain = stayed_words(&stayed);
            return match (plain, not_moved) {
                (None, None) => {}
                (Some(words), None) => self.say_help(words, super::help::DRAG),
                (None, Some(words)) => {
                    self.say_listed(words, true, lines);
                    self.said_help = Some(super::help::DRAG);
                }
                (Some(plain), Some(words)) => {
                    self.say_listed(t!("folders-outside-both", first = &plain, second = &words), true, lines);
                    self.said_help = Some(super::help::DRAG);
                }
            };
        }
        for (path, ..) in &refused {
            stayed.push((path.clone(), counts.reason()));
        }
        if !stayed.is_empty() {
            self.outside.stayed = Some((self.next_job, stayed));
            if !lines.is_empty() {
                self.outside.listed = Some((self.next_job, lines));
            }
        }
        self.trash_handed(same);
        if self.outside.stayed.as_ref().is_some_and(|(job, _)| !self.jobs.iter().any(|j| j.id == *job)) {
            self.outside.stayed = None;
            self.outside.listed = None;
        }
    }

    fn trash_handed(&mut self, handed: Vec<Handed>) {
        let items = handed.iter().map(|h| h.path().to_path_buf()).collect();
        self.outside.handed = Some(handed);
        self.trash_these(items);
        self.outside.handed = None;
    }

    pub(super) fn outside_handed(&mut self) -> Option<Vec<Handed>> {
        self.outside.handed.take()
    }

    pub(super) fn outside_stayed(&mut self, job: u64) -> Vec<String> {
        match self.outside.stayed.take_if(|(of, _)| *of == job) {
            Some((_, stayed)) => stayed.iter().map(|(p, why)| t!("folders-outside-stayed-each", name = work::name_of(p), why = why)).collect(),
            None => Vec::new(),
        }
    }

    pub(super) fn outside_listed(&mut self, job: u64) -> Option<Vec<String>> {
        self.outside.listed.take_if(|(of, _)| *of == job).map(|(_, lines)| lines)
    }

    pub(super) fn outside_unshown(&mut self, way: Way) {
        if self.drag.take().is_none() {
            return;
        }
        self.landed = None;
        if way == Way::Move {
            self.say_help(t!("folders-outside-unshown"), super::help::DRAG);
        }
    }

    pub(super) fn outside_short(&mut self, way: Way) {
        if self.drag.take().is_none() {
            return;
        }
        self.landed = None;
        self.say_help(if way == Way::Move { t!("folders-outside-short-move") } else { t!("folders-outside-short-copy") }, super::help::DRAG);
    }
}

#[cfg(test)]
mod tests;
