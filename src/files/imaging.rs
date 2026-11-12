//! Turning and resizing pictures from the right-click menu.

use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};

use crate::toolkit::t;

use super::meta::{self, ExifEdit};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
    Over,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Work {
    Turn(Turn),
    Resize(Size),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Share(u32),
    Longest(u32),
}

pub const SIZES: [(Size, &str); 7] = [
    (Size::Share(25), "25%"), (Size::Share(50), "50%"), (Size::Share(75), "75%"),
    (Size::Longest(640), "640"), (Size::Longest(1280), "1280"), (Size::Longest(1920), "1920"), (Size::Longest(3840), "3840"),
];

const QUALITY: u8 = 90;

fn ending(path: &Path) -> String {
    path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).unwrap_or_default()
}

pub fn turnable(path: &Path) -> bool {
    matches!(ending(path).as_str(), "jpg" | "jpeg" | "png" | "webp" | "bmp")
}

pub fn resizable(path: &Path) -> bool {
    matches!(ending(path).as_str(), "jpg" | "jpeg" | "png" | "webp" | "bmp" | "tif" | "tiff")
}

pub fn turn(path: &Path, turn: Turn) -> io::Result<()> {
    if matches!(ending(path).as_str(), "jpg" | "jpeg") {
        let now = crate::toolkit::picture::reader(path)?.into_decoder()
            .ok().and_then(|mut d| image::ImageDecoder::orientation(&mut d).ok())
            .map_or(1, |o| o.to_exif());
        let edit = ExifEdit { orientation: Some(u16::from(turned(now, turn))), ..ExifEdit::default() };
        return meta::write_exif(path, &edit);
    }
    if !turnable(path) {
        return Err(io::Error::new(io::ErrorKind::Unsupported, t!("folders-imaging-no-turn")));
    }
    if std::fs::symlink_metadata(path)?.file_type().is_symlink() {
        return Err(io::Error::new(io::ErrorKind::Unsupported, t!("folders-imaging-no-turn-link")));
    }
    let format = format_of(path)?;
    let old = crate::common::files::read_file(path)?;
    if moving(&old) {
        return Err(io::Error::new(io::ErrorKind::Unsupported, t!("folders-imaging-no-turn-moving")));
    }
    let img = load(path)?;
    let img = match turn {
        Turn::Left => img.rotate270(),
        Turn::Right => img.rotate90(),
        Turn::Over => img.rotate180(),
    };
    let mut new = Vec::new();
    img.write_to(&mut io::Cursor::new(&mut new), format).map_err(io::Error::other)?;
    let new = match format {
        ImageFormat::Png => png_carry(&old, new, turn != Turn::Over),
        ImageFormat::WebP => webp_carry(&old, new, (img.width(), img.height()), img.color().has_alpha()),
        _ => new,
    };
    let own = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.subsec_nanos());
    let side = path.with_file_name(format!(".{}.{}-{own}.turning", super::work::name_of(path), std::process::id()));
    made_new(&side, std::fs::metadata(path)?.mode(), |out| out.write_all(&new))?;
    let swap = || -> io::Result<()> {
        let mode = std::fs::metadata(path)?.permissions();
        std::fs::set_permissions(&side, mode)?;
        std::fs::rename(&side, path)
    };
    swap().inspect_err(|_| drop(std::fs::remove_file(&side)))
}

fn turned(now: u8, turn: Turn) -> u8 {
    const RIGHT: [u8; 9] = [0, 6, 7, 8, 5, 2, 3, 4, 1];
    let now = if (1..=8).contains(&now) { now } else { 1 };
    let times = match turn {
        Turn::Right => 1,
        Turn::Over => 2,
        Turn::Left => 3,
    };
    (0..times).fold(now, |o, _| RIGHT[o as usize])
}

pub fn resize(path: &Path, size: Size) -> io::Result<PathBuf> {
    let img = load(path)?;
    let (w, h) = (img.width().max(1), img.height().max(1));
    let (nw, nh) = match size {
        Size::Share(n) => ((w * n / 100).max(1), (h * n / 100).max(1)),
        Size::Longest(most) if w.max(h) <= most => (w, h),
        Size::Longest(most) if w >= h => (most, (h as u64 * most as u64 / w as u64).max(1) as u32),
        Size::Longest(most) => ((w as u64 * most as u64 / h as u64).max(1) as u32, most),
    };
    let small = img.resize_exact(nw, nh, FilterType::Lanczos3);
    drop(img);
    let dir = path.parent().unwrap_or(Path::new("."));
    let stem = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let end = path.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    let name = format!("{stem} ({nw}\u{d7}{nh}){end}");
    let to = match dir.join(&name) {
        free if std::fs::symlink_metadata(&free).is_err() => free,
        _ => super::work::free_name(dir, std::ffi::OsStr::new(&name), false),
    };
    let saved = std::fs::metadata(path).and_then(|m| save(&small, &to, format_of(path)?, m.mode()));
    crate::common::memory::give_back();
    saved.map(|()| to)
}

fn load(path: &Path) -> io::Result<DynamicImage> {
    crate::toolkit::picture::load(path).ok_or_else(|| io::Error::other(t!("folders-imaging-unreadable")))
}

fn format_of(path: &Path) -> io::Result<ImageFormat> {
    ImageFormat::from_path(path).map_err(|_| io::Error::new(io::ErrorKind::Unsupported, t!("folders-imaging-no-write")))
}

fn save(img: &DynamicImage, to: &Path, format: ImageFormat, mode: u32) -> io::Result<()> {
    made_new(to, mode, |out| match format {
        ImageFormat::Jpeg => img.to_rgb8().write_with_encoder(JpegEncoder::new_with_quality(out, QUALITY)),
        other => img.write_to(out, other),
    }.map_err(io::Error::other))
}

fn made_new(to: &Path, mode: u32, write: impl FnOnce(&mut BufWriter<&std::fs::File>) -> io::Result<()>) -> io::Result<()> {
    let made = OpenOptions::new().write(true).create_new(true).mode(mode & 0o777).open(to)?;
    let all = || -> io::Result<()> {
        let mut out = BufWriter::new(&made);
        write(&mut out)?;
        out.into_inner().map_err(|e| e.into_error())?.sync_all()
    };
    all().inspect_err(|_| super::proof::unmake(&made, to))
}

fn png_chunks(b: &[u8]) -> Option<Vec<([u8; 4], &[u8])>> {
    let mut at = 8;
    let mut out = Vec::new();
    if b.get(..8)? != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    while at + 12 <= b.len() {
        let len = u32::from_be_bytes(b[at..at + 4].try_into().ok()?) as usize;
        let end = at.checked_add(12)?.checked_add(len)?;
        let kind: [u8; 4] = b[at + 4..at + 8].try_into().ok()?;
        out.push((kind, b.get(at..end)?));
        at = end;
    }
    Some(out)
}

fn webp_chunks(b: &[u8]) -> Option<Vec<([u8; 4], &[u8])>> {
    if b.get(..4)? != b"RIFF" || b.get(8..12)? != b"WEBP" {
        return None;
    }
    let mut at = 12;
    let mut out = Vec::new();
    while at + 8 <= b.len() {
        let kind: [u8; 4] = b[at..at + 4].try_into().ok()?;
        let len = u32::from_le_bytes(b[at + 4..at + 8].try_into().ok()?) as usize;
        out.push((kind, b.get(at + 8..at.checked_add(8)?.checked_add(len)?)?));
        at += 8 + len + (len & 1);
    }
    Some(out)
}

fn moving(b: &[u8]) -> bool {
    if let Some(c) = png_chunks(b) {
        return c.iter().any(|(k, _)| k == b"acTL");
    }
    webp_chunks(b).is_some_and(|c| c.iter().any(|(k, d)| k == b"ANIM" || k == b"ANMF" || (k == b"VP8X" && d.first().is_some_and(|f| f & 0x02 != 0))))
}

fn crc32(parts: &[&[u8]]) -> u32 {
    let mut c = !0u32;
    for b in parts.iter().flat_map(|p| p.iter()) {
        c ^= u32::from(*b);
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ 0xEDB8_8320 } else { c >> 1 };
        }
    }
    !c
}

fn png_chunk(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = (data.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    out.extend_from_slice(&crc32(&[kind, data]).to_be_bytes());
    out
}

fn png_carry(old: &[u8], new: Vec<u8>, quarter: bool) -> Vec<u8> {
    const KEPT: [&[u8; 4]; 11] = [b"iCCP", b"sRGB", b"gAMA", b"cHRM", b"sBIT", b"tEXt", b"zTXt", b"iTXt", b"tIME", b"eXIf", b"pHYs"];
    let (Some(was), Some(now)) = (png_chunks(old), png_chunks(&new)) else { return new };
    let mut carried = Vec::new();
    for (kind, whole) in &was {
        if !KEPT.contains(&kind) || now.iter().any(|(k, _)| k == kind) {
            continue;
        }
        let data = &whole[8..whole.len() - 4];
        match kind {
            b"eXIf" => carried.extend(png_chunk(kind, &meta::exif_upright(data).unwrap_or_else(|_| data.to_vec()))),
            b"pHYs" if quarter && data.len() == 9 => {
                let mut d = data.to_vec();
                d[..8].rotate_left(4);
                carried.extend(png_chunk(kind, &d));
            }
            _ => carried.extend_from_slice(whole),
        }
    }
    let Some((_, ihdr)) = now.first().filter(|(k, _)| k == b"IHDR") else { return new };
    let after = 8 + ihdr.len();
    let mut out = new[..after].to_vec();
    out.extend(carried);
    out.extend_from_slice(&new[after..]);
    out
}

fn webp_carry(old: &[u8], new: Vec<u8>, (w, h): (u32, u32), alpha: bool) -> Vec<u8> {
    let (Some(was), Some(now)) = (webp_chunks(old), webp_chunks(&new)) else { return new };
    let find = |k: &[u8; 4]| was.iter().find(|(kind, _)| kind == k).map(|(_, d)| d.to_vec());
    let (icc, exif, xmp) = (find(b"ICCP"), find(b"EXIF").map(|e| meta::exif_upright(&e).unwrap_or(e)), find(b"XMP "));
    if icc.is_none() && exif.is_none() && xmp.is_none() {
        return new;
    }
    let chunk = |kind: &[u8; 4], data: &[u8]| {
        let mut c = kind.to_vec();
        c.extend_from_slice(&(data.len() as u32).to_le_bytes());
        c.extend_from_slice(data);
        if data.len() & 1 == 1 {
            c.push(0);
        }
        c
    };
    let flags = (u8::from(icc.is_some()) << 5) | (u8::from(alpha) << 4) | (u8::from(exif.is_some()) << 3) | (u8::from(xmp.is_some()) << 2);
    let mut vp8x = vec![flags, 0, 0, 0];
    vp8x.extend_from_slice(&(w - 1).to_le_bytes()[..3]);
    vp8x.extend_from_slice(&(h - 1).to_le_bytes()[..3]);
    let mut body = b"WEBP".to_vec();
    body.extend(chunk(b"VP8X", &vp8x));
    if let Some(d) = &icc {
        body.extend(chunk(b"ICCP", d));
    }
    for (kind, data) in now.iter().filter(|(k, _)| k != b"VP8X") {
        body.extend(chunk(kind, data));
    }
    for (kind, d) in [(b"EXIF", &exif), (b"XMP ", &xmp)] {
        if let Some(d) = d {
            body.extend(chunk(kind, d));
        }
    }
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend(body);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("trindle-imaging-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn halves() -> DynamicImage {
        DynamicImage::ImageRgb8(RgbImage::from_fn(32, 16, |x, _| if x < 16 { Rgb([230, 20, 20]) } else { Rgb([20, 20, 230]) }))
    }

    fn red_side(img: &DynamicImage) -> &'static str {
        let rgb = img.to_rgb8();
        let red = |x: u32, y: u32| rgb.get_pixel(x, y)[0] > 128;
        let (w, h) = (rgb.width(), rgb.height());
        match (red(w / 4, h / 2), red(3 * w / 4, h / 2), red(w / 2, h / 4), red(w / 2, 3 * h / 4)) {
            (true, false, _, _) => "left",
            (false, true, _, _) => "right",
            (_, _, true, false) => "top",
            (_, _, false, true) => "bottom",
            _ => "unclear",
        }
    }

    #[test]
    fn a_jpeg_turns_by_its_note_from_any_way_up() {
        let d = dir("jpeg");
        let path = d.join("photo.jpg");
        let right = |side: &str| match side { "left" => "top", "top" => "right", "right" => "bottom", _ => "left" };
        for start in 1..=8u8 {
            let _ = std::fs::remove_file(&path);
            save(&halves(), &path, ImageFormat::Jpeg, 0o644).unwrap();
            meta::write_exif(&path, &ExifEdit { orientation: Some(u16::from(start)), ..ExifEdit::default() }).unwrap();
            let before = red_side(&load(&path).unwrap());
            turn(&path, Turn::Right).unwrap();
            assert_eq!(red_side(&load(&path).unwrap()), right(before), "from {start}");
            turn(&path, Turn::Left).unwrap();
            assert_eq!(red_side(&load(&path).unwrap()), before, "back from {start}");
            turn(&path, Turn::Over).unwrap();
            assert_eq!(red_side(&load(&path).unwrap()), right(right(before)), "over from {start}");
        }
        std::fs::remove_dir_all(&d).unwrap();
    }

    fn exif_turned_right() -> Vec<u8> {
        let mut t = b"II*\0\x08\0\0\0\x01\0".to_vec();
        t.extend_from_slice(&[0x12, 0x01, 3, 0, 1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0]);
        t
    }

    fn with_chunks(png: &[u8], chunks: &[(&[u8; 4], &[u8])]) -> Vec<u8> {
        let mut out = png[..33].to_vec();
        for (k, d) in chunks {
            out.extend(png_chunk(k, d));
        }
        out.extend_from_slice(&png[33..]);
        out
    }

    #[test]
    fn a_turned_png_or_webp_keeps_its_notes_and_exif_and_a_moving_picture_or_link_is_not_turned() {
        let d = dir("carry");
        let path = d.join("drawing.png");
        save(&halves(), &path, ImageFormat::Png, 0o644).unwrap();
        let plain = std::fs::read(&path).unwrap();
        let phys = [0, 0, 0x0b, 0x13, 0, 0, 0x16, 0x26, 1];
        std::fs::write(&path, with_chunks(&plain, &[(b"tEXt", b"Comment\0from the holiday"), (b"eXIf", &exif_turned_right()), (b"pHYs", &phys)])).unwrap();
        assert_eq!(red_side(&load(&path).unwrap()), "top");
        turn(&path, Turn::Right).unwrap();
        let got = std::fs::read(&path).unwrap();
        let chunks = png_chunks(&got).unwrap();
        assert!(chunks.iter().any(|(k, c)| k == b"tEXt" && c.windows(16).any(|w| w == b"from the holiday")), "the note was dropped");
        assert!(chunks.iter().any(|(k, _)| k == b"eXIf"), "the EXIF was dropped");
        assert!(chunks.iter().any(|(k, c)| k == b"pHYs" && c[8..17] == [0, 0, 0x16, 0x26, 0, 0, 0x0b, 0x13, 1]), "the density was not swapped across");
        assert_eq!(red_side(&load(&path).unwrap()), "right", "turned twice: the EXIF still turns it");
        let moving = d.join("moving.png");
        let apng = with_chunks(&plain, &[(b"acTL", &[0, 0, 0, 2, 0, 0, 0, 0])]);
        std::fs::write(&moving, &apng).unwrap();
        assert!(turn(&moving, Turn::Left).is_err());
        assert_eq!(std::fs::read(&moving).unwrap(), apng);
        let link = d.join("link.png");
        std::os::unix::fs::symlink(&moving, &link).unwrap();
        assert!(turn(&link, Turn::Left).is_err());
        assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
        let webp = d.join("photo.webp");
        save(&halves(), &webp, ImageFormat::WebP, 0o644).unwrap();
        let lossless = std::fs::read(&webp).unwrap();
        let (_, vp8l) = webp_chunks(&lossless).unwrap().into_iter().find(|(k, _)| k == b"VP8L").unwrap();
        let mut body = b"WEBP".to_vec();
        for (k, data) in [(b"VP8X", &[0x08u8, 0, 0, 0, 31, 0, 0, 15, 0, 0][..]), (b"VP8L", vp8l), (b"EXIF", &b"II*\0\x08\0\0\0\0\0\0\0\0\0"[..])] {
            body.extend_from_slice(k);
            body.extend_from_slice(&(data.len() as u32).to_le_bytes());
            body.extend_from_slice(data);
            if data.len() & 1 == 1 {
                body.push(0);
            }
        }
        let mut file = b"RIFF".to_vec();
        file.extend_from_slice(&(body.len() as u32).to_le_bytes());
        file.extend(body);
        std::fs::write(&webp, &file).unwrap();
        turn(&webp, Turn::Left).unwrap();
        let got = std::fs::read(&webp).unwrap();
        assert!(webp_chunks(&got).unwrap().iter().any(|(k, _)| k == b"EXIF"), "the WebP's EXIF was dropped");
        let img = load(&webp).unwrap();
        assert_eq!((img.width(), img.height(), red_side(&img)), (16, 32, "bottom"));
        let mut animated = file.clone();
        animated[20] |= 0x02;
        std::fs::write(&webp, &animated).unwrap();
        assert!(turn(&webp, Turn::Left).is_err());
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_png_is_turned_whole_and_a_gif_is_not() {
        let d = dir("png");
        let path = d.join("drawing.png");
        save(&halves(), &path, ImageFormat::Png, 0o644).unwrap();
        turn(&path, Turn::Left).unwrap();
        let img = load(&path).unwrap();
        assert_eq!((img.width(), img.height(), red_side(&img)), (16, 32, "bottom"));
        assert!(!d.read_dir().unwrap().any(|e| e.unwrap().file_name().to_string_lossy().ends_with(".turning")));
        let gif = d.join("moving.gif");
        save(&halves(), &gif, ImageFormat::Gif, 0o644).unwrap();
        assert!(turn(&gif, Turn::Left).is_err());
        let theirs = d.join(".drawing.png.turning");
        std::fs::write(&theirs, b"another's").unwrap();
        turn(&path, Turn::Left).unwrap();
        assert_eq!(std::fs::read(&theirs).unwrap(), b"another's", "a file the turn never made was removed");
        assert_eq!(save(&halves(), &theirs, ImageFormat::Png, 0o644).unwrap_err().kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(std::fs::read(&theirs).unwrap(), b"another's");
        std::fs::remove_file(&theirs).unwrap();
        assert!(!d.read_dir().unwrap().any(|e| e.unwrap().file_name().to_string_lossy().ends_with(".turning")));
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_private_picture_stays_private_when_resized_or_turned() {
        use std::os::unix::fs::PermissionsExt;
        let d = dir("private");
        let path = d.join("secret.png");
        save(&halves(), &path, ImageFormat::Png, 0o600).unwrap();
        let mode = |p: &Path| std::fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&path), 0o600);
        let small = resize(&path, Size::Share(50)).unwrap();
        assert_eq!(mode(&small), 0o600);
        turn(&path, Turn::Right).unwrap();
        assert_eq!(mode(&path), 0o600);
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_resized_copy_is_made_beside_it() {
        let d = dir("resize");
        let path = d.join("photo.jpg");
        save(&halves(), &path, ImageFormat::Jpeg, 0o644).unwrap();
        let half = resize(&path, Size::Share(50)).unwrap();
        assert_eq!(half, d.join("photo (16\u{d7}8).jpg"));
        assert_eq!(image::image_dimensions(&half).unwrap(), (16, 8));
        let again = resize(&path, Size::Share(50)).unwrap();
        assert_eq!(again, d.join("photo (16\u{d7}8) (2).jpg"));
        let same = resize(&path, Size::Longest(640)).unwrap();
        assert_eq!(image::image_dimensions(&same).unwrap(), (32, 16));
        let long = resize(&path, Size::Longest(8)).unwrap();
        assert_eq!(image::image_dimensions(&long).unwrap(), (8, 4));
        meta::write_exif(&path, &ExifEdit { orientation: Some(6), ..ExifEdit::default() }).unwrap();
        let upright = resize(&path, Size::Share(50)).unwrap();
        assert_eq!(image::image_dimensions(&upright).unwrap(), (8, 16));
        assert!(!std::fs::read(&path).unwrap().is_empty());
        std::fs::remove_dir_all(&d).unwrap();
    }
}
