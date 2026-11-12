//! What is said of work, and what is refused while it goes.

use std::fs;
use std::path::{Path, PathBuf};

use crate::files::trash;
use crate::files::work::{self, Ctl, Kind, Over, Plan, Step};
use super::*;

#[test]
fn a_drive_that_would_not_keep_what_was_written_is_always_said_and_never_counted() {
    let over = |failed: usize, not_kept: &[&str]| Over {
        failed: (1..=failed).map(|n| format!("f{n:03}.bin")).collect(),
        not_kept: not_kept.iter().map(|s| s.to_string()).collect(),
        ..Over::default()
    };
    let kept = work::not_kept();
    assert_eq!(failed_words(&over(0, &["stick"]), "copied"), format!("One item could not be copied: stick ({kept})."));
    assert_eq!(failed_words(&over(5, &["stick"]), "copied"), format!("stick: {kept}. 5 items could not be copied: f001.bin, f002.bin, f003.bin and 2 more."));
    let many = failed_words(&over(77, &["stick"]), "copied");
    assert_eq!(many, format!("stick: {kept}. 77 items could not be copied: f001.bin, f002.bin, f003.bin and 74 more."));
    assert_eq!(failed_words(&over(1, &["stick"]), "moved"), format!("stick: {kept}. One item could not be moved: f001.bin."));
    assert_eq!(failed_words(&over(2, &[]), "copied"), "2 items could not be copied: f001.bin, f002.bin.");
    let code = crate::common::source::code_all(crate::files::act::SOURCE);
    let finished = code.split_once("    pub(super) fn finished(").unwrap().1.split_once("\n    }\n").unwrap().0;
    assert!(finished.contains("over.troubled()") && finished.contains("failed_words(&over") && !finished.contains("failed.len()"), "{finished}");
    let dir = std::env::temp_dir().join(format!("trindle-act-not-kept-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("to")).unwrap();
    let items: Vec<PathBuf> = (0..5).map(|n| dir.join(format!("f{n}.bin"))).collect();
    for item in &items {
        std::fs::write(item, vec![7u8; 4096]).unwrap();
    }
    crate::common::gentle::faults::set(&dir.join("to"));
    let done = work::run(Plan { kind: Kind::Copy, items, to: dir.join("to"), key: None }, &Ctl::default(), &|_| {});
    assert_eq!(failed_words(&done, "copied"), format!("One item could not be copied: to ({kept})."));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_job_knows_where_it_changes_sizes() {
    let plan = |kind, items: &[&str], to: &str| Plan { kind, items: items.iter().map(PathBuf::from).collect(), to: to.into(), key: None };
    let p = |s: &str| PathBuf::from(s);
    assert_eq!(touched(&plan(Kind::Copy, &["/a/x", "/a/y"], "/b"), None), Some(vec![p("/b/x"), p("/b/y")]));
    assert_eq!(touched(&plan(Kind::Move, &["/a/x"], "/b"), None), Some(vec![p("/a/x"), p("/b/x")]));
    assert_eq!(touched(&plan(Kind::Delete, &["/a/x"], "/"), None), Some(vec![p("/a/x")]));
    let bin = trash::Bin::at(Path::new("/t"), None);
    assert_eq!(touched(&plan(Kind::Trash, &["/a/x"], "/"), Some(&bin)), Some(vec![p("/a/x"), bin.files.clone()]));
    let undo = Kind::Undo(vec![Step::Back { now: p("/b/x"), was: p("/a/x") }, Step::Remove(p("/c/y"))]);
    assert_eq!(touched(&plan(undo, &[], "/"), None), Some(vec![p("/a/x"), p("/b/x"), p("/c/y")]));
    assert_eq!(touched(&plan(Kind::Undo(vec![Step::Restore(p("/t/x"))]), &[], "/"), None), None);
}

#[test]
fn what_is_being_moved_is_refused_a_second_time() {
    let dir = folder("busy");
    let mut f = files(&dir);
    let mut job = Job::sample(1, "Moving Holiday to right", 100, 10);
    job.from = vec![dir.join("left/Holiday")];
    f.jobs.push(job);
    let plan = |kind, item: &str| Plan { kind, items: vec![dir.join(item)], to: dir.join("right"), key: None };
    for kind in [Kind::Move, Kind::Copy, Kind::Trash, Kind::Delete] {
        assert_eq!(f.busy_with(&plan(kind.clone(), "left/Holiday")), Some(being_moved()));
        assert_eq!(f.busy_with(&plan(kind, "left/Holiday/inside.txt")), Some(being_moved()));
    }
    assert_eq!(f.busy_with(&plan(Kind::Move, "left")), Some(being_moved()));
    assert_eq!(f.busy_with(&plan(Kind::Move, "left/Other")), None);
    assert_eq!(f.busy_with(&plan(Kind::Move, "left/Holiday.1234.trindle-part")), Some(arriving()));
    assert!(f.is_busy(&dir.join("left/Holiday/inside.txt")) && !f.is_busy(&dir.join("left/Other")));
    let words = || Words { doing: "Moving".into(), done: "Moved.".into(), verb: "could not be moved", help: crate::files::help::WORK_NOT_DONE, undo: String::new() };
    f.start(plan(Kind::Move, "left/Holiday"), words(), Turn::New);
    assert_eq!(f.jobs.len(), 1, "no second job");
    assert_eq!(f.said, being_moved());
    let _ = fs::remove_dir_all(&dir);
}
