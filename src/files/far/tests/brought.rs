// What is brought from and sent to a server, whole or not at all.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::files::trash;
use crate::files::app::{Files, Msg};
use crate::files::remote::{Address, Login};
use crate::files::work::{self, Answer, Ctl, Kind, News, Plan, Step};
use crate::common::gentle;

use super::*;

#[test]
fn what_the_drive_refuses_of_files_brought_from_a_server_is_said_and_not_called_done() {
    let (here, far, base) = place("refused");
    fs::write(far.join("good.txt"), b"good").unwrap();
    gentle::faults::set(&here.join("song.txt"));
    let across = AtomicU64::new(0);
    let ctl = Ctl { fixed: Some(Answer::Skip), ..Ctl::default() };
    let plan = Plan { kind: Kind::Copy, items: vec![PathBuf::from(format!("{base}/song.txt")), PathBuf::from(format!("{base}/good.txt"))], to: here.clone(), key: None };
    let over = work::run(plan, &ctl, &|n| if let News::Files(n, _) = n { across.store(n, Ordering::SeqCst) });
    assert_eq!(over.failed.len(), 1, "{:?}", over.failed);
    assert!(over.failed[0].starts_with("song.txt ("), "{:?}", over.failed);
    assert!(!here.join("song.txt").exists(), "a copy the drive refused was left looking whole");
    assert_eq!(fs::read(here.join("good.txt")).unwrap(), b"good");
    assert_eq!(across.load(Ordering::SeqCst), 1, "a file the drive refused was counted as across");
    assert!(matches!(over.undo.as_slice(), [Step::Remove(made)] if *made == here.join("good.txt")), "what never came is not there to undo");
    assert_eq!(fs::read(far.join("song.txt")).unwrap().len(), 10_000, "the first is as it was");
    let (here2, far2, base2) = place("not-kept");
    gentle::faults::set(&here2);
    let over = go(Kind::Copy, vec![PathBuf::from(format!("{base2}/song.txt"))], here2.clone(), Answer::Skip);
    assert_eq!(over.troubles(), [format!("{} ({})", work::name_of(&here2), work::not_kept())]);
    assert_eq!(fs::read(here2.join("song.txt")).unwrap().len(), 10_000, "what was copied stays");
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far), fs::remove_dir_all(here2), fs::remove_dir_all(far2));
}

#[test]
fn opens_and_lists() {
    let (_, far, base) = place("list");
    let links = Links::default();
    let a = Address::parse(&base).unwrap();
    assert_eq!(links.open(&a, &Login::default(), None).unwrap(), "/");
    let mut names: Vec<String> = links.with(&a, |r| r.list("/")).unwrap().entries.into_iter().map(|e| e.name).collect();
    names.sort();
    assert_eq!(names, ["box", "song.txt"]);
    assert!(is_far(Path::new(&base)) && !is_far(&far));
    let _ = fs::remove_dir_all(far);
}

#[test]
fn copies_both_ways_and_undoes_only_what_came_here() {
    let (here, far, base) = place("copy");
    let over = go(Kind::Copy, vec![PathBuf::from(format!("{base}/song.txt")), PathBuf::from(format!("{base}/box"))],
                  here.join("box/inner"), Answer::Skip);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert_eq!(fs::read(here.join("box/inner/song.txt")).unwrap().len(), 10_000);
    assert_eq!(fs::read(here.join("box/inner/box/inner/one.txt")).unwrap(), b"one");
    assert_eq!(over.undo.len(), 2);
    let over = go(Kind::Copy, vec![here.join("box")], PathBuf::from(format!("{base}/box/inner")), Answer::Skip);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert_eq!(fs::read(far.join("box/inner/box/inner/song.txt")).unwrap().len(), 10_000);
    assert!(over.undo.is_empty());
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn moves_deletes_and_keeps_both() {
    let (here, far, base) = place("move");
    let over = go(Kind::Copy, vec![here.join("box")], PathBuf::from(&base), Answer::Both);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert!(far.join("box (2)/inner/one.txt").exists());
    let skipped = go(Kind::Copy, vec![here.join("box")], PathBuf::from(&base), Answer::Skip);
    assert_eq!(skipped.skipped, 1);
    let over = go(Kind::Move, vec![PathBuf::from(format!("{base}/song.txt"))], PathBuf::from(format!("{base}/box")), Answer::Skip);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert!(far.join("box/song.txt").exists() && !far.join("song.txt").exists());
    let over = go(Kind::Move, vec![PathBuf::from(format!("{base}/box (2)"))], here.clone(), Answer::Skip);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert!(here.join("box (2)/inner/one.txt").exists() && !far.join("box (2)").exists());
    let over = go(Kind::Delete, vec![PathBuf::from(format!("{base}/box"))], PathBuf::new(), Answer::Skip);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert!(!far.join("box").exists());
    let over = go(Kind::Copy, vec![here.join("box")], PathBuf::from(&base), Answer::Skip);
    assert!(over.failed.is_empty());
    let over = go(Kind::Copy, vec![PathBuf::from(format!("{base}/box"))], PathBuf::from(format!("{base}/box/inner")), Answer::Skip);
    assert_eq!(over.failed.len(), 1);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_move_removes_nothing_the_other_side_does_not_hold_whole() {
    let (here, far, base) = place("whole");
    fs::write(here.join("short.txt"), vec![3u8; 9000]).unwrap();
    let over = go(Kind::Move, vec![here.join("short.txt")], PathBuf::from(&base), Answer::Skip);
    assert_eq!(fs::read(here.join("short.txt")).map(|b| b.len()).ok(), Some(9000), "the first was removed though half of it arrived");
    assert_eq!(over.failed, ["short.txt (not all of it arrived, so it is still where it was)"]);
    assert!(!far.join("short.txt").exists(), "half a file was left there");
    fs::write(far.join("torn.txt"), vec![4u8; 9000]).unwrap();
    let over = go(Kind::Move, vec![PathBuf::from(format!("{base}/torn.txt"))], here.clone(), Answer::Skip);
    assert_eq!(fs::metadata(far.join("torn.txt")).map(|m| m.len()).ok(), Some(9000), "the first was removed though half of it arrived");
    assert_eq!(over.failed, ["torn.txt (not all of it arrived, so it is still where it was)"]);
    assert!(!here.join("torn.txt").exists(), "half a file was left here");
    let over = go(Kind::Move, vec![PathBuf::from(format!("{base}/song.txt"))], here.clone(), Answer::Skip);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert!(!far.join("song.txt").exists() && fs::read(here.join("song.txt")).unwrap() == vec![5u8; 10_000]);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_server_that_sends_more_than_it_listed_is_stopped_and_nothing_of_it_is_kept() {
    use crate::toolkit::ui::App;
    let mut got = Vec::new();
    let mut most = Most::new(&mut got, 5000, None);
    assert!(most.write_all(&vec![0; 5000 + OVER_MOST as usize]).is_ok() && !most.over);
    assert!(most.write_all(&[0]).is_err() && most.over);
    assert_eq!(got.len(), 5000 + OVER_MOST as usize);
    let mut got = Vec::new();
    let any = Most::new(&mut got, 0, None);
    assert_eq!(any.left, Some(UNLISTED_MOST));
    let mut got = Vec::new();
    let mut tight = Most::new(&mut got, 0, Some(100));
    assert!(tight.write_all(&[0; 100]).is_ok() && !tight.over);
    assert!(tight.write_all(&[0]).is_err() && tight.over);
    let mut got = Vec::new();
    let mut huge = Most::new(&mut got, u64::MAX, Some(50));
    assert!(huge.write_all(&[0; 51]).is_err() && huge.over, "a huge listed size is held to the room left");

    let (here, far, base) = place("endless");
    fs::write(far.join("endless.bin"), vec![9u8; 5000]).unwrap();
    let first = PathBuf::from(format!("{base}/endless.bin"));
    let said = [format!("endless.bin ({})", work::changed_why())];
    let over = go(Kind::Copy, vec![first.clone()], here.clone(), Answer::Skip);
    assert_eq!(over.failed, said);
    assert_eq!(there(&here), ["box"], "what came of it was left here");
    let over = go(Kind::Copy, vec![first.clone()], PathBuf::from(format!("{base}/box")), Answer::Skip);
    assert_eq!(over.failed, said);
    assert_eq!(there(&far.join("box")), ["inner"], "what came of it was left there");
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(base.clone())));
    f.update(Msg::Far(FarMsg::Go));
    f.far_open(&first);
    assert_eq!(f.said, format!("endless.bin cannot be fetched: {}.", work::changed_why()));
    let fetched = fetch_dir("test.invalid");
    assert!(!fetched.join("endless.bin").exists(), "what came of it was left in the fetched copies");
    let over = go(Kind::Copy, vec![PathBuf::from(format!("{base}/song.txt"))], here.clone(), Answer::Skip);
    assert!(over.failed.is_empty() && fs::read(here.join("song.txt")).unwrap() == vec![5u8; 10_000], "{:?}", over.failed);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_file_whose_size_cannot_be_had_is_not_fetched() {
    use crate::toolkit::ui::App;
    let (here, far, base) = place("nostat");
    fs::write(far.join("nostat.bin"), vec![9u8; 50]).unwrap();
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(base.clone())));
    f.update(Msg::Far(FarMsg::Go));
    f.far_open(&PathBuf::from(format!("{base}/nostat.bin")));
    assert!(f.said.contains("cannot be looked at"), "{}", f.said);
    assert!(!fetch_dir("test.invalid").join("nostat.bin").exists());
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_folder_that_came_in_part_from_a_server_is_not_removed_there() {
    let (here, far, base) = place("greedy");
    fs::write(far.join("box/inner/torn.txt"), vec![4u8; 9000]).unwrap();
    fs::create_dir_all(here.join("in")).unwrap();
    let over = go(Kind::Move, vec![PathBuf::from(format!("{base}/box"))], here.join("in"), Answer::Skip);
    assert_eq!(fs::metadata(far.join("box/inner/torn.txt")).map(|m| m.len()).ok(), Some(9000), "a file that never arrived was removed with its folder");
    assert!(here.join("in/box/inner/one.txt").exists() && !far.join("box/inner/one.txt").exists(), "what did arrive was not moved");
    assert_eq!(over.failed, ["torn.txt (not all of it arrived, so it is still where it was)", "box (not moved: some items)"]);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn replacing_a_file_keeps_the_old_one_until_the_new_is_whole() {
    let (here, far, base) = place("replace");
    let there = |far: &Path| {
        let mut names: Vec<String> = fs::read_dir(far).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        names.sort();
        names
    };
    fs::write(far.join("fails.txt"), b"the old one").unwrap();
    fs::write(here.join("fails.txt"), vec![1u8; 9000]).unwrap();
    let over = go(Kind::Copy, vec![here.join("fails.txt")], PathBuf::from(&base), Answer::Replace);
    assert_eq!(fs::read(far.join("fails.txt")).ok().as_deref(), Some(&b"the old one"[..]), "the old one was removed before the new one had arrived");
    assert_eq!(over.failed.len(), 1, "{:?}", over.failed);
    assert_eq!(there(&far), ["box", "fails.txt", "song.txt"], "nothing is left beside it");
    fs::write(here.join("song.txt"), b"a new song").unwrap();
    let over = go(Kind::Copy, vec![here.join("song.txt")], PathBuf::from(&base), Answer::Replace);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert_eq!(fs::read(far.join("song.txt")).unwrap(), b"a new song");
    assert_eq!(there(&far), ["box", "fails.txt", "song.txt"]);
    fs::write(far.join("song.txt"), vec![5u8; 10_000]).unwrap();
    let bin = trash::Bin::at(&here.join("Trash"), None);
    bin.ready().unwrap();
    let ctl = Ctl { fixed: Some(Answer::Replace), bin: Some(bin.clone()), ..Ctl::default() };
    let over = work::run(Plan { kind: Kind::Copy, items: vec![PathBuf::from(format!("{base}/song.txt"))], to: here.clone(), key: None }, &ctl, &|_| {});
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert_eq!(fs::read(here.join("song.txt")).unwrap().len(), 10_000);
    assert_eq!(fs::read(bin.files.join("song.txt")).ok().as_deref(), Some(&b"a new song"[..]), "the file replaced was removed for good");
    let back = work::run(Plan { kind: Kind::Undo(over.undo), items: Vec::new(), to: here.clone(), key: None }, &ctl, &|_| {});
    assert!(back.failed.is_empty(), "{:?}", back.failed);
    assert_eq!(fs::read(here.join("song.txt")).unwrap(), b"a new song");
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_file_on_a_server_that_is_replaced_is_not_lost_when_the_new_one_cannot_be_named() {
    let (here, far, base) = place("replace-stuck");
    fs::write(far.join("stuck.txt"), b"the old one").unwrap();
    fs::write(here.join("stuck.txt"), vec![1u8; 9000]).unwrap();
    let over = go(Kind::Copy, vec![here.join("stuck.txt")], PathBuf::from(&base), Answer::Replace);
    assert_eq!(fs::read(far.join("stuck.txt")).ok().as_deref(), Some(&b"the old one"[..]), "the old one was lost: {:?}", over.failed);
    assert_eq!(over.failed.len(), 1, "{:?}", over.failed);
    assert_eq!(there(&far), ["box", "song.txt", "stuck.txt"], "nothing is left beside it");
    fs::write(far.join("wedged.txt"), b"the old one").unwrap();
    fs::write(here.join("wedged.txt"), vec![1u8; 100]).unwrap();
    let over = go(Kind::Copy, vec![here.join("wedged.txt")], PathBuf::from(&base), Answer::Replace);
    let spares: Vec<String> = there(&far).into_iter().filter(|n| n.starts_with("wedged.txt.") && n.ends_with(SPARE)).collect();
    assert_eq!(spares.len(), 2, "{:?}", there(&far));
    let mut held: Vec<Vec<u8>> = spares.iter().map(|n| fs::read(far.join(n)).unwrap()).collect();
    held.sort();
    assert_eq!(held, [vec![1u8; 100], b"the old one".to_vec()], "the old one and the new one are both whole");
    assert!(over.failed.len() == 1 && spares.iter().all(|n| over.failed[0].contains(n.as_str())), "where each lies is said: {:?}", over.failed);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn on_a_server_that_cannot_refuse_what_came_in_the_last_moment_is_not_written_over() {
    let (here, far, base) = place("careless-raced");
    let theirs: &[u8] = b"another person's, saved in the very moment of the rename";
    fs::write(far.join("report.txt"), b"the old one").unwrap();
    fs::write(here.join("report.txt"), b"brought from here").unwrap();
    let target = far.join("report.txt");
    fake::as_named(&far, move || fs::write(&target, theirs).unwrap());
    let over = go(Kind::Copy, vec![here.join("report.txt")], PathBuf::from(&base), Answer::Replace);
    assert_eq!(fs::read(far.join("report.txt")).unwrap(), theirs, "what was never asked about was written over");
    assert!(over.failed.iter().any(|f| f.contains(work::taken_meanwhile())), "{:?}", over.failed);
    assert_eq!(there(&far), ["box", "report.txt", "song.txt"], "nothing is left under a spare name");
    fs::write(far.join("report.txt"), b"the old one").unwrap();
    let over = go(Kind::Copy, vec![here.join("report.txt")], PathBuf::from(&base), Answer::Replace);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert_eq!(fs::read(far.join("report.txt")).unwrap(), b"brought from here");
    assert_eq!(there(&far), ["box", "report.txt", "song.txt"]);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_file_here_that_is_replaced_stays_until_the_new_one_is_whole() {
    let (here, far, base) = place("replace-here");
    let bin = trash::Bin::at(&here.join("Trash"), None);
    bin.ready().unwrap();
    let ctl = Ctl { fixed: Some(Answer::Replace), bin: Some(bin.clone()), ..Ctl::default() };
    for name in ["torn.txt", "torn too.txt"] {
        fs::write(far.join(name), vec![5u8; 10_000]).unwrap();
        fs::write(here.join(name), b"the old one").unwrap();
        let over = work::run(Plan { kind: Kind::Copy, items: vec![PathBuf::from(format!("{base}/{name}"))], to: here.clone(), key: None }, &ctl, &|_| {});
        assert_eq!(over.failed.len(), 1, "{name}: {:?}", over.failed);
        assert_eq!(fs::read(here.join(name)).ok().as_deref(), Some(&b"the old one"[..]), "{name}: the old one made way for a copy that never came whole");
        assert!(over.undo.is_empty(), "{name}: nothing was done, so there is nothing to undo: {:?}", over.undo);
    }
    assert_eq!(there(&here), ["Trash", "box", "torn too.txt", "torn.txt"], "nothing is left beside them");
    assert_eq!(fs::read_dir(&bin.files).unwrap().count(), 0, "nothing went to the Trash");
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}
