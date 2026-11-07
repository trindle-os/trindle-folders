// Files: Trindle's own file manager, drawn by the Trindle Toolkit.

pub mod kinds;
mod act;
mod alone;
pub mod help;
#[cfg(test)]
mod help_cases;
mod app;
mod archive;
pub mod askpass;
mod below;
mod checksum;
mod choose;
mod clip;
mod columns;
mod compare;
mod density;
mod computer;
mod details;
mod dialogs;
mod diff;
mod dupes;
mod earlier;
mod fonts;
mod far;
mod reading;
mod found;
mod groups;
mod heading;
mod inplace;
mod inside;
mod icons;
mod imaging;
mod index;
mod jobs;
mod kept;
mod leave;
mod libarchive;
mod locked;
mod media;
mod menu;
mod meta;
mod meter;
mod narrow;
mod nearby;
mod neighbours;
mod outside;
mod pair;
#[cfg(test)]
mod outside_test;
#[cfg(test)]
mod big_test;
mod panel;
mod pictures;
pub(crate) mod places;
mod play;
mod preview;
mod progress;
mod queue;
mod remote;
mod remembered;
mod rename;
mod search;
mod settings;
mod shown;
#[cfg(test)]
mod shots;
#[cfg(test)]
mod names_test;
#[cfg(test)]
mod page_shots;
mod drives;
mod shares;
mod sharing;
mod sized;
mod sizes;
mod thumbs;
mod tools;
pub mod unarchive;
pub(crate) mod view;
mod viewer;
pub(crate) mod walled;
mod wipe;
mod zoom;
mod work;

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use std::time::Duration;

use crate::common::volumes;
use crate::toolkit::common::file_work::{proof, reach, trash};
use crate::toolkit::t;
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::window::{Display, Kind};
use act::Busy;
use app::{Files, Msg};

pub(super) const NAME: &str = match option_env!("FOLDERS_BUS_NAME") { Some(n) => n, None => IFACE };
const IFACE: &str = "org.trindle.Files";
const PATH: &str = "/org/trindle/Files";
const XML: &str = r#"<interface name="org.trindle.Files"><method name="Show"/><method name="Settings"/><method name="Open"><arg type="as" direction="in"/></method></interface>"#;
pub const NEW_WINDOW: &str = "--new-window";
const MANAGER: &str = "org.freedesktop.FileManager1";
const MANAGER_PATH: &str = "/org/freedesktop/FileManager1";
const MANAGER_XML: &str = r#"<interface name="org.freedesktop.FileManager1"><method name="ShowFolders"><arg type="as" direction="in"/><arg type="s" direction="in"/></method><method name="ShowItems"><arg type="as" direction="in"/><arg type="s" direction="in"/></method><method name="ShowItemProperties"><arg type="as" direction="in"/><arg type="s" direction="in"/></method></interface>"#;
pub const FOR_A_CALL: &str = "--for-a-call";
const MANAGER_MOST: usize = 32;
const SIZE: (u32, u32) = (1200, 760);

enum Do {
    Show,
    Open(Vec<PathBuf>),
    Reveal(Vec<PathBuf>, bool),
    Settings,
}

struct Running {
    shown: Rc<Shown<Files>>,
    _served: Option<Served>,
    _raise: Option<Served>,
    _manager: Option<Served>,
    _index: index::Indexer,
}

pub fn main(args: Vec<String>) -> i32 {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    let folders = folders(args.iter().skip(1).map(String::as_str), &cwd);
    let settings = args.iter().any(|a| a == settings::OPEN_FLAG);
    let for_a_call = args.iter().any(|a| a == FOR_A_CALL);
    let events = Loop::new();
    if args.iter().any(|a| a == choose::FLAG) {
        return choose_for_a_program(&events);
    }
    if args.iter().any(|a| a == NEW_WINDOW) {
        return open(&events, folders, settings);
    }
    let Some(bus) = Bus::session(&events).ok() else { return open(&events, folders, settings) };
    let code = Rc::new(Cell::new(0));
    let (ev, c) = (events.clone(), code.clone());
    let kept = Rc::new(RefCell::new(None));
    let (k, b) = (kept.clone(), bus.clone());
    let wanted = RefCell::new(Some(folders.clone()));
    let (ev2, b2) = (ev.clone(), b.clone());
    crate::toolkit::once::claim(&bus, &events, NAME, PATH, move || {
        inside::forget_taken_out();
        match start(&ev, &b, wanted.borrow_mut().take().unwrap_or_default(), for_a_call) {
            Some(running) => {
                if settings {
                    running.shown.change(|f| f.update(Msg::Own(settings::OwnMsg::Open)));
                }
                *k.borrow_mut() = Some(running);
            }
            None => {
                c.set(1);
                ev.quit();
            }
        }
    }, move || {
        let (ev, b) = (ev2, b2);
        let paths: Vec<String> = folders.iter().map(|p| p.to_string_lossy().into_owned()).collect();
        let quit = ev.clone();
        if settings {
            let (again, ev2) = (b.clone(), ev.clone());
            b.call(NAME, PATH, IFACE).method("Open", &(paths,), false, Some(Duration::from_secs(2)), &ev, move |_| {
                again.call(NAME, PATH, IFACE).method("Settings", &(), false, Some(Duration::from_secs(2)), &ev2, move |_| quit.quit());
            });
            return;
        }
        b.call(NAME, PATH, IFACE).method("Open", &(paths,), false, Some(Duration::from_secs(2)), &ev, move |_| quit.quit());
    });
    events.run();
    if let Some(r) = kept.borrow().as_ref() {
        r.shown.with(Files::closing);
    }
    drop(kept);
    code.get()
}

fn folders<'a>(args: impl Iterator<Item = &'a str>, cwd: &Path) -> Vec<PathBuf> {
    args.filter(|a| !a.starts_with('-'))
        .filter_map(|a| if a.starts_with("file://") { kinds::uri_path(a) } else { Some(PathBuf::from(a)) })
        .map(|p| if p.is_absolute() { p } else { cwd.join(p) })
        .filter_map(|p| if shows(&p) { Some(p) } else { p.parent().filter(|d| d.is_dir()).map(Path::to_path_buf) })
        .collect()
}

fn shows(path: &Path) -> bool {
    reach::is_dir(path) || inside::opens(path)
}

fn start(events: &Loop, bus: &Bus, folders: Vec<PathBuf>, for_a_call: bool) -> Option<Running> {
    let (shown, asks) = window(events, folders, false)?;
    if for_a_call {
        shown.change(|f| f.for_a_call = true);
    }
    let manager = serve_manager(events, bus, asks.clone());
    let served = bus.serve(PATH, Offer {
        name: IFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            if asked.member() == "Open" {
                let args = asked.message().body().deserialize::<(Vec<String>,)>().map(|a| a.0).unwrap_or_default();
                let paths: Vec<PathBuf> = args.into_iter().map(PathBuf::from).filter(|p| p.is_absolute() && shows(p)).collect();
                if !paths.is_empty() {
                    asks.send(Do::Open(paths));
                }
            }
            asks.send(Do::Show);
            if asked.member() == "Settings" {
                asks.send(Do::Settings);
            }
            asked.reply(&());
        }),
        props: Box::new(Props::new),
        set: None,
    }).ok();
    let raise = crate::toolkit::once::serve_window(bus, PATH, &shown.window).ok();
    Some(Running { shown, _served: served, _raise: raise, _manager: manager, _index: index::Indexer::home() })
}

fn serve_manager(events: &Loop, bus: &Bus, asks: Post<Do>) -> Option<Served> {
    if NAME != IFACE {
        return None;
    }
    bus.own(MANAGER, false, events, |_| {});
    bus.serve(MANAGER_PATH, Offer {
        name: MANAGER.into(),
        xml: MANAGER_XML.into(),
        call: Box::new(move |asked: Asked| {
            let uris = asked.message().body().deserialize::<(Vec<String>, String)>().map(|a| a.0).unwrap_or_default();
            if let Some(ask) = manager_ask(&asked.member(), &uris) {
                asks.send(ask);
            }
            asks.send(Do::Show);
            asked.reply(&());
        }),
        props: Box::new(Props::new),
        set: None,
    }).ok()
}

fn manager_ask(method: &str, uris: &[String]) -> Option<Do> {
    let paths: Vec<PathBuf> = uris.iter().take(MANAGER_MOST)
        .filter_map(|u| if u.starts_with("file://") { kinds::uri_path(u) } else { None })
        .filter(|p| p.is_absolute() && p.exists())
        .collect();
    if paths.is_empty() {
        return None;
    }
    match method {
        "ShowFolders" => Some(Do::Open(paths.into_iter().filter(|p| shows(p)).collect())),
        "ShowItems" => Some(Do::Reveal(paths, false)),
        "ShowItemProperties" => Some(Do::Reveal(paths, true)),
        _ => None,
    }
}

fn choose_for_a_program(events: &Loop) -> i32 {
    let Some(ask) = choose::read_ask(std::io::stdin().lock()) else {
        eprintln!("trindle-folders: the request on standard input could not be read");
        return 2;
    };
    let start = ask.folder.clone().filter(|p| p.is_absolute() && reach::is_dir(p)).unwrap_or_else(crate::common::files::home);
    let title = choose::title(&ask);
    let Some((shown, _)) = window(events, vec![start], true) else { return 1 };
    shown.window.set_title(&title);
    shown.change(|f| {
        f.update(Msg::PanesAre(false));
        f.choosing_for(choose::Choosing::new(ask, |outcome| choose::print(&outcome)));
    });
    events.run();
    shown.with(Files::closing);
    0
}

fn open(events: &Loop, folders: Vec<PathBuf>, settings: bool) -> i32 {
    let Some(running) = window(events, folders, false) else { return 1 };
    if settings {
        running.0.change(|f| f.update(Msg::Own(settings::OwnMsg::Open)));
    }
    events.run();
    running.0.with(Files::closing);
    0
}

const PLUGGED_SETTLE: Duration = Duration::from_millis(800);
const RELIST_SETTLE: Duration = Duration::from_millis(400);
const DRIVES_RECHECK: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Next {
    Stay,
    Hide,
    Show,
    Quit,
}

fn on_close(busy: Busy) -> Next {
    if busy.asking {
        Next::Stay
    } else if busy.running > 0 {
        Next::Hide
    } else {
        Next::Quit
    }
}

fn on_news(away: bool, over: bool, busy: Busy) -> Next {
    if busy.asking || busy.trouble {
        if away { Next::Show } else { Next::Stay }
    } else if busy.running == 0 && (away || over) {
        Next::Quit
    } else {
        Next::Stay
    }
}

fn window(events: &Loop, folders: Vec<PathBuf>, chooser: bool) -> Option<(Rc<Shown<Files>>, Post<Do>)> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    let slot: Rc<RefCell<Weak<Shown<Files>>>> = Rc::new(RefCell::new(Weak::new()));
    let away = Rc::new(Cell::new(false));
    let asks = answer_asks(events, slot.clone(), away.clone());
    let app = Files::new(folders);
    let fd = app.watch_fd();
    let kind = Kind::Window { title: t!("folders-mod-window-title").into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, SIZE, app)?);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    let w = Rc::downgrade(&shown);
    let post = events.post(move |m: Msg| {
        if let Some(s) = w.upgrade() {
            s.change(|f| f.update(m));
        }
    });
    let over = Rc::new(Cell::new(false));
    let close = closer(events, &shown, away.clone(), over.clone(), chooser, post.clone());
    let c = close.clone();
    shown.window.on_close(move || c());
    let closing = events.post(move |_: ()| close());
    repaint_on_frames(events, &shown);
    let told = told_of_work(events, away, over, asks.clone());
    let w = Rc::downgrade(&shown);
    let speak = events.post(move |words: String| {
        if let Some(s) = w.upgrade() {
            s.announce(&words);
        }
    });
    let drives_post = post.clone();
    hook_up(events, &shown, post, speak, told, closing);
    let window = Rc::downgrade(&shown.window);
    shown.change(move |f| f.set_drop_hook(move |id, whole| {
        if let Some(window) = window.upgrade() {
            window.drop_done(id, whole);
        }
    }));
    keep_drives(events, &display, &shown, drives_post);
    if let Some(fd) = fd {
        watch_folders(events, &shown, fd);
    }
    if let Some(plugged) = remote::Plugged::open() {
        watch_plugged(events, &shown, plugged);
    }
    Some((shown, asks))
}

fn answer_asks(events: &Loop, slot: Rc<RefCell<Weak<Shown<Files>>>>, away: Rc<Cell<bool>>) -> Post<Do> {
    events.post(move |ask: Do| {
        let Some(shown) = slot.borrow().upgrade() else { return };
        match ask {
            Do::Show => {
                away.set(false);
                shown.window.raise();
            }
            Do::Open(paths) => shown.change(|f| f.update(Msg::Add(paths))),
            Do::Reveal(items, props) => shown.change(|f| f.update(Msg::Reveal(items, props))),
            Do::Settings => shown.change(|f| f.update(Msg::Own(settings::OwnMsg::Open))),
        }
    })
}

fn closer(events: &Loop, shown: &Rc<Shown<Files>>, away: Rc<Cell<bool>>, over: Rc<Cell<bool>>, chooser: bool, waiting: Post<Msg>) -> Rc<dyn Fn()> {
    let (ev, w) = (events.clone(), Rc::downgrade(shown));
    Rc::new(move || {
        let Some(s) = w.upgrade() else { return ev.quit() };
        s.change(Files::choose_closed);
        over.set(chooser);
        match on_close(s.with(Files::busy)) {
            Next::Hide => {
                away.set(true);
                s.window.set_visible(false);
            }
            Next::Quit => ev.quit(),
            Next::Stay | Next::Show => {
                waiting.send(Msg::Waiting);
            }
        }
    })
}

fn repaint_on_frames(events: &Loop, shown: &Rc<Shown<Files>>) {
    let w = Rc::downgrade(shown);
    let frame = events.post(move |_: ()| {
        if let Some(s) = w.upgrade() {
            s.window.patch();
        }
    });
    play::set_repaint(move || {
        frame.send(());
    });
}

fn told_of_work(events: &Loop, away: Rc<Cell<bool>>, over: Rc<Cell<bool>>, back: Post<Do>) -> Post<Busy> {
    let ev = events.clone();
    events.post(move |busy: Busy| match on_news(away.get(), over.get(), busy) {
        Next::Show => {
            back.send(Do::Show);
        }
        Next::Quit => ev.quit(),
        Next::Stay | Next::Hide => {}
    })
}

fn hook_up(events: &Loop, shown: &Rc<Shown<Files>>, post: Post<Msg>, speak: Post<String>, told: Post<Busy>, closing: Post<()>) {
    let (ev, back) = (events.clone(), post.clone());
    shown.change(move |f| {
        f.set_post(post);
        f.set_speak(speak);
        let chooser = Rc::new(RefCell::new(None));
        f.set_pick_hook(move |pick| {
            let (back, then) = (back.clone(), pick.then);
            let ask = crate::toolkit::chooser::Ask { title: pick.ask.title, folder: pick.ask.folder, ..Default::default() };
            let done = move |path: Option<PathBuf>| {
                if let Some(m) = then(path.into_iter().collect()) {
                    back.send(m);
                }
            };
            let Ok(bus) = Bus::session(&ev) else { return };
            let listen = if pick.ask.directory { crate::toolkit::chooser::open_folder(&bus, &ev, ask, done) } else { crate::toolkit::chooser::open(&bus, &ev, ask, done) };
            *chooser.borrow_mut() = listen;
        });
        f.set_jobs_hook(move |busy| {
            told.send(busy);
        });
        f.set_close_hook(move || {
            closing.send(());
        });
    });
}

fn watch_folders(events: &Loop, shown: &Rc<Shown<Files>>, fd: std::os::fd::RawFd) {
    let (w, ev, due) = (Rc::downgrade(shown), events.clone(), Rc::new(Cell::new(false)));
    events.watch(fd, move |_| {
        let Some(shown) = w.upgrade() else { return Flow::Stop };
        if shown.with(Files::saw) && !due.replace(true) {
            let (w, due) = (w.clone(), due.clone());
            ev.after(RELIST_SETTLE, move || {
                due.set(false);
                if let Some(s) = w.upgrade() {
                    s.change(|f| f.update(Msg::Changed));
                }
            });
        }
        Flow::Continue
    });
}

fn watch_plugged(events: &Loop, shown: &Rc<Shown<Files>>, plugged: remote::Plugged) {
    let fd = plugged.fd();
    let (w, ev, due) = (Rc::downgrade(shown), events.clone(), Rc::new(Cell::new(false)));
    events.watch(fd, move |_| {
        if w.strong_count() == 0 {
            return Flow::Stop;
        }
        if plugged.drain() && !due.replace(true) {
            let (w, due) = (w.clone(), due.clone());
            ev.after(PLUGGED_SETTLE, move || {
                due.set(false);
                if let Some(s) = w.upgrade() {
                    s.change(|f| f.update(Msg::Far(far::FarMsg::Plugged)));
                }
            });
        }
        Flow::Continue
    });
}

fn keep_drives(events: &Loop, display: &Rc<Display>, shown: &Rc<Shown<Files>>, post: Post<Msg>) {
    let Ok(system) = Bus::system(events) else { return };
    let slot: Rc<RefCell<Option<volumes::Watching>>> = Rc::new(RefCell::new(None));
    let (p, ev, weak) = (post.clone(), events.clone(), Rc::downgrade(&slot));
    let watching = volumes::watch_all(&system, events, move |list| {
        if crate::common::locked::path().exists() {
            let weak = weak.clone();
            ev.after(DRIVES_RECHECK, move || {
                if let Some(slot) = weak.upgrade() {
                    if let Some(w) = slot.borrow().as_ref() {
                        w.repeat();
                    }
                }
            });
            return;
        }
        p.send(Msg::Drive(drives::DriveMsg::Listed(list.to_vec())));
    });
    *slot.borrow_mut() = Some(watching);
    let (ev, display) = (events.clone(), display.clone());
    shown.change(move |f| {
        f.set_drives(slot, move |cmd| {
            let (bus, lp) = (system.clone(), ev.clone());
            match cmd {
                drives::Cmd::Mount(v) => {
                    let (back, id) = (post.clone(), v.id.clone());
                    volumes::mount(&bus, &lp, &v, move |r| {
                        back.send(Msg::Drive(drives::DriveMsg::Mounted(id, r)));
                    });
                }
                drives::Cmd::Unlock(v) => {
                    let (back, id) = (post.clone(), v.id.clone());
                    volumes::ask_and_mount(&bus, &lp, &display, &v, move |r| {
                        back.send(Msg::Drive(drives::DriveMsg::Mounted(id, r)));
                    });
                }
                drives::Cmd::Eject(id, steps) => {
                    let back = post.clone();
                    volumes::eject(&bus, &lp, steps, move |r| {
                        back.send(Msg::Drive(drives::DriveMsg::Ejected(id, r)));
                    });
                }
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_window_closed_while_work_goes_on_leaves_the_work_going_and_comes_back_to_ask() {
        let busy = |running, asking, trouble| Busy { running, asking, trouble };
        assert_eq!(on_close(busy(0, false, false)), Next::Quit);
        assert_eq!(on_close(busy(0, false, true)), Next::Quit);
        assert_eq!(on_close(busy(2, false, false)), Next::Hide);
        assert_eq!(on_close(busy(1, true, false)), Next::Stay);
        assert_eq!(on_news(false, false, busy(0, false, false)), Next::Stay);
        assert_eq!(on_news(false, false, busy(1, true, true)), Next::Stay);
        assert_eq!(on_news(true, false, busy(1, false, false)), Next::Stay);
        assert_eq!(on_news(true, false, busy(1, true, false)), Next::Show);
        assert_eq!(on_news(true, false, busy(0, false, true)), Next::Show);
        assert_eq!(on_news(true, false, busy(0, false, false)), Next::Quit);
    }

    #[test]
    fn a_chooser_that_has_answered_ends_with_its_work_on_the_screen_or_off_it() {
        let busy = |running, asking, trouble| Busy { running, asking, trouble };
        assert_eq!(on_news(false, true, busy(1, false, false)), Next::Stay);
        assert_eq!(on_news(false, true, busy(1, true, false)), Next::Stay);
        assert_eq!(on_news(false, true, busy(0, false, true)), Next::Stay);
        assert_eq!(on_news(false, true, busy(0, false, false)), Next::Quit);
        assert_eq!(on_news(false, false, busy(0, false, false)), Next::Stay);
        assert_eq!(on_news(true, true, busy(1, true, false)), Next::Show);
        assert_eq!(on_news(false, true, busy(0, false, false)), Next::Quit);
    }

    #[test]
    fn asked_for_paths_become_folders() {
        let dir = std::env::temp_dir().join(format!("trindle-files-args-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("inner")).unwrap();
        std::fs::write(dir.join("note.txt"), b"hello").unwrap();
        let uri = kinds::file_uri(&dir.join("inner"));
        let got = folders(["inner", "note.txt", "missing/also", "--new-window", uri.as_str()].into_iter(), &dir);
        assert_eq!(got, [dir.join("inner"), dir.clone(), dir.join("inner")]);
        for name in ["photos.rar", "old.tar.zst", "disc.iso", "notes.txt.gz", "some.deb"] {
            std::fs::write(dir.join(name), b"x").unwrap();
        }
        let got = folders(["photos.rar", "old.tar.zst", "disc.iso", "notes.txt.gz", "some.deb"].into_iter(), &dir);
        assert_eq!(got, [dir.join("photos.rar"), dir.join("old.tar.zst"), dir.join("disc.iso"), dir.join("notes.txt.gz"), dir.clone()]);
        assert_eq!(inside::place(dir.join("photos.rar")), places::Where::Inside(dir.join("photos.rar"), String::new()));
        assert_eq!(inside::place(dir.join("inner")), places::Where::Folder(dir.join("inner")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn show_in_folder_calls_are_understood() {
        let dir = std::env::temp_dir().join(format!("trindle-files-manager-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("inner")).unwrap();
        std::fs::write(dir.join("a.txt"), "a").unwrap();
        let uri = |p: &Path| kinds::file_uri(p);
        let items = vec![uri(&dir.join("a.txt")), uri(&dir.join("gone.txt"))];
        assert!(matches!(manager_ask("ShowItems", &items), Some(Do::Reveal(p, false)) if p == [dir.join("a.txt")]));
        assert!(matches!(manager_ask("ShowItemProperties", &items), Some(Do::Reveal(_, true))));
        let both = vec![uri(&dir.join("inner")), uri(&dir.join("a.txt"))];
        assert!(matches!(manager_ask("ShowFolders", &both), Some(Do::Open(p)) if p == [dir.join("inner")]));
        assert!(manager_ask("ShowItems", &[uri(&dir.join("gone.txt"))]).is_none());
        assert!(manager_ask("ShowItems", &["file://a.txt".to_string(), "a.txt".to_string(), "sftp://host/a.txt".to_string()]).is_none());
        assert!(manager_ask("Unknown", &items).is_none());
        let crowd = vec![uri(&dir.join("a.txt")); MANAGER_MOST * 3];
        assert!(matches!(manager_ask("ShowItems", &crowd), Some(Do::Reveal(p, _)) if p.len() == MANAGER_MOST));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_bus_service_file_starts_folders_for_a_call() {
        let service = include_str!("../../packaging/dbus/org.trindle.Files.FileManager1.service");
        assert!(service.contains("Name=org.freedesktop.FileManager1"));
        assert!(service.contains(&format!("Exec=/usr/bin/trindle-folders {FOR_A_CALL}")));
        assert!(include_str!("../../packaging/build-deb.sh").contains("org.trindle.Files.FileManager1.service"));
        assert!(MANAGER_XML.contains("ShowFolders") && MANAGER_XML.contains("ShowItems") && MANAGER_XML.contains("ShowItemProperties"));
    }
}
