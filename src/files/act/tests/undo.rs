//! The question, the buttons, undo and redo, the Trash and what is cut.

use std::fs;

use crate::files::app::Msg;
use crate::files::work::{self, Answer, News, Step};
use crate::toolkit::ui::App;
use super::*;

#[test]
fn a_question_reads_the_two_that_clash_and_starts_with_its_switch_off() {
    let dir = folder("ask");
    fs::write(dir.join("left/report.txt"), b"the newer and longer one").unwrap();
    fs::write(dir.join("right/report.txt"), b"old").unwrap();
    fs::create_dir_all(dir.join("left/Photos")).unwrap();
    fs::create_dir_all(dir.join("right/Photos")).unwrap();
    for n in 0..3 {
        fs::write(dir.join("right/Photos").join(format!("{n}.png")), b"x").unwrap();
    }
    let mut f = files(&dir);
    f.jobs.push(Job::sample(7, "Copying 2 items to right", 100, 10));
    let files_clash = work::Clash { name: "report.txt".into(), folders: false, replace: true,
                                    sides: Some((dir.join("left/report.txt"), dir.join("right/report.txt"))) };
    f.update(Msg::Work(7, News::Clash(files_clash)));
    let a = f.jobs[0].asking.as_ref().unwrap();
    assert_eq!((a.coming.as_ref().unwrap().size, a.there.as_ref().unwrap().size), (24, 3));
    assert!(a.holds.is_none());
    assert!(f.busy().asking);
    f.update(Msg::AllSame(7, true));
    f.update(Msg::Answer(7, Answer::Skip));
    assert!(f.jobs[0].asking.is_none() && f.jobs[0].all);
    let folders_clash = work::Clash { name: "Photos".into(), folders: true, replace: true,
                                      sides: Some((dir.join("left/Photos"), dir.join("right/Photos"))) };
    f.update(Msg::Work(7, News::Clash(folders_clash)));
    let a = f.jobs[0].asking.as_ref().unwrap();
    assert!(!f.jobs[0].all);
    assert_eq!(a.holds, Some((0, 3)));
    f.update(Msg::Work(7, News::Clash(work::Clash { name: "far.txt".into(), folders: false, sides: None, replace: true })));
    let a = f.jobs[0].asking.as_ref().unwrap();
    assert!(a.coming.is_none() && a.there.is_none());
    let _ = f.view();
    f.jobs.clear();
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_copy_button_copies_and_undo_takes_it_back() {
    let dir = folder("copy");
    fs::write(dir.join("left/photo.jpg"), b"picture").unwrap();
    let mut f = files(&dir);
    pick(&mut f, 0, "photo.jpg");
    f.update(Msg::Copy);
    assert_eq!(fs::read(dir.join("right/photo.jpg")).unwrap(), b"picture");
    assert!(f.said.starts_with("Copied photo.jpg"), "{}", f.said);
    assert_eq!(f.undo.len(), 1);
    f.update(Msg::Undo);
    assert!(!dir.join("right/photo.jpg").exists());
    assert!(dir.join("left/photo.jpg").exists());
    assert!(f.undo.is_empty());
    assert!(dir.join("Trash/files/photo.jpg").exists());
    fs::remove_dir_all(&dir).unwrap();
}

fn a_copy_is_undone_and_done_again(f: &mut Files, dir: &Path) {
    pick(f, 0, "photo.jpg");
    f.update(Msg::Copy);
    assert!(f.redo.is_empty());
    f.update(Msg::Undo);
    assert!(!dir.join("right/photo.jpg").exists());
    assert_eq!(f.redo.last().unwrap().0, "Redo copying photo.jpg");
    assert!(f.undo.is_empty());
    f.update(Msg::Redo);
    assert_eq!(fs::read(dir.join("right/photo.jpg")).unwrap(), b"picture");
    assert!(f.said.starts_with("Redid copying photo.jpg"), "{}", f.said);
    assert!(f.redo.is_empty());
    assert_eq!(f.undo.last().unwrap().0, "Undo copying photo.jpg");
    f.update(Msg::Undo);
    assert!(!dir.join("right/photo.jpg").exists());
    f.update(Msg::Redo);
    assert!(dir.join("right/photo.jpg").exists() && dir.join("left/photo.jpg").exists());
}

fn a_move_the_trash_and_a_new_name_are_redone(f: &mut Files, dir: &Path) {
    pick(f, 0, "notes.txt");
    f.update(Msg::Move);
    f.update(Msg::Undo);
    assert!(dir.join("left/notes.txt").exists() && !dir.join("right/notes.txt").exists());
    f.update(Msg::Redo);
    assert!(!dir.join("left/notes.txt").exists() && dir.join("right/notes.txt").exists());
    pick(f, 1, "notes.txt");
    f.update(Msg::Trash);
    f.update(Msg::Undo);
    assert!(dir.join("right/notes.txt").exists());
    f.update(Msg::Redo);
    assert!(!dir.join("right/notes.txt").exists() && dir.join("Trash/files/notes.txt").exists());
    assert_eq!(f.undo.last().unwrap().0, "Undo putting notes.txt in the Trash");
    pick(f, 0, "photo.jpg");
    f.update(Msg::Rename);
    f.update(Msg::RenameText("picture.jpg".into()));
    f.update(Msg::RenameDone);
    f.update(Msg::Undo);
    assert!(dir.join("left/photo.jpg").exists() && !dir.join("left/picture.jpg").exists());
    assert_eq!(f.redo.last().unwrap().0, "Redo renaming photo.jpg to picture.jpg");
    f.update(Msg::Redo);
    assert!(dir.join("left/picture.jpg").exists() && !dir.join("left/photo.jpg").exists());
}

fn a_new_folder_is_redone_and_something_new_clears_it(f: &mut Files, dir: &Path) {
    f.side = 0;
    f.update(Msg::NewFolder);
    f.update(Msg::RenameStop);
    f.update(Msg::Undo);
    assert!(!dir.join("left/New folder").exists());
    f.update(Msg::Redo);
    assert!(dir.join("left/New folder").is_dir());
    f.update(Msg::Undo);
    assert_eq!(f.redo.len(), 1);
    pick(f, 0, "picture.jpg");
    f.update(Msg::Trash);
    assert!(f.redo.is_empty());
    f.update(Msg::Redo);
    assert_eq!(f.said, "There is nothing to redo.");
    f.update(Msg::Undo);
    assert_eq!(f.redo.len(), 1);
    pick(f, 0, "picture.jpg");
    f.update(Msg::Delete);
    f.update(Msg::DeleteSure);
    assert!(f.redo.is_empty(), "{:?}", f.redo);
    for n in 0..UNDO_KEPT + 3 {
        keep(&mut f.redo, format!("Redo making {n}"), vec![Step::Remove(dir.join("x"))]);
    }
    assert_eq!(f.redo.len(), UNDO_KEPT);
}

#[test]
fn redo_does_again_what_undo_took_back_and_a_new_action_clears_it() {
    let dir = folder("redo");
    fs::write(dir.join("left/photo.jpg"), b"picture").unwrap();
    fs::write(dir.join("left/notes.txt"), b"notes").unwrap();
    let mut f = files(&dir);
    f.update(Msg::Redo);
    assert_eq!(f.said, "There is nothing to redo.");
    a_copy_is_undone_and_done_again(&mut f, &dir);
    a_move_the_trash_and_a_new_name_are_redone(&mut f, &dir);
    a_new_folder_is_redone_and_something_new_clears_it(&mut f, &dir);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_new_folder_is_named_in_place() {
    let dir = folder("new");
    let mut f = files(&dir);
    f.update(Msg::NewFolder);
    assert!(dir.join("left/New folder").is_dir());
    assert!(f.renaming.as_ref().is_some_and(|r| r.fresh));
    f.update(Msg::RenameText("a/b".into()));
    f.update(Msg::RenameDone);
    assert!(f.bad && f.renaming.is_some());
    f.update(Msg::RenameText("Holiday".into()));
    f.update(Msg::RenameDone);
    assert!(dir.join("left/Holiday").is_dir());
    assert!(f.renaming.is_none());
    let pane = &f.cols[0];
    assert_eq!(pane.chosen.map(|i| pane.entries[i].name.clone()).as_deref(), Some("Holiday"));
    assert_eq!(f.undo.len(), 1);
    assert_eq!(f.undo[0].0, "Undo making Holiday");
    f.update(Msg::RenameStop);
    f.update(Msg::NewDocument);
    f.update(Msg::RenameStop);
    f.update(Msg::NewDocument);
    assert!(dir.join("left/New document (2)").is_file());
    f.update(Msg::RenameStop);
    pick(&mut f, 0, "Holiday");
    f.update(Msg::Rename);
    f.update(Msg::RenameText("Trips".into()));
    f.update(Msg::RenameDone);
    assert!(dir.join("left/Trips").is_dir());
    f.update(Msg::Undo);
    assert!(dir.join("left/Holiday").is_dir() && !dir.join("left/Trips").exists());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn emptying_the_trash_asks_first_and_takes_the_notes() {
    let dir = folder("empty");
    fs::write(dir.join("left/a.txt"), b"a").unwrap();
    fs::write(dir.join("left/.b"), b"b").unwrap();
    let mut f = files(&dir);
    f.update(Msg::Hidden);
    pick(&mut f, 0, "a.txt");
    f.update(Msg::Trash);
    pick(&mut f, 0, ".b");
    f.update(Msg::Trash);
    f.update(Msg::EmptyTrash);
    assert!(f.emptying && f.confirm.as_ref().is_some_and(|c| c.len() == 2));
    f.update(Msg::NoDelete);
    assert!(dir.join("Trash/files/a.txt").exists());
    f.update(Msg::EmptyTrash);
    f.update(Msg::DeleteSure);
    assert_eq!(fs::read_dir(dir.join("Trash/files")).unwrap().count(), 0);
    assert_eq!(fs::read_dir(dir.join("Trash/info")).unwrap().count(), 0);
    f.update(Msg::EmptyTrash);
    assert!(f.confirm.is_none());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn cut_items_are_known_until_pasted() {
    let dir = folder("cut");
    fs::write(dir.join("left/a.txt"), b"a").unwrap();
    let mut f = files(&dir);
    pick(&mut f, 0, "a.txt");
    f.update(Msg::Cut);
    assert!(f.is_cut(&dir.join("left/a.txt")));
    f.update(Msg::CopyClip);
    assert!(!f.is_cut(&dir.join("left/a.txt")));
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn deleting_asks_first_and_the_trash_can_be_undone() {
    let dir = folder("delete");
    fs::write(dir.join("left/old.txt"), b"old").unwrap();
    fs::write(dir.join("left/keep.txt"), b"keep").unwrap();
    let mut f = files(&dir);
    pick(&mut f, 0, "old.txt");
    f.update(Msg::Delete);
    assert!(f.confirm.is_some() && dir.join("left/old.txt").exists());
    f.update(Msg::NoDelete);
    assert!(f.confirm.is_none() && dir.join("left/old.txt").exists());
    f.update(Msg::Delete);
    f.update(Msg::DeleteSure);
    assert!(!dir.join("left/old.txt").exists());
    assert!(f.undo.is_empty());
    pick(&mut f, 0, "keep.txt");
    f.update(Msg::Trash);
    assert!(!dir.join("left/keep.txt").exists());
    assert!(dir.join("Trash/files/keep.txt").exists());
    assert!(dir.join("Trash/info/keep.txt.trashinfo").exists());
    f.update(Msg::Undo);
    assert!(dir.join("left/keep.txt").exists());
    assert!(!dir.join("Trash/info/keep.txt.trashinfo").exists());
    pick(&mut f, 0, "keep.txt");
    f.update(Msg::Trash);
    assert_eq!(f.undo.len(), 1);
    f.ask_about(vec![dir.join("Trash/files/keep.txt")]);
    f.update(Msg::DeleteSure);
    assert!(f.undo.is_empty() && !dir.join("Trash/files/keep.txt").exists());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_refused_delete_keeps_undo_and_escape_drops_the_whole_question() {
    let dir = folder("refused-delete");
    let a = dir.join("left/a.txt");
    fs::write(&a, b"a").unwrap();
    let mut f = files(&dir);
    f.remember("Undo making a.txt".into(), vec![Step::Remove(a.clone())]);
    let mut moving = super::super::Job::sample(9, "Moving a.txt", 1, 0);
    moving.from = vec![a.clone()];
    f.jobs.push(moving);
    f.confirm = Some(vec![a.clone()]);
    f.update(Msg::DeleteSure);
    assert!(a.exists());
    assert_eq!(f.undo.len(), 1, "a delete that never started forgot the Undo");
    f.jobs.clear();
    f.update(Msg::EmptyTrash);
    pick(&mut f, 0, "a.txt");
    f.update(Msg::Delete);
    assert!(f.confirm.is_some() && !f.asked.is_empty());
    f.update(Msg::Escape);
    assert!(f.confirm.is_none() && f.asked.is_empty() && !f.emptying);
    fs::remove_dir_all(&dir).unwrap();
}
