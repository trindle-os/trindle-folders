//! The bigger jobs the right-click menu opens a window for: Compress,

use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::app::{Files, Msg};
use super::archive::{self, Format, Place};
use super::compare::{self, Newer, Row, State, Why};
use super::dialogs::{foot, head, short_path, AskMsg, Dialog};
use super::dupes::{self, Found};
use super::rename::{self, At, Case, Clash, Ending, Planned, Rule};
use super::view::size_words;
use super::wipe::{self, Caveat};
use super::work::{self, Extra, Kind, Step};
use crate::toolkit::event::Post;
use crate::toolkit::t;
use crate::toolkit::tiles::standard_ring;
use crate::toolkit::widget::{button, card, choice, column, entry, item, label, list, main_action, note, row, scroll, space, switch, Length, Node, Tone};

const NARROW: f32 = 420.0;
const WIDE: f32 = 640.0;
const DIFF_W: f32 = 1100.0;
const DIFF_MARGIN: f32 = 40.0;
const DIFF_H: f32 = 440.0;
const KEY_W: f32 = 90.0;
const PREVIEW_H: f32 = 200.0;
const LIST_H: f32 = 300.0;
const RING: f32 = 48.0;
const TICK: Duration = Duration::from_millis(120);
pub(super) fn stopped(words: Node<Msg>) -> Node<Msg> {
    row(vec![words.width(Length::Fill), crate::common::more::button(Msg::Help(super::help::COMPARE_STOPPED))]).gap(8.0).width(Length::Fill)
}

fn passes() -> [(u32, &'static str); 3] {
    [(1, t!("folders-tools-pass-once")), (3, t!("folders-tools-pass-three")), (7, t!("folders-tools-pass-seven"))]
}
fn cases() -> [&'static str; 5] {
    [t!("folders-tools-case-keep"), t!("folders-tools-case-lower"), t!("folders-tools-case-upper"), t!("folders-tools-case-words"), t!("folders-tools-case-first")]
}
fn endings() -> [&'static str; 5] {
    [t!("folders-tools-ending-keep"), t!("folders-tools-ending-lower"), t!("folders-tools-ending-upper"), t!("folders-tools-ending-new"), t!("folders-tools-ending-none")]
}
const NEW_ENDING: usize = 3;

pub enum Tool {
    Pack(Pack),
    Unpack { archive: PathBuf, places: Vec<(String, PathBuf)> },
    Rename(Batch),
    Wipe(WipeAsk),
    Dupes(Dupes),
    Compare(Comparing),
    Resize { items: Vec<PathBuf>, size: usize },
    Diff(Diffing),
}

pub struct Diffing {
    pub left: PathBuf,
    pub right: PathBuf,
    pub id: u64,
    stop: Arc<AtomicBool>,
    pub got: Option<Result<super::diff::Outcome, String>>,
    pub width: f32,
}

impl Drop for Diffing {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

pub struct Pack {
    pub items: Vec<PathBuf>,
    pub to: PathBuf,
    pub name: String,
    pub format: usize,
}

#[derive(Default)]
pub struct Batch {
    pub items: Vec<PathBuf>,
    pub find: String,
    pub with: String,
    pub wildcard: bool,
    pub match_case: bool,
    pub before: String,
    pub after: String,
    pub number: bool,
    pub at_end: bool,
    pub start: String,
    pub case: usize,
    pub ending: usize,
    pub new_end: String,
    pub plan: Vec<Planned>,
}

pub struct WipeAsk {
    pub items: Vec<PathBuf>,
    pub caveats: Vec<Caveat>,
    pub passes: usize,
    pub left_out: Vec<String>,
}

pub struct Dupes {
    pub roots: Vec<PathBuf>,
    pub id: u64,
    stop: Arc<AtomicBool>,
    pub progress: (u64, u64),
    pub found: Option<Result<Found, String>>,
    pub rows: Vec<(usize, PathBuf)>,
    pub picked: Vec<usize>,
}

pub struct Comparing {
    pub left: PathBuf,
    pub right: PathBuf,
    pub content: bool,
    pub all: bool,
    pub id: u64,
    stop: Arc<AtomicBool>,
    pub progress: (u64, u64),
    pub rows: Option<Result<Vec<Row>, String>>,
}

impl Drop for Dupes {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for Comparing {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[derive(Clone, Copy)]
pub enum Field {
    PackName,
    Find,
    With,
    Before,
    After,
    Start,
    NewEnd,
}

#[derive(Clone, Copy)]
pub enum Flag {
    Wildcard,
    MatchCase,
    Number,
    AtEnd,
    Content,
    All,
}

#[derive(Clone)]
pub enum ToolMsg {
    Text(Field, String),
    Flag(Flag, bool),
    Format(usize),
    Case(usize),
    Ending(usize),
    Passes(usize),
    Pack,
    Unpack(usize),
    Rename,
    Wipe,
    Progress(u64, u64, u64),
    Found(u64, Result<Found, String>),
    Picked(Vec<usize>),
    PickExtras,
    TrashPicked,
    Compared(u64, Result<Vec<Row>, String>),
    Again,
    CopyOver(bool),
    Stay,
    Size(usize),
    Resize,
    Diffed(u64, Result<super::diff::Outcome, String>),
}

fn pack_name(items: &[PathBuf], to: &Path) -> String {
    match items {
        [one] if super::reach::is_dir(one) => work::name_of(one),
        [one] => archive::stem(one),
        _ => to.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| t!("folders-tools-archive-name").into()),
    }
}

fn pad_for(start: u64, count: usize) -> usize {
    (start + count.saturating_sub(1) as u64).to_string().len()
}

impl Batch {
    pub fn new(items: Vec<PathBuf>) -> Batch {
        let mut b = Batch { items, start: "1".into(), ..Batch::default() };
        b.refresh();
        b
    }

    fn rules(&self) -> Vec<Rule> {
        let mut rules = Vec::new();
        if !self.find.is_empty() {
            rules.push(Rule::Replace { find: self.find.clone(), with: self.with.clone(), wildcard: self.wildcard, match_case: self.match_case });
        }
        if !self.before.is_empty() {
            rules.push(Rule::Before(self.before.clone()));
        }
        if !self.after.is_empty() {
            rules.push(Rule::After(self.after.clone()));
        }
        match self.case {
            1 => rules.push(Rule::Case(Case::Lower)),
            2 => rules.push(Rule::Case(Case::Upper)),
            3 => rules.push(Rule::Case(Case::Title)),
            4 => rules.push(Rule::Case(Case::Sentence)),
            _ => {}
        }
        match self.ending {
            1 => rules.push(Rule::Ending(Ending::Lower)),
            2 => rules.push(Rule::Ending(Ending::Upper)),
            NEW_ENDING if !self.new_end.trim().trim_start_matches('.').is_empty() => {
                rules.push(Rule::Ending(Ending::Set(self.new_end.trim().trim_start_matches('.').to_string())));
            }
            4 => rules.push(Rule::Ending(Ending::Set(String::new()))),
            _ => {}
        }
        if self.number {
            let start = self.start.trim().parse::<u64>().unwrap_or(1);
            let at = if self.at_end { At::End } else { At::Start };
            rules.push(Rule::Number { start, step: 1, pad: pad_for(start, self.items.len()), at, sep: " ".into() });
        }
        rules
    }

    fn refresh(&mut self) {
        self.plan = rename::preview(&self.items, &self.rules());
    }

    fn tally(&self) -> (usize, bool) {
        (self.plan.iter().filter(|p| p.moves()).count(), self.plan.iter().any(|p| p.clash.is_some()))
    }
}

fn clash_words(c: Clash) -> &'static str {
    match c {
        Clash::Empty => t!("folders-tools-clash-empty"),
        Clash::Bad => t!("folders-tools-clash-bad"),
        Clash::Long => t!("folders-tools-clash-long"),
        Clash::Twice => t!("folders-tools-clash-twice"),
        Clash::Taken => t!("folders-tools-clash-taken"),
        Clash::NotText => t!("folders-tools-clash-not-text"),
    }
}

fn caveat_words(c: &Caveat) -> String {
    match c {
        Caveat::Flash => t!("folders-tools-caveat-flash").into(),
        Caveat::CopyOnWrite(fs) => t!("folders-tools-caveat-copy-on-write", fs = fs),
        Caveat::Network(fs) => t!("folders-tools-caveat-network", fs = fs),
    }
}

fn to_copy(rows: &[Row], to_right: bool) -> BTreeMap<PathBuf, Vec<PathBuf>> {
    let mut out: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for r in rows {
        let (only, newer, side) = if to_right { (State::OnlyLeft, Newer::Left, &r.left) } else { (State::OnlyRight, Newer::Right, &r.right) };
        let file = side.as_ref().is_some_and(|s| s.kind != compare::Kind::Folder);
        let take = r.state == only || (matches!(r.state, State::Different(_)) && r.newer == Some(newer) && file);
        if take {
            let parent = r.name.parent().map(Path::to_path_buf).unwrap_or_default();
            out.entry(parent).or_default().push(r.name.clone());
        }
    }
    out
}

fn row_words(r: &Row) -> (&'static str, String) {
    let newer = match r.newer {
        Some(Newer::Left) => "left",
        Some(Newer::Right) => "right",
        None => "none",
    };
    match r.state {
        State::OnlyLeft => ("go-previous-symbolic", t!("folders-tools-row-only-left").into()),
        State::OnlyRight => ("go-next-symbolic", t!("folders-tools-row-only-right").into()),
        State::Same => (crate::common::no_tick::MARK_ICON, t!("folders-tools-row-same").into()),
        State::Different(why) => {
            let why = match why {
                Why::Kind => "kind",
                Why::Size => "size",
                Why::Date => "date",
                Why::Content => "content",
                Why::Target => "target",
                Why::Inside => "inside",
                Why::Unread => "unread",
            };
            ("document-edit-symbolic", t!("folders-tools-row-different", why = why, newer = newer))
        }
    }
}

fn extras(rows: &[(usize, PathBuf)]) -> Vec<usize> {
    (0..rows.len()).filter(|&i| i > 0 && rows[i - 1].0 == rows[i].0).collect()
}

fn takes_a_whole_set(rows: &[(usize, PathBuf)], picked: &[usize]) -> bool {
    let mut left: BTreeMap<usize, usize> = BTreeMap::new();
    for (g, _) in rows {
        *left.entry(*g).or_default() += 1;
    }
    for &i in picked {
        if let Some((g, _)) = rows.get(i) {
            *left.entry(*g).or_default() -= 1;
        }
    }
    left.values().any(|&n| n == 0)
}

fn dupe_rows(found: &Found) -> Vec<(usize, PathBuf)> {
    found.groups.iter().enumerate().flat_map(|(g, grp)| grp.files.iter().map(move |f| (g, f.clone()))).collect()
}

fn ticker(post: Post<Msg>, id: u64) -> impl Fn(u64, u64) {
    let last = Cell::new(None::<Instant>);
    move |done, total| {
        let now = Instant::now();
        if last.get().is_some_and(|l| now.duration_since(l) < TICK) {
            return;
        }
        last.set(Some(now));
        post.send(Msg::Tool(ToolMsg::Progress(id, done, total)));
    }
}

impl Files {
    fn off<T: Send + 'static>(
        &mut self,
        name: &str,
        job: impl FnOnce(&AtomicBool, &dyn Fn(u64, u64)) -> T + Send + 'static,
        done: fn(u64, T) -> ToolMsg,
    ) -> (u64, Arc<AtomicBool>, Option<ToolMsg>) {
        self.next_job += 1;
        let id = self.next_job;
        let stop = Arc::new(AtomicBool::new(false));
        match &self.post {
            Some(post) => {
                let (post, halt) = (post.clone(), stop.clone());
                let _ = std::thread::Builder::new().name(name.into()).spawn(move || {
                    let tick = ticker(post.clone(), id);
                    let got = job(&halt, &tick);
                    post.send(Msg::Tool(done(id, got)));
                });
                (id, stop, None)
            }
            None => {
                let got = job(&stop, &|_, _| {});
                (id, stop, Some(done(id, got)))
            }
        }
    }

    pub(super) fn pack(&mut self, act: super::menu::Act, p: usize, items: Vec<PathBuf>) {
        use super::menu::Act;
        let Some(here) = self.cols[p].at.folder().map(Path::to_path_buf) else { return };
        match act {
            Act::Compress if !items.is_empty() => {
                let name = pack_name(&items, &here);
                self.dialog = Some(Dialog::Tool(Tool::Pack(Pack { items, to: here, name, format: 0 })));
            }
            Act::ExtractHere => self.send_to(items, here, Kind::Other(Extra::Unpack(Place::Here))),
            Act::ExtractTo if items.len() > 1 => self.send_to(items, here, Kind::Other(Extra::Unpack(Place::Folder))),
            Act::ExtractTo => {
                let Some(archive) = items.into_iter().next() else { return };
                let other = self.cols[1 - p].at.folder().map(Path::to_path_buf);
                let mut places = vec![(t!("folders-tools-place-here").to_string(), here)];
                for pl in super::dialogs::send_places(&self.home, other.as_deref(), &self.pins_named()) {
                    if !places.iter().any(|(_, q)| *q == pl.1) {
                        places.push(pl);
                    }
                }
                self.dialog = Some(Dialog::Tool(Tool::Unpack { archive, places }));
            }
            _ => {}
        }
    }

    pub(super) fn compare_files(&mut self, left: PathBuf, right: PathBuf) {
        let width = (self.room.0 - 2.0 * DIFF_MARGIN).clamp(WIDE, DIFF_W);
        let (l, r) = (left.clone(), right.clone());
        let (id, stop, now) = self.off("folders-diff", move |stop, _| super::diff::compare(&l, &r, stop), ToolMsg::Diffed);
        self.dialog = Some(Dialog::Tool(Tool::Diff(Diffing { left, right, id, stop, got: None, width })));
        if let Some(m) = now {
            self.tool_msg(m);
        }
    }

    pub(super) fn ask_resize(&mut self, items: Vec<PathBuf>) {
        if !items.is_empty() {
            self.dialog = Some(Dialog::Tool(Tool::Resize { items, size: 1 }));
        }
    }

    pub(super) fn batch_rename(&mut self, items: Vec<PathBuf>) {
        self.renaming = None;
        self.dialog = Some(Dialog::Tool(Tool::Rename(Batch::new(items))));
    }

    pub(super) fn ask_wipe(&mut self, items: Vec<PathBuf>) {
        let (folders, files): (Vec<PathBuf>, Vec<PathBuf>) = items.into_iter().partition(|i| super::reach::is_dir(i));
        let left_out = folders.iter().map(|f| work::name_of(f)).collect();
        let Some(first) = files.first() else { return self.say(t!("folders-tools-only-files-wiped"), false) };
        let caveats = wipe::caveats(first.parent().unwrap_or(Path::new("/")));
        self.asked = super::proof::asked(&files);
        self.dialog = Some(Dialog::Tool(Tool::Wipe(WipeAsk { items: files, caveats, passes: 0, left_out })));
    }

    pub(super) fn find_dupes(&mut self, roots: Vec<PathBuf>) {
        if roots.is_empty() {
            return;
        }
        let search = roots.clone();
        let (id, stop, now) = self.off("folders-dupes", move |stop, tell| {
            dupes::find(&search, &dupes::Options::default(), stop, tell).map_err(|e| e.to_string())
        }, ToolMsg::Found);
        self.dialog = Some(Dialog::Tool(Tool::Dupes(Dupes { roots, id, stop, progress: (0, 0), found: None, rows: Vec::new(), picked: Vec::new() })));
        if let Some(m) = now {
            self.tool_msg(m);
        }
    }

    pub(super) fn compare_panes(&mut self) {
        let panes = &self.cols;
        let (Some(left), Some(right)) = (panes[0].at.folder().map(Path::to_path_buf), panes[1].at.folder().map(Path::to_path_buf)) else {
            return self.say(t!("folders-tools-compare-need-folders"), false);
        };
        self.dialog = Some(Dialog::Tool(Tool::Compare(Comparing {
            left, right, content: false, all: false, id: 0, stop: Arc::new(AtomicBool::new(false)), progress: (0, 0), rows: None,
        })));
        self.compare_again();
    }

    fn compare_again(&mut self) {
        let Some(Dialog::Tool(Tool::Compare(c))) = &mut self.dialog else { return };
        c.stop.store(true, Ordering::Relaxed);
        let (left, right) = (c.left.clone(), c.right.clone());
        let options = compare::Options { content: c.content, ..compare::Options::default() };
        let (id, stop, now) = self.off("folders-compare", move |stop, tell| {
            compare::compare(&left, &right, &options, stop, tell).map_err(|e| e.to_string())
        }, ToolMsg::Compared);
        if let Some(Dialog::Tool(Tool::Compare(c))) = &mut self.dialog {
            (c.id, c.stop, c.rows, c.progress) = (id, stop, None, (0, 0));
        }
        if let Some(m) = now {
            self.tool_msg(m);
        }
    }

    pub(super) fn tool_msg(&mut self, m: ToolMsg) {
        let Some(Dialog::Tool(tool)) = &self.dialog else { return };
        match tool {
            Tool::Pack(_) | Tool::Unpack { .. } | Tool::Resize { .. } | Tool::Diff(_) => self.pack_tool_msg(m),
            Tool::Rename(_) => self.rename_tool_msg(m),
            Tool::Wipe(_) => self.wipe_tool_msg(m),
            Tool::Dupes(_) => self.dupes_tool_msg(m),
            Tool::Compare(_) => self.compare_tool_msg(m),
        }
    }

    fn pack_tool_msg(&mut self, m: ToolMsg) {
        let Some(Dialog::Tool(tool)) = &mut self.dialog else { return };
        match (tool, m) {
            (Tool::Pack(p), ToolMsg::Text(Field::PackName, t)) => p.name = t,
            (Tool::Pack(p), ToolMsg::Format(i)) => p.format = i.min(Format::ALL.len() - 1),
            (Tool::Pack(p), ToolMsg::Pack) => {
                if bad_pack(p).is_some() {
                    return;
                }
                let (items, to, kind) = (std::mem::take(&mut p.items), p.to.clone(), Kind::Other(Extra::Pack(Format::ALL[p.format], p.name.trim().to_string())));
                self.dialog = None;
                self.send_to(items, to, kind);
            }
            (Tool::Diff(d), ToolMsg::Diffed(id, got)) if d.id == id => d.got = Some(got),
            (Tool::Resize { size, .. }, ToolMsg::Size(i)) => *size = i.min(super::imaging::SIZES.len() - 1),
            (Tool::Resize { items, size }, ToolMsg::Resize) => {
                let (items, size) = (std::mem::take(items), super::imaging::SIZES[*size].0);
                let to = items.first().and_then(|i| i.parent()).map(Path::to_path_buf).unwrap_or_default();
                self.dialog = None;
                self.send_to(items, to, Kind::Other(Extra::Picture(super::imaging::Work::Resize(size))));
            }
            (Tool::Unpack { archive, places }, ToolMsg::Unpack(i)) => {
                let Some((_, to)) = places.get(i).cloned() else { return };
                let archive = archive.clone();
                self.dialog = None;
                self.send_to(vec![archive], to, Kind::Other(Extra::Unpack(Place::Folder)));
            }
            _ => {}
        }
    }

    fn rename_tool_msg(&mut self, m: ToolMsg) {
        let Some(Dialog::Tool(tool)) = &mut self.dialog else { return };
        match (tool, m) {
            (Tool::Rename(b), ToolMsg::Rename) => {
                let (changes, clash) = b.tally();
                if changes == 0 || clash {
                    return;
                }
                let plan: Vec<Planned> = b.plan.iter().filter(|p| p.moves()).cloned().collect();
                self.dialog = None;
                match rename::apply(&plan) {
                    Ok(undo) => {
                        let n = undo.moves.len();
                        self.remember(t!("folders-tools-rename-undo", n = n), vec![Step::Other(undo)]);
                        self.say(t!("folders-tools-rename-done", n = n), false);
                    }
                    Err(e) => self.say_help(t!("folders-tools-rename-stopped", error = &e.to_string()), super::help::WORK_NOT_DONE),
                }
                self.reload_all();
            }
            (Tool::Rename(b), m) => {
                match m {
                    ToolMsg::Text(Field::Find, t) => b.find = t,
                    ToolMsg::Text(Field::With, t) => b.with = t,
                    ToolMsg::Text(Field::Before, t) => b.before = t,
                    ToolMsg::Text(Field::After, t) => b.after = t,
                    ToolMsg::Text(Field::Start, t) => b.start = t.chars().filter(char::is_ascii_digit).take(9).collect(),
                    ToolMsg::Text(Field::NewEnd, t) => b.new_end = t,
                    ToolMsg::Flag(Flag::Wildcard, on) => b.wildcard = on,
                    ToolMsg::Flag(Flag::MatchCase, on) => b.match_case = on,
                    ToolMsg::Flag(Flag::Number, on) => b.number = on,
                    ToolMsg::Flag(Flag::AtEnd, on) => b.at_end = on,
                    ToolMsg::Case(i) => b.case = i,
                    ToolMsg::Ending(i) => b.ending = i,
                    _ => return,
                }
                b.refresh();
            }
            _ => {}
        }
    }

    fn wipe_tool_msg(&mut self, m: ToolMsg) {
        let Some(Dialog::Tool(tool)) = &mut self.dialog else { return };
        match (tool, m) {
            (Tool::Wipe(w), ToolMsg::Passes(i)) => w.passes = i.min(passes().len() - 1),
            (Tool::Wipe(w), ToolMsg::Wipe) => {
                let (items, passes) = (std::mem::take(&mut w.items), passes()[w.passes].0);
                self.dialog = None;
                self.forget_undo(&items);
                self.send_to(items, PathBuf::new(), Kind::Other(Extra::Wipe(passes)));
            }
            _ => {}
        }
    }

    fn dupes_tool_msg(&mut self, m: ToolMsg) {
        let Some(Dialog::Tool(tool)) = &mut self.dialog else { return };
        match (tool, m) {
            (Tool::Dupes(d), ToolMsg::Progress(id, done, total)) if d.id == id && d.found.is_none() => d.progress = (done, total),
            (Tool::Dupes(d), ToolMsg::Found(id, found)) if d.id == id => {
                d.rows = found.as_ref().map(dupe_rows).unwrap_or_default();
                d.picked = extras(&d.rows);
                d.found = Some(found);
            }
            (Tool::Dupes(d), ToolMsg::Picked(p)) => d.picked = p,
            (Tool::Dupes(d), ToolMsg::PickExtras) => d.picked = extras(&d.rows),
            (Tool::Dupes(d), ToolMsg::TrashPicked) => {
                if d.picked.is_empty() || takes_a_whole_set(&d.rows, &d.picked) {
                    return;
                }
                let gone: Vec<PathBuf> = d.picked.iter().filter_map(|&i| d.rows.get(i)).map(|(_, p)| p.clone()).collect();
                if let Some(Ok(found)) = &mut d.found {
                    for g in &mut found.groups {
                        g.files.retain(|f| !gone.contains(f));
                    }
                    found.groups.retain(|g| g.files.len() > 1);
                    d.rows = dupe_rows(found);
                }
                d.picked.clear();
                self.trash_these(gone);
            }
            _ => {}
        }
    }

    fn compare_tool_msg(&mut self, m: ToolMsg) {
        let Some(Dialog::Tool(tool)) = &mut self.dialog else { return };
        match (tool, m) {
            (Tool::Compare(c), ToolMsg::Progress(id, done, total)) if c.id == id && c.rows.is_none() => c.progress = (done, total),
            (Tool::Compare(c), ToolMsg::Compared(id, rows)) if c.id == id => c.rows = Some(rows),
            (Tool::Compare(c), ToolMsg::Flag(Flag::All, on)) => c.all = on,
            (Tool::Compare(c), ToolMsg::Flag(Flag::Content, on)) => {
                c.content = on;
                self.compare_again();
            }
            (Tool::Compare(_), ToolMsg::Again) => self.compare_again(),
            (Tool::Compare(c), ToolMsg::CopyOver(to_right)) => {
                let Some(Ok(rows)) = &c.rows else { return };
                let (from, to) = if to_right { (c.left.clone(), c.right.clone()) } else { (c.right.clone(), c.left.clone()) };
                let plan = to_copy(rows, to_right);
                self.dialog = None;
                for (parent, names) in plan {
                    self.send_to(names.iter().map(|n| from.join(n)).collect(), to.join(parent), Kind::Copy);
                }
            }
            _ => {}
        }
    }
}

fn bad_pack(p: &Pack) -> Option<&'static str> {
    super::act::bad_name(&format!("{}{}", p.name.trim(), Format::ALL[p.format].ending()))
        .or_else(|| p.name.trim().is_empty().then_some(t!("folders-tools-name-empty")))
}

fn tool(m: ToolMsg) -> Msg {
    Msg::Tool(m)
}

fn act(icon: &str, words: &str, m: ToolMsg, off: bool) -> Node<Msg> {
    main_action(icon.to_string(), words.to_string()).on_press(tool(m)).off(off)
}

pub(super) fn keyed(key: &str, body: Vec<Node<Msg>>) -> Node<Msg> {
    let mut all = vec![note(key.to_string()).width(Length::Fixed(KEY_W))];
    all.extend(body);
    row(all).gap(8.0).width(Length::Fill)
}

fn labelled(key: &str, body: Vec<Node<Msg>>) -> Node<Msg> {
    column(vec![note(key.to_string()), row(body).gap(8.0).width(Length::Fill)]).gap(4.0).width(Length::Fill)
}

fn buttons(mut b: Vec<Node<Msg>>) -> Node<Msg> {
    b.insert(0, space().width(Length::Fill));
    row(b).gap(8.0).width(Length::Fill)
}

fn progress(done: u64, total: u64, listing: &str) -> Node<Msg> {
    let (share, words) = if total == 0 && done == 0 {
        (None, listing.to_string())
    } else if total == 0 {
        (None, t!("folders-tools-looked-at", done = done))
    } else {
        (Some(crate::common::gentle::share(done, total, None, false)), t!("folders-tools-read-of", done = &size_words(done), total = &size_words(total)))
    };
    row(vec![standard_ring(share, RING).width(Length::Shrink), note(words).width(Length::Fill)]).gap(12.0).width(Length::Fill).centred()
}

pub fn view(t: &Tool, turn: u32) -> Node<Msg> {
    let home = crate::common::files::home();
    let (body, width): (Vec<Node<Msg>>, f32) = match t {
        Tool::Pack(p) => {
            let first = p.items.first().map(|i| work::name_of(i)).unwrap_or_default();
            let what = t!("folders-tools-compress-title", n = p.items.len(), name = &first);
            let bad = bad_pack(p);
            let endings = Format::ALL.iter().map(|f| f.ending().to_string()).collect();
            let mut b = vec![
                head("package-x-generic-symbolic", what),
                entry(p.name.clone(), t!("folders-tools-pack-name-tip"), |t| tool(ToolMsg::Text(Field::PackName, t)))
                    .key("packname").grab(turn).on_submit(tool(ToolMsg::Pack)).on_escape(Msg::Ask(AskMsg::Close))
                    .width(Length::Fill),
                choice(endings, p.format, t!("folders-tools-pack-kind-tip"), |i| tool(ToolMsg::Format(i))),
            ];
            b.push(note(bad.map_or_else(|| t!("folders-tools-pack-made-in", place = &short_path(&p.to, &home)), str::to_string)));
            b.push(foot(Vec::new(), act("package-x-generic-symbolic", t!("folders-tools-compress"), ToolMsg::Pack, bad.is_some())));
            (b, NARROW)
        }
        Tool::Unpack { archive, places } => {
            let rows = places.iter().map(|(name, path)| item(name.clone()).icon("folder-symbolic").folder().detail(short_path(path, &home))).collect();
            (vec![
                head("archive-extract-symbolic", t!("folders-tools-extract-to", name = &work::name_of(archive))),
                note(t!("folders-tools-extract-new-folder", name = &archive::stem(archive))),
                super::dialogs::dialog_list("extract-to", list(rows, None, |i| tool(ToolMsg::Unpack(i))).grab(turn), places.len()),
            ], NARROW)
        }
        Tool::Rename(b) => (rename_view(b, turn, &home), WIDE),
        Tool::Diff(d) => (diff_view(d), d.width),
        Tool::Resize { items, size } => {
            let first = items.first().map(|i| work::name_of(i)).unwrap_or_default();
            let what = t!("folders-tools-resize-title", n = items.len(), name = &first);
            let words = super::imaging::SIZES.iter().map(|(_, w)| w.to_string()).collect();
            (vec![
                head(super::icons::stand_in::RESIZE, what),
                choice(words, *size, t!("folders-tools-resize-size-tip"), |i| tool(ToolMsg::Size(i))),
                note(t!("folders-tools-resize-note")),
                foot(Vec::new(), act(super::icons::stand_in::RESIZE, t!("folders-tools-resize"), ToolMsg::Resize, false)),
            ], NARROW)
        }
        Tool::Wipe(w) => {
            let first = w.items.first().map(|i| work::name_of(i)).unwrap_or_default();
            let what = t!("folders-tools-wipe-title", n = w.items.len(), name = &first);
            let mut lines = vec![
                head(super::icons::stand_in::WIPE, what),
                label(t!("folders-tools-wipe-warning")),
            ];
            if !w.left_out.is_empty() {
                lines.push(note(t!("folders-tools-wipe-left-out", names = &super::act::names(&w.left_out))));
            }
            for c in &w.caveats {
                lines.push(note(caveat_words(c)));
            }
            lines.push(keyed(t!("folders-tools-wipe-write-over"), vec![choice(passes().iter().map(|(_, w)| w.to_string()).collect(), w.passes, t!("folders-tools-wipe-passes-tip"), |i| tool(ToolMsg::Passes(i)))]));
            lines.push(foot(Vec::new(), act(super::icons::stand_in::WIPE, t!("folders-tools-wipe-button"), ToolMsg::Wipe, false).cannot_be_undone()));
            (lines, NARROW)
        }
        Tool::Dupes(d) => (dupes_view(d, &home), WIDE),
        Tool::Compare(c) => (compare_view(c), WIDE),
    };
    card(column(body).gap(12.0).width(Length::Fill)).width(Length::Fixed(width)).on_escape(Msg::Ask(AskMsg::Close))
}

fn rename_view(b: &Batch, turn: u32, home: &Path) -> Vec<Node<Msg>> {
    let text = |value: &str, tip: &str, key: &str, f: Field| {
        entry(value.to_string(), tip.to_string(), move |t| tool(ToolMsg::Text(f, t))).key(key.to_string()).on_escape(Msg::Ask(AskMsg::Close))
    };
    let flag = |on: bool, tip: &str, f: Flag| switch(on, tip.to_string(), move |v| tool(ToolMsg::Flag(f, v)));
    let mut number = vec![flag(b.number, t!("folders-tools-rename-number-tip"), Flag::Number)];
    if b.number {
        number.push(choice(vec![t!("folders-tools-rename-at-start").into(), t!("folders-tools-rename-at-end").into()], usize::from(b.at_end), t!("folders-tools-rename-where-tip"), |i| tool(ToolMsg::Flag(Flag::AtEnd, i == 1))));
        number.push(text(&b.start, t!("folders-tools-rename-first-number-tip"), "rn-start", Field::Start));
    } else {
        number.push(space().width(Length::Fill));
    }
    let mut ending = vec![choice(endings().iter().map(|s| s.to_string()).collect(), b.ending, t!("folders-tools-rename-ending-tip"), |i| tool(ToolMsg::Ending(i)))];
    if b.ending == NEW_ENDING {
        ending.push(text(&b.new_end, t!("folders-tools-rename-new-ending-tip"), "rn-end", Field::NewEnd));
    }
    let rows = b.plan.iter().map(|p| {
        let new = work::name_of(&p.to);
        let old = work::name_of(&p.from);
        match p.clash {
            Some(c) => item(new).icon("dialog-warning-symbolic").detail(t!("folders-tools-rename-cannot", why = clash_words(c))),
            None if p.moves() => item(new).icon("document-edit-symbolic").detail(t!("folders-tools-rename-was", old = &old)),
            None => item(new).icon("text-x-generic-symbolic").detail(t!("folders-tools-rename-stays")),
        }
    }).collect();
    let (changes, clash) = b.tally();
    let list_h = (b.plan.len() as f32 * crate::toolkit::theme::sheet().row).min(PREVIEW_H);
    let place = b.items.first().and_then(|i| i.parent()).map(|p| short_path(p, home)).unwrap_or_default();
    vec![
        head(super::icons::stand_in::RENAME_MANY, t!("folders-tools-rename-title", n = b.items.len())),
        keyed(t!("folders-tools-rename-replace"), vec![
            text(&b.find, t!("folders-tools-rename-find-tip"), "rn-find", Field::Find).grab(turn).on_submit(tool(ToolMsg::Rename)),
            text(&b.with, t!("folders-tools-rename-with-tip"), "rn-with", Field::With).on_submit(tool(ToolMsg::Rename)),
        ]),
        keyed("", vec![
            flag(b.wildcard, t!("folders-tools-rename-wildcard-tip"), Flag::Wildcard), label(t!("folders-tools-rename-wildcard")),
            flag(b.match_case, t!("folders-tools-rename-match-case-tip"), Flag::MatchCase), label(t!("folders-tools-rename-match-case")),
        ]),
        keyed(t!("folders-tools-rename-add"), vec![
            text(&b.before, t!("folders-tools-rename-before-tip"), "rn-before", Field::Before),
            text(&b.after, t!("folders-tools-rename-after-tip"), "rn-after", Field::After),
        ]),
        keyed(t!("folders-tools-rename-number"), number),
        labelled(t!("folders-tools-rename-letters"), vec![choice(cases().iter().map(|s| s.to_string()).collect(), b.case, t!("folders-tools-rename-case-tip"), |i| tool(ToolMsg::Case(i)))]),
        labelled(t!("folders-tools-rename-ending"), ending),
        scroll("rename-preview", list(rows, None, |_| tool(ToolMsg::Stay))).height(Length::Fixed(list_h)),
        note(t!("folders-tools-rename-summary", clash = if clash { "yes" } else { "no" }, changes = changes, total = b.plan.len(), place = &place)),
        foot(Vec::new(), act(super::icons::stand_in::RENAME_MANY, t!("folders-tools-rename-button"), ToolMsg::Rename, changes == 0 || clash)),
    ]
}

fn dupes_view(d: &Dupes, home: &Path) -> Vec<Node<Msg>> {
    let one = match d.roots.as_slice() {
        [one] if one == home => t!("folders-tools-home").to_string(),
        [one] => work::name_of(one),
        _ => String::new(),
    };
    let what = t!("folders-tools-dupes-title", n = d.roots.len(), folder = &one);
    let mut out = vec![head(super::icons::stand_in::DUPES, what)];
    match &d.found {
        None => {
            out.push(progress(d.progress.0, d.progress.1, t!("folders-tools-listing-files")));
        }
        Some(Err(e)) => {
            out.push(stopped(label(t!("folders-tools-dupes-stopped", error = e)).tone(Tone::Problem)));
        }
        Some(Ok(found)) if found.groups.is_empty() => {
            out.push(label(t!("folders-tools-dupes-none")));
            if !found.unread.is_empty() {
                out.push(stopped(note(t!("folders-tools-dupes-unread", n = found.unread.len()))));
            }
        }
        Some(Ok(found)) => {
            let spare: u64 = found.groups.iter().map(dupes::Group::spare).sum();
            out.push(note(t!("folders-tools-dupes-sets", n = found.groups.len(), size = &size_words(spare))));
            let rows = d.rows.iter().map(|(g, path)| {
                let size = found.groups.get(*g).map_or(0, |grp| grp.size);
                let at = path.parent().map(|p| short_path(p, home)).unwrap_or_default();
                item(work::name_of(path)).icon("text-x-generic-symbolic").detail(t!("folders-tools-dupes-row", set = g + 1, size = &size_words(size), at = &at))
            }).collect();
            out.push(scroll("duplicates", list(rows, None, |_| tool(ToolMsg::Stay)).many(d.picked.clone(), |p| tool(ToolMsg::Picked(p)))).height(Length::Fixed(LIST_H)));
            if !found.unread.is_empty() {
                out.push(note(t!("folders-tools-dupes-unread-left-out", n = found.unread.len())));
            }
            let whole = takes_a_whole_set(&d.rows, &d.picked);
            if whole {
                out.push(note(t!("folders-tools-dupes-keep-one")));
            }
            out.push(foot(
                vec![button("edit-select-all-symbolic", t!("folders-tools-dupes-pick-extras")).on_press(tool(ToolMsg::PickExtras))],
                main_action("user-trash-symbolic", t!("folders-tools-dupes-trash")).on_press(tool(ToolMsg::TrashPicked)).off(d.picked.is_empty() || whole),
            ));
        }
    }
    out
}

fn diff_view(d: &Diffing) -> Vec<Node<Msg>> {
    use super::diff::Outcome;
    let names = row(vec![
        label(work::name_of(&d.left)).cut().width(Length::Fill),
        label(work::name_of(&d.right)).cut().width(Length::Fill),
    ]).gap(8.0).width(Length::Fill);
    let mut out = vec![head(super::icons::stand_in::COMPARE, t!("folders-tools-diff-title")), names];
    match &d.got {
        None => out.push(progress(0, 0, t!("folders-tools-comparing"))),
        Some(Err(e)) => out.push(stopped(label(t!("folders-tools-diff-failed", error = e)).tone(Tone::Problem))),
        Some(Ok(Outcome::Bytes { first, sizes })) => out.push(label(match first {
            None => t!("folders-tools-diff-same-bytes").to_string(),
            Some(at) if *at == sizes.0.min(sizes.1) => t!("folders-tools-diff-same-as-shorter", one = &size_words(sizes.0), other = &size_words(sizes.1)),
            Some(at) => t!("folders-tools-diff-from-byte", n = at + 1),
        })),
        Some(Ok(Outcome::Text { lines, changes, cut, whole })) => {
            out.push(note(t!("folders-tools-diff-lines", n = *changes)));
            if !whole {
                out.push(note(t!("folders-tools-diff-too-different")));
            }
            if *cut {
                out.push(note(t!("folders-tools-diff-cut")));
            }
            if *changes > 0 {
                out.push(scroll("differences", super::diff::view(lines)).height(Length::Fixed(DIFF_H)));
            }
        }
    }
    out
}

fn compare_view(c: &Comparing) -> Vec<Node<Msg>> {
    let mut out = vec![
        head(super::icons::stand_in::COMPARE, t!("folders-tools-compare-title", left = &work::name_of(&c.left), right = &work::name_of(&c.right))),
        row(vec![
            label(t!("folders-tools-compare-bytes")), switch(c.content, t!("folders-tools-compare-bytes-tip"), |on| tool(ToolMsg::Flag(Flag::Content, on))),
            space().width(Length::Fill),
            label(t!("folders-tools-compare-show-same")), switch(c.all, t!("folders-tools-compare-show-same-tip"), |on| tool(ToolMsg::Flag(Flag::All, on))),
        ]).gap(8.0).width(Length::Fill),
    ];
    let mut right = 0;
    let mut left = 0;
    match &c.rows {
        None => out.push(progress(c.progress.0, c.progress.1, t!("folders-tools-looking"))),
        Some(Err(e)) => out.push(stopped(label(t!("folders-tools-compare-stopped", error = e)).tone(Tone::Problem))),
        Some(Ok(rows)) => {
            let count = |f: &dyn Fn(&State) -> bool| rows.iter().filter(|r| f(&r.state)).count();
            let (only_left, only_right) = (count(&|s| *s == State::OnlyLeft), count(&|s| *s == State::OnlyRight));
            let (different, same) = (count(&|s| matches!(s, State::Different(_))), count(&|s| *s == State::Same));
            out.push(note(t!("folders-tools-compare-counts", left = only_left, right = only_right, different = different, same = same)));
            let shown: Vec<_> = rows.iter().filter(|r| c.all || r.state != State::Same).map(|r| {
                let (icon, words) = row_words(r);
                item(r.name.to_string_lossy().into_owned()).icon(icon).detail(words)
            }).collect();
            if shown.is_empty() {
                out.push(label(t!("folders-tools-compare-all-same")));
            } else {
                out.push(scroll("compared", list(shown, None, |_| tool(ToolMsg::Stay))).height(Length::Fixed(LIST_H)));
            }
            right = to_copy(rows, true).values().map(Vec::len).sum();
            left = to_copy(rows, false).values().map(Vec::len).sum();
        }
    }
    out.push(buttons(vec![
        button("view-refresh-symbolic", t!("folders-tools-compare-again")).on_press(tool(ToolMsg::Again)),
        button("go-previous-symbolic", t!("folders-tools-compare-copy-left", n = left)).on_press(tool(ToolMsg::CopyOver(false))).off(left == 0),
        button("go-next-symbolic", t!("folders-tools-compare-copy-right", n = right)).on_press(tool(ToolMsg::CopyOver(true))).off(right == 0),
    ]));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::places::Where;
    use crate::toolkit::ui::App;

    #[test]
    fn wipe_securely_is_the_red_button_and_no_other_tool_has_one() {
        let wipe = Tool::Wipe(WipeAsk { items: vec![PathBuf::from("/home/john/notes.txt")], caveats: Vec::new(), passes: 0, left_out: Vec::new() });
        assert_eq!(view(&wipe, 0).for_good(), ["Wipe securely"]);
        assert!(act("x", "Resize", ToolMsg::Resize, false).for_good().is_empty());
    }

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("trindle-files-tools-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn files(d: &Path) -> Files {
        let mut f = Files::new(vec![d.to_path_buf()]);
        f.update(Msg::Add(Vec::new()));
        f.bin = Some(super::super::trash::Bin::at(&d.join("Trash"), None));
        f.bin.as_ref().unwrap().ready().unwrap();
        f
    }

    #[test]
    fn the_wipe_window_names_the_folders_it_leaves_out() {
        let d = dir("wipe-left");
        std::fs::write(d.join("note.txt"), b"x").unwrap();
        std::fs::create_dir_all(d.join("Letters")).unwrap();
        let mut f = files(&d);
        f.ask_wipe(vec![d.join("note.txt"), d.join("Letters")]);
        let Some(super::super::dialogs::Dialog::Tool(tool)) = &f.dialog else { panic!("no wipe window") };
        let Tool::Wipe(w) = tool else { panic!() };
        assert_eq!(w.items, [d.join("note.txt")]);
        assert!(view(tool, 0).said().iter().any(|l| l.contains("Letters") && l.contains("left as they are")), "{:?}", view(tool, 0).said());
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn names_and_numbers_are_worked_out() {
        assert_eq!(pack_name(&[PathBuf::from("/data/a/report.pdf")], Path::new("/data/a")), "report");
        assert_eq!(pack_name(&[PathBuf::from("/data/a/x"), PathBuf::from("/data/a/y")], Path::new("/data/a")), "a");
        assert_eq!(pad_for(1, 9), 1);
        assert_eq!(pad_for(1, 10), 2);
        assert_eq!(pad_for(95, 10), 3);
        let rows = vec![(0, PathBuf::from("a")), (0, PathBuf::from("b")), (1, PathBuf::from("c")), (1, PathBuf::from("d")), (1, PathBuf::from("e"))];
        assert_eq!(extras(&rows), [1, 3, 4]);
        assert!(!takes_a_whole_set(&rows, &[1, 3, 4]));
        assert!(takes_a_whole_set(&rows, &[0, 1]));
    }

    #[test]
    fn pictures_are_resized_from_the_menu_and_undone() {
        let d = dir("resize");
        image::DynamicImage::new_rgb8(40, 20).save(d.join("wide.png")).unwrap();
        let mut f = files(&d);
        f.go(0, Where::Folder(d.clone()));
        f.do_act(super::super::menu::Act::Resize, 0, vec![d.join("wide.png")]);
        assert!(matches!(&f.dialog, Some(Dialog::Tool(Tool::Resize { size: 1, .. }))));
        f.update(tool(ToolMsg::Size(0)));
        f.update(tool(ToolMsg::Resize));
        let copy = d.join("wide (10\u{d7}5).png");
        assert_eq!(image::image_dimensions(&copy).unwrap(), (10, 5));
        assert_eq!(image::image_dimensions(d.join("wide.png")).unwrap(), (40, 20));
        f.update(Msg::Undo);
        assert!(!copy.exists());
        f.do_act(super::super::menu::Act::TurnRight, 0, vec![d.join("wide.png")]);
        assert_eq!(image::image_dimensions(d.join("wide.png")).unwrap(), (20, 40));
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn two_files_are_compared_from_the_menu() {
        let d = dir("diff");
        std::fs::write(d.join("a.txt"), "one\ntwo\nthree\n").unwrap();
        std::fs::write(d.join("b.txt"), "one\n2\nthree\n").unwrap();
        let mut f = files(&d);
        f.do_act(super::super::menu::Act::CompareFiles, 0, vec![d.join("a.txt"), d.join("b.txt")]);
        let Some(Dialog::Tool(Tool::Diff(diff))) = &f.dialog else { panic!("no compare window") };
        assert!(matches!(&diff.got, Some(Ok(super::super::diff::Outcome::Text { changes: 1, .. }))));
        assert!(diff.width >= WIDE && diff.width <= DIFF_W);
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn many_are_renamed_and_put_back() {
        let d = dir("rename");
        for n in ["a.txt", "b.txt"] {
            std::fs::write(d.join(n), b"x").unwrap();
        }
        let mut f = files(&d);
        f.batch_rename(vec![d.join("a.txt"), d.join("b.txt")]);
        f.tool_msg(ToolMsg::Text(Field::Before, "Old ".into()));
        f.tool_msg(ToolMsg::Ending(2));
        f.tool_msg(ToolMsg::Rename);
        assert!(f.dialog.is_none());
        assert!(d.join("Old a.TXT").exists() && d.join("Old b.TXT").exists());
        assert_eq!(f.undo.len(), 1);
        f.update(Msg::Undo);
        assert!(d.join("a.txt").exists() && d.join("b.txt").exists());
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_clash_stops_the_rename() {
        let d = dir("clash");
        for n in ["a", "b"] {
            std::fs::write(d.join(n), b"x").unwrap();
        }
        let mut f = files(&d);
        f.batch_rename(vec![d.join("a"), d.join("b")]);
        f.tool_msg(ToolMsg::Text(Field::Find, "*".into()));
        f.tool_msg(ToolMsg::Flag(Flag::Wildcard, true));
        f.tool_msg(ToolMsg::Text(Field::With, "same".into()));
        f.tool_msg(ToolMsg::Rename);
        assert!(f.dialog.is_some(), "two would get one name, so nothing happens");
        assert!(d.join("a").exists());
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn duplicates_are_found_and_the_extras_trashed() {
        let d = dir("dupes");
        std::fs::create_dir_all(d.join("in")).unwrap();
        for n in ["one", "two", "in/three"] {
            std::fs::write(d.join(n), b"the same words").unwrap();
        }
        std::fs::write(d.join("other"), b"something else").unwrap();
        let mut f = files(&d);
        f.find_dupes(vec![d.join("in"), d.join("one"), d.join("two"), d.join("other")]);
        let Some(Dialog::Tool(Tool::Dupes(dp))) = &f.dialog else { panic!("no window") };
        assert_eq!(dp.rows.len(), 3);
        assert_eq!(dp.picked.len(), 2);
        f.tool_msg(ToolMsg::Picked(vec![0, 1, 2]));
        f.tool_msg(ToolMsg::TrashPicked);
        assert_eq!(std::fs::read_dir(&d).unwrap().count(), 5, "a whole set is never trashed");
        f.tool_msg(ToolMsg::PickExtras);
        f.tool_msg(ToolMsg::TrashPicked);
        let left = ["one", "two", "in/three"].iter().filter(|n| d.join(n).exists()).count();
        assert_eq!(left, 1);
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn the_panes_are_compared_and_copied_across() {
        let d = dir("compare");
        for side in ["l", "r"] {
            std::fs::create_dir_all(d.join(side).join("sub")).unwrap();
            std::fs::write(d.join(side).join("same"), b"x").unwrap();
        }
        std::fs::write(d.join("l/sub/new"), b"only left").unwrap();
        std::fs::write(d.join("r/right"), b"only right").unwrap();
        let mut f = files(&d);
        f.go(0, Where::Folder(d.join("l")));
        f.go(1, Where::Folder(d.join("r")));
        f.compare_panes();
        let Some(Dialog::Tool(Tool::Compare(c))) = &f.dialog else { panic!("no window") };
        let Some(Ok(rows)) = &c.rows else { panic!("not compared") };
        assert_eq!(to_copy(rows, true).get(Path::new("sub")).map(Vec::len), Some(1));
        f.tool_msg(ToolMsg::CopyOver(true));
        assert!(d.join("r/sub/new").exists());
        assert!(!d.join("l/right").exists());
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn archives_are_made_and_unpacked() {
        let d = dir("pack");
        std::fs::create_dir_all(d.join("box")).unwrap();
        std::fs::write(d.join("box/note"), b"hello").unwrap();
        let mut f = files(&d);
        f.pack(super::super::menu::Act::Compress, 0, vec![d.join("box")]);
        f.tool_msg(ToolMsg::Format(2));
        f.tool_msg(ToolMsg::Pack);
        assert!(d.join("box.tar.gz").exists());
        f.pack(super::super::menu::Act::Compress, 0, vec![d.join("box")]);
        f.tool_msg(ToolMsg::Format(2));
        f.tool_msg(ToolMsg::Pack);
        assert!(d.join("box (2).tar.gz").exists(), "a clash is numbered before the whole ending");
        std::fs::remove_dir_all(d.join("box")).unwrap();
        f.pack(super::super::menu::Act::ExtractHere, 0, vec![d.join("box.tar.gz")]);
        assert_eq!(std::fs::read(d.join("box/note")).unwrap(), b"hello");
        f.pack(super::super::menu::Act::ExtractTo, 0, vec![d.join("box.tar.gz"), d.join("box (2).tar.gz")]);
        let made: Vec<String> = std::fs::read_dir(&d).unwrap().flatten().filter(|e| e.path().is_dir())
            .map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        assert!(made.len() >= 3, "a folder for each archive beside the first: {made:?}");
        std::fs::remove_dir_all(&d).unwrap();
    }
}
