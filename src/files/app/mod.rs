//! What Files holds and how it answers: tabs of two panes, what each pane

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::os::fd::RawFd;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

use super::act::{Clip, Drag, Job, Way, DRAG_SLOP};
use super::inplace::{slow_second, Renaming, Slow, TWICE};
use super::icons;
use super::kinds::Mime;
use super::places::{self, Entry, Key, Where};
use super::trash::Bin;
use super::work::{self, Answer, News};
use crate::common::files::{data_home, home};
use crate::toolkit::event::Post;
use crate::toolkit::input::{Button, Input, Keysym, Mods};
use crate::toolkit::popup::PopupMsg;
use crate::toolkit::t;
use crate::toolkit::ui::{App, Keyboard};
use crate::toolkit::watch::Watch;
use crate::toolkit::widget::{Node, Pointer};

mod handle;
mod keys;
mod listing;
mod msg;
mod navigate;
mod pointer;
mod sidebar;
mod tabs;
#[cfg(test)]
mod tests;
mod window;

pub use msg::Msg;
#[cfg(test)]
use keys::key;

#[cfg(test)]
pub(super) const SOURCE: &str = concat!(
    include_str!("mod.rs"), include_str!("handle.rs"), include_str!("keys.rs"), include_str!("listing.rs"), include_str!("msg.rs"),
    include_str!("navigate.rs"), include_str!("pointer.rs"), include_str!("sidebar.rs"), include_str!("tabs.rs"), include_str!("window.rs"),
);

const GUESS_TWO: f32 = 440.0;
const GUESS_ONE: f32 = 900.0;
const HISTORY: usize = 50;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    Icons,
    List,
    Compact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Left,
    Right,
    Up,
    Down,
    First,
    Last,
}

#[derive(Clone, Debug)]
pub struct Band {
    pub from: (f32, f32),
    pub to: (f32, f32),
    base: Vec<usize>,
}

pub struct Pane {
    pub at: Where,
    pub entries: Vec<Entry>,
    pub chosen: Option<usize>,
    pub picked: Vec<usize>,
    anchor: Option<usize>,
    pub band: Option<Band>,
    pub width: Rc<Cell<f32>>,
    pub grid: icons::Held,
    pub groups: Vec<(usize, usize, String)>,
    pub tall: Rc<Cell<f32>>,
    pub seen: Rc<super::pictures::Seen>,
    pub hover: Option<usize>,
    pub(super) last: Option<(Instant, usize)>,
    pub(super) stayed: bool,
    pub name_at: Rc<Cell<Option<(usize, crate::toolkit::widget::Rect)>>>,
    pub filter: String,
    pub narrow: super::narrow::Narrow,
    pub all: Vec<Entry>,
    pub hunt: Option<super::found::Hunt>,
    pub back: Vec<Where>,
    pub ahead: Vec<Where>,
    pub seek: u32,
    pub(super) before: Vec<usize>,
    pub(super) pointed: Option<(f32, f32)>,
    pub nudge: (u32, f32),
    pub(super) floor: Cell<f32>,
    pub error: Option<String>,
    pub free: Option<u64>,
    pub(super) read: u64,
    pub(super) reading: bool,
    pub(super) late: bool,
}

impl Pane {
    fn new(at: Where) -> Pane {
        Pane { at, entries: Vec::new(), chosen: None, picked: Vec::new(), anchor: None, band: None,
               width: Rc::new(Cell::new(GUESS_TWO)), grid: icons::Held::new(icons::Grid::icons(GUESS_TWO)), groups: Vec::new(),
               tall: Rc::new(Cell::new(0.0)), seen: Rc::default(), hover: None, last: None, stayed: false, name_at: Rc::default(),
               filter: String::new(), narrow: Default::default(), all: Vec::new(), hunt: None, back: Vec::new(), ahead: Vec::new(), seek: 0, before: Vec::new(), pointed: None, nudge: (0, 0.0), floor: Cell::new(0.0), error: None, free: None, read: 0, reading: false, late: false }
    }

    pub(super) fn pick(&mut self, i: usize, run: bool) {
        if run {
            self.picked = places::run(self.anchor.or(self.chosen).unwrap_or(i), i);
        } else {
            self.picked = vec![i];
            self.anchor = Some(i);
        }
        self.chosen = Some(i);
    }

    pub(super) fn clear(&mut self) {
        self.chosen = None;
        self.picked.clear();
        self.anchor = None;
        self.band = None;
        self.hover = None;
        self.last = None;
    }
}

pub(super) enum Picked<'a> {
    Few(&'a [usize]),
    Many(HashSet<usize>),
}

impl Picked<'_> {
    pub(super) fn of(picked: &[usize]) -> Picked<'_> {
        if picked.len() <= 16 { Picked::Few(picked) } else { Picked::Many(picked.iter().copied().collect()) }
    }

    pub(super) fn has(&self, i: usize) -> bool {
        match self {
            Picked::Few(list) => list.contains(&i),
            Picked::Many(set) => set.contains(&i),
        }
    }
}

pub struct Column {
    pub tabs: Vec<Pane>,
    pub tab: usize,
}

impl Column {
    pub(super) fn new(at: Where) -> Column {
        Column { tabs: vec![Pane::new(at)], tab: 0 }
    }

    pub(super) fn open(&mut self, at: Where) {
        self.open_behind(at);
        self.tab += 1;
    }

    pub(super) fn open_behind(&mut self, at: Where) {
        let pane = Pane::new(at);
        pane.width.set(self.width.get());
        self.tabs.insert(self.tab + 1, pane);
    }
}

impl std::ops::Deref for Column {
    type Target = Pane;
    fn deref(&self) -> &Pane {
        &self.tabs[self.tab]
    }
}

impl std::ops::DerefMut for Column {
    fn deref_mut(&mut self) -> &mut Pane {
        &mut self.tabs[self.tab]
    }
}

pub struct Files {
    pub home: PathBuf,
    pub cols: [Column; 2],
    pub side: usize,
    pub two: bool,
    pub hidden: bool,
    pub view: View,
    pub sort: Key,
    pub by: Option<super::columns::Col>,
    pub up: bool,
    pub pinned: Vec<PathBuf>,
    pub said: String,
    pub(super) said_help: Option<&'static str>,
    #[cfg(test)]
    pub(super) helped: Vec<&'static str>,
    pub(super) reasons: Option<(String, Vec<String>)>,
    pub(super) reasons_open: bool,
    pub turn: u32,
    pub(super) mime: std::sync::Arc<Mime>,
    pub(super) watch: Option<Watch>,
    pub(super) watched: HashSet<PathBuf>,
    pub(super) reads: u64,
    pub(super) trash_dirs: Vec<PathBuf>,
    news: RefCell<Vec<PathBuf>>,
    pub(super) mods: Cell<Mods>,
    pub(super) post: Option<Post<Msg>>,
    pub(super) speak: Option<Post<String>>,
    pub(super) menu_told: String,
    pub jobs: Vec<Job>,
    pub ended: Vec<super::progress::Ended>,
    pub(super) next_job: u64,
    pub undo: Vec<(String, Vec<work::Step>)>,
    pub redo: Vec<(String, Vec<work::Step>)>,
    pub clip: Option<Clip>,
    pub cut_note: Option<PathBuf>,
    pub confirm: Option<Vec<PathBuf>>,
    pub asked: Vec<super::proof::Asked>,
    pub emptying: bool,
    pub renaming: Option<Renaming>,
    pub slow: Option<Slow>,
    pub(super) slows: u64,
    pub(super) listed: Option<super::far::Listed>,
    pub(super) lines_tall: Rc<Cell<Option<f32>>>,
    pub drag: Option<Drag>,
    pub(super) outside: super::outside::State,
    pub notice: Option<(u64, String)>,
    pub(super) notices: u64,
    pub dropping: Option<(Vec<PathBuf>, Way)>,
    pub(super) landed: Option<Where>,
    pub(super) pin_at: Option<usize>,
    pub(super) pins_file: Option<PathBuf>,
    pub(super) own_file: Option<PathBuf>,
    pub bad: bool,
    pub link_note: Option<String>,
    pub(super) pointer: Cell<(f32, f32)>,
    pub(super) bin: Option<Bin>,
    pub(super) jobs_hook: Option<Box<dyn Fn(super::act::Busy)>>,
    pub(super) pick_hook: Option<Box<dyn Fn(super::act::Pick)>>,
    pub(super) close_hook: Option<Box<dyn Fn()>>,
    pub(super) choosing: Option<super::choose::Choosing>,
    pub(super) closes: u32,
    pub kept: super::kept::Kept,
    pub cache: super::columns::Cache,
    pub menu: Option<super::menu::Menu>,
    pub dialog: Option<super::dialogs::Dialog>,
    pub far: super::far::Far,
    pub inside: super::inside::Opened,
    pub(super) unlocks: super::locked::Waiting,
    pub(super) room: (f32, f32),
    pub(super) sizes: super::sized::Measured,
    pub(super) media: super::media::Reading,
    pub(super) neighbours: Vec<super::neighbours::Computer>,
    pub(super) near: super::neighbours::Near,
    pub(super) shares: super::shares::Shares,
    pub(super) drives: super::drives::Drives,
    pub(super) sharing: super::sharing::Sharing,
    pub find: super::found::Finding,
    pub locate: Option<(usize, String)>,
    pub(super) locate_line: bool,
    pub(super) pin_names: std::collections::HashMap<PathBuf, String>,
    pub(super) pin_rename: Option<(PathBuf, String)>,
    pub for_a_call: bool,
    pub seeking: u32,
    pub(super) pictures: super::pictures::Pictures,
    pub(in crate::files) as_root: bool,
    pub(super) ahead: crate::toolkit::ahead::Typed,
    pub(super) head_down: Option<(usize, super::heading::Part)>,
    pub(super) sizing: Option<super::heading::Sizing>,
    pub(super) edge_last: Option<(Instant, usize, usize)>,
    pub(super) nudging: u32,
    pub(super) wheel: Cell<f32>,
    pub ticks: bool,
    pub(super) touch: Cell<bool>,
    pub endings: bool,
    pub density: super::density::Density,
    pub(super) shown: super::shown::Shown,
    pub(super) below: super::below::Below,
    pub(super) ring: Option<PathBuf>,
    pub(super) parts: super::alone::Parts,
    pub(super) panel: super::panel::Panel,
    pub(super) computer: super::computer::Computer,
    pub(super) earlier: super::earlier::Earlier,
}

impl Files {
    pub fn new(folders: Vec<PathBuf>) -> Files {
        let home = home();
        #[cfg(test)]
        let (start, mut kept, pins) = ((Where::Recent, Where::Folder(home.clone())), super::kept::Kept::default(), Vec::new());
        #[cfg(test)]
        {
            kept.dual = true;
        }
        #[cfg(not(test))]
        let (start, kept, pins) = (super::kept::start_places(&home), super::kept::Kept::load(), places::load_pins(&places::pins_file(), &home));
        #[allow(unused_mut)]
        let mut files = Files::with(folders, home, start, kept, pins);
        #[cfg(not(test))]
        {
            files.pins_file = Some(places::pins_file());
            files.pin_names = places::read_names(&places::names_beside(&places::pins_file()));
            files.shares.remembered_file = Some(super::remembered::file());
            files.shares.remembered = super::remembered::read(&super::remembered::file());
            files.own_file = Some(crate::common::folders_conf::ready());
            files.read_settings();
        }
        files
    }

    pub(super) fn with(folders: Vec<PathBuf>, home: PathBuf, start: (Where, Where), kept: super::kept::Kept, pinned: Vec<PathBuf>) -> Files {
        let mut cols = match folders.is_empty() {
            true => [Column::new(start.0), Column::new(start.1)],
            false => [Column { tabs: folders.into_iter().map(|f| Pane::new(super::inside::place(f))).collect(), tab: 0 },
                      Column::new(Where::Folder(home.clone()))],
        };
        let two = kept.dual;
        let guess = if two { GUESS_TWO } else { GUESS_ONE };
        for pane in cols.iter_mut().flat_map(|c| c.tabs.iter_mut()) {
            pane.width.set(guess);
            pane.grid.set(icons::Grid::icons(guess));
        }
        let mut files = Files {
            home, cols, side: 0, two, hidden: false, view: View::List, sort: Key::Name, by: None, up: true,
            pinned, said: String::new(), said_help: None, #[cfg(test)] helped: Vec::new(), reasons: None, reasons_open: false, turn: 1,
            mime: std::sync::Arc::new(Mime::load()), watch: Watch::new().ok(), watched: HashSet::new(), reads: 0, trash_dirs: Vec::new(), news: RefCell::new(Vec::new()),
            mods: Cell::new(Mods::default()), post: None, speak: None, menu_told: String::new(), jobs: Vec::new(), ended: Vec::new(), next_job: 1, undo: Vec::new(), redo: Vec::new(), clip: None,
            confirm: None, cut_note: None, asked: Vec::new(), emptying: false, renaming: None, slow: None, slows: 0, listed: None, lines_tall: Rc::default(), drag: None, outside: Default::default(), notice: None, notices: 0, dropping: None, landed: None, pin_at: None, pins_file: None, own_file: None, bad: false, link_note: None, pointer: Cell::new((0.0, 0.0)), bin: None,
            jobs_hook: None, pick_hook: None, close_hook: None, choosing: None, closes: 0, kept, cache: Default::default(), menu: None, dialog: None, far: super::far::Far::new(),
            inside: Default::default(), unlocks: Default::default(), sizes: Default::default(), media: Default::default(), neighbours: Vec::new(), near: Default::default(), shares: Default::default(), drives: Default::default(), sharing: Default::default(), find: Default::default(), pictures: Default::default(),
            locate: None, locate_line: false, pin_names: Default::default(), pin_rename: None, for_a_call: false, seeking: 0, room: (1200.0, 760.0), ahead: Default::default(),
            head_down: None, sizing: None, edge_last: None, nudging: 0, wheel: Cell::new(0.0), ticks: false, touch: Cell::new(false), endings: true, density: Default::default(), shown: Default::default(), below: Default::default(), ring: None, parts: super::alone::Parts::all(),
            panel: Default::default(), computer: Default::default(), earlier: Default::default(),
            as_root: unsafe { libc::geteuid() } == 0,
        };
        files.read_settings();
        files.watch_also(&data_home());
        files.shares.refresh();
        #[cfg(test)]
        files.first_reads();
        files.computer_wanted();
        files
    }

    pub(super) fn read_settings(&mut self) {
        self.read_own();
        #[cfg(not(test))]
        {
            let ring = crate::common::favourites::file();
            self.ring = crate::common::favourites::ring_there(&ring).then_some(ring);
            self.parts = super::alone::Parts::look();
        }
    }

    pub fn shown_name<'a>(&self, e: &'a Entry) -> &'a str {
        self.shown.name(e, self.endings)
    }

    pub fn watch_fd(&self) -> Option<RawFd> {
        self.watch.as_ref().map(Watch::fd)
    }

    pub fn saw(&self) -> bool {
        let Some(w) = &self.watch else { return false };
        let changed = w.changed();
        let shown = changed.iter().any(|p| self.cols.iter().flat_map(|c| c.tabs.iter()).any(|pane| listing::touches(&pane.at, p, &self.trash_dirs)));
        let saved = changed.iter().any(|p| self.far.fetched.contains_key(p));
        if shown || saved {
            self.news.borrow_mut().extend(changed);
        }
        shown || saved
    }

    pub fn pane(&self) -> &Pane {
        &self.cols[self.side]
    }

    pub fn side(&self) -> usize {
        self.side
    }

    pub fn view_of(&self, p: usize) -> View {
        self.kept.view_of(&self.cols[p].at).unwrap_or(self.view)
    }

    pub fn tint_of(&self, path: &Path) -> Option<crate::toolkit::colour::Colour> {
        colour_named(self.kept.colours.get(path)?)
    }

    pub(in crate::files) fn off_thread<T: Send + 'static>(&mut self, name: &'static str, job: impl FnOnce() -> T + Send + 'static, done: impl FnOnce(T) -> Msg + Send + 'static) {
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name(name.into()).spawn(move || post.send(done(job())));
            }
            None => {
                use crate::toolkit::ui::App;
                self.update(done(job()));
            }
        }
    }

    pub fn closing(&self) {
        self.stop_all();
        self.unlocks.tidy();
        self.far.forget_copies();
        #[cfg(not(test))]
        {
            super::kept::save_last(&self.cols[0].at, &self.cols[1].at);
        }
    }

    pub fn is_cut(&self, path: &std::path::Path) -> bool {
        self.clip.as_ref().is_some_and(|c| c.cut && c.paths.iter().any(|p| p == path))
    }

    pub(super) fn pane_mut(&mut self, p: usize) -> &mut Pane {
        &mut self.cols[p]
    }

    pub(super) fn watch_ear(&self) -> Option<std::os::fd::OwnedFd> {
        self.watch.as_ref().and_then(Watch::lend)
    }
}

pub(super) fn colour_named(name: &str) -> Option<crate::toolkit::colour::Colour> {
    let hex = crate::common::style::ACCENTS.iter()
        .find(|(_, n)| n.eq_ignore_ascii_case(name)).map(|(h, _)| *h).unwrap_or(name);
    crate::toolkit::colour::Colour::parse(hex)
}
