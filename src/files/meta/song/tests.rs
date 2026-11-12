//! Tests of reading and writing ID3 tags.

use std::fs;
use std::os::unix::fs::PermissionsExt;

use super::super::tests::{place, song};
use super::*;

fn v2(major: u8, flags: u8, frames: &[u8]) -> Vec<u8> {
    let mut out = b"ID3".to_vec();
    out.extend_from_slice(&[major, 0, flags]);
    out.extend_from_slice(&syncsafe_bytes(frames.len() as u32));
    out.extend_from_slice(frames);
    out
}

fn frame(id: &str, size: [u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = id.as_bytes().to_vec();
    out.extend_from_slice(&size);
    out.extend_from_slice(&[0, 0]);
    out.extend_from_slice(data);
    out
}

#[test]
fn id3_writes_reads_and_keeps_the_song() {
    let dir = place("id3");
    let p = dir.join("song.mp3");
    fs::write(&p, song()).unwrap();
    fs::set_permissions(&p, fs::Permissions::from_mode(0o640)).unwrap();
    assert_eq!(read_id3(&p).unwrap(), None);
    let mut id3 = Id3 { title: "Zoë's Song".into(), artist: "John Smith 東京".into(), album: "Example".into(), year: "2026".into(), track: "3/12".into(), genre: "Rock".into() };
    write_id3(&p, &id3).unwrap();
    assert_eq!(read_id3(&p).unwrap().as_ref(), Some(&id3));
    let bytes = fs::read(&p).unwrap();
    assert_eq!(&bytes[..4], b"ID3\x03");
    assert!(bytes.ends_with(&song()));
    assert_eq!(fs::metadata(&p).unwrap().permissions().mode() & 0o777, 0o640);
    let len = bytes.len();
    id3.album.clear();
    id3.title = "Short".into();
    write_id3(&p, &id3).unwrap();
    assert_eq!(fs::metadata(&p).unwrap().len() as usize, len);
    assert_eq!(read_id3(&p).unwrap().as_ref(), Some(&id3));
    id3.title = "John Smith ".repeat(500).trim_end().to_string();
    write_id3(&p, &id3).unwrap();
    assert_eq!(read_id3(&p).unwrap().as_ref(), Some(&id3));
    assert!(fs::read(&p).unwrap().ends_with(&song()));
    assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn id3_reads_every_version() {
    let dir = place("versions");
    let p = dir.join("a.mp3");
    let mut frames = frame("TIT2", 300u32.to_be_bytes(), &[&[3u8][..], "Café".as_bytes(), &[b' '; 294]].concat());
    frames.extend(frame("TCON", syncsafe_bytes(5), b"\x00(17)"));
    frames.extend(frame("TDRC", syncsafe_bytes(11), b"\x002026-10-03"));
    frames.extend([0; 20]);
    fs::write(&p, [v2(4, 0, &frames), song()].concat()).unwrap();
    let got = read_id3(&p).unwrap().unwrap();
    assert_eq!((got.title.as_str(), got.genre.as_str(), got.year.as_str()), ("Café", "Rock", "2026"));
    let data = [1, 0xff, 0xfe, 0xff, 0x00, b'a', 0];
    let mut raw = frame("TPE1", 7u32.to_be_bytes(), &data);
    let mut sync = Vec::new();
    for b in raw.drain(..) {
        sync.push(b);
        if b == 0xff {
            sync.push(0);
        }
    }
    fs::write(&p, [v2(3, 0x80, &sync), song()].concat()).unwrap();
    assert_eq!(read_id3(&p).unwrap().unwrap().artist, "ÿa");
    let mut f22 = b"TT2\x00\x00\x06\x00Hello".to_vec();
    f22.extend(b"TCO\x00\x00\x05\x00(8)x");
    fs::write(&p, [v2(2, 0, &f22), song()].concat()).unwrap();
    let got = read_id3(&p).unwrap().unwrap();
    assert_eq!((got.title.as_str(), got.genre.as_str()), ("Hello", "x"));
    let mut v1 = [0u8; 128];
    v1[..3].copy_from_slice(b"TAG");
    v1[3..8].copy_from_slice(b"Title");
    v1[93..97].copy_from_slice(b"1999");
    v1[126] = 4;
    v1[127] = 8;
    fs::write(&p, [song(), v1.to_vec()].concat()).unwrap();
    let got = read_id3(&p).unwrap().unwrap();
    assert_eq!(got, Id3 { title: "Title".into(), year: "1999".into(), track: "4".into(), genre: "Jazz".into(), ..Id3::default() });
    write_id3(&p, &Id3 { title: "New".into(), track: "7".into(), genre: "Pop".into(), ..got }).unwrap();
    let bytes = fs::read(&p).unwrap();
    let tail = &bytes[bytes.len() - 128..];
    assert_eq!((&tail[3..6], tail[126], tail[127]), (&b"New"[..], 7, 13));
    assert_eq!(read_id3(&p).unwrap().unwrap().title, "New");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn id3_keeps_other_frames_and_drops_the_footer() {
    let dir = place("keep");
    let p = dir.join("b.mp3");
    let picture = [b"\x00image/png\x00\x03\x00".to_vec(), vec![0xff, 0x00, 0xe0, 0x12, 0xff]].concat();
    let mut frames = frame("APIC", syncsafe_bytes(picture.len() as u32), &picture);
    frames.extend(frame("TIT2", syncsafe_bytes(4), b"\x03Old"));
    let mut tag = v2(4, 0x10, &frames);
    let mut footer = tag[..10].to_vec();
    footer[..3].copy_from_slice(b"3DI");
    tag.extend(footer);
    fs::write(&p, [tag, song()].concat()).unwrap();
    let id3 = Id3 { title: "New".into(), ..Id3::default() };
    write_id3(&p, &id3).unwrap();
    let bytes = fs::read(&p).unwrap();
    assert_eq!(&bytes[..6], b"ID3\x04\x00\x00");
    assert!(bytes.windows(picture.len()).any(|w| w == picture));
    assert!(!bytes.windows(4).any(|w| w == b"3DI\x04"));
    assert!(bytes.ends_with(&song()));
    assert_eq!(read_id3(&p).unwrap(), Some(id3));
    fs::remove_dir_all(&dir).unwrap();
}
