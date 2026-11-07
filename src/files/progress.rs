// Work under way, shown in the Folders window itself.

use super::act::{Asking, Job};
use super::icons::{self, stand_in};
use super::places::Entry;
use super::app::{Files, Msg};
use super::dialogs::{foot, toggle};
use super::work::Answer;
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::tiles::standard_ring_with;
use crate::toolkit::widget::{button, card, column, column_heading, drawing, form_label, label, main_action, main_word, note, placed, row, space, word_button, Length, Node, Tone};
use std::time::Instant;

const WIDTH: f32 = 520.0;
const RING: f32 = 18.0;
const FIGURE: f32 = 40.0;
pub(super) const GAP: f32 = 8.0;

fn tall() -> f32 {
    crate::toolkit::theme::icon_button()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Bytes,
    Items,
}

impl Unit {
    pub fn of(kind: &super::work::Kind) -> Unit {
        use super::work::{Extra, Kind};
        match kind {
            Kind::Trash | Kind::Delete | Kind::Other(Extra::Picture(_)) => Unit::Items,
            _ => Unit::Bytes,
        }
    }
}

const SAMPLE: f64 = 1.0;
const FRESH: f64 = 0.3;
const ENOUGH: u32 = 2;
const STEADY: f64 = 0.2;
const MANY_FILES: u64 = 50;

#[derive(Clone, Debug)]
pub struct Pace {
    unit: Unit,
    last: Option<(Instant, u64)>,
    rate: Option<f64>,
    measures: u32,
    left: Option<f64>,
    files: Option<(u64, u64)>,
}

impl Pace {
    pub fn new(unit: Unit) -> Pace {
        Pace { unit, last: None, rate: None, measures: 0, left: None, files: None }
    }

    pub fn note(&mut self, now: Instant, done: u64, total: u64) {
        let Some((then, was)) = self.last else {
            self.last = Some((now, done));
            return;
        };
        let gone = now.saturating_duration_since(then).as_secs_f64();
        if gone < SAMPLE {
            return;
        }
        let fresh = done.saturating_sub(was) as f64 / gone;
        let rate = match self.rate {
            Some(r) => r * (1.0 - FRESH) + fresh * FRESH,
            None => fresh,
        };
        self.rate = Some(rate);
        self.measures += 1;
        self.last = Some((now, done));
        if self.measures < ENOUGH || rate < 1e-6 {
            self.left = None;
            return;
        }
        let guess = total.saturating_sub(done) as f64 / rate;
        self.left = Some(match self.left {
            Some(said) => {
                let due = (said - gone).max(0.0);
                if (guess - due).abs() <= due * STEADY { due } else { guess }
            }
            None => guess,
        });
    }

    pub fn rest(&mut self) {
        self.last = None;
    }

    pub fn files(&mut self, done: u64, of: u64) {
        self.files = Some((done, of));
    }

    pub fn files_counted(&self) -> Option<(u64, u64)> {
        self.files
    }

    pub fn words(&self) -> String {
        let files = self.files.filter(|(_, of)| *of >= MANY_FILES);
        let (done, of) = files.map_or_else(Default::default, |(done, of)| (grouped(done), grouped(of)));
        let counted = if files.is_some() { "yes" } else { "no" };
        match (self.rate, self.left) {
            (Some(rate), Some(left)) => {
                let rate = match self.unit {
                    Unit::Bytes => speed_words(rate),
                    Unit::Items => super::view::items_words(rate.round().max(1.0) as u64),
                };
                t!("folders-progress-pace-known", files = counted, done = &done, of = &of, rate = &rate, left = &left_words(left))
            }
            _ => t!("folders-progress-pace-wait", files = counted, done = &done, of = &of),
        }
    }
}

fn grouped(n: u64) -> String {
    crate::common::thousands(n)
}

fn speed_words(rate: f64) -> String {
    let mut n = rate;
    for unit in 0..4 {
        if n < 999.5 || unit == 3 {
            let figure = trindle_toolkit::lang::format::decimal(n, if unit != 0 && n < 9.95 { 1 } else { 0 });
            return match unit {
                0 => t!("folders-progress-speed-bytes", n = &figure),
                1 => t!("folders-progress-speed-kb", n = &figure),
                2 => t!("folders-progress-speed-mb", n = &figure),
                _ => t!("folders-progress-speed-gb", n = &figure),
            };
        }
        n /= 1000.0;
    }
    String::new()
}

fn left_words(secs: f64) -> String {
    let mins = (secs / 60.0).round() as u64;
    if secs < 8.0 {
        t!("folders-progress-left-few").to_string()
    } else if secs < 50.0 {
        t!("folders-progress-left-seconds", n = ((secs / 10.0).round() as u64).max(1) * 10)
    } else if mins <= 1 {
        t!("folders-progress-left-minute").to_string()
    } else if mins < 90 {
        t!("folders-progress-left-minutes", n = mins)
    } else {
        t!("folders-progress-left-hours", n = (mins as f64 / 60.0).round() as u64)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ended {
    pub id: u64,
    pub slot: usize,
    pub pane: usize,
    pub words: String,
    pub bad: bool,
    pub help: Option<&'static str>,
    pub stays: bool,
}

pub(super) fn done_for() -> std::time::Duration {
    theme::sheet().work_done
}

fn foot_of(f: &Files, pane: usize) -> usize {
    if f.two { pane } else { f.side() }
}

fn fit(room: (f32, f32)) -> usize {
    ((room.1 / 2.0 / (tall() + GAP)).floor() as usize).max(1)
}

pub(super) fn free_slot(f: &Files, pane: usize) -> usize {
    let at = foot_of(f, pane);
    let taken: Vec<usize> = f.jobs.iter().filter(|j| foot_of(f, j.pane) == at).map(|j| j.slot)
        .chain(f.ended.iter().filter(|e| foot_of(f, e.pane) == at).map(|e| e.slot)).collect();
    (0..).find(|s| !taken.contains(s)).unwrap_or(0)
}

enum Shown<'a> {
    Going(&'a Job),
    Over(&'a Ended),
}

fn placed_rows(f: &Files, p: usize) -> Vec<(usize, Shown<'_>)> {
    let mut all: Vec<(usize, u64, Shown)> = f.jobs.iter().filter(|j| foot_of(f, j.pane) == p).map(|j| (j.slot, j.id, Shown::Going(j)))
        .chain(f.ended.iter().filter(|e| foot_of(f, e.pane) == p).map(|e| (e.slot, e.id, Shown::Over(e)))).collect();
    all.sort_by_key(|(slot, id, _)| (*slot, *id));
    let mut out: Vec<(usize, Shown)> = Vec::new();
    for (slot, _, s) in all {
        let at = out.last().map_or(slot, |(last, _)| slot.max(last + 1));
        out.push((at, s));
    }
    out
}

pub(super) fn rows_in(f: &Files, p: usize) -> usize {
    placed_rows(f, p).last().map_or(0, |(at, _)| (at + 1).min(fit(f.room)))
}

pub(super) fn rows(f: &Files, p: usize, line: Node<Msg>) -> Vec<Node<Msg>> {
    let fit = fit(f.room);
    let placed = placed_rows(f, p);
    let over = placed.iter().filter(|(at, _)| at + 1 >= fit).count();
    let mut by_place: Vec<Option<Node<Msg>>> = (0..rows_in(f, p).max(1)).map(|_| None).collect();
    for (at, s) in &placed {
        if *at + 1 >= fit && over > 1 {
            continue;
        }
        by_place[(*at).min(fit - 1)] = Some(match s {
            Shown::Going(j) => job(j),
            Shown::Over(e) => ended(e),
        });
    }
    if over > 1 {
        by_place[fit - 1] = Some(label(t!("folders-progress-more", n = over)).cut().width(Length::Fill).height(Length::Fixed(tall())));
    }
    if by_place[0].is_none() {
        by_place[0] = Some(line);
    }
    by_place.into_iter().rev().map(|n| n.unwrap_or_else(|| space().width(Length::Fill).height(Length::Fixed(tall())))).collect()
}

fn rows_tall(f: &Files, p: usize) -> f32 {
    rows_in(f, p).saturating_sub(1) as f32 * (tall() + GAP)
}

pub(super) fn layers(f: &Files) -> Vec<Node<Msg>> {
    let Some(j) = f.jobs.iter().filter(|j| j.asking.is_some()).min_by_key(|j| (j.slot, j.id)) else { return Vec::new() };
    let s = theme::sheet();
    let right = s.panel_gap + s.card_padding;
    let width = WIDTH.min(f.room.0 - 2.0 * right);
    let under = s.panel_gap + 2.0 * s.card_padding + super::view::STATUS_H + rows_tall(f, foot_of(f, j.pane)) + GAP;
    vec![placed(question(f, j).width(Length::Fixed(width)), move |(w, h), (aw, ah)| {
        (aw - right - w, (ah - under - h).max(0.0))
    })]
}

pub(super) fn heavy_words(j: &Job) -> Option<String> {
    heavy_by(j, std::time::Instant::now())
}

fn heavy_by(j: &Job, now: Instant) -> Option<String> {
    if j.from.is_empty() || j.paused {
        return None;
    }
    let (mb, after) = theme::sheet().work_heavy;
    if (j.total as f64) < mb as f64 * 1e6 && now.saturating_duration_since(j.began) < after {
        return None;
    }
    let verb = if j.words.doing.starts_with("Moving") { "moving" } else { "copying" };
    Some(t!("folders-progress-heavy", verb = verb))
}

fn how(j: &Job) -> String {
    if j.asking.is_some() {
        t!("folders-progress-waiting").to_string()
    } else if j.paused {
        t!("folders-progress-paused").to_string()
    } else {
        j.pace.words()
    }
}

pub(super) fn job(j: &Job) -> Node<Msg> {
    let id = j.id;
    let (icon, tip) = if j.paused { ("media-playback-start-symbolic", t!("folders-progress-carry-on")) } else { ("media-playback-pause-symbolic", t!("folders-progress-pause")) };
    let line = t!("folders-progress-line", doing = &j.words.doing, how = how(j));
    let words = match heavy_words(j) {
        Some(w) => t!("folders-progress-line-heavy", line = line, heavy = w),
        None => line,
    };
    row(vec![
        standard_ring_with(Some(j.share()), None, RING).width(Length::Shrink),
        label(format!("{:.0}%", j.share() * 100.0)).width(Length::Fixed(FIGURE)),
        label(words).cut().width(Length::Fill),
        button(icon, tip).on_press(Msg::Pause(id)).off(j.asking.is_some()),
        button("process-stop-symbolic", t!("folders-progress-stop")).on_press(Msg::Cancel(id)),
    ]).gap(8.0).width(Length::Fill).height(Length::Fixed(tall())).centred()
}

fn ended(e: &Ended) -> Node<Msg> {
    let words = label(e.words.clone()).cut().width(Length::Fill);
    let mut line = vec![if e.bad { words.tone(Tone::Problem) } else { words }];
    line.extend(e.help.map(|a| crate::common::more::button(Msg::Help(a))));
    row(line).gap(8.0)
        .width(Length::Fill).height(Length::Fixed(tall())).centred()
}

const PICTURE: f32 = 48.0;

pub(super) fn marks(coming: &Entry, there: &Entry) -> (String, String) {
    let (mut c, mut t) = ((false, false), (false, false));
    match coming.when.cmp(&there.when) {
        std::cmp::Ordering::Greater => c.0 = true,
        std::cmp::Ordering::Less => t.0 = true,
        std::cmp::Ordering::Equal => {}
    }
    if !coming.folder && !there.folder {
        match coming.size.cmp(&there.size) {
            std::cmp::Ordering::Greater => c.1 = true,
            std::cmp::Ordering::Less => t.1 = true,
            std::cmp::Ordering::Equal => {}
        }
    }
    (mark_words(c), mark_words(t))
}

fn mark_words((newer, larger): (bool, bool)) -> String {
    match (newer, larger) {
        (true, true) => t!("folders-progress-mark-newer-larger").to_string(),
        (true, false) => t!("folders-progress-mark-newer").to_string(),
        (false, true) => t!("folders-progress-mark-larger").to_string(),
        (false, false) => String::new(),
    }
}

fn side(f: &Files, head: &str, e: &Entry, holds: Option<u64>, marks: String) -> Node<Msg> {
    let size = match (e.folder, holds) {
        (true, Some(n)) => Asking::holds_words(n),
        (true, None) => "\u{a0}".to_string(),
        _ => super::view::size_words(e.size),
    };
    let changed = if e.when > 0 { t!("folders-progress-changed", when = &super::kinds::when_text(e.when)) } else { String::new() };
    let marks = if marks.is_empty() { "\u{a0}".to_string() } else { marks };
    let (pic, icon) = (f.picture_of(&e.path), e.icon());
    let picture = drawing((PICTURE, PICTURE), "", move |canvas, pen| {
        let px = PICTURE * pen.scale;
        match &pic {
            Some(p) => icons::picture(canvas, p, 0.0, 0.0, px, false),
            None => icons::icon(canvas, pen, icon, 0.0, 0.0, px, pen.theme.fg),
        }
    });
    column(vec![
        column_heading(head.to_string()),
        row(vec![
            picture,
            column(vec![note(size).room_for(1), note(changed).room_for(1), label(marks).room_for(1)]).gap(2.0).width(Length::Fill),
        ]).gap(12.0).width(Length::Fill),
    ]).gap(8.0).width(Length::Fill)
}

pub(super) fn question(f: &Files, j: &Job) -> Node<Msg> {
    let id = j.id;
    let Some(a) = &j.asking else { return column(Vec::new()) };
    let folders = a.clash.folders;
    let kind = match (folders, &a.clash.sides) {
        (true, _) => "folder",
        (false, Some((_, there))) => if there.is_dir() { "folder" } else { "file" },
        (false, None) => "unknown",
    };
    let head = t!("folders-progress-taken", kind = kind, name = &a.clash.name);
    let mut parts = vec![form_label(head)];
    if let (Some(coming), Some(there)) = (&a.coming, &a.there) {
        let (c, t) = marks(coming, there);
        parts.push(row(vec![
            side(f, t!("folders-progress-coming"), coming, a.holds.map(|h| h.0), c),
            side(f, t!("folders-progress-there"), there, a.holds.map(|h| h.1), t),
        ]).gap(12.0).width(Length::Fill));
    }
    let answer = |a: Answer| Msg::Answer(id, a);
    let mut quiet = vec![button("media-skip-forward-symbolic", t!("folders-progress-skip")).on_press(answer(Answer::Skip))];
    if !j.fonts {
        quiet.push(button("edit-copy-symbolic", t!("folders-progress-keep-both")).on_press(answer(Answer::Both)));
    }
    let (main, rest) = if folders {
        if a.clash.replace {
            quiet.push(word_button(t!("folders-progress-replace")).spoken(t!("folders-progress-replace-folder-spoken")).on_press(answer(Answer::Replace)));
        }
        (main_action(stand_in::MERGE, t!("folders-progress-merge")).on_press(answer(Answer::Merge)),
         t!("folders-progress-all-folders"))
    } else {
        (main_word(t!("folders-progress-replace")).on_press(answer(Answer::Replace)), t!("folders-progress-all-files"))
    };
    parts.push(foot(quiet, main));
    parts.push(toggle(t!("folders-progress-same-for-rest"), rest, j.all, t!("folders-progress-same-for-rest"), move |on| Msg::AllSame(id, on)));
    card(column(parts).gap(12.0).width(Length::Fill)).floats()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::ui::App;

    fn rows(f: &Files, p: usize) -> Vec<Node<Msg>> {
        if rows_in(f, p) == 0 { Vec::new() } else { super::rows(f, p, space()) }
    }

    #[test]
    fn work_takes_the_place_of_the_line_of_words_and_gives_it_back() {
        fn foot_height(f: &Files) -> f32 {
            super::super::view::pane_foot(f, f.side()).items().len() as f32
        }
        let (dir, mut f) = two_folders("replaces-line");
        let before = foot_words(&f, 0);
        let mut job = Job::sample(1, "Copying 3 items to Documents", 300, 120);
        job.slot = free_slot(&f, 0);
        f.jobs.push(job);
        let during = foot_words(&f, 0);
        assert!(during.iter().any(|w| w.contains("Copying 3 items to Documents")), "{during:?}");
        assert!(during.iter().any(|w| w == "button: Pause") && during.iter().any(|w| w == "button: Stop"), "small Pause and Stop: {during:?}");
        assert_eq!(foot_height(&f), 1.0, "one line, in the place of the line");
        f.jobs.clear();
        assert_eq!(foot_words(&f, 0), before, "the line comes back");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_heavy_copy_says_other_programs_may_feel_slower() {
        let (mb, after) = theme::sheet().work_heavy;
        let mut j = Job::sample(1, "Copying 3 items to Documents", 5_000_000, 0);
        let now = Instant::now();
        assert_eq!(heavy_by(&j, now), None, "nothing is being taken from");
        j.from = vec![std::path::PathBuf::from("/a")];
        assert_eq!(heavy_by(&j, j.began), None, "small and quick");
        let said = heavy_by(&j, j.began + after).unwrap();
        assert!(said.starts_with("Copying uses the disk heavily"), "{said}");
        j.total = (mb as u64 + 1) * 1_000_000;
        assert!(heavy_by(&j, j.began).is_some(), "big at once");
        j.words.doing = "Moving a to b".into();
        assert!(heavy_by(&j, j.began).unwrap().starts_with("Moving"));
        j.paused = true;
        assert_eq!(heavy_by(&j, j.began), None);
    }

    fn asking(name: &str) -> Asking {
        let clash = crate::files::work::Clash { name: name.into(), folders: false, sides: None, replace: true };
        Asking { clash, coming: None, there: None, holds: None }
    }

    #[test]
    fn replace_is_a_word_in_the_question_for_a_name_already_taken() {
        let f = Files::new(vec![std::env::temp_dir()]);
        for (folders, replace) in [(false, true), (true, true), (true, false)] {
            let mut job = Job::sample(1, "Copying 3 items to Documents", 300, 120);
            let mut a = asking("Photos");
            a.clash.folders = folders;
            a.clash.replace = replace;
            job.asking = Some(a);
            let buttons = question(&f, &job).worded();
            let replaces: Vec<_> = buttons.iter().filter(|(tip, _)| tip.starts_with("Replace")).collect();
            assert_eq!(replaces.len(), usize::from(replace), "{folders}, {replace}: {buttons:?}");
            for (tip, words) in replaces {
                assert_eq!(words.as_deref(), Some("Replace"), "{folders}: {tip} is a bare icon");
            }
            let icons: Vec<_> = question(&f, &job).iconed().into_iter().filter(|(tip, _)| tip.starts_with("Replace")).collect();
            assert!(icons.is_empty(), "{folders}: Replace has an icon beside its word: {icons:?}");
        }
    }

    #[test]
    fn the_question_for_a_name_taken_says_which_kind_is_there() {
        let f = Files::new(vec![std::env::temp_dir()]);
        let dir = std::env::temp_dir().join(format!("trindle-files-question-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("ABC")).unwrap();
        std::fs::write(dir.join("XYZ"), b"made up").unwrap();
        std::fs::write(dir.join("other"), b"made up").unwrap();
        for (there, folders, want) in [("ABC", false, "A folder called ABC is already there"), ("XYZ", false, "A file called XYZ is already there"),
                                       ("ABC", true, "A folder called ABC is already there")] {
            let mut job = Job::sample(1, "Copying 1 item to Documents", 300, 120);
            let mut a = asking(there);
            a.clash.folders = folders;
            a.clash.sides = Some((dir.join("other"), dir.join(there)));
            job.asking = Some(a);
            assert!(question(&f, &job).said().iter().any(|w| w.contains(want)), "{there}: {:?}", question(&f, &job).said());
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_question_for_a_font_has_no_keep_both() {
        let f = Files::new(vec![std::env::temp_dir()]);
        for (font, kept) in [(false, true), (true, false)] {
            let mut job = Job::sample(1, "Installing Inter.ttf", 300, 120);
            job.fonts = font;
            job.asking = Some(asking("Inter.ttf"));
            let tips: Vec<String> = question(&f, &job).iconed().into_iter().map(|(tip, _)| tip).collect();
            assert_eq!(tips.iter().any(|t| t.starts_with("Keep both")), kept, "{font}: {tips:?}");
            assert!(tips.iter().any(|t| t.starts_with("Skip")), "{font}: {tips:?}");
        }
    }

    #[test]
    fn newer_and_larger_are_said_of_whichever_is() {
        let e = |size, when, folder| Entry { name: "report.txt".into(), path: "/tmp/report.txt".into(), folder, size, when, kind: None };
        assert_eq!(marks(&e(10, 200, false), &e(5, 100, false)), ("newer, larger".to_string(), String::new()));
        assert_eq!(marks(&e(5, 200, false), &e(10, 100, false)), ("newer".to_string(), "larger".to_string()));
        assert_eq!(marks(&e(5, 100, false), &e(5, 100, false)), (String::new(), String::new()));
        assert_eq!(marks(&e(0, 100, true), &e(0, 300, true)), (String::new(), "newer".to_string()));
        assert_eq!(Asking::holds_words(1), "1 item");
        assert_eq!(Asking::holds_words(10_000), "10,000 items or more");
    }

    fn foot_words(f: &Files, p: usize) -> Vec<String> {
        super::super::view::pane_foot(f, if f.two { p } else { f.side() }).said()
    }

    fn floating(n: &Node<Msg>) -> bool {
        n.floats || n.items().iter().any(floating)
    }

    fn two_folders(name: &str) -> (std::path::PathBuf, Files) {
        use super::super::places::Where;
        let dir = std::env::temp_dir().join(format!("trindle-files-progress-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for sub in ["left", "right/box"] {
            std::fs::create_dir_all(dir.join(sub)).unwrap();
        }
        for file in ["a.txt", "b.txt", "c.txt", "d.txt"] {
            std::fs::write(dir.join("left").join(file), file).unwrap();
        }
        let mut f = Files::new(vec![dir.join("left")]);
        f.update(Msg::Add(Vec::new()));
        f.cols[1].at = Where::Folder(dir.join("right"));
        f.bin = Some(super::super::trash::Bin::at(&dir.join("Trash"), None));
        f.bin.as_ref().unwrap().ready().unwrap();
        f.room = (1200.0, 2000.0);
        f.update(Msg::PanesAre(true));
        f.update(Msg::Changed);
        (dir, f)
    }

    fn pick(f: &mut Files, p: usize, name: &str) {
        let i = f.cols[p].entries.iter().position(|e| e.name == name).unwrap();
        f.side = p;
        f.update(Msg::Choose(p, i));
        f.update(Msg::Pick(p, vec![i]));
    }

    #[test]
    fn work_shows_in_the_foot_by_every_route() {
        use super::super::act::Way;
        let (dir, mut f) = two_folders("routes");
        let ends = |f: &Files| f.ended.last().map(|e| (e.pane, e.words.clone()));
        pick(&mut f, 0, "a.txt");
        f.update(Msg::Copy);
        assert_eq!(ends(&f), Some((1, "Copied a.txt to right.".to_string())));
        assert!(foot_words(&f, 1).contains(&"Copied a.txt to right.".to_string()), "{:?}", foot_words(&f, 1));
        assert!(!foot_words(&f, 0).iter().any(|w| w.starts_with("Copied")), "{:?}", foot_words(&f, 0));
        pick(&mut f, 0, "b.txt");
        f.update(Msg::Move);
        assert_eq!(ends(&f).map(|e| e.0), Some(1));
        assert!(ends(&f).unwrap().1.starts_with("Moved b.txt"), "{:?}", ends(&f));
        pick(&mut f, 0, "c.txt");
        f.update(Msg::Cut);
        f.side = 1;
        f.update(Msg::Paste);
        assert_eq!(ends(&f).map(|e| e.0), Some(1));
        f.side = 0;
        f.drop_into(vec![dir.join("left/d.txt")], super::super::places::Where::Folder(dir.join("right/box")), Way::Copy);
        assert_eq!(ends(&f), Some((1, "Copied d.txt to box.".to_string())));
        pick(&mut f, 0, "d.txt");
        f.update(Msg::Trash);
        assert_eq!(ends(&f).map(|e| e.0), Some(0));
        assert!(foot_words(&f, 0).iter().any(|w| w.starts_with("Put d.txt in the Trash")), "{:?}", foot_words(&f, 0));
        f.update(Msg::Undo);
        assert!(ends(&f).is_some_and(|e| e.1.starts_with("Undid putting d.txt in the Trash")), "{:?}", ends(&f));
        f.update(Msg::PanesAre(false));
        assert!(foot_words(&f, 0).contains(&"Copied a.txt to right.".to_string()), "{:?}", foot_words(&f, 0));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn work_done_in_a_moment_still_shows_for_the_least_time() {
        let (dir, mut f) = two_folders("moment");
        pick(&mut f, 0, "a.txt");
        f.update(Msg::Copy);
        assert!(f.jobs.is_empty() && dir.join("right/a.txt").exists());
        let id = f.ended[0].id;
        for msg in [Msg::Changed, Msg::Choose(0, 0), Msg::Escape, Msg::Hidden, Msg::Hidden] {
            f.update(msg);
            assert!(foot_words(&f, 1).contains(&"Copied a.txt to right.".to_string()), "{:?}", foot_words(&f, 1));
        }
        assert_eq!(rows_in(&f, 1), 1);
        f.update(Msg::WorkGone(id + 1));
        assert_eq!(rows_in(&f, 1), 1);
        f.update(Msg::WorkGone(id));
        assert_eq!(rows_in(&f, 1), 0);
        assert_eq!(done_for(), std::time::Duration::from_secs(3));
        let code = crate::common::source::code_all(crate::files::act::SOURCE);
        let start = code.split_once("    pub(in crate::files) fn start(").unwrap().1.split_once("\n    }\n").unwrap().0;
        assert!(!start.contains("sleep"), "work waits before it is shown: {start}");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_end_is_said_in_the_foot_then_the_row_goes() {
        let (dir, mut f) = two_folders("end");
        pick(&mut f, 0, "a.txt");
        f.update(Msg::Copy);
        let said = foot_words(&f, 1);
        assert_eq!(said.iter().filter(|w| w.contains("Copied a.txt")).count(), 1, "said twice or not at all: {said:?}");
        assert!(!said.iter().any(|w| w == "button: Pause" || w == "button: Stop"), "{said:?}");
        let shown = rows(&f, 1);
        assert_eq!(shown.len(), 1);
        assert!(shown[0].iconed().is_empty() && shown[0].worded().is_empty(), "a row that has ended has no buttons");
        f.update(Msg::WorkGone(f.ended[0].id));
        assert!(rows(&f, 1).is_empty());
        assert!(f.said.is_empty(), "done is said for a moment only: {}", f.said);
        pick(&mut f, 0, "b.txt");
        std::fs::remove_file(dir.join("left/b.txt")).unwrap();
        f.update(Msg::Copy);
        let e = f.ended.last().unwrap().clone();
        assert!(e.bad && e.stays && e.words.contains("could not be copied"), "{e:?}");
        f.update(Msg::WorkGone(e.id));
        assert!(f.bad && f.said == e.words, "a failure stays on the line to be read: {}", f.said);
        let mut job = Job::sample(9, "Copying 3 items to right", 300, 120);
        job.pane = 1;
        f.jobs.push(job);
        f.update(Msg::Work(9, crate::files::work::News::Over(crate::files::work::Over { stopped: true, ..Default::default() })));
        let e = f.ended.last().unwrap().clone();
        assert!(e.stays && e.words == "Stopped part way.", "{e:?}");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn nothing_of_the_work_floats_over_the_window() {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.room = (1200.0, 760.0);
        for two in [false, true] {
            f.update(Msg::PanesAre(two));
            f.jobs.push(Job::sample(1, "Copying 3 items to Documents", 300, 120));
            f.ended.push(Ended { id: 2, slot: 1, pane: 0, words: "Moved notes.txt to Videos.".into(), bad: false, help: None, stays: false });
            assert!(!floating(&f.view()), "two panes: {two}");
            assert!(layers(&f).is_empty());
            let said = foot_words(&f, 0);
            assert!(said.iter().any(|w| w.starts_with("Copying 3 items to Documents")) && said.contains(&"Moved notes.txt to Videos.".to_string()), "{said:?}");
            f.jobs[0].asking = Some(asking("report.txt"));
            assert_eq!(layers(&f).len(), 1);
            f.jobs.clear();
            f.ended.clear();
        }
    }

    #[test]
    fn a_failed_row_has_a_more_that_opens_its_page() {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.room = (1200.0, 760.0);
        let more = format!("button: {}", crate::toolkit::t!("common-more"));
        for (bad, help, has) in [(true, Some(crate::files::help::WORK_NOT_DONE), true), (true, None, false), (false, None, false)] {
            f.ended.clear();
            f.ended.push(Ended { id: 2, slot: 0, pane: 0, words: "Two items could not be copied: a, b.".into(), bad, help, stays: bad });
            assert_eq!(foot_words(&f, 0).contains(&more), has, "{bad} {help:?}");
        }
        f.update(Msg::Help(crate::files::help::WORK_NOT_DONE));
        assert_eq!(f.helped, [crate::files::help::WORK_NOT_DONE]);
    }

    #[test]
    fn work_keeps_its_place_and_a_press_brings_no_other_row_under_the_pointer() {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.update(Msg::PanesAre(true));
        assert!(rows(&f, 0).is_empty());
        for id in 1..=3 {
            let slot = free_slot(&f, 0);
            let mut job = Job::sample(id, "Copying 3 items to Documents", 300, 120);
            job.slot = slot;
            f.jobs.push(job);
        }
        assert_eq!(f.jobs.iter().map(|j| j.slot).collect::<Vec<_>>(), [0, 1, 2]);
        assert_eq!(rows(&f, 0).len(), 3);
        assert_eq!(free_slot(&f, 1), 0);
        assert!(rows(&f, 1).is_empty());
        let middle = f.jobs.remove(1);
        f.ended.push(Ended { id: middle.id, slot: middle.slot, pane: 0, words: "Copied 3 items to Documents.".into(), bad: false, help: None, stays: false });
        assert_eq!(free_slot(&f, 0), 3);
        assert_eq!(rows(&f, 0).len(), 3);
        f.update(Msg::WorkGone(middle.id));
        assert_eq!(rows(&f, 0).len(), 3);
        assert_eq!(free_slot(&f, 0), 1);
        f.jobs.pop();
        assert_eq!(rows(&f, 0).len(), 1);
        f.jobs[0].asking = Some(asking("report.txt"));
        assert_eq!(layers(&f).len(), 1);
        let _ = f.view();
    }

    #[test]
    fn speed_and_time_left_are_said_plainly_and_steadily() {
        let start = Instant::now();
        let at = |secs: f64| start + std::time::Duration::from_secs_f64(secs);
        const MB: u64 = 1_000_000;
        let total = 3_000 * MB;
        let mut p = Pace::new(Unit::Bytes);
        assert_eq!(p.words(), "working out how long\u{2026}");
        p.note(at(0.0), 0, total);
        p.note(at(0.5), 12 * MB, total);
        p.note(at(1.0), 24 * MB, total);
        assert_eq!(p.words(), "working out how long\u{2026}", "one measure is not enough");
        p.note(at(2.0), 48 * MB, total);
        assert_eq!(p.words(), "24 MB a second, about 2 minutes left");
        p.note(at(2.1), 60 * MB, total);
        assert_eq!(p.words(), "24 MB a second, about 2 minutes left");
        p.note(at(3.0), 88 * MB, total);
        let said = p.words();
        assert!(said.starts_with("29 MB a second") && said.ends_with("about 2 minutes left"), "{said}");
        let mut done = 88 * MB;
        for s in 4..40 {
            done += 200 * MB;
            p.note(at(s as f64), done.min(total), total);
        }
        assert!(p.words().contains("a few seconds left") || p.words().contains("seconds left"), "{}", p.words());
        let mut p = Pace::new(Unit::Bytes);
        p.note(at(0.0), 0, total);
        p.note(at(1.0), 24 * MB, total);
        p.note(at(2.0), 48 * MB, total);
        p.rest();
        p.note(at(600.0), 48 * MB, total);
        p.note(at(601.0), 72 * MB, total);
        assert_eq!(p.words(), "24 MB a second, about 2 minutes left");
        p.files(1_240, 9_000);
        assert_eq!(p.words(), "1,240 of 9,000 files, 24 MB a second, about 2 minutes left");
        p.files(3, 12);
        assert_eq!(p.words(), "24 MB a second, about 2 minutes left");
        let mut p = Pace::new(Unit::Items);
        p.note(at(0.0), 0, 3_000);
        p.note(at(1.0), 20, 3_000);
        p.note(at(2.0), 40, 3_000);
        assert_eq!(p.words(), "20 items a second, about 2 minutes left");
        p.note(at(3.0), 40, 3_000);
        for s in 4..60 {
            p.note(at(s as f64), 40, 3_000);
        }
        assert_eq!(p.words(), "working out how long\u{2026}");
    }

    #[test]
    fn the_words_for_speed_time_and_counts() {
        assert_eq!(speed_words(800.0), "800 bytes");
        assert_eq!(speed_words(4_200_000.0), "4.2 MB");
        assert_eq!(speed_words(24_400_000.0), "24 MB");
        assert_eq!(speed_words(999_900.0), "1.0 MB");
    }

    #[test]
    fn counts_and_speeds_follow_the_languages_marks() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let made_up = "kit-number-decimal = ,\nkit-number-group = _\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(speed_words(4_200_000.0), "4,2 MB");
            assert_eq!(speed_words(24_400_000.0), "24 MB");
            assert_eq!(grouped(1_234_567), "1_234_567");
        });
        assert_eq!(left_words(3.0), "a few seconds left");
        assert_eq!(left_words(24.0), "about 20 seconds left");
        assert_eq!(left_words(70.0), "about a minute left");
        assert_eq!(left_words(130.0), "about 2 minutes left");
        assert_eq!(left_words(3.0 * 3600.0), "about 3 hours left");
        assert_eq!(grouped(0), "0");
        assert_eq!(grouped(999), "999");
        assert_eq!(grouped(9_000), "9,000");
        assert_eq!(grouped(1_234_567), "1,234,567");
    }

    #[test]
    fn a_card_says_paused_and_waiting_in_place_of_a_speed() {
        let mut job = Job::sample(1, "Copying 3 items to Documents", 300, 120);
        job.paused = true;
        assert_eq!(how(&job), "paused");
        job.paused = false;
        assert_eq!(how(&job), "working out how long\u{2026}");
        job.asking = Some(asking("report.txt"));
        assert_eq!(how(&job), "waiting for an answer");
    }

    #[test]
    fn more_work_than_a_foot_holds_is_counted_on_the_top_row() {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.room = (1200.0, 400.0);
        let fit = fit(f.room);
        for id in 0..fit as u64 + 3 {
            let mut job = Job::sample(id + 1, "Copying", 10, 1);
            job.slot = id as usize;
            f.jobs.push(job);
        }
        let rows = rows(&f, 0);
        assert_eq!(rows.len(), fit);
        assert_eq!(rows[0].said(), ["4 more pieces of work are under way"]);
    }
}
