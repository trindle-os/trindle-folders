// Kinds read through libarchive in its box.

use super::*;
use super::sz_bytes::SZ_LOCKED;

fn boxed_things(script: &[u8]) -> [Thing<'_>; 6] {
    [
        (b"docs", 0o040_755, b""),
        (b"docs/note.txt", 0o100_644, b"John Smith wrote this."),
        (b"docs/deep", 0o040_700, b""),
        (b"docs/deep/run.sh", 0o100_755, script),
        ("docs/deep/caf\u{e9} \u{2713}.txt".as_bytes(), 0o100_600, b"hello"),
        (b"top.txt", 0o100_600, b"top"),
    ]
}

#[test]
fn rar_and_the_other_kinds_open_list_and_unpack_through_the_box() {
    let dir = place("boxed");
    let script = format!("#!/bin/sh\ntouch '{}'\n", dir.join("ran").display());
    let things = boxed_things(script.as_bytes());
    let tar = tar_of(&things);
    let kinds = [
        ("a.rar", rar5(&things)), ("a.cpio", cpio(&things)), ("a.tar.zst", zstd(&tar)), ("a.tzst", zstd(&tar)),
        ("a.rpm", rpm(&things)), ("a.xar", xar(&things)), ("a.tar.lz4", lz4(&tar)), ("a.tar.lzma", lzma_alone(&tar)),
        ("a.tar.lz", lzip(&tar)), ("a.tlz", lzip(&tar)), ("a.tar.Z", compress_z(&tar)),
    ];
    let stop = AtomicBool::new(false);
    for (name, bytes) in kinds {
        let arc = dir.join(name);
        fs::write(&arc, bytes).unwrap();
        assert_eq!(Format::of(&arc), Some(Format::Other), "{name}");
        assert_eq!(find(&arc).unwrap(), Format::Other, "{name}");
        assert_eq!(stem(&arc), "a");
        boxed_lists(&arc, name);
        boxed_unpacks(&dir, &arc, name, &script, &stop);
        boxed_stops_and_breaks(&dir, &arc, name, &stop);
    }
    boxed_harm(&dir, &stop);
    let _ = fs::remove_dir_all(&dir);
}

fn boxed_lists(arc: &Path, name: &str) {
    let all = list(arc, None).unwrap();
    let names: Vec<&str> = all.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["docs", "docs/note.txt", "docs/deep", "docs/deep/run.sh", "docs/deep/caf\u{e9} \u{2713}.txt", "top.txt"], "{name}");
    assert!(all[0].folder && !all[1].folder && all[1].size == 22 && all[1].modified == Some(1_700_000_000), "{name} {all:?}");
    assert_eq!(inside(&all, "docs").len(), 2);
}

fn boxed_unpacks(dir: &Path, arc: &Path, name: &str, script: &str, stop: &AtomicBool) {
    let out = dir.join(format!("out-{name}"));
    fs::create_dir(&out).unwrap();
    let seen = Cell::new((0, 0));
    let tell = |d, t| seen.set((d, t));
    let got = extract(arc, None, &out, Place::Here, stop, &tell).unwrap();
    assert!(got.refused.is_empty(), "{name} {:?}", got.refused);
    assert_eq!(got.made, vec![out.join("a")], "{name}: two things at the top go in a folder");
    assert!(seen.get().1 == fs::metadata(arc).unwrap().len() && seen.get().0 > 0 && seen.get().0 <= seen.get().1, "{name} {:?}", seen.get());
    let top = out.join("a");
    assert_eq!(fs::read(top.join("docs/note.txt")).unwrap(), b"John Smith wrote this.", "{name}");
    assert_eq!(fs::read(top.join("docs/deep/caf\u{e9} \u{2713}.txt")).unwrap(), b"hello", "{name}");
    assert_eq!(fs::read(top.join("top.txt")).unwrap(), b"top", "{name}");
    let mode = |p: &str| fs::metadata(top.join(p)).unwrap().mode() & 0o7777;
    assert_eq!((mode("docs/note.txt"), mode("docs/deep/run.sh"), mode("top.txt"), mode("docs/deep")), (0o644, 0o755, 0o600, 0o700), "{name}");
    assert_eq!(fs::metadata(top.join("docs/note.txt")).unwrap().mtime(), 1_700_000_000, "{name}");
    assert!(leftovers(&out).is_empty());

    let picked = take_out(arc, None, &["docs/deep".into(), "top.txt".into(), "docs/deep/run.sh".into()], &out, stop, &quiet).unwrap();
    assert_eq!(picked.made, vec![out.join("deep"), out.join("top.txt")], "{name}");
    assert_eq!(fs::read(out.join("deep/run.sh")).unwrap(), script.as_bytes());
    assert!(!out.join("note.txt").exists() && !out.join("docs").exists());
    assert_eq!(read(arc, None, "docs/note.txt", 100).unwrap(), b"John Smith wrote this.", "{name}");
    assert!(read(arc, None, "docs/note.txt", 5).is_err());
    assert!(read(arc, None, "docs/nothing", 100).is_err());
}

fn boxed_stops_and_breaks(dir: &Path, arc: &Path, name: &str, stop: &AtomicBool) {
    let halt = AtomicBool::new(false);
    let stopper = |_: u64, total: u64| {
        if total > 0 {
            halt.store(true, Ordering::Relaxed);
        }
    };
    let empty = dir.join(format!("stopped-{name}"));
    fs::create_dir(&empty).unwrap();
    let e = extract(arc, None, &empty, Place::Folder, &halt, &stopper).unwrap_err();
    assert!(is_stopped(&e), "{name} {e}");
    assert_eq!(fs::read_dir(&empty).unwrap().count(), 0, "{name}");

    let cut = dir.join(format!("cut-{name}"));
    let whole = fs::read(arc).unwrap();
    fs::write(&cut, &whole[..whole.len() / 2]).unwrap();
    let e = extract(&cut, None, &empty, Place::Folder, stop, &quiet).unwrap_err();
    assert_eq!(why(&e), "it is damaged or cut short", "{name}");
    assert_eq!(fs::read_dir(&empty).unwrap().count(), 0, "{name}");
}

fn boxed_harm(dir: &Path, stop: &AtomicBool) {
    let harm: [Thing; 3] = [(b"../evil", 0o100_644, b"x"), (b"tool", 0o104_755, b"#!/bin/sh\n"), (b"fine.txt", 0o100_644, b"fine")];
    let bad = dir.join("bad.xar");
    fs::write(&bad, xar(&harm)).unwrap();
    let out = dir.join("out-bad");
    fs::create_dir(&out).unwrap();
    let got = extract(&bad, None, &out, Place::Folder, stop, &quiet).unwrap();
    assert_eq!(got.refused, ["../evil"]);
    assert_eq!(fs::read(out.join("bad/fine.txt")).unwrap(), b"fine");
    let tool = fs::metadata(out.join("bad/tool")).unwrap();
    assert_eq!((tool.mode() & 0o7777, tool.uid()), (0o755, unsafe { libc::getuid() }));
    assert!(!out.join("evil").exists() && !dir.join("evil").exists());
    let deb = dir.join("some.deb");
    fs::write(&deb, ar(&[("debian-binary", b"2.0\n"), ("control.tar", b"not really")])).unwrap();
    assert_eq!(list(&deb, None).unwrap().iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["debian-binary", "control.tar"]);
    assert!(!dir.join("ran").exists(), "nothing that comes out of an archive is run");
}

#[test]
fn a_cabinet_and_an_lha_archive_open_list_and_unpack_through_the_box() {
    let dir = place("cab-lha");
    let script = format!("#!/bin/sh\ntouch '{}'\n", dir.join("ran").display());
    let long = "John Smith wrote this line again.\n".repeat(40);
    let files: [(&str, &[u8]); 5] = [
        ("docs/note.txt", b"John Smith wrote this."),
        ("docs/deep/run.sh", script.as_bytes()),
        ("docs/long.txt", long.as_bytes()),
        ("top.txt", b"top"),
        ("../evil.txt", b"out"),
    ];
    let stop = AtomicBool::new(false);
    for (name, bytes) in [("a.cab", cab(&files)), ("a.lzh", lha(&files)), ("a.lha", lha(&files))] {
        let arc = dir.join(name);
        fs::write(&arc, &bytes).unwrap();
        assert_eq!(Format::of(&arc), Some(Format::Other), "{name}");
        assert_eq!(stem(&arc), "a");
        let whole: usize = files.iter().map(|f| f.1.len()).sum();
        assert!(name != "a.cab" || bytes.len() < whole, "{name}");
        cab_lha_lists(&arc, name, &long);
        cab_lha_unpacks(&dir, &arc, name, (&script, &long), &stop);
        cab_lha_bent(&dir, name, &bytes, &long, &stop);
    }
    assert!(!dir.join("ran").exists(), "nothing that comes out of an archive is run");
    let _ = fs::remove_dir_all(&dir);
}

fn cab_lha_lists(arc: &Path, name: &str, long: &str) {
    let all = list(arc, None).unwrap();
    let names: Vec<&str> = all.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["docs/note.txt", "docs/deep/run.sh", "docs/long.txt", "top.txt", "../evil.txt"], "{name}");
    assert!(!all[0].folder && all[0].size == 22 && all[2].size == long.len() as u64, "{name} {all:?}");
    let docs: Vec<String> = inside(&all, "docs").into_iter().map(|e| e.name).collect();
    assert_eq!(docs, ["docs/deep", "docs/long.txt", "docs/note.txt"], "{name}");
}

fn cab_lha_unpacks(dir: &Path, arc: &Path, name: &str, text: (&str, &str), stop: &AtomicBool) {
    let (script, long) = text;
    let out = dir.join(format!("out-{name}"));
    fs::create_dir(&out).unwrap();
    let got = extract(arc, None, &out, Place::Folder, stop, &quiet).unwrap();
    assert_eq!(got.refused, ["../evil.txt"], "{name}: the name that climbs out is refused");
    assert_eq!(got.made, vec![out.join("a")], "{name}");
    let top = out.join("a");
    assert_eq!(fs::read(top.join("docs/note.txt")).unwrap(), b"John Smith wrote this.", "{name}");
    assert_eq!(fs::read(top.join("docs/deep/run.sh")).unwrap(), script.as_bytes(), "{name}");
    assert_eq!(fs::read(top.join("docs/long.txt")).unwrap(), long.as_bytes(), "{name}");
    assert_eq!(fs::read(top.join("top.txt")).unwrap(), b"top", "{name}");
    assert!(!dir.join("evil.txt").exists() && !out.join("evil.txt").exists(), "{name}");
    assert_eq!(fs::metadata(top.join("docs/deep/run.sh")).unwrap().mode() & 0o7111, 0, "{name}: nothing comes out able to run");
    assert!(leftovers(&out).is_empty());

    let picked = take_out(arc, None, &["docs/deep".into(), "top.txt".into()], &out, stop, &quiet).unwrap();
    assert_eq!(picked.made, vec![out.join("deep"), out.join("top.txt")], "{name}");
    assert_eq!(read(arc, None, "docs/long.txt", 10_000).unwrap(), long.as_bytes(), "{name}");
    assert!(read(arc, None, "docs/long.txt", 5).is_err() && read(arc, None, "docs/nothing", 100).is_err(), "{name}");
}

fn cab_lha_bent(dir: &Path, name: &str, bytes: &[u8], long: &str, stop: &AtomicBool) {
    let empty = dir.join(format!("empty-{name}"));
    fs::create_dir(&empty).unwrap();
    let cut = dir.join(format!("cut-{name}"));
    fs::write(&cut, &bytes[..bytes.len() / 2]).unwrap();
    assert!(extract(&cut, None, &empty, Place::Folder, stop, &quiet).is_err(), "{name}");
    assert_eq!(fs::read_dir(&empty).unwrap().count(), 0, "{name}");
    for i in (0..bytes.len()).step_by(bytes.len() / 12) {
        let mut bent = bytes.to_vec();
        bent[i] ^= 0x5a;
        fs::write(&cut, &bent).unwrap();
        if let Ok(got) = read(&cut, None, "docs/long.txt", 10_000) {
            assert!(got == long.as_bytes(), "{name} bent at {i}");
        }
    }
}

#[test]
fn a_file_squeezed_on_its_own_opens_as_an_archive_of_one() {
    let dir = place("single");
    let words = "John Smith wrote this line again.\n".repeat(300);
    let gz = {
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(words.as_bytes()).unwrap();
        e.finish().unwrap()
    };
    let bz2 = {
        let mut e = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::default());
        e.write_all(words.as_bytes()).unwrap();
        e.finish().unwrap()
    };
    let xz = {
        let mut e = lzma_rust2::XzWriter::new(Vec::new(), lzma_rust2::XzOptions::with_preset(1)).unwrap();
        e.write_all(words.as_bytes()).unwrap();
        e.finish().unwrap()
    };
    let stop = AtomicBool::new(false);
    let kinds = [
        ("notes.txt.gz", gz, Format::Gz), ("notes.txt.bz2", bz2, Format::Bz2), ("notes.txt.xz", xz, Format::Xz), ("notes.txt.zst", zstd(words.as_bytes()), Format::Other),
        ("notes.txt.lz4", lz4(words.as_bytes()), Format::Other), ("notes.txt.lzma", lzma_alone(words.as_bytes()), Format::Other),
        ("notes.txt.lz", lzip(words.as_bytes()), Format::Other), ("notes.txt.Z", compress_z(words.as_bytes()), Format::Other),
    ];
    for (name, bytes, format) in kinds {
        let arc = dir.join(name);
        fs::write(&arc, &bytes).unwrap();
        assert_eq!(find(&arc).unwrap(), format, "{name}");
        let all = list(&arc, None).unwrap();
        assert_eq!(all.len(), 1, "{name}");
        assert_eq!((all[0].name.as_str(), all[0].folder), ("notes.txt", false), "{name}");
        if format != Format::Other {
            assert_eq!(all[0].size, words.len() as u64, "{name}");
        }
        let out = dir.join(format!("out-{name}"));
        fs::create_dir(&out).unwrap();
        let got = extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap();
        assert_eq!(got.made, vec![out.join("notes.txt")], "{name}: one thing goes straight in");
        assert_eq!(fs::read(out.join("notes.txt")).unwrap(), words.as_bytes(), "{name}");
        assert_eq!(read(&arc, None, "notes.txt", 20_000).unwrap(), words.as_bytes(), "{name}");
        assert!(name.ends_with(".zst") || bytes.len() < words.len() / 2, "{name}: {} of {}", bytes.len(), words.len());
        if name.ends_with(".Z") {
            continue;
        }
        let cut = dir.join(format!("cut-{name}"));
        fs::write(&cut, &bytes[..bytes.len() - 20]).unwrap();
        assert!(extract(&cut, None, &out, Place::Folder, &stop, &quiet).is_err(), "{name}");
        assert_eq!(fs::read_dir(&out).unwrap().count(), 1, "{name}");
    }
    let tgz = dir.join("misnamed.gz");
    compress(&[dir.join("notes.txt.gz")], &dir.join("real.tar.gz"), Format::TarGz, &stop, &quiet).unwrap();
    fs::rename(dir.join("real.tar.gz"), &tgz).unwrap();
    assert_eq!(find(&tgz).unwrap(), Format::TarGz);
    assert_eq!(list(&tgz, None).unwrap()[0].name, "notes.txt.gz");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn squeezed_rar_old_and_new_and_the_zips_and_7z_only_libarchive_reads_unpack() {
    let dir = place("squeezed");
    let stop = AtomicBool::new(false);
    let note = "John Smith wrote this line again.\n".repeat(60);
    for name in ["rar5.rar", "rar4.rar", "ppmd.7z", "bzip2.zip", "deflate64.zip"] {
        let arc = fixture(&dir, name);
        let all = list(&arc, None).unwrap();
        let size = |n: &str| all.iter().find(|e| e.name == n && !e.folder).map(|e| e.size);
        assert_eq!((size("album/hello.txt"), size("album/docs/note.txt"), size("album/docs/caf\u{e9}.txt")), (Some(23), Some(2040), Some(5)), "{name} {all:?}");
        let out = dir.join(format!("out-{name}"));
        fs::create_dir(&out).unwrap();
        let got = extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap();
        assert!(got.refused.is_empty(), "{name}");
        assert_eq!(got.made, vec![out.join("album")], "{name}");
        assert_eq!(fs::read(out.join("album/hello.txt")).unwrap(), b"Hello from John Smith.\n", "{name}");
        assert_eq!(fs::read(out.join("album/docs/note.txt")).unwrap(), note.as_bytes(), "{name}");
        assert_eq!(fs::read(out.join("album/docs/caf\u{e9}.txt")).unwrap(), b"hello", "{name}");
        let when = fs::metadata(out.join("album/hello.txt")).unwrap().mtime();
        assert!((when - 1_767_323_045).abs() <= 86_400, "{name} {when}");
        assert_eq!(read(&arc, None, "album/docs/note.txt", 5000).unwrap(), note.as_bytes(), "{name}");
        let picked = take_out(&arc, None, &["album/docs".into()], &out, &stop, &quiet).unwrap();
        assert_eq!(picked.made, vec![out.join("docs")], "{name}");
        assert_eq!(fs::read(out.join("docs/note.txt")).unwrap(), note.as_bytes(), "{name}");
    }
    for name in ["rar5.rar", "rar4.rar"] {
        let whole = FIXTURES.iter().find(|f| f.0 == name).unwrap().1;
        let out = dir.join(format!("bent-{name}"));
        fs::create_dir(&out).unwrap();
        let mut errors = 0;
        for at in (0..whole.len()).step_by(7) {
            let mut bent = whole.to_vec();
            bent[at] ^= 0x10;
            let arc = dir.join("bent.rar");
            fs::write(&arc, bent).unwrap();
            match extract(&arc, None, &out, Place::Folder, &stop, &quiet) {
                Err(_) => {
                    errors += 1;
                    assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "{name} {at}");
                }
                Ok(done) => {
                    let made = fs::read(done.made[0].join("album/docs/note.txt")).ok();
                    assert!(made.is_none() || made.as_deref() == Some(note.as_bytes()), "{name} {at}: wrong bytes came out as if they were right");
                    fs::remove_dir_all(&done.made[0]).unwrap();
                }
            }
        }
        assert!(errors > whole.len() / 7 / 2, "{name}: {errors} of {} bent copies were refused", whole.len() / 7);
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_rar_split_across_files_unpacks_whole_or_says_a_part_is_missing() {
    let dir = place("split");
    let stop = AtomicBool::new(false);
    let said = "a part of it is missing, and every part has to be in the same folder";
    let old = rar4_split(b"album/noise.bin", &noise(2600), &[1000, 1000]);
    type Parts<'a> = Vec<(&'a str, &'a [u8])>;
    let sets: [(&str, Parts); 3] = [
        ("split", ["split.part01.rar", "split.part02.rar", "split.part03.rar"].iter().map(|n| (*n, FIXTURES.iter().find(|f| f.0 == *n).unwrap().1)).collect()),
        ("old", ["old.part1.rar", "old.part2.rar", "old.part3.rar"].into_iter().zip(old.iter().map(Vec::as_slice)).collect()),
        ("older", ["older.rar", "older.r00", "older.r01"].into_iter().zip(old.iter().map(Vec::as_slice)).collect()),
    ];
    for (name, parts) in sets {
        let set = dir.join(name);
        fs::create_dir(&set).unwrap();
        for (part, bytes) in &parts {
            fs::write(set.join(part), bytes).unwrap();
        }
        let out = dir.join(format!("out-{name}"));
        fs::create_dir(&out).unwrap();
        let openable: Vec<&str> = parts.iter().map(|p| p.0).filter(|p| Format::of(Path::new(p)).is_some()).collect();
        assert_eq!(openable.len(), if name == "older" { 1 } else { 3 });
        for part in openable {
            let arc = set.join(part);
            assert_eq!(stem(&arc), name);
            let all = list(&arc, None).unwrap();
            assert_eq!(all.iter().find(|e| e.name == "album/noise.bin").map(|e| e.size), Some(2600), "{part} {all:?}");
            let seen = Cell::new((0, 0));
            let tell = |d, t| seen.set((d, t));
            let got = extract(&arc, None, &out, Place::Folder, &stop, &tell).unwrap();
            assert_eq!(seen.get().1, parts.iter().map(|p| p.1.len() as u64).sum::<u64>(), "{part}: the whole is all the parts");
            assert_eq!(fs::read(got.made[0].join("album/noise.bin")).unwrap(), noise(2600), "{part}");
            assert_eq!(read(&arc, None, "album/noise.bin", 5000).unwrap(), noise(2600), "{part}");
            fs::remove_dir_all(&got.made[0]).unwrap();
        }
        let first = set.join(parts[0].0);
        fs::remove_file(set.join(parts[2].0)).unwrap();
        assert_eq!(why(&extract(&first, None, &out, Place::Folder, &stop, &quiet).unwrap_err()), said, "{name}");
        assert_eq!(why(&read(&first, None, "album/noise.bin", 5000).unwrap_err()), said, "{name}");
        assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "{name}");
        assert!(list(&first, None).unwrap().iter().any(|e| e.name == "album/noise.bin"), "{name}");
    }
    let second = dir.join("split/split.part02.rar");
    fs::remove_file(dir.join("split/split.part01.rar")).unwrap();
    assert_eq!(why(&extract(&second, None, &dir.join("out-split"), Place::Folder, &stop, &quiet).unwrap_err()), said);
    assert_eq!(fs::read_dir(dir.join("out-split")).unwrap().count(), 0);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_locked_rar_says_it_is_a_rar_with_a_password_and_leaves_nothing() {
    let dir = place("locked");
    let stop = AtomicBool::new(false);
    let said = "it is a RAR locked with a password, which Folders cannot unlock; nothing is wrong with the file";
    let out = dir.join("out");
    fs::create_dir(&out).unwrap();
    for name in ["rar5-locked.rar", "rar4-locked.rar"] {
        let arc = fixture(&dir, name);
        let all = list(&arc, None).unwrap();
        assert!(all.iter().any(|e| e.name == "album/hello.txt"), "{name} {all:?}");
        assert_eq!(why(&extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap_err()), said, "{name}");
        assert_eq!(why(&read(&arc, None, "album/hello.txt", 1000).unwrap_err()), said, "{name}");
        assert_eq!(why(&take_out(&arc, None, &["album/docs".into()], &out, &stop, &quiet).unwrap_err()), said, "{name}");
        assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "{name}");
    }
    for name in ["rar5-lockedhead.rar", "rar4-lockedhead.rar"] {
        let arc = fixture(&dir, name);
        assert_eq!(why(&list(&arc, None).unwrap_err()), said, "{name}");
        assert_eq!(why(&extract(&arc, None, &out, Place::Here, &stop, &quiet).unwrap_err()), said, "{name}");
        assert_eq!(fs::read_dir(&out).unwrap().count(), 0, "{name}");
    }
    let seven = dir.join("locked.7z");
    fs::write(&seven, SZ_LOCKED).unwrap();
    assert_eq!(why(&extract(&seven, None, &out, Place::Here, &stop, &quiet).unwrap_err()), "it is locked with a password");
    assert_eq!(fs::read_dir(&out).unwrap().count(), 0);
    let _ = fs::remove_dir_all(&dir);
}
