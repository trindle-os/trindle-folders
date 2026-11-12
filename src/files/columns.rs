//! The List view's columns, chosen by the person: which there are, their

use std::cell::RefCell;
use std::collections::HashMap;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::toolkit::t;

use super::far;
use super::icons::LINE_PAD;

use super::kinds::{kind_name, when_text};
use super::media::Media;
use super::places::Entry;
use super::view::{items_words, size_words};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Col {
    Size,
    Type,
    Date,
    Made,
    Items,
    Rights,
    Owner,
    Group,
    Title,
    Artist,
    Album,
    Track,
    Length,
    Camera,
    Dimensions,
    Pages,
}

impl Col {
    pub const ALL: [Col; 16] = [Col::Size, Col::Type, Col::Date, Col::Made, Col::Items, Col::Rights, Col::Owner, Col::Group,
                                Col::Title, Col::Artist, Col::Album, Col::Track, Col::Length, Col::Camera, Col::Dimensions, Col::Pages];
    pub const USUAL: [Col; 2] = [Col::Size, Col::Date];

    pub fn word(self) -> &'static str {
        match self {
            Col::Size => t!("folders-columns-size"),
            Col::Type => t!("folders-columns-type"),
            Col::Date => t!("folders-columns-date"),
            Col::Made => t!("folders-columns-made"),
            Col::Items => t!("folders-columns-items"),
            Col::Rights => t!("folders-columns-rights"),
            Col::Owner => t!("folders-columns-owner"),
            Col::Group => t!("folders-columns-group"),
            Col::Title => t!("folders-columns-title"),
            Col::Artist => t!("folders-columns-artist"),
            Col::Album => t!("folders-columns-album"),
            Col::Track => t!("folders-columns-track"),
            Col::Length => t!("folders-columns-length"),
            Col::Camera => t!("folders-columns-camera"),
            Col::Dimensions => t!("folders-columns-dimensions"),
            Col::Pages => t!("folders-columns-pages"),
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Col::Size => "size",
            Col::Type => "type",
            Col::Date => "date",
            Col::Made => "made",
            Col::Items => "items",
            Col::Rights => "rights",
            Col::Owner => "owner",
            Col::Group => "group",
            Col::Title => "title",
            Col::Artist => "artist",
            Col::Album => "album",
            Col::Track => "track",
            Col::Length => "length",
            Col::Camera => "camera",
            Col::Dimensions => "dimensions",
            Col::Pages => "pages",
        }
    }

    pub fn named(word: &str) -> Option<Col> {
        Col::ALL.into_iter().find(|c| c.key() == word)
    }

    pub fn width(self) -> f32 {
        match self {
            Col::Size => 92.0,
            Col::Type => 160.0,
            Col::Date | Col::Made => 116.0,
            Col::Items => 76.0,
            Col::Rights => 92.0,
            Col::Owner | Col::Group => 80.0,
            Col::Title | Col::Artist | Col::Album | Col::Camera => 130.0,
            Col::Track => 56.0,
            Col::Pages => 76.0,
            Col::Length => 64.0,
            Col::Dimensions => 96.0,
        }
    }

    pub fn media(self) -> bool {
        matches!(self, Col::Title | Col::Artist | Col::Album | Col::Track | Col::Length | Col::Camera | Col::Dimensions | Col::Pages)
    }

    fn reads(self) -> bool {
        matches!(self, Col::Made | Col::Items | Col::Rights | Col::Owner | Col::Group)
    }
}

#[derive(Clone, Default)]
struct Facts {
    mode: Option<u32>,
    uid: Option<u32>,
    gid: Option<u32>,
    items: Option<usize>,
    made: Option<i64>,
}

#[derive(Default)]
pub struct Cache {
    facts: RefCell<HashMap<PathBuf, Facts>>,
    kinds: RefCell<HashMap<String, String>>,
    owners: RefCell<HashMap<u32, String>>,
    groups: RefCell<HashMap<u32, String>>,
    sizes: RefCell<HashMap<PathBuf, (u64, bool)>>,
    media: RefCell<HashMap<(PathBuf, i64), Media>>,
}

impl Cache {
    pub fn forget(&self, entries: &[Entry]) {
        let mut f = self.facts.borrow_mut();
        for e in entries {
            f.remove(&e.path);
        }
    }

    pub fn media(&self, e: &Entry) -> Option<Media> {
        self.media.borrow().get(&(e.path.clone(), e.when)).cloned()
    }

    pub fn set_media(&self, path: PathBuf, when: i64, media: Media) {
        self.media.borrow_mut().insert((path, when), media);
    }

    pub fn size(&self, folder: &Path) -> Option<(u64, bool)> {
        self.sizes.borrow().get(folder).copied()
    }

    pub fn set_size(&self, folder: &Path, bytes: u64, done: bool) {
        self.sizes.borrow_mut().insert(folder.to_path_buf(), (bytes, done));
    }

    pub fn stale_sizes(&self) {
        for v in self.sizes.borrow_mut().values_mut() {
            v.1 = false;
        }
    }

    pub fn sized_folders(&self) -> Vec<PathBuf> {
        self.sizes.borrow().keys().cloned().collect()
    }

    pub fn stale_round(&self, folder: &Path) {
        for (k, v) in self.sizes.borrow_mut().iter_mut() {
            if k.starts_with(folder) || folder.starts_with(k) {
                v.1 = false;
            }
        }
    }

    pub fn stale_above(&self, folder: &Path) {
        let mut sizes = self.sizes.borrow_mut();
        for up in folder.ancestors() {
            if let Some(v) = sizes.get_mut(up) {
                v.1 = false;
            }
        }
    }

    fn facts(&self, e: &Entry, items: bool) -> Facts {
        if let Some(f) = self.facts.borrow().get(&e.path) {
            if !items || f.items.is_some() || !e.folder {
                return f.clone();
            }
        }
        let f = if far::is_far(&e.path) {
            Facts::default()
        } else {
            let (path, folder) = (e.path.clone(), items && e.folder);
            let Some(f) = super::reach::within(&e.path, move || {
            let meta = std::fs::symlink_metadata(&path).ok();
            Facts {
                mode: meta.as_ref().map(|m| m.mode() & 0o7777),
                uid: meta.as_ref().map(|m| m.uid()),
                gid: meta.as_ref().map(|m| m.gid()),
                made: meta.as_ref().and_then(|m| m.created().ok()).and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs() as i64),
                items: folder.then(|| std::fs::read_dir(&path).map(|d| d.count()).ok()).flatten(),
            }
            }) else {
                return Facts::default();
            };
            f
        };
        self.facts.borrow_mut().insert(e.path.clone(), f.clone());
        f
    }

    fn kind(&self, mime: &str) -> String {
        self.kinds.borrow_mut().entry(mime.to_string()).or_insert_with(|| kind_name(mime)).clone()
    }

    pub fn kind_of(&self, e: &Entry) -> String {
        super::kinds::said_kind(e.folder, || e.kind.as_deref().map(|k| self.kind(k)).unwrap_or_default())
    }

    pub fn kind_beside(&self, e: &Entry) -> String {
        super::kinds::said_kind_beside(e.folder, &e.name, || e.kind.as_deref().map(|k| self.kind(k)).unwrap_or_default())
    }

    pub fn cells(&self, e: &Entry, cols: &[Col]) -> Vec<String> {
        let facts = if cols.iter().any(|c| c.reads()) { self.facts(e, cols.contains(&Col::Items)) } else { Default::default() };
        cols.iter().map(|c| match c {
            Col::Size if e.folder => match self.size(&e.path) {
                Some((bytes, true)) => size_words(bytes),
                Some((bytes, false)) => format!("{}\u{2026}", size_words(bytes)),
                None => String::new(),
            },
            Col::Size => size_words(e.size),
            Col::Type => self.kind_of(e),
            Col::Date => when_text(e.when),
            Col::Made => facts.made.map(when_text).unwrap_or_default(),
            Col::Items => facts.items.map(|n| items_words(n as u64)).unwrap_or_default(),
            Col::Rights => facts.mode.map(letters).unwrap_or_default(),
            Col::Owner => facts.uid.map(|u| self.owners.borrow_mut().entry(u).or_insert_with(|| owner_name(u)).clone()).unwrap_or_default(),
            Col::Group => facts.gid.map(|g| self.groups.borrow_mut().entry(g).or_insert_with(|| group_name(g)).clone()).unwrap_or_default(),
            media => {
                let m = if e.folder { None } else { self.media(e) };
                let m = m.unwrap_or_default();
                match media {
                    Col::Title => m.title.unwrap_or_default(),
                    Col::Artist => m.artist.unwrap_or_default(),
                    Col::Album => m.album.unwrap_or_default(),
                    Col::Track => m.track.unwrap_or_default(),
                    Col::Length => m.length.map(super::media::length_words).unwrap_or_default(),
                    Col::Camera => m.camera.unwrap_or_default(),
                    Col::Dimensions => m.dimensions.map(|(w, h)| format!("{w} \u{d7} {h}")).unwrap_or_default(),
                    _ => m.pages.map(|n| t!("folders-columns-pages-count", n = n)).unwrap_or_default(),
                }
            }
        }).collect()
    }
}

impl Cache {
    pub fn order(&self, a: &Entry, b: &Entry, col: Col) -> std::cmp::Ordering {
        let media = |e: &Entry| if e.folder { Media::default() } else { self.media(e).unwrap_or_default() };
        let figure = |e: &Entry| -> Option<u64> {
            match col {
                Col::Size => Some(e.size),
                Col::Date => Some(e.when.max(0) as u64),
                Col::Made => self.facts(e, false).made.map(|t| t.max(0) as u64),
                Col::Items => self.facts(e, true).items.map(|n| n as u64),
                Col::Rights => self.facts(e, false).mode.map(u64::from),
                Col::Track => media(e).track.and_then(|t| t.chars().take_while(char::is_ascii_digit).collect::<String>().parse().ok()),
                Col::Length => media(e).length,
                Col::Dimensions => media(e).dimensions.map(|(w, h)| w as u64 * h as u64),
                Col::Pages => media(e).pages.map(u64::from),
                _ => None,
            }
        };
        match col {
            Col::Size | Col::Date | Col::Made | Col::Items | Col::Rights | Col::Track | Col::Length | Col::Dimensions | Col::Pages => figure(a).cmp(&figure(b)),
            _ => {
                let words = |e: &Entry| crate::toolkit::ahead::fold(&self.cells(e, &[col]).pop().unwrap_or_default());
                words(a).cmp(&words(b))
            }
        }
    }
}

pub const NAME_LEAST: f32 = 140.0;

pub fn fitting(cols: &[(Col, f32)], width: f32) -> Vec<(Col, f32)> {
    let mut room = width - 3.0 * LINE_PAD - crate::toolkit::theme::sheet().rich_icon - NAME_LEAST;
    cols.iter().copied().take_while(|(_, w)| {
        room -= w;
        room >= 0.0
    }).collect()
}

pub fn letters(mode: u32) -> String {
    (0..9).rev().map(|bit| if mode & (1 << bit) == 0 { '-' } else { ['x', 'w', 'r'][bit % 3] }).collect()
}

pub fn owner_name(uid: u32) -> String {
    let mut buf = vec![0u8; 4096];
    let mut pw: libc::passwd = unsafe { std::mem::zeroed() };
    let mut found: *mut libc::passwd = std::ptr::null_mut();
    let rc = unsafe { libc::getpwuid_r(uid, &mut pw, buf.as_mut_ptr().cast(), buf.len(), &mut found) };
    if rc != 0 || found.is_null() || pw.pw_name.is_null() {
        return uid.to_string();
    }
    unsafe { std::ffi::CStr::from_ptr(pw.pw_name) }.to_string_lossy().into_owned()
}

pub fn group_name(gid: u32) -> String {
    let mut buf = vec![0u8; 16384];
    let mut gr: libc::group = unsafe { std::mem::zeroed() };
    let mut found: *mut libc::group = std::ptr::null_mut();
    let rc = unsafe { libc::getgrgid_r(gid, &mut gr, buf.as_mut_ptr().cast(), buf.len(), &mut found) };
    if rc != 0 || found.is_null() || gr.gr_name.is_null() {
        return gid.to_string();
    }
    unsafe { std::ffi::CStr::from_ptr(gr.gr_name) }.to_string_lossy().into_owned()
}

pub fn read(text: &str) -> Vec<Col> {
    text.split_whitespace().filter_map(Col::named).collect()
}

pub fn write(cols: &[Col]) -> String {
    cols.iter().map(|c| c.key()).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn columns_are_kept_as_words() {
        let cols = [Col::Date, Col::Owner, Col::Size];
        assert_eq!(read(&write(&cols)), cols);
        assert_eq!(read("size nonsense rights"), [Col::Size, Col::Rights]);
        assert_eq!(letters(0o754), "rwxr-xr--");
        let usual = |cols: &[Col]| cols.iter().map(|c| (*c, c.width())).collect::<Vec<_>>();
        let fits = |cols: &[Col], width: f32| fitting(&usual(cols), width).into_iter().map(|c| c.0).collect::<Vec<_>>();
        assert_eq!(fits(&Col::ALL, 2000.0), Col::ALL);
        assert_eq!(fits(&Col::USUAL, 300.0), [Col::Size]);
        assert!(fits(&Col::USUAL, 100.0).is_empty());
        assert!(fitting(&[(Col::Size, 200.0), (Col::Date, 106.0)], 400.0).is_empty());
    }

    #[test]
    fn any_column_orders_what_it_shows() {
        use std::cmp::Ordering::{Equal, Greater, Less};
        let dir = std::env::temp_dir().join(format!("trindle-files-order-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("few")).unwrap();
        std::fs::create_dir_all(dir.join("many")).unwrap();
        std::fs::write(dir.join("few/a"), b"a").unwrap();
        for n in ["a", "b", "c"] {
            std::fs::write(dir.join("many").join(n), b"x").unwrap();
        }
        let entry = |name: &str, folder: bool, size: u64, when: i64| Entry { name: name.into(), path: dir.join(name), folder, size, when, kind: None };
        let (few, many) = (entry("few", true, 0, 5), entry("many", true, 0, 9));
        let cache = Cache::default();
        assert_eq!(cache.order(&few, &many, Col::Items), Less);
        assert_eq!(cache.order(&many, &few, Col::Date), Greater);
        assert_eq!(cache.order(&few, &many, Col::Owner), Equal);
        let (one, two, three) = (entry("one.mp3", false, 10, 1), entry("two.mp3", false, 20, 1), entry("three.mp3", false, 30, 1));
        cache.set_media(one.path.clone(), 1, Media { artist: Some("\u{c9}lan".into()), track: Some("2/12".into()), length: Some(200), ..Media::default() });
        cache.set_media(two.path.clone(), 1, Media { artist: Some("abba".into()), track: Some("10".into()), length: Some(90), ..Media::default() });
        assert_eq!(cache.order(&one, &two, Col::Artist), Greater, "accents and capitals aside, Elan comes after Abba");
        assert_eq!(cache.order(&one, &two, Col::Track), Less, "tracks by their number, not their letters");
        assert_eq!(cache.order(&one, &two, Col::Length), Greater);
        assert_eq!(cache.order(&three, &two, Col::Length), Less);
        assert_eq!(cache.order(&one, &two, Col::Size), Less);
        let made = std::fs::metadata(dir.join("few")).unwrap().created().ok();
        assert_eq!(cache.cells(&few, &[Col::Made])[0].is_empty(), made.is_none());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_folder_counts_its_items() {
        let dir = std::env::temp_dir().join(format!("trindle-files-columns-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("box")).unwrap();
        std::fs::write(dir.join("box/a"), b"a").unwrap();
        std::fs::write(dir.join("box/b"), b"b").unwrap();
        let e = Entry { name: "box".into(), path: dir.join("box"), folder: true, size: 0, when: 0, kind: None };
        let cache = Cache::default();
        let got = cache.cells(&e, &[Col::Size, Col::Type, Col::Items, Col::Rights]);
        assert_eq!(got[..3], ["", "", "2 items"], "a folder's Type cell is empty: its icon says it");
        let other = Entry { name: "bin".into(), path: dir.join("bin"), folder: true, size: 0, when: 0, kind: None };
        assert_eq!(cache.kind_of(&other), "", "never the word Folder");
        assert_eq!(got[3].len(), 9);
        std::fs::write(dir.join("box/c"), b"c").unwrap();
        assert_eq!(cache.cells(&e, &[Col::Items])[0], "2 items");
        cache.forget(std::slice::from_ref(&e));
        assert_eq!(cache.cells(&e, &[Col::Items])[0], "3 items");
        cache.set_size(&e.path, 2_000, true);
        assert_eq!(cache.cells(&e, &[Col::Size])[0], "2.0 KB");
        cache.stale_above(&e.path.join("deeper"));
        assert_eq!(cache.cells(&e, &[Col::Size])[0], "2.0 KB\u{2026}");
        let _ = std::fs::remove_dir_all(dir);
    }
}
