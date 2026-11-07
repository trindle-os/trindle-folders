// The longer work the archive, wipe and picture engines do, run as Files' other work is.

use std::sync::atomic::Ordering;

use crate::toolkit::t;

use super::archive;
use super::imaging::{self, Work};
use super::wipe::{self, Refused};
use super::work::{name_of, Ctl, Extra, Kind, News, Over, Plan, Step, NAP};

pub fn run(plan: &Plan, ctl: &Ctl, tell: &dyn Fn(News)) -> Over {
    let mut over = Over::default();
    let total = std::cell::Cell::new(None::<u64>);
    let told = |done: u64, all: u64| {
        while ctl.pause.load(Ordering::Relaxed) && !ctl.stop.load(Ordering::Relaxed) {
            std::thread::sleep(NAP);
        }
        if total.get() != Some(all) {
            total.set(Some(all));
            tell(News::Total(all));
        }
        tell(News::Done(done));
    };
    match &plan.kind {
        Kind::Other(Extra::Pack(format, name)) => pack(plan, ctl, &told, &mut over, *format, name),
        Kind::Other(Extra::Unpack(place)) => unpack(plan, ctl, &told, &mut over, *place),
        Kind::Other(Extra::TakeOut(from)) => take_out(plan, ctl, &told, &mut over, from),
        Kind::Other(Extra::Picture(work)) => pictures(plan, ctl, tell, &mut over, work),
        Kind::Other(Extra::Wipe(passes)) => wipe_all(plan, ctl, &told, &mut over, *passes),
        _ => {}
    }
    over.stopped = ctl.stop.load(Ordering::Relaxed);
    over
}

fn pack(plan: &Plan, ctl: &Ctl, told: &dyn Fn(u64, u64), over: &mut Over, format: archive::Format, name: &str) {
    let end = format.ending();
    let to = (1..)
        .map(|n| plan.to.join(if n == 1 { format!("{name}{end}") } else { format!("{name} ({n}){end}") }))
        .find(|p| std::fs::symlink_metadata(p).is_err())
        .unwrap_or_default();
    match archive::compress(&plan.items, &to, format, &ctl.stop, told) {
        Ok(()) => over.undo.push(Step::Remove(to)),
        Err(_) => over.failed.push(name_of(&to)),
    }
}

fn unpack(plan: &Plan, ctl: &Ctl, told: &dyn Fn(u64, u64), over: &mut Over, place: archive::Place) {
    for item in &plan.items {
        if ctl.stop.load(Ordering::Relaxed) {
            break;
        }
        match archive::extract(item, plan.key.as_ref(), &plan.to, place, &ctl.stop, told) {
            Ok(done) => {
                over.undo.extend(done.made.into_iter().map(Step::Remove));
                over.failed.extend(done.refused);
            }
            Err(e) => refused(over, item, &e),
        }
    }
}

fn take_out(plan: &Plan, ctl: &Ctl, told: &dyn Fn(u64, u64), over: &mut Over, from: &std::path::Path) {
    let names: Vec<String> = plan.items.iter().filter_map(|i| super::inside::inner(from, i)).collect();
    match archive::take_out(from, plan.key.as_ref(), &names, &plan.to, &ctl.stop, told) {
        Ok(done) => {
            over.undo.extend(done.made.into_iter().map(Step::Remove));
            over.failed.extend(done.refused);
        }
        Err(e) => refused(over, from, &e),
    }
}

fn pictures(plan: &Plan, ctl: &Ctl, tell: &dyn Fn(News), over: &mut Over, work: &Work) {
    tell(News::Total(plan.items.len() as u64));
    for (n, item) in plan.items.iter().enumerate() {
        while ctl.pause.load(Ordering::Relaxed) && !ctl.stop.load(Ordering::Relaxed) {
            std::thread::sleep(NAP);
        }
        if ctl.stop.load(Ordering::Relaxed) {
            break;
        }
        let done = match work {
            Work::Turn(turn) => imaging::turn(item, *turn),
            Work::Resize(size) => imaging::resize(item, *size).map(|copy| over.undo.push(Step::Remove(copy))),
        };
        if done.is_err() {
            over.failed.push(name_of(item));
        }
        tell(News::Done(n as u64 + 1));
    }
}

fn wipe_all(plan: &Plan, ctl: &Ctl, told: &dyn Fn(u64, u64), over: &mut Over, passes: u32) {
    match wipe::wipe_asked(&ctl.asked, &plan.items, passes, &ctl.stop, told) {
        Ok(done) => {
            for (path, why) in done.refused {
                over.failed.push(t!("folders-jobs-failed-why", name = &name_of(&path), why = refused_words(why)));
            }
            for gone in &done.gone {
                super::trash::forget(gone);
            }
        }
        Err(_) => over.failed.extend(plan.items.iter().map(|p| name_of(p))),
    }
}

fn refused(over: &mut Over, archive: &std::path::Path, e: &std::io::Error) {
    match archive::wants_key(e) {
        Some(t) => over.locked.push((archive.to_path_buf(), t)),
        None => over.failed.push(with_why(name_of(archive), e)),
    }
}

fn with_why(name: String, e: &std::io::Error) -> String {
    if super::meter::is_stopped(e) { name } else { t!("folders-jobs-failed-why", name = &name, why = &archive::why(e)) }
}

fn refused_words(why: Refused) -> &'static str {
    match why {
        Refused::OtherNames => t!("folders-jobs-refused-names"),
        Refused::NotAFile => t!("folders-jobs-refused-not-file"),
        Refused::Changed => t!("folders-jobs-refused-changed"),
        Refused::Failed(_) => t!("folders-jobs-refused-disk"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn a_failed_archive_removes_nothing_under_the_name_it_would_have_had() {
        let dir = std::env::temp_dir().join(format!("trindle-files-jobs-pack-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("out")).unwrap();
        std::fs::write(dir.join("big.bin"), vec![7u8; 4 << 20]).unwrap();
        let ctl = Ctl::default();
        let (stop, theirs) = (ctl.stop.clone(), dir.join("out/box.zip"));
        let plan = Plan { kind: Kind::Other(Extra::Pack(archive::Format::Zip, "box".into())), items: vec![dir.join("big.bin")], to: dir.join("out"), key: None };
        let over = run(&plan, &ctl, &|news| {
            if let News::Total(_) = news {
                std::fs::write(&theirs, b"another program's own").unwrap();
                stop.store(true, Ordering::Relaxed);
            }
        });
        assert!(over.stopped || !over.failed.is_empty());
        assert_eq!(std::fs::read(&theirs).ok().as_deref(), Some(&b"another program's own"[..]), "a file the work never made was removed");
        let left: Vec<PathBuf> = std::fs::read_dir(dir.join("out")).unwrap().flatten().map(|e| e.path()).collect();
        assert_eq!(left, std::slice::from_ref(&theirs), "and nothing half made is left");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn an_archive_is_numbered_when_a_folder_or_a_file_has_its_name() {
        let dir = std::env::temp_dir().join(format!("trindle-files-jobs-pack-names-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("out/box.zip")).unwrap();
        std::fs::write(dir.join("out/box (2).zip"), b"a file").unwrap();
        std::fs::write(dir.join("one.txt"), b"one").unwrap();
        let plan = Plan { kind: Kind::Other(Extra::Pack(archive::Format::Zip, "box".into())), items: vec![dir.join("one.txt")], to: dir.join("out"), key: None };
        let over = run(&plan, &Ctl::default(), &|_| {});
        assert!(over.failed.is_empty(), "{:?}", over.failed);
        assert!(dir.join("out/box (3).zip").is_file());
        assert!(dir.join("out/box.zip").is_dir() && std::fs::read(dir.join("out/box (2).zip")).unwrap() == b"a file");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
