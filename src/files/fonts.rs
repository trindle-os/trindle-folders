// Fonts in Folders: shown in the viewer and installed for this person alone.

use std::fs;
use std::path::{Path, PathBuf};

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{DynamicImage, Rgba, RgbaImage};

use crate::toolkit::t;
use skrifa::string::StringId;
use skrifa::MetadataProvider;

const MOST: u64 = 64 << 20;
const WIDTH: u32 = 900;
const MARGIN: f32 = 28.0;
const LINES: [(&str, f32); 7] = [
    ("The quick brown fox jumps over the lazy dog.", 16.0),
    ("The quick brown fox jumps over the lazy dog.", 24.0),
    ("The quick brown fox jumps over the lazy dog.", 36.0),
    ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 28.0),
    ("abcdefghijklmnopqrstuvwxyz", 28.0),
    ("0123456789 .,;:!? & @ £ € ( ) \" '", 28.0),
    ("Aa", 72.0),
];

pub fn is_font(path: &Path) -> bool {
    let end = path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).unwrap_or_default();
    matches!(end.as_str(), "ttf" | "otf" | "ttc" | "otc")
}

pub fn folder() -> PathBuf {
    #[cfg(test)]
    return std::env::temp_dir().join(format!("trindle-files-fonts-{}", std::process::id()));
    #[cfg(not(test))]
    crate::common::files::data_home().join("fonts")
}

pub fn name(bytes: &[u8]) -> Option<String> {
    let font = skrifa::FontRef::from_index(bytes, 0).ok()?;
    let get = |id| font.localized_strings(id).english_or_first().map(|s| s.to_string());
    let family = get(StringId::TYPOGRAPHIC_FAMILY_NAME).or_else(|| get(StringId::FAMILY_NAME))?;
    let style = get(StringId::TYPOGRAPHIC_SUBFAMILY_NAME).or_else(|| get(StringId::SUBFAMILY_NAME)).unwrap_or_default();
    Some(if style.is_empty() || style.eq_ignore_ascii_case("regular") { family } else { format!("{family} {style}") })
}

pub fn sample(path: &Path) -> Result<(String, DynamicImage), String> {
    if fs::metadata(path).map_err(|e| e.to_string())?.len() > MOST {
        return Err(t!("folders-fonts-too-large").into());
    }
    let bytes = crate::common::files::read_file(path).map_err(|e| e.to_string())?;
    let font = FontRef::try_from_slice_and_index(&bytes, 0).map_err(|_| t!("folders-fonts-unreadable").to_string())?;
    let name = name(&bytes).unwrap_or_else(|| super::work::name_of(path));
    let lines: Vec<(&str, f32)> = std::iter::once((name.as_str(), 40.0)).chain(LINES).collect();
    let height = lines.iter().map(|(_, px)| px * 1.5).sum::<f32>() + 2.0 * MARGIN;
    let mut page = RgbaImage::from_pixel(WIDTH, height.ceil() as u32, Rgba([255, 255, 255, 255]));
    let mut y = MARGIN;
    for (text, px) in lines {
        let scaled = font.as_scaled(PxScale::from(px));
        let baseline = y + scaled.ascent();
        let mut x = MARGIN;
        let mut last = None;
        for c in text.chars() {
            let id = font.glyph_id(c);
            if let Some(prev) = last {
                x += scaled.kern(prev, id);
            }
            let glyph = id.with_scale_and_position(PxScale::from(px), ab_glyph::point(x, baseline));
            x += scaled.h_advance(id);
            last = Some(id);
            let Some(outline) = font.outline_glyph(glyph) else { continue };
            let b = outline.px_bounds();
            outline.draw(|gx, gy, cover| {
                let (px_x, px_y) = (b.min.x as i32 + gx as i32, b.min.y as i32 + gy as i32);
                if px_x < 0 || px_y < 0 || px_x as u32 >= page.width() || px_y as u32 >= page.height() {
                    return;
                }
                let p = page.get_pixel_mut(px_x as u32, px_y as u32);
                let ink = (255.0 * (1.0 - cover.clamp(0.0, 1.0))) as u8;
                let v = p[0].min(ink);
                *p = Rgba([v, v, v, 255]);
            });
        }
        y += px * 1.5;
    }
    Ok((name, DynamicImage::ImageRgba8(page)))
}

pub fn installed(path: &Path) -> bool {
    let Some(name) = path.file_name() else { return false };
    let there = folder().join(name);
    there == path || (fs::metadata(&there).ok().map(|m| m.len()) == fs::metadata(path).ok().map(|m| m.len())
        && crate::common::files::read_file(&there).ok() == crate::common::files::read_file(path).ok())
}

pub fn to_install(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    for path in paths.iter().filter(|p| is_font(p) && !installed(p)) {
        if !out.contains(path) {
            out.push(path.clone());
        }
    }
    out
}

impl super::app::Files {
    pub(super) fn install_fonts(&mut self, items: Vec<PathBuf>) {
        let items = to_install(&items);
        if items.is_empty() {
            return self.say(t!("folders-fonts-have-it"), false);
        }
        let dir = folder();
        if let Err(e) = fs::create_dir_all(&dir) {
            return self.say_help(t!("folders-fonts-install-failed", why = &e.to_string()), super::help::MAKE_FAILED);
        }
        let what = match items.as_slice() {
            [one] => super::work::name_of(one),
            many => t!("folders-fonts-many", n = many.len()),
        };
        let words = super::act::Words {
            doing: t!("folders-fonts-doing", what = &what),
            done: t!("folders-fonts-done", what = &what),
            verb: t!("folders-fonts-verb"),
            help: super::help::WORK_NOT_DONE,
            undo: t!("folders-fonts-undo", what = &what),
        };
        self.start(super::work::Plan { kind: super::work::Kind::Copy, items, to: dir, key: None }, words, super::act::Turn::New);
    }

    pub(super) fn fonts_done(&self) {
        #[cfg(not(test))]
        crate::common::run::spawn(&["fc-cache", &folder().to_string_lossy()]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn some_font() -> PathBuf {
        crate::common::needs::file(Path::new("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"), "the DejaVu font (fonts-dejavu-core)")
    }

    #[test]
    fn a_font_is_named_and_drawn_on_a_white_page() {
        let font = some_font();
        let (name, page) = sample(&font).unwrap();
        assert!(name.starts_with("DejaVu Sans"), "{name}");
        let page = page.to_rgba8();
        assert_eq!(page.width(), WIDTH);
        assert_eq!(page.get_pixel(1, 1).0, [255, 255, 255, 255]);
        assert!(page.pixels().any(|p| p[0] < 64));
        let words = std::env::temp_dir().join(format!("trindle-not-a-font-{}.ttf", std::process::id()));
        fs::write(&words, "just words").unwrap();
        assert!(sample(&words).is_err());
        fs::remove_file(&words).unwrap();
        assert!(is_font(Path::new("a.TTF")) && is_font(Path::new("b.otc")) && !is_font(Path::new("c.txt")));
    }

    #[test]
    fn a_font_is_listed_for_installing_once_and_not_when_it_is_there() {
        let font = some_font();
        let _ = fs::remove_dir_all(folder());
        assert!(!installed(&font));
        assert_eq!(to_install(&[font.clone(), font.clone(), PathBuf::from("/a/readme.txt")]), std::slice::from_ref(&font));
        fs::create_dir_all(folder()).unwrap();
        fs::copy(&font, folder().join(font.file_name().unwrap())).unwrap();
        assert!(installed(&font));
        assert!(to_install(&[font]).is_empty());
        fs::remove_dir_all(folder()).unwrap();
    }
}
