//! The three plain choices beside Find's box: what kind of thing, how big,

use std::path::Path;
use std::time::UNIX_EPOCH;

use super::kinds::Mime;
use super::places::Entry;
use crate::common::clock::Local;
use crate::toolkit::t;

const MB: u64 = 1_000_000;
const GB: u64 = 1_000 * MB;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Kind {
    #[default]
    Anything,
    Folders,
    Pictures,
    Documents,
    Music,
    Video,
    Archives,
}

impl Kind {
    pub const ALL: [Kind; 7] = [Kind::Anything, Kind::Folders, Kind::Pictures, Kind::Documents, Kind::Music, Kind::Video, Kind::Archives];

    pub fn word(self) -> &'static str {
        match self {
            Kind::Anything => t!("folders-narrow-kind-anything"),
            Kind::Folders => t!("folders-narrow-kind-folders"),
            Kind::Pictures => t!("folders-narrow-kind-pictures"),
            Kind::Documents => t!("folders-narrow-kind-documents"),
            Kind::Music => t!("folders-narrow-kind-music"),
            Kind::Video => t!("folders-narrow-kind-video"),
            Kind::Archives => t!("folders-narrow-kind-archives"),
        }
    }

    fn group(self) -> Option<usize> {
        match self {
            Kind::Anything => None,
            Kind::Folders => Some(0),
            Kind::Documents => Some(1),
            Kind::Pictures => Some(2),
            Kind::Music => Some(3),
            Kind::Video => Some(4),
            Kind::Archives => Some(5),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    #[default]
    Any,
    Over1Mb,
    Over100Mb,
    Over1Gb,
}

impl Size {
    pub const ALL: [Size; 4] = [Size::Any, Size::Over1Mb, Size::Over100Mb, Size::Over1Gb];

    pub fn word(self) -> &'static str {
        match self {
            Size::Any => t!("folders-narrow-size-any"),
            Size::Over1Mb => t!("folders-narrow-size-1mb"),
            Size::Over100Mb => t!("folders-narrow-size-100mb"),
            Size::Over1Gb => t!("folders-narrow-size-1gb"),
        }
    }

    fn over(self) -> Option<u64> {
        match self {
            Size::Any => None,
            Size::Over1Mb => Some(MB),
            Size::Over100Mb => Some(100 * MB),
            Size::Over1Gb => Some(GB),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Changed {
    #[default]
    Any,
    Today,
    Week,
    Month,
    Year,
}

impl Changed {
    pub const ALL: [Changed; 5] = [Changed::Any, Changed::Today, Changed::Week, Changed::Month, Changed::Year];

    pub fn word(self) -> &'static str {
        match self {
            Changed::Any => t!("folders-narrow-changed-any"),
            Changed::Today => t!("folders-narrow-changed-today"),
            Changed::Week => t!("folders-narrow-changed-week"),
            Changed::Month => t!("folders-narrow-changed-month"),
            Changed::Year => t!("folders-narrow-changed-year"),
        }
    }

    fn holds(self, when: Local, now: Local) -> bool {
        let (day, today) = (when.day_number(), now.day_number());
        if day >= today {
            return true;
        }
        match self {
            Changed::Any => true,
            Changed::Today => false,
            Changed::Week => day >= today - (today + 3).rem_euclid(7),
            Changed::Month => (when.year(), when.month()) == (now.year(), now.month()),
            Changed::Year => when.year() == now.year(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Narrow {
    pub kind: Kind,
    pub size: Size,
    pub changed: Changed,
}

impl Narrow {
    pub fn is_any(&self) -> bool {
        *self == Narrow::default()
    }

    pub fn wants_kinds(&self) -> bool {
        !matches!(self.kind, Kind::Anything | Kind::Folders)
    }

    pub fn fits(&self, e: &Entry, now: Local) -> bool {
        self.kind.group().is_none_or(|g| super::groups::kind_of(e.folder, e.kind.as_deref().unwrap_or(""), &e.path) == g)
            && self.size.over().is_none_or(|least| !e.folder && e.size > least)
            && self.changed.holds(Local::at(e.when), now)
    }

    pub fn fits_found(&self, path: &Path, folder: bool, size: u64, mime: Option<&Mime>, now: Local) -> bool {
        if let Some(g) = self.kind.group() {
            let is = if folder {
                g == 0
            } else if g == 0 {
                false
            } else {
                let name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
                let kind = mime.and_then(|m| m.of(&name)).unwrap_or_default();
                super::groups::kind_of(false, &kind, path) == g
            };
            if !is {
                return false;
            }
        }
        if self.size.over().is_some_and(|least| folder || size <= least) {
            return false;
        }
        if self.changed == Changed::Any {
            return true;
        }
        let when = std::fs::symlink_metadata(path).and_then(|m| m.modified()).ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs() as i64);
        self.changed.holds(Local::at(when), now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn entry(name: &str, folder: bool, size: u64, when: i64, kind: Option<&str>) -> Entry {
        Entry { name: name.into(), path: PathBuf::from("/made/up").join(name), folder, size, when, kind: kind.map(str::to_string) }
    }

    #[test]
    fn each_choice_lets_through_what_it_says() {
        let noon = 1_791_979_200;
        let now = Local::at(noon);
        let day = 86_400;
        let photo = entry("beach.jpg", false, 3 * MB, noon - 3600, Some("image/jpeg"));
        let letter = entry("letter.odt", false, 20_000, noon - 2 * day, Some("application/vnd.oasis.opendocument.text"));
        let film = entry("film.mkv", false, 2 * GB, noon - 20 * day, Some("video/x-matroska"));
        let song = entry("song.mp3", false, 5 * MB, noon - 200 * day, Some("audio/mpeg"));
        let old = entry("old.zip", false, 150 * MB, noon - 400 * day, Some("application/zip"));
        let folder = entry("Papers", true, 0, noon - day, None);
        let all = [&photo, &letter, &film, &song, &old, &folder];
        let names = |n: Narrow| all.iter().filter(|e| n.fits(e, now)).map(|e| e.name.as_str()).collect::<Vec<_>>();
        assert!(Narrow::default().is_any());
        assert_eq!(names(Narrow::default()).len(), 6);
        let kind = |kind| Narrow { kind, ..Narrow::default() };
        assert_eq!(names(kind(Kind::Folders)), ["Papers"]);
        assert_eq!(names(kind(Kind::Pictures)), ["beach.jpg"]);
        assert_eq!(names(kind(Kind::Documents)), ["letter.odt"]);
        assert_eq!(names(kind(Kind::Music)), ["song.mp3"]);
        assert_eq!(names(kind(Kind::Video)), ["film.mkv"]);
        assert_eq!(names(kind(Kind::Archives)), ["old.zip"]);
        let size = |size| Narrow { size, ..Narrow::default() };
        assert_eq!(names(size(Size::Over1Mb)), ["beach.jpg", "film.mkv", "song.mp3", "old.zip"]);
        assert_eq!(names(size(Size::Over100Mb)), ["film.mkv", "old.zip"]);
        assert_eq!(names(size(Size::Over1Gb)), ["film.mkv"]);
        assert!(!size(Size::Over1Mb).fits(&entry("just.bin", false, MB, noon, None), now), "a megabyte is not over one");
        let changed = |changed| Narrow { changed, ..Narrow::default() };
        assert_eq!(names(changed(Changed::Today)), ["beach.jpg"]);
        assert_eq!(names(changed(Changed::Week)), ["beach.jpg", "letter.odt", "Papers"]);
        assert_eq!(names(changed(Changed::Month)), ["beach.jpg", "letter.odt", "Papers"]);
        assert_eq!(names(changed(Changed::Year)), ["beach.jpg", "letter.odt", "film.mkv", "song.mp3", "Papers"]);
        assert!(changed(Changed::Today).fits(&entry("ahead.txt", false, 1, noon + 3 * day, None), now));
        let both = Narrow { kind: Kind::Video, size: Size::Over1Gb, changed: Changed::Year };
        assert_eq!(names(both), ["film.mkv"]);
        assert!(names(Narrow { changed: Changed::Week, ..both }).is_empty());
        assert_eq!(Kind::ALL.map(Kind::word), ["Anything", "Folders", "Pictures", "Documents", "Music", "Video", "Archives"]);
        assert_eq!(Size::ALL.map(Size::word), ["Any size", "Over 1 MB", "Over 100 MB", "Over 1 GB"]);
        assert_eq!(Changed::ALL.map(Changed::word), ["Any time", "Today", "This week", "This month", "This year"]);
    }
}
