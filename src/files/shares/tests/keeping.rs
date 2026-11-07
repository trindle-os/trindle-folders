// Logins kept and sent over several openings: when the card is asked, and when it is not.

use std::path::PathBuf;

use crate::files::app::{Files, Msg};
use crate::files::dialogs::Dialog;
use crate::files::places::Where;
use crate::files::remote::Address;
use crate::common::keyring_said;
use crate::common::smb::Login;

use crate::toolkit::ui::App;
use super::*;

#[test]
fn two_windows_keeping_logins_at_once_take_turns_and_lose_no_address() {
    use std::time::Duration;
    let folder = std::env::temp_dir().join(format!("trindle-shares-turn-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).unwrap();
    let file = folder.join("turn");
    let ring = pretend::ring();
    pretend::slow(Duration::from_millis(300));
    let window = |name: &'static str, at: &'static str| {
        let (ring, file) = (ring.clone(), file.clone());
        std::thread::spawn(move || {
            keyring_said::pretend(Some(name.into()));
            keyring_said::pretend_shared(Some(file));
            pretend::share_ring(ring);
            let waited = keep_login("nas.invalid", None, &Login::typed("jo", "right"), Some(&Confirmed::made_up(at)), false);
            (waited.now, waited.late.is_none())
        })
    };
    let windows = [window("test:folders-window-one", "10.0.0.2"), window("test:folders-window-two", "10.0.0.3")];
    for _ in 0..400 {
        if pretend::most_at_once() > 0 {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(pretend::most_at_once(), 1, "a keeping is under way");
    keyring_said::pretend(Some("test:folders-window-three".into()));
    keyring_said::pretend_shared(Some(file.clone()));
    let read = seen_for("nas.invalid", &Login::typed("jo", "right")).expect("the keyring answers");
    assert!(!read.is_empty(), "the list was read while a keeping was half done");
    assert_eq!(kept_login("nas.invalid", None), Ok(Some(Login::typed("jo", "right"))));
    for w in windows {
        assert_eq!(w.join().unwrap(), (Ok(()), true));
    }
    assert_eq!(pretend::most_at_once(), 1, "two windows were keeping a login at once");
    let mut seen = pretend::seen_at("nas.invalid");
    seen.sort();
    assert_eq!(seen, ["10.0.0.2", "10.0.0.3"], "one window's address was lost");
    keyring_said::pretend(None);
    keyring_said::pretend_shared(None);
    let _ = std::fs::remove_dir_all(&folder);
}

#[test]
fn nothing_typed_goes_to_an_address_only_heard_until_the_card_has_shown_it() {
    use crate::files::neighbours::Computer;
    let found = |f: &mut Files, at: &str| f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some(at.into()) }]));
    let with_login = || pretend::sent().into_iter().filter(|(_, _, user)| user.is_some()).collect::<Vec<_>>();
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    found(&mut f, "10.0.0.66");
    f.shares_login("nas.invalid", Login::typed("jo", "right"));
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(with_login().is_empty(), "the login went out unseen: {:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert_eq!((a.address.as_deref(), a.user.as_str(), a.password.as_str()), (Some("10.0.0.66"), "jo", "right"));
    let said = view(a, 1).said();
    assert!(said.iter().any(|s| s.contains("10.0.0.66") && s.contains("Any device on a network can answer to a name")), "{said:?}");
    assert!(said.iter().any(|s| s.contains("goes to 10.0.0.66") && s.contains("kept for that address")), "{said:?}");
    found(&mut f, "10.0.0.77");
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(with_login(), [("nas.invalid".to_string(), Some("10.0.0.66".to_string()), Some("jo".to_string()))]);
    let mut g = Files::new(vec![std::env::temp_dir()]);
    found(&mut g, "10.0.0.5");
    g.go(p, Where::Remote("smb://nas.invalid/".into()));
    let Some(Dialog::Share(a)) = &g.dialog else { panic!("no card: {}", g.said) };
    let said = view(a, 1).said();
    assert!(said.iter().any(|s| s.contains("goes to 10.0.0.5")), "{said:?}");
}

#[test]
fn a_card_never_names_one_address_and_sends_to_another() {
    use crate::files::neighbours::Computer;
    let found = |f: &mut Files, at: &str| f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some(at.into()) }]));
    pretend::kept("nas.invalid", Login::typed("jo", "right"), &["10.0.0.2"]);
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    found(&mut f, "10.0.0.66");
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert_eq!(a.address.as_deref(), Some("10.0.0.66"));
    found(&mut f, "10.0.0.77");
    f.smb_open(p, &Address::parse("smb://nas.invalid/Photos").unwrap());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    let said = view(a, 1).said().join(" ");
    assert_eq!(a.address.as_deref(), Some("10.0.0.66"));
    assert!(said.contains("10.0.0.66") && !said.contains("10.0.0.77"), "the card names where it does not send: {said}");
    f.update(Msg::Share(ShareMsg::User("jo".into())));
    f.update(Msg::Share(ShareMsg::Password("right".into())));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(pretend::sent().iter().all(|(_, ip, _)| ip.as_deref() != Some("10.0.0.77")), "{:?}", pretend::sent());
}

#[test]
fn a_held_login_waits_for_the_card_at_every_opening_of_an_address_only_heard() {
    use crate::files::neighbours::Computer;
    let jo = || Login::typed("jo", "right");
    let found = |f: &mut Files, at: &str| f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some(at.into()) }]));
    let open = |f: &mut Files| {
        let p = f.side();
        f.go(p, Where::Network);
        f.go(p, Where::Remote("smb://nas.invalid/".into()));
    };
    let sent_to = |at: &str| ("nas.invalid".to_string(), Some(at.to_string()), Some("jo".to_string()));
    let signed = now() - 1000;
    pretend::kept_on("nas.invalid", jo(), &[("10.0.0.2", signed)]);
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    found(&mut f, "10.0.0.2");
    open(&mut f);
    assert!(pretend::sent().is_empty(), "the kept login went to an address only heard before the card was answered: {:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert_eq!((a.address.as_deref(), a.user.as_str(), a.password.as_str()), (Some("10.0.0.2"), "jo", "right"), "the card is filled in with the kept login");
    assert!(a.heard && !a.failed && !a.held);
    let said = view(a, 1).said().join(" ");
    assert!(said.contains("nas.invalid") && said.contains("10.0.0.2") && said.contains("goes to 10.0.0.2"), "{said}");
    assert!(!said.contains('\u{2014}'), "{said}");
    assert_eq!(a.why, again_words("nas.invalid", "10.0.0.2"));
    assert!(a.why.contains("port 445") && a.why.contains("only heard on the network") && a.why.contains("has not been sent"), "{}", a.why);
    assert!(a.remember, "the keyring has it, and the switch says so");
    f.reload_all();
    assert!(pretend::sent().is_empty(), "{:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("the card went: {}", f.said) };
    assert_eq!((a.user.as_str(), a.password.as_str(), a.busy), ("jo", "right", false), "the card that was up lost what was on it");
    found(&mut f, "10.0.0.77");
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(pretend::sent(), [sent_to("10.0.0.2")]);
    assert_eq!(pretend::seen_when("nas.invalid"), [("10.0.0.2".to_string(), signed)]);
    found(&mut f, "10.0.0.2");
    open(&mut f);
    assert_eq!(pretend::sent().len(), 1, "a second opening sent the login with no card: {:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card the second time: {}", f.said) };
    assert_eq!((a.address.as_deref(), a.user.as_str()), (Some("10.0.0.2"), "jo"));
    f.update(Msg::Share(ShareMsg::Close));
    assert_eq!(pretend::sent().len(), 1);
    f.smb_open(p, &Address::parse("smb://nas.invalid/Photos").unwrap());
    assert_eq!(pretend::sent().len(), 1, "{:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card for the share: {}", f.said) };
    assert_eq!((a.share.as_deref(), a.address.as_deref(), a.user.as_str()), (Some("Photos"), Some("10.0.0.2"), "jo"));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(pretend::sent(), [sent_to("10.0.0.2"), sent_to("10.0.0.2")]);
    assert_eq!(f.cols[p].tabs[0].at, Where::Folder(pretend::folder("Photos")));
    f.update(Msg::Share(ShareMsg::Eject(0)));
    open(&mut f);
    f.update(Msg::Share(ShareMsg::Password("wrong".into())));
    f.update(Msg::Share(ShareMsg::Go));
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("the card closed") };
    assert!(a.failed && a.address.as_deref() == Some("10.0.0.2"));
    f.update(Msg::Share(ShareMsg::Close));
    let _ = std::fs::remove_dir_all(pretend::folder("Photos"));
}

type Route = (&'static str, fn(&mut Files));

fn routes_to_a_computer() -> [Route; 9] {
    use crate::files::far::FarMsg;
    [
        ("the sidebar's Network", |f| {
            let p = f.side();
            f.go(p, Where::Network);
            let i = f.cols[p].tabs[0].entries.iter().position(|e| e.name == "nas.local").expect("the computer listed");
            f.open(p, i);
        }),
        ("the address box", |f| {
            f.update(Msg::Locate);
            f.update(Msg::LocateText("smb://nas.local/".into()));
            f.update(Msg::LocateDone);
        }),
        ("a share's own address, as a place kept or a link gives it", |f| {
            let p = f.side();
            f.go(p, Where::Remote("smb://nas.local/Photos".into()));
        }),
        ("the Connect card with no login typed", |f| {
            f.update(Msg::Far(FarMsg::Open));
            f.update(Msg::Far(FarMsg::Address("smb://nas.local/".into())));
            f.update(Msg::Far(FarMsg::Go));
        }),
        ("a new tab", |f| {
            let p = f.side();
            f.new_tab(p, Some(Where::Remote("smb://nas.local/".into())));
        }),
        ("reading the pane again", |f| {
            let p = f.side();
            f.go(p, Where::Remote("smb://nas.local/".into()));
            f.update(Msg::Share(ShareMsg::Close));
            f.update(Msg::Reload);
        }),
        ("every pane read again, as after a share closes", |f| {
            let p = f.side();
            f.go(p, Where::Remote("smb://nas.local/".into()));
            f.update(Msg::Share(ShareMsg::Close));
            f.reload_all();
        }),
        ("going back", |f| {
            let p = f.side();
            f.go(p, Where::Remote("smb://nas.local/".into()));
            f.update(Msg::Share(ShareMsg::Close));
            f.go(p, Where::Network);
            f.update(Msg::Back);
        }),
        ("a second window of the same person's", |f| {
            let mut g = Files::new(vec![std::env::temp_dir()]);
            g.update(Msg::Neighbours(f.neighbours.clone()));
            let p = g.side();
            g.go(p, Where::Remote("smb://nas.local/".into()));
            assert!(matches!(g.dialog, Some(Dialog::Share(_))), "{}", g.said);
            f.go(p, Where::Remote("smb://nas.local/".into()));
        }),
    ]
}

fn a_window_put_back_asks_and_sends_nothing() {
    let home = std::env::temp_dir();
    let last = [(PathBuf::from("smb://nas.local/"), "left".to_string()), (PathBuf::from("smb://nas.local/Photos"), "right".to_string())];
    let start = crate::files::kept::start_from(&home, "last", [String::new(), String::new()], &last);
    assert!(!matches!(start.0, Where::Remote(_)) && !matches!(start.1, Where::Remote(_)), "a window is put back at a server");
    let f = Files::with(Vec::new(), home, (Where::Remote("smb://nas.local/".into()), Where::Recent), Default::default(), Vec::new());
    assert!(pretend::sent().is_empty(), "a window put back at a computer sent this with no card: {:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("a window put back at a computer shows no card: {}", f.said) };
    assert_eq!((a.address.as_deref(), a.user.as_str(), a.heard), (Some("10.0.0.2"), "jo", true));
}

#[test]
fn no_route_sends_a_held_login_to_an_address_only_heard_until_open_is_pressed() {
    use crate::files::neighbours::Computer;
    let jo = || Login::typed("jo", "right");
    pretend::dns("nas.local", "10.0.0.2");
    pretend::kept("nas.local", jo(), &["10.0.0.2"]);
    for (name, route) in routes_to_a_computer() {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.update(Msg::Neighbours(vec![Computer { name: "nas.local".into(), address: Some("10.0.0.2".into()) }]));
        route(&mut f);
        assert!(pretend::sent().is_empty(), "by {name}, this went out before the card was answered: {:?}", pretend::sent());
        let Some(Dialog::Share(a)) = &f.dialog else { panic!("by {name} there is no card: {}", f.said) };
        assert_eq!((a.address.as_deref(), a.user.as_str(), a.heard), (Some("10.0.0.2"), "jo", true), "by {name}");
    }
    a_window_put_back_asks_and_sends_nothing();
}

#[test]
fn an_address_the_person_gave_is_not_asked_about_again_within_its_90_days() {
    let jo = || Login::typed("jo", "right");
    let open = |f: &mut Files, at: &str| {
        let p = f.side();
        f.go(p, Where::Network);
        f.go(p, Where::Remote(at.into()));
    };
    pretend::kept_on("10.0.0.9", jo(), &[("10.0.0.9", now() - LAPSE + 3600)]);
    pretend::dns("nas.invalid", "10.0.0.2");
    let mut f = Files::new(vec![std::env::temp_dir()]);
    for n in 1..=3 {
        open(&mut f, "smb://10.0.0.9/");
        assert!(f.dialog.is_none(), "{}", f.said);
        assert_eq!(pretend::sent().len(), n);
    }
    assert!(pretend::sent().iter().all(|s| *s == ("10.0.0.9".to_string(), Some("10.0.0.9".to_string()), Some("jo".to_string()))));
    pretend::kept("nas.invalid", jo(), &["10.0.0.2"]);
    let mut g = Files::new(vec![std::env::temp_dir()]);
    for n in 4..=5 {
        open(&mut g, "smb://nas.invalid/");
        assert!(g.dialog.is_none(), "{}", g.said);
        assert_eq!(pretend::sent().len(), n);
    }
    assert_eq!(pretend::sent()[4], ("nas.invalid".to_string(), Some("10.0.0.2".to_string()), Some("jo".to_string())));
}
