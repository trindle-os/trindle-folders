// Where Files starts, and the switch between one pane and two.

use super::*;

#[test]
fn with_nothing_kept_one_pane_shows_home_and_two_show_home_and_recent() {
    let home = scratch("fresh");
    let start = crate::files::kept::start_from(&home, "", [String::new(), String::new()], &[]);
    let f = Files::with(Vec::new(), home.clone(), start, crate::files::kept::Kept::default(), Vec::new());
    assert!(!f.two, "one pane by default");
    assert_eq!(f.cols[0].tabs[0].at, Where::Folder(home.clone()));
    assert_eq!(f.cols[1].tabs[0].at, Where::Recent);
    assert_eq!(f.side(), 0);
    assert!(f.shown_pane(0) && !f.shown_pane(1));
    std::fs::remove_dir_all(&home).unwrap();
}

#[test]
fn a_places_the_person_set_beat_the_defaults() {
    use crate::files::kept::start_from;
    let home = scratch("set");
    std::fs::create_dir_all(home.join("Downloads")).unwrap();
    let chosen = ["~/Downloads".to_string(), "~".to_string()];
    assert_eq!(start_from(&home, "chosen", chosen.clone(), &[]), (Where::Folder(home.join("Downloads")), Where::Folder(home.clone())));
    assert_eq!(start_from(&home, "chosen", [String::new(), "~/Gone".into()], &[]), (Where::Folder(home.clone()), Where::Recent));
    let last = vec![(PathBuf::from("trash:"), "left".to_string()), (PathBuf::from("recent:"), "right".to_string())];
    assert_eq!(start_from(&home, "last", chosen, &last), (Where::Trash, Where::Recent));
    assert_eq!(start_from(&home, "last", [String::new(), String::new()], &[]), (Where::Folder(home.clone()), Where::Recent));
    std::fs::remove_dir_all(&home).unwrap();
}

#[test]
fn the_single_and_dual_strip_switches_at_once_without_reading_again_and_is_kept() {
    let dir = scratch("panes");
    let conf = dir.join("conf");
    std::fs::write(dir.join("a.txt"), "a").unwrap();
    let start = (Where::Folder(dir.clone()), Where::Recent);
    let kept = crate::files::kept::Kept::from(Some(conf.clone()));
    assert!(!kept.dual, "nothing kept: single");
    let mut f = Files::with(Vec::new(), dir.clone(), start, kept, Vec::new());
    assert!(!f.two);
    std::fs::write(dir.join("b.txt"), "b").unwrap();
    let before: Vec<String> = f.cols[0].tabs[0].entries.iter().map(|e| e.name.clone()).collect();
    f.update(Msg::PanesAre(true));
    assert!(f.two && f.shown_pane(0) && f.shown_pane(1));
    assert_eq!(f.cols[0].tabs[0].entries.iter().map(|e| e.name.clone()).collect::<Vec<_>>(), before);
    assert!(crate::files::kept::Kept::from(Some(conf.clone())).dual, "kept for next time");
    f.update(Msg::PanesAre(true));
    assert!(f.two, "saying dual again changes nothing");
    f.update(Msg::Panes);
    assert!(!f.two);
    assert!(!crate::files::kept::Kept::from(Some(conf.clone())).dual);
    std::fs::write(conf.join(crate::files::kept::PANES), "sideways\n").unwrap();
    assert!(!crate::files::kept::Kept::from(Some(conf)).dual, "an unknown word is single");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn starts_on_recent_and_home() {
    let f = Files::new(Vec::new());
    assert_eq!(f.cols[0].tabs.len(), 1);
    assert_eq!(f.cols[0].tabs[0].at, Where::Recent);
    assert_eq!(f.cols[1].tabs[0].at, Where::Folder(home()));
}

#[test]
fn each_pane_has_its_own_tabs() {
    let dir = scratch("tabs");
    for d in ["a", "b", "c"] {
        std::fs::create_dir_all(dir.join(d)).unwrap();
    }
    let start = (Where::Folder(dir.join("a")), Where::Folder(dir.join("b")));
    let mut kept = crate::files::kept::Kept::default();
    kept.dual = true;
    let mut f = Files::with(Vec::new(), dir.clone(), start, kept, Vec::new());
    assert_eq!((f.cols[0].tabs.len(), f.cols[1].tabs.len()), (1, 1), "a strip with one tab each");
    f.update(Msg::NewTab(Some(0)));
    f.update(Msg::NewTab(Some(0)));
    assert_eq!((f.cols[0].tabs.len(), f.cols[1].tabs.len()), (3, 1));
    assert_eq!(f.cols[0].tab, 2);
    assert_eq!(f.cols[0].tabs[2].at, Where::Folder(dir.join("a")));
    f.go(0, Where::Folder(dir.join("c")));
    assert_eq!(f.cols[0].tabs[1].at, Where::Folder(dir.join("a")));
    assert_eq!(f.cols[1].at, Where::Folder(dir.join("b")));
    f.update(Msg::Tab(1, 0));
    f.update(Msg::Tab(0, 0));
    assert_eq!((f.cols[0].tab, f.cols[1].tab, f.side()), (0, 0, 0));
    f.update(Msg::OtherPane);
    f.update(Msg::NewTab(None));
    assert_eq!((f.cols[0].tabs.len(), f.cols[1].tabs.len(), f.side()), (3, 2, 1));
    f.update(Msg::CloseTab(None));
    assert_eq!(f.cols[1].tabs.len(), 1);
    f.update(Msg::CloseTab(Some(1)));
    assert_eq!(f.cols[1].tabs.len(), 1);
    f.update(Msg::Tab(0, 2));
    f.update(Msg::CloseTab(Some(0)));
    assert_eq!((f.cols[0].tabs.len(), f.cols[0].tab), (2, 1));
    f.update(Msg::NextTab(true));
    assert_eq!(f.cols[0].tab, 0);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_folder_opened_in_a_new_tab_goes_in_the_pane_it_was_opened_from() {
    let dir = scratch("opentab");
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    let start = (Where::Folder(dir.clone()), Where::Folder(dir.clone()));
    let mut kept = crate::files::kept::Kept::default();
    kept.dual = true;
    let mut f = Files::with(Vec::new(), dir.clone(), start, kept, Vec::new());
    f.do_act(crate::files::menu::Act::OpenTab, 1, vec![dir.join("sub")]);
    assert_eq!((f.cols[0].tabs.len(), f.cols[1].tabs.len()), (1, 2));
    assert_eq!(f.cols[1].at, Where::Folder(dir.join("sub")));
    assert_eq!(f.side(), 1);
    std::fs::remove_dir_all(&dir).unwrap();
}
