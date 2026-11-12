//! What a server or a stranger's folder holds is never followed, written over or removed unproved.

use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use crate::files::work::{self, Answer, Ctl, Kind, Plan, Which};

use super::*;

#[test]
fn a_link_or_a_pipe_in_what_is_sent_to_a_server_is_not_followed_and_not_waited_on() {
    use std::os::unix::fs::OpenOptionsExt;
    let (here, far, base) = place("named");
    fs::create_dir_all(here.join("private")).unwrap();
    fs::write(here.join("private/key"), "a made-up key").unwrap();
    std::os::unix::fs::symlink(here.join("private/key"), here.join("box/inner/invoice.pdf")).unwrap();
    std::os::unix::fs::symlink(here.join("private"), here.join("box/inner/holiday")).unwrap();
    let pipe = here.join("box/inner/pipe");
    let name = std::ffi::CString::new(pipe.as_os_str().as_bytes()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
    let (tx, rx) = std::sync::mpsc::channel();
    let (items, to) = (vec![here.join("box")], PathBuf::from(format!("{base}/box/inner")));
    std::thread::spawn(move || {
        let _ = tx.send(go(Kind::Copy, items, to, Answer::Skip));
    });
    let got = rx.recv_timeout(std::time::Duration::from_secs(10));
    let _ = fs::OpenOptions::new().write(true).custom_flags(libc::O_NONBLOCK).open(&pipe);
    let over = got.expect("sending waited on a pipe");
    let sent = far.join("box/inner/box/inner");
    assert_eq!(there(&sent), ["one.txt"], "only the ordinary file beside them is sent");
    assert_eq!(fs::read(sent.join("one.txt")).unwrap(), b"one");
    let mut failed = over.failed.clone();
    failed.sort();
    assert_eq!(failed, ["holiday (it's a link, so Folders didn't send it)", "invoice.pdf (it's a link, so Folders didn't send it)",
                        "pipe (it isn't a file or folder, so Folders didn't send it)"]);
    let over = go(Kind::Move, vec![here.join("box/inner/invoice.pdf")], PathBuf::from(format!("{base}/box")), Answer::Skip);
    assert_eq!(over.failed, ["invoice.pdf (it's a link, so Folders didn't send it)"]);
    assert!(fs::symlink_metadata(far.join("box/invoice.pdf")).is_err() && fs::symlink_metadata(here.join("box/inner/invoice.pdf")).is_ok());
    assert_eq!(fs::read_to_string(here.join("private/key")).unwrap(), "a made-up key");
    std::os::unix::fs::symlink(here.join("box/inner/one.txt"), here.join("alias.txt")).unwrap();
    std::os::unix::fs::symlink(&pipe, here.join("drain")).unwrap();
    let over = go(Kind::Copy, vec![here.join("alias.txt"), here.join("drain")], PathBuf::from(&base), Answer::Skip);
    assert_eq!(over.failed, ["drain (it isn't a file or folder, so Folders didn't send it)"]);
    assert_eq!(fs::read(far.join("alias.txt")).ok().as_deref(), Some(&b"one"[..]));
    let dropped = here.join("alias.txt");
    let which = Which::of(&fs::symlink_metadata(&dropped).unwrap());
    let ctl = Ctl { fixed: Some(Answer::Both), sure: vec![work::Sure { path: dropped.clone(), which, held: None, known: None }], ..Ctl::default() };
    let over = work::run(Plan { kind: Kind::Copy, items: vec![dropped], to: PathBuf::from(&base), key: None }, &ctl, &|_| {});
    assert_eq!(over.failed, ["alias.txt (it's a link, so Folders didn't send it)"]);
    for (address, said) in [("ftp://jo@example.com", true), ("ftps://jo@example.com", true), ("ftpes://jo@example.com", true),
                            ("sftp://jo@example.com", false), ("mtp://phone/Internal", true), ("dav://jo@example.com", false), ("davs://jo@example.com", false)] {
        let words = view(&Connect::new(address.into(), 0), 1).said();
        assert_eq!(words.iter().any(|w| w == ftp_cannot_refuse()), said, "{address}: {words:?}");
    }
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn what_another_made_on_a_server_meanwhile_is_not_written_over_or_removed() {
    for kind in [Kind::Move, Kind::Copy] {
        let (here, far, base) = place("raced");
        fs::write(here.join("report.txt"), b"mine").unwrap();
        let theirs = far.join("report.txt");
        let made = theirs.clone();
        fake::meanwhile(&far, move || fs::write(&made, b"theirs").unwrap());
        let over = go(kind.clone(), vec![here.join("report.txt")], PathBuf::from(&base), Answer::Skip);
        assert_eq!(fs::read(&theirs).unwrap(), b"theirs", "a file another made meanwhile was written over");
        assert_eq!(fs::read(here.join("report.txt")).unwrap(), b"mine", "the first went though it never arrived");
        assert_eq!(over.failed, ["report.txt (something else was given that name meanwhile)"]);
        assert_eq!(there(&far), ["box", "report.txt", "song.txt"], "nothing half made is left beside it");
        fs::write(here.join("fails.txt"), vec![1u8; 9000]).unwrap();
        let theirs = far.join("fails.txt");
        let made = theirs.clone();
        fake::meanwhile(&far, move || fs::write(&made, b"theirs").unwrap());
        let over = go(kind.clone(), vec![here.join("fails.txt")], PathBuf::from(&base), Answer::Skip);
        assert_eq!(fs::read(&theirs).ok().as_deref(), Some(&b"theirs"[..]), "what another made was removed, or written over, as the half-made copy");
        assert_eq!(fs::read(here.join("fails.txt")).unwrap().len(), 9000);
        assert_eq!(over.failed.len(), 1, "{:?}", over.failed);
        assert_eq!(there(&far), ["box", "fails.txt", "report.txt", "song.txt"]);
        let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
    }
}

#[test]
fn what_is_put_in_a_servers_folder_while_it_is_moved_is_not_removed_with_it() {
    let (here, far, base) = place("greedy-late");
    fs::create_dir_all(here.join("in")).unwrap();
    let late = far.join("box/inner/late.txt");
    let made = late.clone();
    fake::meanwhile(&far, move || fs::write(&made, b"put there meanwhile").unwrap());
    let over = go(Kind::Move, vec![PathBuf::from(format!("{base}/box"))], here.join("in"), Answer::Skip);
    assert_eq!(fs::read(&late).ok().as_deref(), Some(&b"put there meanwhile"[..]), "a file that was never brought was removed with its folder");
    assert!(here.join("in/box/inner/one.txt").exists() && !far.join("box/inner/one.txt").exists(), "what did arrive was not moved");
    assert_eq!(over.failed, ["box (not moved: some items)"]);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_file_put_under_the_name_here_while_the_first_went_to_a_server_is_not_removed() {
    let (here, far, base) = place("resaved");
    let mine = here.join("report.txt");
    fs::write(&mine, b"first words").unwrap();
    let saved = mine.clone();
    fake::meanwhile(&far, move || {
        fs::remove_file(&saved).unwrap();
        fs::write(&saved, b"other words").unwrap();
    });
    let over = go(Kind::Move, vec![mine.clone()], PathBuf::from(&base), Answer::Skip);
    assert_eq!(fs::read(&mine).ok().as_deref(), Some(&b"other words"[..]), "a file the server never had was removed");
    if far.join("report.txt").exists() {
        assert_eq!(fs::read(far.join("report.txt")).unwrap(), b"first words");
        assert_eq!(over.failed, ["report.txt (it changed after it was copied, so it is still where it was)"]);
    } else {
        assert_eq!(there(&far), ["box", "song.txt"]);
        assert_eq!(over.failed, ["report.txt (it changed while it was being copied)"]);
    }
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_mixed_pick_from_a_server_is_brought_once_and_to_the_right_folder() {
    let whole = ["test.invalid/", "test.invalid/box/", "test.invalid/box/inner/", "test.invalid/box/inner/one.txt=3", "test.invalid/song.txt=10000"];
    let boxed = ["box/", "box/inner/", "box/inner/one.txt=3"];
    let (here, far, base) = place("x488");
    let top = PathBuf::from(format!("{base}/"));
    let b = |s: &str| PathBuf::from(format!("{base}/{s}"));
    let cases: Vec<(&str, Vec<PathBuf>, Vec<&str>)> = vec![
        ("top", vec![top.clone()], whole.to_vec()),
        ("top and a folder in it", vec![top.clone(), b("box")], whole.to_vec()),
        ("a folder and a file in it, the top last", vec![b("box"), b("song.txt"), top.clone()], whole.to_vec()),
        ("top twice", vec![top.clone(), top.clone()], whole.to_vec()),
        ("folders from inside", vec![b("box"), b("song.txt")], vec!["box/", "box/inner/", "box/inner/one.txt=3", "song.txt=10000"]),
        ("a folder, one in it, a file in that", vec![b("box"), b("box/inner"), b("box/inner/one.txt")], boxed.to_vec()),
        ("the same file twice", vec![b("song.txt"), b("song.txt")], vec!["song.txt=10000"]),
        ("a file before its folder", vec![b("box/inner/one.txt"), b("box")], boxed.to_vec()),
    ];
    for (name, items, want) in &cases {
        for kind in [Kind::Copy, Kind::Move] {
            let moving = matches!(kind, Kind::Move);
            if moving && items.contains(&top) {
                continue;
            }
            let to = here.join("to");
            let _ = fs::remove_dir_all(&to);
            fs::create_dir_all(&to).unwrap();
            let over = go(kind, items.clone(), to.clone(), Answer::Both);
            assert!(over.failed.is_empty() && !over.stopped, "{name}: {:?}", over.failed);
            assert_eq!(tree(&to), *want, "{name} (moving: {moving})");
            if moving {
                assert_eq!(far.join("box/inner/one.txt").exists(), !want.contains(&"box/inner/one.txt=3"), "{name}: moved or kept wrongly");
                assert_eq!(far.join("song.txt").exists(), !want.contains(&"song.txt=10000"), "{name}: moved or kept wrongly");
                let _ = fs::remove_dir_all(&far);
                fs::create_dir_all(far.join("box/inner")).unwrap();
                fs::write(far.join("box/inner/one.txt"), b"one").unwrap();
                fs::write(far.join("song.txt"), vec![5u8; 10_000]).unwrap();
            }
        }
    }
    fs::create_dir_all(far.join("in")).unwrap();
    let to = PathBuf::from(format!("{base}/in"));
    let over = go(Kind::Copy, vec![here.join("box/inner/one.txt"), here.join("box"), here.join("box")], to, Answer::Both);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert_eq!(tree(&far.join("in")), boxed);
    let p = |s: &str| PathBuf::from(s);
    assert_eq!(work::once_each(vec![p("/a/b/c"), p("/z"), p("/a/b"), p("/a"), p("/z")]), vec![p("/z"), p("/a")]);
    assert_eq!(work::once_each(vec![p("/a/b"), p("/y"), p("/a/c")]), vec![p("/a/b"), p("/y"), p("/a/c")]);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}
