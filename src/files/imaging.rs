// Turning and resizing pictures from the right-click menu, with the image crate.

use std::fs::OpenOptions;
use std::io::{self, BufWriter};
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
    let img = load(path)?;
    let img = match turn {
        Turn::Left => img.rotate270(),
        Turn::Right => img.rotate90(),
        Turn::Over => img.rotate180(),
    };
    let own = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.subsec_nanos());
    let side = path.with_file_name(format!(".{}.{}-{own}.turning", super::work::name_of(path), std::process::id()));
    save(&img, &side, format_of(path)?, std::fs::metadata(path)?.mode())?;
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
    let made = OpenOptions::new().write(true).create_new(true).mode(mode & 0o777).open(to)?;
    let write = || -> io::Result<()> {
        let mut out = BufWriter::new(&made);
        match format {
            ImageFormat::Jpeg => img.to_rgb8().write_with_encoder(JpegEncoder::new_with_quality(&mut out, QUALITY)),
            other => img.write_to(&mut out, other),
        }.map_err(io::Error::other)?;
        out.into_inner().map_err(|e| e.into_error())?.sync_all()
    };
    write().inspect_err(|_| super::proof::unmake(&made, to))
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
