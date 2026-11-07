// Round trips, zip64 and the shapes of listings and times.

use super::*;

#[test]
fn every_format_goes_round() {
    let dir = place("round");
    let src = sample(&dir);
    fs::write(dir.join("single.txt"), b"one thing").unwrap();
    let stop = AtomicBool::new(false);
    for format in Format::ALL {
        let to = dir.join(format!("pack{}", format.ending()));
        let seen = Cell::new((0, 0));
        let tell = |d, t| seen.set((d, t));
        compress(&[src.clone(), dir.join("single.txt")], &to, format, &stop, &tell).unwrap();
        assert_eq!(seen.get().0, seen.get().1, "{format:?}");
        assert_eq!(find(&to).unwrap(), format);
        assert!(compress(std::slice::from_ref(&src), &to, format, &stop, &quiet).is_err(), "never over an archive");

        let names: Vec<String> = list(&to, None).unwrap().into_iter().map(|e| e.name).collect();
        assert!(names.contains(&"src/deep/deeper/café ünïcode.txt".to_string()), "{format:?} {names:?}");
        let out = dir.join(format!("out-{format:?}"));
        fs::create_dir(&out).unwrap();
        let got = extract(&to, None, &out, Place::Here, &stop, &quiet).unwrap();
        assert!(got.refused.is_empty(), "{:?}", got.refused);
        assert_eq!(got.made, vec![out.join("pack")], "two things at the top go in a folder");
        same(&src, &out.join("pack/src"));
        assert_eq!(fs::read(out.join("pack/single.txt")).unwrap(), b"one thing");
        assert!(leftovers(&out).is_empty());
        assert!(leftovers(&dir).is_empty());

        let picked = take_out(&to, None, &["src/deep".into(), "src/deep/words.txt".into(), "src/link".into()], &out, &stop, &quiet).unwrap();
        assert_eq!(picked.made, vec![out.join("deep"), out.join("link")]);
        same(&src.join("deep"), &out.join("deep"));
        assert_eq!(read(&to, None, "src/deep/deeper/café ünïcode.txt", 100).unwrap(), b"hello");
        assert!(read(&to, None, "src/noise.bin", 100).is_err());
        assert!(read(&to, None, "src/nothing", 100).is_err());
    }
    let one = dir.join("one.tar.gz");
    compress(std::slice::from_ref(&src), &one, Format::TarGz, &stop, &quiet).unwrap();
    let out = dir.join("out-one");
    fs::create_dir(&out).unwrap();
    fs::create_dir(out.join("src")).unwrap();
    let got = extract(&one, None, &out, Place::Here, &stop, &quiet).unwrap();
    assert_eq!(got.made, vec![out.join("src (2)")], "one thing at the top goes straight in, beside what is there");
    let got = extract(&one, None, &out, Place::Folder, &stop, &quiet).unwrap();
    assert_eq!(got.made, vec![out.join("one")]);
    same(&src, &out.join("one/src"));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn zip64_goes_round() {
    let dir = place("zip64");
    let src = sample(&dir);
    let to = dir.join("big.zip");
    let stop = AtomicBool::new(false);
    let meter = Meter::new(&stop, &quiet);
    pack(std::slice::from_ref(&src), &to, Format::Zip, 1000, &meter).unwrap();
    let out = dir.join("out");
    fs::create_dir(&out).unwrap();
    extract(&to, None, &out, Place::Here, &stop, &quiet).unwrap();
    same(&src, &out.join("src"));
    let mut shifted = b"#!/bin/sh\nexit 0\n".to_vec();
    shifted.extend(fs::read(&to).unwrap());
    fs::write(dir.join("shifted.zip"), shifted).unwrap();
    assert_eq!(read(&dir.join("shifted.zip"), None, "src/deep/deeper/café ünïcode.txt", 10).unwrap(), b"hello");
    let small = dir.join("small.zip");
    compress(std::slice::from_ref(&src), &small, Format::Zip, &stop, &quiet).unwrap();
    assert!(fs::metadata(&small).unwrap().len() < 205_000, "words squeezed, noise stored");
    let _ = fs::remove_dir_all(&dir);
}

fn crafted_zip64(prefix: usize, packed: u64, offset: u64, locator_to: Option<u64>) -> Vec<u8> {
    let mut z = vec![0u8; prefix];
    let mut local = [0u8; 31];
    local[..6].copy_from_slice(&[0x50, 0x4b, 3, 4, 20, 0]);
    (local[26], local[30]) = (1, b'a');
    z.extend(local);
    let cd_at = z.len() - prefix;
    let mut extra = vec![1, 0, 24, 0];
    for n in [0u64, packed, offset] {
        extra.extend(n.to_le_bytes());
    }
    let mut c = vec![0x50, 0x4b, 1, 2, 20, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    c.extend([0xff; 8]);
    c.extend([1, 0, extra.len() as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    c.extend([0xff; 4]);
    c.push(b'a');
    c.extend(extra);
    let cd_size = c.len();
    z.extend(c);
    if let Some(to) = locator_to {
        z.extend([0x50, 0x4b, 6, 7, 0, 0, 0, 0]);
        z.extend(to.to_le_bytes());
        z.extend([1, 0, 0, 0]);
    }
    z.extend([0x50, 0x4b, 5, 6, 0, 0, 0, 0, 1, 0, 1, 0]);
    z.extend((cd_size as u32).to_le_bytes());
    z.extend((cd_at as u32).to_le_bytes());
    z.extend([0, 0]);
    z
}

#[test]
fn a_crafted_zip64_gives_an_error_and_never_overflows() {
    let dir = place("zip64-crafted");
    let write = |name: &str, bytes: Vec<u8>| {
        let to = dir.join(name);
        fs::write(&to, bytes).unwrap();
        to
    };
    let sane = write("sane.zip", crafted_zip64(0, 0, 0, None));
    assert_eq!(list(&sane, None).unwrap().len(), 1, "the made-up zip is itself a good one");
    assert!(read(&sane, None, "a", 10).unwrap().is_empty());
    let far = write("locator.zip", crafted_zip64(0, 0, 0, Some(u64::MAX - 10)));
    assert!(list(&far, None).is_err(), "a zip64 record placed at the very end of the numbers");
    let moved = write("offset.zip", crafted_zip64(7, 0, u64::MAX - 3, None));
    assert!(list(&moved, None).is_err(), "a local record placed so that data in front of the zip carries it past the end");
    let long = write("packed.zip", crafted_zip64(0, u64::MAX - 5, 0, None));
    assert!(read(&long, None, "a", 10).is_err(), "an entry claiming more bytes than there are numbers");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn listings_look_like_folders() {
    let e = |name: &str, folder, size| Entry { name: name.into(), folder, size, modified: None, mode: None, locked: false };
    let all = [e("a/b/c.txt", false, 5), e("a/d.txt", false, 3), e("a", true, 0), e("top.txt", false, 1), e("a/b/e.txt", false, 2)];
    assert_eq!(inside(&all, ""), vec![e("a", true, 10), e("top.txt", false, 1)]);
    assert_eq!(inside(&all, "a/"), vec![e("a/b", true, 7), e("a/d.txt", false, 3)]);
    assert_eq!(inside(&all, "a/b"), vec![e("a/b/c.txt", false, 5), e("a/b/e.txt", false, 2)]);
    assert_eq!(stem(Path::new("/x/Photos.TAR.GZ")), "Photos");
    assert_eq!(stem(Path::new("notes.7z")), "notes");
    assert_eq!(Format::of(Path::new("a.tbz2")), Some(Format::TarBz2));
    assert_eq!(Format::of(Path::new(".zip")), None);
}

#[test]
fn dos_times_and_old_names() {
    for t in [315_532_800 + 86_400 * 400, 1_700_000_000, 1_790_924_588] {
        let (time, date) = to_dos(t);
        assert_eq!(from_dos(time, date), Some(t));
    }
    assert_eq!(to_dos(0), (0, (1 << 5) | 1));
    assert_eq!(days_from_civil(1970, 1, 1), 0);
    assert_eq!(days_from_civil(2000, 3, 1), 11_017);
    assert_eq!(CP437_HIGH.chars().count(), 128);
    assert_eq!(cp437(0x81), 'ü');
    assert_eq!(cp437(0xFF), '\u{a0}');
    assert!(tar_sum_holds(&{
        let mut h = tar::Header::new_old();
        h.set_path("x").unwrap();
        h.set_cksum();
        h.as_bytes().to_vec()
    }));
}
