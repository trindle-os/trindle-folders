// What the window holds for acts: the hooks it is given, what it says, and what is picked.

use std::path::{Path, PathBuf};

use crate::files::app::{Files, Msg};
use crate::files::work::{Extra, Kind, Plan, Step};
use super::{NOTICE_FOR, arriving, being_moved};
use super::steps::keep;
use super::types::{Ask, Busy, Pick};

impl Files {
    pub fn set_speak(&mut self, speak: crate::toolkit::event::Post<String>) {
        self.speak = Some(speak);
    }

    pub fn set_post(&mut self, post: crate::toolkit::event::Post<Msg>) {
        self.post = Some(post);
        self.sharing_refresh(true);
        self.shares.refresh();
        self.remember_all_mounted();
        self.remember_pinned_shares();
        self.shares_at_start();
        self.earlier_look();
        #[cfg(not(test))]
        self.first_reads();
    }

    pub fn set_jobs_hook(&mut self, hook: impl Fn(Busy) + 'static) {
        self.jobs_hook = Some(Box::new(hook));
    }

    pub fn set_close_hook(&mut self, hook: impl Fn() + 'static) {
        self.close_hook = Some(Box::new(hook));
    }

    pub fn set_pick_hook(&mut self, hook: impl Fn(Pick) + 'static) {
        self.pick_hook = Some(Box::new(hook));
    }

    pub fn pick(&self, ask: Ask, then: impl FnOnce(Vec<PathBuf>) -> Option<Msg> + 'static) {
        if let Some(hook) = &self.pick_hook {
            hook(Pick { ask, then: Box::new(then) });
        }
    }

    pub fn busy(&self) -> Busy {
        Busy { running: self.jobs.len(), asking: self.jobs.iter().any(|j| j.asking.is_some()),
               trouble: (self.bad && !self.said.is_empty()) || matches!(self.dialog, Some(crate::files::dialogs::Dialog::Unlock(_))) }
    }

    pub(super) fn tell_jobs(&self) {
        if let Some(hook) = &self.jobs_hook {
            hook(self.busy());
        }
    }

    pub(in crate::files) fn say(&mut self, words: impl Into<String>, bad: bool) {
        self.said = words.into();
        self.bad = bad;
        self.said_help = None;
        self.reasons_open = false;
    }

    pub(in crate::files) fn say_help(&mut self, words: impl Into<String>, address: &'static str) {
        self.say(words, true);
        self.said_help = Some(address);
    }

    pub(in crate::files) fn help_opens(&mut self, address: &'static str) {
        #[cfg(test)]
        self.helped.push(address);
        #[cfg(not(test))]
        crate::common::more::open(address);
    }

    pub(in crate::files) fn say_listed(&mut self, words: impl Into<String>, bad: bool, lines: Vec<String>) {
        self.say(words, bad);
        self.reasons = Some((self.said.clone(), lines));
    }

    pub(in crate::files) fn notify(&mut self, words: impl Into<String>) {
        self.notices += 1;
        let n = self.notices;
        self.notice = Some((n, words.into()));
        if let Some(post) = self.post.clone() {
            let _ = std::thread::Builder::new().name("folders-notice".into()).spawn(move || {
                std::thread::sleep(NOTICE_FOR);
                post.send(Msg::NoticeGone(n));
            });
        }
    }

    pub(in crate::files) fn picked_paths(&self, p: usize) -> Vec<PathBuf> {
        let pane = &self.cols[p];
        let mut out: Vec<PathBuf> = pane.picked.iter().filter_map(|&i| pane.entries.get(i)).map(|e| e.path.clone()).collect();
        if out.is_empty() {
            if let Some(e) = pane.chosen.and_then(|i| pane.entries.get(i)) {
                out.push(e.path.clone());
            }
        }
        out
    }

    pub(in crate::files) fn choose_path(&mut self, p: usize, path: &Path) {
        let pane = self.pane_mut(p);
        if let Some(i) = pane.entries.iter().position(|e| e.path == path) {
            pane.pick(i, false);
            self.seek(p);
        }
    }

    pub(in crate::files) fn remember(&mut self, label: String, steps: Vec<Step>) {
        self.follow_moves(&steps, false);
        self.redo.clear();
        keep(&mut self.undo, label, steps);
    }

    pub(in crate::files) fn busy_with(&self, plan: &Plan) -> Option<&'static str> {
        if !matches!(plan.kind, Kind::Copy | Kind::Move | Kind::Trash | Kind::Delete | Kind::Other(Extra::Wipe(_))) {
            return None;
        }
        let is_part = |p: &Path| p.file_name().is_some_and(|n| n.to_string_lossy().ends_with(crate::common::files::SPARE_END));
        if plan.items.iter().any(|i| is_part(i)) {
            return Some(arriving());
        }
        let taken = |i: &Path| self.jobs.iter().any(|j| j.from.iter().any(|f| i.starts_with(f) || f.starts_with(i)));
        plan.items.iter().any(|i| taken(i)).then_some(being_moved())
    }

    pub fn is_busy(&self, path: &Path) -> bool {
        self.jobs.iter().any(|j| j.from.iter().any(|f| path.starts_with(f)))
    }
}
