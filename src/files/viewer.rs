// What the built-in viewer shows a file as, and the file read ready for it.

use std::io::{self, Read};
use std::path::Path;

use encoding_rs::{Encoding, UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1252};
use image::DynamicImage;

use crate::common::files::open_file;
use crate::toolkit::picture;
use crate::toolkit::t;

pub const PEEK: usize = 8192;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Picture,
    Text,
    Other,
}

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub text: u64,
    pub picture: u64,
    pub pixels: u64,
}

impl Default for Limits {
    fn default() -> Limits {
        Limits { text: 4 << 20, picture: 100 << 20, pixels: 120_000_000 }
    }
}

pub enum Shown {
    Picture(DynamicImage),
    Text {
        text: String,
        encoding: &'static str,
        cut: bool,
        read: u64,
    },
    Broken(&'static str),
    TooLarge { bytes: u64, pixels: Option<u64> },
    Pdf { first: DynamicImage, pages: Option<u32> },
    Media { info: super::walled::Probe, cover: Option<DynamicImage> },
    Unplayable(&'static str),
    Font { name: String, page: DynamicImage },
    Other,
    Unread(String),
}

pub fn kind(path: &Path) -> Kind {
    let mut head = Vec::with_capacity(PEEK);
    match open_file(path) {
        Ok(f) => {
            if f.take(PEEK as u64).read_to_end(&mut head).is_err() {
                return Kind::Other;
            }
        }
        Err(_) => return Kind::Other,
    }
    kind_of(path, &head)
}

pub fn kind_of(path: &Path, head: &[u8]) -> Kind {
    if image::guess_format(head).is_ok() || is_svg(path) {
        Kind::Picture
    } else if is_text(head) {
        Kind::Text
    } else {
        Kind::Other
    }
}

pub fn is_svg(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("svg"))
}

pub fn is_text(head: &[u8]) -> bool {
    if Encoding::for_bom(head).is_some() || utf16(head).is_some() {
        return true;
    }
    if head.contains(&0) {
        return false;
    }
    let odd = head.iter().filter(|&&b| b < 0x20 && !b"\t\n\r\x0c\x1b".contains(&b)).count();
    odd * 100 <= head.len()
}

pub fn load(path: &Path, limits: &Limits) -> Shown {
    let size = match std::fs::metadata(path) {
        Ok(m) if m.is_dir() => return Shown::Other,
        Ok(m) => m.len(),
        Err(e) => return Shown::Unread(words(&e)),
    };
    let mut head = Vec::with_capacity(PEEK);
    if let Err(e) = open_file(path).and_then(|f| f.take(PEEK as u64).read_to_end(&mut head)) {
        return Shown::Unread(words(&e));
    }
    if super::fonts::is_font(path) {
        return match super::fonts::sample(path) {
            Ok((name, page)) => Shown::Font { name, page },
            Err(why) => Shown::Unread(why),
        };
    }
    if let Some(shown) = played(path, &head) {
        return shown;
    }
    let kind = kind_of(path, &head);
    if kind != Kind::Picture {
        match named(path, &head) {
            Some((what, false)) => return Shown::Broken(what),
            Some((_, true)) => return Shown::Other,
            None => {}
        }
    }
    match kind {
        Kind::Picture => picture_of(path, size, limits),
        Kind::Text => match read_text(path, limits.text) {
            Ok((text, encoding, cut, read)) => Shown::Text { text, encoding, cut, read },
            Err(e) => Shown::Unread(words(&e)),
        },
        Kind::Other => Shown::Other,
    }
}

pub const PDF_MOST: u32 = 1400;

fn played(path: &Path, head: &[u8]) -> Option<Shown> {
    let end = path.extension()?.to_str()?.to_ascii_lowercase();
    match end.as_str() {
        e if is_media(e) => Some(media(path)),
        "pdf" if head.windows(5).take(1024).any(|w| w == b"%PDF-") && super::walled::can_pdf() => {
            let first = super::walled::pdf_page(path, 1, PDF_MOST)?;
            Some(Shown::Pdf { first, pages: super::media::read(path).pages })
        }
        _ => None,
    }
}

const SOUNDS: [&str; 9] = ["mp3", "flac", "ogg", "oga", "opus", "wav", "m4a", "aac", "wma"];
const VIDEOS: [&str; 10] = ["mp4", "m4v", "mkv", "webm", "mov", "avi", "ogv", "mpg", "mpeg", "wmv"];

fn is_media(end: &str) -> bool {
    SOUNDS.contains(&end) || VIDEOS.contains(&end)
}

pub const COVER_MOST: u32 = 600;

fn media(path: &Path) -> Shown {
    if !super::walled::can_play() {
        return Shown::Unplayable(t!("folders-viewer-no-ffmpeg"));
    }
    match super::walled::probe(path) {
        Some(info) => {
            let cover = if info.cover && info.video.is_none() { super::walled::video(path, COVER_MOST) } else { None };
            Shown::Media { info, cover }
        }
        None => Shown::Unplayable(t!("folders-viewer-unplayable")),
    }
}

fn named(path: &Path, h: &[u8]) -> Option<(&'static str, bool)> {
    let end = path.extension()?.to_str()?.to_ascii_lowercase();
    let riff = |kind: &[u8]| h.starts_with(b"RIFF") && h.get(8..12) == Some(kind);
    Some(match end.as_str() {
        "pdf" => (t!("folders-viewer-kind-pdf"), h.windows(5).take(1024).any(|w| w == b"%PDF-")),
        "png" => (t!("folders-viewer-kind-png"), h.starts_with(b"\x89PNG")),
        "jpg" | "jpeg" => (t!("folders-viewer-kind-jpeg"), h.starts_with(&[0xff, 0xd8, 0xff])),
        "gif" => (t!("folders-viewer-kind-gif"), h.starts_with(b"GIF8")),
        "webp" => (t!("folders-viewer-kind-webp"), riff(b"WEBP")),
        "zip" | "docx" | "xlsx" | "pptx" | "odt" | "ods" | "odp" | "epub" | "jar" | "apk" => (t!("folders-viewer-kind-zip"), h.starts_with(b"PK")),
        "gz" | "tgz" => (t!("folders-viewer-kind-gzip"), h.starts_with(&[0x1f, 0x8b])),
        "xz" | "txz" => (t!("folders-viewer-kind-xz"), h.starts_with(&[0xfd, b'7', b'z', b'X', b'Z', 0])),
        "zst" => (t!("folders-viewer-kind-zstd"), h.starts_with(&[0x28, 0xb5, 0x2f, 0xfd])),
        "7z" => (t!("folders-viewer-kind-7z"), h.starts_with(&[b'7', b'z', 0xbc, 0xaf, 0x27, 0x1c])),
        "rar" => (t!("folders-viewer-kind-rar"), h.starts_with(b"Rar!\x1a\x07")),
        "bz2" | "tbz2" | "tbz" => (t!("folders-viewer-kind-bzip2"), h.starts_with(b"BZh")),
        "cab" => (t!("folders-viewer-kind-cab"), h.starts_with(b"MSCF")),
        "deb" => (t!("folders-viewer-kind-deb"), h.starts_with(b"!<arch>\n")),
        "rpm" => (t!("folders-viewer-kind-rpm"), h.starts_with(&[0xed, 0xab, 0xee, 0xdb])),
        "mp4" | "m4a" | "m4v" | "mov" => (t!("folders-viewer-kind-media"), h.get(4..8) == Some(b"ftyp")),
        "mkv" | "webm" => (t!("folders-viewer-kind-video"), h.starts_with(&[0x1a, 0x45, 0xdf, 0xa3])),
        "ogg" | "oga" | "ogv" | "opus" => (t!("folders-viewer-kind-ogg"), h.starts_with(b"OggS")),
        "flac" => (t!("folders-viewer-kind-flac"), h.starts_with(b"fLaC")),
        "wav" => (t!("folders-viewer-kind-wav"), riff(b"WAVE")),
        "mp3" => (t!("folders-viewer-kind-mp3"), h.starts_with(b"ID3") || (h.len() > 1 && h[0] == 0xff && h[1] & 0xe0 == 0xe0)),
        _ => return None,
    })
}

fn picture_of(path: &Path, size: u64, limits: &Limits) -> Shown {
    if size > limits.picture {
        return Shown::TooLarge { bytes: size, pixels: None };
    }
    if is_svg(path) {
        return match svg(path, 2048) {
            Some(img) => Shown::Picture(img),
            None => Shown::Unread(t!("folders-viewer-picture-unread").to_string()),
        };
    }
    let dims = picture::reader(path).ok().and_then(|r| r.into_dimensions().ok());
    let Some((w, h)) = dims else { return Shown::Unread(t!("folders-viewer-picture-unread").to_string()) };
    let pixels = w as u64 * h as u64;
    if pixels > limits.pixels {
        return Shown::TooLarge { bytes: size, pixels: Some(pixels) };
    }
    match picture::load(path) {
        Some(img) => Shown::Picture(img),
        None => Shown::Unread(t!("folders-viewer-picture-unread").to_string()),
    }
}

pub fn svg(path: &Path, most: u32) -> Option<DynamicImage> {
    let file = picture::file(path).ok()?;
    if file.metadata().ok()?.len() > picture::SVG_MOST {
        return None;
    }
    super::walled::svg(path, most)
}

pub fn read_text(path: &Path, most: u64) -> io::Result<(String, &'static str, bool, u64)> {
    let mut bytes = Vec::new();
    open_file(path)?.take(most + 1).read_to_end(&mut bytes)?;
    let cut = bytes.len() as u64 > most;
    bytes.truncate(most as usize);
    if cut {
        trim_partial(&mut bytes);
    }
    let (text, enc) = decode(&bytes);
    if cut && (enc == UTF_16LE || enc == UTF_16BE) && bytes.len() % 2 == 1 {
        bytes.pop();
    }
    Ok((text, enc.name(), cut, bytes.len() as u64))
}

pub fn read_more(path: &Path, from: u64, most: u64, encoding: &str) -> io::Result<(String, u64, bool)> {
    use std::io::{Seek, SeekFrom};
    let mut file = open_file(path)?;
    file.seek(SeekFrom::Start(from))?;
    let mut bytes = Vec::new();
    file.take(most + 1).read_to_end(&mut bytes)?;
    let cut = bytes.len() as u64 > most;
    bytes.truncate(most as usize);
    let enc = Encoding::for_label(encoding.as_bytes()).unwrap_or(UTF_8);
    if enc == UTF_16LE || enc == UTF_16BE {
        bytes.truncate(bytes.len() & !1);
    } else if cut && enc == UTF_8 {
        trim_partial(&mut bytes);
    }
    let text = enc.decode_without_bom_handling(&bytes).0.into_owned();
    Ok((text, bytes.len() as u64, cut))
}

fn trim_partial(bytes: &mut Vec<u8>) {
    if let Err(e) = std::str::from_utf8(bytes) {
        if e.error_len().is_none() {
            bytes.truncate(e.valid_up_to());
        }
    }
}

pub fn decode(bytes: &[u8]) -> (String, &'static Encoding) {
    if let Some((enc, skip)) = Encoding::for_bom(bytes) {
        return (enc.decode_without_bom_handling(&bytes[skip..]).0.into_owned(), enc);
    }
    if let Some(enc) = utf16(bytes) {
        return (enc.decode_without_bom_handling(bytes).0.into_owned(), enc);
    }
    match std::str::from_utf8(bytes) {
        Ok(s) => (s.to_string(), UTF_8),
        Err(_) => (WINDOWS_1252.decode_without_bom_handling(bytes).0.into_owned(), WINDOWS_1252),
    }
}

fn utf16(bytes: &[u8]) -> Option<&'static Encoding> {
    if bytes.len() < 4 {
        return None;
    }
    let pairs = bytes.len() / 2;
    let even = bytes.iter().step_by(2).filter(|&&b| b == 0).count();
    let odd = bytes.iter().skip(1).step_by(2).filter(|&&b| b == 0).count();
    if odd * 10 >= pairs * 9 && even == 0 {
        Some(UTF_16LE)
    } else if even * 10 >= pairs * 9 && odd == 0 {
        Some(UTF_16BE)
    } else {
        None
    }
}

fn words(e: &io::Error) -> String {
    match e.kind() {
        io::ErrorKind::NotFound => t!("folders-viewer-gone").to_string(),
        io::ErrorKind::PermissionDenied => t!("folders-viewer-denied").to_string(),
        _ => t!("folders-viewer-read-failed", why = e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-viewer-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn files_are_told_apart_and_read() {
        let dir = place("load");
        let png = dir.join("dot.png");
        image::RgbImage::from_pixel(3, 2, image::Rgb([200, 10, 10])).save(&png).unwrap();
        pictures_are_read_and_svgs_drawn(&dir, &png);
        text_is_cut_whole_and_read_on(&dir);
        other_encodings_and_what_is_not_text(&dir);
        broken_files_are_said_to_be(&dir, &png);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    fn pictures_are_read_and_svgs_drawn(dir: &Path, png: &Path) {
        let renamed = dir.join("dot.txt");
        std::fs::copy(png, &renamed).unwrap();
        assert_eq!(kind(&renamed), Kind::Picture);
        match load(png, &Limits::default()) {
            Shown::Picture(img) => assert_eq!((img.width(), img.height()), (3, 2)),
            _ => panic!("not a picture"),
        }
        let small = Limits { pixels: 5, ..Limits::default() };
        assert!(matches!(load(png, &small), Shown::TooLarge { pixels: Some(6), .. }));

        crate::common::needs::program("bwrap");
        let svg = dir.join("box.svg");
        std::fs::write(&svg, r#"<svg xmlns="http://www.w3.org/2000/svg" width="40" height="20"><rect width="40" height="20" fill="blue"/></svg>"#).unwrap();
        match load(&svg, &Limits::default()) {
            Shown::Picture(img) => assert_eq!((img.width(), img.height()), (40, 20)),
            _ => panic!("not a picture"),
        }
    }

    fn text_is_cut_whole_and_read_on(dir: &Path) {
        let note = dir.join("note");
        std::fs::write(&note, "Caf\u{e9} au lait\n".repeat(10)).unwrap();
        let limits = Limits { text: 18, ..Limits::default() };
        match load(&note, &limits) {
            Shown::Text { text, encoding, cut, read } => {
                assert_eq!(encoding, "UTF-8");
                assert!(cut);
                assert_eq!(text, "Caf\u{e9} au lait\nCaf");
                assert_eq!(read, 17);
                let (more, used, more_still) = read_more(&note, read, 18, encoding).unwrap();
                assert_eq!(more, "\u{e9} au lait\nCaf\u{e9} a");
                assert!(more_still);
                assert_eq!(used, 18);
                let (_, _, more_still) = read_more(&note, 140, 18, encoding).unwrap();
                assert!(!more_still);
            }
            _ => panic!("not text"),
        }
    }

    fn other_encodings_and_what_is_not_text(dir: &Path) {
        let old = dir.join("old.txt");
        std::fs::write(&old, b"Caf\xe9\n").unwrap();
        assert!(matches!(load(&old, &Limits::default()), Shown::Text { ref text, encoding: "windows-1252", cut: false, .. } if text == "Caf\u{e9}\n"));
        let wide = dir.join("wide.txt");
        std::fs::write(&wide, b"h\0e\0l\0l\0o\0").unwrap();
        assert!(matches!(load(&wide, &Limits::default()), Shown::Text { ref text, encoding: "UTF-16LE", .. } if text == "hello"));

        let bin = dir.join("bin");
        std::fs::write(&bin, [0u8, 1, 2, 3, 0, 0, 7, 9, 200]).unwrap();
        assert!(matches!(load(&bin, &Limits::default()), Shown::Other));
        assert!(matches!(load(&dir.join("gone"), &Limits::default()), Shown::Unread(_)));
    }

    fn broken_files_are_said_to_be(dir: &Path, png: &Path) {
        let pdf = dir.join("tiny.pdf");
        std::fs::write(&pdf, "oops").unwrap();
        assert!(matches!(load(&pdf, &Limits::default()), Shown::Broken(w) if w == "PDF document"));
        std::fs::write(&pdf, "%PDF-1.7\n1 0 obj").unwrap();
        assert!(matches!(load(&pdf, &Limits::default()), Shown::Other));
        let empty = dir.join("empty.png");
        std::fs::write(&empty, "").unwrap();
        assert!(matches!(load(&empty, &Limits::default()), Shown::Broken(w) if w == "PNG picture"));
        let jpg = dir.join("really.jpg");
        std::fs::copy(png, &jpg).unwrap();
        assert!(matches!(load(&jpg, &Limits::default()), Shown::Picture(_)));
    }
}
