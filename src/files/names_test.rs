//! Tests letting things go on the names of the path at the top, with the

use std::fs;
use std::path::PathBuf;

use super::act::Land;
use super::app::{Files, Msg};
use super::places::Where;
use super::shots::{size, Live};
use crate::toolkit::input::{Button, Input};
use crate::toolkit::ui::App as _;

fn made(tag: &str) -> (PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("trindle-files-names-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let shown = dir.join("home/Videos/FOSSNerds banners");
    fs::create_dir_all(&shown).unwrap();
    fs::write(shown.join("clip.mp4"), b"made up").unwrap();
    for more in ["other.mp4", "third.mp4"] {
        fs::write(shown.join(more), b"made up too").unwrap();
    }
    fs::create_dir_all(dir.join("home/Videos/Sorted")).unwrap();
    (dir, shown)
}

fn window(home: PathBuf, at: PathBuf) -> Files {
    let mut f = Files::new(vec![at]);
    f.home = home;
    f.room = size();
    f.update(Msg::PanesAre(false));
    f
}

fn name(live: &Live, i: usize) -> (f32, f32) {
    let r = live.ui.pill_boxes("path0")[i];
    (r.x + r.w / 2.0, r.y + r.h / 2.0)
}

fn hold_over(live: &mut Live, f: &mut Files, item: &str, button: Button, i: usize) -> (f32, f32) {
    let (x, y) = live.item(f, 0, item);
    let (to_x, to_y) = name(live, i);
    live.send(f, Input::PointerMove { x, y });
    live.send(f, Input::Press { button, x, y });
    live.send(f, Input::PointerMove { x: x + 30.0, y: y - 30.0 });
    live.send(f, Input::PointerMove { x: to_x, y: to_y });
    (to_x, to_y)
}

fn let_go(live: &mut Live, f: &mut Files, button: Button, at: (f32, f32)) {
    live.send(f, Input::Release { button, x: at.0, y: at.1 });
}

struct Resting(tiny_skia::Pixmap);

impl Resting {
    fn of(live: &mut Live, f: &Files) -> Resting {
        Resting(live.show(f))
    }

    fn lit(&self, live: &mut Live, f: &Files, i: usize) -> bool {
        let now = live.show(f);
        let r = live.ui.pill_boxes("path0")[i];
        let w = now.width() as usize;
        (r.y as usize..(r.y + r.h) as usize).any(|y| (r.x as usize..(r.x + r.w) as usize).any(|x| now.data()[(y * w + x) * 4..][..4] != self.0.data()[(y * w + x) * 4..][..4]))
    }
}

#[test]
fn a_file_let_go_on_a_parent_name_goes_into_that_folder_and_undo_takes_it_back() {
    let (dir, shown) = made("parent");
    let mut f = window(dir.join("home"), shown.clone());
    let mut live = Live::new(&f);
    let rest = Resting::of(&mut live, &f);
    let names = live.ui.pill_boxes("path0").len();
    assert_eq!(names, 3, "Home, Videos and the folder shown");
    let at = hold_over(&mut live, &mut f, "clip.mp4", Button::Left, 1);
    assert!(rest.lit(&mut live, &f, 1) && !rest.lit(&mut live, &f, 0) && !rest.lit(&mut live, &f, 2), "Videos is lit, and only it");
    let home = name(&live, 0);
    live.send(&mut f, Input::PointerMove { x: home.0, y: home.1 });
    assert!(rest.lit(&mut live, &f, 0) && !rest.lit(&mut live, &f, 1), "Home is lit instead");
    live.send(&mut f, Input::PointerMove { x: at.0, y: at.1 });
    assert!(rest.lit(&mut live, &f, 1));
    let_go(&mut live, &mut f, Button::Left, at);
    assert!(dir.join("home/Videos/clip.mp4").exists() && !shown.join("clip.mp4").exists(), "{}", f.said);
    f.update(Msg::Undo);
    assert!(shown.join("clip.mp4").exists() && !dir.join("home/Videos/clip.mp4").exists(), "{}", f.said);
    let at = hold_over(&mut live, &mut f, "third.mp4", Button::Left, 0);
    assert!(rest.lit(&mut live, &f, 0));
    let_go(&mut live, &mut f, Button::Left, at);
    assert!(dir.join("home/third.mp4").exists() && !shown.join("third.mp4").exists(), "{}", f.said);
    let at = hold_over(&mut live, &mut f, "other.mp4", Button::Left, 1);
    live.send(&mut f, Input::Mods(crate::toolkit::input::Mods { ctrl: true, ..Default::default() }));
    let_go(&mut live, &mut f, Button::Left, at);
    live.send(&mut f, Input::Mods(Default::default()));
    assert!(dir.join("home/Videos/other.mp4").exists() && shown.join("other.mp4").exists(), "{}", f.said);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn the_folder_shown_and_the_dots_take_nothing_and_what_is_let_go_on_them_stays() {
    let (dir, shown) = made("none");
    let mut f = window(dir.join("home"), shown.clone());
    let mut live = Live::new(&f);
    let rest = Resting::of(&mut live, &f);
    let at = hold_over(&mut live, &mut f, "clip.mp4", Button::Left, 2);
    assert!(!rest.lit(&mut live, &f, 2), "the last name is not lit");
    let_go(&mut live, &mut f, Button::Left, at);
    assert!(shown.join("clip.mp4").exists() && f.jobs.is_empty() && f.menu.is_none() && f.dropping.is_none(), "{}", f.said);
    let long: PathBuf = (1..=5).fold(dir.join("home"), |p, n| p.join(format!("Folder number {n} with a longish name")));
    fs::create_dir_all(&long).unwrap();
    fs::write(long.join("clip.mp4"), b"made up").unwrap();
    fs::write(long.join("other.mp4"), b"made up").unwrap();
    let mut f = window(dir.join("home"), long.clone());
    f.room = (1200.0, 700.0);
    let mut live = Live::new(&f);
    let rest = Resting::of(&mut live, &f);
    let names = live.ui.pill_boxes("path0").len();
    assert!((3..6).contains(&names), "{names} names");
    let at = hold_over(&mut live, &mut f, "clip.mp4", Button::Left, 0);
    assert!(!rest.lit(&mut live, &f, 0), "the dots come first and are not lit");
    let_go(&mut live, &mut f, Button::Left, at);
    assert!(long.join("clip.mp4").exists() && f.jobs.is_empty() && f.dropping.is_none());
    let at = hold_over(&mut live, &mut f, "other.mp4", Button::Left, 1);
    assert!(rest.lit(&mut live, &f, 1) && !rest.lit(&mut live, &f, 0));
    let_go(&mut live, &mut f, Button::Left, at);
    assert!(!long.join("other.mp4").exists(), "{}", f.said);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_place_that_cannot_be_written_to_takes_nothing() {
    use std::os::unix::fs::PermissionsExt;
    let (dir, shown) = made("locked");
    let videos = dir.join("home/Videos");
    fs::set_permissions(&videos, fs::Permissions::from_mode(0o555)).unwrap();
    let mut f = window(dir.join("home"), shown.clone());
    let mut live = Live::new(&f);
    let rest = Resting::of(&mut live, &f);
    let at = hold_over(&mut live, &mut f, "clip.mp4", Button::Left, 1);
    assert!(!rest.lit(&mut live, &f, 1), "Videos cannot be written to");
    let_go(&mut live, &mut f, Button::Left, at);
    assert!(shown.join("clip.mp4").exists() && !videos.join("clip.mp4").exists() && f.jobs.is_empty());
    let at = hold_over(&mut live, &mut f, "other.mp4", Button::Left, 0);
    assert!(rest.lit(&mut live, &f, 0));
    let_go(&mut live, &mut f, Button::Left, at);
    assert!(dir.join("home/other.mp4").exists(), "{}", f.said);
    assert_eq!(f.land_place(Land::Part(0, 1)), None);
    fs::set_permissions(&videos, fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(f.land_place(Land::Part(0, 1)), Some(Where::Folder(videos)));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn the_right_button_asks_on_a_name_and_still_can_copy_in_the_folder_shown() {
    use crate::files::menu::Act;
    let (dir, shown) = made("right");
    let mut f = window(dir.join("home"), shown.clone());
    let mut live = Live::new(&f);
    let rest = Resting::of(&mut live, &f);
    let at = hold_over(&mut live, &mut f, "clip.mp4", Button::Right, 1);
    assert!(rest.lit(&mut live, &f, 1));
    let_go(&mut live, &mut f, Button::Right, at);
    assert_eq!(f.menu.as_ref().map(|m| m.drop.clone()), Some(Some(Where::Folder(dir.join("home/Videos")))));
    assert!(f.menu.as_ref().unwrap().lines.iter().any(|l| l.act == Act::MoveHere));
    assert!(shown.join("clip.mp4").exists(), "nothing is done until it is picked");
    f.menu = None;
    let at = hold_over(&mut live, &mut f, "clip.mp4", Button::Right, 2);
    let_go(&mut live, &mut f, Button::Right, at);
    let acts: Vec<Act> = f.menu.as_ref().map(|m| m.lines.iter().map(|l| l.act).collect()).unwrap_or_default();
    assert!(acts.contains(&Act::CopyHere) && !acts.contains(&Act::MoveHere), "{acts:?}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn nothing_but_a_folder_is_a_place_on_the_computer_page_in_the_trash_in_recent_or_inside_an_archive() {
    let (dir, shown) = made("places");
    let mut f = window(dir.join("home"), shown);
    assert!(f.pill_place(0, 1).is_some() && f.pill_place(0, 2).is_some());
    for at in [Where::Computer, Where::Trash, Where::Recent, Where::Inside(dir.join("home/Videos/films.zip"), String::new())] {
        f.cols[0].at = at.clone();
        assert!((0..4).all(|j| f.pill_place(0, j).is_none()), "{at:?}");
        assert!((0..4).all(|j| f.land_place(Land::Part(0, j)).is_none()), "{at:?}");
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_folder_is_not_let_go_on_its_own_name_or_one_inside_it() {
    let (dir, shown) = made("self");
    let mut f = window(dir.join("home"), shown.clone());
    let videos = dir.join("home/Videos");
    f.drag = Some(super::act::Drag { start: (0.0, 0.0), pane: 1, items: vec![videos.clone()], moving: true, then: None, right: false, pins: true, was: Vec::new() });
    assert_eq!(f.pill_place(0, 1), None, "Videos into itself");
    assert_eq!(f.pill_place(0, 2), None, "or into a folder inside it");
    assert!(f.pill_place(0, 0).is_none(), "Videos is in Home already");
    f.drag.as_mut().unwrap().items = vec![dir.join("home/Videos/Sorted")];
    assert_eq!(f.pill_place(0, 1), None, "already in Videos");
    assert!(f.pill_place(0, 2).is_some() && f.pill_place(0, 0).is_some());
    let _ = fs::remove_dir_all(&dir);
}
