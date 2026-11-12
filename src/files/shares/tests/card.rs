//! The sign-in card, and what is said of a login that could not be kept.

use zeroize::Zeroizing;

use crate::files::app::{Files, Msg};
use crate::files::dialogs::Dialog;
use crate::files::places::Where;
use crate::files::remote::Address;
use crate::common::keyring_said;
use crate::common::smb::Login;

use crate::toolkit::ui::App;
use super::*;

#[test]
fn problems_are_said_plainly() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.go(p, Where::Remote("smb://asleep.invalid/".into()));
    assert!(f.dialog.is_none() && f.bad && f.said.contains("asleep"), "{}", f.said);
    f.update(Msg::Neighbours(vec![crate::files::neighbours::Computer { name: "nas.invalid".into(), address: Some("10.0.0.2".into()) }]));
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    f.update(Msg::Share(ShareMsg::Close));
    f.smb_open(p, &Address::parse("smb://nas.invalid/Nothing").unwrap());
    assert!(f.dialog.is_some(), "a guest is asked to log in first");
    f.update(Msg::Share(ShareMsg::User("jo".into())));
    f.update(Msg::Share(ShareMsg::Password(String::from("right").into())));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none() && f.bad && f.said.contains("doesn't have a shared folder called Nothing"), "{}", f.said);
    let mut g = Files::new(vec![std::env::temp_dir()]);
    g.smb_open(p, &Address::parse("smb://nas.invalid/Public").unwrap());
    assert!(g.dialog.is_none(), "{}", g.said);
    assert_eq!(g.cols[p].tabs[0].at, Where::Folder(pretend::folder("Public")));
    let _ = std::fs::remove_dir_all(pretend::folder("Public"));
}

#[test]
fn the_card_asks_plainly_and_hides_the_password() {
    let a = Ask { server: "NAS".into(), share: Some("Photos".into()), rest: String::new(), user: "jo".into(),
                  password: Zeroizing::new("hidden".into()), remember: true, why: "NAS wants a user name and password.".into(), keyring: None,
                  failed: false, busy: false, tab: 0, pane: 0, address: None, held: false, heard: false, before: Vec::new(), forget: false, filled: None };
    let node = view(&a, 1);
    let said = node.said();
    assert!(said.iter().any(|s| s == "Open Photos on NAS"), "{said:?}");
    assert!(said.iter().any(|s| s == "NAS wants a user name and password."));
    assert!(said.iter().any(|s| s == "secret: Password"), "the password box shows dots: {said:?}");
    assert!(said.iter().any(|s| s == "switch: Remember for 90 days"));
    assert!(said.iter().any(|s| s == "The 90 days start again each time you use it."), "{said:?}");
    assert_eq!(LAPSE, 90 * 24 * 60 * 60, "the switch says 90 days");
    assert!(said.iter().any(|s| s == "button: Open") && said.iter().any(|s| s == "button: Close"));
    assert!(!said.iter().any(|s| s.contains("hidden")));
}

#[test]
fn a_keyring_that_cannot_say_is_said_in_red_on_the_card_and_never_as_no_login_kept() {
    use crate::common::keyring_said::{locked_words, missing_words, silent_words};
    for why in [locked_words(), missing_words(), silent_words()] {
        let mut g = Files::new(vec![std::env::temp_dir()]);
        let p = g.side();
        pretend::kept("nas.invalid", Login::typed("jo", "right"), &["10.0.0.2"]);
        pretend::dns("nas.invalid", "10.0.0.2");
        pretend::keyring_cannot_say(Some(why));
        let before = pretend::sent().len();
        g.go(p, Where::Remote("smb://nas.invalid/".into()));
        let Some(Dialog::Share(a)) = &g.dialog else { panic!("{why}: no card asked: {}", g.said) };
        assert_eq!(a.keyring.as_deref(), Some(why), "the card does not say why the keyring gave nothing");
        assert!(pretend::sent()[before..].iter().all(|(_, _, user)| user.is_none()), "{:?}", pretend::sent());
        let card = view(a, 0);
        let (said, red) = keyed(&card, "sharekeyring").expect("the card has no line for the keyring");
        assert_eq!(said, why);
        assert!(red, "what the keyring could not say is not in red");
        let lines = card.said();
        let at = |w: &str| lines.iter().position(|l| l == w).unwrap_or_else(|| panic!("{w:?} not on the card: {lines:?}"));
        assert_eq!(at(why), at(&a.why) + 1);
        assert!(!lines.iter().any(|l| l.to_lowercase().contains("no login kept")), "{lines:?}");
        g.update(Msg::Share(ShareMsg::User("jo".into())));
        g.update(Msg::Share(ShareMsg::Password(String::from("right").into())));
        g.update(Msg::Share(ShareMsg::Go));
        assert!(g.dialog.is_none(), "{why}: {}", g.said);
        pretend::keyring_cannot_say(None);
    }
    let mut g = Files::new(vec![std::env::temp_dir()]);
    let p = g.side();
    let before = pretend::sent().len();
    g.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(g.dialog.is_none(), "{}", g.said);
    assert!(pretend::sent()[before..].iter().any(|(_, _, user)| user.as_deref() == Some("jo")), "{:?}", pretend::sent());
    let plain = Ask { server: "NAS".into(), share: None, rest: String::new(), user: String::new(),
                      password: Zeroizing::new(String::new()), remember: true, why: "NAS wants a user name and password.".into(), keyring: None,
                      failed: false, busy: false, tab: 0, pane: 0, address: None, held: false, heard: false, before: Vec::new(), forget: false, filled: None };
    assert!(keyed(&view(&plain, 0), "sharekeyring").is_none());
}

#[test]
fn a_login_that_worked_and_could_not_be_kept_is_told_why() {
    use crate::common::keyring_said::locked_words;
    pretend::keyring_cannot_say(Some(locked_words()));
    let login = Login::typed("jo", "right");
    let at = Confirmed::made_up("10.0.0.2");
    assert_eq!(worked("nas.invalid", Some("Photos"), &login, Some(&at), Some(false), true), (Some(locked_words().to_string()), None));
    assert!(unkept_words(locked_words()).contains("couldn't save the login") && unkept_words(locked_words()).contains(locked_words()));
    assert_eq!(worked("nas.invalid", Some("Photos"), &login, Some(&at), None, true), (None, None));
    pretend::keyring_cannot_say(None);
    assert_eq!(worked("nas.invalid", Some("Photos"), &login, Some(&at), Some(false), true), (None, None));
    assert_eq!(pretend::kept_login("nas.invalid", None), Ok(Some(login.clone())));
    assert_eq!(worked("other.invalid", None, &login, None, Some(false), true), (None, None));
    assert!(pretend::seen_at("other.invalid").is_empty() && pretend::seen_at("nas.invalid") == ["10.0.0.2"]);
}

#[test]
fn a_login_the_keyring_keeps_late_is_heard_of_and_told() {
    use crate::files::neighbours::Computer;
    use crate::common::keyring_said::{locked_words, never_words};
    let open = |how: Result<(), String>, share: bool| {
        pretend::nothing_kept();
        let mut f = Files::new(vec![std::env::temp_dir()]);
        let p = f.side();
        f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some("10.0.0.2".into()) }]));
        f.go(p, Where::Remote(if share { "smb://nas.invalid/Photos" } else { "smb://nas.invalid/" }.into()));
        assert!(matches!(&f.dialog, Some(Dialog::Share(_))), "no card: {}", f.said);
        f.update(Msg::Share(ShareMsg::User("jo".into())));
        f.update(Msg::Share(ShareMsg::Password(String::from("right").into())));
        pretend::keeps_late(how);
        f.update(Msg::Share(ShareMsg::Go));
        assert!(f.dialog.is_none(), "{}", f.said);
        (f.said.clone(), f.bad)
    };
    for share in [true, false] {
        assert_eq!(open(Ok(()), share), ("After a delay, the login was saved in the keyring.".to_string(), false));
        assert_eq!(pretend::seen_at("nas.invalid"), ["10.0.0.2"], "kept after all");
        let refused = open(Err(locked_words().into()), share);
        assert!(refused.1 && refused.0.starts_with("Folders couldn't save the login in the keyring.") && refused.0.ends_with(locked_words()), "{refused:?}");
        assert_eq!(open(Err(never_words().into()), share), ("Folders gave up waiting for the keyring, so the login wasn't saved.".to_string(), true));
    }
    let _ = std::fs::remove_dir_all(pretend::folder("Photos"));
    let login = Login::typed("jo", "right");
    pretend::keeps_late(Ok(()));
    let (why, heard) = worked("nas.invalid", None, &login, Some(&Confirmed::made_up("10.0.0.2")), Some(false), true);
    let heard = heard.expect("the way to hear how it ended");
    assert!(why.is_none() && !keyring_said::waiting("login", false).failed);
    assert_eq!(heard.wait(), Ok(()));
    assert_eq!(heard.wait(), Err(never_words().to_string()), "heard once");
    let mut f = Files::new(vec![std::env::temp_dir()]);
    f.update(Msg::Share(ShareMsg::KeptLate(Ok(()))));
    assert_eq!((f.said.as_str(), f.bad), ("After a delay, the login was saved in the keyring.", false));
}

#[test]
fn a_login_used_at_an_address_no_card_showed_waits_for_the_card_once_the_address_is_only_heard() {
    use crate::files::neighbours::Computer;
    let with_login = || pretend::sent().into_iter().filter(|(_, _, user)| user.is_some()).count();
    let open = |f: &mut Files| {
        let p = f.side();
        f.go(p, Where::Network);
        f.go(p, Where::Remote("smb://nas.invalid/".into()));
    };
    pretend::dns("nas.invalid", "10.0.0.9");
    let mut f = Files::new(vec![std::env::temp_dir()]);
    f.shares_login("nas.invalid", Login::typed("jo", "right"));
    open(&mut f);
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(pretend::sent(), [("nas.invalid".to_string(), Some("10.0.0.9".to_string()), Some("jo".to_string()))]);
    open(&mut f);
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(with_login(), 2);
    pretend::dns_forgets("nas.invalid");
    f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some("10.0.0.9".into()) }]));
    assert_eq!(reach("nas.invalid", Some("10.0.0.9".into())), (Some("10.0.0.9".to_string()), Trust::Heard));
    open(&mut f);
    assert_eq!(with_login(), 2, "the login went to an address only heard, which no card had shown: {:?}", pretend::sent());
    assert_eq!(pretend::sent().len(), 2, "nothing at all went there, not even a guest's knock");
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(a.held && a.heard && !a.failed && a.user.is_empty() && a.password.is_empty());
    assert_eq!(a.address.as_deref(), Some("10.0.0.9"));
    assert_eq!(a.why, unshown_words("nas.invalid", "10.0.0.9"));
    assert!(a.why.contains("found it on the local network") && a.why.contains("hasn't sent your login") && !a.why.contains("a different address") && !a.why.contains('\u{2014}'), "{}", a.why);
    f.update(Msg::Share(ShareMsg::User("jo".into())));
    f.update(Msg::Share(ShareMsg::Password(String::from("right").into())));
    f.update(Msg::Share(ShareMsg::Remember(false)));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(with_login(), 3);
    open(&mut f);
    assert_eq!(with_login(), 3);
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert_eq!((a.held, a.user.as_str(), a.password.as_str(), a.remember), (false, "jo", "right", false));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(with_login(), 4);
    assert!(pretend::seen_at("nas.invalid").is_empty(), "nothing was asked to be kept");
}
