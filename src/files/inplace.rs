// Changing a name where it is drawn: the name in the list becomes a typing box, in every view.

#[cfg(test)]
mod tests;

use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::toolkit::t;
use super::far;
use super::act::bad_name;
use super::app::{Files, Msg};
use super::places::Where;
use super::work::{self, Step};

pub(super) fn rename_here(path: &Path, name: &str) -> Result<PathBuf, String> {
    let new = path.with_file_name(name);
    if let Ok(there) = fs::symlink_metadata(&new) {
        if !only_case_differs(path, &new, &there) {
            return Err(taken_words(name, new.is_dir()));
        }
        return case_only(path, &new);
    }
    crate::common::files::rename_new(path, &new).map_err(|e| t!("folders-inplace-not-renamed", name = work::name_of(path), why = e.to_string()))?;
    Ok(new)
}

fn only_case_differs(path: &Path, new: &Path, there: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    let (a, b) = (work::name_of(path), work::name_of(new));
    a != b && a.to_lowercase() == b.to_lowercase() && fs::symlink_metadata(path).is_ok_and(|m| (m.dev(), m.ino()) == (there.dev(), there.ino()))
}

fn case_only(path: &Path, new: &Path) -> Result<PathBuf, String> {
    let why = |e: std::io::Error| t!("folders-inplace-not-renamed", name = work::name_of(path), why = e.to_string());
    let step = path.with_file_name(format!(".{}.{}.case", work::name_of(path), std::process::id()));
    crate::common::files::rename_new(path, &step).map_err(why)?;
    if let Err(e) = crate::common::files::rename_new(&step, new) {
        let _ = crate::common::files::rename_new(&step, path);
        return Err(why(e));
    }
    Ok(new.to_path_buf())
}

pub(super) fn taken_words(name: &str, folder: bool) -> String {
    t!("folders-inplace-taken", kind = if folder { "folder" } else { "file" }, name = name)
}

pub(super) const TWICE: Duration = crate::toolkit::ui::MULTI_CLICK;
pub(super) const SLOW_WITHIN: Duration = Duration::from_secs(5);

#[derive(Clone, Debug)]
pub struct Renaming {
    pub side: usize,
    pub tab: usize,
    pub path: PathBuf,
    pub text: String,
    pub folder: bool,
    pub fresh: bool,
    pub why: Option<String>,
    pub tail: String,
}

impl Renaming {
    pub fn was(&self) -> String {
        work::name_of(&self.path)
    }

    pub fn selected(&self) -> Range<usize> {
        let was = self.was();
        if self.tail.is_empty() { 0..stem_len(&was, self.folder) } else { 0..was.len().saturating_sub(self.tail.len()) }
    }

    pub fn name(&self) -> String {
        format!("{}{}", self.text, self.tail)
    }
}

pub(super) fn stem_len(name: &str, folder: bool) -> usize {
    match name.rfind('.') {
        Some(i) if i > 0 && !folder => i,
        _ => name.len(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum End {
    Enter,
    Away,
    Next(bool),
    Gone,
}

#[derive(Clone, Debug)]
pub struct Slow {
    pub n: u64,
    pub side: usize,
    pub path: PathBuf,
    pub due: Instant,
    pub ripe: bool,
}

pub(super) fn slow_second(last: Option<(Instant, usize)>, i: usize, now: Instant) -> bool {
    last.is_some_and(|(t, j)| {
        let gap = now.saturating_duration_since(t);
        j == i && gap >= TWICE && gap <= SLOW_WITHIN
    })
}

pub(super) fn can_rename(at: &Where) -> bool {
    match at {
        Where::Folder(_) => true,
        Where::Remote(a) => !super::remote::Address::parse(a).is_ok_and(|a| matches!(a.scheme, super::remote::Scheme::Smb | super::remote::Scheme::Nfs)),
        _ => false,
    }
}

impl Files {
    pub(super) fn rename_start(&mut self) {
        let p = self.side();
        if !can_rename(&self.pane().at) {
            return self.say(t!("folders-inplace-only-folder"), false);
        }
        let pane = self.pane();
        if pane.picked.len() > 1 {
            let items = pane.picked.iter().filter_map(|&i| pane.entries.get(i)).map(|e| e.path.clone()).collect();
            return self.batch_rename(items);
        }
        let i = pane.chosen.filter(|i| pane.picked.is_empty() || pane.picked.contains(i)).or(pane.picked.first().copied());
        match i.filter(|&i| i < pane.entries.len()) {
            Some(i) => self.rename_of(p, i, false),
            None => self.say(t!("folders-inplace-pick-first"), false),
        }
    }

    pub(super) fn rename_of(&mut self, p: usize, i: usize, fresh: bool) {
        let tab = self.cols[p].tab;
        let pane = self.pane_mut(p);
        let Some(e) = pane.entries.get(i).cloned() else { return };
        pane.pick(i, false);
        self.side = p;
        self.slow = None;
        if self.bad {
            self.say("", false);
        }
        let cut = self.shown.hidden_from(&e, self.endings).unwrap_or(e.name.len());
        let (text, tail) = (e.name[..cut].to_string(), e.name[cut..].to_string());
        self.renaming = Some(Renaming { side: p, tab, path: e.path, text, folder: e.folder, fresh, why: None, tail });
        self.seek(p);
        self.turn += 1;
    }

    pub(super) fn rename_path(&mut self, p: usize, path: &Path, fresh: bool) {
        if let Some(i) = self.cols[p].entries.iter().position(|e| e.path == path) {
            self.rename_of(p, i, fresh);
        }
    }

    pub(super) fn rename_refused(&mut self, mut r: Renaming, why: String, page: &'static str) {
        self.say_help(why.clone(), page);
        let i = self.cols[r.side].entries.iter().position(|e| e.path == r.path).filter(|_| self.cols[r.side].tab == r.tab);
        let Some(i) = i else { return };
        self.pane_mut(r.side).pick(i, false);
        r.why = Some(why);
        self.renaming = Some(r);
        self.turn += 1;
    }

    fn rename_held(&self, r: &Renaming) -> bool {
        let pane = &self.cols[r.side];
        pane.tab == r.tab && pane.chosen.and_then(|i| pane.entries.get(i)).is_some_and(|e| e.path == r.path)
    }

    fn neighbour(&self, r: &Renaming, forward: bool) -> Option<PathBuf> {
        let entries = &self.cols[r.side].tabs.get(r.tab)?.entries;
        let i = entries.iter().position(|e| e.path == r.path)?;
        let to = if forward { i.checked_add(1)? } else { i.checked_sub(1)? };
        entries.get(to).map(|e| e.path.clone())
    }

    pub(super) fn rename_end(&mut self, how: End) {
        let Some(r) = self.renaming.take() else { return };
        let old = r.was();
        let next = match how {
            End::Next(forward) => self.neighbour(&r, forward),
            _ => None,
        };
        let name = r.name();
        if name == old {
            if let Some(next) = next {
                self.rename_path(r.side, &next, false);
            }
            return;
        }
        if let Some(why) = bad_name(if r.text.is_empty() { "" } else { &name }) {
            return self.rename_refused(r, why.to_string(), super::help::NAME_REFUSED);
        }
        if let Some(a) = far::address(&r.path) {
            return self.far_rename(r, a, next);
        }
        let new = match rename_here(&r.path, &name) {
            Ok(new) => new,
            Err(why) => return self.rename_refused(r, why, super::help::NAME_REFUSED),
        };
        let made = vec![Step::Remove(r.path.clone())];
        match self.undo.last_mut() {
            Some((label, steps)) if r.fresh && *steps == made => {
                *steps = vec![Step::Remove(new.clone())];
                *label = t!("folders-inplace-undo-making", name = &name);
            }
            _ => self.remember(t!("folders-inplace-undo-renaming", old = &old, name = &name), vec![Step::Back { now: new.clone(), was: r.path.clone() }]),
        }
        self.say(t!("folders-inplace-renamed", old = &old, name = &name), false);
        let held = self.rename_held(&r);
        if r.tab < self.cols[r.side].tabs.len() {
            self.load(r.tab, r.side);
        }
        self.when_read(r.side, held.then_some(new), next.map(|n| (n, false)));
    }

    fn far_rename(&mut self, r: Renaming, a: super::remote::Address, next: Option<PathBuf>) {
        let name = r.name();
        let taken = self.cols[r.side].tabs.get(r.tab).is_some_and(|pane| pane.entries.iter().any(|e| e.name == name));
        if taken {
            let there = self.cols[r.side].tabs.get(r.tab).and_then(|pane| pane.entries.iter().find(|e| e.name == name)).is_some_and(|e| e.folder);
            return self.rename_refused(r, taken_words(&name, there), super::help::NAME_REFUSED);
        }
        let new = a.parent().join(&name);
        let (from, to) = (a.path.clone(), new.path.clone());
        self.say(t!("folders-inplace-renaming", old = r.was(), name = &name), false);
        let now = self.rename_held(&r).then(|| PathBuf::from(new.to_string()));
        self.far_do(a, move |link, _| link.rename(&from, &to), move |got| far::FarMsg::Renamed { was: Box::new(r), now, next, got });
    }

    pub(super) fn slow_arm(&mut self, p: usize, path: PathBuf, now: Instant) {
        self.slows += 1;
        let n = self.slows;
        self.slow = Some(Slow { n, side: p, path, due: now + TWICE, ripe: false });
        if let Some(post) = self.post.clone() {
            let _ = std::thread::Builder::new().name("folders-slow-click".into()).spawn(move || {
                std::thread::sleep(TWICE);
                post.send(Msg::SlowRename(n));
            });
        }
    }

    pub(super) fn slow_due(&mut self, n: u64, now: Instant) {
        let Some(s) = self.slow.as_mut().filter(|s| s.n == n && now >= s.due) else { return };
        if self.drag.is_some() {
            s.ripe = true;
            return;
        }
        self.slow_start();
    }

    pub(super) fn slow_let_go(&mut self) {
        if self.slow.as_ref().is_some_and(|s| s.ripe) {
            self.slow_start();
        }
    }

    fn slow_start(&mut self) {
        let Some(s) = self.slow.take() else { return };
        if self.renaming.is_some() || self.menu.is_some() || self.dialog.is_some() || self.confirm.is_some() || self.dropping.is_some() {
            return;
        }
        let pane = &self.cols[s.side];
        let i = pane.entries.iter().position(|e| e.path == s.path).filter(|i| pane.picked == [*i] && pane.chosen == Some(*i));
        if let Some(i) = i {
            self.rename_of(s.side, i, false);
        }
    }

    pub(super) fn rename_keep(&mut self) -> bool {
        self.rename_end(End::Away);
        self.renaming.is_none()
    }

    pub(super) fn rename_follow(&mut self) {
        let Some(r) = &self.renaming else { return };
        let shown = self.cols[r.side].tab == r.tab && self.cols[r.side].entries.iter().any(|e| e.path == r.path);
        if !shown {
            self.rename_end(End::Gone);
        }
    }
}
