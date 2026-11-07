// Small pictures of files, kept in the freedesktop thumbnail cache.

use std::fs::{self, DirBuilder, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use flate2::Crc;
use image::codecs::png::PngEncoder;
use image::{DynamicImage, ImageEncoder};
use md5::{Digest, Md5};

use super::queue::Queue;
use super::viewer::{self, Kind, Limits};
use super::walled;
use crate::common::files::cache_home;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Normal,
    Large,
}

impl Size {
    pub fn pixels(self) -> u32 {
        match self {
            Size::Normal => 128,
            Size::Large => 256,
        }
    }

    pub fn folder(self) -> &'static str {
        match self {
            Size::Normal => "normal",
            Size::Large => "large",
        }
    }

    pub fn for_pixels(pixels: u32) -> Size {
        if pixels > Size::Normal.pixels() { Size::Large } else { Size::Normal }
    }
}

const FAILED: &str = "fail/trindle-files";

pub fn uri(path: &Path) -> String {
    let whole = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().map(|d| d.join(path)).unwrap_or_else(|_| path.to_path_buf())
    };
    let mut out = String::from("file://");
    for &b in whole.as_os_str().as_bytes() {
        if b.is_ascii_alphanumeric() || b"!$&'()*+,-./:=@_~".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

pub fn can_thumb(path: &Path) -> bool {
    if !fs::metadata(path).is_ok_and(|m| m.is_file()) {
        return false;
    }
    if viewer::is_svg(path) {
        return walled::can_svg();
    }
    viewer::kind(path) == Kind::Picture || boxed_kind(path).is_some()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Boxed {
    Video,
    Pdf,
}

fn boxed_kind(path: &Path) -> Option<Boxed> {
    let mut head = Vec::with_capacity(16);
    crate::common::files::open_file(path).and_then(|f| f.take(16).read_to_end(&mut head)).ok()?;
    if walled::can_video() && video_head(&head) {
        Some(Boxed::Video)
    } else if walled::can_pdf() && head.starts_with(b"%PDF-") {
        Some(Boxed::Pdf)
    } else {
        None
    }
}

fn video_head(h: &[u8]) -> bool {
    let at = |i: usize, m: &[u8]| h.get(i..i + m.len()) == Some(m);
    at(4, b"ftyp") || at(0, &[0x1a, 0x45, 0xdf, 0xa3]) || at(0, b"RIFF") && at(8, b"AVI ")
        || at(0, &[0, 0, 1, 0xba]) || at(0, &[0, 0, 1, 0xb3]) || at(0, b"FLV\x01")
        || at(0, &[0x30, 0x26, 0xb2, 0x75, 0x8e, 0x66, 0xcf, 0x11])
}

#[derive(Clone, Debug)]
pub struct Thumbs {
    root: PathBuf,
    size: Size,
}

impl Default for Thumbs {
    fn default() -> Thumbs {
        Thumbs::new()
    }
}

impl Thumbs {
    pub fn new() -> Thumbs {
        Thumbs::at(&cache_home().join("thumbnails"))
    }

    pub fn at(root: &Path) -> Thumbs {
        Thumbs { root: root.to_path_buf(), size: Size::Normal }
    }

    pub fn sized(self, size: Size) -> Thumbs {
        Thumbs { size, ..self }
    }

    pub fn path_for(&self, path: &Path) -> PathBuf {
        self.root.join(self.size.folder()).join(name(&uri(path)))
    }

    pub fn cached(&self, path: &Path) -> Option<PathBuf> {
        let thumb = self.path_for(path);
        fits(&thumb, path).then_some(thumb)
    }

    pub fn failed(&self, path: &Path) -> bool {
        fits(&self.root.join(FAILED).join(name(&uri(path))), path)
    }

    pub fn make(&self, path: &Path) -> io::Result<PathBuf> {
        if path.starts_with(&self.root) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a thumbnail is not made of a thumbnail"));
        }
        let meta = fs::metadata(path)?;
        let tags = Tags { uri: uri(path), mtime: meta.mtime(), bytes: meta.len() };
        let boxed = if viewer::kind(path) == Kind::Picture { None } else { boxed_kind(path) };
        let made = match boxed {
            Some(Boxed::Video) => walled::video(path, self.size.pixels()),
            Some(Boxed::Pdf) => walled::pdf(path, self.size.pixels()),
            None => picture(path, meta.len(), self.size.pixels()),
        };
        let Some(img) = made else {
            let fail = self.root.join(FAILED).join(name(&tags.uri));
            if let Err(e) = save(&fail, &DynamicImage::new_rgba8(1, 1), &tags) {
                eprintln!("trindle: files: thumbnail failure note: {e}");
            }
            return Err(io::Error::new(io::ErrorKind::InvalidData, "no picture could be made of this file"));
        };
        let thumb = self.path_for(path);
        save(&thumb, &img, &tags)?;
        Ok(thumb)
    }

    pub fn get(&self, path: &Path) -> Option<PathBuf> {
        if let Some(thumb) = self.cached(path) {
            return Some(thumb);
        }
        if self.failed(path) || !can_thumb(path) {
            return None;
        }
        self.make(path).ok()
    }
}

fn name(uri: &str) -> String {
    format!("{:x}.png", Md5::digest(uri.as_bytes()))
}

struct Tags {
    uri: String,
    mtime: i64,
    bytes: u64,
}

fn fits(thumb: &Path, path: &Path) -> bool {
    let Ok(meta) = fs::metadata(path) else { return false };
    let Ok(png) = crate::common::files::read_file(thumb) else { return false };
    let texts = texts(&png);
    let get = |key: &str| texts.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str());
    let mtime = get("Thumb::MTime").and_then(|t| t.split('.').next()).and_then(|t| t.parse::<i64>().ok());
    let size_ok = get("Thumb::Size").is_none_or(|s| s.parse::<u64>().ok() == Some(meta.len()));
    get("Thumb::URI") == Some(uri(path).as_str()) && mtime == Some(meta.mtime()) && size_ok
}

fn texts(png: &[u8]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if !png.starts_with(b"\x89PNG\r\n\x1a\n") {
        return out;
    }
    let mut at = 8;
    while at + 8 <= png.len() {
        let len = u32::from_be_bytes([png[at], png[at + 1], png[at + 2], png[at + 3]]) as usize;
        let kind = &png[at + 4..at + 8];
        let Some(data) = png.get(at + 8..at + 8 + len) else { break };
        if kind == b"IDAT" || kind == b"IEND" {
            break;
        }
        if kind == b"tEXt" {
            if let Some(nul) = data.iter().position(|&b| b == 0) {
                let latin = |b: &[u8]| b.iter().map(|&c| c as char).collect::<String>();
                out.push((latin(&data[..nul]), latin(&data[nul + 1..])));
            }
        }
        at += 12 + len;
    }
    out
}

fn picture(path: &Path, bytes: u64, most: u32) -> Option<DynamicImage> {
    let limits = Limits::default();
    if bytes > limits.picture {
        return None;
    }
    let img = if viewer::is_svg(path) {
        viewer::svg(path, most)?
    } else {
        let (w, h) = crate::toolkit::picture::reader(path).ok()?.into_dimensions().ok()?;
        if w as u64 * h as u64 > limits.pixels {
            return None;
        }
        let img = crate::toolkit::picture::load(path)?;
        let big = w as u64 * h as u64 > 4_000_000;
        let small = if img.width() > most || img.height() > most { img.thumbnail(most, most) } else { img };
        if big {
            crate::common::memory::give_back();
        }
        small
    };
    Some(img)
}

fn save(thumb: &Path, img: &DynamicImage, tags: &Tags) -> io::Result<()> {
    let folder = thumb.parent().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no folder"))?;
    DirBuilder::new().recursive(true).mode(0o700).create(folder)?;
    let rgba = img.to_rgba8();
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
        .write_image(rgba.as_raw(), rgba.width(), rgba.height(), image::ExtendedColorType::Rgba8)
        .map_err(io::Error::other)?;
    if png.get(12..16) != Some(b"IHDR") {
        return Err(io::Error::other("the PNG writer gave no header"));
    }
    let mut notes = Vec::new();
    for (k, v) in [
        ("Thumb::URI", tags.uri.clone()),
        ("Thumb::MTime", tags.mtime.to_string()),
        ("Thumb::Size", tags.bytes.to_string()),
        ("Software", "Trindle Folders".to_string()),
    ] {
        chunk(&mut notes, b"tEXt", &[k.as_bytes(), b"\0", v.as_bytes()].concat());
    }
    png.splice(33..33, notes);
    let part = thumb.with_extension(format!("png.{}.part", std::process::id()));
    let wrote = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&part)
        .and_then(|mut f| f.write_all(&png))
        .and_then(|_| fs::rename(&part, thumb));
    if wrote.is_err() {
        let _ = fs::remove_file(&part);
    }
    wrote
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    let mut crc = Crc::new();
    crc.update(kind);
    crc.update(data);
    out.extend_from_slice(&crc.sum().to_be_bytes());
}

pub struct Maker {
    queue: Queue<PathBuf>,
}

impl Maker {
    pub fn new(thumbs: Thumbs, tell: impl Fn(&Path, Option<PathBuf>) + Send + Sync + 'static) -> Maker {
        let thumbs = Arc::new(thumbs);
        let queue = Queue::new("folders-thumbs", 1, move |path: PathBuf, _stop: &AtomicBool| {
            tell(&path, thumbs.get(&path));
        });
        Maker { queue }
    }

    pub fn ask(&self, path: &Path) {
        self.queue.ask(path.to_path_buf());
    }

    pub fn cancel(&self) {
        self.queue.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{mpsc, Mutex};
    use std::time::{Duration, SystemTime};

    #[test]
    fn a_pipe_named_as_a_picture_keeps_nothing_in_folders_waiting() {
        use std::os::unix::fs::OpenOptionsExt;
        let dir = place("pipe");
        let (pipe, drawn) = (dir.join("x.jpg"), dir.join("y.svg"));
        for p in [&pipe, &drawn] {
            let c = std::ffi::CString::new(p.to_str().unwrap()).unwrap();
            assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
        }
        let (tell, told) = mpsc::channel();
        let (asked, svg) = (pipe.clone(), drawn.clone());
        std::thread::spawn(move || {
            let _ = tell.send((boxed_kind(&asked).is_none(), picture(&asked, 0, 64).is_none(), super::super::imaging::turn(&asked, super::super::imaging::Turn::Right).is_err(),
                               matches!(viewer::kind(&asked), viewer::Kind::Other), matches!(viewer::load(&asked, &Limits::default()), viewer::Shown::Unread(_)),
                               viewer::read_text(&asked, 10).is_err(), viewer::svg(&svg, 64).is_none()));
        });
        let got = told.recv_timeout(Duration::from_secs(5));
        for p in [&pipe, &drawn] {
            let _ = fs::OpenOptions::new().write(true).custom_flags(libc::O_NONBLOCK).open(p);
        }
        assert_eq!(got, Ok((true, true, true, true, true, true, true)), "a pipe was waited on, or read as a picture");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn nothing_a_picture_names_is_followed_by_the_thumbnail_or_the_viewer() {
        use crate::toolkit::picture::strangers::{naming, red, red_and_blue};
        crate::common::needs::program("bwrap");
        let dir = place("names");
        let thumbs = Thumbs::at(&dir.join("cache"));
        let other = dir.join("other.svg");
        fs::write(&other, red()).unwrap();
        let mut all = naming(&other);
        all.push(("a device with no end", crate::toolkit::picture::strangers::blue("<image href=\"/dev/zero\" width=\"8\" height=\"8\"/>").into_bytes()));
        for (n, (what, data)) in all.into_iter().enumerate() {
            let squeezed = data.starts_with(&[0x1f, 0x8b]);
            let file = dir.join(format!("{n}.{}", if squeezed { "svgz" } else { "svg" }));
            fs::write(&file, &data).unwrap();
            if squeezed {
                assert_eq!(red_and_blue(&walled::svg(&file, 8).unwrap()), (false, true), "{what}");
                continue;
            }
            assert!(can_thumb(&file), "{what}");
            let made = thumbs.make(&file).unwrap_or_else(|e| panic!("{what}: {e}"));
            assert_eq!(red_and_blue(&image::open(&made).unwrap()), (false, true), "{what}: the thumbnail");
            match viewer::load(&file, &Limits::default()) {
                viewer::Shown::Picture(img) => assert_eq!(red_and_blue(&img), (false, true), "{what}: the viewer"),
                _ => panic!("{what}: the viewer did not show it"),
            }
        }
        let long = dir.join("long.svg");
        fs::write(&long, vec![b' '; crate::toolkit::picture::SVG_MOST as usize + 1]).unwrap();
        assert!(viewer::svg(&long, 8).is_none());
        fs::remove_dir_all(&dir).unwrap();
    }

    fn place(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-thumbs-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn names_follow_the_freedesktop_rules() {
        assert_eq!(uri(Path::new("/home/jens/photos/me.png")), "file:///home/jens/photos/me.png");
        assert_eq!(name(&uri(Path::new("/home/jens/photos/me.png"))), "c6ee772d9e49320e97ec29a7eb5b1697.png");
        assert_eq!(uri(Path::new("/home/john/a b#1%;\u{e9}.png")), "file:///home/john/a%20b%231%25%3B%C3%A9.png");
    }

    #[test]
    fn a_pdf_has_a_thumbnail_of_its_first_page() {
        let dir = place("pdf");
        let file = dir.join("letter");
        fs::write(&file, walled::tests::a4_pdf("0 0 0 rg 50 50 100 100 re f")).unwrap();
        crate::common::needs::programs(&["bwrap", "pdftoppm"]);
        assert!(can_thumb(&file));
        let thumbs = Thumbs::at(&dir.join("cache"));
        let img = image::open(thumbs.get(&file).unwrap()).unwrap();
        assert_eq!(img.height(), 128);
        let broken = dir.join("broken.pdf");
        fs::write(&broken, b"%PDF-1.7 but nothing after").unwrap();
        assert!(thumbs.get(&broken).is_none());
        assert!(thumbs.failed(&broken));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn videos_are_told_by_how_they_start() {
        assert!(video_head(b"\0\0\0\x18ftypisom"));
        assert!(video_head(&[0x1a, 0x45, 0xdf, 0xa3, 1]));
        assert!(video_head(b"RIFF\0\0\0\0AVI LIST"));
        assert!(video_head(b"FLV\x01\x05"));
        assert!(!video_head(b"RIFF\0\0\0\0WAVEfmt "));
        assert!(!video_head(b"%PDF-1.7"));
        assert!(!video_head(b"ftyp"));
        assert!(!video_head(b""));
    }

    #[test]
    fn a_video_has_a_thumbnail_when_ffmpeg_is_there() {
        let dir = place("video");
        let clip = dir.join("clip.webm");
        assert!(!can_thumb(&dir.join("missing.mp4")));
        crate::common::needs::programs(&["bwrap", "ffmpeg"]);
        let made = std::process::Command::new("ffmpeg")
            .args(["-nostdin", "-loglevel", "error", "-f", "lavfi", "-i", "testsrc=size=320x240:rate=5:duration=1"])
            .arg(&clip).status();
        assert!(made.is_ok_and(|s| s.success()), "ffmpeg could not make the clip this test takes a thumbnail of");
        assert!(can_thumb(&clip));
        let thumbs = Thumbs::at(&dir.join("cache"));
        let img = image::open(thumbs.get(&clip).unwrap()).unwrap();
        assert_eq!((img.width(), img.height()), (128, 96));
        let broken = dir.join("broken.mkv");
        fs::write(&broken, b"\x1a\x45\xdf\xa3 but nothing after").unwrap();
        assert!(can_thumb(&broken));
        assert!(thumbs.get(&broken).is_none());
        assert!(thumbs.failed(&broken));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_thumbnail_is_made_reused_and_made_again_after_a_change() {
        let dir = place("make");
        let thumbs = Thumbs::at(&dir.join("cache"));
        let photo = dir.join("photo.png");
        image::RgbImage::from_pixel(600, 300, image::Rgb([20, 120, 220])).save(&photo).unwrap();
        assert!(thumbs.cached(&photo).is_none());
        let made = thumbs.get(&photo).unwrap();
        assert_eq!(made, thumbs.path_for(&photo));
        assert!(made.starts_with(dir.join("cache/normal")));
        let img = image::open(&made).unwrap();
        assert_eq!((img.width(), img.height()), (128, 64));
        let large = thumbs.clone().sized(Size::Large);
        assert!(large.cached(&photo).is_none());
        let big = large.get(&photo).unwrap();
        assert!(big.starts_with(dir.join("cache/large")));
        let img = image::open(&big).unwrap();
        assert_eq!((img.width(), img.height()), (256, 128));
        assert_eq!((Size::for_pixels(96), Size::for_pixels(128), Size::for_pixels(192), Size::for_pixels(512)), (Size::Normal, Size::Normal, Size::Large, Size::Large));
        let texts = texts(&fs::read(&made).unwrap());
        assert!(texts.contains(&("Thumb::URI".into(), uri(&photo))));
        assert_eq!(fs::metadata(&made).unwrap().mode() & 0o777, 0o600);

        let written = fs::metadata(&made).unwrap().modified().unwrap();
        assert_eq!(thumbs.get(&photo), Some(made.clone()));
        assert_eq!(fs::metadata(&made).unwrap().modified().unwrap(), written);

        let later = SystemTime::now() + Duration::from_secs(10);
        fs::File::options().write(true).open(&photo).unwrap().set_modified(later).unwrap();
        assert!(thumbs.cached(&photo).is_none());
        assert!(thumbs.get(&photo).is_some());
        assert!(thumbs.cached(&photo).is_some());

        let dot = dir.join("dot.png");
        image::RgbImage::from_pixel(10, 5, image::Rgb([0, 0, 0])).save(&dot).unwrap();
        let img = image::open(thumbs.get(&dot).unwrap()).unwrap();
        assert_eq!((img.width(), img.height()), (10, 5));

        let broken = dir.join("broken.png");
        fs::write(&broken, b"\x89PNG\r\n\x1a\nnot really").unwrap();
        assert!(thumbs.get(&broken).is_none());
        assert!(thumbs.failed(&broken));
        assert!(!can_thumb(&dir.join("cache")));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_maker_works_on_its_own_thread() {
        let dir = place("maker");
        let svg = dir.join("box.svg");
        fs::write(&svg, r#"<svg xmlns="http://www.w3.org/2000/svg" width="400" height="200"><rect width="400" height="200" fill="red"/></svg>"#).unwrap();
        let (tx, rx) = mpsc::channel();
        let tx = Mutex::new(tx);
        let maker = Maker::new(Thumbs::at(&dir.join("cache")), move |p, t| {
            tx.lock().unwrap().send((p.to_path_buf(), t)).unwrap();
        });
        maker.ask(&svg);
        let (p, t) = rx.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(p, svg);
        let img = image::open(t.unwrap()).unwrap();
        assert_eq!((img.width(), img.height()), (128, 64));
        fs::remove_dir_all(&dir).unwrap();
    }
}
