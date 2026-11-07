// Earlier versions: Folders' side of the copies of the home folders that Trindle keeps each day (common/earlier.rs reads them).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::toolkit::t;

use super::app::{Files, Msg};
use super::dialogs::{head, Dialog};
use super::menu::{Act, Line};
use super::places::Where;
use super::view::size_words;
use super::work::{self, Extra, Kind, Plan, Step};
use crate::common::clock::Local;
use crate::common::earlier::{self as copies, PutBack, Store, Version};
use crate::toolkit::theme;
use crate::toolkit::tiles::loading_state;
use crate::toolkit::widget::{button, card, column, label, note, row, scroll, space, Length, Node, Tone};

pub fn shut() -> &'static str {
    t!("folders-earlier-shut")
}
const LOOKS: [Act; 15] = [Act::Open, Act::OpenTab, Act::OpenWindow, Act::OpenWith, Act::Preview, Act::CopyPath, Act::Copy, Act::Properties,
                          Act::Terminal, Act::SelectAll, Act::SelectNone, Act::Invert, Act::CompareFiles, Act::SendTo, Act::InstallFont];
const WIDTH: f32 = 520.0;
const ROWS: f32 = 5.0;

pub struct Earlier {
    pub(super) store: Store,
    any: bool,
    days: HashMap<u32, i64>,
    next: u64,
}

impl Default for Earlier {
    fn default() -> Earlier {
        #[cfg(test)]
        let store = Store::at(std::env::temp_dir().join("trindle-no-such-home"), std::env::temp_dir().join("trindle-no-such-copies"));
        #[cfg(not(test))]
        let store = Store::here();
        Earlier { store, any: false, days: HashMap::new(), next: 0 }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum What {
    Versions(PathBuf),
    Deleted(PathBuf),
}

#[derive(Clone, Debug)]
pub struct Row {
    name: String,
    version: Version,
    said: Option<(String, bool)>,
    busy: bool,
    done: bool,
}

pub struct Card {
    pub what: What,
    pane: usize,
    id: u64,
    rows: Option<Vec<Row>>,
}

#[derive(Clone)]
pub enum EarlierMsg {
    Any(bool),
    Listed(u64, Vec<Row>),
    Open(usize),
    PutBack(usize),
    PutBackDone(u64, usize, Result<PutBack, String>),
    Compare(usize),
    Compared(u64, usize, Result<bool, String>),
}

pub fn day_words(at: Local, now: Local) -> String {
    let (day, time, date) = day_parts(at, now);
    t!("folders-earlier-day-words", day = day, time = &time, date = &date)
}

fn day_parts(at: Local, now: Local) -> (&'static str, String, String) {
    let time = at.line(trindle_toolkit::lang::format::Pattern::Clock);
    match now.day_number() - at.day_number() {
        0 => ("today", time, String::new()),
        1 => ("yesterday", time, String::new()),
        _ if at.year() == now.year() => ("other", time, at.line(trindle_toolkit::lang::format::Pattern::DayLong)),
        _ => ("other", time, at.line(trindle_toolkit::lang::format::Pattern::DayLongYear)),
    }
}

fn day_line(secs: i64) -> String {
    let (day, time, date) = day_parts(Local::at(secs), Local::now());
    t!("folders-earlier-day-line", day = day, time = &time, date = &date)
}

impl Files {
    pub(super) fn earlier_look(&mut self) {
        let store = self.earlier.store.clone();
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name("folders-earlier".into()).spawn(move || {
                    post.send(Msg::Earlier(EarlierMsg::Any(store.any())));
                });
            }
            None => self.earlier.any = store.any(),
        }
    }

    pub(super) fn earlier_holds(&self, path: &Path) -> bool {
        self.earlier.store.holds(path).is_some()
    }

    pub(super) fn earlier_at(&self, at: &Where) -> bool {
        at.folder().is_some_and(|f| self.earlier_holds(f))
    }

    fn earlier_offers(&self, path: &Path) -> bool {
        self.earlier.any && path.starts_with(&self.home) && !self.earlier_holds(path) && self.earlier.store.covers(path)
    }

    pub(super) fn earlier_lines(&self, at: &Where, items: &[PathBuf], lines: Vec<Line>) -> Vec<Line> {
        let Where::Folder(here) = at else { return lines };
        if self.earlier_holds(here) {
            return looks_only(lines);
        }
        match items {
            [one] if self.earlier_offers(one) => with_row(lines, true),
            [] if self.earlier_offers(&here.join("x")) => with_row(lines, false),
            _ => lines,
        }
    }

    pub(super) fn earlier_refuses(&mut self, act: Act, p: usize, items: &[PathBuf]) -> bool {
        let inside = self.earlier_at(&self.cols[p].at) || items.iter().any(|i| self.earlier_holds(i));
        if !inside || LOOKS.contains(&act) {
            return false;
        }
        self.say(shut(), false);
        true
    }

    pub(super) fn earlier_pane(&self, other: bool) -> bool {
        let p = if other { 1 - self.side() } else { self.side() };
        (!other || self.two) && self.earlier_at(&self.cols[p].at)
    }

    pub(super) fn earlier_blocks(&self, plan: &Plan) -> bool {
        let into = !plan.to.as_os_str().is_empty() && self.earlier_holds(&plan.to);
        let from = || plan.items.iter().any(|i| self.earlier_holds(i));
        match &plan.kind {
            Kind::Copy | Kind::Other(Extra::Pack(..) | Extra::Unpack(_) | Extra::TakeOut(_)) => into,
            Kind::Move => into || from(),
            Kind::Trash | Kind::Delete | Kind::Other(Extra::Wipe(_) | Extra::Picture(_)) => from(),
            Kind::Restore | Kind::Undo(_) => false,
        }
    }

    pub(super) fn earlier_note(&self, at: &Where) -> Option<String> {
        let number = self.earlier.store.holds(at.folder()?)?;
        Some(match self.earlier.days.get(&number) {
            Some(taken) => t!("folders-earlier-note-dated", when = &day_words(Local::at(*taken), Local::now())),
            None => t!("folders-earlier-note").to_string(),
        })
    }

    pub fn tab_name(&self, at: &Where) -> String {
        let name = at.name(&self.home);
        if self.earlier_at(at) { t!("folders-earlier-tab-name", name = &name) } else { name }
    }

    pub(super) fn earlier_open(&mut self, p: usize, what: What) {
        self.earlier.next += 1;
        let id = self.earlier.next;
        self.dialog = Some(Dialog::Earlier(Box::new(Card { what: what.clone(), pane: p, id, rows: None })));
        let store = self.earlier.store.clone();
        let read = move || -> Vec<Row> {
            let row = |name: String, version: Version| Row { name, version, said: None, busy: false, done: false };
            match &what {
                What::Versions(of) => store.changes(of).into_iter().map(|v| row(work::name_of(of), v)).collect(),
                What::Deleted(from) => store.gone_from(from).into_iter().map(|g| row(g.name, g.last)).collect(),
            }
        };
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name("folders-earlier".into()).spawn(move || {
                    post.send(Msg::Earlier(EarlierMsg::Listed(id, read())));
                });
            }
            None => self.earlier_msg(EarlierMsg::Listed(id, read())),
        }
    }

    fn earlier_card(&mut self, id: Option<u64>) -> Option<&mut Card> {
        match &mut self.dialog {
            Some(Dialog::Earlier(c)) if id.is_none_or(|id| c.id == id) => Some(c.as_mut()),
            _ => None,
        }
    }

    pub(super) fn earlier_msg(&mut self, m: EarlierMsg) {
        match m {
            EarlierMsg::Any(any) => self.earlier.any = any,
            EarlierMsg::Listed(id, rows) => {
                if let Some(c) = self.earlier_card(Some(id)) {
                    c.rows = Some(rows);
                }
            }
            EarlierMsg::Open(i) => self.earlier_open_row(i),
            EarlierMsg::PutBack(i) => self.earlier_put_back(i),
            EarlierMsg::PutBackDone(id, i, got) => self.earlier_put_back_done(id, i, got),
            EarlierMsg::Compare(i) => self.earlier_compare(i),
            EarlierMsg::Compared(id, i, got) => match got {
                Ok(true) => self.earlier_said(Some(id), i, t!("folders-earlier-same").into(), false),
                Ok(false) => self.earlier_said(Some(id), i, t!("folders-earlier-differs").into(), false),
                Err(why) => self.earlier_said(Some(id), i, t!("folders-earlier-compare-failed", why = &why), true),
            },
        }
    }

    fn earlier_open_row(&mut self, i: usize) {
        let Some(c) = self.earlier_card(None) else { return };
        let (p, Some(r)) = (c.pane, c.rows.as_ref().and_then(|rows| rows.get(i)).cloned()) else { return };
        match r.version.kind {
            copies::Kind::Folder => {
                self.earlier.days.insert(r.version.number, r.version.taken);
                self.dialog = None;
                self.new_tab(p, Some(Where::Folder(r.version.path)));
            }
            copies::Kind::File => {
                let _ = self.open_file(&r.version.path);
            }
            copies::Kind::Link => {}
        }
    }

    fn earlier_put_back(&mut self, i: usize) {
        let Some(c) = self.earlier_card(None) else { return };
        let Some(r) = c.rows.as_mut().and_then(|rows| rows.get_mut(i)).filter(|r| !r.busy && !r.done) else { return };
        r.busy = true;
        let (id, version) = (c.id, r.version.clone());
        let to = match &c.what {
            What::Versions(of) => of.clone(),
            What::Deleted(from) => from.join(&r.name),
        };
        let store = self.earlier.store.clone();
        let put = move || store.put_back(&version, &to);
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name("folders-earlier".into()).spawn(move || {
                    post.send(Msg::Earlier(EarlierMsg::PutBackDone(id, i, put())));
                });
            }
            None => self.earlier_msg(EarlierMsg::PutBackDone(id, i, put())),
        }
    }

    fn earlier_put_back_done(&mut self, id: u64, i: usize, got: Result<PutBack, String>) {
        let name = match &got {
            Ok(done) => work::name_of(&done.put),
            Err(_) => String::new(),
        };
        match got {
            Ok(done) => {
                let mut steps = Vec::new();
                if let Some(kept) = &done.kept {
                    steps.push(Step::Back { now: kept.clone(), was: done.put.clone() });
                }
                steps.push(Step::Remove(done.put.clone()));
                self.remember(t!("folders-earlier-undo-put-back", name = &name), steps);
                let said = match &done.kept {
                    Some(_) => t!("folders-earlier-put-back-kept").to_string(),
                    None => t!("folders-earlier-put-back").to_string(),
                };
                self.earlier_said(Some(id), i, said, false);
                if let Some(r) = self.earlier_card(Some(id)).and_then(|c| c.rows.as_mut()).and_then(|rows| rows.get_mut(i)) {
                    r.done = true;
                }
                self.reload_all();
            }
            Err(why) => self.earlier_said(Some(id), i, why, true),
        }
    }

    fn earlier_compare(&mut self, i: usize) {
        let Some(c) = self.earlier_card(None) else { return };
        let What::Versions(now) = c.what.clone() else { return };
        let Some(r) = c.rows.as_mut().and_then(|rows| rows.get_mut(i)).filter(|r| !r.busy) else { return };
        r.busy = true;
        let (id, then) = (c.id, r.version.path.clone());
        let read = move || copies::same_content(&then, &now).map_err(|e| super::places::unread_words(&e));
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name("folders-earlier".into()).spawn(move || {
                    post.send(Msg::Earlier(EarlierMsg::Compared(id, i, read())));
                });
            }
            None => self.earlier_msg(EarlierMsg::Compared(id, i, read())),
        }
    }

    fn earlier_said(&mut self, id: Option<u64>, i: usize, words: String, bad: bool) {
        if let Some(r) = self.earlier_card(id).and_then(|c| c.rows.as_mut()).and_then(|rows| rows.get_mut(i)) {
            r.said = Some((words, bad));
            r.busy = false;
        }
    }
}

pub(super) fn with_row(mut lines: Vec<Line>, one: bool) -> Vec<Line> {
    let (act, words) = if one { (Act::Earlier, t!("folders-earlier-menu-versions")) } else { (Act::DeletedHere, t!("folders-earlier-menu-deleted")) };
    let added = Line { act, words, icon: "document-open-recent-symbolic", keys: "", rule: true, sub: Vec::new() };
    match lines.iter().position(|l| l.act == Act::Properties) {
        Some(i) => {
            lines[i].rule = false;
            lines.insert(i, added);
        }
        None => lines.push(added),
    }
    lines
}

pub(super) fn looks_only(lines: Vec<Line>) -> Vec<Line> {
    let mut out: Vec<Line> = Vec::new();
    let mut rule = false;
    for l in lines {
        if !LOOKS.contains(&l.act) {
            rule |= l.rule;
            continue;
        }
        out.push(Line { rule: (l.rule || rule) && !out.is_empty(), ..l });
        rule = false;
    }
    out
}

pub fn view(c: &Card) -> Node<Msg> {
    let tall = ROWS * row_tall();
    let (title, none, foot) = match &c.what {
        What::Versions(of) => (t!("folders-earlier-versions-title", name = &work::name_of(of)),
                               t!("folders-earlier-versions-none").to_string(),
                               t!("folders-earlier-versions-foot").to_string()),
        What::Deleted(from) => (t!("folders-earlier-deleted-title", name = &work::name_of(from)),
                                t!("folders-earlier-deleted-none").to_string(),
                                t!("folders-earlier-deleted-foot").to_string()),
    };
    let body = match &c.rows {
        None => loading_state(tall),
        Some(rows) if rows.is_empty() => column(vec![note(none).width(Length::Fill)]).width(Length::Fill).height(Length::Fixed(tall)),
        Some(rows) => {
            let lines: Vec<Node<Msg>> = rows.iter().enumerate().map(|(i, r)| one(c, i, r)).collect();
            scroll(format!("earlier:{}", c.id), column(lines).gap(0.0).width(Length::Fill)).width(Length::Fill).height(Length::Fixed(tall))
        }
    };
    card(column(vec![
        head("document-open-recent-symbolic", title),
        body,
        note(foot).room_for(2).width(Length::Fill),
    ]).gap(12.0).width(Length::Fill)).width(Length::Fixed(WIDTH)).on_escape(Msg::Ask(super::dialogs::AskMsg::Close))
}

fn row_tall() -> f32 {
    theme::sheet().rich_row + 8.0
}

fn one(c: &Card, i: usize, r: &Row) -> Node<Msg> {
    let v = &r.version;
    let folder = v.kind == copies::Kind::Folder;
    let (first, second) = match &c.what {
        What::Versions(_) if folder => (day_line(v.taken), t!("folders-earlier-folder-then").to_string()),
        What::Versions(_) => (day_line(v.modified), t!("folders-earlier-version-line", size = &size_words(v.size), date = &Local::at(v.taken).line(trindle_toolkit::lang::format::Pattern::DayLong))),
        What::Deleted(_) if folder => (r.name.clone(), t!("folders-earlier-deleted-folder-line", when = &day_words(Local::at(v.taken), Local::now()))),
        What::Deleted(_) => (r.name.clone(), t!("folders-earlier-deleted-line", size = &size_words(v.size), when = &day_words(Local::at(v.taken), Local::now()))),
    };
    let under = match &r.said {
        Some((words, true)) => note(words.clone()).tone(Tone::Problem),
        Some((words, false)) => note(words.clone()),
        None if r.busy => note(t!("folders-earlier-working")),
        None => note(second),
    };
    let to = |m: EarlierMsg| Msg::Earlier(m);
    let (open_icon, open_tip) = if folder { ("tab-new-symbolic", t!("folders-earlier-open-folder-tip")) } else { ("document-open-symbolic", t!("folders-earlier-open-tip")) };
    let mut parts = vec![
        column(vec![label(first).cut().width(Length::Fill), under.cut().width(Length::Fill)]).gap(theme::sheet().rich_gap).width(Length::Fill),
        button(open_icon, open_tip).on_press(to(EarlierMsg::Open(i))).off(v.kind == copies::Kind::Link),
        button("document-revert-symbolic", t!("folders-earlier-put-back-tip")).on_press(to(EarlierMsg::PutBack(i))).off(r.done).off_because(crate::common::ui::wait_while(r.busy)),
    ];
    if matches!(c.what, What::Versions(_)) {
        parts.push(button(super::icons::stand_in::COMPARE, t!("folders-earlier-compare-tip")).on_press(to(EarlierMsg::Compare(i))).off_because(crate::common::ui::wait_while(r.busy)));
    } else {
        parts.push(space().width(Length::Fixed(0.0)));
    }
    let line = column(vec![row(parts).gap(8.0).width(Length::Fill).centred()]).padding(4.0).width(Length::Fill).height(Length::Fixed(row_tall()));
    if i % 2 == 1 { line.banded() } else { card(line).snug().flat() }
}

#[cfg(test)]
type RowSaid = (String, u32, Option<(String, bool)>);

#[cfg(test)]
impl Files {
    pub(super) fn earlier_use(&mut self, home: &Path, store: Store) {
        self.home = home.to_path_buf();
        self.earlier.store = store;
        self.earlier_look();
    }

    pub(super) fn earlier_rows(&self) -> Option<Vec<RowSaid>> {
        let Some(Dialog::Earlier(c)) = &self.dialog else { return None };
        c.rows.as_ref().map(|rows| rows.iter().map(|r| (r.name.clone(), r.version.number, r.said.clone())).collect())
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::common::home_snapshots::tests::{add_snapshot, made_up_snapshots};
    use crate::common::home_snapshots::DESCRIPTION;
    use crate::files::menu::MenuMsg;
    use crate::files::trash::Bin;
    use crate::toolkit::ui::App;
    use std::time::{Duration, UNIX_EPOCH};

    pub struct World {
        pub root: PathBuf,
        pub store: Store,
    }

    impl World {
        pub fn new(name: &str) -> World {
            let root = std::fs::canonicalize(made_up_snapshots(&format!("folders-{name}"))).unwrap();
            std::fs::create_dir_all(root.join("home/john")).unwrap();
            std::fs::create_dir_all(root.join("snaps")).unwrap();
            World { store: Store::at(root.join("home"), root.join("snaps")), root }
        }

        pub fn live(&self, below: &str) -> PathBuf {
            self.root.join("home/john").join(below)
        }

        pub fn copy(&self, number: u32, day: u32, files: &[(&str, &str, u64)]) -> PathBuf {
            let tree = add_snapshot(&self.root.join("snaps"), number, &format!("2026-10-{day:02} 03:00:00"), DESCRIPTION);
            for (below, words, changed) in files {
                write(&tree.join("john").join(below), words, *changed);
            }
            tree.join("john")
        }

        pub fn files(&self, below: &str) -> Files {
            let mut f = Files::new(vec![self.live(below)]);
            let bin = Bin::at(&self.root.join("Trash"), None);
            bin.ready().unwrap();
            f.bin = Some(bin);
            f.earlier_use(&self.live(""), self.store.clone());
            f.update(Msg::PanesAre(false));
            f
        }

        pub fn copies(&self) -> Vec<(PathBuf, Vec<u8>)> {
            fn walk(dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
                let mut names: Vec<PathBuf> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
                names.sort();
                for p in names {
                    if p.is_dir() {
                        out.push((p.clone(), Vec::new()));
                        walk(&p, out);
                    } else {
                        out.push((p.clone(), std::fs::read(&p).unwrap()));
                    }
                }
            }
            let mut out = Vec::new();
            walk(&self.root.join("snaps"), &mut out);
            out
        }
    }

    impl Drop for World {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    pub fn write(path: &Path, words: &str, changed: u64) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, words).unwrap();
        std::fs::File::options().write(true).open(path).unwrap().set_modified(UNIX_EPOCH + Duration::from_secs(changed)).unwrap();
    }

    fn pick(f: &mut Files, names: &[&str]) {
        let picked: Vec<usize> = names.iter().map(|n| f.cols[0].entries.iter().position(|e| e.name == *n).unwrap_or_else(|| panic!("no {n}"))).collect();
        f.update(Msg::Choose(0, picked[0]));
        f.update(Msg::Pick(0, picked));
    }

    fn offered(f: &mut Files, names: &[&str]) -> Vec<Act> {
        if names.is_empty() {
            f.update(Msg::Unpick);
            f.cols[0].chosen = None;
        } else {
            pick(f, names);
        }
        f.update(Msg::Menu(MenuMsg::Open));
        let acts = f.menu.as_ref().map(|m| crate::files::menu::every(&m.lines).iter().map(|l| l.act).collect()).unwrap_or_default();
        f.menu = None;
        acts
    }

    #[test]
    fn a_moment_is_said_in_plain_words() {
        let at = |day: i64, hour: i64, min: i64| Local::utc(1_790_000_000 - 1_790_000_000 % 86_400 + day * 86_400 + hour * 3_600 + min * 60);
        let now = at(0, 16, 30);
        assert_eq!(day_words(at(0, 14, 5), now), "today at 14:05");
        assert_eq!(day_words(at(-1, 14, 5), now), "yesterday at 14:05");
        assert_eq!(day_words(at(-1, 23, 59), at(0, 0, 1)), "yesterday at 23:59");
        let earlier = day_words(at(-3, 9, 0), now);
        assert!(earlier.ends_with(" at 09:00") && !earlier.contains("day") && !earlier.contains("20"), "{earlier}");
        let long_ago = day_words(at(-800, 9, 0), now);
        assert!(long_ago.contains(" 20") && long_ago.ends_with(" at 09:00"), "{long_ago}");
    }

    #[test]
    fn a_moment_is_dated_the_way_the_language_dates_it() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let at = |day: i64, hour: i64, min: i64| Local::utc(1_790_000_000 - 1_790_000_000 % 86_400 + day * 86_400 + hour * 3_600 + min * 60);
        let made_up = "kit-date-month-sep = Zepto\nkit-date-pattern-day-long = %B %-d\nkit-date-pattern-clock = %H.%M\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(day_words(at(0, 14, 5), at(0, 16, 30)), "today at 14.05");
            assert_eq!(day_words(at(-3, 9, 0), at(0, 16, 30)), "Zepto 18 at 09.00");
        });
    }

    #[test]
    fn the_menu_offers_earlier_versions_only_when_there_are_copies_to_look_in() {
        let w = World::new("menu");
        write(&w.live("Documents/report.txt"), "third draft", 3000);
        write(&w.live("Documents/notes.txt"), "notes", 3000);
        std::fs::create_dir_all(w.live("Documents/Letters")).unwrap();
        let mut f = w.files("Documents");
        assert!(!offered(&mut f, &["report.txt"]).contains(&Act::Earlier));
        assert!(!offered(&mut f, &[]).contains(&Act::DeletedHere));
        w.copy(1, 1, &[("Documents/report.txt", "first", 1000)]);
        f.earlier_look();
        let acts = offered(&mut f, &["report.txt"]);
        assert_eq!(acts.iter().filter(|a| **a == Act::Earlier).count(), 1);
        assert_eq!(acts.last(), Some(&Act::Properties));
        pick(&mut f, &["report.txt"]);
        f.update(Msg::Menu(MenuMsg::Open));
        let lines = f.menu.take().expect("a menu").lines;
        let (row, inner) = crate::files::menu::find(&lines, Act::Earlier).expect("Earlier versions is offered");
        assert!(inner.is_some() && lines[row].act == Act::Sub(crate::files::menu::Kind::Tools), "{:?}", lines[row].act);
        let flat = with_row(crate::files::menu::lines_for(&Where::Folder(w.live("Documents")), &[crate::files::menu::Sel::default()], false), true);
        assert_eq!(flat[flat.len() - 2..].iter().map(|l| (l.act, l.rule)).collect::<Vec<_>>(), [(Act::Earlier, true), (Act::Properties, false)]);
        assert!(offered(&mut f, &["Letters"]).contains(&Act::Earlier));
        assert!(!offered(&mut f, &["report.txt", "notes.txt"]).contains(&Act::Earlier));
        let room = offered(&mut f, &[]);
        assert!(room.contains(&Act::DeletedHere) && !room.contains(&Act::Earlier));
        let outside = w.root.join("elsewhere");
        write(&outside.join("a.txt"), "a", 1);
        f.go(0, Where::Folder(outside));
        assert!(!offered(&mut f, &["a.txt"]).contains(&Act::Earlier));
        assert!(!offered(&mut f, &[]).contains(&Act::DeletedHere));
    }

    #[test]
    fn a_version_is_put_back_with_the_present_one_kept_beside_it_and_undo_takes_it_back() {
        let w = World::new("put-back");
        write(&w.live("Documents/report.txt"), "third draft", 3000);
        w.copy(1, 1, &[("Documents/report.txt", "first", 1000)]);
        w.copy(2, 2, &[("Documents/report.txt", "first", 1000)]);
        w.copy(3, 3, &[("Documents/report.txt", "second draft", 2000)]);
        w.copy(4, 4, &[("Documents/report.txt", "third draft", 3000)]);
        let before = w.copies();
        let mut f = w.files("Documents");
        pick(&mut f, &["report.txt"]);
        f.update(Msg::Menu(MenuMsg::Open));
        assert!(f.menu_pick(Act::Earlier), "the menu has Earlier versions");
        assert_eq!(f.earlier_rows().unwrap().iter().map(|r| r.1).collect::<Vec<_>>(), [3, 2]);
        f.update(Msg::Earlier(EarlierMsg::Compare(0)));
        assert_eq!(f.earlier_rows().unwrap()[0].2, Some(("It differs from the one there now.".to_string(), false)));
        f.update(Msg::Earlier(EarlierMsg::PutBack(0)));
        let read = |below: &str| std::fs::read_to_string(w.live(below)).unwrap_or_default();
        assert_eq!(read("Documents/report.txt"), "second draft");
        assert_eq!(read("Documents/report (before putting back).txt"), "third draft");
        let said = f.earlier_rows().unwrap()[0].2.clone().unwrap();
        assert!(!said.1 && said.0.contains("kept beside it"), "{said:?}");
        assert!(f.cols[0].entries.iter().any(|e| e.name == "report (before putting back).txt"), "the list shows it at once");
        f.update(Msg::Earlier(EarlierMsg::PutBack(0)));
        assert!(!w.live("Documents/report (before putting back) 2.txt").exists());
        assert_eq!(f.undo.last().map(|u| u.0.as_str()), Some("Undo putting back report.txt"));
        f.update(Msg::Undo);
        assert_eq!(read("Documents/report.txt"), "third draft");
        assert!(!w.live("Documents/report (before putting back).txt").exists());
        f.update(Msg::Redo);
        assert_eq!(read("Documents/report.txt"), "second draft");
        assert_eq!(read("Documents/report (before putting back).txt"), "third draft");
        assert_eq!(w.copies(), before);
    }

    #[test]
    fn things_deleted_from_a_folder_are_listed_and_put_back() {
        let w = World::new("deleted");
        write(&w.live("Documents/kept.txt"), "k", 1);
        w.copy(1, 1, &[("Documents/letter.odt", "dear sir", 1000), ("Documents/kept.txt", "k", 1)]);
        w.copy(2, 2, &[("Documents/letter.odt", "dear sir or madam", 2000), ("Documents/kept.txt", "k", 1), ("Documents/Old/a.txt", "a", 5)]);
        let mut f = w.files("Documents");
        f.do_act(Act::DeletedHere, 0, Vec::new());
        let rows = f.earlier_rows().unwrap();
        assert_eq!(rows.iter().map(|r| (r.0.as_str(), r.1)).collect::<Vec<_>>(), [("Old", 2), ("letter.odt", 2)]);
        f.update(Msg::Earlier(EarlierMsg::PutBack(1)));
        assert_eq!(std::fs::read_to_string(w.live("Documents/letter.odt")).unwrap(), "dear sir or madam");
        assert_eq!(f.earlier_rows().unwrap()[1].2, Some(("Put back where it was.".to_string(), false)));
        f.update(Msg::Earlier(EarlierMsg::PutBack(0)));
        assert_eq!(std::fs::read_to_string(w.live("Documents/Old/a.txt")).unwrap(), "a");
        f.update(Msg::Undo);
        assert!(!w.live("Documents/Old").exists() && w.live("Documents/letter.odt").exists());
        f.update(Msg::Undo);
        assert!(!w.live("Documents/letter.odt").exists());
        f.update(Msg::Ask(crate::files::dialogs::AskMsg::Close));
        assert!(f.dialog.is_none());
        f.do_act(Act::DeletedHere, 0, Vec::new());
        assert_eq!(f.earlier_rows().unwrap().len(), 2, "what Undo took away is gone again");
    }

    #[test]
    fn that_days_folder_opens_in_a_tab_that_says_what_it_is_and_refuses_every_change() {
        let w = World::new("tab");
        write(&w.live("Documents/Letters/now.txt"), "now", 3000);
        write(&w.live("Documents/mine.txt"), "mine", 3000);
        let then = w.copy(1, 1, &[("Documents/Letters/old.txt", "old", 1000), ("Documents/Letters/Sub/deep.txt", "deep", 1000)]);
        let before = w.copies();
        let mut f = w.files("Documents");
        f.do_act(Act::Earlier, 0, vec![w.live("Documents/Letters")]);
        assert_eq!(f.earlier_rows().unwrap().len(), 1);
        f.update(Msg::Earlier(EarlierMsg::Open(0)));
        let at = Where::Folder(then.join("Documents/Letters"));
        assert!(f.dialog.is_none() && f.cols[0].tabs.len() == 2 && f.cols[0].at == at);
        assert_eq!(f.tab_name(&at), "Earlier: Letters");
        let note = f.earlier_note(&at).unwrap();
        assert!(note.starts_with("An earlier copy, as it was ") && note.contains(" at ") && note.contains("Nothing in it can be changed"), "{note}");
        assert!(f.earlier_note(&Where::Folder(w.live("Documents"))).is_none());
        assert_eq!(f.tab_name(&Where::Folder(w.live("Documents"))), "Documents");
        its_menu_offers_only_what_looks_or_copies_out(&mut f);
        every_key_and_row_that_changes_something_is_refused(&mut f, &then);
        nothing_is_moved_pasted_or_dropped_into_it(&mut f, &w, &at);
        no_work_that_would_change_a_copy_is_started(&f, &w, &then);
        assert_eq!(w.copies(), before, "nothing in the copies was changed");
        assert!(w.live("Documents/mine.txt").exists());
        f.side = 0;
        pick(&mut f, &["old.txt"]);
        f.update(Msg::Copy);
        assert_eq!(std::fs::read_to_string(w.live("Documents/old.txt")).unwrap(), "old");
        f.side = 0;
        pick(&mut f, &["Sub"]);
        f.update(Msg::Enter);
        assert!(f.earlier_at(&f.cols[0].at) && f.earlier_note(&f.cols[0].at).is_some());
        assert_eq!(w.copies(), before);
    }

    fn its_menu_offers_only_what_looks_or_copies_out(f: &mut Files) {
        let acts = offered(f, &["old.txt"]);
        assert!(acts.contains(&Act::Open) && acts.contains(&Act::Copy) && acts.contains(&Act::Properties), "{acts:?}");
        for gone in [Act::Cut, Act::Rename, Act::Trash, Act::Delete, Act::Wipe, Act::NewFolder, Act::NewDocument, Act::Compress, Act::Link, Act::Earlier] {
            assert!(!acts.contains(&gone), "{gone:?} is offered in an earlier copy");
        }
        let room = offered(f, &[]);
        assert!(!room.contains(&Act::Paste) && !room.contains(&Act::NewFolder) && !room.contains(&Act::DeletedHere), "{room:?}");
    }

    fn every_key_and_row_that_changes_something_is_refused(f: &mut Files, then: &Path) {
        pick(f, &["old.txt"]);
        for key in [Msg::Trash, Msg::Delete, Msg::Rename, Msg::Cut, Msg::Paste, Msg::NewFolder, Msg::NewDocument] {
            f.said.clear();
            f.update(key);
            assert!(f.said == shut() && !f.bad && f.confirm.is_none() && f.renaming.is_none() && f.clip.is_none(), "{}", f.said);
        }
        for act in [Act::Trash, Act::Delete, Act::Rename, Act::Wipe, Act::Compress, Act::ExtractHere, Act::NewFolder, Act::Link, Act::TurnLeft, Act::Colour] {
            f.said.clear();
            f.do_act(act, 0, vec![then.join("Documents/Letters/old.txt")]);
            assert!(f.said == shut() && f.dialog.is_none(), "{act:?}: {}", f.said);
        }
    }

    fn nothing_is_moved_pasted_or_dropped_into_it(f: &mut Files, w: &World, at: &Where) {
        f.update(Msg::PanesAre(true));
        f.go(1, Where::Folder(w.live("Documents")));
        f.side = 0;
        pick(f, &["old.txt"]);
        f.said.clear();
        f.update(Msg::Move);
        assert_eq!(f.said, shut());
        f.said.clear();
        f.drop_into(vec![w.live("Documents/mine.txt")], at.clone(), crate::files::act::Way::Copy);
        assert_eq!(f.said, shut());
        f.said.clear();
        f.drop_done(Act::LinkHere, vec![w.live("Documents/mine.txt")], at.clone());
        assert_eq!(f.said, shut());
    }

    fn no_work_that_would_change_a_copy_is_started(f: &Files, w: &World, then: &Path) {
        let plan = |kind: Kind, items: Vec<PathBuf>, to: &Path| Plan { kind, items, to: to.to_path_buf(), key: None };
        let old = then.join("Documents/Letters/old.txt");
        assert!(f.earlier_blocks(&plan(Kind::Move, vec![old.clone()], &w.live("Documents"))));
        assert!(f.earlier_blocks(&plan(Kind::Trash, vec![old.clone()], Path::new(""))));
        assert!(f.earlier_blocks(&plan(Kind::Delete, vec![old.clone()], Path::new(""))));
        assert!(f.earlier_blocks(&plan(Kind::Copy, vec![w.live("Documents/mine.txt")], &then.join("Documents/Letters"))));
        assert!(!f.earlier_blocks(&plan(Kind::Copy, vec![old.clone()], &w.live("Documents"))));
        assert!(!f.earlier_blocks(&plan(Kind::Move, vec![w.live("Documents/mine.txt")], &w.live("Documents/Letters"))));
    }
}
