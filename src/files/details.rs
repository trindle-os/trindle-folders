// The part of Properties that can be changed for one file.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::app::Msg;
use super::dialogs::{toggle, AskMsg};
use super::tools::keyed;
use super::meta::{self, ExifEdit, Flags, Id3, Tag};
use super::remote::{civil, unix_time};
use crate::common::clock::Local;
use crate::toolkit::t;
use crate::toolkit::widget::{button, choice, column, entry, form_label, label, row, scroll, space, Length, Node, Tone};

const EXIF_H: f32 = 160.0;
const DATE: &str = "%Y-%m-%d %H:%M";
const PHOTOS: [&str; 9] = ["jpg", "jpeg", "tif", "tiff", "png", "webp", "heic", "heif", "avif"];
const WRITABLE: [&str; 3] = ["jpg", "jpeg", "png"];
fn turn_words() -> [(u16, &'static str); 4] {
    [(1, t!("folders-details-turn-upright")), (6, t!("folders-details-turn-right")), (3, t!("folders-details-turn-down")), (8, t!("folders-details-turn-left"))]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum When {
    Modified,
    Opened,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Song {
    Title,
    Artist,
    Album,
    Year,
    Track,
    Genre,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Note {
    Taken,
    Description,
    Artist,
    Copyright,
}

fn note_words() -> [(Note, &'static str); 4] {
    [(Note::Taken, t!("folders-details-note-taken")), (Note::Description, t!("folders-details-note-description")), (Note::Artist, t!("folders-details-note-artist")), (Note::Copyright, t!("folders-details-note-copyright"))]
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Photo {
    taken: String,
    description: String,
    artist: String,
    copyright: String,
    turn: u16,
    located: bool,
}

fn song_words() -> [(Song, &'static str); 6] {
    [
        (Song::Title, t!("folders-details-song-title")),
        (Song::Artist, t!("folders-details-song-artist")),
        (Song::Album, t!("folders-details-song-album")),
        (Song::Year, t!("folders-details-song-year")),
        (Song::Track, t!("folders-details-song-track")),
        (Song::Genre, t!("folders-details-song-genre")),
    ]
}

#[derive(Clone)]
pub enum DetailMsg {
    ReadOnly(bool),
    Runnable(bool),
    Date(When, String),
    Song(Song, String),
    SaveSong,
    Note(Note, String),
    Turn(usize),
    DropLocation(bool),
    SavePhoto,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kept {
    Flags,
    Date(When),
    Song,
    Photo,
}

pub struct Details {
    pub path: PathBuf,
    flags: Option<Flags>,
    folder: bool,
    modified: String,
    opened: String,
    exif: Vec<Tag>,
    song: Option<Id3>,
    song_was: Option<Id3>,
    photo: Option<Photo>,
    photo_was: Option<Photo>,
    kept: Option<Kept>,
    bad: Option<String>,
}

fn date_text(t: Option<SystemTime>) -> String {
    t.and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| Local::at(d.as_secs() as i64).format(DATE)).unwrap_or_default()
}

fn parse_date(text: &str) -> Option<SystemTime> {
    let (day, time) = text.trim().split_once(' ')?;
    let widths = day.split('-').chain(time.trim().split(':')).map(str::len).collect::<Vec<_>>();
    if widths != [4, 2, 2, 2, 2] {
        return None;
    }
    let mut d = day.split('-').map(|n| n.parse::<i64>().ok());
    let (y, mo, dd) = (d.next()??, d.next()??, d.next()??);
    let mut t = time.trim().split(':').map(|n| n.parse::<i64>().ok());
    let (h, mi) = (t.next()??, t.next()??);
    if d.next().is_some() || t.next().is_some() || !(1970..=9999).contains(&y) || !(1..=12).contains(&mo) || !(1..=31).contains(&dd) || !(0..24).contains(&h) || !(0..60).contains(&mi) {
        return None;
    }
    let wall = unix_time(y, mo as u32, dd as u32, h as u32, mi as u32, 0);
    if civil(wall) != (y, mo as u32, dd as u32) {
        return None;
    }
    let guess = wall - Local::at(wall).utc_offset();
    let unix = wall - Local::at(guess).utc_offset();
    (unix >= 0).then(|| UNIX_EPOCH + Duration::from_secs(unix as u64))
}

fn song_field(s: &mut Id3, f: Song) -> &mut String {
    match f {
        Song::Title => &mut s.title,
        Song::Artist => &mut s.artist,
        Song::Album => &mut s.album,
        Song::Year => &mut s.year,
        Song::Track => &mut s.track,
        Song::Genre => &mut s.genre,
    }
}

fn note_field(p: &mut Photo, n: Note) -> &mut String {
    match n {
        Note::Taken => &mut p.taken,
        Note::Description => &mut p.description,
        Note::Artist => &mut p.artist,
        Note::Copyright => &mut p.copyright,
    }
}

fn taken_ok(text: &str) -> bool {
    let b = text.trim().as_bytes();
    b.len() == 19 && b.iter().enumerate().all(|(i, c)| match i {
        4 | 7 => *c == b'-' || *c == b':',
        10 => *c == b' ',
        13 | 16 => *c == b':',
        _ => c.is_ascii_digit(),
    })
}

fn read_photo(path: &Path) -> Photo {
    let mut p = Photo { turn: 1, ..Photo::default() };
    let Ok(file) = crate::common::files::open_file(path) else { return p };
    let Ok(e) = exif::Reader::new().read_from_container(&mut std::io::BufReader::new(file)) else { return p };
    let text = |tag| match e.get_field(tag, exif::In::PRIMARY).map(|f| &f.value) {
        Some(exif::Value::Ascii(parts)) => parts.first().map(|b| String::from_utf8_lossy(b).trim_end_matches('\0').trim().to_string()).unwrap_or_default(),
        _ => String::new(),
    };
    p.taken = text(exif::Tag::DateTimeOriginal);
    if p.taken.len() == 19 {
        p.taken.replace_range(4..5, "-");
        p.taken.replace_range(7..8, "-");
    }
    p.description = text(exif::Tag::ImageDescription);
    p.artist = text(exif::Tag::Artist);
    p.copyright = text(exif::Tag::Copyright);
    p.turn = e.get_field(exif::Tag::Orientation, exif::In::PRIMARY).and_then(|f| f.value.get_uint(0)).map_or(1, |n| n as u16);
    p.located = e.fields().any(|f| matches!(f.tag.context(), exif::Context::Gps));
    p
}

fn photo_edit(was: &Photo, now: &Photo) -> ExifEdit {
    let changed = |a: &String, b: &String| (a.trim() != b.trim()).then(|| b.trim().to_string());
    ExifEdit {
        taken: changed(&was.taken, &now.taken),
        description: changed(&was.description, &now.description),
        artist: changed(&was.artist, &now.artist),
        copyright: changed(&was.copyright, &now.copyright),
        orientation: (was.turn != now.turn).then_some(now.turn),
        drop_location: was.located && !now.located,
    }
}

fn turns(now: u16) -> Vec<(u16, &'static str)> {
    let mut all = turn_words().to_vec();
    if !all.iter().any(|(n, _)| *n == now) {
        all.push((now, t!("folders-details-turn-mirrored")));
    }
    all
}

fn ending(path: &Path) -> String {
    path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default()
}

impl Details {
    pub fn load(path: &Path) -> Option<Details> {
        let m = std::fs::symlink_metadata(path).ok()?;
        if m.file_type().is_symlink() {
            return None;
        }
        let dates = meta::dates(path).unwrap_or_default();
        let end = ending(path);
        let exif = if m.is_file() && PHOTOS.contains(&end.as_str()) { meta::exif(path) } else { Vec::new() };
        let song = if m.is_file() && end == "mp3" { Some(meta::read_id3(path).ok().flatten().unwrap_or_default()) } else { None };
        let photo = (m.is_file() && WRITABLE.contains(&end.as_str())).then(|| read_photo(path));
        Some(Details {
            path: path.to_path_buf(),
            flags: meta::flags(path).ok(),
            folder: m.is_dir(),
            modified: date_text(dates.modified),
            opened: date_text(dates.accessed),
            exif,
            song_was: song.clone(),
            song,
            photo_was: photo.clone(),
            photo,
            kept: None,
            bad: None,
        })
    }

    pub fn update(&mut self, m: DetailMsg) -> bool {
        self.bad = None;
        self.kept = None;
        self.change(m);
        self.kept.is_some()
    }

    fn change(&mut self, m: DetailMsg) {
        match m {
            DetailMsg::ReadOnly(on) => self.set_flag(|f| f.read_only = on),
            DetailMsg::Runnable(on) => self.set_flag(|f| f.executable = on),
            DetailMsg::Date(which, text) => self.set_date(which, text),
            DetailMsg::Song(f, text) => {
                if let Some(s) = &mut self.song {
                    *song_field(s, f) = text;
                }
            }
            DetailMsg::SaveSong => self.save_song(),
            DetailMsg::Note(n, text) => {
                if let Some(p) = &mut self.photo {
                    *note_field(p, n) = text;
                }
            }
            DetailMsg::Turn(i) => {
                if let Some(p) = &mut self.photo {
                    if let Some((n, _)) = turns(self.photo_was.as_ref().map_or(1, |w| w.turn)).get(i) {
                        p.turn = *n;
                    }
                }
            }
            DetailMsg::DropLocation(drop) => {
                if let (Some(p), Some(w)) = (&mut self.photo, &self.photo_was) {
                    p.located = w.located && !drop;
                }
            }
            DetailMsg::SavePhoto => self.save_photo(),
        }
    }

    fn set_flag(&mut self, edit: impl FnOnce(&mut Flags)) {
        let Some(mut want) = self.flags else { return };
        edit(&mut want);
        match meta::set_flags(&self.path, want) {
            Ok(()) => {
                self.flags = meta::flags(&self.path).ok();
                self.kept = Some(Kept::Flags);
            }
            Err(e) => self.bad = Some(t!("folders-details-change-failed", error = &e.to_string())),
        }
    }

    fn set_date(&mut self, which: When, text: String) {
        let at = parse_date(&text);
        if which == When::Modified { self.modified = text } else { self.opened = text }
        let Some(at) = at else { return };
        let (modified, opened) = if which == When::Modified { (Some(at), None) } else { (None, Some(at)) };
        match meta::set_dates(&self.path, modified, opened) {
            Ok(()) => self.kept = Some(Kept::Date(which)),
            Err(e) => self.bad = Some(t!("folders-details-date-failed", error = &e.to_string())),
        }
    }

    fn save_song(&mut self) {
        let Some(s) = &self.song else { return };
        if self.song == self.song_was {
            return;
        }
        match meta::write_id3(&self.path, s) {
            Ok(()) => {
                self.song_was = self.song.clone();
                self.kept = Some(Kept::Song);
            }
            Err(e) => self.bad = Some(t!("folders-details-song-failed", error = &e.to_string())),
        }
    }

    fn save_photo(&mut self) {
        let (Some(now), Some(was)) = (&self.photo, &self.photo_was) else { return };
        if now == was {
            return;
        }
        if !now.taken.trim().is_empty() && !taken_ok(&now.taken) {
            self.bad = Some(t!("folders-details-taken-bad").into());
            return;
        }
        match meta::write_exif(&self.path, &photo_edit(was, now)) {
            Ok(()) => {
                let fresh = read_photo(&self.path);
                self.photo = Some(fresh.clone());
                self.photo_was = Some(fresh);
                self.exif = meta::exif(&self.path);
                self.kept = Some(Kept::Photo);
            }
            Err(e) => self.bad = Some(t!("folders-details-photo-failed", error = &e.to_string())),
        }
    }
}

fn detail(m: DetailMsg) -> Msg {
    Msg::Ask(AskMsg::Detail(m))
}

fn kept(yes: bool) -> u32 {
    u32::from(yes)
}

impl Details {
    pub fn has_media(&self) -> bool {
        self.song.is_some() || self.photo.is_some() || !self.exif.is_empty()
    }
}

pub fn view(d: &Details, media: bool) -> Vec<Node<Msg>> {
    let mut out = Vec::new();
    if media {
        out.extend(media_rows(d));
    } else {
        out.extend(basic_rows(d));
    }
    if let Some(b) = &d.bad {
        out.push(row(vec![label(b.clone()).tone(Tone::Problem).width(Length::Fill), crate::common::more::button(Msg::Help(super::help::DETAILS_NOT_CHANGED))]).gap(8.0).width(Length::Fill));
    }
    out
}

fn basic_rows(d: &Details) -> Vec<Node<Msg>> {
    let mut out = Vec::new();
    if let Some(f) = d.flags {
        out.push(toggle(t!("folders-details-read-only"), t!("folders-details-read-only-sub"), f.read_only, t!("folders-details-read-only-tip"), |on| detail(DetailMsg::ReadOnly(on))));
        if !d.folder {
            out.push(toggle(t!("folders-details-runnable"), t!("folders-details-runnable-sub"), f.executable, t!("folders-details-runnable-tip"), |on| detail(DetailMsg::Runnable(on))));
        }
    }
    for (which, key, text) in [(When::Modified, t!("folders-details-modified"), &d.modified), (When::Opened, t!("folders-details-opened"), &d.opened)] {
        let good = parse_date(text).is_some() || text.is_empty();
        let mut field = entry(text.clone(), t!("folders-details-date-tip", name = key, example = &Local::now().format(DATE)), move |t| detail(DetailMsg::Date(which, t)))
            .key(format!("date-{which:?}"))
            .on_escape(Msg::Ask(AskMsg::Close))
            .glow(kept(d.kept == Some(Kept::Date(which))));
        if !good {
            field = field.tone(Tone::Problem);
        }
        out.push(keyed(key, vec![field]));
    }
    out
}

fn media_rows(d: &Details) -> Vec<Node<Msg>> {
    let mut out = Vec::new();
    if let Some(s) = &d.song {
        out.push(form_label(t!("folders-details-song")));
        let mut copy = s.clone();
        for (f, key) in song_words() {
            let value = song_field(&mut copy, f).clone();
            out.push(keyed(key, vec![
                entry(value, key.to_string(), move |t| detail(DetailMsg::Song(f, t))).key(format!("song-{f:?}"))
                    .on_submit(detail(DetailMsg::SaveSong)).on_escape(Msg::Ask(AskMsg::Close)).glow(kept(d.kept == Some(Kept::Song))),
            ]));
        }
        let changed = d.song != d.song_was;
        out.push(row(vec![
            space().width(Length::Fill),
            button(crate::common::ui::SAVE_ICON, t!("folders-details-write-song")).on_press(detail(DetailMsg::SaveSong)).off(!changed),
        ]).gap(8.0).width(Length::Fill));
    }
    if let (Some(p), Some(was)) = (&d.photo, &d.photo_was) {
        out.push(form_label(t!("folders-details-photo")));
        let mut copy = p.clone();
        for (n, key) in note_words() {
            let value = note_field(&mut copy, n).clone();
            let tip = if n == Note::Taken { t!("folders-details-taken-tip").to_string() } else { key.to_string() };
            let bad = n == Note::Taken && !value.trim().is_empty() && !taken_ok(&value);
            let mut field = entry(value, tip, move |t| detail(DetailMsg::Note(n, t))).key(format!("photo-{n:?}"))
                .on_submit(detail(DetailMsg::SavePhoto)).on_escape(Msg::Ask(AskMsg::Close)).glow(kept(d.kept == Some(Kept::Photo)));
            if bad {
                field = field.tone(Tone::Problem);
            }
            out.push(keyed(key, vec![field]));
        }
        let all = turns(was.turn);
        let at = all.iter().position(|(n, _)| *n == p.turn).unwrap_or(0);
        out.push(keyed(t!("folders-details-way-up"), vec![choice(all.iter().map(|(_, w)| w.to_string()).collect(), at, t!("folders-details-way-up-tip"), |i| detail(DetailMsg::Turn(i)))]));
        if was.located {
            out.push(toggle(t!("folders-details-drop-place"), t!("folders-details-drop-place-sub"), !p.located,
                            t!("folders-details-drop-place-tip"), |on| detail(DetailMsg::DropLocation(on))));
        }
        out.push(row(vec![
            space().width(Length::Fill),
            button(crate::common::ui::SAVE_ICON, t!("folders-details-write-photo")).on_press(detail(DetailMsg::SavePhoto)).off(p == was),
        ]).gap(8.0).width(Length::Fill));
    }
    if !d.exif.is_empty() {
        out.push(form_label(t!("folders-details-camera-notes")));
        let rows = d.exif.iter().map(|t| keyed(&t.name, vec![label(t.value.clone()).width(Length::Fill)])).collect();
        out.push(scroll("camera-notes", column(rows).gap(4.0)).height(Length::Fixed(EXIF_H)));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_are_read_back_as_typed() {
        assert!(parse_date("2026-02-30 10:00").is_none());
        assert!(parse_date("2026-13-01 10:00").is_none());
        assert!(parse_date("2026-10-03").is_none());
        assert!(parse_date("2026-10-03 14:3").is_none());
        assert!(parse_date("2024-02-29 23:59").is_some());
        let t = parse_date("2026-10-03 14:30").unwrap();
        assert_eq!(date_text(Some(t)), "2026-10-03 14:30");
    }

    #[test]
    fn flags_and_dates_are_kept_at_once() {
        let dir = std::env::temp_dir().join(format!("trindle-files-details-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("note.txt");
        std::fs::write(&file, b"x").unwrap();
        let mut d = Details::load(&file).unwrap();
        assert!(d.song.is_none() && d.exif.is_empty());
        d.update(DetailMsg::Runnable(true));
        assert!(meta::flags(&file).unwrap().executable);
        d.update(DetailMsg::Date(When::Modified, "2020-01-02 03:".into()));
        assert!(d.kept.is_none(), "half a date is not kept");
        d.update(DetailMsg::Date(When::Modified, "2020-01-02 03:04".into()));
        assert!(d.kept == Some(Kept::Date(When::Modified)));
        assert_eq!(date_text(meta::dates(&file).unwrap().modified), "2020-01-02 03:04");
        std::os::unix::fs::symlink(&file, dir.join("link")).unwrap();
        assert!(Details::load(&dir.join("link")).is_none());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_photos_notes_are_written_when_asked() {
        let dir = std::env::temp_dir().join(format!("trindle-files-photo-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("pic.png");
        image::RgbImage::from_fn(4, 4, |x, y| image::Rgb([(x * 60) as u8, (y * 60) as u8, 90])).save(&file).unwrap();
        let mut d = Details::load(&file).unwrap();
        assert_eq!(d.photo.as_ref().unwrap().turn, 1);
        d.update(DetailMsg::Note(Note::Artist, "John Smith".into()));
        d.update(DetailMsg::Note(Note::Taken, "2026-01-02 03:04".into()));
        d.update(DetailMsg::SavePhoto);
        assert!(d.bad.is_some() && d.kept.is_none(), "a half date is refused");
        d.update(DetailMsg::Note(Note::Taken, "2026-01-02 03:04:05".into()));
        d.update(DetailMsg::Turn(2));
        d.update(DetailMsg::SavePhoto);
        assert!(d.kept == Some(Kept::Photo), "{:?}", d.bad);
        let back = read_photo(&file);
        assert_eq!((back.artist.as_str(), back.taken.as_str(), back.turn), ("John Smith", "2026-01-02 03:04:05", 3));
        d.update(DetailMsg::Note(Note::Artist, String::new()));
        d.update(DetailMsg::SavePhoto);
        assert_eq!(read_photo(&file).artist, "");
        assert!(Details::load(&dir.join("pic.png")).unwrap().exif.iter().any(|t| t.name == "DateTimeOriginal"));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_refused_change_has_a_more() {
        let dir = std::env::temp_dir().join(format!("trindle-files-details-more-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("a.txt");
        std::fs::write(&file, b"x").unwrap();
        let more = format!("button: {}", t!("common-more"));
        let mut d = Details::load(&file).unwrap();
        assert!(!column(view(&d, false)).said().contains(&more));
        d.bad = Some("That could not be changed: x.".into());
        assert!(column(view(&d, false)).said().contains(&more));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
