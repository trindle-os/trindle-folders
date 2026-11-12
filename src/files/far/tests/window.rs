//! A server in a window: its names, its pane, its card and the copies opened from it.

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::files::trash;
use crate::files::preview;
use crate::files::app::{Files, Msg};
use crate::files::dialogs::Dialog;
use crate::files::places::Where;
use crate::files::remote::{self, Error};
use crate::files::work::{self, Answer, Ctl, Kind, News, Plan};

use super::*;

#[test]
fn a_name_a_server_gives_that_is_not_one_name_puts_nothing_outside_the_chosen_folder() {
    use crate::toolkit::ui::App;
    let (here, far, base) = place("names");
    let chosen = here.join("chosen");
    fs::create_dir_all(&chosen).unwrap();
    fs::create_dir_all(far.join("bag")).unwrap();
    fs::write(far.join("bag/plain.txt"), b"plain").unwrap();
    let stroke = fake::STROKE;
    let whole = format!("{}/planted.txt", here.display());
    fs::write(far.join(format!("bag/..{stroke}..{stroke}climbed.txt")), b"climbed").unwrap();
    let kept = far.join("bag").join(whole.replace('/', &stroke.to_string()));
    assert!(kept.file_name().unwrap().len() <= 255, "the temporary folder's name is too long for this test");
    fs::write(&kept, b"planted").unwrap();
    let bag = PathBuf::from(format!("{base}/bag"));
    for kind in [Kind::Copy, Kind::Move] {
        let over = go(kind, vec![bag.clone()], chosen.clone(), Answer::Both);
        assert!(!here.join("climbed.txt").exists(), "a name that climbs made a file outside the folder that was chosen");
        assert!(!here.join("planted.txt").exists(), "a name that is a whole path made a file where the server said");
        assert_eq!(over.failed.iter().filter(|f| f.contains(remote::not_a_name())).count(), 2, "each is said: {:?}", over.failed);
        let came: Vec<PathBuf> = fs::read_dir(&chosen).unwrap().flatten().map(|e| e.path()).collect();
        assert_eq!(came.len(), 1, "{came:?}");
        assert_eq!(fs::read(came[0].join("plain.txt")).unwrap(), b"plain");
        assert_eq!(fs::read_dir(&came[0]).unwrap().count(), 1);
        assert!(far.join("bag").is_dir() && kept.exists(), "a folder holding what could not be brought was removed");
        fs::remove_dir_all(&came[0]).unwrap();
        fs::write(far.join("bag/plain.txt"), b"plain").unwrap();
    }
    let over = go(Kind::Delete, vec![bag.clone()], PathBuf::new(), Answer::Both);
    assert!(over.failed.iter().any(|f| f.contains(remote::not_a_name())), "{:?}", over.failed);
    assert!(kept.exists() && !here.join("planted.txt").exists());
    fs::write(far.join("bag/plain.txt"), b"plain").unwrap();
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(format!("{base}/bag"))));
    f.update(Msg::Far(FarMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(names(&f, f.side()), ["plain.txt"]);
    assert!(f.bad && f.said.starts_with("Folders skipped 2 items because their names aren't safe to use: "), "{}", f.said);
    assert!(f.said.contains("../../climbed.txt") && f.said.contains("/planted.txt"), "{}", f.said);
    f.far_open(Path::new(&format!("{base}/bag/..")));
    assert!(f.bad && f.said == "Folders skipped a file because its name isn't safe to use.", "{}", f.said);
    assert!(End::Far(address(Path::new(&format!("{base}/"))).unwrap()).name() == "test.invalid", "a server's top goes under the server's name");
    for d in [&here, &far] {
        fs::remove_dir_all(d).unwrap();
    }
}

#[test]
fn a_move_on_one_server_takes_anothers_place_only_as_the_person_was_asked() {
    let (here, far, base) = place("replace-same");
    fs::create_dir_all(far.join("old")).unwrap();
    fs::write(far.join("old/song.txt"), b"the old one").unwrap();
    let over = go(Kind::Move, vec![PathBuf::from(format!("{base}/song.txt"))], PathBuf::from(format!("{base}/old")), Answer::Replace);
    assert!(over.failed.is_empty() && !over.stopped, "{:?}", over.failed);
    assert_eq!(fs::read(far.join("old/song.txt")).unwrap().len(), 10_000, "the one moved did not take the old one's place");
    assert!(!far.join("song.txt").exists());
    assert_eq!(fs::read_dir(far.join("old")).unwrap().count(), 1, "nothing is left under a spare name");
    for d in [&here, &far] {
        fs::remove_dir_all(d).unwrap();
    }
    let (here, far, base) = place("careless-replace");
    fs::create_dir_all(far.join("old")).unwrap();
    fs::write(here.join("song.txt"), b"brought from here").unwrap();
    let theirs: &[u8] = b"another person's, saved while the question was up";
    for from in [PathBuf::from(format!("{base}/song.txt")), here.join("song.txt")] {
        fs::write(far.join("old/song.txt"), b"the old one").unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        let ctl = Ctl { answers: Some(rx), ..Ctl::default() };
        let target = far.join("old/song.txt");
        let tell = move |n: News| {
            if let News::Clash(_) = n {
                fs::write(&target, theirs).unwrap();
                let _ = tx.send((Answer::Replace, false));
            }
        };
        let over = work::run(Plan { kind: Kind::Move, items: vec![from.clone()], to: PathBuf::from(format!("{base}/old")), key: None }, &ctl, &tell);
        assert_eq!(fs::read(far.join("old/song.txt")).unwrap(), theirs, "{from:?}: what was never asked about was written over");
        assert!(over.failed.iter().any(|f| f.contains(work::taken_meanwhile())), "{from:?}: {:?}", over.failed);
        assert!(far.join("song.txt").exists() && here.join("song.txt").exists(), "{from:?}: the first is still where it was");
        assert_eq!(fs::read_dir(far.join("old")).unwrap().count(), 1, "{from:?}: nothing is left under a spare name");
    }
    for d in [&here, &far] {
        fs::remove_dir_all(d).unwrap();
    }
}

#[test]
fn a_doubtful_certificate_is_asked_about() {
    use crate::toolkit::ui::App;
    let mut f = Files::new(vec![std::env::temp_dir()]);
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address("ftpes://jo@cert.invalid".into())));
    let fp = "AB:CD".to_string();
    f.update(Msg::Far(FarMsg::Refused(why(Error::CertUnknown { host: "cert.invalid".into(), fingerprint: fp.clone(), why: "it has expired".into() }, false))));
    let Some(Dialog::Connect(c)) = &f.dialog else { panic!("the card closed") };
    assert!(matches!(&c.question, Some(Question::NewCert { fingerprint, .. }) if *fingerprint == fp));
    f.update(Msg::Far(FarMsg::Trust));
    let Some(Dialog::Connect(c)) = &f.dialog else { panic!("the card closed") };
    assert_eq!(c.trust_cert.as_deref(), Some("AB:CD"));
    assert!(!c.trust, "an ssh host is not trusted by this");
    f.update(Msg::Far(FarMsg::Address("ftps://jo@other.invalid".into())));
    let Some(Dialog::Connect(c)) = &f.dialog else { panic!("the card closed") };
    assert!(c.trust_cert.is_none() && c.question.is_none());
    assert!(is_far(Path::new("ftps://jo@other.invalid/x")) && is_far(Path::new("ftpes://h/x")));
}

#[test]
fn a_server_opens_in_a_pane() {
    use crate::toolkit::ui::App;
    let (here, far, base) = place("pane");
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.bin = Some(trash::Bin::at(&here.join("Trash"), None));
    f.update(Msg::Far(FarMsg::Open));
    assert!(matches!(f.dialog, Some(Dialog::Connect(_))));
    f.update(Msg::Far(FarMsg::Address("sftp://nobody@nowhere.invalid".into())));
    f.update(Msg::Far(FarMsg::Go));
    let Some(Dialog::Connect(c)) = &f.dialog else { panic!("the card closed") };
    assert!(c.bad.is_some() && !c.busy);
    f.update(Msg::Far(FarMsg::Address(base.clone())));
    f.update(Msg::Far(FarMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(f.far.servers.len(), 1);
    let p = f.side();
    assert!(matches!(&f.cols[p].at, Where::Remote(a) if a.contains("test.invalid")));
    assert_eq!(names(&f, p), ["box", "song.txt"]);
    assert!(f.place_list().iter().any(|w| matches!(w, Where::Remote(_))));
    let t = f.cols[p].tab;
    f.cols[1 - p].tabs[t].at = Where::Folder(here.clone());
    let i = f.cols[p].tabs[t].entries.iter().position(|e| e.name == "song.txt").unwrap();
    f.update(Msg::Choose(p, i));
    f.update(Msg::Pick(p, vec![i]));
    f.update(Msg::Copy);
    assert_eq!(fs::read(here.join("song.txt")).unwrap().len(), 10_000, "{}", f.said);
    f.update(Msg::NewFolder);
    assert!(far.join("New folder").is_dir(), "{}", f.said);
    assert!(names(&f, p).contains(&"New folder".to_string()));
    let r = f.renaming.clone().expect("the new folder's name is open");
    assert!(r.fresh && r.was() == "New folder" && r.selected() == (0.."New folder".len()), "{r:?}");
    assert_eq!(f.cols[p].chosen.map(|i| f.cols[p].entries[i].name.clone()).as_deref(), Some("New folder"));
    f.update(Msg::RenameText("Songs".into()));
    f.update(Msg::RenameDone);
    assert!(far.join("Songs").is_dir() && !far.join("New folder").exists(), "{}", f.said);
    assert!(f.renaming.is_none());
    assert_eq!(f.cols[p].chosen.map(|i| f.cols[p].entries[i].name.clone()).as_deref(), Some("Songs"));
    f.update(Msg::Rename);
    f.update(Msg::RenameText("box".into()));
    f.update(Msg::RenameDone);
    assert!(f.bad && f.renaming.as_ref().is_some_and(|r| r.text == "box" && r.why.is_some()), "{}", f.said);
    f.update(Msg::RenameStop);
    assert!(far.join("Songs").is_dir());
    f.update(Msg::NewDocument);
    assert_eq!(fs::metadata(far.join("New document")).map(|m| m.len()).ok(), Some(0), "{}", f.said);
    assert!(names(&f, p).contains(&"New document".to_string()));
    assert!(f.renaming.as_ref().is_some_and(|r| r.fresh && r.text == "New document"));
    f.update(Msg::RenameStop);
    assert!(far.join("New document").is_file() && f.renaming.is_none());
    f.trash_these(vec![PathBuf::from(format!("{base}/song.txt"))]);
    assert!(far.join("song.txt").exists() && f.said.contains("Trash"));
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_new_document_does_not_empty_one_another_made_meanwhile() {
    use crate::toolkit::ui::App;
    let (here, far, base) = place("make-raced");
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(base)));
    f.update(Msg::Far(FarMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    let theirs = far.join("New document");
    let made = theirs.clone();
    fake::meanwhile(&far, move || fs::write(&made, b"theirs").unwrap());
    f.update(Msg::NewDocument);
    assert_eq!(fs::read(&theirs).ok().as_deref(), Some(&b"theirs"[..]), "a file another made was emptied: {}", f.said);
    assert!(f.said.contains("already there") && f.renaming.is_none(), "{}", f.said);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn an_opened_copy_sends_its_changes_back() {
    use crate::toolkit::ui::App;
    let (here, far, base) = place("back");
    fs::write(far.join("note.txt"), "first\n").unwrap();
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(base.clone())));
    f.update(Msg::Far(FarMsg::Go));
    let p = f.side();
    let file = PathBuf::from(format!("{base}/note.txt"));
    f.far_open(&file);
    let copy = f.far.fetched.iter().find(|(_, c)| c.far == file).map(|(k, _)| k.clone()).expect("no copy");
    assert_eq!(fs::read_to_string(&copy).unwrap(), "first\n");
    std::thread::sleep(std::time::Duration::from_millis(20));
    let before = fs::metadata(far.join("note.txt")).unwrap().ino();
    fs::write(&copy, "second\n").unwrap();
    f.far_saved(std::slice::from_ref(&copy));
    assert_eq!(fs::read_to_string(far.join("note.txt")).unwrap(), "second\n", "{}", f.said);
    assert!(f.said.contains("Saved note.txt"), "{}", f.said);
    assert_ne!(fs::metadata(far.join("note.txt")).unwrap().ino(), before, "the file on the server was written over in place");
    assert!(fs::read_dir(&far).unwrap().all(|e| !e.unwrap().file_name().to_string_lossy().ends_with(SPARE)), "a spare copy was left on the server");
    fs::write(far.join("note.txt"), "server\n").unwrap();
    f.far_saved(std::slice::from_ref(&copy));
    assert_eq!(fs::read_to_string(far.join("note.txt")).unwrap(), "server\n");
    let gone = far.with_extension("gone");
    fs::rename(&far, &gone).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(20));
    fs::write(&copy, "third\n").unwrap();
    f.far_saved(std::slice::from_ref(&copy));
    assert!(f.said.contains("couldn't save") && fs::read_to_string(&copy).unwrap() == "third\n", "{}", f.said);
    fs::rename(&gone, &far).unwrap();
    let i = f.cols[p].entries.iter().position(|e| e.name == "note.txt").unwrap();
    f.update(Msg::Choose(p, i));
    f.update(Msg::Preview(preview::PreviewMsg::Toggle));
    let Some(Dialog::Preview(pv)) = &f.dialog else { panic!("no viewer") };
    assert!(matches!(&pv.seen, preview::Seen::Text { blocks, .. } if blocks[0] == "server"), "{}", f.said);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far), fs::remove_dir_all(copy.parent().unwrap()));
}

#[test]
fn files_of_one_name_from_two_folders_are_two_copies_each_sent_to_its_own() {
    use crate::toolkit::ui::App;
    let (here, far, base) = place("twins");
    for d in ["a", "b"] {
        fs::create_dir_all(far.join(d)).unwrap();
        fs::write(far.join(d).join("notes.txt"), format!("{d}\n")).unwrap();
    }
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(base.clone())));
    f.update(Msg::Far(FarMsg::Go));
    let (fa, fb) = (PathBuf::from(format!("{base}/a/notes.txt")), PathBuf::from(format!("{base}/b/notes.txt")));
    let copy_of = |f: &Files, file: &Path| f.far.fetched.iter().find(|(_, c)| c.far == file).map(|(k, _)| k.clone());
    f.far_open(&fa);
    let ca = copy_of(&f, &fa).expect("no copy of a");
    f.far_open(&fb);
    let cb = copy_of(&f, &fb).expect("no copy of b");
    assert_ne!(ca, cb, "two server files share one copy here");
    assert_eq!(copy_of(&f, &fa).as_ref(), Some(&ca), "the first file lost its copy");
    assert_eq!(fs::read_to_string(&ca).unwrap(), "a\n", "the first copy was written over by the second");
    std::thread::sleep(std::time::Duration::from_millis(20));
    fs::write(&ca, "a changed\n").unwrap();
    f.far_open(&fa);
    assert_eq!(fs::read_to_string(&ca).unwrap(), "a changed\n", "opening again threw away a change not yet sent");
    f.far_saved(std::slice::from_ref(&ca));
    assert_eq!(fs::read_to_string(far.join("a/notes.txt")).unwrap(), "a changed\n", "{}", f.said);
    assert_eq!(fs::read_to_string(far.join("b/notes.txt")).unwrap(), "b\n", "a change was sent to the other file");
    std::thread::sleep(std::time::Duration::from_millis(20));
    fs::write(&ca, "a seen\n").unwrap();
    f.far_fetch(&fa, Then::View(7));
    assert_eq!(fs::read_to_string(&ca).unwrap(), "a seen\n", "the viewer wrote over the copy open in an app");
    f.far_saved(std::slice::from_ref(&ca));
    assert_eq!(fs::read_to_string(far.join("a/notes.txt")).unwrap(), "a seen\n", "a save after viewing never reached the server: {}", f.said);
    f.closing();
    assert!(!ca.exists() && !cb.exists(), "copies of the server's files were left here");
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_rename_on_a_careless_server_never_goes_over_a_hidden_file() {
    use crate::toolkit::ui::App;
    let (here, far, base) = place("careless-hidden");
    fs::write(far.join(".secret"), b"keep me").unwrap();
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(base.clone())));
    f.update(Msg::Far(FarMsg::Go));
    let p = f.side();
    assert!(!names(&f, p).contains(&".secret".to_string()), "hidden files show");
    let i = f.cols[p].entries.iter().position(|e| e.name == "song.txt").unwrap();
    f.update(Msg::Choose(p, i));
    f.update(Msg::Pick(p, vec![i]));
    f.update(Msg::Rename);
    f.update(Msg::RenameText(".secret".into()));
    f.update(Msg::RenameDone);
    assert_eq!(fs::read(far.join(".secret")).unwrap(), b"keep me", "the hidden file was written over: {}", f.said);
    assert!(far.join("song.txt").exists());
    assert!(f.bad && f.renaming.as_ref().is_some_and(|r| r.why.is_some()), "{}", f.said);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_link_on_a_server_to_a_big_file_opens() {
    use crate::toolkit::ui::App;
    let (here, far, base) = place("big-link");
    fs::write(far.join("big.bin"), vec![3u8; 9 << 20]).unwrap();
    std::os::unix::fs::symlink("big.bin", far.join("link.bin")).unwrap();
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(base.clone())));
    f.update(Msg::Far(FarMsg::Go));
    let link = PathBuf::from(format!("{base}/link.bin"));
    f.far_fetch(&link, Then::View(3));
    let copy = anywhere(&fetch_dir("test.invalid"), "link.bin").expect("no copy was made");
    assert_eq!(fs::metadata(&copy).unwrap().len(), 9 << 20, "{}", f.said);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far), fs::remove_dir_all(copy.parent().unwrap()));
}

fn anywhere(dir: &Path, name: &str) -> Option<PathBuf> {
    fs::read_dir(dir).ok()?.flatten().find_map(|e| if e.file_name() == name { Some(e.path()) } else if e.path().is_dir() { anywhere(&e.path(), name) } else { None })
}

#[test]
fn a_reason_inside_a_sentence_has_no_stop_of_its_own() {
    let e = Error::Failed("The server said no.".into());
    let said = crate::toolkit::t!("folders-far-failed-why", name = "a.txt", why = &super::super::hands::reason(&e));
    assert!(!said.contains(".)") && !said.contains(".."), "{said}");
}
