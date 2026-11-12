//! Tests of how the Files window is laid out.

use super::*;

#[test]
fn deleting_for_good_is_the_red_button() {
    assert_eq!(confirm(1, false).for_good(), ["Delete permanently"]);
    assert_eq!(confirm(3, false).for_good(), ["Delete permanently"]);
    assert_eq!(confirm(3, true).for_good(), ["Empty the Trash"]);
}

#[test]
fn a_long_path_gives_way_in_a_narrow_pane() {
    let c: Vec<String> = ["Home", "mnt", "Photos on NAS", "2024", "Holiday"].iter().map(|s| s.to_string()).collect();
    assert_eq!(fit_crumbs(c.clone(), 2000.0), (c.clone(), 0));
    let (shown, skipped) = fit_crumbs(c.clone(), 230.0);
    assert!(skipped > 0 && shown[0] == "\u{2026}" && shown.last() == c.last());
    assert!(shown.iter().map(|s| pill_guess(s)).sum::<f32>() <= 230.0);
    let (tiny, _) = fit_crumbs(c.clone(), 10.0);
    assert_eq!(tiny.last(), c.last());
}

#[test]
fn an_empty_pane_scrolls_in_the_narrowest_two_pane_window() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};
    let home = std::env::temp_dir().join(format!("trindle-empty-pane-{}", std::process::id()));
    std::fs::create_dir_all(&home).unwrap();
    let mut kept = super::super::kept::Kept::default();
    kept.dual = true;
    let mut f = Files::with(Vec::new(), home.clone(), (Where::Network, Where::Network), kept, Vec::new());
    f.set_density(super::super::density::Density::Roomy);
    let w = (super::super::panel::least_window(true) - super::super::panel::LEAST).ceil();
    let mut with_button = 0;
    let heights = [700.0, 520.0, 420.0, 360.0, 320.0, 280.0, 240.0];
    for h in heights {
        f.resized(w, h);
        let said = view(&f).said();
        assert!(said.contains(&f.near.words()) && said.contains(&"button: Connect to a server".to_string()), "{said:?}");
        let mut ui = Ui::new(view(&f), Theme::new(true, "#fbc711"));
        ui.lay_at(w as u32, h as u32);
        let mut ui = Ui::new(view(&f), Theme::new(true, "#fbc711"));
        ui.lay_at(w as u32, h as u32);
        for p in 0..2 {
            let name = empty_area(p, f.cols[p].tab, &Where::Network);
            let places = ui.places();
            let (_, _, most) = places.iter().find(|a| a.0 == name).cloned().unwrap_or_else(|| panic!("{name} is a scrolling area at {h}: {places:?}"));
            assert!(!places.iter().any(|a| a.0.starts_with(&format!("pane{p}:"))), "no other area in pane {p} at {h}: {places:?}");
            ui.to_the_ends();
            let places = ui.places();
            let at = places.iter().find(|a| a.0 == name).unwrap();
            assert_eq!(at.1, at.2, "pane {p} reaches its end at {h}");
            let boxes = ui.boxes();
            let area = boxes.iter().find(|b| b.0 == name).map(|b| b.1).unwrap();
            let inside: Vec<_> = boxes.iter().filter(|b| b.0 != name && b.2.is_some_and(|c| c.x >= area.x - 0.5 && c.x + c.w <= area.x + area.w + 0.5
                && c.y >= area.y - 0.5 && c.y + c.h <= area.y + area.h + 0.5)).collect();
            let buttons: Vec<String> = ui.pressables().into_iter().filter(|k| k.1 == "button").map(|k| k.0).collect();
            let foot = inside.iter().map(|b| b.1.y + b.1.h).fold(0.0f32, f32::max);
            assert!(foot >= area.y + area.h - 1.5, "the strip reaches the foot of pane {p} at {h}: {foot} in {area:?}");
            if let Some(button) = inside.iter().find(|b| buttons.contains(&b.0)) {
                with_button += 1;
                assert!(button.1.y >= area.y - 0.5 && button.1.y + button.1.h <= area.y + area.h + 0.5, "the button shows whole in pane {p} at {h}: {:?} in {area:?}", button.1);
                assert!(foot > button.1.y + button.1.h + 1.0, "room to right-click under the button in pane {p} at {h}: {foot} in {area:?}");
            }
            assert!(most == 0.0 || most >= 1.0);
        }
    }
    assert_eq!(with_button, 2 * heights.len(), "both panes have the button at every height, as a press in either makes it the one in use");
    assert_ne!(empty_area(0, 0, &Where::Network), empty_area(1, 0, &Where::Network));
    assert_ne!(empty_area(0, 0, &Where::Network), empty_area(0, 0, &Where::Trash));
    std::fs::remove_dir_all(&home).unwrap();
}

#[test]
fn the_two_panes_are_always_the_same_width() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};
    let home = std::env::temp_dir().join(format!("trindle-equal-panes-{}", std::process::id()));
    let long = "a".repeat(120);
    std::fs::create_dir_all(home.join(&long)).unwrap();
    std::fs::write(home.join(format!("{long}.txt")), b"x").unwrap();
    let mut kept = super::super::kept::Kept::default();
    kept.dual = true;
    let mut f = Files::with(Vec::new(), home.clone(), (Where::Folder(home.clone()), Where::Network), kept, Vec::new());
    let least = super::super::panel::least_window(true) - super::super::panel::LEAST;
    let mut tried = 0;
    for panel in [false, true] {
        f.kept.panel = panel;
        for w in [least.ceil(), 900.0, 1001.0, 1337.0, 1920.0] {
            f.resized(w, 600.0);
            let mut widths: Vec<(char, f32)> = Vec::new();
            for _ in 0..2 {
                let mut ui = Ui::new(view(&f), Theme::new(true, "#fbc711"));
                ui.lay_at(w as u32, 600);
                widths = ui.boxes().iter()
                    .filter_map(|b| {
                        let rest = b.0.strip_prefix("pane").or_else(|| b.0.strip_prefix("empty"))?;
                        let side = rest.chars().next().filter(char::is_ascii_digit)?;
                        Some((side, b.2.map_or(b.1.w, |c| c.w.min(b.1.w))))
                    })
                    .collect();
            }
            let of = |side: char| widths.iter().filter(|x| x.0 == side).map(|x| x.1).fold(0.0f32, f32::max);
            assert!(of('0') > 0.0 && of('1') > 0.0, "both panes are drawn at {w}: {widths:?}");
            assert!((of('0') - of('1')).abs() < 0.5, "the panes are the same width at {w} with the panel {panel}: {widths:?}");
            tried += 1;
        }
    }
    assert_eq!(tried, 10);
    std::fs::remove_dir_all(&home).unwrap();
}

#[test]
fn the_top_bar_is_the_path_and_nine_buttons()  {
    let f = Files::new(Vec::new());
    let bar = toolbar(&f, f.pane(), f.side());
    let tips: Vec<String> = bar.items().iter().filter_map(|n| n.tip().map(str::to_string)).collect();
    assert_eq!(tips.len(), 9, "{tips:?}");
    assert!(tips.iter().any(|t| t.starts_with("Show hidden files")), "{tips:?}");
    use crate::toolkit::ui::App as _;
    let mut f = f;
    f.update(Msg::Hidden);
    let tips: Vec<String> = toolbar(&f, f.pane(), f.side()).items().iter().filter_map(|n| n.tip().map(str::to_string)).collect();
    assert!(tips.iter().any(|t| t.starts_with("Hide hidden files")), "{tips:?}");
    f.update(Msg::Hidden);
    let bar = toolbar(&f, f.pane(), f.side());
    assert!(bar.said().iter().any(|w| w.ends_with("New")), "and New, a worded button: {:?}", bar.said());
    assert!(tips.iter().all(|t| !t.starts_with("Undo") && !t.starts_with("Redo") && !t.starts_with("New folder") && !t.starts_with("Folders settings")), "{tips:?}");
}

#[test]
fn one_row_holds_the_buttons_and_the_path_and_search_takes_its_place() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};
    let dir = std::env::temp_dir().join(format!("trindle-files-one-row-{}", std::process::id()));
    let deep = dir.join("Albums/2025/Summer/Beach/Morning");
    std::fs::create_dir_all(&deep).unwrap();
    let mut f = Files::new(vec![deep.clone()]);
    f.update(Msg::PanesAre(false));
    f.home = dir.clone();
    let boxes = |f: &Files, w: f32| {
        let mut ui = Ui::new(view(f), Theme::new(true, "#fbc711"));
        ui.lay_at(w as u32, 700);
        (ui.pill_boxes("path0"), ui.boxes().iter().any(|b| b.0 == "find"), ui.boxes().iter().any(|b| b.0 == "locate"))
    };
    f.room = (1200.0, 700.0);
    assert_eq!(top_bar(&f).items().len(), 1);
    let said = view(&f).said().join("\n");
    assert!(!said.contains("Location"), "no switch between the location and search: {said}");
    let (names, finding, locating) = boxes(&f, 1200.0);
    assert!(names.len() >= 5 && !finding && !locating, "{names:?}");
    let row_top = names[0].y;
    assert!(names.iter().all(|n| (n.y - row_top).abs() < 1.0), "the names are on one row");
    f.update(Msg::PanesAre(true));
    assert_eq!(top_bar(&f).items().len(), 1);
    let alone = { f.update(Msg::PanesAre(false)); pane(&f, 0).items().len() };
    f.update(Msg::PanesAre(true));
    assert_eq!(pane(&f, 0).items().len(), alone, "the pane's own line, and the faint line under it, as with one pane");
    f.update(Msg::PanesAre(false));
    f.update(Msg::Find(FindMsg::Open(Some(0))));
    assert_eq!(top_bar(&f).items().len(), 2, "the row, then the kind, size and changed lists");
    let (names, finding, _) = boxes(&f, 1200.0);
    assert!(names.is_empty() && finding);
    f.update(Msg::Find(FindMsg::Open(Some(0))));
    assert!(f.find.open.is_none() && !boxes(&f, 1200.0).0.is_empty());
    f.update(Msg::Find(FindMsg::Open(Some(0))));
    f.update(Msg::Find(FindMsg::Close));
    assert!(!boxes(&f, 1200.0).0.is_empty());
    f.update(Msg::Find(FindMsg::Open(Some(0))));
    f.go(0, Where::Folder(dir.join("Albums")));
    assert!(f.find.open.is_none() && !boxes(&f, 1200.0).0.is_empty());
    f.update(Msg::Locate);
    let (names, _, locating) = boxes(&f, 1200.0);
    assert!(names.is_empty() && locating);
    f.update(Msg::LocateStop);
    f.go(0, Where::Folder(deep.clone()));
    f.room = (700.0, 700.0);
    let (narrow, _, _) = boxes(&f, 700.0);
    assert!(!narrow.is_empty() && narrow.len() < 6, "{} names", narrow.len());
    assert!(narrow.iter().all(|n| n.x + n.w <= 700.0), "within the window: {narrow:?}");
    f.room = (1200.0, 700.0);
    f.go(0, Where::Computer);
    let bar = toolbar(&f, f.pane(), 0);
    assert!(bar.items().iter().filter_map(|n| n.tip()).any(|t| t.contains("can't search on this page")));
    f.update(Msg::Find(FindMsg::Open(None)));
    assert!(f.find.open.is_none());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn new_and_more_open_their_menus_under_the_top_bar() {
    use crate::toolkit::ui::App as _;
    use super::super::menu::Act;
    let dir = std::env::temp_dir().join(format!("trindle-files-top-menus-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut f = Files::new(vec![dir.clone()]);
    f.update(Msg::Menu(MenuMsg::New));
    let acts = |f: &Files| f.menu.as_ref().map(|m| m.lines.iter().map(|l| l.act).collect::<Vec<_>>()).unwrap_or_default();
    assert_eq!(acts(&f), [Act::NewFolder, Act::NewDocument]);
    assert_eq!(f.menu.as_ref().unwrap().at.1, top_bar_height(&f));
    f.update(Msg::Menu(MenuMsg::More));
    assert_eq!(acts(&f), [Act::Settings], "nothing to undo or redo yet");
    f.menu = None;
    f.go(0, Where::Trash);
    f.update(Msg::Menu(MenuMsg::New));
    assert!(f.menu.is_none(), "nothing can be made in the Trash");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_areas_are_named_for_a_sheet_and_each_a_step_of_grey_in_trindles_own() {
    use crate::toolkit::ui::App as _;
    fn areas<M>(n: &Node<M>, out: &mut Vec<Area>) {
        out.extend(n.area.map(|a| a.0));
        for i in n.items() {
            areas(i, out);
        }
    }
    let mut f = Files::new(Vec::new());
    for two in [false, true] {
        f.update(Msg::PanesAre(two));
        let mut got = Vec::new();
        areas(&view(&f), &mut got);
        assert_eq!(got, vec![Area::Top], "two panes: {two}; the strip between them is on the window's own dark");
    }
    let t = crate::toolkit::theme::Theme::new(false, crate::common::style::DEFAULT_ACCENT);
    assert!(t.top_bar.a == 1.0 && t.top_bar != t.bg && t.top_bar != t.side_bar, "{:?} on {:?}", t.top_bar, t.bg);
    assert!(t.side_bar.a == 1.0 && t.side_bar != t.bg, "{:?} on {:?}", t.side_bar, t.bg);
    assert_eq!(t.area_line.a, 0.0, "no area has a line");
    assert!(t.foot.a == 1.0 && t.foot != t.panel, "{:?} on {:?}", t.foot, t.panel);
    assert!(t.top_bar.a == 1.0 && t.top_bar != t.panel, "{:?} on {:?}", t.top_bar, t.panel);
}

#[test]
fn a_pane_is_where_it_is_then_files_then_foot() {
    use crate::toolkit::ui::App as _;
    fn section<'a, M>(n: &'a Node<M>, which: Area, out: &mut Vec<&'a Node<M>>) {
        if matches!(n.area, Some((a, _)) if a == which) {
            out.push(n);
        }
        n.items().iter().for_each(|i| section(i, which, out));
    }
    let dir = std::env::temp_dir().join(format!("trindle-files-sections-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("Letter.txt"), "x").unwrap();
    let mut f = Files::new(vec![dir.clone()]);
    let mut alone = 0;
    for two in [false, true] {
        f.update(Msg::PanesAre(two));
        let tree = view(&f);
        let mut tops = Vec::new();
        section(&tree, Area::Top, &mut tops);
        let top = tops[0].said().join("\n");
        assert!(top.contains("Back") && top.contains("Forward") && top.contains("Up") && !top.contains("Letter.txt"), "the top bar: {top}");
        let mut shaded = Vec::new();
        section(&tree, Area::Controls, &mut shaded);
        assert!(shaded.is_empty(), "no strip of controls in a pane (two: {two})");
        let panes: Vec<usize> = if two { vec![0, 1] } else { vec![f.side()] };
        for p in panes {
            let card = pane(&f, p);
            let kids = card.items();
            assert!(kids.iter().all(|k| k.area.is_none()), "nothing in the card has a shade of its own (pane {p})");
            let first = 2;
            if !two {
                alone = kids.len();
            } else if p == f.side() {
                assert_eq!(kids.len(), alone, "with two as with one, the card starts with where it is and a line (pane {p})");
            }
            let list = kids[first..].iter().flat_map(|k| k.said()).collect::<Vec<_>>().join("\n");
            assert!(!list.contains("Back") && !list.contains("Forward") && !list.contains(" free"), "the list holds none of the controls, nor the foot: {list}");
            assert!(status_words(&f, &f.cols[p], p, f.side() == p).contains("item") && free_words(&f.cols[p]).is_some_and(|w| w.ends_with(" free")),
                    "the foot says how many items and the room left (pane {p})");
        }
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn two_panes_are_two_cards_with_the_strip_between() {
    use crate::toolkit::ui::App as _;
    fn cards<M>(n: &Node<M>, out: &mut Vec<(bool, bool)>) {
        let between = n.key.as_deref() == Some(BETWEEN);
        if n.current || between {
            out.push((n.current, between));
        }
        for i in n.items() {
            cards(i, out);
        }
    }
    fn strip_said<M>(n: &Node<M>) -> Option<Vec<String>> {
        if n.key.as_deref() == Some(BETWEEN) {
            return Some(n.said());
        }
        n.items().iter().find_map(strip_said)
    }
    let dir = std::env::temp_dir().join(format!("trindle-files-two-cards-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("Letters")).unwrap();
    let mut f = Files::new(vec![dir.clone()]);
    f.update(Msg::PanesAre(true));
    f.go(1, Where::Folder(dir.join("Letters")));
    let seen = |f: &Files| {
        let mut got = Vec::new();
        cards(&view(f), &mut got);
        got
    };
    f.side = 0;
    assert_eq!(seen(&f), [(true, false), (false, true)], "the left pane's card is marked, then the strip; the right card is not");
    assert_eq!(strip_said(&view(&f)).unwrap(), ["button: Copy to the right pane", "button: Move to the right pane",
                                                "button: Show this folder in the right pane", "button: Swap the panes",
                                                "button: Link the two panes", "button: Select what differs from the right pane",
                                                "button: Compare with the right pane"]);
    f.update(Msg::OtherPane);
    assert_eq!(seen(&f), [(false, true), (true, false)], "the mark follows the keyboard");
    assert_eq!(strip_said(&view(&f)).unwrap()[0], "button: Copy to the left pane");
    let folder = super::super::menu::Sel { folder: true, ..Default::default() };
    for sel in [&[][..], &[folder][..]] {
        for line in super::super::menu::lines_for(&f.cols[0].at, sel, false) {
            for word in ["Swap", "Show this folder", "Pick what differs", "Link the", "Compare with", "to the left pane", "to the right pane"] {
                assert!(!line.words.contains(word), "{} is in the menu too", line.words);
            }
        }
    }
    assert_eq!((strip(), strip_gap()), (42.0, theme::sheet().panel_gap / 2.0));
    assert_eq!(super::super::act::between(), 2.0 * theme::sheet().card_padding + 42.0 + theme::sheet().panel_gap);
    f.update(Msg::PanesAre(false));
    assert!(seen(&f).is_empty() && strip_said(&view(&f)).is_none(), "single view is one card, as it was");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn no_worded_buttons_words_leave_it_anywhere_in_folders() {
    use crate::toolkit::ui::{App as _, Ui};
    use super::super::density::Density;
    let dir = std::env::temp_dir().join(format!("trindle-files-worded-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Empty")).unwrap();
    std::fs::write(dir.join("notes.txt"), b"x").unwrap();
    type Make = fn(&mut Files, &std::path::Path);
    let screens: [(&str, Make); 6] = [
        ("Network, empty", |f, _| f.go(f.side(), Where::Network)),
        ("an empty folder", |f, d| f.go(f.side(), Where::Folder(d.join("Empty")))),
        ("Delete permanently?", |f, d| f.confirm = Some(vec![d.join("notes.txt")])),
        ("Empty the Trash?", |f, d| {
            f.confirm = Some(vec![d.join("notes.txt")]);
            f.emptying = true;
        }),
        ("Connect to a server", |f, _| f.update(Msg::Far(super::super::far::FarMsg::Open))),
        ("a locked archive", |f, d| f.unlock_ask(d.join("holiday photos.7z"), super::super::locked::Then::List, None)),
    ];
    let mut seen = 0;
    for (name, make) in screens {
        for level in Density::ALL {
            for (two, right) in [(false, false), (true, false), (true, true)] {
                for (w, h) in [(1200.0, 760.0), (900.0, 600.0), (if two { 820.0 } else { 560.0 }, 480.0)] {
                    for scale in [1.0, 2.0] {
                        let mut f = Files::new(vec![dir.clone()]);
                        f.set_density(level);
                        f.room = (w, h);
                        f.update(Msg::PanesAre(two));
                        f.side = usize::from(right);
                        make(&mut f, &dir);
                        f.side = usize::from(right);
                        let mut found = Vec::new();
                        for _ in 0..2 {
                            let mut ui = Ui::new(view(&f), theme::Theme::new(false, crate::common::style::DEFAULT_ACCENT));
                            let mut pix = tiny_skia::Pixmap::new((w * scale) as u32, (h * scale) as u32).unwrap();
                            ui.paint(&mut pix.as_mut(), scale);
                            found = ui.worded_buttons();
                        }
                        let at = format!("{name}, {level:?}, {} at {w} by {h}, scale {scale}", if two { "two panes" } else { "one pane" });
                        assert!(!found.is_empty(), "{at}: no worded button to check");
                        for (words, pill, text) in found {
                            seen += 1;
                            assert!(text.x >= pill.x && text.x + text.w <= pill.x + pill.w + 0.5, "{at}: {words:?} at {text:?} leaves its button {pill:?} sideways");
                            assert!(text.y >= pill.y - 0.5 && text.y + text.h <= pill.y + pill.h + 0.5, "{at}: {words:?} at {text:?} leaves its button {pill:?} up or down");
                            assert!(pill.x >= 0.0 && pill.x + pill.w <= w * scale && pill.y >= 0.0 && pill.y + pill.h <= h * scale, "{at}: {words:?} at {pill:?} is off the window");
                        }
                    }
                }
            }
        }
    }
    assert!(seen >= 6 * 3 * 3 * 3 * 2);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_files_row_does_not_say_its_kind_in_words() {
    use super::super::columns::Col;
    use super::super::density::Density;
    use crate::toolkit::ui::App as _;
    let dir = std::env::temp_dir().join(format!("trindle-files-no-kind-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Letters")).unwrap();
    std::fs::write(dir.join("notes.md"), b"made up").unwrap();
    std::fs::write(dir.join("letter.txt"), b"made up").unwrap();
    assert!(!Col::USUAL.contains(&Col::Type), "the Type column is not shown until it is chosen");
    let mut seen = 0;
    for level in Density::ALL {
        for at in [Where::Folder(dir.clone()), Where::Recent] {
            for view in [View::List, View::Icons, View::Compact] {
                for width in [1200.0, 620.0, 300.0] {
                    let mut f = Files::new(vec![dir.clone()]);
                    f.set_density(level);
                    f.update(Msg::PanesAre(false));
                    f.go(0, at.clone());
                    f.view = view;
                    let pane = &f.cols[0];
                    pane.width.set(width);
                    let what = format!("{level:?}, {at:?}, {view:?}, {width} wide");
                    assert!(!shown_columns(&f, pane, 0).contains(&Col::Type), "{what}: a Type column nobody chose");
                    if view != View::Icons {
                        let one_line = if view == View::List { f.room_now().row } else { f.room_now().line } + ROW_ROOM;
                        assert_eq!(grid_of(&f, pane, 0).h, one_line, "{what}: a row is one line");
                    }
                    seen += 1;
                }
            }
        }
    }
    assert_eq!(seen, 3 * 2 * 3 * 3);
    let mut f = Files::new(vec![dir.clone()]);
    f.update(Msg::PanesAre(false));
    f.go(0, Where::Folder(dir.clone()));
    f.view = View::List;
    f.kept.columns = Some(vec![Col::Size, Col::Type, Col::Date]);
    let pane = &f.cols[0];
    pane.width.set(1200.0);
    assert!(shown_columns(&f, pane, 0).contains(&Col::Type));
    assert_eq!(grid_of(&f, pane, 0).h, f.room_now().row + ROW_ROOM);
    let said = |name: &str| f.cache.cells(pane.entries.iter().find(|e| e.name == name).unwrap(), &[Col::Type]).concat();
    assert!(!said("notes.md").is_empty() && !said("letter.txt").is_empty(), "the column chosen says the kind");
    assert_eq!(said("Letters"), "");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_tab_name_is_cut_short_to_leave_room() {
    assert_eq!(short_name("Photos", 14), "Photos");
    assert_eq!(short_name("A very long folder name", 14).chars().count(), 14);
    assert!(short_name("A very long folder name", 14).ends_with('\u{2026}'));
}

#[test]
fn sizes_read_plainly() {
    assert_eq!(size_words(0), "0 bytes");
    assert_eq!(size_words(1), "1 byte");
    assert_eq!(size_words(999), "999 bytes");
    assert_eq!(size_words(4_200_000), "4.2 MB");
}

#[test]
fn sizes_follow_the_languages_marks_and_units() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    let made_up = "kit-number-decimal = ,\nkit-size-mb = { $size } Mo\nkit-size-bytes =\n    { $n ->\n        [one] { $n } oktetoj\n       *[other] { $n } oktetoj\n    }\n";
    with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
        assert_eq!(size_words(4_200_000), "4,2 Mo");
        assert_eq!(size_words(1), "1 oktetoj");
        assert_eq!(size_words(4_500), "4,5 KB");
    });
}
