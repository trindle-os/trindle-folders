// Tests of reading and writing EXIF notes.

use std::fs;
use std::os::unix::fs::PermissionsExt;

use super::super::tests::place;
use super::super::ROOM;
use super::*;

#[test]
fn a_changed_photo_needs_room_for_itself_and_not_for_its_old_notes_too() {
    let dir = place("exif-room");
    let p = dir.join("photo.jpg");
    let (old, ..) = jpeg(&full_tiff(Order { big: false }));
    fs::write(&p, &old).unwrap();
    let edit = ExifEdit { taken: None, description: None, artist: None, copyright: None, orientation: None, drop_location: true };
    ROOM.set(Some(old.len() as u64));
    let changed = write_exif(&p, &edit);
    ROOM.set(None);
    changed.expect("a change that had room was refused");
    assert!(fs::metadata(&p).unwrap().len() <= old.len() as u64);
    let was = fs::read(&p).unwrap();
    ROOM.set(Some(was.len() as u64 / 2));
    let none = write_exif(&p, &edit).unwrap_err();
    ROOM.set(None);
    assert_eq!(none.kind(), io::ErrorKind::StorageFull);
    assert_eq!(fs::read(&p).unwrap(), was);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn exif_is_read_from_a_jpeg() {
    let dir = place("exif");
    let p = dir.join("photo.jpg");
    let mut tiff = b"II*\x00\x08\x00\x00\x00\x01\x00".to_vec();
    tiff.extend([0x0f, 0x01, 2, 0, 8, 0, 0, 0, 26, 0, 0, 0]);
    tiff.extend([0, 0, 0, 0]);
    tiff.extend(b"Trindle\x00");
    let app1 = [b"Exif\x00\x00".to_vec(), tiff].concat();
    let mut jpeg = vec![0xff, 0xd8, 0xff, 0xe1];
    jpeg.extend(((app1.len() + 2) as u16).to_be_bytes());
    jpeg.extend(app1);
    jpeg.extend([0xff, 0xd9]);
    fs::write(&p, jpeg).unwrap();
    let tags = exif(&p);
    assert_eq!(tags, [Tag { name: "Make".into(), value: "\"Trindle\"".into() }]);
    fs::write(&p, b"not a picture").unwrap();
    assert!(exif(&p).is_empty());
    fs::remove_dir_all(&dir).unwrap();
}

type Block = (Vec<(u16, Val)>, Option<usize>);

enum Val {
    Raw(u16, u32, Vec<u8>),
    Block(usize),
    Tail,
}

fn ascii(s: &str) -> Val {
    let mut b = s.as_bytes().to_vec();
    b.push(0);
    Val::Raw(2, b.len() as u32, b)
}

fn short(o: Order, v: u16) -> Val {
    Val::Raw(3, 1, o.put16(v).to_vec())
}

fn rationals(o: Order, v: &[(u32, u32)]) -> Val {
    Val::Raw(5, v.len() as u32, v.iter().flat_map(|&(a, b)| [o.put32(a), o.put32(b)]).flatten().collect())
}

fn tiff(o: Order, blocks: &[Block], tail: &[u8]) -> Vec<u8> {
    let value_len = |v: &Val| match v {
        Val::Raw(_, _, d) if d.len() > 4 => d.len().next_multiple_of(2),
        _ => 0,
    };
    let mut starts = Vec::new();
    let mut at = 8;
    for (fields, _) in blocks {
        starts.push(at);
        at += 6 + 12 * fields.len() + fields.iter().map(|f| value_len(&f.1)).sum::<usize>();
    }
    let tail_at = at as u32;
    let mut out = if o.big { b"MM\0*".to_vec() } else { b"II*\0".to_vec() };
    out.extend(o.put32(8));
    for (i, (fields, next)) in blocks.iter().enumerate() {
        let mut values = Vec::new();
        let values_at = starts[i] + 6 + 12 * fields.len();
        out.extend(o.put16(fields.len() as u16));
        for (tag, v) in fields {
            out.extend(o.put16(*tag));
            let (kind, count, data) = match v {
                Val::Raw(k, c, d) => (*k, *c, d.clone()),
                Val::Block(b) => (4, 1, o.put32(starts[*b] as u32).to_vec()),
                Val::Tail => (4, 1, o.put32(tail_at).to_vec()),
            };
            out.extend(o.put16(kind));
            out.extend(o.put32(count));
            if data.len() <= 4 {
                let mut d = data;
                d.resize(4, 0);
                out.extend(d);
            } else {
                out.extend(o.put32((values_at + values.len()) as u32));
                values.extend(&data);
                values.resize(values.len().next_multiple_of(2), 0);
            }
        }
        out.extend(o.put32(next.map_or(0, |n| starts[n] as u32)));
        out.extend(values);
    }
    assert_eq!(out.len(), tail_at as usize);
    out.extend(tail);
    out
}

const THUMB: &[u8] = b"\xff\xd8 a made-up preview \xff\xd9";
const NOTE: &[u8] = b"Nikon\0\x02\x10\0\0 the camera's own notes, which point at themselves";

fn full_tiff(o: Order) -> Vec<u8> {
    let title: Vec<u8> = "Old title".encode_utf16().chain([0]).flat_map(u16::to_le_bytes).collect();
    let ifd0 = vec![
        (0x010f, ascii("Trindle")),
        (0x0110, ascii("Test Camera One")),
        (T_ORIENTATION, short(o, 6)),
        (T_ARTIST, ascii("Old Artist")),
        (T_EXIF, Val::Block(1)),
        (T_GPS, Val::Block(3)),
        (T_XP_TITLE, Val::Raw(1, title.len() as u32, title)),
        (0xc4a5, Val::Raw(7, 9, b"PrintIM01".to_vec())),
    ];
    let exif = vec![
        (0x829a, rationals(o, &[(1, 250)])),
        (T_VERSION, Val::Raw(7, 4, b"0231".to_vec())),
        (T_TAKEN, ascii("2025:12:31 23:59:58")),
        (T_MAKER, Val::Raw(7, NOTE.len() as u32, NOTE.to_vec())),
        (T_INTEROP, Val::Block(2)),
    ];
    let interop = vec![(0x0001, ascii("R98"))];
    let gps = vec![
        (0x0000, Val::Raw(1, 4, vec![2, 3, 0, 0])),
        (0x0001, ascii("N")),
        (0x0002, rationals(o, &[(52, 1), (20, 1), (3015, 100)])),
    ];
    let ifd1 = vec![(0x0103, short(o, 6)), (T_THUMB, Val::Tail), (T_THUMB_LEN, Val::Raw(4, 1, o.put32(THUMB.len() as u32).to_vec()))];
    tiff(o, &[(ifd0, Some(4)), (exif, None), (interop, None), (gps, None), (ifd1, None)], THUMB)
}

fn jpeg_part(marker: u8, body: &[u8]) -> Vec<u8> {
    let mut out = vec![0xff, marker];
    out.extend(((body.len() + 2) as u16).to_be_bytes());
    out.extend(body);
    out
}

fn jpeg(tiff: &[u8]) -> (Vec<u8>, usize, usize) {
    let mut out = vec![0xff, 0xd8];
    out.extend(jpeg_part(0xe0, b"JFIF\0\x01\x02\0\0\x01\0\x01\0\0"));
    let from = out.len();
    out.extend(jpeg_part(0xe1, &[b"Exif\0\0".as_slice(), tiff].concat()));
    let to = out.len();
    out.extend(jpeg_part(0xe1, b"http://ns.adobe.com/xap/1.0/\0<x:xmpmeta/>"));
    out.extend(jpeg_part(0xdb, &[0; 65]));
    out.extend(jpeg_part(0xda, &[1, 1, 0, 0, 0x3f, 0]));
    out.extend((0..300u32).map(|i| (i * 13 % 255) as u8));
    out.extend([0xff, 0xd9]);
    (out, from, to)
}

fn read_back(p: &Path) -> exif::Exif {
    exif::Reader::new().read_from_container(&mut BufReader::new(File::open(p).unwrap())).unwrap()
}

fn text(e: &exif::Exif, tag: exif::Tag) -> Option<String> {
    match &e.get_field(tag, exif::In::PRIMARY)?.value {
        exif::Value::Ascii(v) => Some(String::from_utf8_lossy(&v[0]).into_owned()),
        other => panic!("{tag} is {other:?}"),
    }
}

fn number(e: &exif::Exif, tag: exif::Tag, ifd: exif::In) -> Option<u32> {
    e.get_field(tag, ifd)?.value.get_uint(0)
}

fn maker(e: &exif::Exif) -> (Vec<u8>, u32) {
    match &e.get_field(exif::Tag::MakerNote, exif::In::PRIMARY).unwrap().value {
        exif::Value::Undefined(b, at) => (b.clone(), *at),
        other => panic!("maker notes are {other:?}"),
    }
}

fn preview(e: &exif::Exif) -> Vec<u8> {
    let at = number(e, exif::Tag::JPEGInterchangeFormat, exif::In::THUMBNAIL).unwrap() as usize;
    let len = number(e, exif::Tag::JPEGInterchangeFormatLength, exif::In::THUMBNAIL).unwrap() as usize;
    e.buf()[at..at + len].to_vec()
}

fn xp_title(e: &exif::Exif) -> Option<String> {
    let f = e.fields().find(|f| f.tag.number() == T_XP_TITLE && f.ifd_num == exif::In::PRIMARY)?;
    let exif::Value::Byte(b) = &f.value else { panic!("the title is {:?}", f.value) };
    let units: Vec<u16> = b.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&u| u != 0).collect();
    Some(String::from_utf16(&units).unwrap())
}

fn names(dir: &Path) -> Vec<String> {
    let mut out: Vec<String> = fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
    out.sort();
    out
}

#[test]
fn exif_is_written_into_a_jpeg_keeping_the_rest() {
    let dir = place("exif-write");
    let p = dir.join("photo.jpg");
    let o = Order { big: false };
    let (old, from, to) = jpeg(&full_tiff(o));
    fs::write(&p, &old).unwrap();
    fs::set_permissions(&p, fs::Permissions::from_mode(0o640)).unwrap();
    let before = read_back(&p);
    let edit = ExifEdit {
        taken: Some("2026-01-02T03:04:05".into()),
        description: Some("A walk by the river, café".into()),
        artist: Some("John Smith".into()),
        copyright: Some("Copyright John Smith".into()),
        orientation: Some(1),
        drop_location: true,
    };
    write_exif(&p, &edit).unwrap();
    let new = fs::read(&p).unwrap();
    let after = read_back(&p);

    the_edited_notes_are_written(&before, &after);
    every_other_note_is_as_it_was(&before, &after);

    assert_eq!(new[..from], old[..from]);
    assert_eq!(&new[from..from + 2], &[0xff, 0xe1]);
    let end = from + 2 + usize::from(u16::from_be_bytes([new[from + 2], new[from + 3]]));
    assert_eq!(new[end..], old[to..]);
    assert_eq!(fs::metadata(&p).unwrap().permissions().mode() & 0o777, 0o640);
    assert_eq!(names(&dir), ["photo.jpg"]);

    write_exif(&p, &ExifEdit { description: Some(String::new()), artist: Some(String::new()), ..Default::default() }).unwrap();
    let again = read_back(&p);
    assert_eq!(text(&again, exif::Tag::ImageDescription), None);
    assert_eq!(text(&again, exif::Tag::Artist), None);
    assert_eq!(xp_title(&again), None);
    assert_eq!(text(&again, exif::Tag::Copyright).as_deref(), Some("Copyright John Smith"));
    assert_eq!(text(&again, exif::Tag::DateTimeOriginal).as_deref(), Some("2026:01:02 03:04:05"));
    assert_eq!(maker(&again), maker(&before));
    assert_eq!(preview(&again), THUMB);
    fs::remove_dir_all(&dir).unwrap();
}

fn the_edited_notes_are_written(before: &exif::Exif, after: &exif::Exif) {
    assert_eq!(text(after, exif::Tag::DateTimeOriginal).as_deref(), Some("2026:01:02 03:04:05"));
    assert_eq!(text(after, exif::Tag::ImageDescription).as_deref(), Some("A walk by the river, café"));
    assert_eq!(text(after, exif::Tag::Artist).as_deref(), Some("John Smith"));
    assert_eq!(text(after, exif::Tag::Copyright).as_deref(), Some("Copyright John Smith"));
    assert_eq!(number(after, exif::Tag::Orientation, exif::In::PRIMARY), Some(1));
    assert_eq!(xp_title(before).as_deref(), Some("Old title"));
    assert_eq!(xp_title(after).as_deref(), Some("A walk by the river, café"));
    assert!(after.fields().all(|f| f.tag.context() != exif::Context::Gps));
    assert!(before.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY).is_some());
}

fn every_other_note_is_as_it_was(before: &exif::Exif, after: &exif::Exif) {
    assert_eq!(text(after, exif::Tag::Make).as_deref(), Some("Trindle"));
    assert_eq!(text(after, exif::Tag::Model).as_deref(), Some("Test Camera One"));
    assert_eq!(text(after, exif::Tag::InteroperabilityIndex).as_deref(), Some("R98"));
    assert_eq!(number(after, exif::Tag::Compression, exif::In::THUMBNAIL), Some(6));
    let exposure = |e: &exif::Exif| e.get_field(exif::Tag::ExposureTime, exif::In::PRIMARY).unwrap().display_value().to_string();
    assert_eq!(exposure(after), "1/250");
    assert_eq!(exposure(before), exposure(after));
    let unknown = |e: &exif::Exif| match e.fields().find(|f| f.tag.number() == 0xc4a5).map(|f| f.value.clone()) {
        Some(exif::Value::Undefined(b, _)) => b,
        other => panic!("the unknown tag is {other:?}"),
    };
    assert_eq!(unknown(after), b"PrintIM01");
    assert_eq!(unknown(before), unknown(after));
    assert_eq!(maker(after), maker(before));
    assert_eq!(&after.buf()[maker(after).1 as usize..][..NOTE.len()], NOTE);
    assert_eq!(preview(after), THUMB);
}

#[test]
fn the_camera_column_names_the_maker_once() {
    let dir = place("exif-camera");
    let p = dir.join("photo.jpg");
    fs::write(&p, jpeg(&full_tiff(Order { big: false })).0).unwrap();
    assert_eq!(crate::files::media::read(&p).camera.as_deref(), Some("Trindle Test Camera One"));
}

#[test]
fn exif_written_big_endian_stays_big_endian() {
    let dir = place("exif-big");
    let p = dir.join("photo.jpeg");
    let o = Order { big: true };
    fs::write(&p, jpeg(&full_tiff(o)).0).unwrap();
    let before = read_back(&p);
    write_exif(&p, &ExifEdit { artist: Some("John Smith".into()), taken: Some("2026:03:04 05:06:07".into()), ..Default::default() }).unwrap();
    let after = read_back(&p);
    assert!(!after.little_endian());
    assert_eq!(text(&after, exif::Tag::Artist).as_deref(), Some("John Smith"));
    assert_eq!(text(&after, exif::Tag::DateTimeOriginal).as_deref(), Some("2026:03:04 05:06:07"));
    assert_eq!(number(&after, exif::Tag::Orientation, exif::In::PRIMARY), Some(6));
    let place_of = |e: &exif::Exif| e.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY).unwrap().display_value().to_string();
    assert_eq!(place_of(&after), place_of(&before));
    assert_eq!(maker(&after), maker(&before));
    assert_eq!(preview(&after), THUMB);
    assert_eq!(xp_title(&after).as_deref(), Some("Old title"));
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn exif_is_added_to_photos_that_have_none() {
    let dir = place("exif-new");
    let pic = image::RgbImage::from_fn(8, 8, |x, y| image::Rgb([(x * 30) as u8, (y * 30) as u8, 90]));

    let p = dir.join("plain.jpg");
    pic.save(&p).unwrap();
    let old = fs::read(&p).unwrap();
    let app0 = 4 + usize::from(u16::from_be_bytes([old[4], old[5]]));
    assert_eq!(old[3], 0xe0);
    write_exif(&p, &ExifEdit { taken: Some("2026:01:02 03:04:05".into()), artist: Some("John Smith".into()), ..Default::default() }).unwrap();
    let new = fs::read(&p).unwrap();
    assert_eq!(new[..app0], old[..app0]);
    assert_eq!(&new[app0..app0 + 2], &[0xff, 0xe1]);
    let end = app0 + 2 + usize::from(u16::from_be_bytes([new[app0 + 2], new[app0 + 3]]));
    assert_eq!(new[end..], old[app0..]);
    let e = read_back(&p);
    assert_eq!(text(&e, exif::Tag::DateTimeOriginal).as_deref(), Some("2026:01:02 03:04:05"));
    assert_eq!(text(&e, exif::Tag::Artist).as_deref(), Some("John Smith"));
    assert_eq!(e.get_field(exif::Tag::ExifVersion, exif::In::PRIMARY).unwrap().display_value().to_string(), "2.32");
    assert_eq!(image::open(&p).unwrap().to_rgb8().dimensions(), (8, 8));

    let bare = [&[0xff, 0xd8][..], &old[app0..]].concat();
    fs::write(&p, &bare).unwrap();
    write_exif(&p, &ExifEdit { copyright: Some("Copyright John Smith".into()), ..Default::default() }).unwrap();
    let new = fs::read(&p).unwrap();
    assert_eq!(&new[..4], &[0xff, 0xd8, 0xff, 0xe1]);
    assert_eq!(text(&read_back(&p), exif::Tag::Copyright).as_deref(), Some("Copyright John Smith"));
    assert!(read_back(&p).get_field(exif::Tag::ExifVersion, exif::In::PRIMARY).is_none());
    assert!(image::open(&p).is_ok());

    let p = dir.join("plain.png");
    pic.save(&p).unwrap();
    write_exif(&p, &ExifEdit { description: Some("Made-up squares".into()), orientation: Some(3), ..Default::default() }).unwrap();
    write_exif(&p, &ExifEdit { artist: Some("John Smith".into()), ..Default::default() }).unwrap();
    let e = read_back(&p);
    assert_eq!(text(&e, exif::Tag::ImageDescription).as_deref(), Some("Made-up squares"));
    assert_eq!(text(&e, exif::Tag::Artist).as_deref(), Some("John Smith"));
    assert_eq!(number(&e, exif::Tag::Orientation, exif::In::PRIMARY), Some(3));
    let png = fs::read(&p).unwrap();
    let mut at = 8;
    let mut kinds = Vec::new();
    while at < png.len() {
        let len = u32::from_be_bytes(png[at..at + 4].try_into().unwrap()) as usize;
        let mut crc = flate2::Crc::new();
        crc.update(&png[at + 4..at + 8 + len]);
        assert_eq!(crc.sum().to_be_bytes(), png[at + 8 + len..at + 12 + len]);
        kinds.push(String::from_utf8_lossy(&png[at + 4..at + 8]).into_owned());
        at += 12 + len;
    }
    assert_eq!(kinds.iter().filter(|k| *k == "eXIf").count(), 1);
    assert!(kinds.iter().position(|k| k == "eXIf") < kinds.iter().position(|k| k == "IDAT"));
    assert!(image::open(&p).is_ok());
    assert_eq!(names(&dir), ["plain.jpg", "plain.png"]);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn exif_refuses_what_it_cannot_write() {
    let dir = place("exif-refuse");
    let p = dir.join("photo.jpg");
    let (old, _, _) = jpeg(&full_tiff(Order { big: false }));
    fs::write(&p, &old).unwrap();
    let kind = |e: &ExifEdit| write_exif(&p, e).unwrap_err().kind();
    assert_eq!(kind(&ExifEdit { orientation: Some(9), ..Default::default() }), io::ErrorKind::InvalidInput);
    assert_eq!(kind(&ExifEdit { orientation: Some(0), ..Default::default() }), io::ErrorKind::InvalidInput);
    assert_eq!(kind(&ExifEdit { taken: Some("yesterday".into()), ..Default::default() }), io::ErrorKind::InvalidInput);
    assert_eq!(kind(&ExifEdit { taken: Some("2026/01/02 03:04:05".into()), ..Default::default() }), io::ErrorKind::InvalidInput);
    assert_eq!(kind(&ExifEdit { artist: Some("John\0Smith".into()), ..Default::default() }), io::ErrorKind::InvalidInput);
    assert_eq!(kind(&ExifEdit { artist: Some("x".repeat(70_000)), ..Default::default() }), io::ErrorKind::InvalidInput);
    assert_eq!(fs::read(&p).unwrap(), old);

    let mut bad = full_tiff(Order { big: false });
    let at = bad.windows(2).position(|w| w == T_EXIF.to_le_bytes()).unwrap();
    bad[at + 8..at + 12].copy_from_slice(&60_000u32.to_le_bytes());
    let (broken, _, _) = jpeg(&bad);
    fs::write(&p, &broken).unwrap();
    assert_eq!(kind(&ExifEdit::default()), io::ErrorKind::InvalidData);
    assert_eq!(fs::read(&p).unwrap(), broken);

    let t = dir.join("notes.txt");
    fs::write(&t, b"not a photo").unwrap();
    assert_eq!(write_exif(&t, &ExifEdit::default()).unwrap_err().kind(), io::ErrorKind::Unsupported);
    assert_eq!(names(&dir), ["notes.txt", "photo.jpg"]);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn exif_damage_is_an_error_not_a_crash() {
    let dir = place("exif-damage");
    let p = dir.join("photo.jpg");
    let (old, from, to) = jpeg(&full_tiff(Order { big: true }));
    let edit = ExifEdit { artist: Some("John Smith".into()), taken: Some("2026:01:02 03:04:05".into()), drop_location: true, ..Default::default() };
    for i in (0..to).filter(|i| *i < 24 || *i >= from) {
        let mut b = old.clone();
        b[i] ^= 0x5a;
        fs::write(&p, &b).unwrap();
        if write_exif(&p, &edit).is_ok() {
            let _ = exif::Reader::new().read_from_container(&mut BufReader::new(File::open(&p).unwrap()));
        }
    }
    for cut in (0..to).step_by(3) {
        fs::write(&p, &old[..cut]).unwrap();
        let _ = write_exif(&p, &edit);
    }
    assert_eq!(names(&dir), ["photo.jpg"]);
    fs::remove_dir_all(&dir).unwrap();
}
