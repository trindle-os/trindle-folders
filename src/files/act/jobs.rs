// Work in the window: starting a job, stopping all of it, and what is said when one ends.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;

use crate::files::app::{Files, Msg};
use crate::files::places::Where;
use crate::files::trash;
use crate::files::work::{self, Extra, Answer, Ctl, Kind, News, Over, Plan};
use crate::toolkit::t;
use super::QUIT_WAIT;
use super::steps::{keep, touched, trashed};
use super::types::{Job, Turn, Words};
use super::text::failed_words;

impl Files {
    pub(in crate::files) fn start(&mut self, plan: Plan, words: Words, turn: Turn) {
        if self.earlier_blocks(&plan) {
            return self.say(crate::files::earlier::shut(), false);
        }
        if let Some(why) = self.busy_with(&plan) {
            return self.say(why, false);
        }
        self.bad = false;
        let id = self.next_job;
        self.next_job += 1;
        let pause = Arc::new(AtomicBool::new(false));
        let stop = Arc::new(AtomicBool::new(false));
        let again = match &plan.kind {
            Kind::Other(Extra::Unpack(place)) => Some(crate::files::locked::Then::Unpack(*place, plan.to.clone())),
            Kind::Other(Extra::TakeOut(_)) => Some(crate::files::locked::Then::Take(plan.items.clone(), plan.to.clone())),
            _ => None,
        };
        let pane = self.work_pane(&plan.to);
        let mut job = Job { id, words, total: 0, done: 0, paused: false, asking: None, all: false,
                            slot: crate::files::progress::free_slot(self, pane), pane,
                            fonts: matches!(plan.kind, Kind::Copy) && plan.to == crate::files::fonts::folder(),
                            pace: crate::files::progress::Pace::new(crate::files::progress::Unit::of(&plan.kind)),
                            pause: pause.clone(), stop: stop.clone(), answers: None, turn,
                            thread: None, began: std::time::Instant::now(), from: if matches!(plan.kind, Kind::Copy | Kind::Move) { plan.items.clone() } else { Vec::new() },
                            touched: touched(&plan, self.bin.as_ref()), again };
        let forget = forget_untrashed(&plan);
        let Some(post) = self.post.clone() else {
            let ctl = Ctl { fixed: Some(Answer::Both), bin: self.bin.clone(), links: self.far.links.clone(), sure: self.outside_sure(), handed: self.outside_handed(), asked: self.asked_for(&plan), ..Ctl::default() };
            self.jobs.push(job);
            let over = work::run(plan, &ctl, &|_| {});
            forget();
            self.finished(id, over);
            return;
        };
        let (tx, rx) = mpsc::channel();
        job.answers = Some(tx);
        let ctl = Ctl { pause, stop, answers: Some(rx), fixed: None, bin: self.bin.clone(), links: self.far.links.clone(), sure: self.outside_sure(), handed: self.outside_handed(), asked: self.asked_for(&plan) };
        self.jobs.push(job);
        let spawned = crate::common::gentle::spawn("folders-work", move || {
            let tell = work::rated(|n: News| {
                post.send(Msg::Work(id, n));
            });
            let over = work::run(plan, &ctl, &tell);
            forget();
            post.send(Msg::Work(id, News::Over(over)));
        });
        self.job_threaded(id, spawned);
        self.tell_jobs();
    }

    fn job_threaded(&mut self, id: u64, spawned: std::io::Result<std::thread::JoinHandle<()>>) {
        match spawned {
            Ok(handle) => {
                if let Some(j) = self.jobs.iter_mut().find(|j| j.id == id) {
                    j.thread = Some(handle);
                }
            }
            Err(_) => {
                self.jobs.retain(|j| j.id != id);
                self.say_help(t!("folders-act-no-start"), crate::files::help::WORK_NOT_DONE);
            }
        }
    }

    pub fn stop_all(&self) {
        for j in &self.jobs {
            j.stop.store(true, Ordering::Relaxed);
            if let Some(tx) = &j.answers {
                let _ = tx.send((Answer::Skip, false));
            }
        }
        let until = std::time::Instant::now() + QUIT_WAIT;
        for handle in self.jobs.iter().filter_map(|j| j.thread.as_ref()) {
            while !handle.is_finished() && std::time::Instant::now() < until {
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        }
    }

    pub(super) fn finished(&mut self, id: u64, mut over: Over) {
        let Some(k) = self.jobs.iter().position(|j| j.id == id) else { return };
        let job = self.jobs.remove(k);
        over.failed.extend(self.outside_stayed(id));
        self.outside_finished(id, &over);
        match &job.turn {
            Turn::New => self.remember(job.words.undo.clone(), over.undo.clone()),
            Turn::Undo(what) => keep(&mut self.redo, t!("folders-act-redo-label", what = what), over.undo.clone()),
            Turn::Redo(what) => keep(&mut self.undo, t!("folders-act-undo-label", what = what), over.undo.clone()),
        }
        let listed = self.outside_listed(id);
        let mut locked = Vec::new();
        for (archive, t) in std::mem::take(&mut over.locked) {
            match &job.again {
                Some(then) if self.unlock_free() && !over.stopped => locked.push((archive, then.clone(), t)),
                _ => over.failed.push(format!("{} ({t})", work::name_of(&archive))),
            }
        }
        if over.stopped {
            self.say(t!("folders-act-stopped"), false);
        } else if over.troubled() {
            let words = failed_words(&over, job.words.verb);
            match listed {
                Some(lines) => self.say_listed(words, true, lines),
                None => self.say(words, true),
            }
            self.said_help = Some(job.words.help);
        } else if over.skipped > 0 {
            self.say(t!("folders-act-skipped", done = &job.words.done, n = over.skipped), false);
        } else if !locked.is_empty() && over.undo.is_empty() {
            self.say(String::new(), false);
        } else {
            self.say(job.words.done.clone(), false);
        }
        if job.fonts && !over.undo.is_empty() {
            self.fonts_done();
        }
        self.ended_say(&job, over.stopped);
        match &job.touched {
            Some(folders) => self.sizes_touched(folders),
            None => self.sizes_all_changed(),
        }
        self.reload_all();
        for (archive, then, t) in locked {
            self.unlock_ask(archive, then, (t != crate::files::archive::Trouble::Locked).then_some(t));
        }
        self.tell_jobs();
    }

    fn work_pane(&self, to: &Path) -> usize {
        let shows = |p: usize, at: &Path| match &self.cols[p].at {
            Where::Folder(f) => f == at,
            Where::Remote(a) => Path::new(a) == at,
            _ => false,
        };
        if !self.two {
            return self.side();
        }
        let near = |at: &Path| {
            let both: Vec<usize> = (0..2).filter(|&p| shows(p, at)).collect();
            (both.len() == 1).then(|| both[0])
        };
        near(to).or_else(|| to.parent().and_then(near)).unwrap_or_else(|| self.side())
    }

    fn ended_say(&mut self, job: &Job, stopped: bool) {
        if self.said.is_empty() {
            return;
        }
        self.ended.push(crate::files::progress::Ended { id: job.id, slot: job.slot, pane: job.pane, words: self.said.clone(), bad: self.bad, help: self.said_help.filter(|_| self.bad),
                                                         stays: self.bad || stopped });
        if let Some(post) = self.post.clone() {
            let id = job.id;
            let _ = std::thread::Builder::new().name("folders-work-ended".into()).spawn(move || {
                std::thread::sleep(crate::files::progress::done_for());
                post.send(Msg::WorkGone(id));
            });
        }
    }

    pub(super) fn job_mut(&mut self, id: u64) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.id == id)
    }
}

fn forget_untrashed(plan: &Plan) -> impl FnOnce() + Send + 'static {
    let untrash: Vec<PathBuf> = match plan.kind {
        Kind::Move => plan.items.iter().filter(|p| trashed(p)).cloned().collect(),
        _ => Vec::new(),
    };
    move || {
        for item in &untrash {
            if fs::symlink_metadata(item).is_err() {
                trash::forget(item);
            }
        }
    }
}
