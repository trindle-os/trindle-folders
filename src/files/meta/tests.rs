//! Tests of dates and flags, and of how a song or a photo is changed: by a

use std::fs;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::PathBuf;

use super::*;

pub(super) fn place(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-files-meta-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

pub(super) fn song() -> Vec<u8> {
    (0..5000u32).map(|i| (i * 7 % 251) as u8).chain([0xff, 0xfb, 0x90]).collect()
}

#[test]
fn a_song_or_a_photo_is_changed_by_a_new_copy_and_never_where_it_lies() {
    let dir = place("whole");
    let (p, kept) = (dir.join("song.mp3"), dir.join("kept"));
    fs::write(&p, song()).unwrap();
    fs::set_permissions(&p, fs::Permissions::from_mode(0o640)).unwrap();
    let first = Id3 { title: "First".into(), artist: "John Smith".into(), ..Id3::default() };
    write_id3(&p, &first).unwrap();
    let was = fs::read(&p).unwrap();
    fs::hard_link(&p, &kept).unwrap();
    let second = Id3 { title: "Second".into(), ..first.clone() };
    write_id3(&p, &second).unwrap();
    assert_eq!(fs::read(&kept).unwrap(), was, "the song was written into where it lay");
    assert_eq!(read_id3(&p).unwrap().as_ref(), Some(&second));
    assert_eq!(fs::metadata(&p).unwrap().len() as usize, was.len(), "a tag that fits keeps the song's length");
    assert!(fs::read(&p).unwrap().ends_with(&song()));
    assert_eq!(fs::metadata(&p).unwrap().permissions().mode() & 0o777, 0o640);
    fs::remove_file(&kept).unwrap();
    let whole = fs::read(&p).unwrap();
    crate::common::files::FULL.set(true);
    let full = write_id3(&p, &Id3 { title: "Third".into(), ..first.clone() });
    crate::common::files::FULL.set(false);
    assert!(full.is_err());
    assert_eq!(fs::read(&p).unwrap(), whole, "an edit cut short left the song changed");
    assert_eq!(fs::read_dir(&dir).unwrap().count(), 1, "half a copy was left behind");
    ROOM.set(Some(whole.len() as u64 - 1));
    let none = write_id3(&p, &Id3 { title: "Fourth".into(), ..first.clone() }).unwrap_err();
    ROOM.set(None);
    assert_eq!((none.kind(), none.to_string().as_str()), (io::ErrorKind::StorageFull, t!("folders-meta-no-room")));
    assert_eq!(fs::read(&p).unwrap(), whole);
    assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
    let mut old = song();
    let mut v1 = [0u8; 128];
    v1[..3].copy_from_slice(b"TAG");
    old.extend_from_slice(&v1);
    fs::write(&p, &old).unwrap();
    write_id3(&p, &second).unwrap();
    let now = fs::read(&p).unwrap();
    assert!(now[..now.len() - 128].ends_with(&song()) && &now[now.len() - 128..now.len() - 125] == b"TAG");
    assert_eq!(&now[now.len() - 125..now.len() - 119], b"Second");
    let (link, pipe) = (dir.join("link.mp3"), dir.join("pipe.mp3"));
    std::os::unix::fs::symlink(&p, &link).unwrap();
    let c = std::ffi::CString::new(pipe.to_str().unwrap()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
    let target = fs::read(&p).unwrap();
    let (tell, told) = std::sync::mpsc::channel();
    let (l, q) = (link.clone(), pipe.clone());
    std::thread::spawn(move || {
        let edit = Id3 { title: "Through".into(), ..Id3::default() };
        let _ = tell.send((write_id3(&l, &edit).is_err(), write_id3(&q, &edit).is_err(), write_exif(&l, &ExifEdit::default()).is_err(), write_exif(&q, &ExifEdit::default()).is_err()));
    });
    let got = told.recv_timeout(std::time::Duration::from_secs(5));
    let _ = fs::OpenOptions::new().write(true).custom_flags(libc::O_NONBLOCK).open(&pipe);
    assert_eq!(got, Ok((true, true, true, true)), "a pipe was waited on, or a link written through");
    assert_eq!(fs::read(&p).unwrap(), target, "a song was changed through a link to it");
    assert!(fs::symlink_metadata(&link).unwrap().file_type().is_symlink(), "the link was replaced by a file");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_song_longer_than_one_gentle_step_is_written_again_whole() {
    let dir = place("steps");
    let p = dir.join("long.mp3");
    let long: Vec<u8> = song().iter().copied().cycle().take(crate::common::gentle::STEP as usize * 2 + 4321).collect();
    fs::write(&p, &long).unwrap();
    let tag = Id3 { title: "A long one".into(), artist: "John Smith".into(), ..Id3::default() };
    write_id3(&p, &tag).unwrap();
    assert_eq!(read_id3(&p).unwrap().as_ref(), Some(&tag));
    let now = fs::read(&p).unwrap();
    assert!(now.len() > long.len() && now.ends_with(&long), "the song was not kept whole");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn dates_and_flags() {
    let dir = place("flags");
    let p = dir.join("f");
    fs::write(&p, b"x").unwrap();
    fs::set_permissions(&p, fs::Permissions::from_mode(0o644)).unwrap();
    let when = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
    set_dates(&p, Some(when), None).unwrap();
    let d = dates(&p).unwrap();
    assert_eq!(d.modified, Some(when));
    set_flags(&p, Flags { read_only: true, executable: true }).unwrap();
    assert_eq!(fs::metadata(&p).unwrap().permissions().mode() & 0o777, 0o555);
    assert_eq!(flags(&p).unwrap(), Flags { read_only: true, executable: true });
    set_flags(&p, Flags { read_only: false, executable: false }).unwrap();
    assert_eq!(fs::metadata(&p).unwrap().permissions().mode() & 0o777, 0o644);
    set_flags(&dir, Flags { read_only: false, executable: false }).unwrap();
    assert!(fs::metadata(&dir).unwrap().permissions().mode() & 0o100 != 0);
    let link = dir.join("link");
    std::os::unix::fs::symlink(&p, &link).unwrap();
    assert_eq!(set_dates(&link, Some(when), None).unwrap_err().kind(), io::ErrorKind::InvalidInput);
    assert_eq!(set_flags(&link, Flags::default()).unwrap_err().kind(), io::ErrorKind::InvalidInput);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn an_old_tags_cover_and_comment_go_across_when_it_is_written_again() {
    let dir = place("v22");
    let p = dir.join("old.mp3");
    let frame = |id: &[u8], data: &[u8]| {
        let n = data.len() as u32;
        let mut f = id.to_vec();
        f.extend_from_slice(&[(n >> 16) as u8, (n >> 8) as u8, n as u8]);
        f.extend_from_slice(data);
        f
    };
    let mut body = frame(b"TT2", b"\0Old title");
    body.extend(frame(b"COM", b"\0engsaid\0Recorded live"));
    body.extend(frame(b"PIC", b"\0JPG\x03cover\0\xff\xd8\xff\xe0picture"));
    let n = body.len() as u32;
    let mut file = b"ID3\x02\x00\x00".to_vec();
    file.extend_from_slice(&[(n >> 21 & 0x7f) as u8, (n >> 14 & 0x7f) as u8, (n >> 7 & 0x7f) as u8, (n & 0x7f) as u8]);
    file.extend(body);
    file.extend(song());
    fs::write(&p, &file).unwrap();
    write_id3(&p, &Id3 { title: "New title".into(), ..Id3::default() }).unwrap();
    let got = fs::read(&p).unwrap();
    let has = |b: &[u8]| got.windows(b.len()).any(|w| w == b);
    assert!(has(b"COMM") && has(b"Recorded live"), "the comment was dropped");
    assert!(has(b"APIC") && has(b"image/jpeg") && has(b"\xff\xd8\xff\xe0picture"), "the cover picture was dropped");
    assert!(has(b"New title") && !has(b"Old title"));
    fs::remove_dir_all(&dir).unwrap();
}
