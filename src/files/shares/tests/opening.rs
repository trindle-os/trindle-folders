//! A share opens as an ordinary folder.

use crate::files::app::{Files, Msg};
use crate::files::dialogs::Dialog;
use crate::files::places::Where;

use crate::toolkit::ui::App;
use super::*;

#[test]
fn a_share_opens_as_a_folder() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.go(p, Where::Network);
    f.update(Msg::Neighbours(vec![crate::files::neighbours::Computer { name: "nas.invalid".into(), address: Some("10.0.0.2".into()) }]));
    let i = f.cols[p].tabs[0].entries.iter().position(|e| e.name == "nas.invalid").expect("the computer listed");
    f.open(p, i);
    assert!(matches!(&f.cols[p].tabs[0].at, Where::Remote(a) if a == "smb://nas.invalid/"));
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert!(!a.failed && a.share.is_none() && a.remember);
    f.update(Msg::Share(ShareMsg::User("jo".into())));
    f.update(Msg::Share(ShareMsg::Password(String::from("wrong").into())));
    f.update(Msg::Share(ShareMsg::Go));
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("the card closed") };
    assert!(a.failed && a.why.contains("is wrong"), "{}", a.why);
    assert!(a.password.is_empty() && a.user == "jo", "the wrong password is emptied, the name kept");
    f.update(Msg::Share(ShareMsg::Password(String::from("right").into())));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    assert_eq!(names(&f, p), ["Photos", "Public"]);
    let i = names(&f, p).iter().position(|n| n == "Photos").unwrap();
    f.open(p, i);
    let Some(Dialog::Share(a)) = &f.dialog else { panic!("no card: {}", f.said) };
    assert_eq!((a.share.as_deref(), a.user.as_str(), a.password.as_str(), a.address.as_deref()), (Some("Photos"), "jo", "right", Some("10.0.0.2")));
    f.update(Msg::Share(ShareMsg::Go));
    assert!(f.dialog.is_none(), "{}", f.said);
    let at = pretend::folder("Photos");
    assert_eq!(f.cols[p].tabs[0].at, Where::Folder(at.clone()));
    assert_eq!(f.share_places().len(), 1);
    assert_eq!(f.share_places()[0].name(), "Photos on nas.invalid");
    assert!(matches!(f.cols[p].tabs[0].back.last(), Some(Where::Remote(a)) if a == "smb://nas.invalid/"));
    f.go(p, Where::Remote("smb://nas.invalid/Photos".into()));
    assert_eq!(f.cols[p].tabs[0].at, Where::Folder(at.clone()));
    f.update(Msg::Share(ShareMsg::Eject(0)));
    assert!(f.share_places().is_empty(), "{}", f.said);
    assert_eq!(f.cols[p].tabs[0].at, Where::Network);
    assert!(!at.exists());
}
