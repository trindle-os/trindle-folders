//! What the child sends and how Folders reads it.

use crate::files::unarchive::{HAS_BYTES, LOCKED, MARK, MISSING, NO_MEMORY, SELF, UNKNOWN, UNSUPPORTED};
use crate::files::unarchive::child::{asked, serve};
use crate::files::unarchive::rar::{rar_part, volumes};
use crate::files::unarchive::run::{command, listen};
use super::*;

fn frames_of(tag: &str, bytes: &[u8], need: Need, chosen: &[u32]) -> Vec<u8> {
    let dir = place(tag);
    std::fs::write(dir.join("a"), bytes).unwrap();
    let mut out = Vec::new();
    serve(&[dir.join("a")], need, chosen, &mut out).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    out
}

type Heard = (String, char, Option<u32>, Vec<u8>);

fn heard(frames: &[u8]) -> io::Result<Vec<Heard>> {
    let stop = AtomicBool::new(false);
    let quiet = |_: u64, _: u64| {};
    let meter = Meter::new(&stop, &quiet);
    let mut all = Vec::new();
    listen(frames, &meter, "alone", &mut |item, r| {
        let mut bytes = Vec::new();
        r.read_to_end(&mut bytes)?;
        let kind = match &item.kind {
            Kind::File => 'f',
            Kind::Folder => 'd',
            Kind::Link(to) => {
                bytes = to.clone().into_bytes();
                'l'
            }
            Kind::Hard(_) => 'h',
        };
        all.push((item.name.clone(), kind, item.mode, bytes));
        Ok(true)
    })?;
    Ok(all)
}

#[test]
fn the_library_is_here() {
    assert!(libarchive::there(), "libarchive13t64 is one of Trindle's packages");
}

#[test]
fn a_cpio_comes_through_the_frames_without_its_devices() {
    let things: [Thing; 5] = [
        (b"dir", 0o040_755, b""),
        (b"dir/words.txt", 0o104_755, b"John Smith wrote this."),
        (b"dir/link", 0o120_777, b"words.txt"),
        (b"dir/tty", 0o020_666, b""),
        (b"dir/caf\xe9 \n-x", 0o100_600, b"odd"),
    ];
    let frames = frames_of("cpio", &cpio(&things), Need::All, &[]);
    let got = heard(&frames).unwrap();
    assert_eq!(got.len(), 4, "the device is not sent: {got:?}");
    assert_eq!(got[0], ("dir".to_string(), 'd', Some(0o755), vec![]));
    assert_eq!(got[1], ("dir/words.txt".to_string(), 'f', Some(0o4755), b"John Smith wrote this.".to_vec()), "the mode comes as it is; unpacking drops the set-user bit");
    assert_eq!(got[2], ("dir/link".to_string(), 'l', Some(0o777), b"words.txt".to_vec()));
    assert_eq!(got[3].0, "dir/caf\u{fffd} \n-x", "a name that is not UTF-8 still comes");
    assert_eq!(got[3].3, b"odd");

    let names = heard(&frames_of("cpio-names", &cpio(&things), Need::Names, &[])).unwrap();
    assert_eq!(names.iter().map(|n| n.0.as_str()).collect::<Vec<_>>(), ["dir", "dir/words.txt", "dir/link", "dir/caf\u{fffd} \n-x"]);
    assert!(names.iter().all(|n| n.1 != 'f' || n.3.is_empty()));

    let some = heard(&frames_of("cpio-some", &cpio(&things), Need::Some, &[1, 3])).unwrap();
    assert_eq!(some.iter().map(|n| n.0.as_str()).collect::<Vec<_>>(), ["dir/words.txt", "dir/caf\u{fffd} \n-x"]);
}

#[test]
fn rar_ar_and_zstd_come_through_the_frames() {
    let rar = rar5(&[(b"docs", 0o040_755, b""), (b"docs/note.txt", 0o100_644, b"hello from a rar")]);
    let got = heard(&frames_of("rar", &rar, Need::All, &[])).unwrap();
    assert_eq!(got, [("docs".to_string(), 'd', Some(0o755), vec![]), ("docs/note.txt".to_string(), 'f', Some(0o644), b"hello from a rar".to_vec())]);

    let deb = ar(&[("debian-binary", b"2.0\n"), ("odd", b"abc")]);
    let got = heard(&frames_of("ar", &deb, Need::All, &[])).unwrap();
    assert_eq!(got.iter().map(|g| (g.0.as_str(), g.3.as_slice())).collect::<Vec<_>>(), [("debian-binary", &b"2.0\n"[..]), ("odd", b"abc")]);

    let got = heard(&frames_of("zst", &zstd(b"squeezed words"), Need::All, &[])).unwrap();
    assert_eq!(got, [("alone".to_string(), 'f', None, b"squeezed words".to_vec())]);
}

#[test]
fn what_is_no_archive_or_is_damaged_ends_with_the_reason() {
    let reason = |frames: &[u8]| archive::why(&heard(frames).unwrap_err());
    assert_eq!(reason(&frames_of("plain", b"just some words, not an archive at all", Need::All, &[])), Trouble::Unknown.to_string());
    assert_eq!(reason(&frames_of("empty", b"", Need::All, &[])), Trouble::Unknown.to_string());
    let rar = rar5(&[(b"big.bin", 0o100_644, &[7u8; 5000])]);
    assert_eq!(reason(&frames_of("cut", &rar[..rar.len() - 3000], Need::All, &[])), "it's damaged or incomplete");
    let mut bent = rar.clone();
    bent[20] ^= 1;
    assert_eq!(reason(&frames_of("bent", &bent, Need::All, &[])), "it's damaged or incomplete");
}

#[test]
fn frames_that_make_no_sense_are_refused_not_believed() {
    let head = |kind: u8, flags: u8, name: &[u8], name_len: u32| {
        let mut h = MARK.to_vec();
        h.extend_from_slice(&[b'H', kind, flags]);
        h.extend_from_slice(&0u32.to_le_bytes());
        h.extend_from_slice(&0i64.to_le_bytes());
        h.extend_from_slice(&0u64.to_le_bytes());
        h.extend_from_slice(&name_len.to_le_bytes());
        h.extend_from_slice(&0u32.to_le_bytes());
        h.extend_from_slice(name);
        h
    };
    let with = |mut h: Vec<u8>, more: &[u8]| {
        h.extend_from_slice(more);
        h
    };
    let good = with(head(0, HAS_BYTES, b"a", 1), b"D\x02\x00\x00\x00abD\x01\x00\x00\x00cZE\x00");
    assert_eq!(heard(&good).unwrap(), [("a".to_string(), 'f', None, b"abc".to_vec())]);
    assert_eq!(heard(&with(b"\nrunning 1 test\n".to_vec(), &good)).unwrap().len(), 1);
    let bad: [Vec<u8>; 9] = [
        Vec::new(),
        b"no mark here".to_vec(),
        head(0, 0, b"a", u32::MAX),
        head(0, 0, b"a", 5),
        with(head(9, 0, b"a", 1), b"E\x00"),
        with(head(1, HAS_BYTES, b"a", 1), b"ZE\x00"),
        with(head(0, HAS_BYTES, b"a", 1), b"D\x00\x00\x00\x00ZE\x00"),
        with(head(0, HAS_BYTES, b"a", 1), b"D\xff\xff\xff\x7f"),
        with(head(0, HAS_BYTES, b"a", 1), b"D\x05\x00\x00\x00ab"),
    ];
    for (n, frames) in bad.iter().enumerate() {
        assert!(heard(frames).is_err(), "{n}");
    }
    assert_eq!(archive::why(&heard(&head(0, 0, b"a", 1)).unwrap_err()), Trouble::GaveUp.to_string());
    for (code, said) in [(LOCKED, Trouble::Locked), (UNKNOWN, Trouble::Unknown), (NO_MEMORY, Trouble::GaveUp), (MISSING, Trouble::NoReader), (UNSUPPORTED, Trouble::Foreign)] {
        assert_eq!(archive::why(&heard(&with(MARK.to_vec(), &[b'E', code])).unwrap_err()), said.to_string());
    }
}

#[test]
fn the_box_shows_the_child_only_the_archive_and_itself() {
    let cmd = command(&[PathBuf::from("/tmp/some.part1.rar"), PathBuf::from("/tmp/some.part2.rar")], Need::Names).unwrap();
    let args: Vec<String> = cmd.get_args().map(|a| a.to_string_lossy().into_owned()).collect();
    let has = |run: &[&str]| args.windows(run.len()).any(|w| w == run);
    assert!(has(&["--unshare-all"]) && has(&["--clearenv"]) && has(&["--die-with-parent"]) && has(&["--new-session"]));
    assert!(has(&["--ro-bind", "/tmp/some.part1.rar", walled::INSIDE]));
    assert!(has(&["--ro-bind", "/tmp/some.part2.rar", "/thumb/in.1"]), "a split archive's other parts beside the first");
    assert!(has(&["--ro-bind", &std::env::current_exe().unwrap().to_string_lossy(), SELF]));
    assert!(has(&["--setenv", CHILD, "names 2"]));
    assert!(!args.iter().any(|a| a == "--bind" || a == "--dev-bind" || a == "--share-net"), "nothing it can write to, and no network");
    let binds = args.iter().filter(|a| a.ends_with("bind") || a.ends_with("bind-try")).count();
    assert_eq!(binds, 7, "/usr, three of /etc's lists, the archive's two parts and this program, all read only");
    assert_eq!(asked("all 3"), Some((Need::All, 3)));
    assert!(asked("all").is_none() && asked("all 0").is_none() && asked("all 1001").is_none() && asked("every 1").is_none());
}

#[test]
fn the_parts_of_a_split_rar_are_found_whichever_is_opened() {
    let dir = place("volumes");
    let make = |names: &[&str]| names.iter().for_each(|n| std::fs::write(dir.join(n), b"x").unwrap());
    let found = |name: &str| volumes(&dir.join(name)).iter().map(|p| p.file_name().unwrap().to_string_lossy().into_owned()).collect::<Vec<_>>();
    make(&["a.part01.rar", "a.part02.rar", "a.part03.rar", "a.part05.rar", "b.rar", "b.r00", "b.r01", "b.r03", "c.rar", "d.part2.rar", "E.PART1.RAR", "E.PART2.RAR", "f.zip"]);
    assert_eq!(found("a.part02.rar"), ["a.part01.rar", "a.part02.rar", "a.part03.rar"], "in order, up to the first gap");
    assert_eq!(found("a.part05.rar"), ["a.part05.rar"], "past a gap it stands alone, and its own header says a part is missing");
    assert_eq!(found("b.rar"), ["b.rar", "b.r00", "b.r01"]);
    assert_eq!(found("c.rar"), ["c.rar"]);
    assert_eq!(found("d.part2.rar"), ["d.part2.rar"]);
    assert_eq!(found("E.PART2.RAR"), ["E.PART1.RAR", "E.PART2.RAR"]);
    assert_eq!(found("f.zip"), ["f.zip"]);
    assert_eq!(rar_part("photos.part03"), Some(("photos.part", "03")));
    assert!(rar_part("photos").is_none() && rar_part(".part1").is_none() && rar_part("photos.part").is_none() && rar_part("photos.partx").is_none());
    let _ = std::fs::remove_dir_all(&dir);
}
