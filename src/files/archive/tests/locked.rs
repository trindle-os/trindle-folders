//! Zip and 7z locked with a password.

use super::*;

#[test]
fn a_locked_zip_or_7z_opens_with_its_password_and_no_other() {
    let dir = place("unlock");
    let stop = AtomicBool::new(false);
    let (right, wrong) = (Key::new(LOCKED_KEY), Key::new("john-smith-tesT"));
    let note = sz_note();
    for name in ["locked-old.zip", "locked-aes.zip", "locked.7z"] {
        let arc = locked(&dir, name);
        let out = dir.join(format!("out-{name}"));
        fs::create_dir(&out).unwrap();
        let all = list(&arc, None).unwrap();
        let get = |n: &str| all.iter().find(|e| e.name == n).unwrap_or_else(|| panic!("{name}: no {n}")).clone();
        assert!(get("hello.txt").locked && get("docs/note.txt").locked && !get("docs").locked, "{name}");
        assert_eq!((get("hello.txt").size, get("docs/note.txt").size), (23, 98), "{name}");
        for e in [
            extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap_err(),
            take_out(&arc, None, &["docs".into()], &out, &stop, &quiet).unwrap_err(),
            read(&arc, None, "hello.txt", 1000).unwrap_err(),
        ] {
            assert_eq!(wants_key(&e), Some(Trouble::Locked), "{name}: {e}");
            assert_eq!(why(&e), "it is locked with a password", "{name}");
        }
        let sure = if name.ends_with(".zip") { Trouble::WrongKey } else { Trouble::WrongOrBad };
        assert_eq!(wants_key(&unlocks(&arc, &wrong).unwrap_err()), Some(sure), "{name}");
        for e in [
            extract(&arc, Some(&wrong), &out, Place::Here, &stop, &quiet).unwrap_err(),
            take_out(&arc, Some(&wrong), &["docs".into()], &out, &stop, &quiet).unwrap_err(),
            read(&arc, Some(&wrong), "hello.txt", 1000).unwrap_err(),
        ] {
            assert_eq!(wants_key(&e), Some(sure), "{name}: {e}");
        }
        assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "{name}: nothing is left");
        unlocks(&arc, &right).unwrap();
        assert_eq!(read(&arc, Some(&right), "hello.txt", 1000).unwrap(), SZ_HELLO, "{name}");
        assert_eq!(read(&arc, Some(&right), "docs/note.txt", 1000).unwrap(), note, "{name}");
        let done = extract(&arc, Some(&right), &out, Place::Folder, &stop, &quiet).unwrap();
        assert!(done.refused.is_empty(), "{name}: {:?}", done.refused);
        let top = out.join(stem(&arc));
        assert_eq!(done.made, vec![top.clone()], "{name}");
        assert_eq!(fs::read(top.join("hello.txt")).unwrap(), SZ_HELLO, "{name}");
        assert_eq!(fs::read(top.join("docs/note.txt")).unwrap(), note, "{name}");
        if name.ends_with(".7z") {
            assert_eq!(fs::metadata(top.join("hello.txt")).unwrap().mtime(), SZ_TIME, "{name}");
        }
        let done = take_out(&arc, Some(&right), &["docs".into()], &out, &stop, &quiet).unwrap();
        assert_eq!(done.made, vec![out.join("docs")], "{name}");
        assert_eq!(fs::read(out.join("docs/note.txt")).unwrap(), note, "{name}");
        assert!(leftovers(&out).is_empty(), "{name}");
    }
    let old = dir.join("locked-old.zip");
    assert!(!list(&old, None).unwrap().iter().find(|e| e.name == "open.txt").unwrap().locked);
    assert_eq!(read(&old, None, "open.txt", 1000).unwrap(), b"Not locked.\n");
    let mut both = note.clone();
    both.extend_from_slice(SZ_HELLO);
    assert_eq!(read(&dir.join("locked-aes.zip"), Some(&right), "docs/copy.txt", 1000).unwrap(), both);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_7z_with_its_names_locked_asks_before_it_lists() {
    let dir = place("unlock-names");
    let stop = AtomicBool::new(false);
    let (right, wrong) = (Key::new(LOCKED_KEY), Key::new("not it"));
    let arc = locked(&dir, "locked-names.7z");
    let out = dir.join("out");
    fs::create_dir(&out).unwrap();
    assert_eq!(wants_key(&list(&arc, None).unwrap_err()), Some(Trouble::Locked));
    assert_eq!(wants_key(&extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap_err()), Some(Trouble::Locked));
    let e = list(&arc, Some(&wrong)).unwrap_err();
    assert_eq!(wants_key(&e), Some(Trouble::WrongKey));
    assert_eq!(why(&e), "that password does not open it");
    assert_eq!(wants_key(&unlocks(&arc, &wrong).unwrap_err()), Some(Trouble::WrongKey));
    assert_eq!(wants_key(&extract(&arc, Some(&wrong), &out, Place::Here, &stop, &quiet).unwrap_err()), Some(Trouble::WrongKey));
    assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "nothing is left");
    let all = list(&arc, Some(&right)).unwrap();
    let names: Vec<&str> = all.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["hello.txt", "docs/note.txt", "docs/blank.txt", "docs"]);
    assert!(all[0].locked && all[1].locked && !all[2].locked && all[3].folder);
    unlocks(&arc, &right).unwrap();
    extract(&arc, Some(&right), &out, Place::Folder, &stop, &quiet).unwrap();
    assert_eq!(fs::read(out.join("locked-names/hello.txt")).unwrap(), SZ_HELLO);
    assert_eq!(fs::read(out.join("locked-names/docs/note.txt")).unwrap(), sz_note());
    assert_eq!(fs::read(out.join("locked-names/docs/blank.txt")).unwrap(), b"");
    assert!(leftovers(&out).is_empty());
    let once = sz_key("a", 0, b"").unwrap();
    assert_eq!(once.as_slice(), Sha256::digest([b'a', 0, 0, 0, 0, 0, 0, 0, 0, 0]).as_slice());
    assert_eq!(&sz_key("ab", 0x3f, b"s").unwrap()[..6], [b's', b'a', 0, b'b', 0, 0]);
    assert!(sz_key("a", 25, b"").is_err(), "a lock made to take for ever is refused");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_lock_folders_cannot_open_says_which_and_never_asks_in_vain() {
    let dir = place("unlock-odd");
    let stop = AtomicBool::new(false);
    let right = Key::new(LOCKED_KEY);
    let out = dir.join("out");
    fs::create_dir(&out).unwrap();
    let odd = locked(&dir, "locked-odd.7z");
    let zip = dir.join("odd.zip");
    locked_zip(&zip, &[("hello.txt", S_IFREG | 0o644, SZ_HELLO)], LOCKED_KEY, 12);
    for arc in [&odd, &zip] {
        assert!(list(arc, None).unwrap().iter().any(|e| e.name == "hello.txt" && e.locked), "{arc:?}");
        for key in [None, Some(&right)] {
            let e = extract(arc, key, &out, Place::Here, &stop, &quiet).unwrap_err();
            assert_eq!(trouble_of(&e), Some(Trouble::LockedForeign), "{arc:?}");
            assert_eq!(wants_key(&e), None, "{arc:?}");
            assert_eq!(why(&e), "it uses a kind of password protection Folders can't unlock");
        }
        assert_eq!(trouble_of(&unlocks(arc, &right).unwrap_err()), Some(Trouble::LockedForeign), "{arc:?}");
    }
    for name in ["rar5-locked.rar", "rar4-locked.rar", "rar5-lockedhead.rar", "rar4-lockedhead.rar"] {
        let e = extract(&fixture(&dir, name), Some(&right), &out, Place::Here, &stop, &quiet).unwrap_err();
        assert_eq!(trouble_of(&e), Some(Trouble::LockedRar), "{name}");
        assert_eq!(wants_key(&e), None, "{name}");
    }
    assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "nothing is left");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_locked_archive_is_held_to_every_rule_once_it_is_unlocked() {
    let dir = place("unlock-evil");
    let stop = AtomicBool::new(false);
    let right = Key::new(LOCKED_KEY);
    let entries: [(&str, u32, &[u8]); 10] = [
        ("../evil", S_IFREG | 0o644, b"x"),
        ("/abs", S_IFREG | 0o644, b"x"),
        ("ok/../../x", S_IFREG | 0o644, b"x"),
        ("ok/up", S_IFLNK | 0o777, b"../../etc"),
        ("ok/root", S_IFLNK | 0o777, b"/etc/passwd"),
        ("ok/in", S_IFLNK | 0o777, b"../ok/fine.txt"),
        ("ok/a", S_IFLNK | 0o777, b"."),
        ("ok/a/x", S_IFLNK | 0o777, b".."),
        ("ok/fine.txt", S_IFREG | 0o644, b"fine"),
        ("ok/tool", S_IFREG | 0o6755, b"#!/bin/sh\ntouch ran\n"),
    ];
    let zip = dir.join("evil.zip");
    locked_zip(&zip, &entries, LOCKED_KEY, 0);
    let out = dir.join("out");
    fs::create_dir(&out).unwrap();
    assert_eq!(wants_key(&extract(&zip, None, &out, Place::Folder, &stop, &quiet).unwrap_err()), Some(Trouble::Locked));
    let got = extract(&zip, Some(&right), &out, Place::Folder, &stop, &quiet).unwrap();
    let mut refused = got.refused.clone();
    refused.sort();
    assert_eq!(refused, ["../evil", "/abs", "ok/../../x", "ok/a/x", "ok/root", "ok/up"]);
    let top = out.join("evil");
    assert_eq!(fs::read(top.join("ok/fine.txt")).unwrap(), b"fine");
    assert_eq!(fs::read(top.join("ok/in")).unwrap(), b"fine");
    assert_eq!(fs::metadata(top.join("ok/tool")).unwrap().mode() & 0o7777, 0o755, "the set-user and set-group bits are gone");
    assert!(!dir.join("evil").exists() && !dir.join("x").exists() && !out.join("x").exists() && !top.join("ok/ran").exists());
    assert!(leftovers(&out).is_empty());
    let seven = locked(&dir, "locked-evil.7z");
    let out = dir.join("out7");
    fs::create_dir(&out).unwrap();
    assert_eq!(wants_key(&list(&seven, None).unwrap_err()), Some(Trouble::Locked));
    let got = extract(&seven, Some(&right), &out, Place::Folder, &stop, &quiet).unwrap();
    let mut refused = got.refused.clone();
    refused.sort();
    assert_eq!(refused, ["../escape.txt", "/abs.txt", "link-out"]);
    let top = out.join("locked-evil");
    assert_eq!(fs::read(top.join("ok.txt")).unwrap(), b"fine");
    assert_eq!(fs::metadata(top.join("sub/tool")).unwrap().mode() & 0o7777, 0o755);
    assert!(!dir.join("escape.txt").exists() && !out.join("escape.txt").exists() && fs::symlink_metadata(top.join("link-out")).is_err());
    assert!(leftovers(&out).is_empty());
    let arc = locked(&dir, "locked.7z");
    let room = dir.join("room");
    fs::create_dir(&room).unwrap();
    for (most, said) in [(Most { things: 3, bytes: u64::MAX }, Trouble::TooMany), (Most { things: MOST_THINGS, bytes: 10 }, Trouble::NoRoom)] {
        for arc in [&arc, &zip] {
            let e = unpack(arc, Some(&right), &room, Taking { need: Need::All, want: &|_| true, roots: Roots::Together, most }, &stop, &quiet).unwrap_err();
            assert_eq!(trouble_of(&e), Some(said), "{arc:?}");
        }
    }
    let halt = AtomicBool::new(false);
    let e = extract(&arc, Some(&right), &room, Place::Folder, &halt, &|_, _| halt.store(true, Ordering::Relaxed)).unwrap_err();
    assert!(is_stopped(&e), "{e}");
    assert_eq!(fs::read_dir(&room).unwrap().count(), 0, "nothing is left");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_damaged_locked_archive_fails_or_reads_right_and_never_gives_wrong_bytes() {
    let dir = place("unlock-damage");
    let right = Key::new(LOCKED_KEY);
    let arc = dir.join("bent");
    let note = sz_note();
    for (name, step) in [("locked-old.zip", 1), ("locked-aes.zip", 1), ("locked-names.7z", 1), ("locked.7z", 9)] {
        let bytes = LOCKED.iter().find(|(n, _)| *n == name).unwrap().1;
        for i in (0..bytes.len()).step_by(step) {
            let mut b = bytes.to_vec();
            b[i] ^= 0x5a;
            fs::write(&arc, &b).unwrap();
            let _ = list(&arc, Some(&right));
            for (inner, want) in [("hello.txt", SZ_HELLO), ("docs/note.txt", note.as_slice())] {
                if let Ok(got) = read(&arc, Some(&right), inner, 1 << 20) {
                    assert_eq!(got, want, "{name} bent at {i}: {inner}");
                }
            }
            if i % 7 == 0 {
                fs::write(&arc, &bytes[..i]).unwrap();
                assert!(read(&arc, Some(&right), "docs/note.txt", 1 << 20).is_err(), "{name} cut at {i}");
            }
        }
    }
    let _ = fs::remove_dir_all(&dir);
}
