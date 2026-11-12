//! 7z, listed, read and unpacked.

use super::*;
use super::sz_bytes::{SZ_COPY, SZ_LOCKED, SZ_LOCKEDHEAD, SZ_LZMA, SZ_LZMA2};

fn sz_kinds() -> [(&'static str, &'static [u8]); 3] {
    [("lzma.7z", &SZ_LZMA), ("lzma2.7z", &SZ_LZMA2), ("copy.7z", &SZ_COPY)]
}

#[test]
fn sevenz_lists_and_reads_each_way_it_is_packed() {
    let dir = place("7z-read");
    for (name, bytes) in sz_kinds() {
        let arc = dir.join(name);
        fs::write(&arc, bytes).unwrap();
        assert_eq!(sniff(&arc).unwrap(), Some(Format::SevenZ), "{name}");
        let all = list(&arc, None).unwrap();
        let get = |n: &str| all.iter().find(|e| e.name == n).unwrap_or_else(|| panic!("{name}: no {n}")).clone();
        assert!(get("docs").folder, "{name}");
        assert!(get("docs/empty").folder, "{name}");
        let blank = get("docs/blank.txt");
        assert!(!blank.folder && blank.size == 0, "{name}");
        let hello = get("hello.txt");
        assert_eq!((hello.size, hello.modified), (23, Some(SZ_TIME)), "{name}");
        assert_eq!(hello.mode.map(|m| m & 0o777), Some(0o664), "{name}");
        assert_eq!(get("docs/note.txt").size, 98, "{name}");
        assert_eq!(read(&arc, None, "hello.txt", 1000).unwrap(), SZ_HELLO, "{name}");
        assert_eq!(read(&arc, None, "docs/note.txt", 1000).unwrap(), sz_note(), "{name}");
        assert_eq!(read(&arc, None, "docs/blank.txt", 1000).unwrap(), b"", "{name}");
        assert!(read(&arc, None, "docs/note.txt", 10).is_err(), "{name}");
        assert_eq!(read(&arc, None, "nothing", 1000).unwrap_err().kind(), io::ErrorKind::NotFound);
        let top: Vec<String> = inside(&all, "").into_iter().map(|e| e.name).collect();
        assert!(top.contains(&"docs".to_string()) && top.contains(&"hello.txt".to_string()), "{name}: {top:?}");
        let docs: Vec<String> = inside(&all, "docs").into_iter().map(|e| e.name).collect();
        assert_eq!(docs, ["docs/blank.txt", "docs/empty", "docs/note.txt"], "{name}");
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sevenz_keeps_links_and_unpacks_whole() {
    let dir = place("7z-out");
    let arc = dir.join("lzma.7z");
    fs::write(&arc, SZ_LZMA).unwrap();
    let odd = dir.join("photos.dat");
    fs::write(&odd, SZ_LZMA2).unwrap();
    assert_eq!(read(&odd, None, "hello.txt", 1000).unwrap(), SZ_HELLO);
    let stop = AtomicBool::new(false);
    let out = dir.join("out");
    fs::create_dir(&out).unwrap();
    extract(&arc, None, &out, Place::Folder, &stop, &quiet).unwrap();
    let top = out.join("lzma");
    assert_eq!(fs::read(top.join("hello.txt")).unwrap(), SZ_HELLO);
    assert_eq!(fs::read(top.join("docs/note.txt")).unwrap(), sz_note());
    assert_eq!(fs::read(top.join("docs/blank.txt")).unwrap(), b"");
    assert!(top.join("docs/empty").is_dir());
    assert_eq!(fs::read_link(top.join("link")).unwrap(), Path::new("hello.txt"));
    assert_eq!(fs::metadata(top.join("hello.txt")).unwrap().mtime(), SZ_TIME);
    assert!(leftovers(&out).is_empty());
    let made = compress(&[top.join("hello.txt")], &dir.join("new.7z"), Format::SevenZ, &stop, &quiet);
    assert_eq!(made.unwrap_err().kind(), io::ErrorKind::Unsupported);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sevenz_locked_says_so() {
    let dir = place("7z-locked");
    let open = dir.join("locked.7z");
    fs::write(&open, SZ_LOCKED).unwrap();
    let all = list(&open, None).unwrap();
    assert!(all.iter().any(|e| e.name == "hello.txt" && e.size == 23));
    let e = read(&open, None, "hello.txt", 1000).unwrap_err();
    assert!(e.to_string().contains("password"), "{e}");
    let shut = dir.join("lockedhead.7z");
    fs::write(&shut, SZ_LOCKEDHEAD).unwrap();
    let e = list(&shut, None).unwrap_err();
    assert!(e.to_string().contains("password"), "{e}");
    let (right, wrong) = (Key::new("secret"), Key::new("Secret"));
    assert_eq!(read(&open, Some(&right), "hello.txt", 1000).unwrap(), SZ_HELLO);
    assert_eq!(wants_key(&read(&open, Some(&wrong), "hello.txt", 1000).unwrap_err()), Some(Trouble::WrongOrBad));
    let all = list(&shut, Some(&right)).unwrap();
    assert!(all.iter().any(|e| e.name == "hello.txt" && e.size == 23 && e.locked), "{all:?}");
    assert_eq!(read(&shut, Some(&right), "hello.txt", 1000).unwrap(), SZ_HELLO);
    assert_eq!(wants_key(&list(&shut, Some(&wrong)).unwrap_err()), Some(Trouble::WrongKey));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sevenz_damage_is_an_error_not_a_crash() {
    let dir = place("7z-damage");
    let arc = dir.join("bad.7z");
    for (name, bytes) in sz_kinds() {
        if name == "copy.7z" {
            let mut b = bytes.to_vec();
            let at = b.windows(5).position(|w| w == b"Hello").unwrap();
            b[at] = b'J';
            fs::write(&arc, &b).unwrap();
            assert!(list(&arc, None).is_ok());
            assert!(read(&arc, None, "hello.txt", 1000).is_err());
        }
        for i in 0..bytes.len() {
            let mut b = bytes.to_vec();
            b[i] ^= 0x5a;
            fs::write(&arc, &b).unwrap();
            if list(&arc, None).is_ok() {
                let _ = read(&arc, None, "docs/note.txt", 1 << 20);
            }
            fs::write(&arc, &bytes[..i]).unwrap();
            assert!(list(&arc, None).is_err(), "{name} cut at {i}");
        }
    }
    let _ = fs::remove_dir_all(&dir);
}

fn sz_with_header(head: &[u8]) -> Vec<u8> {
    let mut tail = Vec::new();
    tail.extend(0u64.to_le_bytes());
    tail.extend((head.len() as u64).to_le_bytes());
    let mut sum = Crc::new();
    sum.update(head);
    tail.extend(sum.sum().to_le_bytes());
    let mut start = Crc::new();
    start.update(&tail);
    let mut all = SZ_MAGIC.to_vec();
    all.extend([0, 4]);
    all.extend(start.sum().to_le_bytes());
    all.extend(&tail);
    all.extend(head);
    all
}

fn sz_number(v: u64) -> Vec<u8> {
    let i = (0..8).find(|i| v < 1 << (7 * (i + 1))).unwrap();
    let mut out = vec![((0xffu32 << (8 - i)) as u8) | (v >> (8 * i)) as u8];
    out.extend(&v.to_le_bytes()[..i]);
    out
}

#[test]
fn a_7z_that_counts_more_files_than_its_header_holds_is_refused_and_a_full_one_opens() {
    let dir = place("7z-count");
    let arc = dir.join("count.7z");
    let mut head = vec![0x01, 0x05];
    head.extend(sz_number(1 << 22));
    head.extend([0x00, 0x00]);
    fs::write(&arc, sz_with_header(&head)).unwrap();
    let why = list(&arc, None).expect_err("four million files in nine bytes").to_string();
    assert!(why.contains(t!("folders-archive-7z-a-count-is-far-too-big")), "{why}");
    let n = 20_000usize;
    let all_set = vec![0xffu8; n.div_ceil(8)];
    let mut names = vec![0u8];
    for i in 0..n {
        names.extend(format!("f{i}").encode_utf16().chain([0]).flat_map(u16::to_le_bytes));
    }
    let mut head = vec![0x01, 0x05];
    head.extend(sz_number(n as u64));
    let alternate = vec![0b1010_1010u8; n.div_ceil(8)];
    for (kind, body) in [(0x0e, &all_set), (0x0f, &alternate), (0x11, &names)] {
        head.push(kind);
        head.extend(sz_number(body.len() as u64));
        head.extend(body);
    }
    head.extend([0x00, 0x00]);
    fs::write(&arc, sz_with_header(&head)).unwrap();
    let got = list(&arc, None).unwrap();
    assert_eq!(got.len(), n);
    assert_eq!(got.iter().filter(|e| e.folder).count(), n / 2);
    assert_eq!(got[3].name, "f3");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_7z_block_whose_dictionaries_together_are_too_big_is_refused() {
    let dir = place("7z-dict");
    let arc = dir.join("dict.7z");
    fs::write(&arc, [0u8; 64]).unwrap();
    let dict = 0x6000_0000u32;
    let lzma = || {
        let mut props = vec![0x5d];
        props.extend(dict.to_le_bytes());
        SzCoder { id: vec![0x03, 0x01, 0x01], props, ins: 1, outs: 1 }
    };
    let folder = SzFolder { coders: vec![lzma(), lzma()], binds: vec![(0, 1)], packed: vec![1], sizes: vec![u64::from(dict); 2], crc: None };
    let streams = SzStreams { pack_pos: 0, pack_sizes: vec![32], folders: vec![folder], counts: vec![1], sizes: vec![u64::from(dict)], crcs: vec![None] };
    let (stop, quiet) = (AtomicBool::new(false), |_: u64, _: u64| {});
    let meter = Meter::new(&stop, &quiet);
    let file = File::open(&arc).unwrap();
    let got = sz_unpack(&file, &streams, 0, &SzKeys::new(None), &meter);
    assert_eq!(got.err().map(|e| e.kind()), Some(io::ErrorKind::OutOfMemory));
    let _ = fs::remove_dir_all(&dir);
}
