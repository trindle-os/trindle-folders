//! The preview panel at the right of the window, following whatever is chosen in the active pane.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use image::{DynamicImage, GenericImageView};
use tiny_skia::{Pixmap, PixmapPaint, Transform};

use super::app::{Files, Msg};
use super::icons::stand_in;
use super::kinds::when_text;
use super::places::{Entry, Where};
use super::view::{items_words, size_words};
use super::viewer::{self, Limits, Shown};
use crate::toolkit::colour::Colour;
use crate::toolkit::event::Post;
use crate::toolkit::input::Cursor;
use crate::toolkit::t;
use crate::toolkit::text::Style;
use crate::toolkit::theme;
use crate::toolkit::tiles::loading_state;
use crate::toolkit::widget::{card, column, drawing, heading, label, note, row, scroll, Length, Node, Pointer};

pub const LEAST: f32 = 240.0;
pub const MOST: f32 = 480.0;
pub const PANE_LEAST: f32 = 275.0;
const TEXT_MOST: u64 = 16 << 10;
const TEXT_LINES: usize = 40;
const LINE_MOST: usize = 200;
const PICTURE_MOST: u32 = 1024;
const COUNTED_MOST: u64 = 64 << 20;
const FACT_W: f32 = 84.0;
fn working() -> &'static str {
    t!("folders-panel-working")
}

type Fitted = Rc<RefCell<Option<((u32, u32), Pixmap)>>>;

#[derive(Clone, Debug, PartialEq)]
enum Subject {
    Place(Where, usize),
    One(Entry),
    Many(usize, u64, bool),
}

#[derive(Clone, Debug, PartialEq)]
enum Want {
    File(PathBuf),
    Folder(PathBuf),
    Archive(PathBuf),
}

enum Seen {
    Waiting,
    Picture(Rc<DynamicImage>, Fitted),
    Text(Rc<str>),
    Plain,
    Trouble(String),
}

struct About {
    subject: Subject,
    name: String,
    icon: &'static str,
    folder: bool,
    tint: Option<Colour>,
    facts: Vec<(&'static str, String)>,
    seen: Seen,
}

pub struct Got {
    seen: Read,
    facts: Vec<(&'static str, String)>,
}

enum Read {
    Picture(DynamicImage),
    Text(String),
    Plain,
    Trouble(String),
}

struct Asked {
    id: u64,
    want: Want,
    stop: Arc<AtomicBool>,
}

#[derive(Default)]
struct Slot {
    next: Mutex<(Option<Asked>, bool)>,
    wake: Condvar,
}

impl Slot {
    fn put(&self, asked: Asked) -> Option<Asked> {
        let mut next = self.next.lock().unwrap_or_else(|e| e.into_inner());
        let dropped = next.0.replace(asked);
        if let Some(d) = &dropped {
            d.stop.store(true, Ordering::Relaxed);
        }
        self.wake.notify_one();
        dropped
    }

    fn take(&self) -> Option<Asked> {
        let mut next = self.next.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if next.1 {
                return None;
            }
            if let Some(a) = next.0.take() {
                return Some(a);
            }
            next = self.wake.wait(next).unwrap_or_else(|e| e.into_inner());
        }
    }

    fn close(&self) {
        self.next.lock().unwrap_or_else(|e| e.into_inner()).1 = true;
        self.wake.notify_one();
    }
}

struct Held {
    from: f32,
    was: f32,
    moved: bool,
}

#[derive(Default)]
pub struct Panel {
    about: Option<About>,
    id: u64,
    stop: Arc<AtomicBool>,
    slot: Option<Arc<Slot>>,
    held: Option<Held>,
}

impl Drop for Panel {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(slot) = &self.slot {
            slot.close();
        }
    }
}

#[derive(Clone)]
pub enum PanelMsg {
    Switch,
    Got(u64, Arc<Mutex<Option<Got>>>),
    Edge(Pointer),
}

pub fn least_window(two: bool) -> f32 {
    let s = theme::sheet();
    let panes = if two { 2.0 * PANE_LEAST + super::act::between() } else { PANE_LEAST };
    2.0 * s.panel_gap + super::view::side_panel() + s.panel_gap + 2.0 * s.card_padding + panes + s.panel_gap + LEAST
}

pub fn width_in(room: f32, two: bool, kept: f32) -> Option<f32> {
    let spare = room - (least_window(two) - LEAST);
    (spare >= LEAST).then(|| kept.clamp(LEAST, MOST).min(spare))
}

pub fn too_narrow() -> &'static str {
    t!("folders-panel-too-narrow")
}

fn where_words(path: &Path, home: &Path) -> String {
    let Some(dir) = path.parent() else { return String::new() };
    match dir.strip_prefix(home) {
        Ok(rest) if rest.as_os_str().is_empty() => t!("folders-panel-home").to_string(),
        Ok(rest) => t!("folders-panel-home-under", rest = rest.display().to_string()),
        Err(_) => dir.display().to_string(),
    }
}

impl Files {
    pub(super) fn panel_room(&self) -> Option<f32> {
        width_in(self.room.0, self.two, self.kept.panel_width())
    }

    pub(super) fn panel_shows(&self) -> bool {
        self.kept.panel && self.panel_room().is_some() && !self.as_root
    }

    fn panel_keep(&mut self) {
        if self.kept.save_panel().is_err() {
            self.say_help(t!("folders-panel-keep-failed"), super::help::NOT_KEPT);
        }
    }

    pub(super) fn panel_msg(&mut self, m: PanelMsg) {
        match m {
            PanelMsg::Switch => {
                let on = !self.kept.panel;
                if on && self.panel_room().is_none() {
                    return self.say(t!("folders-panel-too-narrow-said"), false);
                }
                self.kept.set_panel(on, None);
                self.panel_keep();
                if !on {
                    self.panel_forget();
                }
            }
            PanelMsg::Got(id, got) => {
                let Some(got) = got.lock().ok().and_then(|mut g| g.take()) else { return };
                let Some(about) = self.panel.about.as_mut().filter(|_| self.panel.id == id) else { return };
                about.seen = match got.seen {
                    Read::Picture(img) => Seen::Picture(Rc::new(img), Fitted::default()),
                    Read::Text(text) => Seen::Text(text.into()),
                    Read::Plain => Seen::Plain,
                    Read::Trouble(w) => Seen::Trouble(w),
                };
                for (key, words) in got.facts {
                    match about.facts.iter_mut().find(|(k, _)| *k == key) {
                        Some(fact) => fact.1 = words,
                        None => about.facts.push((key, words)),
                    }
                }
            }
            PanelMsg::Edge(at) => self.panel_edge(at),
        }
    }

    fn panel_edge(&mut self, at: Pointer) {
        let x = self.pointer.get().0;
        match at {
            Pointer::Down(..) => {
                let Some(was) = self.panel_room() else { return };
                self.panel.held = Some(Held { from: x, was, moved: false });
            }
            Pointer::Move(..) => {
                let Some(held) = &mut self.panel.held else { return };
                let by = x - held.from;
                held.moved |= by.abs() >= super::act::DRAG_SLOP;
                if held.moved {
                    let to = (held.was - by).clamp(LEAST, MOST);
                    self.kept.set_panel(true, Some(to));
                }
            }
            Pointer::Up(..) => {
                let Some(held) = self.panel.held.take() else { return };
                if !held.moved {
                    let steps = [LEAST, theme::sheet().preview_panel, MOST];
                    let next = steps.into_iter().find(|w| *w > held.was + 0.5).unwrap_or(LEAST);
                    self.kept.set_panel(true, Some(next));
                }
                self.panel_keep();
            }
            _ => {}
        }
    }

    fn panel_forget(&mut self) {
        self.panel.stop.store(true, Ordering::Relaxed);
        self.panel.about = None;
        self.panel.id += 1;
    }

    fn panel_subject(&self) -> Subject {
        let p = self.side();
        let pane = &self.cols[p];
        match pane.picked.as_slice() {
            [] if pane.at == Where::Computer => Subject::Place(Where::Computer, self.computer.cards.len()),
            [] => Subject::Place(pane.at.clone(), pane.entries.len()),
            [one] => match pane.entries.get(*one) {
                Some(e) => Subject::One(e.clone()),
                None => Subject::Place(pane.at.clone(), pane.entries.len()),
            },
            _ => {
                let (count, bytes, short) = self.picked_total(p);
                Subject::Many(count, bytes, short)
            }
        }
    }

    pub(super) fn panel_wanted(&mut self) {
        if !self.panel_shows() {
            if self.panel.about.is_some() {
                self.panel_forget();
            }
            return;
        }
        let subject = self.panel_subject();
        if self.panel.about.as_ref().is_some_and(|a| a.subject == subject) {
            return;
        }
        self.panel.stop.store(true, Ordering::Relaxed);
        self.panel.stop = Arc::new(AtomicBool::new(false));
        self.panel.id += 1;
        let (about, want) = self.panel_about(subject);
        self.panel.about = Some(about);
        if let Some(want) = want {
            self.panel_read(want);
        }
    }

    fn panel_about(&self, subject: Subject) -> (About, Option<Want>) {
        let chosen = subject.clone();
        match &chosen {
            Subject::Place(at, count) => self.place_about(subject, at, *count),
            Subject::One(e) => self.one_about(subject, e),
            Subject::Many(count, bytes, short) => {
                let size = if *short { working().to_string() } else { size_words(*bytes) };
                let (name, facts) = (t!("folders-panel-selected", count = *count), vec![(t!("folders-panel-fact-size"), size)]);
                (About { subject, name, icon: stand_in::SEVERAL, folder: false, tint: None, facts, seen: Seen::Plain }, None)
            }
        }
    }

    fn place_about(&self, subject: Subject, at: &Where, count: usize) -> (About, Option<Want>) {
        let name = self.place_name(at);
        match at {
            Where::Folder(f) => {
                let when = std::fs::metadata(f).ok().and_then(|m| m.modified().ok())
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs() as i64);
                let facts = vec![(t!("folders-panel-fact-holds"), working().to_string()), (t!("folders-panel-fact-size"), working().to_string()),
                                 (t!("folders-panel-fact-changed"), when_text(when)), (t!("folders-panel-fact-where"), where_words(f, &self.home))];
                let want = Want::Folder(f.clone());
                (About { subject, name, icon: "folder-symbolic", folder: true, tint: self.tint_of(f), facts, seen: Seen::Plain }, Some(want))
            }
            _ => {
                let icon = match at {
                    Where::Trash => "user-trash-symbolic",
                    Where::Recent => "document-open-recent-symbolic",
                    Where::Network => "network-workgroup-symbolic",
                    Where::Computer => "computer-symbolic",
                    Where::Remote(_) => stand_in::SERVER,
                    Where::Inside(..) => "package-x-generic-symbolic",
                    Where::Found(..) => "system-search-symbolic",
                    Where::Folder(_) => "folder-symbolic",
                };
                let facts = match at {
                    Where::Computer => vec![(t!("folders-panel-fact-holds"), super::computer::count_words(self.computer.cards.len()))],
                    _ => vec![(t!("folders-panel-fact-holds"), items_words(count as u64))],
                };
                (About { subject, name, icon, folder: false, tint: None, facts, seen: Seen::Plain }, None)
            }
        }
    }

    fn one_about(&self, subject: Subject, e: &Entry) -> (About, Option<Want>) {
        let here = &self.pane().at;
        let near = |path: &Path| matches!(here, Where::Folder(_) | Where::Recent | Where::Trash | Where::Found(..)) && !super::far::is_far(path);
        let name = e.name.clone();
        let mut facts: Vec<(&'static str, String)> = Some(self.cache.kind_beside(e)).filter(|k| !k.is_empty()).map(|k| (t!("folders-panel-fact-kind"), k)).into_iter().collect();
        let want = if !near(&e.path) {
            None
        } else if e.folder {
            Some(Want::Folder(e.path.clone()))
        } else if super::inside::opens(&e.path) {
            Some(Want::Archive(e.path.clone()))
        } else {
            Some(Want::File(e.path.clone()))
        };
        if e.folder {
            if want.is_some() {
                facts.push((t!("folders-panel-fact-holds"), working().to_string()));
                facts.push((t!("folders-panel-fact-size"), working().to_string()));
            }
        } else {
            facts.push((t!("folders-panel-fact-size"), size_words(e.size)));
        }
        facts.push((t!("folders-panel-fact-changed"), when_text(e.when)));
        facts.push((t!("folders-panel-fact-where"), where_words(&e.path, &self.home)));
        facts.retain(|(_, words)| !words.is_empty());
        let seen = match &want {
            Some(Want::File(_)) => Seen::Waiting,
            _ => Seen::Plain,
        };
        (About { subject, name, icon: e.icon(), folder: e.folder_icon(), tint: self.tint_of(&e.path), facts, seen }, want)
    }

    fn panel_read(&mut self, want: Want) {
        let asked = Asked { id: self.panel.id, want, stop: self.panel.stop.clone() };
        let Some(post) = self.post.clone() else {
            let got = read(&asked.want, &asked.stop);
            return self.panel_msg(PanelMsg::Got(asked.id, Arc::new(Mutex::new(Some(got)))));
        };
        let slot = self.panel.slot.get_or_insert_with(|| {
            let slot = Arc::new(Slot::default());
            let theirs = slot.clone();
            let _ = std::thread::Builder::new().name("folders-panel".into()).spawn(move || work(&theirs, &post));
            slot
        });
        slot.put(asked);
    }
}

fn work(slot: &Slot, post: &Post<Msg>) {
    while let Some(asked) = slot.take() {
        if asked.stop.load(Ordering::Relaxed) {
            continue;
        }
        let got = read(&asked.want, &asked.stop);
        if asked.stop.load(Ordering::Relaxed) {
            continue;
        }
        if !post.send(Msg::Panel(PanelMsg::Got(asked.id, Arc::new(Mutex::new(Some(got)))))) {
            return;
        }
    }
}

fn read(want: &Want, stop: &AtomicBool) -> Got {
    match want {
        Want::File(path) => read_file(path),
        Want::Folder(path) => {
            let facts = match super::sizes::measure(path, stop, &mut |_| {}) {
                Ok(t) => vec![(t!("folders-panel-fact-holds"), t!("folders-panel-in-all", items = items_words(t.items()))), (t!("folders-panel-fact-size"), size_words(t.bytes))],
                Err(_) => vec![(t!("folders-panel-fact-holds"), t!("folders-panel-unread").to_string()), (t!("folders-panel-fact-size"), String::new())],
            };
            Got { seen: Read::Plain, facts }
        }
        Want::Archive(path) => Got { seen: Read::Plain, facts: archive_facts(path) },
    }
}

fn archive_facts(path: &Path) -> Vec<(&'static str, String)> {
    use super::archive::{self, Format};
    let listed = matches!(Format::of(path), Some(Format::Zip | Format::SevenZ));
    let small = std::fs::metadata(path).is_ok_and(|m| m.len() <= COUNTED_MOST);
    if !listed && !small {
        return Vec::new();
    }
    match archive::list(path, None) {
        Ok(entries) => {
            let bytes: u64 = entries.iter().filter(|e| !e.folder).map(|e| e.size).sum();
            vec![(t!("folders-panel-fact-holds"), items_words(entries.len() as u64)), (t!("folders-panel-fact-unpacked"), size_words(bytes))]
        }
        Err(e) if archive::wants_key(&e).is_some() => vec![(t!("folders-panel-fact-holds"), t!("folders-panel-locked").to_string())],
        Err(_) => vec![(t!("folders-panel-fact-holds"), t!("folders-panel-unread").to_string())],
    }
}

fn read_file(path: &Path) -> Got {
    let limits = Limits { text: TEXT_MOST, ..Limits::default() };
    let mut facts = Vec::new();
    let seen = match viewer::load(path, &limits) {
        Shown::Picture(img) => {
            let (w, h) = img.dimensions();
            facts.push((t!("folders-panel-fact-pixels"), t!("folders-panel-by", w = w, h = h)));
            if let Some(camera) = super::media::read(path).camera {
                facts.push((t!("folders-panel-fact-camera"), camera));
            }
            Read::Picture(small(img))
        }
        Shown::Pdf { first, pages } => {
            if let Some(n) = pages {
                facts.push((t!("folders-panel-fact-pages"), n.to_string()));
            }
            Read::Picture(small(first))
        }
        Shown::Text { text, .. } => Read::Text(start_of(&text)),
        Shown::Media { info, cover } => {
            if let Some((w, h, _)) = info.video {
                facts.push((t!("folders-panel-fact-pixels"), t!("folders-panel-by", w = w, h = h)));
            }
            if let Some(length) = info.length {
                facts.push((t!("folders-panel-fact-length"), super::media::length_words(length.max(0.0) as u64)));
            }
            let picture = if info.video.is_some() {
                super::walled::video(path, PICTURE_MOST)
            } else {
                let m = super::media::read(path);
                for (key, words) in [(t!("folders-panel-fact-title"), m.title), (t!("folders-panel-fact-artist"), m.artist), (t!("folders-panel-fact-album"), m.album)] {
                    if let Some(words) = words {
                        facts.push((key, words));
                    }
                }
                cover
            };
            picture.map_or(Read::Plain, |img| Read::Picture(small(img)))
        }
        Shown::Font { name, page } => {
            facts.push((t!("folders-panel-fact-font"), name));
            Read::Picture(small(page))
        }
        Shown::Unplayable(_) | Shown::Other => Read::Plain,
        Shown::Broken(what) => Read::Trouble(t!("folders-panel-broken", what = what)),
        Shown::TooLarge { .. } => Read::Trouble(t!("folders-panel-too-large").to_string()),
        Shown::Unread(why) => Read::Trouble(t!("folders-panel-read-failed", why = why)),
    };
    Got { seen, facts }
}

fn small(img: DynamicImage) -> DynamicImage {
    let (w, h) = img.dimensions();
    if w.max(h) <= PICTURE_MOST { img } else { img.thumbnail(PICTURE_MOST, PICTURE_MOST) }
}

fn start_of(text: &str) -> String {
    let lines: Vec<String> = text.lines().take(TEXT_LINES).map(|line| {
        line.chars().filter(|c| *c == '\t' || !c.is_control()).take(LINE_MOST).collect()
    }).collect();
    lines.join("\n")
}

pub fn edge() -> Node<Msg> {
    let wide = theme::sheet().panel_gap;
    drawing((wide, 0.0), t!("folders-panel-edge"), |_, _| {})
        .width(Length::Fixed(wide))
        .height(Length::Fill)
        .on_pointer(|at| Msg::Panel(PanelMsg::Edge(at)))
        .cursor_with(|_, _| Cursor::ColResize)
}

pub fn view(f: &Files, width: f32) -> Node<Msg> {
    let tall = theme::sheet().preview_picture;
    let mut parts: Vec<Node<Msg>> = Vec::new();
    match &f.panel.about {
        None => {
            parts.push(loading_state(tall));
            parts.push(heading("").room_for(2).width(Length::Fill));
        }
        Some(about) => {
            parts.push(match &about.seen {
                Seen::Waiting => loading_state(tall),
                Seen::Picture(img, fitted) => picture(img.clone(), fitted.clone(), tall, &about.name),
                Seen::Text(text) => words(text.clone(), tall, &about.name),
                Seen::Plain => icon(about.icon, about.folder.then_some(about.tint), None, tall),
                Seen::Trouble(why) => column(vec![
                    icon(about.icon, about.folder.then_some(about.tint), Some(why.clone()), tall),
                    row(vec![crate::toolkit::widget::space().width(Length::Fill), crate::common::more::button(Msg::Help(super::help::FILE_UNREAD)), crate::toolkit::widget::space().width(Length::Fill)]).width(Length::Fill),
                ]).gap(theme::sheet().gap).width(Length::Fill),
            });
            parts.push(heading(about.name.clone()).room_for(2).width(Length::Fill));
            for (key, said) in &about.facts {
                parts.push(row(vec![
                    note(key.to_string()).width(Length::Fixed(FACT_W)),
                    label(said.clone()).cut().width(Length::Fill),
                ]).gap(8.0).width(Length::Fill));
            }
        }
    }
    let inner = column(parts).gap(theme::sheet().gap).width(Length::Fill);
    card(scroll("preview-panel", inner).width(Length::Fill).height(Length::Fill)).width(Length::Fixed(width)).height(Length::Fill)
}

fn picture(img: Rc<DynamicImage>, fitted: Fitted, tall: f32, name: &str) -> Node<Msg> {
    let (iw, ih) = img.dimensions();
    drawing((0.0, tall), t!("folders-panel-picture-name", name = name), move |canvas, _| {
        let want = (canvas.width(), canvas.height());
        let mut kept = fitted.borrow_mut();
        if kept.as_ref().map(|(size, _)| *size) != Some(want) {
            let k = (want.0 as f32 / iw as f32).min(want.1 as f32 / ih as f32).min(1.0);
            let (fw, fh) = (((iw as f32 * k).round() as u32).max(1), ((ih as f32 * k).round() as u32).max(1));
            let made = if (fw, fh) == (iw, ih) { None } else { Some(img.thumbnail_exact(fw, fh)) };
            *kept = crate::toolkit::picture::to_pixmap(made.as_ref().unwrap_or(&img)).map(|px| (want, px));
        }
        if let Some((_, px)) = kept.as_ref() {
            let (x, y) = ((want.0 as i32 - px.width() as i32) / 2, (want.1 as i32 - px.height() as i32) / 2);
            canvas.draw_pixmap(x, y, px.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        }
    }).width(Length::Fill).height(Length::Fixed(tall))
}

fn words(text: Rc<str>, tall: f32, name: &str) -> Node<Msg> {
    drawing((0.0, tall), t!("folders-panel-text-name", name = name), move |canvas, pen| {
        let style = Style::new((theme::sheet().note.0 * pen.scale).round());
        let laid = pen.fonts.layout(&text, &style, Some(canvas.width() as f32));
        pen.fonts.draw(canvas, &laid, 0.0, 0.0, pen.theme.muted);
    }).width(Length::Fill).height(Length::Fixed(tall))
}

fn icon(name: &'static str, folder: Option<Option<Colour>>, why: Option<String>, tall: f32) -> Node<Msg> {
    const SIZE: f32 = 72.0;
    let tip = why.clone().unwrap_or_default();
    drawing((0.0, tall), tip, move |canvas, pen| {
        let s = (SIZE * pen.scale).round();
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        let said = why.as_ref().map(|why| {
            let style = Style::new((theme::sheet().note.0 * pen.scale).round());
            pen.fonts.layout(why, &style, Some(w))
        });
        let below = said.as_ref().map_or(0.0, |l| l.height + 8.0 * pen.scale);
        let top = ((h - s - below) / 2.0).max(0.0).round();
        let t = pen.theme;
        let ink = folder.map_or(t.muted, |own| t.folder_ink(own, &[t.bg, t.panel]));
        pen.icons.draw(canvas, name, ((w - s) / 2.0).round() as i32, top as i32, s as u32, 1, ink);
        if let Some(laid) = &said {
            pen.fonts.draw(canvas, laid, ((w - laid.width) / 2.0).max(0.0).round(), top + s + 8.0 * pen.scale, pen.theme.muted);
        }
    }).width(Length::Fill).height(Length::Fixed(tall))
}

#[cfg(test)]
type PanelShown = (String, Vec<(&'static str, String)>, String);

#[cfg(test)]
impl Files {
    pub(super) fn panel_shown(&self) -> Option<PanelShown> {
        let a = self.panel.about.as_ref()?;
        let seen = match &a.seen {
            Seen::Waiting => "waiting".to_string(),
            Seen::Picture(img, _) => format!("picture {}x{}", img.width(), img.height()),
            Seen::Text(t) => format!("text {t}"),
            Seen::Plain => "plain".to_string(),
            Seen::Trouble(w) => format!("words {w}"),
        };
        Some((a.name.clone(), a.facts.clone(), seen))
    }

    pub(super) fn panel_waits(&mut self) {
        if let Some(a) = &mut self.panel.about {
            a.seen = Seen::Waiting;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::app::View;
    use crate::files::preview::PreviewMsg;
    use crate::toolkit::ui::App;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-files-panel-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Letters/Old")).unwrap();
        image::RgbImage::from_pixel(300, 100, image::Rgb([200, 120, 40])).save(dir.join("beach.png")).unwrap();
        std::fs::write(dir.join("notes.txt"), format!("first line\nsecond line\n\x1b[31mthird\x07 line\n{}", "a made-up line of plain words\n".repeat(4))).unwrap();
        std::fs::write(dir.join("Letters/one.txt"), vec![b'x'; 1_000]).unwrap();
        std::fs::write(dir.join("Letters/Old/two.txt"), vec![b'x'; 2_000]).unwrap();
        dir
    }

    fn files(dir: &Path, two: bool) -> Files {
        let mut f = Files::new(vec![dir.to_path_buf()]);
        f.update(Msg::PanesAre(two));
        f.go(0, Where::Folder(dir.to_path_buf()));
        f
    }

    fn pick(f: &mut Files, p: usize, names: &[&str]) {
        let picked: Vec<usize> = names.iter().map(|n| f.cols[p].entries.iter().position(|e| e.name == *n).unwrap()).collect();
        f.update(Msg::Choose(p, picked[0]));
        f.update(Msg::Pick(p, picked));
    }

    fn fact(f: &Files, key: &str) -> Option<String> {
        f.panel_shown()?.1.into_iter().find(|(k, _)| *k == key).map(|(_, w)| w)
    }

    #[test]
    fn a_folder_is_never_called_a_folder_in_words() {
        use crate::files::app::View;
        use crate::files::columns::Col;
        use crate::files::places::Entry;
        assert_eq!(crate::files::kinds::said_kind(true, || "Folder".into()), "");
        assert_eq!(crate::files::kinds::said_kind(false, || "PNG image".into()), "PNG image");
        let dir = scratch("no-word");
        let mut f = files(&dir, true);
        f.go(1, Where::Folder(dir.join("Letters")));
        let said = |f: &Files, e: &Entry| f.cache.cells(e, &Col::ALL);
        let mut folders = 0;
        for view in [View::List, View::Icons, View::Compact] {
            f.view = view;
            for p in 0..2 {
                for e in &f.cols[p].entries {
                    let words = said(&f, e);
                    assert!(!words.iter().any(|w| w.trim().eq_ignore_ascii_case("folder")), "{view:?}, pane {p}, {}: {words:?}", e.name);
                    let at = Col::ALL.iter().position(|c| *c == Col::Type).unwrap();
                    if e.folder {
                        folders += 1;
                        assert_eq!(words[at], "", "{}: the Type cell is empty", e.name);
                    } else if e.name == "beach.png" {
                        assert!(!f.cache.kind_of(e).is_empty() && words[at] == f.cache.kind_of(e), "a file's Type cell still says its kind");
                    }
                }
            }
        }
        assert!(folders >= 6, "folders were looked at in every view and both panes");
        for made_up in [Where::Recent, Where::Found(dir.clone(), "old".into(), false)] {
            let e = Entry { name: "Old".into(), path: dir.join("Letters/Old"), folder: true, size: 0, when: 0, kind: None };
            assert!(said(&f, &e).iter().all(|w| !w.eq_ignore_ascii_case("folder")), "{made_up:?}");
        }
        f.update(Msg::Panel(PanelMsg::Switch));
        pick(&mut f, 0, &["Letters"]);
        assert_eq!(f.panel_shown().unwrap().0, "Letters");
        assert!(f.panel_shown().unwrap().1.iter().all(|(k, w)| *k != "Type" && !w.eq_ignore_ascii_case("folder")), "{:?}", f.panel_shown().unwrap().1);
        f.update(Msg::Unpick);
        assert!(f.panel_shown().unwrap().1.iter().all(|(k, w)| *k != "Type" && !w.eq_ignore_ascii_case("folder")), "{:?}", f.panel_shown().unwrap().1);
        pick(&mut f, 0, &["beach.png"]);
        assert!(fact(&f, "Type").is_none(), "beach.png: the ending says it is a PNG image, so the panel does not");
        assert_eq!(crate::files::kinds::said_kind_beside(false, "beach.png", || "PNG image".into()), "");
        assert_eq!(crate::files::kinds::said_kind_beside(false, "notes.txt", || "Plain text document".into()), "");
        assert_eq!(crate::files::kinds::said_kind_beside(true, "Letters", || "Folder".into()), "");
        assert_eq!(crate::files::kinds::said_kind_beside(false, "Makefile", || "Makefile".into()), "Makefile");
        assert_eq!(crate::files::kinds::said_kind_beside(false, ".profile", || "Shell script".into()), "Shell script");
        assert_eq!(crate::files::kinds::said_kind_beside(false, "README", || "Text".into()), "Text");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_panel_follows_what_is_chosen_and_the_space_preview_still_works() {
        let dir = scratch("follows");
        let mut f = files(&dir, false);
        pick(&mut f, 0, &["beach.png"]);
        assert!(f.panel_shown().is_none() && !f.panel_shows());
        f.update(Msg::Panel(PanelMsg::Switch));
        assert!(f.kept.panel && f.panel_shows());
        let (name, _, seen) = f.panel_shown().unwrap();
        assert_eq!((name.as_str(), seen.as_str()), ("beach.png", "picture 300x100"));
        assert_eq!(fact(&f, "Pixels").as_deref(), Some("300 by 100"));
        assert!(fact(&f, "Size").is_some() && fact(&f, "Type").is_none() && fact(&f, "Changed").is_some(), "the ending says what kind it is");
        assert!(fact(&f, "Where").is_some_and(|w| w.ends_with(&dir.display().to_string()) || w.starts_with("Home")));
        pick(&mut f, 0, &["notes.txt"]);
        assert!(f.panel_shown().unwrap().2.starts_with("text first line\nsecond line\n[31mthird line\na made-up line"), "{}", f.panel_shown().unwrap().2);
        pick(&mut f, 0, &["Letters"]);
        assert_eq!(f.panel_shown().unwrap().2, "plain");
        assert_eq!((fact(&f, "Contains").as_deref(), fact(&f, "Size").as_deref()), (Some("3 items in all"), Some("3.0 KB")));
        pick(&mut f, 0, &["beach.png", "notes.txt"]);
        let (name, facts, _) = f.panel_shown().unwrap();
        assert_eq!(name, "2 selected");
        assert_eq!(facts.len(), 1);
        f.update(Msg::Unpick);
        let (name, _, _) = f.panel_shown().unwrap();
        assert_eq!(name, f.place_name(&Where::Folder(dir.clone())));
        assert_eq!(fact(&f, "Contains").as_deref(), Some("6 items in all"));
        pick(&mut f, 0, &["beach.png"]);
        f.update(Msg::Preview(PreviewMsg::Toggle));
        assert!(matches!(f.dialog, Some(crate::files::dialogs::Dialog::Preview(_))));
        f.update(Msg::Preview(PreviewMsg::Toggle));
        assert!(f.dialog.is_none() && f.panel_shown().unwrap().0 == "beach.png");
        f.update(Msg::Panel(PanelMsg::Switch));
        assert!(!f.kept.panel && f.panel_shown().is_none());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn what_is_read_for_something_no_longer_chosen_is_dropped() {
        let dir = scratch("late");
        let mut f = files(&dir, false);
        f.update(Msg::Panel(PanelMsg::Switch));
        pick(&mut f, 0, &["beach.png"]);
        let old = f.panel.id;
        pick(&mut f, 0, &["notes.txt"]);
        assert!(f.panel.id > old);
        let late = Got { seen: Read::Trouble("late".into()), facts: vec![("Pixels", "1 by 1".into())] };
        f.update(Msg::Panel(PanelMsg::Got(old, Arc::new(Mutex::new(Some(late))))));
        let (name, _, seen) = f.panel_shown().unwrap();
        assert!(name == "notes.txt" && seen.starts_with("text") && fact(&f, "Pixels").is_none());
        let slot = Slot::default();
        let ask = |id| Asked { id, want: Want::File(dir.join("beach.png")), stop: Arc::new(AtomicBool::new(false)) };
        assert!(slot.put(ask(1)).is_none());
        let dropped = slot.put(ask(2)).expect("the first gives way");
        assert!(dropped.id == 1 && dropped.stop.load(Ordering::Relaxed));
        assert_eq!(slot.take().map(|a| a.id), Some(2));
        slot.close();
        assert!(slot.take().is_none());
        pick(&mut f, 0, &["Letters"]);
        let stop = f.panel.stop.clone();
        pick(&mut f, 0, &["notes.txt"]);
        assert!(stop.load(Ordering::Relaxed) && !f.panel.stop.load(Ordering::Relaxed));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_panel_serves_the_active_pane_in_dual() {
        let dir = scratch("dual");
        let mut f = files(&dir, true);
        f.room = (1400.0, 800.0);
        f.go(1, Where::Folder(dir.join("Letters")));
        f.update(Msg::Panel(PanelMsg::Switch));
        pick(&mut f, 0, &["beach.png"]);
        assert_eq!(f.panel_shown().unwrap().0, "beach.png");
        pick(&mut f, 1, &["one.txt"]);
        assert_eq!((f.side(), f.panel_shown().unwrap().0.as_str()), (1, "one.txt"));
        f.update(Msg::OtherPane);
        assert!((f.side(), f.cols[0].picked.is_empty()) == (0, true));
        pick(&mut f, 0, &["beach.png"]);
        assert_eq!((f.side(), f.panel_shown().unwrap().0.as_str()), (0, "beach.png"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_panel_needs_a_window_wide_enough_and_says_so_when_it_is_not() {
        let (one, two) = (least_window(false), least_window(true));
        assert!(one < two && two <= 1200.0, "two panes and the panel fit the window Folders opens with: {one} {two}");
        assert_eq!(width_in(one - 1.0, false, 300.0), None);
        assert_eq!(width_in(one, false, 300.0), Some(LEAST));
        assert_eq!(width_in(2000.0, false, 300.0), Some(300.0));
        assert_eq!(width_in(2000.0, true, 900.0), Some(MOST));
        assert_eq!(width_in(2000.0, true, 10.0), Some(LEAST));
        assert_eq!(width_in(one + 20.0, false, 400.0), Some(LEAST + 20.0));
        let dir = scratch("narrow");
        let mut f = files(&dir, false);
        f.room = (one - 10.0, 700.0);
        f.update(Msg::Panel(PanelMsg::Switch));
        assert!(!f.kept.panel && f.said.contains("too narrow") && !f.bad, "{}", f.said);
        f.room = (1200.0, 700.0);
        f.update(Msg::Panel(PanelMsg::Switch));
        assert!(f.panel_shows());
        assert!(f.resized(one - 10.0, 700.0), "the window is laid out again");
        assert!(f.kept.panel && !f.panel_shows() && f.panel_shown().is_none());
        assert!(f.resized(1200.0, 700.0));
        assert!(f.panel_shows() && f.panel_shown().is_some());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_edge_is_dragged_or_pressed_and_the_width_is_kept() {
        use crate::files::kept::{read_panel, Kept, PANEL};
        let dir = scratch("edge");
        let store = dir.join("kept");
        let mut f = files(&dir, false);
        f.kept = Kept::from(Some(store.clone()));
        f.room = (1400.0, 800.0);
        f.update(Msg::Panel(PanelMsg::Switch));
        assert_eq!(f.panel_room(), Some(300.0));
        assert_eq!(read_panel(&std::fs::read_to_string(store.join(PANEL)).unwrap()), (true, None));
        f.pointer.set((1000.0, 300.0));
        f.update(Msg::Panel(PanelMsg::Edge(Pointer::Down(2.0, 300.0))));
        f.pointer.set((940.0, 300.0));
        f.update(Msg::Panel(PanelMsg::Edge(Pointer::Move(2.0, 300.0))));
        assert_eq!(f.panel_room(), Some(360.0));
        f.pointer.set((100.0, 300.0));
        f.update(Msg::Panel(PanelMsg::Edge(Pointer::Move(2.0, 300.0))));
        assert_eq!(f.panel_room(), Some(MOST));
        f.pointer.set((960.0, 300.0));
        f.update(Msg::Panel(PanelMsg::Edge(Pointer::Move(2.0, 300.0))));
        f.update(Msg::Panel(PanelMsg::Edge(Pointer::Up(2.0, 300.0))));
        assert_eq!(f.panel_room(), Some(340.0));
        assert_eq!(read_panel(&std::fs::read_to_string(store.join(PANEL)).unwrap()), (true, Some(340.0)));
        for want in [MOST, LEAST, 300.0] {
            f.update(Msg::Panel(PanelMsg::Edge(Pointer::Down(2.0, 300.0))));
            f.update(Msg::Panel(PanelMsg::Edge(Pointer::Up(2.0, 300.0))));
            assert_eq!(f.panel_room(), Some(want));
        }
        let again = Kept::from(Some(store.clone()));
        assert!(again.panel && again.panel_width() == 300.0);
        f.update(Msg::Panel(PanelMsg::Switch));
        assert_eq!(read_panel(&std::fs::read_to_string(store.join(PANEL)).unwrap()), (false, Some(300.0)));
        assert_eq!(read_panel("nonsense"), (false, None));
        assert_eq!(read_panel("on -4"), (true, None));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn choosing_things_moves_nothing_once_the_panel_is_open() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let dir = scratch("still");
        let mut f = files(&dir, false);
        f.view = View::List;
        f.room = (1200.0, 760.0);
        let paint = |f: &Files| {
            for _ in 0..2 {
                let mut ui = Ui::new(crate::files::view::view(f), Theme::new(false, "#fbc711"));
                let mut pix = tiny_skia::Pixmap::new(1200, 760).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
            }
            f.cols[0].width.get()
        };
        let before = paint(&f);
        f.update(Msg::Panel(PanelMsg::Switch));
        let with = paint(&f);
        assert!((before - with - 300.0 - theme::sheet().panel_gap).abs() < 1.0, "the list gives up the panel's room and its edge: {before} {with}");
        for names in [&["beach.png"][..], &["notes.txt"], &["Letters"], &["beach.png", "notes.txt"], &[]] {
            if names.is_empty() {
                f.update(Msg::Unpick);
            } else {
                pick(&mut f, 0, names);
            }
            assert_eq!(paint(&f), with, "choosing {names:?} moved the list");
        }
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn an_archive_says_how_much_it_holds() {
        let dir = scratch("archive");
        let stop = AtomicBool::new(false);
        crate::files::archive::compress(&[dir.join("Letters")], &dir.join("letters.zip"), crate::files::archive::Format::Zip, &stop, &|_, _| {}).unwrap();
        let mut f = files(&dir, false);
        f.update(Msg::Panel(PanelMsg::Switch));
        pick(&mut f, 0, &["letters.zip"]);
        assert_eq!(fact(&f, "Unpacked").as_deref(), Some("3.0 KB"));
        assert!(fact(&f, "Contains").is_some_and(|h| h.ends_with("items")));
        let _ = std::fs::remove_dir_all(dir);
    }
}
