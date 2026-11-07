// What an act holds and says: a job, a question, the clipboard, a drag, and where things are let go.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use crate::files::app::Msg;
use crate::files::places;
use crate::files::work::{self, Answer};
use crate::toolkit::t;

pub struct Words {
    pub doing: String,
    pub done: String,
    pub verb: &'static str,
    pub help: &'static str,
    pub undo: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::files) enum Turn {
    New,
    Undo(String),
    Redo(String),
}

pub struct Job {
    pub id: u64,
    pub words: Words,
    pub total: u64,
    pub done: u64,
    pub paused: bool,
    pub asking: Option<Asking>,
    pub all: bool,
    pub slot: usize,
    pub pane: usize,
    pub pace: crate::files::progress::Pace,
    pub(in crate::files) pause: Arc<AtomicBool>,
    pub(in crate::files) stop: Arc<AtomicBool>,
    pub(in crate::files) answers: Option<Sender<(Answer, bool)>>,
    pub(in crate::files) turn: Turn,
    pub(in crate::files) thread: Option<std::thread::JoinHandle<()>>,
    pub(in crate::files) from: Vec<PathBuf>,
    pub began: std::time::Instant,
    pub(in crate::files) touched: Option<Vec<PathBuf>>,
    pub(in crate::files) again: Option<crate::files::locked::Then>,
    pub fonts: bool,
}

const HOLDS_MOST: usize = 10_000;

pub struct Asking {
    pub clash: work::Clash,
    pub coming: Option<places::Entry>,
    pub there: Option<places::Entry>,
    pub holds: Option<(u64, u64)>,
}

impl Asking {
    pub(in crate::files) fn new(clash: work::Clash, mime: &crate::files::kinds::Mime) -> Asking {
        let read = |p: &PathBuf| if crate::files::reach::answers(p) { places::Entry::read(p.clone(), mime) } else { None };
        let (coming, there) = match &clash.sides {
            Some((coming, there)) => (read(coming), read(there)),
            None => (None, None),
        };
        let held = |e: &Option<places::Entry>| e.as_ref().and_then(|e| { let p = e.path.clone(); crate::files::reach::within(&e.path, move || fs::read_dir(&p).ok().map_or(0, |r| r.take(HOLDS_MOST).count() as u64)) }).unwrap_or(0);
        let holds = clash.folders.then(|| (held(&coming), held(&there)));
        Asking { clash, coming, there, holds }
    }

    pub(in crate::files) fn holds_words(n: u64) -> String {
        if n >= HOLDS_MOST as u64 { t!("folders-act-holds-most").to_string() } else { crate::files::view::items_words(n) }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Busy {
    pub running: usize,
    pub asking: bool,
    pub trouble: bool,
}

#[cfg(test)]
impl Job {
    pub(in crate::files) fn sample(id: u64, doing: &str, total: u64, done: u64) -> Job {
        let ended = doing.replacen("Copying", "Copied", 1).replacen("Moving", "Moved", 1).replacen("Extracting", "Extracted", 1);
        Job { id, words: Words { doing: doing.into(), done: format!("{ended}."), verb: "could not be copied", help: crate::files::help::WORK_NOT_DONE, undo: String::new() },
              total, done, paused: false, asking: None, all: false, slot: (id as usize).saturating_sub(1), pane: 0,
              pace: crate::files::progress::Pace::new(crate::files::progress::Unit::Bytes),
              pause: Arc::default(), stop: Arc::default(), answers: None,
              turn: Turn::New, thread: None, from: Vec::new(), began: std::time::Instant::now(), touched: None, again: None, fonts: false }
    }
}

impl Job {
    pub fn share(&self) -> f32 {
        crate::common::gentle::share(self.done, self.total, self.pace.files_counted(), false)
    }
}

#[derive(Clone, Debug)]
pub struct Clip {
    pub paths: Vec<PathBuf>,
    pub cut: bool,
}
#[derive(Clone, Debug)]
pub struct Drag {
    pub start: (f32, f32),
    pub pane: usize,
    pub items: Vec<PathBuf>,
    pub moving: bool,
    pub then: Option<usize>,
    pub right: bool,
    pub pins: bool,
    pub was: Vec<Option<crate::files::proof::Picked>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Land {
    Home,
    Pinned(usize),
    Place(usize),
    Drive(usize),
    Share(usize),
    Tab(usize, usize),
    Part(usize, usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Way {
    Copy,
    Move,
    Usual,
    Ask,
}
pub struct Ask {
    pub title: String,
    pub folder: Option<PathBuf>,
    pub directory: bool,
}
pub struct Pick {
    pub ask: Ask,
    pub then: Box<dyn FnOnce(Vec<PathBuf>) -> Option<Msg>>,
}
