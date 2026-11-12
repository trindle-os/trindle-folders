//! Putting one thing into a folder, which may be on a server: the one place a name is joined to the folder the person chose.

use std::collections::HashSet;
use std::fs;
use std::io;
use std::sync::{mpsc, Arc};

use crate::toolkit::t;

use crate::files::remote::{self, Error, not_a_name};
use crate::files::work::{self, Answer, Which};
use crate::common::files::{is_changed, is_not_a_file, is_one_name, open_named, Named};
use super::ends::{Child, End, Found, Got, Info, Up, claim_key, missing, root_of};
use super::{not_followed, not_sent, not_told, path_of};
use super::walk::{Go, Task};

impl Go<'_> {
    fn find(&self, from: &End, up: Up, moving: bool) -> Result<Option<Found>, Error> {
        match from {
            End::Far(a) => match self.links.with(a, |r| r.stat(path_of(a))) {
                Ok(e) if e.kind == remote::Kind::Other => Err(Error::Failed(not_told().into())),
                Ok(e) => Ok(Some(Found { folder: e.is_folder(), size: e.size, modified: e.modified, here: None })),
                Err(e) if missing(&e) => Ok(None),
                Err(e) => Err(e),
            },
            End::Here(p) => {
                let sure = if up.dir.is_none() { self.ctl.sure.iter().find(|s| s.path == *p) } else { None };
                let opened = match open_named(p, up.dir, sure.map(|s| s.which)) {
                    Ok(Named::Link(..)) if up.dir.is_none() && sure.is_none() && !moving => fs::canonicalize(p).and_then(|real| open_named(&real, None, None)),
                    other => other,
                };
                match opened {
                    Ok(Named::File(named, meta)) => Ok(Some(Found { folder: false, size: meta.len(), modified: None, here: Some((named, meta)) })),
                    Ok(Named::Folder(named, meta)) => Ok(Some(Found { folder: true, size: 0, modified: None, here: Some((named, meta)) })),
                    Ok(Named::Link(..)) => Err(Error::Failed(not_followed().into())),
                    Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
                    Err(e) if is_not_a_file(&e) => Err(Error::Failed(not_sent().into())),
                    Err(e) if is_changed(&e) => Err(Error::Failed(work::changed_since().into())),
                    Err(e) => Err(e.into()),
                }
            }
        }
    }

    fn children(&self, from: &End, found: &Found, known: Option<&HashSet<Which>>) -> Result<(Vec<Child>, bool), Error> {
        match (from, &found.here) {
            (End::Here(_), Some((dir, _))) => {
                let listed = work::listed(dir)?;
                let things = listed.things.into_iter().map(|(name, meta)| {
                    let said = name.to_string_lossy().into_owned();
                    match meta {
                        Ok(m) if work::came_since(&name, &m, known) => Child::Left(said, work::not_there_why(known)),
                        Ok(_) => Child::Go(said),
                        Err(_) => Child::Left(said, t!("folders-far-not-looked-at")),
                    }
                }).collect();
                Ok((things, listed.whole))
            }
            (End::Far(a), _) => {
                let listed = self.links.with(a, |r| r.list(path_of(a)))?;
                let left = listed.refused.into_iter().map(|n| Child::Left(n, not_a_name()));
                Ok((listed.entries.into_iter().map(|e| Child::Go(e.name)).chain(left).collect(), true))
            }
            (End::Here(_), None) => Err(Error::Failed(t!("folders-far-not-opened").into())),
        }
    }

    pub(super) fn put(&mut self, from: &End, to_dir: &End, name: &str, moving: bool, up: Up) -> Got {
        if !is_one_name(name.as_bytes()) {
            self.over.failed.push(t!("folders-far-failed-why", name = name, why = not_a_name()));
            return Got::Failed;
        }
        let found = match self.find(from, up, moving) {
            Ok(Some(found)) => found,
            Ok(None) => {
                self.fail(name, &Error::NotFound(name.into()));
                return Got::Failed;
            }
            Err(e) => {
                self.fail(name, &e);
                return Got::Failed;
            }
        };
        if found.folder && to_dir.within(from) {
            self.over.failed.push(t!("folders-far-inside-itself", name = name));
            return Got::Failed;
        }
        let p = Putting { from, name, moving, up };
        let land = match self.landing(&p, to_dir, &found) {
            Ok(land) => land,
            Err(got) => return got,
        };
        if moving && !land.merge && root_of(from).is_some() && root_of(from) == root_of(to_dir) {
            if let (End::Far(_), End::Far(_)) = (from, &land.target) {
                return match self.name_it(from, &land.target, land.over.as_ref(), false) {
                    Ok(()) => Got::Done(land.target),
                    Err(e) => {
                        self.fail(name, &e);
                        Got::Failed
                    }
                };
            }
        }
        if found.folder {
            return self.put_folder(&p, &found, land);
        }
        let (send, came) = mpsc::channel();
        let undo_made = up.top && !moving && matches!(from, End::Far(_));
        let task = Task { from: from.clone(), found, to_dir: to_dir.clone(), target: land.target, name: name.to_string(), over: land.over, moving, up: up.shared.cloned(), undo_made, came: send };
        Got::Later(self.hand(task, came))
    }

    fn landing(&mut self, p: &Putting, to_dir: &End, found: &Found) -> Result<Landing, Got> {
        let Putting { from, name, moving, .. } = *p;
        let mut target = to_dir.join(name);
        let mut merge = false;
        let mut over = None;
        if self.claimed.contains(&claim_key(&target)) {
            self.wait_all();
        }
        let there = match self.stat(&target) {
            Ok(t) => t,
            Err(e) => {
                self.fail(name, &e);
                return Err(Got::Failed);
            }
        };
        if let Some(there) = there {
            let answer = if target == *from {
                if moving {
                    return Err(Got::Done(target));
                }
                Answer::Both
            } else {
                let folders = there.folder && found.folder;
                let clash = work::Clash { name: name.to_string(), folders, sides: None, replace: !folders };
                match work::ask(self.ctl, self.tell, &mut self.same, clash) {
                    Some(a) => a,
                    None => {
                        self.over.stopped = true;
                        return Err(Got::Skipped);
                    }
                }
            };
            match answer {
                Answer::Merge if there.folder && found.folder => merge = true,
                Answer::Skip | Answer::Merge => {
                    self.over.skipped += 1;
                    return Err(Got::Skipped);
                }
                Answer::Both => {
                    let free = work::free_in(std::ffi::OsStr::new(name), found.folder, |n| {
                        let wanted = to_dir.join(&n.to_string_lossy());
                        self.claimed.contains(&claim_key(&wanted)) || matches!(self.stat(&wanted), Ok(Some(_)))
                    });
                    target = to_dir.join(&free.to_string_lossy());
                }
                Answer::Replace if there.folder != found.folder => {
                    self.over.failed.push(if there.folder { t!("folders-far-taken-by-folder", name = name) } else { t!("folders-far-taken-by-file", name = name) });
                    return Err(Got::Failed);
                }
                Answer::Replace if found.folder => merge = true,
                Answer::Replace => over = Some(there),
            }
        }
        Ok(Landing { target, merge, over })
    }

    fn put_folder(&mut self, p: &Putting, found: &Found, land: Landing) -> Got {
        let Putting { from, name, moving, up } = *p;
        let target = land.target;
        if !land.merge {
            if let Err(e) = self.mkdir(&target) {
                self.fail(name, &e);
                return Got::Failed;
            }
        }
        let ctl = self.ctl;
        let known = up.known.or_else(|| match (from, up.dir) {
            (End::Here(p), None) => ctl.sure.iter().find(|s| s.path == *p).and_then(|s| s.known.as_deref()),
            _ => None,
        });
        let (children, all) = match self.children(from, found, known) {
            Ok(c) => c,
            Err(e) => {
                self.fail(name, &e);
                return Got::Failed;
            }
        };
        let mut lost = false;
        if !all {
            self.over.failed.push(t!("folders-far-failed-why", name = name, why = work::not_all()));
            lost = true;
        }
        let (whole, lost) = self.put_children(p, found, &target, children, known, (all, lost));
        if !whole {
            return Got::Part(target, lost);
        }
        if moving && self.rmdir_empty(from).is_err() {
            return Got::Part(target, true);
        }
        Got::Done(target)
    }

    fn put_children(&mut self, p: &Putting, found: &Found, target: &End, children: Vec<Child>, known: Option<&HashSet<Which>>, (mut whole, mut lost): (bool, bool)) -> (bool, bool) {
        let Putting { from, moving, .. } = *p;
        let shared = if moving { found.here.as_ref().and_then(|(dir, _)| dir.try_clone().ok()).map(Arc::new) } else { None };
        let inside = Up { dir: found.here.as_ref().map(|(dir, _)| dir), known, shared: shared.as_ref(), top: false };
        let mut later = Vec::new();
        for child in children {
            if self.stopped() {
                whole = false;
                break;
            }
            let child = match child {
                Child::Go(child) => child,
                Child::Left(child, why) => {
                    self.over.failed.push(t!("folders-far-failed-why", name = &child, why = why));
                    (whole, lost) = (false, true);
                    continue;
                }
            };
            let got = self.put(&from.join(&child), target, &child, moving, inside);
            if let Got::Later(slot) = got {
                later.push(slot);
                continue;
            }
            whole &= matches!(got, Got::Done(_));
            lost |= matches!(got, Got::Failed | Got::Part(_, true) | Got::Stayed);
        }
        for slot in later {
            let got = self.wait(slot);
            whole &= matches!(got, Got::Done(_));
            lost |= matches!(got, Got::Failed | Got::Part(_, true) | Got::Stayed);
        }
        (whole, lost)
    }
}

#[derive(Clone, Copy)]
struct Putting<'a> {
    from: &'a End,
    name: &'a str,
    moving: bool,
    up: Up<'a>,
}

struct Landing {
    target: End,
    merge: bool,
    over: Option<Info>,
}
