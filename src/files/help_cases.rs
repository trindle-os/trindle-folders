//! Tests that each kind of failure in Folders shows its quiet More and

use std::path::{Path, PathBuf};

use super::app::{Files, Msg};
use super::dialogs::Dialog;
use super::far::{FarMsg, Question, Why};
use super::help::*;
use super::remote::Error;
use crate::toolkit::ui::App;

fn more() -> String {
    format!("button: {}", crate::toolkit::t!("common-more"))
}

pub(super) fn window_has_more(said: &[String]) -> bool {
    said.contains(&more())
}

fn page_of(key: &str) -> &'static str {
    MESSAGES.iter().find(|(k, _)| *k == key).unwrap_or_else(|| panic!("{key} is not in the table")).1
}

#[test]
fn a_server_error_lands_on_the_page_for_its_kind() {
    use Error::*;
    let here = || "x".to_string();
    let cert = || CertUnknown { host: "h".into(), fingerprint: "AB".into(), why: here() };
    let cases: Vec<(Error, &str)> = vec![
        (BadAddress(here()), SERVER_REACH), (Unreachable(here()), SERVER_REACH), (Lost(here()), SERVER_REACH),
        (NeedsPassword, SERVER_LOGIN), (LoginRefused, SERVER_LOGIN), (HostKeyUnknown(here()), SERVER_LOGIN), (HostKeyChanged(here()), SERVER_LOGIN),
        (cert(), SERVER_LOGIN), (CertChanged { host: "h".into(), fingerprint: "AB".into() }, SERVER_LOGIN), (Key(here()), SERVER_LOGIN),
        (Heard { host: "h".into(), at: "10.0.0.1".into(), port: 21 }, SERVER_LOGIN),
        (NotFound(here()), SERVER_REFUSED), (Exists(here()), SERVER_REFUSED), (Denied(here()), SERVER_REFUSED), (Busy(here()), SERVER_REFUSED),
        (NotSupported(here()), SERVER_REFUSED), (Cancelled, SERVER_REFUSED), (Failed(here()), SERVER_REFUSED),
        (Io(std::io::Error::other("x")), SERVER_REFUSED),
    ];
    for (e, page) in cases {
        assert_eq!(remote_page(&e, false), page, "{e:?}");
        assert_eq!(remote_page(&e, true), PHONE, "a phone, {e:?}");
    }
}

#[test]
fn the_table_agrees_with_the_page_each_error_kind_leads_to() {
    use Error::*;
    let x = || "x".to_string();
    for (key, e, phone) in [
        ("folders-remote-ftp-no-server", Unreachable(x()), false), ("folders-remote-sftp-no-answer", Unreachable(x()), false),
        ("folders-remote-dav-not-found", Unreachable(x()), false), ("folders-remote-ftp-lost", Lost(x()), false),
        ("folders-remote-sftp-nonsense", Lost(x()), false), ("folders-remote-tls-bad-name", BadAddress(x()), false),
        ("folders-remote-mod-bad-address", BadAddress(x()), false), ("folders-remote-mod-needs-password", NeedsPassword, false),
        ("folders-remote-mod-login-refused", LoginRefused, false), ("folders-remote-mod-host-key-changed", HostKeyChanged(x()), false),
        ("folders-remote-sftp-no-key", Key(x()), false), ("folders-remote-ftp-no-welcome", Failed(x()), false),
        ("folders-remote-dav-full", Failed(x()), false), ("folders-remote-tls-setup-failed", Failed(x()), false),
        ("folders-remote-sftp-no-ssh", Failed(x()), false), ("folders-remote-mod-denied", Denied(x()), false),
        ("folders-remote-mod-not-supported", NotSupported(x()), false), ("folders-remote-ftp-accounts", NotSupported(x()), false),
        ("folders-remote-usb-unplugged", Unreachable(x()), true), ("folders-remote-usb-stopped", Lost(x()), true),
        ("folders-remote-mtp-full", Failed(x()), true), ("folders-remote-mtp-the-phone", Busy(x()), true),
        ("folders-remote-usb-denied", Denied(x()), true),
    ] {
        assert_eq!(page_of(key), remote_page(&e, phone), "{key}");
    }
}

#[test]
fn work_with_a_server_or_a_phone_leads_to_the_page_for_it() {
    use super::work::{Extra, Kind};
    let home = Path::new("/home/john");
    let help = |items: &[&str], to: &str, kind: &Kind| {
        let items: Vec<PathBuf> = items.iter().map(PathBuf::from).collect();
        super::act::words(kind, &items, Path::new(to), home).help
    };
    assert_eq!(help(&["/home/john/a"], "/home/john/b", &Kind::Copy), WORK_NOT_DONE);
    assert_eq!(help(&["/home/john/a"], "sftp://jo@example.com/b", &Kind::Copy), SERVER_REFUSED);
    assert_eq!(help(&["ftp://jo@example.com/a"], "/home/john/b", &Kind::Move), SERVER_REFUSED);
    assert_eq!(help(&["/home/john/a"], "mtp://pixel/internal/b", &Kind::Copy), PHONE);
    assert_eq!(help(&["dav://example.com/a"], "/home/john/b", &Kind::Delete), SERVER_REFUSED);
    assert_eq!(help(&["/home/john/a.zip"], "sftp://jo@example.com/b", &Kind::Other(Extra::Unpack(super::archive::Place::Here))), ARCHIVE_NOT_OPENED);
}

#[test]
fn the_connect_card_has_a_more_beside_a_failure_and_beside_each_question() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    f.update(Msg::Far(FarMsg::Open));
    let said = |f: &Files| match &f.dialog {
        Some(Dialog::Connect(c)) => super::far::view(c, f.turn).said(),
        _ => panic!("no card"),
    };
    assert!(!said(&f).contains(&more()));
    f.update(Msg::Far(FarMsg::Refused(Why::Words("no server called example.invalid could be found".into(), SERVER_REACH))));
    assert!(said(&f).contains(&more()), "{:?}", said(&f));
    for q in [Question::NewHost("h".into()), Question::Changed("h".into()), Question::NeedsKey,
              Question::NewCert { host: "h".into(), fingerprint: "AB:CD".into(), why: "it has expired".into() },
              Question::CertChanged { host: "h".into(), fingerprint: "AB:CD".into() },
              Question::Heard { host: "h".into(), at: "10.0.0.1".into(), port: 21 }] {
        f.update(Msg::Far(FarMsg::Refused(Why::Ask(q))));
        assert!(said(&f).contains(&more()), "{:?}", said(&f));
    }
}

#[test]
fn a_server_failure_in_the_foot_opens_the_page_for_a_server_or_a_phone() {
    use super::far::Then;
    let mut f = Files::new(vec![std::env::temp_dir()]);
    for (far, page) in [("sftp://jo@example.com/a.txt", SERVER_REFUSED), ("mtp://pixel/internal/a.jpg", PHONE)] {
        f.update(Msg::Far(FarMsg::Fetched { far: PathBuf::from(far), then: Then::Open, got: Err("The server said no.".into()) }));
        assert_eq!((f.said_help, f.bad), (Some(page), true), "{far}");
        assert!(window_has_more(&super::view::view(&f).said()), "{far}");
        f.update(Msg::Help(page));
        assert_eq!(f.helped.last(), Some(&page));
    }
    let pane_words = Files::new(vec![std::env::temp_dir()]);
    assert_eq!(pane_words.said_help, None);
}

#[test]
fn a_refused_name_has_a_more_in_the_foot() {
    let dir = std::env::temp_dir().join(format!("trindle-files-help-name-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.txt"), b"x").unwrap();
    let mut f = Files::new(vec![dir.clone()]);
    f.load(0, 0);
    f.update(Msg::Choose(0, 0));
    f.update(Msg::Rename);
    f.update(Msg::RenameText("a/b".into()));
    f.update(Msg::RenameDone);
    assert_eq!((f.said.as_str(), f.said_help), ("A name can't have a slash in it.", Some(NAME_REFUSED)));
    assert!(window_has_more(&super::view::view(&f).said()));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_folder_that_will_not_open_has_a_more_in_the_pane() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    f.load(0, 0);
    let pane = f.pane_mut(0);
    pane.entries.clear();
    pane.all.clear();
    pane.error = Some("This folder is no longer there.".into());
    assert!(window_has_more(&super::view::view(&f).said()));
}

#[test]
fn a_drop_that_brought_nothing_has_a_more() {
    use super::outside::Onto;
    use crate::toolkit::dnd::{Given, Way, Why as Dropped};
    let mut f = Files::new(vec![std::env::temp_dir()]);
    f.update(Msg::Outside(Onto::Icons(0, 0, 500.0, 300.0), Given::Nothing(Dropped::NotFiles), Way::Copy));
    assert_eq!((f.said_help, f.bad), (Some(DRAG), true));
}

#[test]
fn a_share_problem_leads_to_the_page_for_a_login_or_for_not_opening() {
    use crate::common::smb::Problem::*;
    for p in [NeedsLogin, WrongLogin, Account, Denied] {
        assert_eq!(share_page(&p, false), SHARE_LOGIN, "{p:?}");
    }
    for p in [NoShare, Asleep, NoComputer, Smb1, Nfs4, Missing("smbclient"), NotAllowed("x".into()), Other("x".into())] {
        assert_eq!(share_page(&p, false), SHARE_NOT_OPENED, "{p:?}");
    }
    assert_eq!(share_page(&Denied, true), SHARE_NOT_OPENED);
}

#[test]
fn stopped_work_and_unkept_settings_lines_carry_a_more() {
    use crate::toolkit::widget::note;
    let w: crate::toolkit::widget::Node<Msg> = super::tools::stopped(note("They could not be compared: x.".to_string()));
    assert!(w.said().contains(&more()));
    let w: crate::toolkit::widget::Node<Msg> = super::settings::problem("It could not be saved.".into(), NOT_KEPT);
    assert!(w.said().contains(&more()));
}

#[test]
fn a_file_that_cannot_be_shown_has_a_more_in_the_viewer() {
    use super::preview::{PreviewMsg, Seen};
    let dir = std::env::temp_dir().join(format!("trindle-files-help-viewer-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("broken.pdf"), b"not a pdf").unwrap();
    let mut f = Files::new(vec![dir.clone()]);
    f.load(0, 0);
    let at = f.cols[0].entries.iter().position(|e| e.name == "broken.pdf").expect("listed");
    f.update(Msg::Choose(0, at));
    f.update(Msg::Preview(PreviewMsg::Toggle));
    let Some(Dialog::Preview(pv)) = &f.dialog else { panic!("no viewer") };
    assert!(matches!(&pv.seen, Seen::Trouble(_)), "a file whose name lies is a trouble");
    assert!(super::preview::view(pv).said().contains(&more()));
    let _ = std::fs::remove_dir_all(dir);
}
