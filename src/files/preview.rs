//! The built-in viewer: a picture or a text file shown over the window,

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use image::{DynamicImage, GenericImageView};
use tiny_skia::{Pixmap, PixmapPaint, Transform};

use super::app::{Files, Msg};
use super::dialogs::{AskMsg, Dialog};
use super::play::{self, Player};
use super::viewer::{self, Limits, Shown};
use super::view::size_words;
use super::work;
use crate::toolkit::event::Post;
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, drawing, label, main_action, moving, note, row, scroll, slider, space, Length, Node};

const LEAST: (f32, f32) = (420.0, 160.0);
const MARGIN: f32 = 48.0;
const FRAME: f32 = 130.0;
const TEXT_MOST: u64 = 512 << 10;
const INSIDE_MOST: u64 = 100 << 20;
const BLOCK: usize = 200;

static NEXT: AtomicU64 = AtomicU64::new(1);

type Fitted = Rc<RefCell<Option<((u32, u32), Pixmap)>>>;

pub struct Preview {
    pub file: PathBuf,
    pane: usize,
    id: u64,
    pub seen: Seen,
    pub room: (f32, f32),
    copy: Option<PathBuf>,
    fitted: Fitted,
    ticking: Option<Arc<AtomicBool>>,
    closed: Option<Arc<AtomicBool>>,
}

impl Drop for Preview {
    fn drop(&mut self) {
        if let Some(t) = &self.ticking {
            t.store(false, Ordering::Relaxed);
        }
        if let (Some(closed), Some(copy)) = (&self.closed, &self.copy) {
            closed.store(true, Ordering::Relaxed);
            let _ = std::fs::remove_file(copy);
        }
    }
}

#[cfg(test)]
impl Preview {
    pub(super) fn copy(&self) -> Option<&std::path::Path> {
        self.copy.as_deref()
    }
}

const TICK: Duration = Duration::from_millis(100);
const SECOND: Duration = Duration::from_secs(1);
const CONTROLS: f32 = 48.0;
const COVER_ROOM: f32 = 320.0;

pub enum Seen {
    Loading,
    Picture(Rc<DynamicImage>),
    Pdf { page: u32, pages: Option<u32>, drawn: Option<Rc<DynamicImage>> },
    Media { player: Box<Player>, cover: Option<Rc<DynamicImage>> },
    Font { name: String, page: Rc<DynamicImage>, installed: bool },
    Text { blocks: Vec<String>, encoding: &'static str, cut: bool, read: u64 },
    Words(String),
    Trouble(String),
}

#[derive(Clone)]
pub enum PreviewMsg {
    Toggle,
    Step(i32),
    Open,
    Loaded(u64, Arc<Mutex<Option<Shown>>>),
    More,
    Read(u64, Result<(String, u64, bool), String>),
    Page(i32),
    Drawn(u64, u32, Arc<Mutex<Option<DynamicImage>>>),
    Space,
    PlayPause,
    Seek(f32),
    Volume(f32),
    Mute,
    Tick(u64),
    InstallFont,
}

impl Files {
    pub(super) fn preview_msg(&mut self, m: PreviewMsg) {
        match m {
            PreviewMsg::Toggle => self.preview_toggle(),
            PreviewMsg::Step(by) => self.preview_step(by),
            PreviewMsg::Open => self.preview_open(),
            PreviewMsg::Loaded(id, shown) => self.preview_loaded(id, shown),
            PreviewMsg::Space => {
                let media = matches!(&self.dialog, Some(Dialog::Preview(pv)) if matches!(&pv.seen, Seen::Media { player, .. } if !player.failed));
                self.preview_msg(if media { PreviewMsg::PlayPause } else { PreviewMsg::Toggle });
            }
            PreviewMsg::PlayPause | PreviewMsg::Seek(_) | PreviewMsg::Tick(_) => self.preview_player(m),
            PreviewMsg::Volume(v) => play::set_volume(v),
            PreviewMsg::Mute => play::set_muted(!play::muted()),
            PreviewMsg::Page(by) => self.preview_page(by),
            PreviewMsg::Drawn(id, at, img) => self.preview_drawn(id, at, img),
            PreviewMsg::InstallFont => self.preview_install_font(),
            PreviewMsg::More => self.preview_more(),
            PreviewMsg::Read(id, got) => self.preview_read(id, got),
        }
    }

    fn preview_toggle(&mut self) {
        if matches!(self.dialog, Some(Dialog::Preview(_))) {
            self.dialog = None;
            return;
        }
        let p = self.side();
        let pane = &self.cols[p];
        let chosen = pane.chosen.or_else(|| pane.picked.first().copied());
        if let Some(i) = chosen.filter(|&i| viewable(pane.entries.get(i))) {
            self.preview_at(p, i);
        }
    }

    fn preview_step(&mut self, by: i32) {
        let Some(Dialog::Preview(pv)) = &self.dialog else { return };
        let p = pv.pane;
        let pane = &self.cols[p];
        let Some(now) = pane.entries.iter().position(|e| e.path == pv.file) else { return };
        let mut i = now as i64;
        loop {
            i += by.signum() as i64;
            let Some(e) = usize::try_from(i).ok().and_then(|i| pane.entries.get(i)) else { return };
            if viewable(Some(e)) {
                break;
            }
        }
        self.pane_mut(p).pick(i as usize, false);
        self.seek(p);
        self.preview_at(p, i as usize);
    }

    fn preview_open(&mut self) {
        let Some(Dialog::Preview(pv)) = &self.dialog else { return };
        let p = pv.pane;
        let i = self.cols[p].entries.iter().position(|e| e.path == pv.file);
        self.dialog = None;
        if let Some(i) = i {
            self.open(p, i);
        }
    }

    fn preview_loaded(&mut self, id: u64, shown: Arc<Mutex<Option<Shown>>>) {
        let Some(Dialog::Preview(pv)) = &mut self.dialog else { return };
        let Some(shown) = shown.lock().ok().and_then(|mut s| s.take()) else { return };
        if pv.id != id {
            return;
        }
        if let Shown::Media { info, cover } = shown {
            let (wide, tall) = most(pv.room);
            let file = pv.copy.clone().unwrap_or_else(|| pv.file.clone());
            let mut player = Box::new(Player::new(&file, info, (wide - 40.0, tall - CONTROLS)));
            player.play();
            let wake = player.wake.clone();
            pv.seen = Seen::Media { player, cover: cover.map(Rc::new) };
            if let Some(post) = self.post.clone() {
                let on = Arc::new(AtomicBool::new(true));
                pv.ticking = Some(on.clone());
                keep_time(post, on, wake, id);
            }
            return;
        }
        pv.seen = seen(shown);
        if let Seen::Font { installed, .. } = &mut pv.seen {
            *installed = super::fonts::installed(&pv.file);
        }
    }

    fn preview_player(&mut self, m: PreviewMsg) {
        let Some(Dialog::Preview(pv)) = &mut self.dialog else { return };
        let Seen::Media { player, .. } = &mut pv.seen else { return };
        match m {
            PreviewMsg::PlayPause => player.toggle(),
            PreviewMsg::Seek(to) => player.seek(f64::from(to)),
            PreviewMsg::Tick(id) if id == pv.id => player.tick(),
            _ => {}
        }
    }

    fn preview_page(&mut self, by: i32) {
        let Some(Dialog::Preview(pv)) = &mut self.dialog else { return };
        let Seen::Pdf { page, pages, drawn } = &mut pv.seen else { return };
        let next = (*page as i64 + by.signum() as i64).max(1) as u32;
        if next == *page || pages.is_some_and(|n| next > n) {
            return;
        }
        (*page, *drawn) = (next, None);
        *pv.fitted.borrow_mut() = None;
        let (file, id) = (pv.copy.clone().unwrap_or_else(|| pv.file.clone()), pv.id);
        let job = move || super::walled::pdf_page(&file, next, viewer::PDF_MOST);
        match self.post.clone() {
            Some(post) => {
                std::thread::spawn(move || post.send(Msg::Preview(PreviewMsg::Drawn(id, next, Arc::new(Mutex::new(job()))))));
            }
            None => self.preview_msg(PreviewMsg::Drawn(id, next, Arc::new(Mutex::new(job())))),
        }
    }

    fn preview_drawn(&mut self, id: u64, at: u32, img: Arc<Mutex<Option<DynamicImage>>>) {
        let Some(Dialog::Preview(pv)) = &mut self.dialog else { return };
        let Seen::Pdf { page, drawn, .. } = &mut pv.seen else { return };
        if pv.id == id && *page == at {
            match img.lock().ok().and_then(|mut i| i.take()) {
                Some(img) => *drawn = Some(Rc::new(img)),
                None => pv.seen = Seen::Trouble(t!("folders-preview-page-failed", page = at)),
            }
        }
    }

    fn preview_install_font(&mut self) {
        let Some(Dialog::Preview(pv)) = &self.dialog else { return };
        let file = pv.file.clone();
        self.install_fonts(vec![file.clone()]);
        if let Some(Dialog::Preview(pv)) = &mut self.dialog {
            if let Seen::Font { installed, .. } = &mut pv.seen {
                *installed = super::fonts::installed(&file);
            }
        }
    }

    fn preview_more(&mut self) {
        let Some(Dialog::Preview(pv)) = &self.dialog else { return };
        let Seen::Text { encoding, cut: true, read, .. } = &pv.seen else { return };
        let (file, id, from, encoding) = (pv.copy.clone().unwrap_or_else(|| pv.file.clone()), pv.id, *read, *encoding);
        let job = move || viewer::read_more(&file, from, TEXT_MOST, encoding).map_err(|e| e.to_string());
        match self.post.clone() {
            Some(post) => {
                std::thread::spawn(move || post.send(Msg::Preview(PreviewMsg::Read(id, job()))));
            }
            None => self.preview_msg(PreviewMsg::Read(id, job())),
        }
    }

    fn preview_read(&mut self, id: u64, got: Result<(String, u64, bool), String>) {
        let Some(Dialog::Preview(pv)) = &mut self.dialog else { return };
        let Seen::Text { blocks, cut, read, .. } = &mut pv.seen else { return };
        if pv.id != id {
            return;
        }
        match got {
            Ok((text, used, more)) => {
                let mut lines: Vec<String> = blocks.pop().map(|b| b.lines().map(String::from).collect()).unwrap_or_default();
                let mut new = text.split('\n').map(String::from);
                if let (Some(last), Some(first)) = (lines.last_mut(), new.next()) {
                    last.push_str(&first);
                }
                lines.extend(new);
                blocks.extend(lines.chunks(BLOCK).map(|c| c.join("\n")));
                (*read, *cut) = (*read + used, more);
            }
            Err(w) => {
                *cut = false;
                self.say_help(t!("folders-preview-rest-failed", why = &w), super::help::FILE_UNREAD);
            }
        }
    }

    pub(super) fn preview_fetched(&mut self, id: u64, got: Result<PathBuf, String>) {
        let Some(Dialog::Preview(pv)) = &mut self.dialog else { return };
        if pv.id != id {
            return;
        }
        match got {
            Ok(copy) => {
                pv.copy = Some(copy.clone());
                self.preview_load(id, move || copy);
            }
            Err(w) => pv.seen = Seen::Trouble(t!("folders-preview-fetch-failed", why = &w)),
        }
    }

    fn preview_load(&mut self, id: u64, get: impl FnOnce() -> PathBuf + Send + 'static) {
        let limits = Limits { text: TEXT_MOST, ..Limits::default() };
        let job = move || viewer::load(&get(), &limits);
        match self.post.clone() {
            Some(post) => {
                std::thread::spawn(move || post.send(Msg::Preview(PreviewMsg::Loaded(id, Arc::new(Mutex::new(Some(job())))))));
            }
            None => self.preview_msg(PreviewMsg::Loaded(id, Arc::new(Mutex::new(Some(job()))))),
        }
    }

    pub(super) fn preview_path(&mut self, p: usize, file: Option<&PathBuf>) {
        let entries = &self.cols[p].entries;
        if let Some(i) = entries.iter().position(|e| Some(&e.path) == file).filter(|&i| viewable(entries.get(i))) {
            self.preview_at(p, i);
        }
    }

    fn preview_at(&mut self, p: usize, i: usize) {
        self.preview_with(p, i, None);
    }

    pub(super) fn preview_unlocked(&mut self, p: usize, file: &std::path::Path, key: super::archive::Key) {
        if let Some(i) = self.cols[p].entries.iter().position(|e| e.path == file) {
            self.preview_with(p, i, Some(key));
        }
    }

    fn preview_with(&mut self, p: usize, i: usize, key: Option<super::archive::Key>) {
        if self.as_root {
            return self.say(t!("folders-app-root-folders-only"), false);
        }
        let pane = &self.cols[p];
        let Some(e) = pane.entries.get(i) else { return };
        let (file, id) = (e.path.clone(), NEXT.fetch_add(1, Ordering::Relaxed));
        let inside = match &pane.at {
            super::places::Where::Inside(a, _) => super::inside::inner(a, &file).map(|n| (a.clone(), n)),
            _ => None,
        };
        if let (Some((a, name)), None) = (&inside, &key) {
            if self.inside_locked(a, std::slice::from_ref(name)) && !matches!(self.dialog, Some(Dialog::Unlock(_))) {
                self.dialog = None;
                return self.unlock_ask(a.clone(), super::locked::Then::View(p, file), None);
            }
        }
        let copy = inside.as_ref().map(|(_, n)| copy_dir().join(format!("{id}-{}", n.rsplit('/').next().unwrap_or(n))));
        let closed = copy.as_ref().map(|_| Arc::new(AtomicBool::new(false)));
        let pv = Preview { file: file.clone(), pane: p, id, seen: Seen::Loading, room: self.room, copy: copy.clone(), fitted: Default::default(),
                           ticking: None, closed: closed.clone() };
        self.dialog = Some(Dialog::Preview(pv));
        if super::far::is_far(&file) {
            self.far_fetch(&file, super::far::Then::View(id));
        } else if let (Some((a, name)), Some(copy)) = (inside, copy) {
            self.preview_load(id, move || {
                let _ = std::fs::create_dir_all(copy_dir());
                let written = super::archive::read(&a, key.as_ref(), &name, INSIDE_MOST).and_then(|b| std::fs::write(&copy, b));
                if written.is_err() || closed.is_some_and(|c| c.load(Ordering::Relaxed)) {
                    let _ = std::fs::remove_file(&copy);
                }
                copy
            });
        } else {
            self.preview_load(id, move || file);
        }
    }
}

fn keep_time(post: Post<Msg>, on: Arc<AtomicBool>, wake: Arc<AtomicBool>, id: u64) {
    let _ = std::thread::Builder::new().name("folders-preview-tick".into()).spawn(move || {
        let mut last = std::time::Instant::now();
        while on.load(Ordering::Relaxed) {
            std::thread::sleep(TICK);
            if (wake.swap(false, Ordering::Relaxed) || last.elapsed() >= SECOND)
                && !post.send(Msg::Preview(PreviewMsg::Tick(id))) {
                break;
            }
            if last.elapsed() >= SECOND {
                last = std::time::Instant::now();
            }
        }
    });
}

pub(super) fn forget_viewed() {
    let base = copy_dir();
    std::thread::spawn(move || {
        if std::fs::symlink_metadata(&base).is_ok_and(|m| m.is_dir()) {
            let _ = std::fs::remove_dir_all(&base);
        }
    });
}

fn copy_dir() -> PathBuf {
    #[cfg(test)]
    let base = std::env::temp_dir().join(format!("trindle-files-viewed-{}", std::process::id()));
    #[cfg(not(test))]
    let base = crate::common::files::cache_home().join("trindle-files").join("viewed");
    base
}

fn viewable(e: Option<&super::places::Entry>) -> bool {
    e.is_some_and(|e| !e.folder && (super::far::is_far(&e.path) || !e.path.exists() || e.path.is_file()))
}

fn seen(shown: Shown) -> Seen {
    match shown {
        Shown::Picture(img) => Seen::Picture(Rc::new(img)),
        Shown::Pdf { first, pages } => Seen::Pdf { page: 1, pages, drawn: Some(Rc::new(first)) },
        Shown::Media { .. } => Seen::Loading,
        Shown::Unplayable(why) => Seen::Trouble(why.into()),
        Shown::Font { name, page } => Seen::Font { name, page: Rc::new(page), installed: false },
        Shown::Text { text, encoding, cut, read } => {
            let whole = if cut { text.as_str() } else { text.strip_suffix('\n').unwrap_or(&text) };
            let lines: Vec<&str> = whole.split('\n').collect();
            Seen::Text { blocks: lines.chunks(BLOCK).map(|c| c.join("\n")).collect(), encoding, cut, read }
        }
        Shown::Broken(what) => Seen::Trouble(t!("folders-preview-broken", what = what)),
        Shown::TooLarge { bytes, pixels } => Seen::Trouble(match pixels {
            Some(px) => t!("folders-preview-too-large-pixels", millions = format!("{:.0}", px as f64 / 1e6)),
            None => t!("folders-preview-too-large-bytes", size = size_words(bytes)),
        }),
        Shown::Other => Seen::Words(t!("folders-preview-other").into()),
        Shown::Unread(why) => Seen::Trouble(t!("folders-preview-unread", why = why)),
    }
}

fn most(room: (f32, f32)) -> (f32, f32) {
    ((room.0 - 2.0 * MARGIN).max(LEAST.0), (room.1 - 2.0 * MARGIN - FRAME).max(LEAST.1))
}

type Body = (Node<Msg>, String, f32);

pub fn view(pv: &Preview) -> Node<Msg> {
    let to = |m: PreviewMsg| Msg::Preview(m);
    let (wide, tall) = most(pv.room);
    let says = super::kinds::ending_of(&work::name_of(&pv.file)).is_some();
    let pdf_word: String = if says { String::new() } else { t!("folders-preview-pdf").into() };
    let (body, about, width): Body = match &pv.seen {
        Seen::Loading => (crate::toolkit::tiles::loading_state(LEAST.1), String::new(), LEAST.0),
        Seen::Words(w) => (note(w.clone()).width(Length::Fill), String::new(), (wide * 0.6).max(LEAST.0)),
        Seen::Trouble(w) => (row(vec![note(w.clone()).width(Length::Fill), crate::common::more::button(Msg::Help(super::help::FILE_UNREAD))]).gap(8.0).width(Length::Fill),
                             String::new(), (wide * 0.6).max(LEAST.0)),
        Seen::Text { blocks, encoding, cut, read } => text_body(pv, blocks, encoding, *cut, *read, says, (wide, tall)),
        Seen::Picture(img) => {
            let (w, h) = img.dimensions();
            let (node, shown_w) = picture(img.clone(), pv.fitted.clone(), (wide - 40.0, tall));
            (node, t!("folders-preview-picture-size", w = w, h = h), (shown_w + 40.0).max(LEAST.0))
        }
        Seen::Pdf { page, pages, drawn } => pdf_body(pv, *page, *pages, drawn, pdf_word, (wide, tall)),
        Seen::Font { name, page, installed } => font_body(pv, name, page, *installed, (wide, tall)),
        Seen::Media { player, cover } => media_body(pv, player, cover, says, (wide, tall)),
    };
    let name = work::name_of(&pv.file);
    card(column(vec![
        super::dialogs::head_to(super::icons::stand_in::PREVIEW, name, t!("folders-preview-close"), Msg::Ask(AskMsg::Close)),
        body,
        row(vec![
            button("go-previous-symbolic", t!("folders-preview-file-before")).on_press(to(PreviewMsg::Step(-1))),
            button("go-next-symbolic", t!("folders-preview-file-after")).on_press(to(PreviewMsg::Step(1))),
            note(about).width(Length::Fill),
            main_action("document-open-symbolic", t!("folders-preview-open")).on_press(to(PreviewMsg::Open)),
        ]).gap(8.0).width(Length::Fill).centred(),
    ]).gap(12.0).width(Length::Fill))
    .width(Length::Fixed(width.min(wide)))
    .on_escape(Msg::Ask(AskMsg::Close))
}

fn text_body(pv: &Preview, blocks: &[String], encoding: &'static str, cut: bool, read: u64, says: bool, (wide, tall): (f32, f32)) -> Body {
    let to = |m: PreviewMsg| Msg::Preview(m);
    let mut parts: Vec<Node<Msg>> = blocks.iter().map(|b| label(b.clone()).width(Length::Fill)).collect();
    if cut {
        parts.push(row(vec![
            space().width(Length::Fill),
            button("go-down-symbolic", t!("folders-preview-read-more")).on_press(to(PreviewMsg::More)),
            space().width(Length::Fill),
        ]).width(Length::Fill));
    }
    let text = column(parts).gap(0.0).width(Length::Fill);
    let about = match (cut, says) {
        (true, true) => t!("folders-preview-text-so-far", encoding = encoding, size = size_words(read)),
        (false, true) => t!("folders-preview-text", encoding = encoding),
        (true, false) => t!("folders-preview-kind-text-so-far", encoding = encoding, size = size_words(read)),
        (false, false) => t!("folders-preview-kind-text", encoding = encoding),
    };
    (scroll(format!("preview:{}", pv.file.display()), text).width(Length::Fill).height(Length::Fixed(tall)), about, wide)
}

fn pdf_body(pv: &Preview, page: u32, pages: Option<u32>, drawn: &Option<Rc<DynamicImage>>, pdf_word: String, (wide, tall): (f32, f32)) -> Body {
    let to = |m: PreviewMsg| Msg::Preview(m);
    let at = match pages {
        Some(n) => t!("folders-preview-page-of", page = page, pages = n),
        None => t!("folders-preview-page", page = page),
    };
    let turn = row(vec![
        space().width(Length::Fill),
        button("go-up-symbolic", t!("folders-preview-page-before")).on_press(to(PreviewMsg::Page(-1))).off(page <= 1),
        label(at),
        button("go-down-symbolic", t!("folders-preview-page-after")).on_press(to(PreviewMsg::Page(1)))
            .off(pages.is_some_and(|n| page >= n)),
        space().width(Length::Fill),
    ]).gap(8.0).width(Length::Fill);
    match drawn {
        Some(img) => {
            let (node, shown_w) = picture(img.clone(), pv.fitted.clone(), (wide - 40.0, tall - 48.0));
            (column(vec![node, turn]).gap(8.0).width(Length::Fill), pdf_word, (shown_w + 40.0).max(LEAST.0))
        }
        None => (column(vec![note(t!("folders-preview-drawing")), turn]).gap(8.0).width(Length::Fill), pdf_word, LEAST.0),
    }
}

fn font_body(pv: &Preview, name: &str, page: &Rc<DynamicImage>, installed: bool, (wide, tall): (f32, f32)) -> Body {
    let to = |m: PreviewMsg| Msg::Preview(m);
    let (node, shown_w) = picture(page.clone(), pv.fitted.clone(), (wide - 40.0, tall - 48.0));
    let state: Node<Msg> = if installed {
        note(t!("folders-preview-font-installed"))
    } else {
        button(super::icons::stand_in::FONT, t!("folders-preview-font-install")).on_press(to(PreviewMsg::InstallFont))
    };
    let below = row(vec![space().width(Length::Fill), state, space().width(Length::Fill)]).width(Length::Fill);
    (column(vec![node, below]).gap(8.0).width(Length::Fill), name.to_string(), (shown_w + 40.0).max(LEAST.0))
}

fn media_body(pv: &Preview, player: &Player, cover: &Option<Rc<DynamicImage>>, says: bool, (wide, tall): (f32, f32)) -> Body {
    let length = player.length().map(time_words);
    let (shown, shown_w, about): (Node<Msg>, f32, String) = if player.failed {
        (row(vec![note(t!("folders-preview-unplayable")).width(Length::Fill), crate::common::more::button(Msg::Help(super::help::FILE_UNREAD))]).gap(8.0).width(Length::Fill),
         (wide * 0.6).max(LEAST.0), String::new())
    } else if let Some((w, h, _)) = player.info.video {
        let (node, shown_w) = frames(player, (wide - 40.0, tall - CONTROLS));
        (node, shown_w + 40.0, match &length {
            Some(l) if says => t!("folders-preview-video-length", w = w, h = h, length = l),
            Some(l) => t!("folders-preview-kind-video-length", w = w, h = h, length = l),
            None if says => t!("folders-preview-video", w = w, h = h),
            None => t!("folders-preview-kind-video", w = w, h = h),
        })
    } else if let Some(img) = cover {
        let room = COVER_ROOM.min(tall - CONTROLS);
        let (node, _) = picture(img.clone(), pv.fitted.clone(), (room, room));
        (node, SOUND_WIDTH, sound_about(&length, says))
    } else {
        (sound_icon(), SOUND_WIDTH, sound_about(&length, says))
    };
    (column(vec![shown, controls(player)]).gap(8.0).width(Length::Fill), about, shown_w.max(LEAST.0))
}

fn picture(img: Rc<DynamicImage>, fitted: Fitted, room: (f32, f32)) -> (Node<Msg>, f32) {
    let (iw, ih) = img.dimensions();
    let k = (room.0 / iw as f32).min(room.1 / ih as f32).min(1.0);
    let (w, h) = ((iw as f32 * k).max(1.0), (ih as f32 * k).max(1.0));
    let shown = drawing((w, h), t!("folders-preview-picture"), move |canvas, _| {
        let want = (canvas.width(), canvas.height());
        let mut kept = fitted.borrow_mut();
        if kept.as_ref().map(|(size, _)| *size) != Some(want) {
            let k = (want.0 as f32 / iw as f32).min(want.1 as f32 / ih as f32).min(1.0);
            let (fw, fh) = (((iw as f32 * k).round() as u32).max(1), ((ih as f32 * k).round() as u32).max(1));
            let small = if (fw, fh) == (iw, ih) { None } else { Some(img.thumbnail_exact(fw, fh)) };
            *kept = crate::toolkit::picture::to_pixmap(small.as_ref().unwrap_or(&img)).map(|px| (want, px));
        }
        if let Some((_, px)) = kept.as_ref() {
            let (x, y) = ((want.0 as i32 - px.width() as i32) / 2, (want.1 as i32 - px.height() as i32) / 2);
            canvas.draw_pixmap(x, y, px.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        }
    });
    (row(vec![space().width(Length::Fill), shown, space().width(Length::Fill)]).width(Length::Fill).height(Length::Fixed(h)), w)
}

const SOUND_WIDTH: f32 = 520.0;

fn time_words(secs: f64) -> String {
    super::media::length_words(secs.max(0.0) as u64)
}

fn frames(player: &Player, room: (f32, f32)) -> (Node<Msg>, f32) {
    let (fw, fh) = player.video().map_or((1.0, 1.0), |(w, h)| (w as f32, h as f32));
    let k = (room.0 / fw).min(room.1 / fh).min(1.0);
    let (w, h) = ((fw * k).floor().max(1.0), (fh * k).floor().max(1.0));
    let shown = moving((w, h), t!("folders-preview-video-name"), player.frame.clone());
    (row(vec![space().width(Length::Fill), shown, space().width(Length::Fill)]).gap(0.0).width(Length::Fill).height(Length::Fixed(h)), w)
}

fn sound_about(length: &Option<String>, says: bool) -> String {
    match (length, says) {
        (Some(l), true) => t!("folders-preview-sound-length", length = l),
        (Some(l), false) => t!("folders-preview-kind-sound-length", length = l),
        (None, true) => String::new(),
        (None, false) => t!("folders-preview-sound").to_string(),
    }
}

fn sound_icon() -> Node<Msg> {
    const SIZE: f32 = 96.0;
    let shown = drawing((SIZE, SIZE), t!("folders-preview-sound-name"), |canvas, pen| {
        let s = (SIZE * pen.scale).round() as u32;
        pen.icons.draw(canvas, "audio-x-generic-symbolic", 0, 0, s, 1, pen.theme.muted);
    });
    row(vec![space().width(Length::Fill), shown, space().width(Length::Fill)]).width(Length::Fill)
}

fn controls(player: &Player) -> Node<Msg> {
    let to = |m: PreviewMsg| Msg::Preview(m);
    let (icon, tip) = if player.playing() {
        ("media-playback-pause-symbolic", t!("folders-preview-pause"))
    } else {
        ("media-playback-start-symbolic", t!("folders-preview-play"))
    };
    let at = player.position();
    let mut parts = vec![button(icon, tip).on_press(to(PreviewMsg::PlayPause)).off(player.failed)];
    match player.length() {
        Some(length) => {
            let step = (length / 1000.0).max(0.1) as f32;
            parts.push(slider(at as f32, 0.0, length as f32, step, t!("folders-preview-position"), |v| Msg::Preview(PreviewMsg::Seek(v)))
                .off(player.failed));
            parts.push(note(format!("{} / {}", time_words(at), time_words(length))));
        }
        None => {
            parts.push(space().width(Length::Fill));
            parts.push(note(time_words(at)));
        }
    }
    if player.info.sound {
        let (icon, tip) = if play::muted() {
            ("audio-volume-muted-symbolic", t!("folders-preview-unmute"))
        } else {
            ("audio-volume-high-symbolic", t!("folders-preview-mute"))
        };
        parts.push(button(icon, tip).on_press(to(PreviewMsg::Mute)));
        parts.push(slider(play::volume(), 0.0, 1.0, 0.05, t!("folders-preview-volume"), |v| Msg::Preview(PreviewMsg::Volume(v))).width(Length::Fixed(110.0)));
    }
    row(parts).gap(8.0).width(Length::Fill)
}

#[cfg(test)]
mod tests {
    use super::{PreviewMsg, Seen};
    use crate::files::app::{Files, Msg};
    use crate::files::dialogs::Dialog;
    use crate::files::places::Where;
    use crate::files::trash::Bin;
    use crate::toolkit::ui::App;
    use std::time::Duration;

    #[test]
    fn space_shows_a_picture_and_text_and_steps_between_them() {
        let dir = std::env::temp_dir().join(format!("trindle-files-preview-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("folder")).unwrap();
        image::RgbImage::from_pixel(300, 100, image::Rgb([10, 200, 10])).save(dir.join("a.png")).unwrap();
        std::fs::write(dir.join("b.txt"), "first line\nsecond line\n").unwrap();

        let mut f = Files::new(vec![dir.clone()]);
        f.bin = Some(Bin::at(&dir.join("Trash"), None));
        f.go(0, Where::Folder(dir.clone()));
        let at = |f: &Files, name: &str| f.cols[0].tabs[0].entries.iter().position(|e| e.name == name).unwrap();

        let folder = at(&f, "folder");
        f.update(Msg::Choose(0, folder));
        f.update(Msg::Preview(PreviewMsg::Toggle));
        assert!(f.dialog.is_none());

        let a = at(&f, "a.png");
        f.update(Msg::Choose(0, a));
        f.update(Msg::Preview(PreviewMsg::Toggle));
        let Some(Dialog::Preview(pv)) = &f.dialog else { panic!("no viewer") };
        assert!(matches!(&pv.seen, Seen::Picture(img) if img.width() == 300));

        f.update(Msg::Preview(PreviewMsg::Step(1)));
        let Some(Dialog::Preview(pv)) = &f.dialog else { panic!("no viewer") };
        assert!(matches!(&pv.seen, Seen::Text { blocks, .. } if blocks[0] == "first line\nsecond line"));
        assert_eq!(f.cols[0].tabs[0].chosen, Some(at(&f, "b.txt")));

        f.update(Msg::Preview(PreviewMsg::Step(1)));
        assert!(matches!(&f.dialog, Some(Dialog::Preview(pv)) if pv.file.ends_with("b.txt")));
        f.update(Msg::Preview(PreviewMsg::Toggle));
        assert!(f.dialog.is_none());
        let _ = std::fs::remove_dir_all(dir);
    }

    fn two_pages() -> Vec<u8> {
        let (blue, green) = ("0 0 1 rg 0 0 595 842 re f", "0 1 0 rg 0 0 595 842 re f");
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
            "<< /Type /Pages /Kids [3 0 R 5 0 R] /Count 2 >>".to_string(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Contents 4 0 R >>".to_string(),
            format!("<< /Length {} >>\nstream\n{blue}\nendstream", blue.len()),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Contents 6 0 R >>".to_string(),
            format!("<< /Length {} >>\nstream\n{green}\nendstream", green.len()),
        ];
        let mut out = b"%PDF-1.4\n".to_vec();
        let mut at = Vec::new();
        for (i, o) in objects.iter().enumerate() {
            at.push(out.len());
            out.extend(format!("{} 0 obj\n{o}\nendobj\n", i + 1).bytes());
        }
        let xref = out.len();
        out.extend(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).bytes());
        for a in at {
            out.extend(format!("{a:010} 00000 n \n").bytes());
        }
        out.extend(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objects.len() + 1).bytes());
        out
    }

    #[test]
    fn a_pdf_is_shown_a_page_at_a_time_and_a_song_offers_to_play() {
        let dir = std::env::temp_dir().join(format!("trindle-files-preview-pdf-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.pdf"), two_pages()).unwrap();
        std::fs::write(dir.join("b.flac"), b"fLaC").unwrap();
        let mut f = Files::new(vec![dir.clone()]);
        f.go(0, Where::Folder(dir.clone()));
        let at = |f: &Files, name: &str| f.cols[0].tabs[0].entries.iter().position(|e| e.name == name).unwrap();
        crate::common::needs::programs(&["bwrap", "pdftoppm", "ffmpeg", "ffprobe"]);
        {
            let a = at(&f, "a.pdf");
            f.update(Msg::Choose(0, a));
            f.update(Msg::Preview(PreviewMsg::Toggle));
            let colour = |f: &Files| match &f.dialog {
                Some(Dialog::Preview(pv)) => match &pv.seen {
                    Seen::Pdf { page, pages, drawn: Some(img) } => {
                        let img = img.to_rgb8();
                        Some((*page, *pages, img.get_pixel(img.width() / 2, img.height() / 2).0))
                    }
                    _ => None,
                },
                _ => None,
            };
            assert_eq!(colour(&f), Some((1, Some(2), [0, 0, 255])));
            f.update(Msg::Preview(PreviewMsg::Page(1)));
            assert_eq!(colour(&f), Some((2, Some(2), [0, 255, 0])));
            f.update(Msg::Preview(PreviewMsg::Page(1)));
            assert_eq!(colour(&f).map(|c| c.0), Some(2));
            f.update(Msg::Preview(PreviewMsg::Toggle));
        }
        {
            let b = at(&f, "b.flac");
            f.update(Msg::Choose(0, b));
            f.update(Msg::Preview(PreviewMsg::Toggle));
            assert!(matches!(&f.dialog, Some(Dialog::Preview(pv)) if matches!(&pv.seen, Seen::Trouble(w) if w.starts_with("Folders can't play this"))));
            f.update(Msg::Preview(PreviewMsg::Space));
            assert!(f.dialog.is_none());
        }
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_video_and_a_sound_play_with_their_controls() {
        crate::common::needs::programs(&["bwrap", "ffmpeg", "ffprobe"]);
        let dir = std::env::temp_dir().join(format!("trindle-files-preview-play-{}", std::process::id()));
        made_up_clips(&dir);
        let mut f = Files::new(vec![dir.clone()]);
        f.room = (1000.0, 800.0);
        f.go(0, Where::Folder(dir.clone()));
        let a = f.cols[0].tabs[0].entries.iter().position(|e| e.name == "a.mkv").unwrap();
        f.update(Msg::Choose(0, a));
        f.update(Msg::Preview(PreviewMsg::Toggle));
        let (playing, _, pid, video) = player_of(&f).expect("a player");
        assert!(playing && pid.is_some());
        assert_eq!(video, Some((320, 240)));
        let started = std::time::Instant::now();
        while player_of(&f).unwrap().1 <= 0.4 && started.elapsed() < Duration::from_secs(120) {
            std::thread::sleep(Duration::from_millis(20));
        }
        pauses_seeks_and_plays_on(&mut f);
        keeps_the_volume_and_mute(&mut f);
        steps_to_the_sound_and_closes(&mut f);
        let _ = std::fs::remove_dir_all(dir);
    }

    fn made_up_clips(dir: &std::path::Path) {
        let _ = std::fs::remove_dir_all(dir);
        std::fs::create_dir_all(dir).unwrap();
        crate::files::walled::tests::test_clip(dir, "a.mkv", 3).expect("ffmpeg could not make the clip this test plays");
        let made = std::process::Command::new("ffmpeg")
            .args(["-nostdin", "-loglevel", "error", "-f", "lavfi", "-i", "sine=frequency=330:duration=2", "-c:a", "libvorbis"])
            .arg(dir.join("b.ogg")).status();
        assert!(made.is_ok_and(|s| s.success()));
    }

    type Playing = (bool, f64, Option<u32>, Option<(u32, u32)>);

    fn player_of(f: &Files) -> Option<Playing> {
        match &f.dialog {
            Some(Dialog::Preview(pv)) => match &pv.seen {
                Seen::Media { player, .. } => Some((player.playing(), player.position(), player.boxed_id(), player.video())),
                _ => None,
            },
            _ => None,
        }
    }

    fn pauses_seeks_and_plays_on(f: &mut Files) {
        f.update(Msg::Preview(PreviewMsg::Space));
        let (playing, held, now, _) = player_of(f).unwrap();
        assert!(!playing && now.is_none() && held > 0.3, "{held}");
        assert!(f.dialog.is_some());
        let Some(Dialog::Preview(pv)) = &f.dialog else { unreachable!() };
        let Seen::Media { player: p, .. } = &pv.seen else { unreachable!() };
        assert!(p.frame.lock().unwrap().is_some());
        std::thread::sleep(Duration::from_millis(300));
        f.update(Msg::Preview(PreviewMsg::Seek(2.0)));
        assert_eq!(player_of(f).unwrap().1, 2.0);
        std::thread::sleep(Duration::from_millis(300));
        f.update(Msg::Preview(PreviewMsg::Tick(0)));
        f.update(Msg::Preview(PreviewMsg::PlayPause));
        assert!(player_of(f).unwrap().0);
    }

    fn keeps_the_volume_and_mute(f: &mut Files) {
        f.update(Msg::Preview(PreviewMsg::Volume(0.4)));
        f.update(Msg::Preview(PreviewMsg::Mute));
        assert!(crate::files::play::muted() && (crate::files::play::volume() - 0.4).abs() < 1e-6);
        f.update(Msg::Preview(PreviewMsg::Mute));
        f.update(Msg::Preview(PreviewMsg::Volume(1.0)));
    }

    fn steps_to_the_sound_and_closes(f: &mut Files) {
        let pid = player_of(f).unwrap().2.unwrap();
        f.update(Msg::Preview(PreviewMsg::Step(1)));
        assert!(!std::path::Path::new(&format!("/proc/{pid}")).exists());
        let (playing, _, pid, video) = player_of(f).expect("a sound");
        assert!(playing && video.is_none());
        f.update(Msg::Ask(crate::files::dialogs::AskMsg::Close));
        assert!(f.dialog.is_none());
        assert!(!std::path::Path::new(&format!("/proc/{}", pid.unwrap())).exists());
    }
}
