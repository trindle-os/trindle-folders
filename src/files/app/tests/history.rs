// Back and Forward, linked panes, and keeping the chosen item in sight.

use super::*;

#[test]
fn linked_panes_go_in_and_up_together() {
    let dir = std::env::temp_dir().join(format!("trindle-files-linked-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for d in ["l/photos/2026", "l/music", "r/photos/2026"] {
        std::fs::create_dir_all(dir.join(d)).unwrap();
    }
    let (l, r) = (dir.join("l"), dir.join("r"));
    let mut f = Files::new(vec![l.clone()]);
    f.go(1, Where::Folder(r.clone()));
    f.go(0, Where::Folder(l.clone()));
    let at = |f: &Files, p: usize| f.cols[p].tabs[0].at.clone();
    let find = |f: &Files, p: usize, name: &str| f.cols[p].tabs[0].entries.iter().position(|e| e.name == name).unwrap();

    assert!(!f.kept.linked);
    f.open(0, find(&f, 0, "photos"));
    assert_eq!(at(&f, 1), Where::Folder(r.clone()));
    f.update(Msg::Up);

    f.update(Msg::Link);
    assert!(f.kept.linked);
    f.open(0, find(&f, 0, "photos"));
    assert_eq!(at(&f, 0), Where::Folder(l.join("photos")));
    assert_eq!(at(&f, 1), Where::Folder(r.join("photos")));
    assert_eq!(f.side(), 0);
    f.open(0, find(&f, 0, "2026"));
    assert_eq!(at(&f, 1), Where::Folder(r.join("photos/2026")));
    f.update(Msg::Up);
    f.update(Msg::Up);
    assert_eq!(at(&f, 0), Where::Folder(l.clone()));
    assert_eq!(at(&f, 1), Where::Folder(r.clone()));

    f.open(0, find(&f, 0, "music"));
    assert_eq!(at(&f, 0), Where::Folder(l.join("music")));
    assert_eq!(at(&f, 1), Where::Folder(r.clone()));
    assert!(f.link_note.as_deref().is_some_and(|n| n.contains("music")));

    f.update(Msg::Up);
    assert_eq!(at(&f, 0), Where::Folder(l.clone()));
    assert_eq!(at(&f, 1), Where::Folder(r.clone()));

    f.update(Msg::Panes);
    f.open(0, find(&f, 0, "photos"));
    assert_eq!(at(&f, 1), Where::Folder(r.clone()));
    f.update(Msg::Panes);
    f.update(Msg::Link);
    assert!(!f.kept.linked && f.link_note.is_none());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn back_and_forward_retrace_the_way() {
    let dir = std::env::temp_dir().join(format!("trindle-files-history-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("a/b")).unwrap();
    std::fs::create_dir_all(dir.join("c")).unwrap();
    let mut f = Files::new(vec![dir.clone()]);
    f.go(0, Where::Folder(dir.clone()));
    let at = |f: &Files| f.cols[0].tabs[0].at.clone();
    f.go(0, Where::Folder(dir.join("a")));
    f.go(0, Where::Folder(dir.join("a/b")));
    f.update(Msg::Back);
    assert_eq!(at(&f), Where::Folder(dir.join("a")));
    f.update(Msg::Up);
    assert_eq!(at(&f), Where::Folder(dir.clone()));
    let pane = &f.cols[0].tabs[0];
    assert_eq!(pane.chosen.map(|i| pane.entries[i].name.as_str()), Some("a"));
    f.update(Msg::Back);
    assert_eq!(at(&f), Where::Folder(dir.join("a")));
    f.update(Msg::Back);
    assert_eq!(at(&f), Where::Folder(dir.clone()));
    f.update(Msg::Forward);
    assert_eq!(at(&f), Where::Folder(dir.join("a")));
    f.go(0, Where::Folder(dir.join("c")));
    assert!(f.cols[0].tabs[0].ahead.is_empty());
    f.update(Msg::Forward);
    assert_eq!(at(&f), Where::Folder(dir.join("c")));
    f.update(Msg::Back);
    assert_eq!(at(&f), Where::Folder(dir.join("a")));
    for i in 0..HISTORY + 10 {
        f.go(0, Where::Folder(dir.join(if i % 2 == 0 { "a" } else { "c" })));
    }
    assert_eq!(f.cols[0].tabs[0].back.len(), HISTORY);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_item_the_arrow_keys_reach_is_kept_in_sight() {
    let dir = std::env::temp_dir().join(format!("trindle-files-seek-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for i in 0..120 {
        std::fs::write(dir.join(format!("file {i:03}")), "").unwrap();
    }
    let mut f = Files::new(vec![dir.clone()]);
    f.go(0, Where::Folder(dir.clone()));
    f.view = View::List;
    let _ = f.view();
    assert!(f.view().newest_seek(&mut Vec::new()).is_none());
    for _ in 0..80 {
        f.update(Msg::Step(Step::Down, false));
    }
    assert_eq!(f.cols[0].tabs[0].chosen, Some(79));
    let turn = f.cols[0].tabs[0].seek;
    assert!(turn > 0);
    let (got, top, tall, _) = f.view().newest_seek(&mut Vec::new()).unwrap();
    let h = f.cols[0].tabs[0].grid.get().h;
    assert_eq!((got, tall), (turn, h));
    assert_eq!(top, (79 % crate::files::view::LINES_PER_STRIP) as f32 * h + crate::files::icons::LINE_GAP);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn linked_panes_go_back_and_forward_together() {
    let dir = std::env::temp_dir().join(format!("trindle-files-linked-back-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for d in ["l/photos/2026", "r/photos/2026"] {
        std::fs::create_dir_all(dir.join(d)).unwrap();
    }
    let (l, r) = (dir.join("l"), dir.join("r"));
    let mut f = Files::new(vec![l.clone()]);
    f.go(1, Where::Folder(r.clone()));
    f.go(0, Where::Folder(l.clone()));
    f.update(Msg::Link);
    let at = |f: &Files, p: usize| f.cols[p].tabs[0].at.clone();
    let find = |f: &Files, name: &str| f.cols[0].tabs[0].entries.iter().position(|e| e.name == name).unwrap();
    f.open(0, find(&f, "photos"));
    f.open(0, find(&f, "2026"));
    assert_eq!(at(&f, 1), Where::Folder(r.join("photos/2026")));
    f.update(Msg::Back);
    assert_eq!((at(&f, 0), at(&f, 1)), (Where::Folder(l.join("photos")), Where::Folder(r.join("photos"))));
    f.update(Msg::Back);
    assert_eq!((at(&f, 0), at(&f, 1)), (Where::Folder(l.clone()), Where::Folder(r.clone())));
    f.update(Msg::Forward);
    assert_eq!((at(&f, 0), at(&f, 1)), (Where::Folder(l.join("photos")), Where::Folder(r.join("photos"))));
    assert_eq!(f.cols[1].tabs[0].ahead.last(), Some(&Where::Folder(r.join("photos/2026"))));
    f.update(Msg::Link);
    f.cols[0].tabs[0].width.set(400.0);
    let x = crate::files::act::panes_left() + 400.0 + crate::files::act::between() + 50.0;
    let back = Input::Press { button: Button::Back, x, y: 200.0 };
    let msg = f.intercept_with(&back, Keyboard::default());
    assert!(matches!(msg, Some(Msg::Travel(1, false))));
    f.update(msg.unwrap());
    assert_eq!((at(&f, 0), at(&f, 1)), (Where::Folder(l.join("photos")), Where::Folder(r.clone())));
    assert_eq!(f.side(), 1);
    std::fs::remove_dir_all(&dir).unwrap();
}
