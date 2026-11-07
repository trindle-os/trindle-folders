// Tests of files dragged between Folders and other programs.

use crate::common::info_popup_tests::{shuts_like_every_other, Program};

#[test]
fn the_i_shuts_on_a_press_anywhere_outside_its_words() {
    let dir = folder("the-i");
    let mut f = files_in(&dir);
    let said = "Not moved: 1 unreadable file.".to_string();
    shuts_like_every_other(&mut f, Program {
        name: "Folders",
        inside: (40.0, 728.0),
        open: &|f| { f.said = said.clone(); f.reasons = Some((said.clone(), vec!["album/cover.jpg: no permission".into()])); f.reasons_open = true; },
        is_open: &|f| f.reasons_open,
        view: &crate::files::view::view,
        update: &|f, m| f.update(m),
        on_escape: &|f, i| f.intercept(i, false),
    });
    let _ = std::fs::remove_dir_all(&dir);
}

use super::*;
use crate::files::act::Drag;
use crate::files::app::View;
use crate::files::trash;
use crate::toolkit::dnd::{Offered, Ways, Why};
use crate::toolkit::input::Mods;
use crate::toolkit::ui::App;

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-files-outside-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for sub in ["here/box", "there"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    std::fs::write(dir.join("there/report.txt"), "the report").unwrap();
    std::fs::write(dir.join("there/notes.txt"), "the notes").unwrap();
    dir
}

fn files_in(dir: &Path) -> Files {
    let mut f = Files::new(vec![dir.join("here")]);
    f.home = dir.to_path_buf();
    f.bin = Some(trash::Bin::at(&dir.join("Trash"), None));
    f.bin.as_ref().unwrap().ready().unwrap();
    f.update(Msg::Changed);
    f.view = View::Icons;
    let pane = &f.cols[0];
    pane.width.set(440.0);
    pane.grid.set(crate::files::icons::Grid::icons(440.0));
    f
}

fn drop_on_pane(f: &mut Files, files: Vec<PathBuf>, way: Way) -> DropEnd {
    f.update(Msg::Outside(Onto::Icons(0, 0, 500.0, 300.0), Given::Files(files), way));
    f.drop_end(7, way)
}

#[test]
fn files_from_another_program_are_copied_into_the_folder_shown_and_undone() {
    let dir = folder("copy");
    let mut f = files_in(&dir);
    let from = vec![dir.join("there/report.txt"), dir.join("there/notes.txt")];
    assert_eq!(drop_on_pane(&mut f, from, Way::Copy), DropEnd::Done);
    assert!(!f.bad, "{}", f.said);
    assert_eq!(std::fs::read_to_string(dir.join("here/report.txt")).unwrap(), "the report");
    assert!(dir.join("here/notes.txt").exists());
    assert!(dir.join("there/report.txt").exists(), "a copy leaves the first where it was");
    f.update(Msg::Undo);
    assert!(!dir.join("here/report.txt").exists() && !dir.join("here/notes.txt").exists());
    assert!(dir.join("there/report.txt").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_move_another_program_asks_for_is_a_copy_and_only_shift_moves() {
    let dir = folder("move");
    let mut f = files_in(&dir);
    drop_on_pane(&mut f, vec![dir.join("there/report.txt")], Way::Move);
    assert!(dir.join("here/report.txt").exists());
    assert!(dir.join("there/report.txt").exists(), "nothing of another program's naming is removed");
    f.mods.set(Mods { shift: true, ..Mods::default() });
    drop_on_pane(&mut f, vec![dir.join("there/notes.txt")], Way::Move);
    assert!(dir.join("here/notes.txt").exists() && !dir.join("there/notes.txt").exists());
    f.update(Msg::Undo);
    assert!(dir.join("there/notes.txt").exists() && !dir.join("here/notes.txt").exists());
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("here/report.txt")], Way::Move), DropEnd::Failed);
    assert_eq!(f.said, "They are in that folder already.");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn what_cannot_be_trusted_or_used_is_refused_in_plain_words() {
    let dir = folder("refuse");
    let mut f = files_in(&dir);
    f.update(Msg::Outside(Onto::Icons(0, 0, 500.0, 300.0), Given::Text("hello".into()), Way::Copy));
    assert_eq!((f.said.as_str(), f.bad), ("Only files and folders can be dropped here.", false));
    assert_eq!(f.drop_end(1, Way::Copy), DropEnd::Failed);
    f.update(Msg::Outside(Onto::Icons(0, 0, 500.0, 300.0), Given::Nothing(Why::NotFiles), Way::Copy));
    assert_eq!((f.said.as_str(), f.bad), (Why::NotFiles.words(), true));
    for bad in [dir.join("there/nothing.txt"), dir.join("there/../there/report.txt"), PathBuf::from("report.txt")] {
        assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/notes.txt"), bad.clone()], Way::Copy), DropEnd::Failed, "{bad:?}");
        assert!(f.bad && !dir.join("here/notes.txt").exists(), "{bad:?}");
    }
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("here")], Way::Copy), DropEnd::Failed);
    assert_eq!(f.said, "A folder cannot be put inside itself.");
    assert_eq!(drop_on_pane(&mut f, vec![dir.clone()], Way::Copy), DropEnd::Failed);
    f.go(0, Where::Trash);
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/notes.txt")], Way::Copy), DropEnd::Failed);
    assert!(std::fs::read_dir(dir.join("here")).unwrap().count() == 1, "only the folder that was there");
    let _ = std::fs::remove_dir_all(&dir);
}

fn card_of(f: &Files) -> Vec<String> {
    match &f.dialog {
        Some(Dialog::Dropped(a)) => view(a).said(),
        _ => panic!("no question card: {}", f.said),
    }
}

fn told(f: &mut Files) -> Rc<std::cell::RefCell<Vec<(u64, bool)>>> {
    let log = Rc::new(std::cell::RefCell::new(Vec::new()));
    let l = log.clone();
    f.set_drop_hook(move |id, whole| l.borrow_mut().push((id, whole)));
    log
}

#[test]
fn what_another_program_names_from_a_private_place_is_asked_about_first() {
    let dir = folder("wary");
    let mut f = files_in(&dir);
    let log = told(&mut f);
    std::fs::create_dir_all(dir.join(".ssh")).unwrap();
    std::fs::write(dir.join(".ssh/id_made_up"), "a made-up key").unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join(".ssh/id_made_up")], Way::Copy), DropEnd::Later);
    assert!(!dir.join("here/id_made_up").exists() && f.jobs.is_empty());
    let said = card_of(&f);
    assert!(said.iter().any(|s| s == "Copy id_made_up here?"), "{said:?}");
    assert!(said.iter().any(|s| s.starts_with("Another program dropped it on Folders and named it itself.") && s.ends_with("nothing is copied until you say.")), "{said:?}");
    assert!(said.iter().any(|s| s == "id_made_up, from ~/.ssh: hidden, or in a hidden folder"), "{said:?}");
    assert!(said.iter().any(|s| s == "Into here."), "{said:?}");
    let buttons: Vec<&String> = said.iter().filter(|s| s.starts_with("button: ")).collect();
    assert_eq!(buttons, ["button: Leave it", "button: Copy it", "button: Leave it"], "{said:?}");
    f.update(Msg::Ask(crate::files::dialogs::AskMsg::Close));
    assert!(f.dialog.is_none() && !dir.join("here/id_made_up").exists());
    assert_eq!((f.said.as_str(), f.bad), ("Nothing was brought.", false));
    assert_eq!(*log.borrow(), [(7, false)]);
    assert_eq!(drop_on_pane(&mut f, vec![dir.join(".ssh/id_made_up")], Way::Copy), DropEnd::Later);
    f.update(Msg::OutsideAnswer(true));
    assert_eq!(std::fs::read_to_string(dir.join("here/id_made_up")).unwrap(), "a made-up key");
    assert_eq!(*log.borrow(), [(7, false), (7, true)]);
    f.update(Msg::Undo);
    assert!(!dir.join("here/id_made_up").exists() && dir.join(".ssh/id_made_up").exists());
    assert_eq!(drop_on_pane(&mut f, vec![PathBuf::from("/etc/hostname"), dir.join("there/notes.txt")], Way::Copy), DropEnd::Later);
    let said = card_of(&f);
    assert!(said.iter().any(|s| s == "Copy these 2 items here?"), "{said:?}");
    let named: Vec<&String> = said.iter().filter(|s| s.contains(", from ")).collect();
    assert_eq!(named, ["hostname, from /etc: outside your home folder and your drives", "notes.txt, from ~/there"], "those asked about come first");
    f.update(Msg::OutsideAnswer(false));
    assert!(!dir.join("here/hostname").exists() && !dir.join("here/notes.txt").exists(), "one answer for the whole drop");
    std::fs::create_dir_all(dir.join("there/project/src")).unwrap();
    std::fs::write(dir.join("there/project/src/.env"), "MADE_UP=1").unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/project")], Way::Copy), DropEnd::Later);
    assert!(card_of(&f).iter().any(|s| s == "project, from ~/there: has hidden things in it"), "{:?}", card_of(&f));
    f.update(Msg::OutsideAnswer(false));
    std::os::unix::fs::symlink(dir.join(".ssh"), dir.join("there/keys")).unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/keys/id_made_up")], Way::Copy), DropEnd::Later);
    assert!(card_of(&f).iter().any(|s| s.ends_with("hidden, or in a hidden folder")), "{:?}", card_of(&f));
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/report.txt")], Way::Copy), DropEnd::Failed);
    assert_eq!(f.said, "Answer the question about what was dropped first.");
    assert!(!dir.join("here/report.txt").exists());
    f.update(Msg::OutsideAnswer(false));
    let before = log.borrow().len();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join(".ssh/id_made_up")], Way::Copy), DropEnd::Later);
    f.dialog = None;
    f.update(Msg::Changed);
    assert_eq!(log.borrow()[before..], [(7, false)]);
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/report.txt")], Way::Copy), DropEnd::Done);
    assert!(f.dialog.is_none() && dir.join("here/report.txt").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_link_dropped_is_judged_by_where_it_leads_too() {
    let dir = folder("link");
    let mut f = files_in(&dir);
    std::fs::create_dir_all(dir.join(".ssh")).unwrap();
    std::fs::write(dir.join(".ssh/id_made_up"), "a made-up key").unwrap();
    std::os::unix::fs::symlink(dir.join(".ssh/id_made_up"), dir.join("there/invoice")).unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/invoice")], Way::Copy), DropEnd::Later, "a link to a hidden file came with no question");
    assert!(card_of(&f).iter().any(|s| s == "invoice, from ~/there: a link to something hidden, or outside your home folder and your drives"), "{:?}", card_of(&f));
    f.update(Msg::OutsideAnswer(false));
    assert!(std::fs::symlink_metadata(dir.join("here/invoice")).is_err());
    std::os::unix::fs::symlink("/no/such/place", dir.join("there/later")).unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/later")], Way::Copy), DropEnd::Later);
    f.update(Msg::OutsideAnswer(false));
    std::fs::create_dir_all(dir.join("there/album/2026")).unwrap();
    std::fs::write(dir.join("there/album/2026/sea.txt"), "the sea").unwrap();
    std::os::unix::fs::symlink(dir.join(".ssh"), dir.join("there/album/2026/more")).unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/album")], Way::Copy), DropEnd::Later, "a folder with a link to a hidden folder in it came with no question");
    assert!(card_of(&f).iter().any(|s| s == "album, from ~/there: has a link in it to something hidden, or outside your home folder and your drives"), "{:?}", card_of(&f));
    f.update(Msg::OutsideAnswer(true));
    assert!(std::fs::symlink_metadata(dir.join("here/album/2026/more")).unwrap().file_type().is_symlink());
    std::os::unix::fs::symlink(dir.join("there/report.txt"), dir.join("there/shortcut")).unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/shortcut")], Way::Copy), DropEnd::Done, "{}", f.said);
    assert_eq!(std::fs::read_link(dir.join("here/shortcut")).unwrap(), dir.join("there/report.txt"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn what_is_put_into_a_dropped_folder_after_it_was_looked_through_is_not_brought() {
    let dir = folder("filled");
    let mut f = files_in(&dir);
    std::fs::create_dir_all(dir.join(".ssh")).unwrap();
    std::fs::write(dir.join(".ssh/id_made_up"), "a made-up key").unwrap();
    std::fs::create_dir_all(dir.join("there/proj/src")).unwrap();
    std::fs::write(dir.join("there/proj/src/main.txt"), "the work").unwrap();
    std::fs::write(dir.join("there/.decoy"), "to put the card up").unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/.decoy"), dir.join("there/proj")], Way::Copy), DropEnd::Later);
    let said = card_of(&f);
    assert!(said.iter().any(|s| s == "proj, from ~/there"), "the folder was looked through and is ordinary: {said:?}");
    std::fs::write(dir.join("there/proj/src/.cache"), "hidden, and never asked about").unwrap();
    std::os::unix::fs::symlink(dir.join(".ssh"), dir.join("there/proj/way")).unwrap();
    std::fs::write(dir.join("there/proj/src/more.txt"), "ordinary").unwrap();
    f.update(Msg::OutsideAnswer(true));
    assert_eq!(std::fs::read_to_string(dir.join("here/proj/src/main.txt")).unwrap(), "the work");
    assert!(!dir.join("here/proj/src/.cache").exists(), "a hidden thing put there after the folder was looked through came unasked");
    assert!(std::fs::symlink_metadata(dir.join("here/proj/way")).is_err(), "a link put there after the folder was looked through came unasked");
    assert!(dir.join("here/proj/src/more.txt").exists(), "an ordinary file is brought as ever");
    assert!(f.bad && f.said.contains(".cache (it was not there when this was looked at, so it was left)"), "{}", f.said);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_move_of_what_is_asked_about_waits_for_the_answer_and_for_the_work() {
    let dir = folder("warymove");
    let mut f = files_in(&dir);
    let log = told(&mut f);
    std::fs::write(dir.join("there/.draft"), "a draft").unwrap();
    f.mods.set(Mods { shift: true, ..Mods::default() });
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/.draft")], Way::Move), DropEnd::Later);
    assert!(card_of(&f).iter().any(|s| s == "Move .draft here?"), "{:?}", card_of(&f));
    assert!(dir.join("there/.draft").exists() && log.borrow().is_empty());
    f.update(Msg::OutsideAnswer(true));
    assert!(dir.join("here/.draft").exists() && !dir.join("there/.draft").exists(), "{}", f.said);
    assert_eq!(*log.borrow(), [(7, true)], "told once the work is over");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn which_places_are_private_and_which_are_a_persons_own() {
    let home = Path::new("/home/john");
    let drives = drive_roots("", home);
    let why = |p: &str| placed(Path::new(p), home, &drives);
    assert_eq!(why("/home/john/Documents/report.pdf"), None);
    assert_eq!(why("/home/john/.ssh/id_made_up"), Some(Wary::Hidden));
    assert_eq!(why("/home/john/Documents/.drafts/a.txt"), Some(Wary::Hidden));
    assert_eq!(why("/home/john/.bashrc"), Some(Wary::Hidden));
    assert_eq!(why("/home/john"), Some(Wary::System), "home itself is not in home");
    assert_eq!(why("/home"), Some(Wary::System));
    assert_eq!(why("/home/jane/Documents/a.txt"), Some(Wary::System), "another person's");
    assert_eq!(why("/etc/passwd"), Some(Wary::System));
    assert_eq!(why("/tmp/made-up.txt"), Some(Wary::System));
    assert_eq!(why("/run/user/1000/keyring/control"), Some(Wary::System));
    assert_eq!(why("/run/media/john/STICK/photo.jpg"), None);
    assert_eq!(why("/media/john/STICK/photo.jpg"), None);
    assert_eq!(why("/mnt/nas/film.mkv"), None);
    assert_eq!(why("/media/john/STICK/.Trash-1000/files/old.txt"), Some(Wary::Hidden));
    assert_eq!(why("/media"), Some(Wary::System));
    let mounts = "/dev/nvme0n1p2 / ext4 rw 0 0\n/dev/nvme0n1p1 /boot/efi vfat rw 0 0\n/dev/sda1 /data ext4 rw 0 0\n\
                  /dev/sdb1 /home ext4 rw 0 0\n/dev/loop3 /snap/core/1 squashfs ro 0 0\ntmpfs /scratch tmpfs rw 0 0\n\
                  nas.example:/films /films nfs4 rw 0 0\n/dev/sdc1 /My\\040Disk ext4 rw 0 0\n/dev/sdd1 /var/lib/made-up ext4 rw 0 0\n";
    let roots = drive_roots(mounts, home);
    assert_eq!(roots, ["/media", "/run/media", "/mnt", "/data", "/films", "/My Disk"].map(PathBuf::from));
    assert_eq!(placed(Path::new("/data/work/a.txt"), home, &roots), None);
    assert_eq!(placed(Path::new("/scratch/a.txt"), home, &roots), Some(Wary::System));
    let dir = folder("deep");
    for n in 0..40 {
        std::fs::write(dir.join(format!("there/{n}.txt")), "x").unwrap();
    }
    assert_eq!(looked_through(&dir.join("there"), &mut 1000, Instant::now() + LOOK_TIME), None);
    assert_eq!(looked_through(&dir.join("there"), &mut 10, Instant::now() + LOOK_TIME), Some(Wary::Deep));
    assert_eq!(looked_through(&dir.join("there"), &mut 1000, Instant::now()), Some(Wary::Deep));
    assert_eq!(looked_through(&dir.join("there/report.txt"), &mut 0, Instant::now()), None, "a file holds nothing");
    let (named, _) = work::open_same(&dir.join("there"), None).unwrap();
    assert_eq!(through_folder(work::through(&named), &mut 1000, Instant::now() + LOOK_TIME, &|_| false, &mut HashSet::new()).0, None);
    std::fs::write(dir.join("there/.kept"), "x").unwrap();
    assert_eq!(through_folder(work::through(&named), &mut 1000, Instant::now() + LOOK_TIME, &|_| false, &mut HashSet::new()).0, Some(Wary::Holds));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_folder_is_not_put_inside_itself_by_another_name() {
    let dir = folder("loop");
    std::os::unix::fs::symlink(dir.join("there"), dir.join("here/way")).unwrap();
    let mut f = files_in(&dir);
    f.go(0, Where::Folder(dir.join("here/way")));
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there")], Way::Copy), DropEnd::Failed);
    assert_eq!((f.said.as_str(), f.bad), ("A folder cannot be put inside itself.", true));
    assert!(!dir.join("there/there").exists() && f.jobs.is_empty());
    assert_eq!(drop_on_pane(&mut f, vec![dir.clone()], Way::Copy), DropEnd::Failed);
    assert_eq!(f.said, "A folder cannot be put inside itself.");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn they_land_in_a_folder_under_the_pointer_a_pinned_folder_or_a_tab() {
    let dir = folder("land");
    let mut f = files_in(&dir);
    f.update(Msg::Outside(Onto::Icons(0, 0, 40.0, 20.0), Given::Files(vec![dir.join("there/report.txt")]), Way::Copy));
    assert!(dir.join("here/box/report.txt").exists(), "{}", f.said);
    f.pinned = vec![dir.join("there")];
    std::fs::write(dir.join("here/new.txt"), "new").unwrap();
    f.update(Msg::Outside(Onto::Land(Land::Pinned(0)), Given::Files(vec![dir.join("here/new.txt")]), Way::Copy));
    assert!(dir.join("there/new.txt").exists(), "{}", f.said);
    f.update(Msg::Outside(Onto::Land(Land::Tab(0, 0)), Given::Files(vec![dir.join("there/notes.txt")]), Way::Copy));
    assert!(dir.join("here/notes.txt").exists(), "{}", f.said);
    let trash = f.place_list().iter().position(|w| *w == Where::Trash).unwrap();
    f.update(Msg::Outside(Onto::Land(Land::Place(trash)), Given::Files(vec![dir.join("there/notes.txt")]), Way::Copy));
    assert!(dir.join("there/notes.txt").exists());
    assert_eq!(f.said, "Things can only be dropped on a folder.");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn they_land_in_the_folder_a_name_of_the_path_stands_for_unless_it_cannot_be_written_to() {
    use std::os::unix::fs::PermissionsExt;
    let dir = folder("names");
    let mut f = files_in(&dir);
    f.go(0, Where::Folder(dir.join("here/box")));
    f.update(Msg::Outside(Onto::Land(Land::Part(0, 0)), Given::Files(vec![dir.join("there/report.txt")]), Way::Copy));
    assert!(dir.join("report.txt").exists(), "{}", f.said);
    f.update(Msg::Outside(Onto::Land(Land::Part(0, 2)), Given::Files(vec![dir.join("there/notes.txt")]), Way::Copy));
    assert!(dir.join("here/box/notes.txt").exists(), "{}", f.said);
    std::fs::set_permissions(dir.join("here"), std::fs::Permissions::from_mode(0o555)).unwrap();
    let before = f.jobs.len();
    f.update(Msg::Outside(Onto::Land(Land::Part(0, 1)), Given::Files(vec![dir.join("there/notes.txt")]), Way::Copy));
    assert!(!dir.join("here/notes.txt").exists() && f.jobs.len() == before);
    std::fs::set_permissions(dir.join("here"), std::fs::Permissions::from_mode(0o755)).unwrap();
    f.go(0, Where::Computer);
    f.update(Msg::Outside(Onto::Land(Land::Part(0, 0)), Given::Files(vec![dir.join("there/notes.txt")]), Way::Copy));
    assert!(!dir.join("notes.txt").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn folders_let_go_between_two_pinned_rows_are_pinned_there() {
    let dir = folder("pin");
    let mut f = files_in(&dir);
    f.pins_file = Some(dir.join("pins"));
    f.pinned = vec![dir.join("here")];
    f.update(Msg::Outside(Onto::PinGap(1), Given::Files(vec![dir.join("there")]), Way::Copy));
    assert_eq!(f.pinned, [dir.join("here"), dir.join("there")], "{}", f.said);
    assert_eq!(f.drop_end(3, Way::Copy), DropEnd::Done);
    assert!(f.jobs.is_empty() && !dir.join("here/there").exists());
    f.update(Msg::Outside(Onto::PinGap(0), Given::Files(vec![dir.join("there/report.txt")]), Way::Copy));
    assert_eq!(f.said, "Only folders can be pinned to the sidebar.");
    assert_eq!(f.drop_end(4, Way::Copy), DropEnd::Failed);
    assert_eq!(f.pinned.len(), 2);
    let held = |ways| Held { offered: Offered { files: true, text: false, ways }, part: Part::Gap(0), mods: Mods::default() };
    assert_eq!(would_pin(&held(Ways::ALL)), Some(Way::Copy));
    assert_eq!(would_pin(&held(Ways::ASK)), Some(Way::Ask));
    assert_eq!(would_pin(&held(Ways::MOVE)), None);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn held_by_the_right_button_elsewhere_it_asks_here() {
    let dir = folder("ask");
    let mut f = files_in(&dir);
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/report.txt")], Way::Ask), DropEnd::Done);
    let menu = f.menu.as_ref().expect("the menu that asks");
    assert_eq!(menu.drop, Some(Where::Folder(dir.join("here"))));
    assert!(!dir.join("here/report.txt").exists(), "nothing is done until a row is picked");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_pointer_says_no_to_what_is_not_files_and_follows_what_is_asked() {
    let held = |files, ways, shift| Held { offered: Offered { files, text: true, ways }, part: Part::Whole, mods: Mods { shift, ..Mods::default() } };
    assert_eq!(would(&held(true, Ways::ALL, false)), Some(Way::Copy));
    assert_eq!(would(&held(true, Ways::ALL, true)), Some(Way::Move));
    assert_eq!(would(&held(true, Ways::MOVE, false)), Some(Way::Move));
    assert_eq!(would(&held(true, Ways::ASK | Ways::COPY, false)), Some(Way::Ask));
    assert_eq!(would(&held(false, Ways::ALL, false)), None, "text alone is turned away");
}

#[test]
fn dragged_out_nothing_goes_unless_a_move_was_taken_and_finished() {
    let dir = folder("out");
    let mut f = files_in(&dir);
    f.go(0, Where::Folder(dir.join("there")));
    let held = |f: &mut Files| {
        f.drag = Some(held_out(vec![dir.join("there/report.txt"), dir.join("there/notes.txt")]));
    };
    held(&mut f);
    let lifted = f.dragging().expect("offered");
    assert_eq!((lifted.files.len(), lifted.label.as_str(), lifted.right), (2, "2 items", false));
    f.update(Msg::OutEnded(None));
    assert!(f.drag.is_none() && f.dragging().is_none());
    assert!(dir.join("there/report.txt").exists() && dir.join("there/notes.txt").exists());
    held(&mut f);
    f.update(Msg::OutEnded(Some(dnd::Taken::made_up(Way::Copy))));
    assert!(dir.join("there/report.txt").exists() && dir.join("there/notes.txt").exists());
    held(&mut f);
    std::fs::remove_file(dir.join("there/notes.txt")).unwrap();
    f.update(Msg::OutEnded(Some(dnd::Taken::made_up(Way::Move))));
    assert!(!dir.join("there/report.txt").exists(), "{}", f.said);
    f.update(Msg::Undo);
    assert_eq!(std::fs::read_to_string(dir.join("there/report.txt")).unwrap(), "the report");
    f.drag = Some(Drag { moving: false, ..held_out(vec![dir.join("there/report.txt")]) });
    assert!(f.dragging().is_none());
    f.drag = Some(Drag { right: true, ..held_out(vec![PathBuf::from("sftp://example.com/home/john/a.txt")]) });
    assert!(f.dragging().is_none());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn what_is_put_in_its_place_while_the_card_is_up_is_not_brought() {
    let dir = folder("swap");
    let mut f = files_in(&dir);
    std::fs::create_dir_all(dir.join(".ssh")).unwrap();
    std::fs::write(dir.join(".ssh/id_made_up"), "a made-up key").unwrap();
    std::fs::write(dir.join("there/.decoy"), "x").unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![dir.join("there/report.txt"), dir.join("there/.decoy")], Way::Copy), DropEnd::Later);
    assert!(card_of(&f).iter().any(|s| s == "report.txt, from ~/there"), "listed as ordinary: {:?}", card_of(&f));
    std::fs::remove_file(dir.join("there/report.txt")).unwrap();
    std::os::unix::fs::symlink(dir.join(".ssh/id_made_up"), dir.join("there/report.txt")).unwrap();
    f.update(Msg::OutsideAnswer(true));
    assert!(std::fs::symlink_metadata(dir.join("here/report.txt")).is_err(), "what took its place was brought");
    assert!(std::fs::symlink_metadata(dir.join("here/.decoy")).is_err(), "one changed and nothing is brought");
    assert!(f.bad && f.said.contains("report.txt") && f.said.contains("changed since it was dropped"), "{}", f.said);
    let mut g = files_in(&dir);
    std::fs::remove_file(dir.join("there/report.txt")).unwrap();
    std::fs::write(dir.join("there/report.txt"), "the report").unwrap();
    assert_eq!(drop_on_pane(&mut g, vec![dir.join("there/report.txt"), dir.join("there/.decoy")], Way::Copy), DropEnd::Later);
    std::fs::rename(dir.join("there/report.txt"), dir.join("there/was.txt")).unwrap();
    std::fs::hard_link(dir.join(".ssh/id_made_up"), dir.join("there/report.txt")).unwrap();
    g.update(Msg::OutsideAnswer(true));
    assert!(std::fs::symlink_metadata(dir.join("here/report.txt")).is_err() && g.bad, "{}", g.said);
    let mut h = files_in(&dir);
    let held = |h: &mut Files| {
        h.drag = Some(held_out(vec![dir.join("there/notes.txt")]));
    };
    held(&mut h);
    h.update(Msg::OutShort(Way::Move));
    assert!(h.drag.is_none() && dir.join("there/notes.txt").exists());
    assert_eq!((h.said.as_str(), h.bad), ("The other program did not take all of what was dragged, so nothing here was moved.", true));
    held(&mut h);
    h.update(Msg::OutShort(Way::Copy));
    assert_eq!((h.said.as_str(), h.bad), ("The other program did not take all of what was dragged, so it may not have all of it.", true));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn what_took_a_dragged_things_name_before_the_move_finished_is_not_put_in_the_trash() {
    let dir = folder("resaved");
    let mut f = files_in(&dir);
    f.go(0, Where::Folder(dir.join("there")));
    let (report, notes) = (dir.join("there/report.txt"), dir.join("there/notes.txt"));
    f.drag = Some(held_out(vec![report.clone(), notes.clone()]));
    std::fs::write(dir.join("there/report.txt.new"), "the report, with more since").unwrap();
    std::fs::rename(dir.join("there/report.txt.new"), &report).unwrap();
    f.update(Msg::OutEnded(Some(dnd::Taken::made_up(Way::Move))));
    assert_eq!(std::fs::read_to_string(&report).ok().as_deref(), Some("the report, with more since"), "a file the other program never had went to the Trash");
    assert!(!notes.exists(), "the one that is as it was is moved: {}", f.said);
    assert!(f.bad && f.said.contains("report.txt (it changed after it was picked up, so it is still where it was)"), "{}", f.said);
    std::fs::write(&notes, "the notes").unwrap();
    f.drag = Some(held_out(vec![notes.clone()]));
    std::fs::write(&notes, "the notes, and more").unwrap();
    f.update(Msg::OutEnded(Some(dnd::Taken::made_up(Way::Move))));
    assert_eq!(std::fs::read_to_string(&notes).ok().as_deref(), Some("the notes, and more"));
    assert_eq!((f.said.as_str(), f.bad), ("notes.txt changed after it was picked up, so it is still where it was.", true));
    f.drag = Some(Drag { was: Vec::new(), ..held_out(vec![notes.clone()]) });
    f.update(Msg::OutEnded(Some(dnd::Taken::made_up(Way::Move))));
    assert!(notes.exists() && f.bad, "{}", f.said);
    f.drag = Some(held_out(vec![notes.clone()]));
    f.update(Msg::OutUnshown(Way::Move));
    assert!(f.drag.is_none() && notes.exists());
    assert_eq!((f.said.as_str(), f.bad), ("The other program took what was dragged in a way that cannot show it read all of it, so nothing here was moved: what it has is a copy.", true));
    f.say("", false);
    f.drag = Some(held_out(vec![notes.clone()]));
    f.update(Msg::OutUnshown(Way::Copy));
    assert!(f.drag.is_none() && notes.exists() && f.said.is_empty() && !f.bad, "{}", f.said);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn what_takes_a_handed_over_things_name_while_the_trash_waits_is_not_put_in_the_trash() {
    use crate::files::work::faults;
    let dir = folder("waited");
    let mut f = files_in(&dir);
    f.go(0, Where::Folder(dir.join("there")));
    let (report, notes) = (dir.join("there/report.txt"), dir.join("there/notes.txt"));
    let binned = || std::fs::read_dir(dir.join("Trash/files")).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect::<Vec<_>>();
    f.drag = Some(held_out(vec![report.clone(), notes.clone()]));
    faults::set(faults::Fault::Swapped, &report);
    f.update(Msg::OutEnded(Some(dnd::Taken::made_up(Way::Move))));
    assert_eq!(std::fs::read(&report).ok().as_deref(), Some(faults::SWAPPED), "a file the other program never had went to the Trash");
    assert_eq!(binned(), ["notes.txt"], "only the one that is as it was is in the Trash");
    assert!(!std::fs::read_dir(dir.join("Trash/info")).unwrap().flatten().any(|e| e.file_name().to_string_lossy().starts_with("report")), "no note is left for what came back");
    assert!(f.bad && f.said.contains("report.txt (it changed after it was picked up, so it is still where it was)"), "{}", f.said);
    f.update(Msg::Undo);
    assert_eq!(std::fs::read_to_string(&notes).ok().as_deref(), Some("the notes"), "{}", f.said);
    assert_eq!(std::fs::read(&report).ok().as_deref(), Some(faults::SWAPPED));
    let album = dir.join("there/album");
    for put_in in [false, true] {
        let _ = std::fs::remove_dir_all(&album);
        std::fs::create_dir(&album).unwrap();
        std::fs::write(album.join("sea.jpg"), "the sea").unwrap();
        let taken = dnd::Taken::made_up(Way::Move);
        let was = act::picked_up(std::slice::from_ref(&album), false).into_iter().next().flatten();
        let left = vec![Handed::of(album.clone(), was, &taken)];
        assert!(matches!(left[0], Left::Same(_)));
        if put_in {
            std::fs::write(album.join("new.jpg"), "new").unwrap();
        } else {
            std::fs::rename(&album, dir.join("there/album-kept")).unwrap();
            std::fs::create_dir(&album).unwrap();
            std::fs::write(album.join("sea.jpg"), "the sea").unwrap();
        }
        f.update(Msg::OutLeft(left, taken));
        assert!(album.join("sea.jpg").exists() && binned().is_empty(), "{put_in}: a folder that was not the one handed over went to the Trash: {}", f.said);
        assert!(f.bad && f.said.contains("album (it changed after it was picked up, so it is still where it was)"), "{put_in}: {}", f.said);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_folder_dragged_out_goes_only_when_everything_in_it_is_as_it_was_picked_up() {
    use std::os::unix::fs::PermissionsExt;
    let dir = folder("album");
    let mut f = files_in(&dir);
    f.go(0, Where::Folder(dir.join("there")));
    let album = dir.join("there/album");
    let fresh = || {
        let _ = std::fs::remove_dir_all(&album);
        std::fs::create_dir_all(album.join("2026/spring")).unwrap();
        std::fs::write(album.join("2026/spring/sea.jpg"), "the sea").unwrap();
        std::fs::write(album.join("cover.jpg"), "the cover").unwrap();
    };
    let moved = || Msg::OutEnded(Some(dnd::Taken::made_up(Way::Move)));
    let changes: [(&str, &dyn Fn()); 4] = [
        ("a file put in it since", &|| std::fs::write(album.join("2026/new.jpg"), "new").unwrap()),
        ("a file in it saved again", &|| std::fs::write(album.join("2026/spring/sea.jpg"), "the sea, and more").unwrap()),
        ("a file in it gone", &|| std::fs::remove_file(album.join("cover.jpg")).unwrap()),
        ("a folder put in it since", &|| std::fs::create_dir(album.join("2026/summer")).unwrap()),
    ];
    for (what, change) in changes {
        fresh();
        f.drag = Some(held_out(vec![album.clone()]));
        change();
        f.update(moved());
        assert!(album.is_dir(), "{what}: the folder went to the Trash");
        assert_eq!((f.said.as_str(), f.bad), ("album changed after it was picked up, so it is still where it was.", true), "{what}");
    }
    fresh();
    std::fs::set_permissions(album.join("cover.jpg"), std::fs::Permissions::from_mode(0o200)).unwrap();
    assert!(std::fs::read(album.join("cover.jpg")).is_err(), "{}", crate::common::files::tests::NOT_AS_ROOT);
    f.drag = Some(held_out(vec![album.clone()]));
    f.update(moved());
    assert!(album.is_dir(), "a folder with a file in it that could not be read went to the Trash");
    assert_eq!((f.said.as_str(), f.bad), ("Not moved: 1 unreadable file.", true));
    assert_eq!(f.reasons.as_ref().map(|(w, l)| (w.as_str(), l.as_slice())), Some(("Not moved: 1 unreadable file.", &["album/cover.jpg: no permission".to_string()][..])));
    f.update(Msg::Listed);
    assert!(f.reasons_open);
    f.say("Pinned Albums to the sidebar.", false);
    assert!(!f.reasons_open && super::super::view::status_words(&f, &f.cols[0], 0, true).contains("sidebar"));
    f.update(Msg::Listed);
    assert!(!f.reasons_open, "a list that no longer goes with the words opened");
    fresh();
    f.drag = Some(held_out(vec![album.clone()]));
    f.update(moved());
    assert!(!album.exists(), "{}", f.said);
    f.update(Msg::Undo);
    assert_eq!(std::fs::read_to_string(album.join("2026/spring/sea.jpg")).ok().as_deref(), Some("the sea"), "{}", f.said);
    let _ = std::fs::remove_dir_all(&dir);
}

fn held_out(items: Vec<PathBuf>) -> Drag {
    Drag { start: (0.0, 0.0), pane: 0, was: act::picked_up(&items, false), items, moving: true, then: None, right: false, pins: false }
}

#[test]
fn a_hidden_thing_put_in_a_folder_after_it_was_looked_at_is_not_brought() {
    let dir = folder("since");
    let mut f = files_in(&dir);
    let proj = dir.join("there/proj");
    std::fs::create_dir_all(proj.join(".git/hooks")).unwrap();
    std::fs::create_dir_all(dir.join(".ssh")).unwrap();
    std::fs::write(dir.join(".ssh/id_made_up"), "a made-up key").unwrap();
    std::fs::write(proj.join(".git/config"), "made up").unwrap();
    std::fs::write(proj.join("readme.txt"), "read me").unwrap();
    assert_eq!(drop_on_pane(&mut f, vec![proj.clone()], Way::Copy), DropEnd::Later);
    assert!(card_of(&f).iter().any(|s| s == "proj, from ~/there: has hidden things in it"), "{:?}", card_of(&f));
    std::fs::write(proj.join(".env"), "a made-up secret").unwrap();
    std::fs::write(proj.join(".git/hooks/.also"), "another").unwrap();
    std::os::unix::fs::symlink(dir.join(".ssh"), proj.join("way")).unwrap();
    std::fs::write(proj.join("notes.txt"), "ordinary").unwrap();
    f.update(Msg::OutsideAnswer(true));
    let brought = dir.join("here/proj");
    assert_eq!(std::fs::read_to_string(brought.join(".git/config")).ok().as_deref(), Some("made up"), "what the person was asked about is brought: {}", f.said);
    assert!(brought.join("readme.txt").exists() && brought.join("notes.txt").exists(), "{}", f.said);
    for late in [".env", ".git/hooks/.also", "way"] {
        assert!(std::fs::symlink_metadata(brought.join(late)).is_err(), "{late} was put there after the person was asked, and was brought");
    }
    assert!(f.bad && f.said.contains(work::not_looked()), "{}", f.said);
    let plain = dir.join("there/plain");
    std::fs::create_dir_all(plain.join("deeper")).unwrap();
    std::fs::write(plain.join("deeper/a.txt"), "a").unwrap();
    let looked = look(std::slice::from_ref(&plain), Some(&dir.join("here")), &dir, false);
    assert!(looked.wary.is_empty());
    std::fs::write(plain.join("deeper/.late"), "late").unwrap();
    let ctl = work::Ctl { sure: looked.which.clone(), ..work::Ctl::default() };
    let over = work::run(work::Plan { kind: work::Kind::Copy, items: vec![plain], to: dir.join("here"), key: None }, &ctl, &|_| {});
    assert_eq!(over.failed, [format!(".late ({})", work::not_looked())]);
    assert!(dir.join("here/plain/deeper/a.txt").exists() && !dir.join("here/plain/deeper/.late").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_dropped_folder_too_big_to_look_through_brings_no_hidden_thing() {
    let dir = folder("toobig");
    let big = dir.join("there/big");
    std::fs::create_dir_all(big.join("deeper")).unwrap();
    for n in 0..LOOK_MOST + 1 {
        std::fs::write(big.join(format!("{n}.txt")), "").unwrap();
    }
    std::fs::write(big.join("deeper/.cache"), "hidden").unwrap();
    std::os::unix::fs::symlink(dir.join("there/notes.txt"), big.join("way")).unwrap();
    let looked = look(std::slice::from_ref(&big), Some(&dir.join("here")), &dir, false);
    assert_eq!(looked.wary.iter().map(|(_, w)| *w).collect::<Vec<_>>(), [Wary::Deep]);
    let ctl = work::Ctl { sure: looked.which.clone(), ..work::Ctl::default() };
    let over = work::run(work::Plan { kind: work::Kind::Copy, items: vec![big], to: dir.join("here"), key: None }, &ctl, &|_| {});
    let brought = dir.join("here/big");
    assert!(brought.join("0.txt").exists(), "an ordinary file is brought as ever");
    assert!(!brought.join("deeper/.cache").exists() && std::fs::symlink_metadata(brought.join("way")).is_err(), "something not looked at was brought");
    let mut failed = over.failed.clone();
    failed.sort();
    assert_eq!(failed, [format!(".cache ({})", work::too_big_to_check()), format!("way ({})", work::too_big_to_check())]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_place_that_does_not_answer_is_given_up_on() {
    let dir = folder("late");
    let mut f = files_in(&dir);
    let log = told(&mut f);
    let files = vec![dir.join("there/report.txt")];
    let waiting = |f: &mut Files, n: u64| {
        f.outside.looking = Some(Looking { n, files: files.clone(), target: Where::Folder(dir.join("here")), how: act::Way::Copy, moving: false, pin: None });
        f.outside.last = Some(Last::Looking);
    };
    waiting(&mut f, 9);
    assert_eq!(f.drop_end(7, Way::Copy), DropEnd::Later);
    f.update(Msg::Outside(Onto::Icons(0, 0, 500.0, 300.0), Given::Files(vec![dir.join("there/notes.txt")]), Way::Copy));
    assert_eq!(f.said, "The last thing dropped is still being looked at.");
    f.update(Msg::Changed);
    assert!(log.borrow().is_empty(), "not told anything while it is looked at");
    f.update(Msg::OutsideLate(8));
    assert!(f.outside.looking.is_some() && log.borrow().is_empty());
    f.update(Msg::OutsideLate(9));
    assert_eq!((f.said.as_str(), f.bad), ("The place report.txt is in is not answering, so nothing was brought.", true));
    assert_eq!(*log.borrow(), [(7, false)]);
    f.update(Msg::OutsideLooked(9, Box::new(look(&files, Some(&dir.join("here")), &dir, false))));
    assert!(!dir.join("here/report.txt").exists() && f.jobs.is_empty());
    waiting(&mut f, 10);
    assert_eq!(f.drop_end(8, Way::Copy), DropEnd::Later);
    f.update(Msg::OutsideLooked(10, Box::new(look(&files, Some(&dir.join("here")), &dir, false))));
    assert!(dir.join("here/report.txt").exists(), "{}", f.said);
    assert_eq!(*log.borrow(), [(7, false), (8, true)]);
    assert_eq!(look(&[dir.join("there/nothing.txt")], None, &dir, false).gone, Some(dir.join("there/nothing.txt")));
    assert!(look(&[dir.join("here")], Some(&dir.join("here/box")), &dir, false).inside);
    let found = look(&[dir.join("there/report.txt")], Some(&dir.join("here")), &dir, false);
    assert!(found.gone.is_none() && !found.inside && found.wary.is_empty() && found.which.len() == 1);
    assert!(look(&[dir.join("there")], None, &dir, true).folders && !look(&[dir.join("there/report.txt")], None, &dir, true).folders);
    assert!(CHECK_WAIT >= Duration::from_secs(2) && CHECK_WAIT <= Duration::from_secs(10));
    let _ = std::fs::remove_dir_all(&dir);
}
