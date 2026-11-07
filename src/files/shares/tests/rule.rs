// The one rule for what is sent, and the addresses a kept login is trusted at.

use crate::files::app::{Files, Msg};
use crate::files::dialogs::Dialog;
use crate::files::places::Where;
use crate::files::remote::Address;
use crate::common::smb::Login;

use crate::toolkit::ui::App;
use super::*;

#[test]
fn a_kept_login_is_never_sent_to_an_address_it_was_not_used_at() {
    use crate::files::neighbours::Computer;
    let jo = || Login::typed("jo", "right");
    let found = |f: &mut Files, at: &str| f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some(at.into()) }]));
    let open = |f: &mut Files| {
        let p = f.side();
        f.go(p, Where::Remote("smb://nas.invalid/".into()));
    };
    let to = |at: &str| pretend::sent().iter().filter(|(_, ip, _)| ip.as_deref() == Some(at)).cloned().collect::<Vec<_>>();
    pretend::kept("nas.invalid", jo(), &["10.0.0.2"]);
    pretend::dns("nas.invalid", "10.0.0.2");
    let mut f = Files::new(vec![std::env::temp_dir()]);
    open(&mut f);
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(to("10.0.0.2"), [("nas.invalid".to_string(), Some("10.0.0.2".to_string()), Some("jo".to_string()))]);
    pretend::dns_forgets("nas.invalid");
    let mut g = Files::new(vec![std::env::temp_dir()]);
    found(&mut g, "10.0.0.66");
    let p = g.side();
    assert_eq!(g.neighbours.len(), 1, "it is still shown as found");
    open(&mut g);
    assert!(to("10.0.0.66").is_empty(), "nothing at all went to the new address: {:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &g.dialog else { panic!("no card: {}", g.said) };
    assert!(a.held && !a.failed && a.user.is_empty() && a.password.is_empty());
    assert!(a.why.contains("10.0.0.66") && a.why.contains("10.0.0.2") && a.why.contains("a different address") && a.why.contains("has not been sent"), "{}", a.why);
    assert_eq!(a.address.as_deref(), Some("10.0.0.66"));
    found(&mut g, "10.0.0.77");
    g.update(Msg::Share(ShareMsg::User("jo".into())));
    g.update(Msg::Share(ShareMsg::Password("right".into())));
    g.update(Msg::Share(ShareMsg::Go));
    assert!(g.dialog.is_none(), "{}", g.said);
    assert_eq!(to("10.0.0.66").len(), 1);
    assert!(to("10.0.0.77").is_empty());
    assert_eq!(pretend::seen_at("nas.invalid"), ["10.0.0.66", "10.0.0.2"]);
    g.smb_open(p, &Address::parse("smb://nas.invalid/Photos").unwrap());
    assert!(to("10.0.0.77").is_empty(), "{:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &g.dialog else { panic!("no card: {}", g.said) };
    assert!(a.held && a.why.contains("10.0.0.77") && a.why.contains("10.0.0.66"), "{}", a.why);
    g.update(Msg::Share(ShareMsg::Close));
    found(&mut g, "10.0.0.66");
    g.smb_open(p, &Address::parse("smb://nas.invalid/Photos").unwrap());
    let Some(Dialog::Share(a)) = &g.dialog else { panic!("no card: {}", g.said) };
    assert_eq!((a.held, a.user.as_str(), a.password.as_str(), a.address.as_deref()), (false, "jo", "right", Some("10.0.0.66")));
    g.update(Msg::Share(ShareMsg::Go));
    assert!(g.dialog.is_none(), "{}", g.said);
    assert_eq!(g.cols[p].tabs[0].at, Where::Folder(pretend::folder("Photos")));
    let _ = std::fs::remove_dir_all(pretend::folder("Photos"));
}

#[test]
fn a_login_with_no_address_known_is_asked_for_and_a_typed_address_is_trusted() {
    use crate::files::neighbours::Computer;
    let jo = || Login::typed("jo", "right");
    pretend::kept("nas.invalid", jo(), &[]);
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some("10.0.0.2".into()) }]));
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(pretend::sent().is_empty());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(a.held && a.why.contains("has not been used at before") && a.why.contains("10.0.0.2"), "{}", a.why);
    f.update(Msg::Share(ShareMsg::Close));
    f.update(Msg::Neighbours(Vec::new()));
    f.go(p, Where::Network);
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(pretend::sent().is_empty());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(a.held && a.why.contains("could not be found beforehand"), "{}", a.why);
    f.update(Msg::Share(ShareMsg::Close));
    pretend::kept("10.0.0.9", jo(), &[]);
    f.go(p, Where::Remote("smb://10.0.0.9/".into()));
    assert!(pretend::sent().is_empty(), "{:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(a.held && a.why.contains("has not been used at before") && a.address.as_deref() == Some("10.0.0.9"), "{}", a.why);
    f.update(Msg::Share(ShareMsg::Close));
    pretend::kept_on("10.0.0.9", jo(), &[("10.0.0.9", now() - LAPSE - 60)]);
    f.go(p, Where::Network);
    f.go(p, Where::Remote("smb://10.0.0.9/".into()));
    assert!(pretend::sent().is_empty(), "{:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(a.held && a.why.contains("more than 90 days ago"), "{}", a.why);
    f.update(Msg::Share(ShareMsg::Close));
    pretend::kept("10.0.0.9", jo(), &["10.0.0.9"]);
    f.go(p, Where::Network);
    f.go(p, Where::Remote("smb://10.0.0.9/".into()));
    assert_eq!(pretend::sent(), [("10.0.0.9".to_string(), Some("10.0.0.9".to_string()), Some("jo".to_string()))]);
}

#[test]
fn an_address_kept_with_no_note_of_whose_it_is_is_said_as_that() {
    use crate::files::neighbours::Computer;
    let jo = || Login::typed("jo", "right");
    pretend::kept_lines("nas.invalid", jo(), &[("10.0.0.2", now(), "")]);
    assert_eq!(pretend::seen_for("nas.invalid", &jo()), Ok(vec![("10.0.0.2".to_string(), 0)]), "given, with no time, as the real keyring gives it");
    let mut f = Files::new(vec![std::env::temp_dir()]);
    f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some("10.0.0.2".into()) }]));
    let p = f.side();
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(pretend::sent().is_empty(), "{:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(a.held && a.why.starts_with("nas.invalid answers at 10.0.0.2, where a login for it was used before, but when, or which login, was not written down."), "{}", a.why);
    pretend::kept_lines("nas.invalid", jo(), &[("10.0.0.2", now(), &mark(&Login::typed("jo", "another")))]);
    assert_eq!(pretend::seen_for("nas.invalid", &jo()), Ok(Vec::new()));
    let many: Vec<String> = (1..=12).map(|n| format!("10.0.0.{n}")).collect();
    let lines: Vec<(&str, u64, &str)> = many.iter().map(|at| (at.as_str(), 5, "")).collect();
    pretend::kept_lines("nas.invalid", jo(), &lines);
    assert_eq!(pretend::seen_for("nas.invalid", &jo()).map(|seen| seen.len()), Ok(SEEN_MOST));
}

const NOW: u64 = 2_000_000_000;

fn jo() -> Login {
    Login::typed("jo", "right")
}

fn none() -> Option<Login> {
    None
}

fn kept() -> Option<Login> {
    Some(jo())
}

fn seen(list: &'static [&'static str]) -> impl Fn(&Login) -> Vec<(String, u64)> {
    move |_: &Login| list.iter().map(|s| (s.to_string(), NOW)).collect()
}

fn used(at: &str) -> Option<Used> {
    Some(Used { login: jo(), at: at.to_string(), confirmed: true })
}

fn given(at: &str) -> Option<Used> {
    Some(Used { login: jo(), at: at.to_string(), confirmed: false })
}

fn nothing_held_and_what_was_just_typed() {
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, none, seen(&[]), NOW), Sends::Login(None));
    assert_eq!(rule("NAS", None, Trust::Heard, None, None, none, seen(&[]), NOW), Sends::Login(None));
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Shown, Some(jo()), None, kept, seen(&[]), NOW), Sends::Login(Some(jo())));
    assert_eq!(rule("files.example.com", Some("192.0.2.5"), Trust::Given, Some(jo()), None, kept, seen(&[]), NOW), Sends::Login(Some(jo())));
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, Some(jo()), None, kept, seen(&["10.0.0.5"]), NOW), Sends::Confirm(jo()));
    assert_eq!(rule("NAS", None, Trust::Heard, Some(jo()), None, none, seen(&[]), NOW), Sends::Held(Vec::new()));
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, kept, seen(&["10.0.0.5"]), NOW), Sends::Again(jo(), true));
    assert_eq!(rule("NAS", Some("fd00:0:0:0:0:0:0:20"), Trust::Heard, None, None, kept, seen(&["fd00::20"]), NOW), Sends::Again(jo(), true));
    assert_eq!(rule("NAS", Some("10.0.0.6"), Trust::Heard, None, None, kept, seen(&["10.0.0.5"]), NOW), Sends::Held(vec!["10.0.0.5".into()]));
    assert_eq!(rule("NAS", None, Trust::Heard, None, None, kept, seen(&["10.0.0.5"]), NOW), Sends::Held(vec!["10.0.0.5".into()]));
    assert_eq!(rule("NAS", Some("10.0.0.6"), Trust::Heard, None, None, kept, seen(&[]), NOW), Sends::Held(Vec::new()));
    assert_eq!(rule("files.example.com", Some("192.0.2.6"), Trust::Given, None, None, kept, seen(&["192.0.2.5"]), NOW), Sends::Held(vec!["192.0.2.5".into()]));
}

fn used_this_run_and_whether_the_card_showed_the_address() {
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, used("10.0.0.5"), none, seen(&[]), NOW), Sends::Again(jo(), false));
    assert_eq!(rule("NAS", Some("10.0.0.6"), Trust::Heard, None, used("10.0.0.5"), none, seen(&[]), NOW), Sends::Held(vec!["10.0.0.5".into()]));
    assert_eq!(rule("NAS", Some("10.0.0.6"), Trust::Heard, None, used("10.0.0.5"), kept, seen(&["10.0.0.5", "10.0.0.4"]), NOW),
               Sends::Held(vec!["10.0.0.5".into(), "10.0.0.4".into()]));
    assert_eq!(rule("files.example.com", Some("192.0.2.5"), Trust::Given, None, given("192.0.2.5"), none, seen(&[]), NOW), Sends::Login(Some(jo())));
    assert_eq!(rule("files.example.com", Some("192.0.2.5"), Trust::Heard, None, given("192.0.2.5"), none, seen(&[]), NOW), Sends::Unshown(vec!["192.0.2.5".into()]));
    assert_eq!(rule("files.example.com", Some("192.0.2.5"), Trust::Heard, None, given("192.0.2.5"), kept, seen(&["192.0.2.4"]), NOW),
               Sends::Unshown(vec!["192.0.2.5".into(), "192.0.2.4".into()]));
    assert_eq!(rule("files.example.com", Some("192.0.2.5"), Trust::Heard, None, given("192.0.2.5"), kept, seen(&["192.0.2.5"]), NOW), Sends::Again(jo(), true));
    assert_eq!(rule("files.example.com", Some("192.0.2.6"), Trust::Heard, None, given("192.0.2.5"), none, seen(&[]), NOW), Sends::Held(vec!["192.0.2.5".into()]));
    let confirmed = |sent: (Sends, bool)| sent.1;
    assert!(confirmed(choose("NAS", Some("10.0.0.5"), Trust::Shown, Some(jo()), None, none, seen(&[]), NOW)));
    assert!(!confirmed(choose("files.example.com", Some("192.0.2.5"), Trust::Given, Some(jo()), None, none, seen(&[]), NOW)));
    assert!(confirmed(choose("files.example.com", Some("192.0.2.5"), Trust::Given, None, None, kept, seen(&["192.0.2.5"]), NOW)));
    assert!(confirmed(choose("files.example.com", Some("192.0.2.5"), Trust::Given, None, used("192.0.2.5"), none, seen(&[]), NOW)));
    assert!(!confirmed(choose("NAS", Some("10.0.0.5"), Trust::Heard, None, None, kept, seen(&["10.0.0.5"]), NOW)), "nothing was sent: the card asks");
    assert!(!confirmed(choose("NAS", Some("10.0.0.5"), Trust::Heard, None, used("10.0.0.5"), none, seen(&[]), NOW)), "nothing was sent: the card asks");
    assert!(!confirmed(choose("files.example.com", Some("192.0.2.5"), Trust::Given, None, given("192.0.2.5"), none, seen(&[]), NOW)));
    assert!(!confirmed(choose("NAS", Some("10.0.0.5"), Trust::Heard, Some(jo()), None, none, seen(&[]), NOW)), "nothing was sent");
    assert!(!confirmed(choose("NAS", Some("10.0.0.5"), Trust::Heard, None, None, none, seen(&[]), NOW)), "a guest has no login");
}

fn a_held_login_never_goes_to_an_address_only_heard() {
    for ago in [0, 60, 3600, 24 * 3600, LAPSE] {
        let then = NOW - ago;
        let at = move |_: &Login| vec![("10.0.0.5".to_string(), then)];
        assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, kept, at, NOW), Sends::Again(jo(), true), "{ago} seconds on");
        assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, used("10.0.0.5"), kept, at, NOW), Sends::Again(jo(), true), "{ago} seconds on");
        assert_eq!(rule("10.0.0.5", Some("10.0.0.5"), Trust::Given, None, None, kept, at, NOW), Sends::Login(Some(jo())), "{ago} seconds on");
    }
    for trust in [Trust::Given, Trust::Heard, Trust::Shown] {
        for (held, list) in [(used("10.0.0.5"), &[][..]), (None, &["10.0.0.5"][..]), (used("10.0.0.5"), &["10.0.0.5"][..]), (given("10.0.0.5"), &[][..])] {
            let list: Vec<(String, u64)> = list.iter().map(|at| (at.to_string(), NOW)).collect();
            let sent = rule("NAS", Some("10.0.0.5"), trust, None, held.clone(), kept, move |_: &Login| list, NOW);
            assert!(trust != Trust::Heard || !matches!(sent, Sends::Login(Some(_))), "a held login was chosen for an address only heard: {held:?}");
        }
    }
}

fn a_name_is_no_address_and_an_address_is_a_name_too() {
    assert_eq!(rule("NAS", Some("nas"), Trust::Heard, None, None, kept, seen(&["nas"]), NOW), Sends::Held(vec!["nas".into()]));
    assert_eq!(rule("10.0.0.9", Some("10.0.0.9"), Trust::Given, None, None, kept, seen(&[]), NOW), Sends::Held(Vec::new()));
    assert_eq!(rule("10.0.0.9", Some("10.0.0.9"), Trust::Given, None, None, kept, seen(&["10.0.0.9"]), NOW), Sends::Login(Some(jo())));
    assert_eq!(rule("10.0.0.9", Some("10.0.0.9"), Trust::Given, None, None, kept, seen(&["10.0.0.8"]), NOW), Sends::Held(vec!["10.0.0.8".into()]));
    let at_nine = |when: u64| move |_: &Login| vec![("10.0.0.9".to_string(), when)];
    assert_eq!(rule("10.0.0.9", Some("10.0.0.9"), Trust::Given, None, None, kept, at_nine(NOW - LAPSE - 1), NOW), Sends::Lapsed(vec!["10.0.0.9".into()]));
    assert_eq!(rule("10.0.0.9", Some("10.0.0.9"), Trust::Given, None, None, kept, at_nine(0), NOW), Sends::Unknown(vec!["10.0.0.9".into()]));
    assert_eq!(rule("fd00::20", Some("fd00:0:0:0:0:0:0:20"), Trust::Given, None, None, kept, seen(&["fd00::20"]), NOW), Sends::Login(Some(jo())));
    assert_eq!(reach("10.0.0.9", Some("10.0.0.66".into())), (Some("10.0.0.9".to_string()), Trust::Given), "what was found for an address is not used in its place");
    assert_eq!(reach("nas", Some("10.0.0.2".into())), (Some("10.0.0.2".to_string()), Trust::Heard));
    let words = held_words("NAS", Some("10.0.0.6"), &["10.0.0.5".to_string()]);
    assert!(words.starts_with("NAS answers at 10.0.0.6 now, a different address from the one your login for it was used at before (10.0.0.5)."), "{words}");
    assert!(words.contains("the login kept for it has not been sent"), "{words}");
}

fn the_time_an_address_counts_and_whose_login_it_is() {
    let on = |when: u64| move |_: &Login| vec![("10.0.0.5".to_string(), when)];
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, kept, on(NOW - LAPSE), NOW), Sends::Again(jo(), true));
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, kept, on(NOW - LAPSE - 1), NOW), Sends::Lapsed(vec!["10.0.0.5".into()]));
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, kept, on(0), NOW), Sends::Unknown(vec!["10.0.0.5".into()]));
    let whose = |l: &Login| if *l == jo() { vec![("10.0.0.5".to_string(), NOW)] } else { Vec::new() };
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, kept, whose, NOW), Sends::Again(jo(), true));
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, || Some(Login::typed("jo", "another")), whose, NOW), Sends::Held(Vec::new()));
    assert_ne!(mark(&jo()), mark(&Login::typed("jo", "another")));
    assert_ne!(mark(&jo()), mark(&Login::typed("WORK\\jo", "right")));
    assert_eq!(mark(&jo()), mark(&Login::typed("jo", "right")));
    let eight: Vec<String> = (1..=8).map(|n| format!("10.0.0.{n}")).collect();
    assert!(held_words("NAS", Some("10.0.0.9"), &eight).contains("Only the last 8 addresses it was used at are remembered."));
    assert!(!held_words("NAS", Some("10.0.0.9"), &eight[..7]).contains("Only the last"));
    assert_eq!(rule("NAS", Some("10.0.0.5"), Trust::Heard, None, None, kept, on(NOW + 500), NOW), Sends::Again(jo(), true), "a clock put back is not a lapse");
    assert_eq!(LAPSE, 90 * 24 * 60 * 60);
    for (name, real) in [("files.example.com", true), ("nas.lan.", true), ("nas", false), ("NAS.local", false), ("nas.local.", false), ("nas.", false)] {
        assert_eq!(real_name(name), real, "{name}");
    }
}

#[test]
fn the_one_rule_for_what_is_sent() {
    nothing_held_and_what_was_just_typed();
    used_this_run_and_whether_the_card_showed_the_address();
    a_held_login_never_goes_to_an_address_only_heard();
    a_name_is_no_address_and_an_address_is_a_name_too();
    the_time_an_address_counts_and_whose_login_it_is();
}

#[test]
fn a_real_host_name_the_persons_own_dns_answers_is_given() {
    use crate::files::neighbours::Computer;
    pretend::dns("nas.invalid", "10.0.0.9");
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some("10.0.0.66".into()) }]));
    assert_eq!(reach("nas.invalid", Some("10.0.0.66".into())), (Some("10.0.0.9".to_string()), Trust::Given));
    f.shares_login("nas.invalid", Login::typed("jo", "right"));
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(pretend::sent(), [("nas.invalid".to_string(), Some("10.0.0.9".to_string()), Some("jo".to_string()))]);
    pretend::dns("nas.local", "10.0.0.9");
    assert_eq!(reach("nas.local", None), (Some("10.0.0.9".to_string()), Trust::Heard));
    assert_eq!(reach("nas.local", Some("10.0.0.66".into())), (Some("10.0.0.66".to_string()), Trust::Heard));
    let mut g = Files::new(vec![std::env::temp_dir()]);
    g.go(p, Where::Remote("smb://nas.invalid/".into()));
    let Some(Dialog::Share(a)) = &g.dialog else { panic!("no card: {}", g.said) };
    assert!(!a.heard && !a.held);
    let said = view(a, 1).said();
    assert!(said.iter().any(|s| s == "What you type goes to 10.0.0.9."), "{said:?}");
    assert!(!said.iter().any(|s| s.contains("Any device")), "{said:?}");
}

#[test]
fn with_no_address_nothing_typed_is_sent() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.shares_login("nas.invalid", Login::typed("jo", "right"));
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(pretend::sent().is_empty(), "{:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(a.address.is_none() && a.user.is_empty() && a.password.is_empty());
    assert!(a.why.contains("could not be found beforehand") && a.why.contains("Open it by its address instead, such as smb://192.168.1.20/."), "{}", a.why);
    assert!(!view(a, 1).said().iter().any(|s| s.starts_with("What you type goes to")));
    f.update(Msg::Share(ShareMsg::User("jo".into())));
    f.update(Msg::Share(ShareMsg::Password("right".into())));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(pretend::sent().is_empty(), "{:?}", pretend::sent());
    assert!(matches!(&f.dialog, Some(Dialog::Share(a)) if !a.busy));
    let mut g = Files::new(vec![std::env::temp_dir()]);
    g.smb_open(p, &Address::parse("smb://nas.invalid/Public").unwrap());
    assert!(g.dialog.is_none(), "{}", g.said);
    assert_eq!(pretend::sent(), [("nas.invalid".to_string(), None, None)]);
    let _ = std::fs::remove_dir_all(pretend::folder("Public"));
}

#[test]
fn an_address_unused_for_long_is_asked_about_again_and_earlier_ones_can_be_forgotten() {
    use crate::files::neighbours::Computer;
    let found = |f: &mut Files, at: &str| f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some(at.into()) }]));
    let jo = || Login::typed("jo", "right");
    let long_ago = now() - LAPSE - 60;
    pretend::kept_on("nas.invalid", jo(), &[("10.0.0.2", long_ago), ("10.0.0.3", now())]);
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    found(&mut f, "10.0.0.2");
    f.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(pretend::sent().is_empty(), "the login went to an address long unused: {:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(a.held && !a.failed && a.user.is_empty());
    assert!(a.why.starts_with("nas.invalid answers at 10.0.0.2, where your login for it was last used more than 90 days ago.") && a.why.contains("has not been sent"), "{}", a.why);
    assert_eq!(a.before, ["10.0.0.2", "10.0.0.3"]);
    let said = view(a, 1).said();
    assert!(said.iter().any(|s| s == "switch: Forget the addresses this login was used at before"), "{said:?}");
    f.update(Msg::Share(ShareMsg::Forget(true)));
    f.update(Msg::Share(ShareMsg::User("jo".into())));
    f.update(Msg::Share(ShareMsg::Password("right".into())));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(pretend::seen_at("nas.invalid"), ["10.0.0.2"]);
    let lately = now() - LAPSE + 3600;
    pretend::kept_on("nas.invalid", jo(), &[("10.0.0.2", lately)]);
    let mut g = Files::new(vec![std::env::temp_dir()]);
    found(&mut g, "10.0.0.2");
    g.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(matches!(&g.dialog, Some(Dialog::Share(a)) if !a.held && a.user == "jo"), "{}", g.said);
    g.update(Msg::Share(ShareMsg::Go));
    assert!(g.dialog.is_none(), "{}", g.said);
    assert_eq!(pretend::seen_when("nas.invalid")[0].1, lately, "a use nobody typed must not renew the address");
    let mut h = Files::new(vec![std::env::temp_dir()]);
    pretend::kept_on("nas.invalid", jo(), &[]);
    found(&mut h, "10.0.0.2");
    h.go(p, Where::Remote("smb://nas.invalid/".into()));
    let Some(Dialog::Share(a)) = &h.dialog else { panic!("no card: {}", h.said) };
    assert!(!view(a, 1).said().iter().any(|s| s.starts_with("switch: Forget")));
}

#[test]
fn trust_in_an_address_is_the_logins_and_only_signing_in_renews_it() {
    use crate::files::neighbours::Computer;
    let found = |f: &mut Files, at: &str| f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some(at.into()) }]));
    let open = |f: &mut Files| {
        let p = f.side();
        f.go(p, Where::Remote("smb://nas.invalid/".into()));
    };
    let to = |at: &str| pretend::sent().iter().filter(|(_, ip, _)| ip.as_deref() == Some(at)).cloned().collect::<Vec<_>>();
    let jo = || Login::typed("jo", "right");
    let lately = now() - LAPSE + 3600;
    pretend::kept_on("nas.invalid", jo(), &[("10.0.0.2", lately)]);
    let mut f = Files::new(vec![std::env::temp_dir()]);
    found(&mut f, "10.0.0.2");
    open(&mut f);
    assert!(matches!(&f.dialog, Some(Dialog::Share(a)) if !a.held && a.user == "jo"), "{}", f.said);
    assert!(to("10.0.0.2").is_empty());
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(to("10.0.0.2").len(), 1);
    assert_eq!(pretend::seen_when("nas.invalid"), [("10.0.0.2".to_string(), lately)], "a use nobody typed renewed the address");
    pretend::kept_on("nas.invalid", jo(), &[("10.0.0.2", now() - LAPSE - 60)]);
    let mut k = Files::new(vec![std::env::temp_dir()]);
    found(&mut k, "10.0.0.2");
    open(&mut k);
    k.update(Msg::Share(ShareMsg::User("jo".into())));
    k.update(Msg::Share(ShareMsg::Password("right".into())));
    k.update(Msg::Share(ShareMsg::Go));
    assert!(k.dialog.is_none(), "{}", k.said);
    assert!(pretend::seen_when("nas.invalid")[0].1 >= now() - 5);
}

#[test]
fn a_login_is_not_sent_to_an_address_only_an_older_login_was_used_at() {
    use crate::files::neighbours::Computer;
    let found = |f: &mut Files, at: &str| f.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some(at.into()) }]));
    let open = |f: &mut Files| {
        let p = f.side();
        f.go(p, Where::Remote("smb://nas.invalid/".into()));
    };
    let to = |at: &str| pretend::sent().iter().filter(|(_, ip, _)| ip.as_deref() == Some(at)).cloned().collect::<Vec<_>>();
    pretend::kept_on("nas.invalid", Login::typed("jo", "an older password"), &[("10.0.0.20", now())]);
    let mut g = Files::new(vec![std::env::temp_dir()]);
    found(&mut g, "10.0.0.30");
    open(&mut g);
    let Some(Dialog::Share(a)) = &g.dialog else { panic!("no card: {}", g.said) };
    assert!(a.held && a.remember && !a.forget);
    g.update(Msg::Share(ShareMsg::User("jo".into())));
    g.update(Msg::Share(ShareMsg::Password("right".into())));
    g.update(Msg::Share(ShareMsg::Go));
    assert!(g.dialog.is_none(), "{}", g.said);
    let mut h = Files::new(vec![std::env::temp_dir()]);
    found(&mut h, "10.0.0.20");
    open(&mut h);
    assert!(to("10.0.0.20").is_empty(), "a login was sent to an address it was never signed in to at: {:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &h.dialog else { panic!("no card: {}", h.said) };
    assert!(a.held && a.why.contains("has not been sent"), "{}", a.why);
    assert_eq!(a.before, ["10.0.0.30"], "the card names the addresses of this login only");
    h.update(Msg::Share(ShareMsg::Close));
}

#[test]
fn an_address_kept_with_no_time_is_said_as_that() {
    use crate::files::neighbours::Computer;
    pretend::kept_on("nas.invalid", Login::typed("jo", "right"), &[("10.0.0.2", 0), ("10.0.0.3", now())]);
    let mut m = Files::new(vec![std::env::temp_dir()]);
    m.update(Msg::Neighbours(vec![Computer { name: "nas.invalid".into(), address: Some("10.0.0.2".into()) }]));
    let p = m.side();
    m.go(p, Where::Remote("smb://nas.invalid/".into()));
    assert!(pretend::sent().is_empty(), "{:?}", pretend::sent());
    let Some(Dialog::Share(a)) = &m.dialog else { panic!("no card: {}", m.said) };
    assert!(a.held && !a.why.contains("90 days ago"), "{}", a.why);
    assert!(a.why.starts_with("nas.invalid answers at 10.0.0.2, where a login for it was used before, but when, or which login, was not written down."), "{}", a.why);
    assert!(view(a, 1).said().iter().any(|s| s.starts_with("switch: Forget")), "{:?}", view(a, 1).said());
    m.update(Msg::Share(ShareMsg::Remember(false)));
    let Some(Dialog::Share(a)) = &m.dialog else { panic!("no card") };
    assert!(!view(a, 1).said().iter().any(|s| s.starts_with("switch: Forget")), "{:?}", view(a, 1).said());
}
