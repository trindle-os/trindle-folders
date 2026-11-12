//! Thumbnails in the Icons view: each picture shown is asked of the

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use tiny_skia::Pixmap;

use super::app::{Files, Msg, View};
use super::far;
use super::places::Where;
use super::thumbs::{can_thumb, Maker, Size, Thumbs};

const MOST: usize = 400;
#[cfg(test)]
const ABOVE: usize = MOST / 4;

const NEAR: usize = 3;

#[derive(Default)]
pub struct Seen {
    top: Cell<Option<usize>>,
    from: Cell<usize>,
    most: Cell<usize>,
    made: RefCell<Vec<(usize, usize)>>,
    screen: Cell<usize>,
    all: Cell<usize>,
    asked: Cell<bool>,
}

impl Seen {
    pub fn start(&self) {
        self.top.set(None);
        self.asked.set(false);
    }

    pub fn made(&self, n: usize, screen: usize, also: &[usize]) -> Vec<(usize, usize)> {
        let screen = screen.max(1);
        let top = self.top.get().unwrap_or(0).min(n);
        let mut runs = vec![(top.saturating_sub(NEAR * screen), (top + (NEAR + 1) * screen).min(n)), (0, (2 * screen).min(n))];
        runs.extend(also.iter().filter(|i| **i < n).map(|i| (i.saturating_sub(screen), (i + screen + 1).min(n))));
        self.screen.set(screen);
        self.all.set(n);
        *self.made.borrow_mut() = runs.clone();
        runs
    }

    pub fn strip_painted(&self, start: usize, end: usize) -> bool {
        if self.asked.get() {
            return false;
        }
        let (screen, n) = (self.screen.get(), self.all.get());
        let made = self.made.borrow();
        if made.is_empty() {
            return false;
        }
        let again = match made.iter().filter(|(a, b)| start < *b && end > *a).map(|(a, b)| (*a, *b)).reduce(|x, y| (x.0.min(y.0), x.1.max(y.1))) {
            None => true,
            Some((a, b)) => self.top.get() == Some(start) && ((a > 0 && start < a + screen) || (b < n && start + 2 * screen > b)),
        };
        self.asked.set(again);
        again
    }

    pub fn painted(&self, first: usize) -> bool {
        if self.top.get().is_some() {
            return false;
        }
        self.top.set(Some(first));
        let (from, most) = (self.from.get(), self.most());
        let above = most / 4;
        if first >= from && first < from + most / 2 && (first >= above || from == 0) {
            return false;
        }
        self.from.set(first.saturating_sub(above));
        true
    }

    #[cfg(test)]
    pub fn asked(&self) -> bool {
        self.asked.get()
    }

    #[cfg(test)]
    pub fn covered(&self) -> bool {
        let Some(top) = self.top.get() else { return false };
        let end = (top + self.screen.get()).min(self.all.get());
        self.made.borrow().iter().any(|(a, b)| *a <= top && end <= *b)
    }

    pub fn top(&self) -> Option<usize> {
        self.top.get()
    }

    fn most(&self) -> usize {
        match self.most.get() {
            0 => MOST,
            most => most,
        }
    }

    pub fn keep(&self, most: usize) {
        self.most.set(most.clamp(1, MOST));
    }

    fn window(&self, n: usize) -> std::ops::Range<usize> {
        let from = self.from.get().min(n);
        from..(from + self.most()).min(n)
    }
}

pub fn kept_pixels(icon: f32) -> u32 {
    ((icon * 2.0).round() as u32).min(Size::Large.pixels())
}

pub fn window_for(icon: f32, in_sight: usize) -> usize {
    let (usual, now) = (kept_pixels(super::icons::ICON_USUAL) as usize, kept_pixels(icon) as usize);
    (MOST * usual * usual / (now * now).max(1)).max(3 * in_sight).min(MOST)
}

#[derive(Default)]
pub struct Pictures {
    maker: Option<Maker>,
    size: Option<Size>,
    asked: HashSet<PathBuf>,
    ready: HashMap<PathBuf, Option<Rc<Pixmap>>>,
    #[cfg(test)]
    pub inline: Option<Thumbs>,
}

impl Files {
    fn pictures_shown(&self) -> impl Iterator<Item = &super::places::Entry> {
        self.cols.iter().enumerate()
            .filter(|(p, pane)| self.shown_pane(*p) && self.view_of(*p) == View::Icons
                    && matches!(pane.at, Where::Folder(_) | Where::Recent | Where::Found(..)))
            .flat_map(|(_, pane)| pane.entries[pane.seen.window(pane.entries.len())].iter())
    }

    fn pictures_asked_about(&self) -> impl Iterator<Item = &super::places::Entry> {
        self.jobs.iter().filter_map(|j| j.asking.as_ref()).flat_map(|a| a.coming.iter().chain(a.there.iter()))
    }

    fn pictures_needed(&self) -> Vec<PathBuf> {
        self.pictures_shown().chain(self.pictures_asked_about())
            .filter(|e| !e.folder && !far::is_far(&e.path))
            .filter(|e| !self.pictures.ready.contains_key(&e.path) && !self.pictures.asked.contains(&e.path) && can_thumb(&e.path))
            .map(|e| e.path.clone()).collect()
    }

    fn pictures_kept(&self) -> HashSet<PathBuf> {
        self.pictures_shown().chain(self.pictures_asked_about()).map(|e| e.path.clone()).collect()
    }

    pub(super) fn pictures_wanted(&mut self) {
        if self.as_root {
            return;
        }
        let want = self.pictures_needed();
        if want.is_empty() {
            return;
        }
        let kept = self.pictures_kept();
        if self.pictures.asked.iter().any(|a| !kept.contains(a)) {
            if let Some(maker) = &self.pictures.maker {
                maker.cancel();
            }
            self.pictures.asked.clear();
        }
        self.pictures.ready.retain(|path, _| kept.contains(path));
        let want = self.pictures_needed();
        if self.pictures.maker.is_none() {
            if let Some(post) = self.post.clone() {
                let size = self.thumb_size();
                self.pictures.size = Some(size);
                self.pictures.maker = Some(Maker::new(Thumbs::new().sized(size), move |file: &Path, thumb| {
                    post.send(Msg::Pictured(file.to_path_buf(), thumb));
                }));
            }
        }
        for file in want {
            self.pictures.asked.insert(file.clone());
            if let Some(maker) = &self.pictures.maker {
                maker.ask(&file);
                continue;
            }
            #[cfg(test)]
            if let Some(thumbs) = self.pictures.inline.clone() {
                let thumb = thumbs.sized(self.thumb_size()).get(&file);
                self.pictured(file, thumb);
            }
        }
    }

    pub(super) fn pictures_moved(&mut self) {
        if let Some(maker) = &self.pictures.maker {
            maker.cancel();
        }
        self.pictures.asked.clear();
        let shown = self.pictures_kept();
        self.pictures.ready.retain(|path, _| shown.contains(path));
    }

    fn thumb_size(&self) -> Size {
        Size::for_pixels(kept_pixels(self.kept.icon()))
    }

    pub(super) fn pictures_resized(&mut self) {
        if let Some(maker) = &self.pictures.maker {
            maker.cancel();
        }
        self.pictures.asked.clear();
        self.pictures.ready.clear();
        if self.pictures.size != Some(self.thumb_size()) {
            self.pictures.maker = None;
        }
    }

    pub(super) fn pictured(&mut self, file: PathBuf, thumb: Option<PathBuf>) {
        let folder = self.thumb_size().folder();
        if thumb.as_deref().is_some_and(|t| !t.parent().is_some_and(|d| d.ends_with(folder))) {
            return;
        }
        self.pictures.asked.remove(&file);
        let kept = kept_pixels(self.kept.icon());
        let ready = thumb.as_deref().and_then(crate::toolkit::picture::load)
            .map(|img| if img.width() > kept || img.height() > kept { img.thumbnail(kept, kept) } else { img })
            .and_then(|img| crate::toolkit::picture::to_pixmap(&img))
            .map(Rc::new);
        self.pictures.ready.insert(file, ready);
    }

    pub(super) fn picture_of(&self, path: &Path) -> Option<Rc<Pixmap>> {
        self.pictures.ready.get(path).cloned().flatten()
    }
}

#[cfg(test)]
mod tests {
    use crate::files::app::{Files, Msg, View};
    use crate::files::places::Where;
    use crate::files::thumbs::Thumbs;
    use crate::files::trash::Bin;
    use crate::toolkit::ui::App;

    #[test]
    fn pictures_in_the_icons_view_get_thumbnails() {
        let dir = std::env::temp_dir().join(format!("trindle-files-pictures-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("photos")).unwrap();
        image::RgbImage::from_pixel(400, 200, image::Rgb([200, 40, 40])).save(dir.join("photos/red.png")).unwrap();
        std::fs::write(dir.join("photos/notes.txt"), b"hello").unwrap();

        let mut f = Files::new(vec![dir.join("photos")]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.pictures.inline = Some(Thumbs::at(&dir.join("thumbnails")));
        f.go(1, Where::Folder(dir.join("photos")));
        f.go(0, Where::Folder(dir.join("photos")));
        f.update(Msg::Changed);
        assert!(f.picture_of(&dir.join("photos/red.png")).is_none(), "the List view has none");

        f.view = View::Icons;
        f.update(Msg::Changed);
        let red = f.picture_of(&dir.join("photos/red.png")).expect("a thumbnail");
        assert_eq!((red.width(), red.height()), (96, 48));
        assert!(f.picture_of(&dir.join("photos/notes.txt")).is_none());
        assert!(dir.join("thumbnails/normal").is_dir() && !dir.join("thumbnails/large").exists());

        use crate::files::zoom::Zoom;
        f.update(Msg::Zoom(Zoom::In, None));
        let red = f.picture_of(&dir.join("photos/red.png")).expect("a thumbnail at 64");
        assert_eq!((f.kept.icon(), red.width(), red.height()), (64.0, 128, 64));
        assert!(!dir.join("thumbnails/large").exists(), "the usual size still does");
        f.update(Msg::Zoom(Zoom::In, None));
        let red = f.picture_of(&dir.join("photos/red.png")).expect("a thumbnail at 96");
        assert_eq!((f.kept.icon(), red.width(), red.height()), (96.0, 192, 96));
        assert!(dir.join("thumbnails/large").is_dir());
        for _ in 0..4 {
            f.update(Msg::Zoom(Zoom::In, None));
        }
        let red = f.picture_of(&dir.join("photos/red.png")).expect("a thumbnail at 256");
        assert_eq!((f.kept.icon(), red.width(), red.height()), (256.0, 256, 128), "never more than the largest made");
        f.update(Msg::Zoom(Zoom::Usual, None));
        let late = dir.join("thumbnails/large/late.png");
        f.pictures.ready.clear();
        f.pictured(dir.join("photos/red.png"), Some(late));
        assert!(f.picture_of(&dir.join("photos/red.png")).is_none());
        f.update(Msg::Changed);
        let red = f.picture_of(&dir.join("photos/red.png")).expect("a thumbnail at the usual size again");
        assert_eq!((red.width(), red.height()), (96, 48));

        f.go(0, Where::Folder(dir.clone()));
        f.go(1, Where::Folder(dir.clone()));
        assert!(f.picture_of(&dir.join("photos/red.png")).is_none());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_window_follows_the_top_in_sight() {
        use super::{Seen, ABOVE, MOST};
        let seen = Seen::default();
        seen.start();
        assert!(!seen.painted(0));
        assert!(!seen.painted(40), "only the first row painted counts");
        assert_eq!(seen.window(1000), 0..MOST);
        seen.start();
        assert!(!seen.painted(MOST / 2 - 1));
        seen.start();
        assert!(seen.painted(700));
        assert_eq!(seen.window(1000), 700 - ABOVE..1000);
        seen.start();
        assert!(!seen.painted(700), "settled once moved");
        seen.start();
        assert!(seen.painted(10));
        assert_eq!(seen.window(1000), 0..MOST);
        assert_eq!(seen.window(50), 0..50);
        use super::{kept_pixels, window_for};
        assert_eq!((kept_pixels(32.0), kept_pixels(48.0), kept_pixels(128.0), kept_pixels(256.0)), (64, 96, 256, 256));
        assert_eq!((window_for(32.0, 60), window_for(48.0, 40), window_for(64.0, 30)), (MOST, MOST, 225));
        assert_eq!((window_for(256.0, 6), window_for(256.0, 40), window_for(256.0, 500)), (56, 120, MOST));
        seen.keep(56);
        assert_eq!(seen.window(1000), 0..56);
        seen.start();
        assert!(seen.painted(40), "past the smaller window's half");
        assert_eq!(seen.window(1000), 26..82);
    }

    #[test]
    fn thumbnails_are_asked_for_round_what_shows_in_a_big_folder() {
        let dir = std::env::temp_dir().join(format!("trindle-files-pictures-many-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("photos")).unwrap();
        for i in 0..1000 {
            std::fs::write(dir.join(format!("photos/p{i:04}.png")), b"\x89PNG\r\n\x1a\n").unwrap();
        }
        let mut f = Files::new(vec![dir.join("photos")]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.view = View::Icons;
        f.go(1, Where::Folder(dir.clone()));
        f.go(0, Where::Folder(dir.join("photos")));
        let name = |p: &std::path::PathBuf| p.file_name().unwrap().to_string_lossy().into_owned();
        let want = f.pictures_needed();
        assert_eq!(want.len(), super::MOST);
        assert_eq!(name(&want[0]), "p0000.png");

        f.pictures_wanted();
        let seen = f.cols[0].seen.clone();
        seen.start();
        assert!(seen.painted(800));
        f.update(Msg::Resized);
        let asked: Vec<String> = f.pictures.asked.iter().map(name).collect();
        assert!(asked.contains(&"p0999.png".to_string()));
        assert!(asked.contains(&format!("p{:04}.png", 800 - super::ABOVE)));
        assert!(!asked.contains(&"p0000.png".to_string()), "the top is let go");
        assert_eq!(asked.len(), 1000 - (800 - super::ABOVE));
        let _ = std::fs::remove_dir_all(dir);
    }
}
