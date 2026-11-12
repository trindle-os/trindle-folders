//! The walk of a piece of work that reaches a server: finding each thing, asking what must be asked, and handing each file to the crew.

use std::collections::HashSet;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Instant;

use crate::toolkit::t;

use crate::files::proof;
use crate::files::remote::{self, Error, Scheme};
use crate::files::work::{self, Ctl, Kind, News, Over, Plan, Step, NAP, TELL_GAP};
use crate::common::files::is_changed;
use crate::common::gentle;
use super::ends::{End, Found, Got, Info, Up, claim_key, root_of};
use super::hands::{Hands, Waiting};
use super::{address, is_far};

pub(super) struct Count<'a> {
    pub(super) ctl: &'a Ctl,
    pub(super) tell: &'a dyn Fn(News),
    pub(super) done: &'a AtomicU64,
    pub(super) across: &'a AtomicU64,
    pub(super) files: u64,
    pub(super) told: Instant,
}

impl Count<'_> {
    fn tick(&mut self) -> bool {
        while self.ctl.pause.load(Ordering::Relaxed) && !self.ctl.stop.load(Ordering::Relaxed) {
            std::thread::sleep(NAP);
        }
        if self.told.elapsed() >= TELL_GAP {
            self.say();
        }
        !self.ctl.stop.load(Ordering::Relaxed)
    }

    fn say(&mut self) {
        if self.files > 0 {
            (self.tell)(News::Files(self.across.load(Ordering::Relaxed), self.files));
        }
        (self.tell)(News::Done(self.done.load(Ordering::Relaxed)));
        self.told = Instant::now();
    }
}

pub(super) struct Task {
    pub(super) from: End,
    pub(super) found: Found,
    pub(super) to_dir: End,
    pub(super) target: End,
    pub(super) name: String,
    pub(super) over: Option<Info>,
    pub(super) moving: bool,
    pub(super) up: Option<Arc<File>>,
    pub(super) undo_made: bool,
    pub(super) came: mpsc::Sender<Outcome>,
}

pub(super) struct Outcome {
    pub(super) got: Got,
    pub(super) failed: Vec<String>,
    pub(super) undo: Vec<Step>,
    pub(super) stopped: bool,
}

pub(super) struct Slot {
    pub(super) failed_at: usize,
    pub(super) undo_at: usize,
    pub(super) came: mpsc::Receiver<Outcome>,
    pub(super) out: Option<Outcome>,
    pub(super) key: String,
}

pub(super) fn at_once(plan: &Plan) -> usize {
    #[cfg(test)]
    if let Some(n) = std::env::var("TRINDLE_TEST_AT_ONCE").ok().and_then(|n| n.parse().ok()) {
        return n;
    }
    let all = || plan.items.iter().chain(std::iter::once(&plan.to));
    if all().any(|p| address(p).is_some_and(|a| a.scheme == Scheme::Mtp)) {
        return 1;
    }
    let spare = std::env::temp_dir();
    let mut here: Vec<&Path> = all().filter(|p| !is_far(p)).map(PathBuf::as_path).collect();
    if here.is_empty() {
        here.push(&spare);
    }
    work::threads(&here)
}

pub(super) struct Go<'a> {
    pub(super) hands: Hands<'a>,
    pub(super) ctl: &'a Ctl,
    pub(super) tell: &'a dyn Fn(News),
    pub(super) count: Count<'a>,
    pub(super) same: work::Same,
    pub(super) over: Over,
    pub(super) bench: &'a work::Bench<Task>,
    pub(super) slots: Vec<Slot>,
    pub(super) claimed: HashSet<String>,
}

impl<'a> std::ops::Deref for Go<'a> {
    type Target = Hands<'a>;

    fn deref(&self) -> &Hands<'a> {
        &self.hands
    }
}

pub fn run(plan: &Plan, ctl: &Ctl, tell: &dyn Fn(News)) -> Over {
    let (done, across) = (AtomicU64::new(0), AtomicU64::new(0));
    let held: Waiting = gentle::Held::default();
    let hands = Hands { links: &ctl.links, pause: &ctl.pause, stop: &ctl.stop, bin: ctl.bin.as_ref(), done: &done, across: &across, held: &held };
    let ends: Vec<Option<End>> = plan.items.iter().map(|p| End::of(p)).collect();
    let each = |task: Task| {
        let came = task.came.clone();
        let _ = came.send(hands.bring(task));
    };
    let over = work::crew(at_once(plan), &each, |bench| {
        let mut go = Go {
            hands,
            ctl,
            tell,
            count: Count { ctl, tell, done: &done, across: &across, files: 0, told: Instant::now() },
            same: work::Same::default(),
            over: Over::default(),
            bench,
            slots: Vec::new(),
            claimed: HashSet::new(),
        };
        match &plan.kind {
            Kind::Copy | Kind::Move => go.send(plan, &ends),
            Kind::Delete => go.delete(plan, &ends),
            _ => go.over.failed.push(t!("folders-far-cannot-do").into()),
        }
        go.gather();
        if matches!(plan.kind, Kind::Copy | Kind::Move) {
            let lost = held.settle(&|n| {
                done.fetch_add(n, Ordering::Relaxed);
            });
            for gentle::Lost { file, whose: at, why } in lost {
                proof::unmake(&file, &at);
                across.fetch_sub(1, Ordering::Relaxed);
                go.over.undo.retain(|step| !matches!(step, Step::Remove(made) if *made == at));
                go.over.failed.push(format!("{} ({})", work::name_of(&at), Error::from(why)));
            }
            if !is_far(&plan.to) && !go.over.stopped && gentle::all_out(&plan.to).is_err() {
                go.over.not_kept.push(work::name_of(&plan.to));
            }
            go.count.say();
        }
        go.over
    });
    ctl.links.rest();
    over
}

impl Go<'_> {
    pub(super) fn stopped(&mut self) -> bool {
        if !self.count.tick() {
            self.over.stopped = true;
        }
        self.over.stopped
    }

    pub(super) fn hand(&mut self, mut task: Task, came: mpsc::Receiver<Outcome>) -> usize {
        let key = claim_key(&task.target);
        loop {
            match self.bench.give_wait(task, gentle::AT_ONCE, TELL_GAP) {
                Ok(()) => break,
                Err(back) => {
                    task = back;
                    self.count.tick();
                }
            }
        }
        self.claimed.insert(key.clone());
        self.slots.push(Slot { failed_at: self.over.failed.len(), undo_at: self.over.undo.len(), came, out: None, key });
        self.slots.len() - 1
    }

    pub(super) fn wait(&mut self, slot: usize) -> Got {
        while self.slots[slot].out.is_none() {
            match self.slots[slot].came.recv_timeout(TELL_GAP) {
                Ok(out) => {
                    let key = std::mem::take(&mut self.slots[slot].key);
                    self.claimed.remove(&key);
                    self.slots[slot].out = Some(out);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    self.count.tick();
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    let name = remote::name_of(&self.slots[slot].key).to_string();
                    self.slots[slot].out = Some(Outcome { got: Got::Failed, failed: vec![name], undo: Vec::new(), stopped: false });
                }
            }
        }
        self.slots[slot].out.as_ref().map_or(Got::Failed, |out| out.got.clone())
    }

    pub(super) fn wait_all(&mut self) {
        for slot in 0..self.slots.len() {
            self.wait(slot);
        }
    }

    fn gather(&mut self) {
        self.wait_all();
        for slot in std::mem::take(&mut self.slots).into_iter().rev() {
            let Some(out) = slot.out else { continue };
            self.over.stopped |= out.stopped;
            self.over.failed.splice(slot.failed_at..slot.failed_at, out.failed);
            self.over.undo.splice(slot.undo_at..slot.undo_at, out.undo);
        }
    }

    pub(super) fn fail(&mut self, name: &str, e: &Error) {
        if matches!(e, Error::Cancelled) || self.ctl.stop.load(Ordering::Relaxed) {
            self.over.stopped = true;
        } else {
            self.over.failed.push(t!("folders-far-failed-why", name = name, why = &e.to_string()));
        }
    }

    fn send(&mut self, plan: &Plan, ends: &[Option<End>]) {
        let moving = matches!(plan.kind, Kind::Move);
        let Some(to) = End::of(&plan.to) else {
            self.over.failed.push(work::name_of(&plan.to));
            return;
        };
        let sizes: Vec<(u64, u64)> = ends.iter().map(|e| match e {
            Some(e) => {
                let both = matches!((e, &to), (End::Far(_), End::Far(_)));
                let same = moving && root_of(e).is_some() && root_of(e) == root_of(&to);
                let (bytes, files) = if same { (0, 0) } else { self.size(e) };
                (bytes * if both { 2 } else { 1 }, files)
            }
            None => (0, 0),
        }).collect();
        (self.tell)(News::Total(sizes.iter().map(|s| s.0).sum()));
        self.count.files = sizes.iter().map(|s| s.1).sum();
        for (i, end) in ends.iter().enumerate() {
            if self.stopped() {
                break;
            }
            let Some(from) = end else {
                self.over.failed.push(work::name_of(&plan.items[i]));
                continue;
            };
            let name = from.name();
            let got = self.put(from, &to, &name, moving, Up { dir: None, known: None, shared: None, top: true });
            if let (Got::Done(End::Here(made)) | Got::Part(End::Here(made), _), End::Far(_), false) = (&got, from, moving) {
                self.over.undo.push(Step::Remove(made.clone()));
            }
            if let (Got::Part(_, true), true, false) = (&got, moving, self.over.stopped) {
                self.over.failed.push(work::trouble(&name, &crate::files::proof::Counts::default().reason()));
            }
        }
    }

    fn delete(&mut self, plan: &Plan, ends: &[Option<End>]) {
        (self.tell)(News::Total(ends.len() as u64));
        for (i, end) in ends.iter().enumerate() {
            if self.stopped() {
                break;
            }
            let name = work::name_of(&plan.items[i]);
            match end {
                Some(End::Here(p)) => match self.ctl.asked.iter().find(|a| a.path() == p).map(|a| a.delete()) {
                    Some(Ok(())) => {}
                    Some(Err(e)) if !is_changed(&e) => self.over.failed.push(name),
                    _ => self.over.failed.push(t!("folders-far-failed-why", name = &name, why = work::not_looked())),
                },
                Some(e) => {
                    if let Err(err) = self.remove_tree(e) {
                        self.fail(&name, &err);
                    }
                }
                None => self.over.failed.push(name),
            }
            self.hands.done.store(i as u64 + 1, Ordering::Relaxed);
            self.count.tick();
        }
        self.count.say();
    }
}
