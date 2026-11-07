// Tests dragging files between Folders and another program.

use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::toolkit::dnd;
use crate::toolkit::drag_test::{harness, scratch, Keys, Mouse, Plain, Stage, LEFT_BUTTON, SHIFT, TEXT, URI_LIST};
use crate::toolkit::event::Loop;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::window::{Display, Kind};

use super::act::panes_left;
use super::app::{Files, Msg};
use super::trash;

type Folders = Rc<Shown<Files>>;

const COPY: u32 = 1;
const MOVE: u32 = 2;
const SIZE: (u32, u32) = (600, 780);
const PLAIN: (usize, usize) = (1240, 200);
const ESCAPE: u32 = 1;

fn folders(events: &Loop, display: &Rc<Display>, at: PathBuf, bin: PathBuf) -> Folders {
    let home = at.parent().map(Path::to_path_buf);
    let mut app = Files::new(vec![at]);
    if let Some(home) = home {
        app.home = home;
    }
    app.bin = Some(trash::Bin::at(&bin, None));
    app.bin.as_ref().unwrap().ready().unwrap();
    let kind = Kind::Window { title: "Folders".into(), app_id: super::NAME.into() };
    let shown = Rc::new(Shown::open(events, display, kind, SIZE, app).expect("Folders' window"));
    let w = Rc::downgrade(&shown);
    let post = events.post(move |m: Msg| {
        if let Some(s) = w.upgrade() {
            s.change(|f| f.update(m));
        }
    });
    let window = Rc::downgrade(&shown.window);
    shown.change(move |f| {
        f.set_post(post);
        f.set_drop_hook(move |id, whole| {
            if let Some(window) = window.upgrade() {
                window.drop_done(id, whole);
            }
        });
        f.update(Msg::Changed);
    });
    shown
}

struct Set {
    plain: Plain,
    stage: Stage<Folders>,
    mouse: Mouse,
    off: (f32, f32),
    on_plain: (f32, f32),
    on_pane: (f32, f32),
}

fn on_folders(stage: &Stage<Folders>, mouse: &mut Mouse, plain: &mut Plain, at: (f32, f32)) -> Option<(f32, f32)> {
    const NOWHERE: (f32, f32) = (-1.0, -1.0);
    stage.ask(|s| s.with(|f| f.pointer.set(NOWHERE)));
    mouse.place(at.0, at.1);
    plain.settle(60);
    Some(stage.ask(|s| s.with(|f| f.pointer.get()))).filter(|p| *p != NOWHERE)
}

fn set(here: &Path, bin: &Path) -> Set {
    let run = harness().expect("the harness");
    let socket = run.join(std::env::var("WAYLAND_DISPLAY").unwrap());
    let mut plain = Plain::open(&socket, "A plain program", PLAIN);
    let (here, bin) = (here.to_path_buf(), bin.to_path_buf());
    let stage = Stage::start(move |events, display| folders(events, display, here, bin));
    stage.until("Folders showed", |s| s.window.seen());
    plain.settle(600);
    let mut mouse = plain.mouse();
    let screen = mouse.screen;
    let mut off = None;
    'found: for y in (60..screen.1).step_by(120) {
        for x in (60..screen.0).step_by(120) {
            on_folders(&stage, &mut mouse, &mut plain, (x as f32 + 3.0, y as f32 + 3.0));
            if let Some(p) = on_folders(&stage, &mut mouse, &mut plain, (x as f32, y as f32)) {
                off = Some((x as f32 - p.0, y as f32 - p.1));
                break 'found;
            }
        }
    }
    let off = off.expect("the pointer found Folders' window");
    let size = stage.ask(|s| s.window.size());
    println!("Folders is {size:?} at {off:?}");
    let mut on_plain = None;
    for x in [off.0 - 120.0, off.0 + size.0 as f32 + 120.0, 60.0, screen.0 as f32 - 60.0] {
        mouse.place(x, screen.1 as f32 / 2.0);
        plain.settle(80);
        if let Some(p) = plain.told.pointer {
            on_plain = Some(((x, screen.1 as f32 / 2.0), (x - p.0 as f32, screen.1 as f32 / 2.0 - p.1 as f32)));
            break;
        }
    }
    let (on_plain, plain_off) = on_plain.expect("the pointer found the plain window");
    let on_pane = (off.0 + panes_left() + 60.0, off.1 + size.1 as f32 - 100.0);
    assert!(on_pane.1 > plain_off.1 + PLAIN.1 as f32 + 10.0, "the foot of Folders' pane shows under the plain window: {on_pane:?} against {plain_off:?}");
    assert!(on_folders(&stage, &mut mouse, &mut plain, on_pane).is_some(), "the foot of the pane is on Folders' window");
    Set { plain, stage, mouse, off, on_plain, on_pane }
}

impl Set {
    fn drag_from_plain(&mut self, ways: u32) {
        self.mouse.place(self.on_plain.0, self.on_plain.1);
        self.plain.until("the pointer reached the plain window", |t| t.pointer.is_some());
        self.plain.told.pressed = None;
        self.mouse.button(LEFT_BUTTON, true);
        self.plain.until("the plain window was pressed", |t| t.pressed.is_some());
        self.plain.drag(ways);
        self.mouse.glide(self.on_pane, &mut self.plain);
    }

    fn first_item(&mut self) -> (f32, f32) {
        let x = self.off.0 + panes_left() + 90.0;
        for y in (60..700).step_by(8) {
            let at = (x, self.off.1 + y as f32);
            if on_folders(&self.stage, &mut self.mouse, &mut self.plain, at).is_some() && self.stage.ask(|s| s.with(|f| f.cols[0].hover == Some(0))) {
                return (at.0, at.1 + 4.0);
            }
        }
        panic!("the pointer never found Folders' first item");
    }

    fn drag_from_folders(&mut self) {
        let item = self.first_item();
        self.mouse.place(item.0, item.1);
        self.plain.settle(120);
        self.mouse.button(LEFT_BUTTON, true);
        self.plain.settle(150);
        self.mouse.glide((item.0 + 30.0, item.1 + 20.0), &mut self.plain);
        self.folders_until("Folders offered what is held to other programs", |s| s.window.dragging_out());
        self.mouse.glide(self.on_plain, &mut self.plain);
    }

    fn folders_until(&mut self, what: &str, done: impl Fn(&Folders) -> bool + Send + Clone + 'static) {
        let end = Instant::now() + Duration::from_secs(10);
        while !self.stage.ask(done.clone()) {
            assert!(Instant::now() < end, "never happened: {what}");
            self.plain.settle(30);
        }
    }

    fn wait(&mut self, what: &str, done: impl Fn() -> bool) {
        let end = Instant::now() + Duration::from_secs(10);
        while !done() {
            assert!(Instant::now() < end, "never happened: {what}");
            self.plain.settle(30);
        }
    }

    fn folders<R: Send + 'static>(&self, f: impl FnOnce(&Files) -> R + Send + 'static) -> R {
        self.stage.ask(|s| s.with(f))
    }

    fn update(&self, msg: fn() -> Msg) {
        self.stage.ask(move |s| s.change(|f| f.update(msg())));
    }
}

fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir).map(|d| d.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect()).unwrap_or_default();
    names.sort();
    names
}

const NO_HARNESS: &str = "This test runs only against the headless test compositor: set TRINDLE_DND_TEST to its runtime folder. Without it nothing would be checked.";

#[test]
#[ignore = "needs a real compositor on the screen, named by the script that runs it: run by hand"]
fn between_programs_files_dropped_on_folders_are_copied_and_undone() {
    assert!(harness().is_some(), "{NO_HARNESS}");
    let dir = scratch("in");
    let (here, there) = (dir.join("here"), dir.join("there"));
    std::fs::create_dir_all(&here).unwrap();
    std::fs::create_dir_all(&there).unwrap();
    std::fs::write(there.join("report.txt"), "the report").unwrap();
    std::fs::write(there.join("two words.txt"), "the notes").unwrap();
    let mut set = set(&here, &dir.join("Trash"));
    let both = vec![there.join("report.txt"), there.join("two words.txt")];
    set.plain.told.serving.insert(URI_LIST.into(), dnd::uri_list(&both).into_bytes());
    set.plain.told.serving.insert(TEXT.into(), dnd::path_lines(&both).into_bytes());

    set.drag_from_plain(COPY | MOVE);
    set.plain.until("Folders said it would take a copy", |t| t.source_action == COPY);
    set.mouse.button(LEFT_BUTTON, false);
    set.plain.until("it was let go on Folders, which finished with it", |t| t.performed && t.finished == 1);
    assert!(set.plain.told.asked.contains(&URI_LIST.to_string()), "the list of files was read: {:?}", set.plain.told.asked);
    assert_eq!(set.plain.told.cancelled, 0);
    set.wait("the files were copied", || here.join("report.txt").exists() && here.join("two words.txt").exists());
    set.folders_until("the work ended", |s| s.with(|f| f.jobs.is_empty()));
    assert_eq!(std::fs::read_to_string(here.join("two words.txt")).unwrap(), "the notes");
    assert!(there.join("report.txt").exists(), "a copy leaves the first where it was");
    let (said, bad) = set.folders(|f| (f.said.clone(), f.bad));
    println!("Folders said: {said}");
    assert!(!bad, "{said}");
    set.update(|| Msg::Undo);
    set.wait("Undo took the copies away", || names(&here).is_empty());
    assert_eq!(names(&there), ["report.txt", "two words.txt"]);

    set.drag_from_plain(MOVE);
    set.plain.until("Folders said it would take a move", |t| t.source_action == MOVE);
    set.mouse.button(LEFT_BUTTON, false);
    set.plain.until("Folders finished with it", |t| t.finished == 2);
    assert!(here.join("report.txt").exists() && here.join("two words.txt").exists(), "told only when the copies are made");
    assert_eq!(names(&there), ["report.txt", "two words.txt"], "what another program names is never removed for it");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
#[ignore = "needs a real compositor on the screen, named by the script that runs it: run by hand"]
fn between_programs_a_bad_list_dropped_on_folders_is_refused() {
    assert!(harness().is_some(), "{NO_HARNESS}");
    let dir = scratch("bad");
    let (here, there) = (dir.join("here"), dir.join("there"));
    std::fs::create_dir_all(&here).unwrap();
    std::fs::create_dir_all(&there).unwrap();
    std::fs::write(there.join("report.txt"), "the report").unwrap();
    let mut set = set(&here, &dir.join("Trash"));
    let good = dnd::uri_list(&[there.join("report.txt")]);
    let huge: String = (0..200_000).map(|i| format!("file:///home/john/{i}\r\n")).collect();
    assert!(huge.len() > dnd::MOST_BYTES);
    let cases: Vec<(&str, Vec<u8>, bool, dnd::Why)> = vec![
        ("one line that is not a file", format!("{good}https://example.com/x\r\n").into_bytes(), false, dnd::Why::NotFiles),
        ("a path that climbs", format!("{good}file://{}/../there/report.txt\r\n", there.display()).into_bytes(), false, dnd::Why::NotFiles),
        ("another computer's file", b"file://example.com/etc/passwd\r\n".to_vec(), false, dnd::Why::NotFiles),
        ("a nought in a name", b"file:///home/john/a%00.txt\r\n".to_vec(), false, dnd::Why::NotFiles),
        ("more files than are taken", (0..=dnd::MOST_FILES).map(|i| format!("file:///home/john/{i}\r\n")).collect::<String>().into_bytes(), false, dnd::Why::TooMany),
        ("nothing at all", Vec::new(), false, dnd::Why::Empty),
        ("more than is ever read", huge.into_bytes(), false, dnd::Why::TooBig),
        ("a list that never ends", good.clone().into_bytes(), true, dnd::Why::TooSlow),
    ];
    for (what, bytes, never_ends, why) in cases {
        println!("dropping {what}");
        let (cancelled, finished) = (set.plain.told.cancelled, set.plain.told.finished);
        set.plain.told.serving.clear();
        set.plain.told.serving.insert(URI_LIST.into(), bytes);
        set.plain.told.never_ends = never_ends;
        set.update(|| Msg::Escape);
        set.drag_from_plain(COPY);
        set.plain.until("Folders said it would take a copy", |t| t.source_action == COPY);
        set.mouse.button(LEFT_BUTTON, false);
        let words = why.words();
        set.folders_until(what, move |s| s.with(|f| f.bad && f.said == words));
        set.plain.until("the plain program was told it came to nothing", move |t| t.cancelled == cancelled + 1);
        assert_eq!(set.plain.told.finished, finished, "{what}: never said to be finished");
        assert!(names(&here).is_empty(), "{what}: nothing was brought");
        assert!(set.folders(|f| f.jobs.is_empty()));
    }
    set.plain.told.serving.insert(URI_LIST.into(), good.into_bytes());
    set.plain.told.never_ends = false;
    set.drag_from_plain(COPY);
    set.plain.until("Folders said it would take a copy", |t| t.source_action == COPY);
    set.mouse.button(LEFT_BUTTON, false);
    set.wait("the file was copied", || here.join("report.txt").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
#[ignore = "needs a real compositor on the screen, named by the script that runs it: run by hand"]
fn between_programs_files_dragged_out_of_folders_arrive_and_a_move_waits() {
    let run = harness().expect(NO_HARNESS);
    let dir = scratch("out");
    let (there, bin) = (dir.join("there"), dir.join("Trash"));
    std::fs::create_dir_all(&there).unwrap();
    std::fs::write(there.join("a report.txt"), "the report").unwrap();
    std::fs::write(there.join("b.txt"), "b").unwrap();
    let first = there.join("a report.txt");
    let mut set = set(&there, &bin);
    assert_eq!(set.folders(|f| f.cols[0].entries.first().map(|e| e.path.clone())), Some(first.clone()));

    left_on_a_program_that_does_not_want_it(&mut set, &there);
    taken_as_a_copy(&mut set, &there, &first);
    let Some(keys) = Keys::outer(&run) else {
        println!("no keyboard on the outer compositor: the move and Escape were not tried");
        let _ = std::fs::remove_dir_all(&dir);
        return;
    };
    given_up_with_escape(&mut set, &there, &keys);
    taken_as_a_move_with_shift(&mut set, &there, &bin, &first, &keys);
    let _ = std::fs::remove_dir_all(&dir);
}

fn left_on_a_program_that_does_not_want_it(set: &mut Set, there: &Path) {
    set.plain.told.answer = None;
    set.drag_from_folders();
    set.plain.until("it was held over the plain window", |t| t.held.is_some());
    let kinds = set.plain.kinds();
    for kind in [URI_LIST, TEXT, "text/plain"] {
        assert!(kinds.iter().any(|k| k == kind), "{kind} is offered: {kinds:?}");
    }
    assert_eq!(set.plain.ways(), COPY, "a move is not offered unasked");
    set.mouse.button(LEFT_BUTTON, false);
    set.folders_until("Folders heard it came to nothing", |s| s.with(|f| f.drag.is_none()) && !s.window.dragging_out());
    assert!(set.plain.told.dropped.is_none());
    assert_eq!(names(there), ["a report.txt", "b.txt"]);
}

fn taken_as_a_copy(set: &mut Set, there: &Path, first: &Path) {
    set.plain.told.answer = Some((URI_LIST.into(), COPY | MOVE, MOVE));
    set.drag_from_folders();
    set.plain.until("the plain window was offered a copy", |t| t.held.is_some() && t.action == COPY);
    set.mouse.button(LEFT_BUTTON, false);
    set.plain.until("it was let go on the plain window", |t| t.dropped.is_some());
    assert_eq!(String::from_utf8(set.plain.read(URI_LIST)).unwrap(), format!("file://{}/a%20report.txt\r\n", there.display()));
    assert_eq!(dnd::parse_uri_list(&set.plain.read(URI_LIST)), Ok(vec![first.to_path_buf()]));
    assert_eq!(String::from_utf8(set.plain.read(TEXT)).unwrap(), first.display().to_string());
    assert!(set.folders(|f| f.drag.is_some()), "held until the other program has finished");
    set.plain.finish();
    set.folders_until("Folders heard it was taken", |s| s.with(|f| f.drag.is_none()));
    set.plain.settle(300);
    assert_eq!(names(there), ["a report.txt", "b.txt"], "a copy leaves everything where it was");
}

fn given_up_with_escape(set: &mut Set, there: &Path, keys: &Keys) {
    set.drag_from_folders();
    set.plain.until("it was held over the plain window", |t| t.held.is_some());
    keys.key(ESCAPE, true);
    keys.key(ESCAPE, false);
    set.folders_until("Folders let go of it", |s| s.with(|f| f.drag.is_none()) && !s.window.dragging_out());
    let on_plain = set.on_plain;
    set.mouse.glide((on_plain.0, on_plain.1 + 30.0), &mut set.plain);
    set.plain.until("the pointer is the plain window's again, the button still down", |t| t.pointer.is_some() && t.held.is_none());
    set.mouse.button(LEFT_BUTTON, false);
    set.plain.settle(200);
    assert!(set.plain.told.dropped.is_none());
    assert_eq!(names(there), ["a report.txt", "b.txt"]);
}

fn taken_as_a_move_with_shift(set: &mut Set, there: &Path, bin: &Path, first: &Path, keys: &Keys) {
    let item = set.first_item();
    set.mouse.place(item.0, item.1);
    set.mouse.button(LEFT_BUTTON, true);
    set.mouse.button(LEFT_BUTTON, false);
    set.plain.settle(700);
    keys.key(SHIFT, true);
    set.folders_until("Folders saw Shift go down", |s| s.with(|f| f.mods.get().shift));
    set.drag_from_folders();
    keys.key(SHIFT, false);
    set.plain.until("the plain window was offered a move", |t| t.held.is_some() && t.action == MOVE);
    assert_eq!(set.plain.ways(), MOVE);
    set.mouse.button(LEFT_BUTTON, false);
    set.plain.until("it was let go on the plain window", |t| t.dropped.is_some());
    assert_eq!(dnd::parse_uri_list(&set.plain.read(URI_LIST)), Ok(vec![first.to_path_buf()]));
    set.plain.settle(800);
    assert!(first.exists(), "nothing is removed before the other program has finished");
    assert!(names(&bin.join("files")).is_empty());
    set.plain.finish();
    set.wait("it went to the Trash once the other program finished", || !first.exists());
    set.folders_until("the work ended", |s| s.with(|f| f.jobs.is_empty()));
    assert_eq!(names(&bin.join("files")), ["a report.txt"]);
    assert_eq!(names(there), ["b.txt"]);
    set.update(|| Msg::Undo);
    set.wait("Undo brought it back", || first.exists());
    assert_eq!(std::fs::read_to_string(first).unwrap(), "the report");
}
