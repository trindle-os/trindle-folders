//! A name taken by something else while an archive is read or made.

use super::*;

#[test]
fn an_archive_that_has_its_name_is_made_though_its_spare_name_cannot_be_removed() {
    use std::os::unix::fs::PermissionsExt;
    let dir = std::env::temp_dir().join(format!("trindle-files-archive-put-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("spare")).unwrap();
    fs::create_dir_all(dir.join("made")).unwrap();
    fs::write(dir.join("spare/part"), b"the whole archive").unwrap();
    fs::set_permissions(dir.join("spare"), fs::Permissions::from_mode(0o500)).unwrap();
    let cannot = fs::remove_file(dir.join("spare/part")).is_err();
    let got = if cannot { put_new(&dir.join("spare/part"), &dir.join("made/Photos.zip")) } else { Ok(()) };
    fs::set_permissions(dir.join("spare"), fs::Permissions::from_mode(0o700)).unwrap();
    assert!(cannot, "{}", crate::common::files::tests::NOT_AS_ROOT);
    assert_eq!(fs::read(dir.join("made/Photos.zip")).ok().as_deref(), Some(&b"the whole archive"[..]));
    assert!(got.is_ok(), "the archive is there whole, and was said to have failed: {got:?}");
    fs::write(dir.join("spare/other"), b"another").unwrap();
    let taken = put_new(&dir.join("spare/other"), &dir.join("made/Photos.zip")).expect_err("a taken name was given away");
    assert_eq!(taken.kind(), io::ErrorKind::AlreadyExists);
    assert_eq!(fs::read(dir.join("made/Photos.zip")).unwrap(), b"the whole archive");
    let _ = fs::remove_dir_all(&dir);
}

fn pipe_at(at: &Path) {
    let c = std::ffi::CString::new(at.as_os_str().as_bytes()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
}

fn in_time<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> Option<T> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || tx.send(work()));
    rx.recv_timeout(Duration::from_secs(10)).ok()
}

#[test]
fn a_pipe_named_as_an_archive_is_refused_and_never_waited_on() {
    let dir = std::env::temp_dir().join(format!("trindle-files-archive-pipe-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("out")).unwrap();
    for name in ["backup.zip", "backup.tar.gz", "backup.7z", "backup.gz", "backup.rar", "backup"] {
        let (pipe, out) = (dir.join(name), dir.join("out"));
        pipe_at(&pipe);
        let asked = pipe.clone();
        let got = in_time(move || {
            let stop = AtomicBool::new(false);
            [
                sniff(&asked).is_err(),
                find(&asked).is_err(),
                list(&asked, None).is_err(),
                read(&asked, None, "note.txt", 1 << 10).is_err(),
                extract(&asked, None, &out, Place::Here, &stop, &|_, _| {}).is_err(),
                take_out(&asked, None, &["note.txt".to_string()], &out, &stop, &|_, _| {}).is_err(),
            ]
        });
        assert_eq!(got, Some([true; 6]), "{name}: sniffed, found, listed, read, unpacked, taken from");
        assert_eq!(fs::read_dir(dir.join("out")).unwrap().count(), 0, "{name}: nothing is made for it");
    }
    let real = dir.join("real.zip");
    compress(&[dir.join("out")], &real, Format::Zip, &AtomicBool::new(false), &|_, _| {}).unwrap();
    symlink(&real, dir.join("link.zip")).unwrap();
    symlink(dir.join("backup.zip"), dir.join("to-pipe.zip")).unwrap();
    let (link, to_pipe) = (dir.join("link.zip"), dir.join("to-pipe.zip"));
    assert_eq!(in_time(move || (list(&link, None).map(|l| l.len()).ok(), list(&to_pipe, None).is_err())), Some((Some(1), true)));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn what_takes_a_files_name_while_it_waits_to_be_packed_is_not_packed() {
    let dir = std::env::temp_dir().join(format!("trindle-files-archive-swapped-{}", std::process::id()));
    for (format, pipe) in [(Format::Zip, false), (Format::Tar, false), (Format::Zip, true), (Format::TarGz, true)] {
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("pack")).unwrap();
        fs::write(dir.join("pack/notes.txt"), b"the notes").unwrap();
        fs::write(dir.join("private.txt"), b"a made-up key").unwrap();
        let (pack, to, notes, private) = (dir.join("pack"), dir.join("made"), dir.join("pack/notes.txt"), dir.join("private.txt"));
        let made = to.clone();
        let got = in_time(move || {
            let once = Cell::new(false);
            compress(&[pack], &to, format, &AtomicBool::new(false), &|_, _| {
                if !once.replace(true) {
                    fs::remove_file(&notes).unwrap();
                    if pipe { pipe_at(&notes) } else { symlink(&private, &notes).unwrap() }
                }
            })
        });
        let got = got.unwrap_or_else(|| panic!("{format:?}: the packing waited on a pipe"));
        assert!(got.is_err(), "{format:?}, pipe {pipe}: what took the name was packed in its place");
        assert!(fs::symlink_metadata(&made).is_err(), "{format:?}: a failed archive is not left under its name");
    }
    let _ = fs::remove_dir_all(&dir);
}
