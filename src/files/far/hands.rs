//! What the crew's threads do with one file: bring it, prove it whole, give it its name, take the first away.

use std::fs::{self, File};
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::toolkit::t;

use crate::files::trash;
use crate::files::proof::{self, FarSeal};
use crate::files::remote::{self, Address, Error, not_a_name};
use crate::files::work::{self, spare_name, Step, Which, NAP};
use crate::common::gentle;
use super::copy::copy_file;
use super::ends::{End, Found, Got, Info, missing};
use super::links::Links;
use super::{not_all_arrived, path_of};
use super::walk::{Outcome, Task};

#[derive(Clone, Copy)]
pub(super) struct Hands<'a> {
    pub(super) links: &'a Links,
    pub(super) pause: &'a AtomicBool,
    pub(super) stop: &'a AtomicBool,
    pub(super) bin: Option<&'a trash::Bin>,
    pub(super) done: &'a AtomicU64,
    pub(super) across: &'a AtomicU64,
    pub(super) held: &'a Waiting,
}

pub(super) type Waiting = gentle::Held<File, PathBuf>;

pub(super) fn save_back(links: &Links, a: &Address, local: &Path) -> Result<(), String> {
    let mut file = crate::common::files::open_file(local).map_err(|e| e.to_string())?;
    let size = file.metadata().map_err(|e| e.to_string())?.len();
    let (done, across, pause, stop) = (AtomicU64::new(0), AtomicU64::new(0), AtomicBool::new(false), AtomicBool::new(false));
    let held: Waiting = gentle::Held::default();
    let hands = Hands { links, pause: &pause, stop: &stop, bin: None, done: &done, across: &across, held: &held };
    let (target, landing) = (End::Far(a.clone()), End::Far(a.parent().join(&spare_name(remote::name_of(&a.path)))));
    let End::Far(spare) = &landing else { return Err(t!("folders-far-address-unknown").into()) };
    let over = hands.stat(&target).map_err(|e| e.to_string())?;
    if over.as_ref().is_some_and(|o| o.folder) {
        return Err(work::taken_meanwhile().into());
    }
    if let Err(e) = links.with(spare, |r| r.write_new(path_of(spare), &mut file, size, &mut |_| true)) {
        if !matches!(e, Error::Exists(_)) {
            let _ = hands.remove_one(&landing);
        }
        return Err(e.to_string());
    }
    let whole = hands.stat(&landing).map_err(|e| e.to_string()).and_then(|i| match i {
        Some(i) if i.size == size => Ok(()),
        _ => Err(t!("folders-far-copy-wrong-length").to_string()),
    });
    if let Err(why) = whole {
        let _ = hands.remove_one(&landing);
        return Err(why);
    }
    hands.name_it(&landing, &target, over.as_ref(), true).map_err(|e| e.to_string())
}

pub(super) fn reason(e: &impl ToString) -> String {
    e.to_string().trim_end_matches('.').to_string()
}

impl Hands<'_> {
    pub(super) fn go_on(&self) -> bool {
        while self.pause.load(Ordering::Relaxed) && !self.stop.load(Ordering::Relaxed) {
            std::thread::sleep(NAP);
        }
        !self.stop.load(Ordering::Relaxed)
    }

    fn trash(&self, path: &Path) -> io::Result<PathBuf> {
        work::to_trash(self.bin, path)
    }

    pub(super) fn bring(&self, task: Task) -> Outcome {
        let Task { from, found, to_dir, target, name, over, moving, up, undo_made, .. } = task;
        let (from, name) = (&from, name.as_str());
        let mut out = Outcome { got: Got::Skipped, failed: Vec::new(), undo: Vec::new(), stopped: false };
        if !self.go_on() {
            out.stopped = true;
            return out;
        }
        let landing = match &target {
            End::Here(_) if over.is_none() => target.clone(),
            _ => to_dir.join(&spare_name(name)),
        };
        let sure = moving || over.is_some();
        let (sent, made) = match copy_file(self, from, &found, &landing, sure) {
            Ok(came) => came,
            Err(e) => {
                if matches!(landing, End::Far(_)) && !matches!(e, Error::Exists(_)) {
                    let _ = self.remove_one(&landing);
                }
                self.fail_file(&mut out, name, &e);
                return out;
            }
        };
        let proof = match self.proved(from, &found, &landing, &made, sent, sure) {
            Ok(proof) => proof,
            Err(e) => {
                self.fail_file(&mut out, name, &e);
                return out;
            }
        };
        if !self.named(&landing, &target, &over, &made, name, &mut out) {
            return out;
        }
        self.across.fetch_add(1, Ordering::Relaxed);
        if let (true, End::Here(made)) = (undo_made, &target) {
            out.undo.push(Step::Remove(made.clone()));
        }
        if moving {
            let gone = match &proof {
                Some(proof) => self.remove_first(from, proof, up.as_deref()),
                None => Err(Error::Failed(work::stayed().into())),
            };
            if let Err(e) = gone {
                self.fail_file(&mut out, name, &e);
                out.got = Got::Stayed;
                return out;
            }
        }
        out.got = Got::Done(target);
        out
    }

    fn fail_file(&self, out: &mut Outcome, name: &str, e: &Error) {
        if matches!(e, Error::Cancelled) || self.stop.load(Ordering::Relaxed) {
            out.stopped = true;
        } else {
            out.failed.push(t!("folders-far-failed-why", name = name, why = &reason(e)));
        }
        out.got = Got::Failed;
    }

    fn proved(&self, from: &End, found: &Found, landing: &End, made: &Option<File>, sent: u64, sure: bool) -> Result<Option<proof::Copied>, Error> {
        if !sure {
            return Ok(None);
        }
        self.arrived(from, found, landing, sent).map(Some).inspect_err(|_| match (landing, made) {
            (End::Here(at), Some(made)) => proof::unmake(made, at),
            _ => {
                let _ = self.remove_one(landing);
            }
        })
    }

    fn named(&self, landing: &End, target: &End, over: &Option<Info>, made: &Option<File>, name: &str, out: &mut Outcome) -> bool {
        match (landing, target, over) {
            (End::Far(_), ..) => {
                if let Err(e) = self.name_it(landing, target, over.as_ref(), true) {
                    self.fail_file(out, name, &e);
                    return false;
                }
            }
            (End::Here(at), End::Here(place), Some(was)) => {
                if let Err(why) = self.take_place(at, place, was, &mut out.undo) {
                    if let Some(made) = made {
                        proof::unmake(made, at);
                    }
                    out.failed.push(t!("folders-far-failed-why", name = name, why = &reason(&why)));
                    out.got = Got::Failed;
                    return false;
                }
            }
            _ => {}
        }
        true
    }

    fn arrived(&self, from: &End, found: &Found, to: &End, sent: u64) -> Result<proof::Copied, Error> {
        let changed = || Error::Failed(work::changed_why().into());
        let lies = self.stat(to).ok().flatten().filter(|i| !i.folder).map(|i| i.size);
        let proof = match (from, &found.here) {
            (End::Here(_), Some((named, first))) => {
                let now = named.metadata()?;
                if proof::Seal::of(&now) != proof::Seal::of(first) {
                    return Err(changed());
                }
                proof::Copied::of_file(first, &now, sent, lies)
            }
            (End::Far(_), _) => {
                let first = FarSeal { size: found.size, modified: found.modified };
                let now = self.stat(from).ok().flatten().filter(|i| !i.folder).map(|i| i.seal());
                if now.is_some_and(|now| now != first) {
                    return Err(changed());
                }
                proof::Copied::of_far(first, now, sent, lies)
            }
            (End::Here(_), None) => None,
        };
        proof.ok_or_else(|| Error::Failed(not_all_arrived().into()))
    }

    fn take_place(&self, landing: &Path, place: &Path, was: &Info, undo: &mut Vec<Step>) -> Result<(), String> {
        match fs::symlink_metadata(place) {
            Ok(now) if Some(Which::of(&now)) != was.which => return Err(work::taken_meanwhile().into()),
            Ok(_) => match self.trash(place) {
                Ok(t) => undo.push(Step::Restore(t)),
                Err(_) => return Err(work::no_trash().into()),
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(_) => return Err(work::no_trash().into()),
        }
        crate::common::files::rename_new(landing, place)
            .map_err(|e| if e.kind() == io::ErrorKind::AlreadyExists { work::taken_meanwhile().into() } else { e.to_string() })
    }

    pub(super) fn name_it(&self, landing: &End, target: &End, over: Option<&Info>, own: bool) -> Result<(), Error> {
        let (End::Far(a), End::Far(b)) = (landing, target) else { return Err(Error::Failed(t!("folders-far-only-server-file").into())) };
        let taken = || Error::Failed(work::taken_meanwhile().into());
        let rename = || self.links.with(a, |r| r.rename(path_of(a), path_of(b)));
        let dropped = |e: Error| {
            if own {
                let _ = self.remove_one(landing);
            }
            e
        };
        let refuses = self.links.with(a, |r| Ok(r.refuses_taken())).unwrap_or(false);
        let Some(was) = over else {
            if !refuses && !matches!(self.stat(target), Ok(None)) {
                return Err(dropped(taken()));
            }
            return rename().map_err(|e| dropped(if matches!(e, Error::Exists(_)) { taken() } else { e }));
        };
        let free = |to: &End| refuses || matches!(self.stat(to), Ok(None));
        let is_it = |at: &End| matches!(self.stat(at), Ok(Some(now)) if !now.folder && now.seal() == was.seal());
        match self.stat(target) {
            Ok(Some(now)) if now.folder || now.seal() != was.seal() => return Err(dropped(taken())),
            Ok(Some(_)) => {}
            Ok(None) => return rename().map_err(|e| dropped(if matches!(e, Error::Exists(_)) { taken() } else { e })),
            Err(e) => return Err(dropped(e)),
        }
        let aside = End::Far(b.parent().join(&spare_name(remote::name_of(path_of(b)))));
        let End::Far(kept) = &aside else { return Err(dropped(taken())) };
        self.links.with(b, |r| r.rename(path_of(b), path_of(kept))).map_err(dropped)?;
        let back = |why: Error| match free(target).then(|| self.links.with(kept, |r| r.rename(path_of(kept), path_of(b)))) {
            Some(Ok(())) => dropped(why),
            _ => Error::Failed(t!("folders-far-aside-both", old = aside.name(), new = landing.name(), why = &reason(&why))),
        };
        if !is_it(&aside) {
            return Err(back(taken()));
        }
        if !free(target) {
            return Err(back(taken()));
        }
        if let Err(e) = rename() {
            return Err(back(if matches!(e, Error::Exists(_)) { taken() } else { e }));
        }
        match self.links.with(kept, |r| r.remove_if(path_of(kept), was.size, was.modified)) {
            Ok(true) => Ok(()),
            Err(e) if missing(&e) => Ok(()),
            Ok(false) => Err(Error::Failed(t!("folders-far-aside-kept", old = aside.name()))),
            Err(e) => Err(Error::Failed(t!("folders-far-aside-kept-why", old = aside.name(), why = &reason(&e)))),
        }
    }

    pub(super) fn stat(&self, e: &End) -> Result<Option<Info>, Error> {
        let got = match e {
            End::Here(p) => fs::symlink_metadata(p).map(|m| Info { folder: m.is_dir(), size: m.len(), modified: None, which: Some(Which::of(&m)) }).map_err(Error::from),
            End::Far(a) => self.links.with(a, |r| r.stat(path_of(a))).map(|e| Info { folder: e.is_folder(), size: e.size, modified: e.modified, which: None }),
        };
        match got {
            Ok(i) => Ok(Some(i)),
            Err(e) if missing(&e) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn names(&self, e: &End) -> Result<(Vec<String>, Vec<String>), Error> {
        match e {
            End::Here(p) => {
                let mut names = Vec::new();
                for d in fs::read_dir(p)? {
                    names.push(d?.file_name().to_string_lossy().into_owned());
                }
                Ok((names, Vec::new()))
            }
            End::Far(a) => {
                let listed = self.links.with(a, |r| r.list(path_of(a)))?;
                Ok((listed.entries.into_iter().map(|e| e.name).collect(), listed.refused))
            }
        }
    }

    pub(super) fn size(&self, e: &End) -> (u64, u64) {
        match self.stat(e) {
            Ok(Some(i)) if i.folder => self.names(e).unwrap_or_default().0.iter().map(|n| self.size(&e.join(n))).fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1)),
            Ok(Some(i)) => (i.size, 1),
            _ => (0, 0),
        }
    }

    pub(super) fn mkdir(&self, e: &End) -> Result<(), Error> {
        match e {
            End::Here(p) => Ok(fs::create_dir(p)?),
            End::Far(a) => self.links.with(a, |r| r.mkdir(path_of(a))),
        }
    }

    pub(super) fn rmdir_empty(&self, e: &End) -> Result<(), Error> {
        match e {
            End::Here(p) => Ok(fs::remove_dir(p)?),
            End::Far(a) => {
                if self.links.with(a, |r| r.list(path_of(a)))?.held() > 0 {
                    return Err(Error::Failed(t!("folders-far-not-empty", name = e.name())));
                }
                self.links.with(a, |r| r.rmdir(path_of(a)))
            }
        }
    }

    fn remove_one(&self, e: &End) -> Result<(), Error> {
        match e {
            End::Here(p) => Ok(fs::remove_file(p)?),
            End::Far(a) => self.links.with(a, |r| r.remove(path_of(a))),
        }
    }

    fn remove_first(&self, from: &End, proof: &proof::Copied, up: Option<&File>) -> Result<(), Error> {
        let stayed = || Error::Failed(work::stayed().into());
        match from {
            End::Here(p) => {
                let (Some(folder), Some(name)) = (p.parent(), p.file_name()) else { return Err(stayed()) };
                let opened;
                let dir = match up {
                    Some(dir) => dir,
                    None => {
                        opened = fs::OpenOptions::new().read(true).custom_flags(libc::O_PATH | libc::O_DIRECTORY).open(folder)?;
                        &opened
                    }
                };
                if proof::take_away(dir, name, proof) { Ok(()) } else { Err(stayed()) }
            }
            End::Far(a) => {
                let Some(was) = proof.far() else { return Err(stayed()) };
                match self.links.with(a, |r| r.remove_if(path_of(a), was.size, was.modified)) {
                    Ok(true) => Ok(()),
                    Ok(false) => Err(stayed()),
                    Err(e) if missing(&e) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub(super) fn remove_tree(&self, e: &End) -> Result<(), Error> {
        if let End::Here(_) = e {
            return Err(Error::Failed(work::not_looked().into()));
        }
        let folder = match self.stat(e)? {
            Some(i) => i.folder,
            None => return Ok(()),
        };
        if !folder {
            return self.remove_one(e);
        }
        let (names, refused) = self.names(e)?;
        for n in names {
            if self.stop.load(Ordering::Relaxed) {
                return Err(Error::Cancelled);
            }
            self.remove_tree(&e.join(&n))?;
        }
        if let Some(n) = refused.first() {
            return Err(Error::Failed(t!("folders-far-failed-why", name = n, why = not_a_name())));
        }
        self.rmdir_empty(e)
    }
}
