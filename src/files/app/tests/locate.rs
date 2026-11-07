// Typing a place in a pane's own line.

use crate::toolkit::theme::Theme;

use super::*;

#[test]
fn pressing_a_panes_own_line_types_a_place_for_that_pane() {
    let (dir, mut f) = two_panes("line-locate");
    for d in ["Albums/2025", "Letters"] {
        std::fs::create_dir_all(dir.join(d)).unwrap();
    }
    std::fs::write(dir.join("Albums/song.mp3"), b"x").unwrap();
    f.room = (1200.0, 760.0);
    f.go(1, Where::Folder(dir.join("Albums")));
    f.side = 0;
    let mut ui = Ui::new(f.view(), Theme::new(false, "#fbc711"));
    show(&f, &mut ui);
    let on = |ui: &Ui<Msg>, id: &str| ui.boxes().iter().any(|b| b.0 == id);
    let list = ui.boxes().into_iter().find(|b| b.0.starts_with("pane1:")).unwrap().1;
    let line = ui.boxes().into_iter().map(|b| b.1).rfind(|r| r.x == list.x && r.w == list.w && r.y < list.y && r.h > 30.0).expect("the pane's line");
    let (x, y) = (line.x + 40.0, line.y + line.h / 2.0);
    press_at(&mut f, &mut ui, x, y);
    assert_eq!((f.side, f.locate.as_ref().map(|l| l.0)), (1, Some(1)), "that pane is in use and the place is typed for it");
    assert!(on(&ui, "locate-line1") && !on(&ui, "locate") && !on(&ui, "locate-line0"), "the box is in that line alone");
    assert_eq!(f.locate.as_ref().unwrap().1, format!("{}/", dir.join("Albums").display()), "it opens on the folder shown, ready for a name");
    f.update(Msg::LocateText(dir.join("Letters").to_string_lossy().into_owned()));
    f.update(Msg::LocateDone);
    assert_eq!((&f.cols[1].at, &f.cols[0].at, f.locate.is_none()), (&Where::Folder(dir.join("Letters")), &Where::Folder(dir.clone()), true));
    f.update(Msg::LocateLine(0));
    assert_eq!(f.side, 0);
    f.update(Msg::LocateText(dir.join("Albums/song.mp3").to_string_lossy().into_owned()));
    f.update(Msg::LocateDone);
    assert_eq!((&f.cols[0].at, &f.cols[1].at), (&Where::Folder(dir.join("Albums")), &Where::Folder(dir.join("Letters"))));
    f.update(Msg::LocateLine(1));
    f.update(Msg::LocateText("~".into()));
    f.update(Msg::LocateDone);
    assert_eq!(f.cols[1].at, Where::Folder(dir.clone()));
    f.update(Msg::LocateLine(0));
    f.update(Msg::LocateText("/no/such/place".into()));
    f.update(Msg::LocateDone);
    assert!(f.bad && f.said.contains("/no/such/place") && f.locate.is_none() && f.cols[0].at == Where::Folder(dir.join("Albums")), "{}", f.said);
    f.update(Msg::LocateLine(1));
    f.update(Msg::LocateStop);
    assert!(f.locate.is_none() && f.cols[1].at == Where::Folder(dir.clone()));
    f.update(Msg::Locate);
    f.update(Msg::Activate(0));
    assert!(f.locate.is_none(), "the top row is now about the other pane");
    f.update(Msg::LocateLine(1));
    f.side = 0;
    f.update(Msg::PanesAre(false));
    assert!(f.locate.is_none(), "the pane the box was in is no longer shown");
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn one_pane_has_its_own_line_too() {
    let (dir, mut f) = two_panes("line-one");
    std::fs::create_dir_all(dir.join("Albums")).unwrap();
    std::fs::write(dir.join("Albums/song.mp3"), b"x").unwrap();
    f.update(Msg::PanesAre(false));
    f.room = (1200.0, 760.0);
    f.go(0, Where::Folder(dir.join("Albums")));
    f.side = 0;
    let mut ui = Ui::new(f.view(), Theme::new(false, "#fbc711"));
    show(&f, &mut ui);
    let on = |ui: &Ui<Msg>, id: &str| ui.boxes().iter().any(|b| b.0 == id);
    let list = ui.boxes().into_iter().find(|b| b.0.starts_with("pane0:")).unwrap().1;
    let line = ui.boxes().into_iter().map(|b| b.1).rfind(|r| r.x == list.x && r.w == list.w && r.y < list.y && r.h > 30.0).expect("the pane's line");
    assert!((line.h - 34.0).abs() < 0.5, "the line is 34 points tall: {}", line.h);
    let (x, y) = (line.x + 40.0, line.y + line.h / 2.0);
    press_at(&mut f, &mut ui, x, y);
    assert_eq!((f.side, f.locate.as_ref().map(|l| l.0), f.locate_line), (0, Some(0), true));
    assert!(on(&ui, "locate-line0") && !on(&ui, "locate"), "the box is in the pane's line alone");
    assert_eq!(f.locate.as_ref().unwrap().1, format!("{}/", dir.join("Albums").display()));
    let boxed = ui.boxes().into_iter().find(|b| b.0 == "locate-line0").unwrap().1;
    assert!((boxed.h - line.h).abs() < 0.5 && (boxed.y - line.y).abs() < 0.5, "the box is where the line was: {:?} {:?}", boxed, line);
    f.update(Msg::LocateText(dir.to_string_lossy().into_owned()));
    f.update(Msg::LocateDone);
    assert_eq!(f.cols[0].at, Where::Folder(dir.clone()));
    f.update(Msg::LocateLine(0));
    f.update(Msg::LocateStop);
    assert!(f.locate.is_none());
    let _ = std::fs::remove_dir_all(dir);
}

fn press_at(f: &mut Files, ui: &mut Ui<Msg>, x: f32, y: f32) {
    for input in [Input::PointerMove { x, y }, Input::Press { button: Button::Left, x, y }, Input::Release { button: Button::Left, x, y }] {
        send(f, ui, input);
    }
}
