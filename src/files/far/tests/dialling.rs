// When a server is dialled, and where a held login goes.

use std::fs;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;

use crate::files::app::{Files, Msg};
use crate::files::dialogs::Dialog;
use crate::files::places::Where;
use crate::files::leave::Confirmed;
use crate::files::remote::{self, Address, Error, Login};

use super::*;

fn with_password() -> usize {
    DIALLED.with_borrow(|d| d.iter().filter(|d| d.password.is_some()).count())
}

fn card(f: &Files) -> (String, String) {
    match &f.dialog {
        Some(Dialog::Connect(c)) => (view(c, 1).said().join(" "), c.password.clone()),
        _ => panic!("no Connect card: {}", f.said),
    }
}

fn an_address_with_a_password_waits_for_the_card_too(here: &Path, user: &str, p: usize) {
    use crate::toolkit::ui::App;
    let mut g = Files::new(vec![here.to_path_buf()]);
    let went = with_password();
    g.go(p, Where::Remote(format!("ftp://{user}:secret@nas.local/")));
    assert_eq!(with_password(), went, "a password written in an address went to an address only heard with no card");
    let (said, _) = card(&g);
    assert!(said.contains("10.0.0.7") && said.contains("port 21"), "{said}");
    let mut h = Files::new(vec![here.to_path_buf()]);
    h.update(Msg::Far(FarMsg::Open));
    h.update(Msg::Far(FarMsg::Address(format!("ftp://{user}@10.0.0.9"))));
    h.update(Msg::Far(FarMsg::Password("secret".into())));
    h.update(Msg::Far(FarMsg::Go));
    assert!(h.dialog.is_none(), "{}", h.said);
    let went = with_password();
    h.far.links.dropped();
    h.update(Msg::Reload);
    assert!(h.dialog.is_none(), "{}", h.said);
    assert!(with_password() > went);
}

#[test]
fn a_password_waits_for_the_card_at_every_dialling_of_an_address_only_heard() {
    use crate::toolkit::ui::App;
    let (here, far, _) = place("heard");
    let user = far.file_name().unwrap().to_string_lossy().into_owned();
    crate::files::shares::pretend::dns("nas.local", "10.0.0.7");
    let mut f = Files::new(vec![here.clone()]);
    f.update(Msg::Add(Vec::new()));
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(format!("ftp://{user}@nas.local"))));
    f.update(Msg::Far(FarMsg::Password("secret".into())));
    f.update(Msg::Far(FarMsg::Go));
    assert_eq!(with_password(), 0, "the password went to an address only heard, which no card had shown");
    let (said, _) = card(&f);
    assert!(said.contains("nas.local") && said.contains("10.0.0.7") && said.contains("port 21"), "{said}");
    assert!(!said.contains('\u{2014}'), "{said}");
    f.update(Msg::Far(FarMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    let p = f.side();
    assert_eq!(names(&f, p), ["box", "song.txt"]);
    let went = with_password();
    assert_eq!(went, 2);
    assert!(DIALLED.with_borrow(|d| d.iter().all(|d| d.at.as_deref() == Some("10.0.0.7") && d.password.as_deref() == Some("secret"))), "{:?}", DIALLED.with_borrow(Clone::clone));
    let a = Address::parse(&format!("ftp://{user}@nas.local/")).unwrap();
    assert!(f.far.links.with(&a, |r| r.list("/")).is_ok());
    assert_eq!(DIALLED.with_borrow(Vec::len), 2);
    f.far.links.dropped();
    let stopped = f.far.links.with(&a, |r| r.list("/")).map(drop).unwrap_err();
    assert_eq!(stopped.to_string(), remote::heard_words("nas.local", "10.0.0.7"));
    assert_eq!(DIALLED.with_borrow(Vec::len), 2);
    for _ in 0..2 {
        f.far.links.dropped();
        f.update(Msg::Reload);
        assert_eq!(with_password(), went, "the held password went to an address only heard with no card");
        let (said, typed) = card(&f);
        assert!(said.contains("nas.local") && said.contains("10.0.0.7") && said.contains("port 21"), "{said}");
        assert!(typed.is_empty(), "the held password is not put in the box");
        f.update(Msg::Far(FarMsg::Close));
    }
    f.far.links.dropped();
    f.update(Msg::Reload);
    f.update(Msg::Far(FarMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert!(with_password() > went);
    assert_eq!(DIALLED.with_borrow(|d| d.last().cloned()).unwrap().password.as_deref(), Some("secret"));
    assert_eq!(names(&f, p), ["box", "song.txt"]);
    an_address_with_a_password_waits_for_the_card_too(&here, &user, p);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_server_that_is_down_is_dialled_once_by_each_that_wants_it_and_each_is_told() {
    use std::sync::atomic::AtomicUsize;
    let (here, far, _) = place("down");
    let user = far.file_name().unwrap().to_string_lossy().into_owned();
    fake::links_most(&user, 0);
    fake::count_asked(&user);
    let a = Address::parse(&format!("sftp://{user}@test.invalid/")).unwrap();
    let links = Links::default();
    let told = Arc::new(AtomicUsize::new(0));
    for _ in 0..4 {
        let (links, a, told) = (links.clone(), a.clone(), told.clone());
        std::thread::spawn(move || {
            if links.claim(&a).is_err() {
                told.fetch_add(1, Ordering::SeqCst);
            }
        });
    }
    let until = Instant::now() + std::time::Duration::from_secs(3);
    while told.load(Ordering::SeqCst) < 4 && Instant::now() < until {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert_eq!(told.load(Ordering::SeqCst), 4, "work waited on a server that is down, which was dialled {} times meanwhile", fake::asked(&user));
    assert_eq!(fake::asked(&user), 4, "each that wanted a link dials once");
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_server_that_gives_one_link_is_opened_and_worked_with_that_one() {
    let (here, far, _) = place("one-link");
    let user = far.file_name().unwrap().to_string_lossy().into_owned();
    fake::links_most(&user, 1);
    crate::files::shares::pretend::dns("nas.local", "10.0.0.7");
    let a = Address::parse(&format!("ftp://{user}@nas.local/")).unwrap();
    let links = Links::default();
    let login = Login { password: Some("secret".into()), ..Login::default() };
    let opened = links.open(&a, &login, Some(&Confirmed::made_up("10.0.0.7")));
    assert!(opened.is_ok(), "a server that gives one link was refused: {:?}", opened.err().map(|e| e.to_string()));
    let dialled = DIALLED.with_borrow(Vec::len);
    assert!(links.look(&a, |r| r.list("/")).is_ok());
    assert!(links.with(&a, |r| r.list("/")).is_ok());
    assert!(links.with(&a, |r| r.list("/")).is_ok(), "the link was not given back");
    assert_eq!(DIALLED.with_borrow(Vec::len), dialled);
    assert_eq!(fake::links_given(&user), 1);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_held_password_goes_again_only_to_the_address_it_was_let_go_for() {
    let (here, far, _) = place("moved");
    let user = far.file_name().unwrap().to_string_lossy().into_owned();
    let dns = crate::files::shares::pretend::dns;
    dns("files.example.com", "10.0.0.7");
    let a = Address::parse(&format!("ftp://{user}@files.example.com/")).unwrap();
    let links = Links::default();
    links.open(&a, &Login { password: Some("secret".into()), ..Login::default() }, None).unwrap();
    let went = || DIALLED.with_borrow(|d| d.iter().filter(|d| d.password.is_some()).map(|d| d.at.clone().unwrap_or_default()).collect::<Vec<_>>());
    assert_eq!(went(), ["10.0.0.7"]);
    links.dropped();
    assert!(links.look(&a, |r| r.list("/")).is_ok());
    assert_eq!(went(), ["10.0.0.7", "10.0.0.7"]);
    dns("files.example.com", "10.6.6.6");
    links.dropped();
    for _ in 0..2 {
        for refused in [links.look(&a, |r| r.list("/")).map(drop), links.with(&a, |r| r.list("/")).map(drop)] {
            match refused {
                Err(Error::Heard { host, at, port }) => assert_eq!((host.as_str(), at.as_str(), port), ("files.example.com", "10.6.6.6", 21)),
                other => panic!("the card was not asked: {:?}", other.map_err(|e| e.to_string())),
            }
        }
    }
    assert_eq!(went().len(), 2, "the held password went to an address it was never let go for: {:?}", went());
    links.open(&a, &Login::default(), Some(&Confirmed::made_up("10.6.6.6"))).unwrap();
    assert_eq!(went().last().map(String::as_str), Some("10.6.6.6"));
    assert_eq!(DIALLED.with_borrow(|d| d.last().cloned()).unwrap().password.as_deref(), Some("secret"));
    let sent = went().len();
    links.dropped();
    assert!(links.look(&a, |r| r.list("/")).is_ok());
    assert_eq!(went().len(), sent + 1);
    dns("files.example.com", "10.0.0.7");
    links.dropped();
    assert!(matches!(links.look(&a, |r| r.list("/")).map(drop), Err(Error::Heard { .. })));
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_link_is_thrown_away_only_when_it_is_lost_and_not_when_the_server_says_no() {
    let (here, far, root) = place("saidno");
    let a = Address::parse(&root).unwrap();
    let links = Links::default();
    links.open(&a, &Login::default(), None).unwrap();
    let dialled = || DIALLED.with_borrow(|d| d.len());
    let both = |why: fn() -> Error| {
        assert!(links.with(&a, |_| Err::<(), _>(why())).is_err());
        assert!(links.look(&a, |_| Err::<(), _>(why())).is_err());
    };
    let used = || assert!(links.with(&a, |r| r.list("/")).is_ok() && links.look(&a, |r| r.list("/")).is_ok());
    used();
    let had = dialled();
    let said_no: [fn() -> Error; 8] = [
        || Error::Failed("The server will not protect file transfers, so nothing was sent.".into()),
        || Error::NotFound("it".into()),
        || Error::Exists("it".into()),
        || Error::Denied("it".into()),
        || Error::Busy("it".into()),
        || Error::NotSupported("That".into()),
        || Error::Cancelled,
        || Error::LoginRefused,
    ];
    for why in said_no {
        both(why);
        used();
        assert_eq!(dialled(), had, "a good link was thrown away for: {}", why());
    }
    let gone: [fn() -> Error; 3] = [
        || Error::Lost("The connection to the server was lost.".into()),
        || Error::Unreachable("it".into()),
        || Error::Io(std::io::Error::other("it")),
    ];
    for (n, why) in gone.into_iter().enumerate() {
        both(why);
        used();
        assert_eq!(dialled(), had + 2 * (n + 1), "the link that works and the one that reads are both dialled again after: {}", why());
    }
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}
