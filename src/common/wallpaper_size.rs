//! How big a wallpaper may be: Folders asks before it sends a picture to the wallpaper program, which holds to the same limits.

use crate::toolkit::t;
use std::path::Path;

pub const MOST_WIDE: u32 = 3840;
pub const MOST_HIGH: u32 = 2160;

pub const MOST_STILL_BYTES: u64 = 20 << 20;
pub const MOST_MOVING_BYTES: u64 = 200 << 20;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Over {
    pub pixels: bool,
    pub film_bigger: bool,
    pub still_bytes: bool,
    pub moving_bytes: bool,
}

impl Over {
    pub fn any(&self) -> bool {
        self.pixels || self.film_bigger || self.still_bytes || self.moving_bytes
    }

    pub fn reasons(&self) -> String {
        let mut out = Vec::new();
        if self.pixels {
            out.push(t!("folders-wallpaper-larger-4k"));
        } else if self.film_bigger {
            out.push(t!("folders-wallpaper-larger-screen"));
        }
        match (self.still_bytes, self.moving_bytes) {
            (true, true) => out.push(t!("folders-wallpaper-over-20-or-200")),
            (true, false) => out.push(t!("folders-wallpaper-over-20")),
            (false, true) => out.push(t!("folders-wallpaper-over-200")),
            (false, false) => {}
        }
        match out.as_slice() {
            [first, second] => t!("folders-wallpaper-reasons-or", first = *first, second = *second),
            [only] => only.to_string(),
            _ => String::new(),
        }
    }
}

fn fits(width: u32, height: u32) -> bool {
    (width <= MOST_WIDE && height <= MOST_HIGH) || (width <= MOST_HIGH && height <= MOST_WIDE)
}

fn moving_one(path: &Path) -> bool {
    path.extension().is_some_and(|x| x.eq_ignore_ascii_case("webm"))
}

fn by_size(path: &Path) -> Option<(Over, u64, std::time::SystemTime)> {
    let about = std::fs::metadata(path).ok()?;
    let mut over = Over::default();
    if moving_one(path) {
        over.moving_bytes = about.len() > MOST_MOVING_BYTES;
    } else {
        over.still_bytes = about.len() > MOST_STILL_BYTES;
    }
    Some((over, about.len(), about.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH)))
}

fn size_of_still(path: &Path) -> Option<(u32, u32)> {
    crate::toolkit::picture::reader(path).ok()?.into_dimensions().ok()
}

pub fn over_quickly(path: &Path) -> Over {
    let Some((mut over, ..)) = by_size(path) else { return Over::default() };
    if !over.any() && !moving_one(path) {
        over.pixels = size_of_still(path).is_some_and(|(w, h)| !fits(w, h));
    }
    over
}
