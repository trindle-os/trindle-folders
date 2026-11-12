//! Shares remembered, pinned and mounted again, and NFS exports.

use std::path::{Path, PathBuf};

use crate::files::app::{Files, Msg};
use crate::files::dialogs::Dialog;
use crate::files::places::Where;
use crate::files::remembered::{self, Remembered};
use crate::files::remote::{Address, Scheme};
use crate::common::smb::{Mounted, Problem};

use crate::toolkit::ui::App;
use super::*;

#[test]
fn an_nfs_address_is_mounted_and_shown_as_a_folder() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.go(p, Where::Remote("nfs://nas.invalid/srv/media".into()));
    assert!(f.dialog.is_none(), "an NFS server has no login to ask for");
    let at = match &f.cols[p].tabs[0].at {
        Where::Folder(at) => at.clone(),
        other => panic!("not shown as a folder: {other:?} {}", f.said),
    };
    assert!(at.ends_with("media on nas.invalid") && at.is_dir());
    assert_eq!(f.share_places().len(), 1);
    assert_eq!(f.share_places()[0].name(), "media on nas.invalid");
    assert!(f.share_places()[0].is_nfs());
    assert!(!matches!(f.cols[p].tabs[0].back.last(), Some(Where::Remote(_))), "Back never leads to the address and straight here again");
    f.go(p, Where::Remote("nfs://nas.invalid/srv/media/".into()));
    assert_eq!(f.share_places().len(), 1);
    assert_eq!(f.cols[p].tabs[0].at, Where::Folder(at.clone()));
    f.update(Msg::Share(ShareMsg::Eject(0)));
    assert!(f.share_places().is_empty(), "{}", f.said);
    assert!(!at.exists());
    let _ = std::fs::remove_dir_all(at.parent().unwrap());
}

#[test]
fn a_remembered_share_keeps_its_pin_and_comes_back_by_reconnect() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.go(p, Where::Remote("nfs://nas.invalid/srv/media".into()));
    let at = f.share_places()[0].at.clone();
    assert_eq!(f.shares.remembered.len(), 1, "{}", f.said);
    assert!(f.down_shares().is_empty());
    f.pinned = vec![at.join("Films")];
    f.update(Msg::Share(ShareMsg::Eject(0)));
    assert_eq!(f.pinned.len(), 1, "the pin is never dropped");
    assert_eq!(f.down_shares(), [0], "pinned, so still remembered");
    assert_eq!(f.pin_down(&f.pinned[0].clone()), Some(0));
    f.update(Msg::Share(ShareMsg::ReconnectAll));
    assert!(f.down_shares().is_empty(), "{}", f.said);
    assert_eq!(f.share_places().len(), 1);
    assert_eq!(f.pinned, [f.share_places()[0].at.join("Films")], "the same pin, found by address");
    f.pinned.clear();
    f.update(Msg::Share(ShareMsg::Eject(0)));
    assert!(f.shares.remembered.is_empty() && f.down_shares().is_empty());
    let _ = std::fs::remove_dir_all(at.parent().unwrap());
}

#[test]
fn a_share_already_mounted_is_remembered() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let at = PathBuf::from("/run/media/john/Media on nas.invalid");
    let up = || vec![Mounted { server: "nas.invalid".into(), share: "Media".into(), at: at.clone() }];
    f.shares.mounted = up();
    f.remember_all_mounted();
    assert_eq!(f.shares.remembered.len(), 1);
    assert_eq!(f.shares.remembered[0].at, at);
    f.shares.remembered.clear();
    f.shares.mounted = up();
    f.shares.typed.insert("nas.invalid".into(), crate::common::smb::Login::typed("john", "secret"));
    f.smb_open(f.side(), &Address::parse("smb://nas.invalid/Media").unwrap());
    assert_eq!(f.shares.remembered.len(), 1, "opening one already mounted remembers it: {}", f.said);
    assert!(f.shares.typed.is_empty(), "a password typed on the card outlived the opening it was for");
}

#[test]
fn a_pin_to_a_missing_share_is_not_connected_with_reconnect_not_a_parent_folder() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let pin = PathBuf::from("/run/media/john/Media on nas.invalid");
    f.pinned = vec![pin.clone(), pin.join("Films")];
    f.shares.mounted.clear();
    f.remember_pinned_shares();
    assert_eq!(f.shares.remembered, [Remembered { server: "nas.invalid".into(), share: "Media".into(), at: pin.clone() }]);
    assert_eq!(f.down_shares(), [0], "offered with its Reconnect");
    assert_eq!(f.pin_down(&pin), Some(0), "shown as not connected");
    assert_eq!(f.pin_down(&pin.join("Films")), Some(0));
    f.remember_pinned_shares();
    assert_eq!(f.shares.remembered.len(), 1, "kept once");
    f.shares.remembered[0].at = "/run/media/john/Media on nas.invalid 2".into();
    assert_eq!(f.pin_down(&pin), Some(0));
    f.update(Msg::Pinned(0));
    if let Where::Folder(at) = &f.pane().at {
        assert!(!(pin.starts_with(at) && at != &pin), "never the folder above: {}", at.display());
    }
    assert!(remembered::from_folder(Path::new("/run/media/john/USB stick")).is_none());
    assert!(remembered::from_folder(Path::new("/home/john/Media on nas")).is_none());
}

#[test]
fn a_pin_inside_an_nfs_export_reopens_at_the_pinned_folder() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.go(p, Where::Remote("nfs://nas.invalid/srv/media".into()));
    let at = f.share_places()[0].at.clone();
    let pin = at.join("Films");
    f.pinned = vec![pin.clone()];
    f.update(Msg::Share(ShareMsg::Eject(0)));
    std::fs::create_dir_all(&pin).unwrap();
    assert!(f.reconnect_pin(&pin));
    assert_eq!(f.cols[p].tabs[f.cols[p].tab].at, Where::Folder(pin), "{}", f.said);
    let _ = std::fs::remove_dir_all(at.parent().unwrap());
}

#[test]
fn reconnecting_goes_past_a_share_that_fails_and_follows_the_setting() {
    let dir = std::env::temp_dir().join(format!("trindle-files-reconnect-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let conf = dir.join("folders.conf");
    let fresh = |conf: &std::path::Path| {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.own_file = Some(conf.to_path_buf());
        f.shares.remembered = ["Photos", "Public"]
            .map(|n| Remembered { server: "nas.invalid".into(), share: n.into(), at: pretend::folder(n) })
            .to_vec();
        f
    };
    let mut f = fresh(&conf);
    f.shares_at_start();
    assert!(f.share_places().is_empty() && f.down_shares() == [0, 1]);
    crate::common::folders_conf::set_in(&conf, crate::common::folders_conf::RECONNECT, "yes").unwrap();
    let mut f = fresh(&conf);
    f.shares_at_start();
    assert!(f.dialog.is_none(), "a quiet reconnect opens no card");
    assert_eq!(f.down_shares(), [0], "{}", f.said);
    assert_eq!(f.share_places().len(), 1);
    assert_eq!(f.share_places()[0].share, "Public");
    let up = f.share_places()[0].at.join("x");
    assert!(!f.reconnect_pin(&up));
    f.update(Msg::Share(ShareMsg::Reconnect(0)));
    assert!(matches!(f.dialog, Some(Dialog::Share(_))), "{}", f.said);
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(pretend::folder("Public").parent().unwrap());
}

#[test]
fn a_remembered_share_with_no_pin_can_be_forgotten() {
    let mut f = Files::new(vec![std::env::temp_dir()]);
    let p = f.side();
    f.go(p, Where::Remote("nfs://nas.invalid/srv/media".into()));
    let at = f.share_places()[0].at.clone();
    f.update(Msg::Share(ShareMsg::ForgetShare(0)));
    assert_eq!(f.shares.remembered.len(), 1, "open, so kept");
    f.pinned = vec![at.join("Films")];
    f.update(Msg::Share(ShareMsg::Eject(0)));
    f.update(Msg::Share(ShareMsg::ForgetShare(0)));
    assert_eq!(f.shares.remembered.len(), 1, "pinned, so kept");
    f.pinned.clear();
    f.shares.remembered.push(Remembered { server: "nas.invalid".into(), share: "Photos".into(), at: pretend::folder("Photos") });
    f.update(Msg::Share(ShareMsg::ForgetShare(1)));
    assert_eq!(f.shares.remembered.len(), 1, "{}", f.said);
    assert!(f.shares.remembered[0].is_nfs(), "the one let go was Photos");
    let _ = std::fs::remove_dir_all(at.parent().unwrap());
}

#[test]
fn an_nfs_address_that_cannot_be_opened_says_why() {
    let words = |address: &str| {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        let p = f.side();
        f.go(p, Where::Remote(address.into()));
        assert!(f.share_places().is_empty() && matches!(f.cols[p].tabs[0].at, Where::Remote(_)), "{address}");
        assert!(f.bad, "{address}: {}", f.said);
        f.said.clone()
    };
    assert_eq!(words("nfs://nas.invalid/srv/other"), "nas.invalid doesn't share a folder called /srv/other.");
    assert!(words("nfs://asleep.invalid/srv/media").contains("didn't respond"));
    assert!(words("nfs://nowhere.invalid/srv/media").contains("Folders couldn't find a computer called nowhere.invalid"));
    assert_eq!(words("nfs://nas.invalid/srv/../etc"), "That isn't a path to a shared folder.");
    assert_eq!(words("nfs://nas.invalid/a%2Cvers=3"), "That isn't a path to a shared folder.");
    assert!(words("nfs://-oopt/srv/media").contains("couldn't read that address"));
    assert_eq!(nfs_words(&Problem::Denied, "nas", "/srv/media"), "This computer isn't allowed to open /srv/media on nas.");
    assert!(nfs_words(&Problem::Missing("nfs-common"), "nas", "/x").contains("nfs-common"));
}

#[test]
fn nfs_asks_the_administrator_every_time() {
    let argv = nfs_mount_argv("/usr/lib/trindle/libexec/trindle-admin", "nas", "/srv/media");
    assert_eq!(argv, ["pkexec", "/usr/lib/trindle/libexec/trindle-admin", "nfs-mount", "nas", "/srv/media"]);
    assert!(Address::parse("nfs://nas/srv/media").is_ok_and(|a| a.scheme == Scheme::Nfs && a.port() == 2049 && !a.scheme.takes_password()));
    assert!(crate::files::far::is_far(Path::new("nfs://nas/srv/media")));
    assert!(include_str!("../../../../packaging/depends.txt").lines().any(|l| l.starts_with("nfs-common ")));
}
