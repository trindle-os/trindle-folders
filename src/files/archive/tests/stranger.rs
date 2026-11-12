//! Archives made to do harm get nowhere: nothing leaves the folder asked for.

use super::*;

#[test]
fn nothing_gets_out() {
    let dir = place("evil");
    let entries: [(&str, u32, &[u8]); 9] = [
        ("../evil", S_IFREG | 0o644, b"x"),
        ("/abs", S_IFREG | 0o644, b"x"),
        ("ok/../../x", S_IFREG | 0o644, b"x"),
        ("ok/up", S_IFLNK | 0o777, b"../../etc"),
        ("ok/root", S_IFLNK | 0o777, b"/etc/passwd"),
        ("ok/in", S_IFLNK | 0o777, b"../ok/fine.txt"),
        ("ok/a", S_IFLNK | 0o777, b"."),
        ("ok/a/x", S_IFLNK | 0o777, b".."),
        ("ok/fine.txt", S_IFREG | 0o644, b"fine"),
    ];
    let stop = AtomicBool::new(false);
    for (n, make) in [evil_zip as fn(&Path, &[(&str, u32, &[u8])]), evil_tar, evil_cpio].into_iter().enumerate() {
        let arc = dir.join(format!("evil{n}"));
        make(&arc, &entries);
        let out = dir.join(format!("out{n}"));
        fs::create_dir(&out).unwrap();
        let got = extract(&arc, None, &out, Place::Folder, &stop, &quiet).unwrap();
        let mut refused = got.refused.clone();
        refused.sort();
        assert_eq!(refused, ["../evil", "/abs", "ok/../../x", "ok/a/x", "ok/root", "ok/up"], "{n}");
        let top = out.join(format!("evil{n}"));
        assert_eq!(fs::read(top.join("ok/fine.txt")).unwrap(), b"fine");
        assert_eq!(fs::read(top.join("ok/in")).unwrap(), b"fine");
        assert_eq!(fs::read_link(top.join("ok/a")).unwrap(), Path::new("."));
        assert!(!dir.join("evil").exists() && !dir.join("x").exists() && !out.join("x").exists());
        assert!(leftovers(&out).is_empty());
        let out = dir.join(format!("here{n}"));
        fs::create_dir(&out).unwrap();
        let got = extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap();
        assert_eq!(got.made, vec![out.join("ok")]);
        assert!(got.refused.contains(&"ok/in".to_string()));
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn stopping_and_damage_leave_nothing() {
    let dir = place("stop");
    let src = sample(&dir);
    let stop = AtomicBool::new(false);
    let stopper = |_: u64, total: u64| {
        if total > 0 {
            stop.store(true, Ordering::Relaxed);
        }
    };
    for format in Format::ALL {
        stop.store(false, Ordering::Relaxed);
        let to = dir.join(format!("a{}", format.ending()));
        let e = compress(std::slice::from_ref(&src), &to, format, &stop, &stopper).unwrap_err();
        assert!(is_stopped(&e), "{format:?} {e}");
        assert!(!to.exists() && leftovers(&dir).is_empty());
        stop.store(false, Ordering::Relaxed);
        compress(std::slice::from_ref(&src), &to, format, &stop, &quiet).unwrap();
        let out = dir.join(format!("out-{format:?}"));
        fs::create_dir(&out).unwrap();
        let e = extract(&to, None, &out, Place::Folder, &stop, &stopper).unwrap_err();
        assert!(is_stopped(&e), "{format:?} {e}");
        assert_eq!(fs::read_dir(&out).unwrap().count(), 0);
    }
    let arc = dir.join("bad.zip");
    let data = noise(5000);
    evil_zip(&arc, &[("noise.bin", S_IFREG | 0o644, &data)]);
    let mut bytes = fs::read(&arc).unwrap();
    let at = bytes.windows(16).position(|w| w == &data[100..116]).unwrap();
    bytes[at] ^= 1;
    fs::write(&arc, bytes).unwrap();
    let out = dir.join("out-bad");
    fs::create_dir(&out).unwrap();
    stop.store(false, Ordering::Relaxed);
    assert!(extract(&arc, None, &out, Place::Here, &stop, &quiet).is_err());
    assert_eq!(fs::read_dir(&out).unwrap().count(), 0);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn devices_owners_and_borrowed_rights_are_never_brought_back() {
    let dir = place("rights");
    let things: [Thing; 6] = [
        (b"box", S_IFDIR | 0o2777, b""),
        (b"box/root-shell", S_IFREG | 0o6755, b"#!/bin/sh\n"),
        (b"box/sticky", S_IFREG | 0o1644, b"x"),
        (b"box/null", S_IFCHR | 0o666, b""),
        (b"box/pipe", S_IFIFO | 0o644, b""),
        (b"box/plain.txt", S_IFREG | 0o640, b"plain"),
    ];
    let stop = AtomicBool::new(false);
    let me = fs::metadata(&dir).unwrap().uid();
    let zip = dir.join("z");
    evil_zip(&zip, &things.iter().filter(|t| t.1 & S_IFMT == S_IFREG || t.1 & S_IFMT == S_IFDIR).map(|t| (std::str::from_utf8(t.0).unwrap(), t.1, t.2)).collect::<Vec<_>>());
    let rar_things: Vec<Thing> = things.iter().filter(|t| t.1 & S_IFMT == S_IFREG || t.1 & S_IFMT == S_IFDIR).copied().collect();
    for (name, bytes) in [("t", tar_of(&things)), ("c", cpio(&things)), ("r", rar5(&rar_things)), ("z", fs::read(&zip).unwrap())] {
        let arc = dir.join(format!("{name}-arc"));
        fs::write(&arc, bytes).unwrap();
        let out = dir.join(format!("out-{name}"));
        fs::create_dir(&out).unwrap();
        let got = extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap();
        assert_eq!(got.made, vec![out.join("box")], "{name}");
        let meta = |p: &str| fs::symlink_metadata(out.join("box").join(p)).unwrap();
        assert_eq!(meta("root-shell").mode() & 0o7777, 0o755, "{name}: the set-user and set-group bits are gone");
        assert_eq!(meta("sticky").mode() & 0o7777, 0o644, "{name}");
        assert_eq!(meta("plain.txt").mode() & 0o7777, 0o640, "{name}");
        assert_eq!(meta("").mode() & 0o7777, 0o777 & !crate::files::archive::unpack::umask(), "{name}: nor are a folder's, and the umask holds");
        assert!(["", "root-shell", "sticky", "plain.txt"].iter().all(|p| meta(p).uid() == me), "{name}: whoever unpacks owns it all");
        assert!(!out.join("box/null").exists() && !out.join("box/pipe").exists(), "{name}: no device and no pipe");
        assert_eq!(fs::read_dir(out.join("box")).unwrap().count(), 3, "{name}");
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn names_no_tool_would_make_still_come_out() {
    let dir = place("odd");
    let things: [Thing; 7] = [
        (b"odd/-rf", S_IFREG | 0o644, b"1"),
        (b"odd/new\nline", S_IFREG | 0o644, b"2"),
        (b"odd/  spaces  ", S_IFREG | 0o644, b"3"),
        (b"odd/*?[x]$(id)`id`;&|", S_IFREG | 0o644, b"4"),
        ("odd/\u{202e}txt.exe \u{1f600}".as_bytes(), S_IFREG | 0o644, b"5"),
        (b"odd/caf\xe9.txt", S_IFREG | 0o644, b"6"),
        (b"odd/\xff\xfe", S_IFREG | 0o644, b"7"),
    ];
    let stop = AtomicBool::new(false);
    for (name, bytes) in [("t", tar_of(&things)), ("c", cpio(&things)), ("r", rar5(&things))] {
        let arc = dir.join(format!("{name}-arc"));
        fs::write(&arc, bytes).unwrap();
        let out = dir.join(format!("out-{name}"));
        fs::create_dir(&out).unwrap();
        let got = extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap();
        assert!(got.refused.is_empty(), "{name} {:?}", got.refused);
        let read = |n: &str| fs::read(out.join("odd").join(n)).unwrap();
        assert_eq!(read("-rf"), b"1", "{name}");
        assert_eq!(read("new\nline"), b"2", "{name}");
        assert_eq!(read("  spaces  "), b"3", "{name}");
        assert_eq!(read("*?[x]$(id)`id`;&|"), b"4", "{name}");
        assert_eq!(read("\u{202e}txt.exe \u{1f600}"), b"5", "{name}");
        assert_eq!(fs::read_dir(out.join("odd")).unwrap().count(), 7, "{name}");
        assert_eq!(list(&arc, None).unwrap().len(), 7, "{name}");
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn an_archive_made_to_fill_the_disk_or_bury_a_folder_is_stopped() {
    let dir = place("bomb");
    let stop = AtomicBool::new(false);
    fs::write(dir.join("noughts.bin"), vec![0u8; 5 << 20]).unwrap();
    let many: Vec<(String, u32, &[u8])> = (0..40).map(|n| (format!("f{n}"), S_IFREG | 0o644, &b"x"[..])).collect();
    let many: Vec<Thing> = many.iter().map(|(n, m, d)| (n.as_bytes(), *m, *d)).collect();
    let out = dir.join("out");
    fs::create_dir(&out).unwrap();
    for format in [Format::Zip, Format::TarGz, Format::TarXz] {
        let arc = dir.join(format!("bomb{}", format.ending()));
        compress(&[dir.join("noughts.bin")], &arc, format, &stop, &quiet).unwrap();
        assert!(fs::metadata(&arc).unwrap().len() < 100_000);
        let most = Most { things: MOST_THINGS, bytes: 1 << 20 };
        let e = unpack(&arc, None, &out, Taking { need: Need::All, want: &|_| true, roots: Roots::Together, most }, &stop, &quiet).unwrap_err();
        assert_eq!(why(&e), "there isn't enough space here for what's in it", "{format:?}");
        assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "{format:?}: nothing is left");
    }
    let noughts = vec![0u8; 3 << 20];
    let big: [Thing; 1] = [(b"noughts.bin", S_IFREG | 0o644, &noughts)];
    for (name, bytes, bytes_most, things_most, said) in [
        ("big.cpio", cpio(&big), 1 << 20, MOST_THINGS, Trouble::NoRoom),
        ("many.cpio", cpio(&many), u64::MAX, 39, Trouble::TooMany),
        ("many.tar", tar_of(&many), u64::MAX, 39, Trouble::TooMany),
    ] {
        let arc = dir.join(name);
        fs::write(&arc, bytes).unwrap();
        let most = Most { things: things_most, bytes: bytes_most };
        let e = unpack(&arc, None, &out, Taking { need: Need::All, want: &|_| true, roots: Roots::Together, most }, &stop, &quiet).unwrap_err();
        assert_eq!(trouble_of(&e), Some(said), "{name}");
        assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "{name}: nothing is left");
        if said == Trouble::TooMany {
            assert_eq!(trouble_of(&list_up_to(&arc, None, 39).unwrap_err()), Some(Trouble::TooMany), "{name}");
            assert_eq!(list_up_to(&arc, None, 40).unwrap().len(), 40, "{name}");
            let most = Most { things: 40, bytes: u64::MAX };
            let done = unpack(&arc, None, &out, Taking { need: Need::All, want: &|_| true, roots: Roots::Together, most }, &stop, &quiet).unwrap();
            fs::remove_dir_all(&done.made[0]).unwrap();
        }
    }
    let real = Most::here(&dir);
    let free = crate::common::files::room_left(&dir).unwrap();
    assert_eq!(real.things, 1_000_000);
    assert!(real.bytes < free && free - real.bytes <= ROOM_KEPT + (64 << 20), "{} of {free}", real.bytes);
    assert_eq!(why(&trouble(Trouble::TooMany)), "it has more than a million items, which is too many for Folders to unpack");
    let _ = fs::remove_dir_all(&dir);
}
