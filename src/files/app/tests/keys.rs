//! The keys Files answers, and what each does.

use std::time::Duration;

use super::*;

const NONE: u8 = 0;
const CTRL: u8 = 1;
const SHIFT: u8 = 2;
const ALT: u8 = 4;

type Row = (&'static [Keysym], u8, &'static str);

const MOVING_KEYS: &[Row] = &[
    (&[Keysym::Return, Keysym::KP_Enter], ALT, "properties"),
    (&[Keysym::d, Keysym::D], ALT, "type a place"),
    (&[Keysym::p, Keysym::P], ALT, "preview panel"),
    (&[Keysym::l, Keysym::L], CTRL, "type a place"),
    (&[Keysym::n, Keysym::N], CTRL, "new window"),
    (&[Keysym::t, Keysym::T], CTRL, "new tab"),
    (&[Keysym::w, Keysym::W], CTRL, "close tab"),
    (&[Keysym::Tab], CTRL, "next tab"),
    (&[Keysym::Tab, Keysym::ISO_Left_Tab], CTRL | SHIFT, "tab before"),
    (&[Keysym::ISO_Left_Tab], CTRL, "tab before"),
    (&[Keysym::f, Keysym::F], CTRL, "find"),
    (&[Keysym::Left, Keysym::KP_Left], ALT, "back"),
    (&[Keysym::Right, Keysym::KP_Right], ALT, "forward"),
    (&[Keysym::Up, Keysym::KP_Up], ALT, "up"),
    (&[Keysym::BackSpace], NONE, "up"),
    (&[Keysym::BackSpace], SHIFT, "up"),
    (&[Keysym::F5], NONE, "reload"),
    (&[Keysym::F5], SHIFT, "reload"),
    (&[Keysym::F2], NONE, "rename"),
    (&[Keysym::F2], SHIFT, "rename"),
    (&[Keysym::Delete, Keysym::KP_Delete], NONE, "trash"),
    (&[Keysym::Delete, Keysym::KP_Delete], SHIFT, "delete"),
    (&[Keysym::n, Keysym::N], CTRL | SHIFT, "new folder"),
    (&[Keysym::a, Keysym::A], CTRL, "select all"),
    (&[Keysym::a, Keysym::A], CTRL | SHIFT, "select none"),
    (&[Keysym::i, Keysym::I], CTRL, "invert"),
];

const OWN_KEYS: &[Row] = &[
    (&[Keysym::F3], NONE, "panes"),
    (&[Keysym::F3], SHIFT, "panes"),
    (&[Keysym::F4], NONE, "terminal"),
    (&[Keysym::F4], SHIFT, "terminal"),
    (&[Keysym::F6], NONE, "other pane"),
    (&[Keysym::F6], SHIFT, "other pane"),
    (&[Keysym::Home, Keysym::KP_Home], ALT, "home"),
    (&[Keysym::Page_Down, Keysym::KP_Page_Down], CTRL, "next tab"),
    (&[Keysym::Page_Up, Keysym::KP_Page_Up], CTRL, "tab before"),
    (&[Keysym::_1, Keysym::KP_1], CTRL, "view Icons"),
    (&[Keysym::_2, Keysym::KP_2], CTRL, "view List"),
    (&[Keysym::_3, Keysym::KP_3], CTRL, "view Compact"),
    (&[Keysym::r, Keysym::R], CTRL, "reload"),
    (&[Keysym::d, Keysym::D], CTRL, "pin"),
    (&[Keysym::h, Keysym::H], CTRL, "hidden"),
    (&[Keysym::x, Keysym::X], CTRL, "cut"),
    (&[Keysym::c, Keysym::C], CTRL, "copy"),
    (&[Keysym::v, Keysym::V], CTRL, "paste"),
    (&[Keysym::z, Keysym::Z], CTRL, "undo"),
    (&[Keysym::z, Keysym::Z], CTRL | SHIFT, "redo"),
    (&[Keysym::y, Keysym::Y], CTRL, "redo"),
    (&[Keysym::plus, Keysym::equal, Keysym::KP_Add], CTRL, "bigger"),
    (&[Keysym::plus, Keysym::equal, Keysym::KP_Add], CTRL | SHIFT, "bigger"),
    (&[Keysym::minus, Keysym::KP_Subtract], CTRL, "smaller"),
    (&[Keysym::_0, Keysym::KP_0], CTRL, "usual size"),
    (&[Keysym::comma], CTRL, "settings"),
    (&[Keysym::Return, Keysym::KP_Enter], NONE, "open"),
    (&[Keysym::Return, Keysym::KP_Enter], SHIFT, "open"),
    (&[Keysym::space], NONE, "preview"),
    (&[Keysym::space], SHIFT, "preview"),
    (&[Keysym::Left, Keysym::KP_Left], NONE, "step Left false"),
    (&[Keysym::Right, Keysym::KP_Right], NONE, "step Right false"),
    (&[Keysym::Up, Keysym::KP_Up], NONE, "step Up false"),
    (&[Keysym::Down, Keysym::KP_Down], NONE, "step Down false"),
    (&[Keysym::Home, Keysym::KP_Home], NONE, "step First false"),
    (&[Keysym::End, Keysym::KP_End], NONE, "step Last false"),
    (&[Keysym::Left, Keysym::KP_Left], SHIFT, "step Left true"),
    (&[Keysym::Right, Keysym::KP_Right], SHIFT, "step Right true"),
    (&[Keysym::Up, Keysym::KP_Up], SHIFT, "step Up true"),
    (&[Keysym::Down, Keysym::KP_Down], SHIFT, "step Down true"),
    (&[Keysym::Home, Keysym::KP_Home], SHIFT, "step First true"),
    (&[Keysym::End, Keysym::KP_End], SHIFT, "step Last true"),
];

fn table() -> impl Iterator<Item = &'static Row> {
    MOVING_KEYS.iter().chain(OWN_KEYS)
}

#[test]
fn every_key_does_one_thing_and_the_table_says_which() {
    let mut known: std::collections::HashMap<(u32, u8), &str> = std::collections::HashMap::new();
    for (syms, mods, what) in table() {
        for sym in *syms {
            assert!(known.insert((sym.raw(), *mods), what).is_none(), "{sym:?} with {mods} is in the table twice");
        }
    }
    let mut all: Vec<Keysym> = table().flat_map(|(syms, ..)| syms.iter().copied()).collect();
    all.extend((0x20u32..0x7f).map(Keysym::new));
    all.extend([Keysym::F1, Keysym::F7, Keysym::F8, Keysym::F9, Keysym::F10, Keysym::F11, Keysym::F12, Keysym::Escape, Keysym::Insert,
                Keysym::Page_Up, Keysym::Page_Down, Keysym::KP_Page_Up, Keysym::KP_Page_Down]);
    let at = Keyboard::default();
    for sym in all {
        for mods in 0..8u8 {
            let held = Mods { ctrl: mods & CTRL != 0, shift: mods & SHIFT != 0, alt: mods & ALT != 0, logo: false };
            let got = key(sym, held, at).map(|m| does(&m));
            assert_eq!(got.as_deref(), known.get(&(sym.raw(), mods)).copied(), "{sym:?} with Ctrl {} Shift {} Alt {}", held.ctrl, held.shift, held.alt);
        }
    }
    a_typing_box_and_a_button_keep_their_own_keys();
}

fn a_typing_box_and_a_button_keep_their_own_keys() {
    let typing = Keyboard { keys: false, text: true, arrows: true };
    let ctrl = Mods { ctrl: true, ..Mods::default() };
    for sym in [Keysym::a, Keysym::x, Keysym::c, Keysym::v, Keysym::z, Keysym::y] {
        assert!(key(sym, ctrl, typing).is_none(), "{sym:?}");
    }
    for sym in [Keysym::BackSpace, Keysym::Delete, Keysym::Left, Keysym::Return, Keysym::space] {
        assert!(key(sym, Mods::default(), typing).is_none(), "{sym:?}");
    }
    let on_button = Keyboard { keys: true, text: false, arrows: false };
    for sym in [Keysym::Left, Keysym::Down, Keysym::Return, Keysym::space, Keysym::Home] {
        assert!(key(sym, Mods::default(), on_button).is_none(), "{sym:?}");
    }
    assert_eq!(key(Keysym::t, ctrl, typing).map(|m| does(&m)).as_deref(), Some("new tab"));
    assert_eq!(key(Keysym::F2, Mods::default(), on_button).map(|m| does(&m)).as_deref(), Some("rename"));
}

#[test]
fn f3_is_the_panes_switch() {
    let f = Files::new(Vec::new());
    let input = Input::Key { sym: Keysym::F3, text: None, mods: Mods::default(), repeat: false };
    assert!(matches!(f.intercept_with(&input, Keyboard::default()), Some(Msg::Panes)));
}

#[test]
fn the_usual_file_manager_keys_are_known() {
    let none = Mods::default();
    let ctrl = Mods { ctrl: true, ..Mods::default() };
    let alt = Mods { alt: true, ..Mods::default() };
    let at = Keyboard::default();
    assert!(matches!(key(Keysym::Left, alt, at), Some(Msg::Back)));
    assert!(matches!(key(Keysym::Right, alt, at), Some(Msg::Forward)));
    assert!(matches!(key(Keysym::l, ctrl, at), Some(Msg::Locate)));
    assert!(matches!(key(Keysym::F5, none, at), Some(Msg::Reload)));
    assert!(matches!(key(Keysym::F4, none, at), Some(Msg::Terminal)));
    assert!(matches!(key(Keysym::r, ctrl, at), Some(Msg::Reload)));
    assert!(matches!(key(Keysym::Tab, ctrl, at), Some(Msg::NextTab(true))));
    assert!(matches!(key(Keysym::F6, none, at), Some(Msg::OtherPane)));
    let ctrl_shift = Mods { ctrl: true, shift: true, ..Mods::default() };
    assert!(matches!(key(Keysym::z, ctrl, at), Some(Msg::Undo)));
    assert!(matches!(key(Keysym::y, ctrl, at), Some(Msg::Redo)));
    assert!(matches!(key(Keysym::Z, ctrl_shift, at), Some(Msg::Redo)));
    let typing = Keyboard { keys: false, text: true, arrows: true };
    assert!(key(Keysym::y, ctrl, typing).is_none() && key(Keysym::Z, ctrl_shift, typing).is_none());
}

fn does(m: &Msg) -> String {
    use crate::files::found::FindMsg;
    use crate::files::preview::PreviewMsg;
    use crate::files::zoom::Zoom;
    match m {
        Msg::Props => "properties".into(),
        Msg::Locate => "type a place".into(),
        Msg::Panel(crate::files::panel::PanelMsg::Switch) => "preview panel".into(),
        Msg::NewWindow => "new window".into(),
        Msg::NewTab(None) => "new tab".into(),
        Msg::CloseTab(None) => "close tab".into(),
        Msg::NextTab(true) => "next tab".into(),
        Msg::NextTab(false) => "tab before".into(),
        Msg::Find(FindMsg::Open(None)) => "find".into(),
        Msg::Back => "back".into(),
        Msg::Forward => "forward".into(),
        Msg::Up => "up".into(),
        Msg::Home => "home".into(),
        Msg::Reload => "reload".into(),
        Msg::GiveUp => "cancel".into(),
        Msg::Rename => "rename".into(),
        Msg::Trash => "trash".into(),
        Msg::Delete => "delete".into(),
        Msg::NewFolder => "new folder".into(),
        Msg::All => "select all".into(),
        Msg::Unpick => "select none".into(),
        Msg::Invert => "invert".into(),
        Msg::Panes => "panes".into(),
        Msg::Terminal => "terminal".into(),
        Msg::OtherPane => "other pane".into(),
        Msg::Pin => "pin".into(),
        Msg::Hidden => "hidden".into(),
        Msg::Cut => "cut".into(),
        Msg::CopyClip => "copy".into(),
        Msg::Paste => "paste".into(),
        Msg::Undo => "undo".into(),
        Msg::Redo => "redo".into(),
        Msg::Enter => "open".into(),
        Msg::Look(v) => format!("view {v:?}"),
        Msg::Zoom(Zoom::In, None) => "bigger".into(),
        Msg::Zoom(Zoom::Out, None) => "smaller".into(),
        Msg::Zoom(Zoom::Usual, None) => "usual size".into(),
        Msg::Step(s, run) => format!("step {s:?} {run}"),
        Msg::Preview(PreviewMsg::Toggle) => "preview".into(),
        Msg::Own(crate::files::settings::OwnMsg::Open) => "settings".into(),
        _ => "something not in the table".into(),
    }
}

#[test]
fn the_tab_and_window_keys_do_what_they_say() {
    let (dir, mut f) = two_panes("tab-keys");
    f.update(Msg::PanesAre(false));
    f.side = 0;
    let ctrl = Mods { ctrl: true, ..Mods::default() };
    let ctrl_shift = Mods { ctrl: true, shift: true, ..Mods::default() };
    let press = |f: &mut Files, sym: Keysym, mods: Mods| {
        let m = f.intercept(&Input::Key { sym, text: None, mods, repeat: false }, false).expect("the key is known");
        f.update(m);
    };
    press(&mut f, Keysym::t, ctrl);
    press(&mut f, Keysym::t, ctrl);
    assert_eq!((f.cols[0].tabs.len(), f.cols[0].tab), (3, 2));
    press(&mut f, Keysym::Tab, ctrl);
    assert_eq!(f.cols[0].tab, 0, "Ctrl and Tab goes round");
    press(&mut f, Keysym::ISO_Left_Tab, ctrl_shift);
    assert_eq!(f.cols[0].tab, 2);
    press(&mut f, Keysym::w, ctrl);
    press(&mut f, Keysym::w, ctrl);
    assert_eq!((f.cols[0].tabs.len(), f.closes), (1, 0));
    press(&mut f, Keysym::w, ctrl);
    assert_eq!((f.cols[0].tabs.len(), f.closes), (1, 1));
    f.update(Msg::PanesAre(true));
    f.update(Msg::NewTab(Some(1)));
    f.side = 0;
    press(&mut f, Keysym::w, ctrl);
    assert_eq!(f.closes, 1);
    assert!(f.said.contains("only tab"), "{}", f.said);
    f.side = 1;
    press(&mut f, Keysym::w, ctrl);
    f.side = 0;
    press(&mut f, Keysym::w, ctrl);
    assert_eq!(f.closes, 2, "each pane on its last tab: the window closes");
    press(&mut f, Keysym::d, Mods { alt: true, ..Mods::default() });
    assert!(f.locate.is_some());
    f.update(Msg::LocateStop);
    press(&mut f, Keysym::l, ctrl);
    assert!(f.locate.is_some());
    f.update(Msg::LocateStop);
    assert!(matches!(key(Keysym::F4, Mods::default(), Keyboard::default()), Some(Msg::Terminal)));
    press(&mut f, Keysym::n, ctrl);
    assert!(!f.bad);
    let _ = std::fs::remove_dir_all(dir);
}

fn chosen(f: &Files, p: usize) -> Option<String> {
    f.cols[p].chosen.map(|i| f.cols[p].entries[i].name.clone())
}

#[test]
fn typing_letters_jumps_to_a_name_in_every_view_pane_and_tab() {
    let (dir, mut f) = two_panes("jump");
    for n in ["banjo.txt", "\u{c9}clair.txt"] {
        std::fs::write(dir.join(n), b"x").unwrap();
    }
    f.update(Msg::NewTab(Some(0)));
    f.update(Msg::NewTab(Some(1)));
    f.reload_all();
    let mut t = Instant::now();
    for view in VIEWS {
        f.view = view;
        for p in 0..2 {
            for tab in 0..2 {
                letters_jump_in(&mut f, &format!("{view:?}, pane {p}, tab {tab}"), (p, tab), &mut t);
            }
        }
    }
    the_other_keys_keep_their_work(&mut f);
    std::fs::remove_dir_all(&dir).unwrap();
}

fn letters_jump_in(f: &mut Files, what: &str, (p, tab): (usize, usize), t: &mut Instant) {
    let ms = Duration::from_millis;
    f.update(Msg::Tab(p, tab));
    f.update(Msg::Unpick);
    f.cols[p].chosen = None;
    *t += Duration::from_secs(10);
    f.typed_at("b", *t);
    assert_eq!(chosen(f, p).as_deref(), Some("Banana.txt"), "{what}");
    f.typed_at("a", *t + ms(200));
    f.typed_at("n", *t + ms(400));
    assert_eq!(chosen(f, p).as_deref(), Some("Banana.txt"), "{what}");
    f.typed_at("j", *t + ms(600));
    assert_eq!(chosen(f, p).as_deref(), Some("banjo.txt"), "{what}");
    assert_eq!(f.cols[p].picked.len(), 1, "{what}");
    assert!(f.cols[p].seek > 0, "{what}");
    *t += Duration::from_secs(10);
    f.typed_at("b", *t);
    assert_eq!(chosen(f, p).as_deref(), Some("Banana.txt"), "{what}");
    f.typed_at("b", *t + ms(200));
    assert_eq!(chosen(f, p).as_deref(), Some("banjo.txt"), "{what}");
    *t += Duration::from_secs(10);
    f.typed_at("\u{e9}c", *t);
    assert_eq!(chosen(f, p).as_deref(), Some("\u{c9}clair.txt"), "{what}");
    *t += Duration::from_secs(10);
    f.typed_at("ec", *t);
    assert_eq!(chosen(f, p).as_deref(), Some("\u{c9}clair.txt"), "{what}");
    *t += Duration::from_secs(10);
    f.typed_at("z", *t);
    assert_eq!(chosen(f, p).as_deref(), Some("\u{c9}clair.txt"), "{what}");
}

fn the_other_keys_keep_their_work(f: &mut Files) {
    f.ahead = Default::default();
    let none = Mods::default();
    let ctrl = Mods { ctrl: true, ..Mods::default() };
    let k = |sym, text: &str, mods| Input::Key { sym, text: (!text.is_empty()).then(|| text.to_string()), mods, repeat: false };
    let list = Keyboard::default();
    assert!(matches!(f.intercept_with(&k(Keysym::c, "c", none), list), Some(Msg::Typed(t)) if t == "c"));
    assert!(matches!(f.intercept_with(&k(Keysym::f, "f", ctrl), list), Some(Msg::Find(_))));
    assert!(matches!(f.intercept_with(&k(Keysym::F3, "", none), list), Some(Msg::Panes)));
    assert!(matches!(f.intercept_with(&k(Keysym::BackSpace, "\u{8}", none), list), Some(Msg::Up)));
    assert!(matches!(f.intercept_with(&k(Keysym::Return, "\r", none), list), Some(Msg::Enter)));
    assert!(matches!(f.intercept_with(&k(Keysym::space, " ", none), list), Some(Msg::Preview(_))));
    f.update(Msg::Typed("c".into()));
    assert!(matches!(f.intercept_with(&k(Keysym::space, " ", none), list), Some(Msg::Typed(_))));
    for at in [Keyboard { keys: false, text: true, arrows: true }, Keyboard { keys: false, text: false, arrows: true },
               Keyboard { keys: true, text: false, arrows: false }] {
        assert!(f.intercept_with(&k(Keysym::c, "c", none), at).is_none());
    }
    f.update(Msg::Find(crate::files::found::FindMsg::Open(None)));
    assert!(f.intercept_with(&k(Keysym::c, "c", none), Keyboard { keys: false, text: true, arrows: true }).is_none());
}
