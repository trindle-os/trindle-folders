//! Pictures for the Folders page of the help site, each a failure drawn by the

use std::path::{Path, PathBuf};

use super::act::{words, Job};
use super::app::{Files, Msg};
use super::dialogs::{AskMsg, Dialog};
use super::help::*;
use super::menu::Act;
use super::places::Where;
use super::work::{self, Extra, Kind, News, Over};
use crate::toolkit::t;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::{App, Ui};

fn out() -> PathBuf {
    let out = PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/page-shots".into()));
    std::fs::create_dir_all(&out).unwrap();
    out
}

fn snap(name: &str, f: &Files) {
    let (w, h) = (1200u32, 760u32);
    for _ in 0..6 {
        std::thread::sleep(std::time::Duration::from_millis(120));
        let mut ui = Ui::new(super::view::view(f), Theme::new(false, &crate::common::style::accent()));
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out().join(format!("folders-{name}.png"))).unwrap();
    }
}

fn papers(tag: &str) -> (PathBuf, Files) {
    let dir = std::env::temp_dir().join(format!("trindle-page-shots-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let real = std::env::var_os("REAL_HOME").map(PathBuf::from).unwrap_or_else(|| {
        let h = PathBuf::from(std::env::var_os("HOME").unwrap());
        std::env::set_var("REAL_HOME", &h);
        h
    });
    std::fs::create_dir_all(dir.join(".config/gtk-4.0")).unwrap();
    std::fs::create_dir_all(dir.join(".local/share")).unwrap();
    let _ = std::fs::copy(real.join(".config/gtk-4.0/settings.ini"), dir.join(".config/gtk-4.0/settings.ini"));
    let _ = std::os::unix::fs::symlink(real.join(".local/share/icons"), dir.join(".local/share/icons"));
    std::env::set_var("HOME", &dir);
    let papers = dir.join("Papers");
    std::fs::create_dir_all(papers.join("Letters")).unwrap();
    std::fs::create_dir_all(dir.join("Documents")).unwrap();
    for (file, bytes) in [("holiday plans.txt", 1_800), ("notes.txt", 640), ("budget.ods", 48_000), ("report.pdf", 220_000), ("report.odt", 27_500),
                          ("invoice 0042.pdf", 81_000), ("garden plan.odt", 27_500), ("readme.txt", 90)] {
        std::fs::write(papers.join(file), vec![b'x'; bytes]).unwrap();
    }
    let then = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000);
    for path in std::fs::read_dir(&papers).unwrap().flatten().map(|e| e.path()).chain([papers.clone()]) {
        std::fs::File::open(&path).unwrap().set_modified(then).unwrap();
    }
    let mut f = Files::new(vec![papers]);
    f.room = (1200.0, 760.0);
    f.update(Msg::PanesAre(false));
    (dir, f)
}

fn ended_with(f: &mut Files, kind: Kind, items: &[&str], to: &str, failed: &[(&str, &str)]) {
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap();
    let items: Vec<PathBuf> = items.iter().map(PathBuf::from).collect();
    let mut job = Job::sample(1, "x", 300, 300);
    job.words = words(&kind, &items, Path::new(to), &home);
    f.jobs.push(job);
    let failed = failed.iter().map(|(n, why)| work::trouble(n, why)).collect();
    f.update(Msg::Work(1, News::Over(Over { failed, ..Over::default() })));
}

fn say(f: &mut Files, key: &'static str, text: String) {
    let page = MESSAGES.iter().find(|(k, _)| *k == key).unwrap_or_else(|| panic!("{key} is not in the table")).1;
    f.say_help(text, page);
}

#[derive(Default)]
struct Homes(Vec<PathBuf>);

impl Homes {
    fn fresh(&mut self, tag: &str) -> Files {
        let (dir, f) = papers(tag);
        self.0.push(dir);
        f
    }

    fn clean(self) {
        for d in self.0 {
            let _ = std::fs::remove_dir_all(d);
        }
    }
}

fn connect(f: &mut Files, addr: &str, why: super::far::Why, name: &str) {
    use super::far::FarMsg;
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address(addr.into())));
    f.update(Msg::Far(FarMsg::Refused(why)));
    snap(name, f);
    f.dialog = None;
}

#[test]
#[ignore = "draws pictures for the help site; run by hand"]
fn page_shots_archives() {
    use super::archive::{Place, Trouble};
    use super::locked::{Then, UnlockMsg};
    let (dir, mut f) = papers("archives");
    let archive = dir.join("Papers/holiday.zip");
    std::fs::write(&archive, b"made up").unwrap();
    f.update(Msg::Reload);
    f.unlock_ask(archive.clone(), Then::Unpack(Place::Here, dir.join("Papers")), None);
    snap("archive-password-card", &f);
    f.dialog = None;
    f.unlock_ask(archive.clone(), Then::Unpack(Place::Here, dir.join("Papers")), Some(Trouble::WrongKey));
    f.update(Msg::Unlock(UnlockMsg::Password(String::from("john-smith").into())));
    snap("archive-password-wrong", &f);
    f.dialog = None;

    let broken = dir.join("Papers/broken.zip");
    std::fs::write(&broken, b"not a zip").unwrap();
    f.go(0, Where::Inside(broken.clone(), String::new()));
    say(&mut f, "folders-locked-could-not-read",
        t!("folders-locked-could-not-read", name = "broken.zip", why = "it's damaged or incomplete"));
    snap("archive-not-opened", &f);

    let (dir2, mut f) = papers("archives-room");
    ended_with(&mut f, Kind::Other(Extra::Unpack(Place::Here)), &["big.7z"], "Papers", &[("big.7z", t!("folders-archive-trouble-no-room"))]);
    snap("archive-no-room", &f);
    let _ = std::fs::remove_dir_all(dir);
    let _ = std::fs::remove_dir_all(dir2);
}

#[test]
#[ignore = "draws pictures for the help site; run by hand"]
fn page_shots_foot() {
    let (dir, mut f) = papers("foot");
    ended_with(&mut f, Kind::Copy, &["Papers/holiday plans.txt", "Papers/notes.txt"], "Documents",
               &[("holiday plans.txt", "permission was refused"), ("notes.txt", "it changed while it was copied")]);
    snap("work-failed", &f);
    let (_d, mut f) = papers("foot-server");
    ended_with(&mut f, Kind::Copy, &["Papers/report.odt"], "sftp://jo@example.com/docs", &[("report.odt", "permission was refused for report.odt")]);
    snap("server-refused", &f);

    let (_d, mut f) = papers("foot-choice");
    say(&mut f, "folders-menu-columns-not-saved", t!("folders-menu-columns-not-saved").to_string());
    snap("choice-not-kept", &f);
    let (_d, mut f) = papers("foot-admin");
    say(&mut f, "folders-menu-no-admin-window", t!("folders-menu-no-admin-window").to_string());
    snap("program-not-started", &f);
    let (_d, mut f) = papers("foot-terminal");
    say(&mut f, "folders-alone-no-terminal", t!("folders-alone-no-terminal").to_string());
    snap("part-missing", &f);
    let (_d, mut f) = papers("foot-drag");
    say(&mut f, "folders-outside-short-move", t!("folders-outside-short-move").to_string());
    snap("drag-short", &f);
    let (_d, mut f) = papers("foot-share");
    f.say_help(crate::common::smb::Problem::Asleep.words("NAS-HOME", None), SHARE_NOT_OPENED);
    snap("share-not-opened", &f);
    let (_d, mut f) = papers("foot-colours");
    say(&mut f, "folders-act-colours-lost", t!("folders-act-colours-lost").to_string());
    snap("colours-lost", &f);
    let (_d, mut f) = papers("foot-make");
    say(&mut f, "folders-act-make-failed", t!("folders-act-make-failed", base = "New folder", why = "Permission denied (os error 13)"));
    snap("make-failed", &f);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
#[ignore = "draws pictures for the help site; run by hand"]
fn page_shots_places() {
    use super::far::{FarMsg, Question, Why};
    let (dir, mut f) = papers("places");

    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address("sftp://jo@nas.example.com".into())));
    f.update(Msg::Far(FarMsg::Refused(Why::Words(t!("folders-remote-sftp-no-server", host = "nas.example.com"), SERVER_REACH))));
    snap("server-no-answer", &f);
    f.dialog = None;
    f.update(Msg::Far(FarMsg::Open));
    f.update(Msg::Far(FarMsg::Address("sftp://jo@nas.example.com".into())));
    f.update(Msg::Far(FarMsg::Refused(Why::Ask(Question::NewHost("nas.example.com".into())))));
    snap("server-trust", &f);
    f.dialog = None;

    let (_d, mut f) = papers("places-phone");
    f.pane_mut(0).at = Where::Remote("mtp://Pixel/".into());
    f.pane_mut(0).entries.clear();
    f.pane_mut(0).all.clear();
    say(&mut f, "folders-remote-mtp-no-storage", t!("folders-remote-mtp-no-storage").to_string());
    snap("phone-no-storage", &f);

    let (_d, mut f) = papers("places-name");
    let at = f.cols[0].entries.iter().position(|e| e.name == "notes.txt").unwrap();
    f.update(Msg::Choose(0, at));
    f.update(Msg::Pick(0, vec![at]));
    f.update(Msg::Rename);
    f.update(Msg::RenameText("notes/old.txt".into()));
    f.update(Msg::RenameDone);
    snap("name-refused", &f);

    let (dir3, mut f) = papers("places-private");
    let private = dir3.join("Papers/Private");
    std::fs::create_dir_all(&private).unwrap();
    std::fs::set_permissions(&private, std::os::unix::fs::PermissionsExt::from_mode(0o000)).unwrap();
    f.go(0, Where::Folder(private.clone()));
    for _ in 0..50 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        f.update(Msg::Reload);
        if f.cols[0].error.is_some() {
            break;
        }
    }
    snap("folder-not-open", &f);
    std::fs::set_permissions(&private, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    let _ = std::fs::remove_dir_all(dir);
}

fn header_of_a_huge_png() -> Vec<u8> {
    let mut png = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0, 0, 0, 13, b'I', b'H', b'D', b'R'];
    png.extend(12_250u32.to_be_bytes());
    png.extend(12_250u32.to_be_bytes());
    png.extend([8, 2, 0, 0, 0]);
    let crc = {
        let mut c = !0u32;
        for b in &png[12..] {
            c ^= *b as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 { 0xedb88320 ^ (c >> 1) } else { c >> 1 };
            }
        }
        !c
    };
    png.extend(crc.to_be_bytes());
    png.extend([0, 0, 0, 0, b'I', b'D', b'A', b'T', 0x35, 0xaf, 0x06, 0x1e]);
    png
}

#[test]
#[ignore = "draws pictures for the help site; run by hand"]
fn page_shots_windows() {
    use super::shares::Tried;
    use super::tools::{Tool, ToolMsg};
    let (dir, mut f) = papers("windows");
    let papers = dir.join("Papers");

    f.update(Msg::Share(super::shares::ShareMsg::Mounted { t: 0, p: 0, server: "NAS-HOME".into(), share: "Photos".into(), rest: String::new(),
        tried: Tried { address: Some("192.168.1.20".into()), ..Tried::default() }, got: Err(crate::common::smb::Problem::WrongLogin) }));
    snap("share-login-card", &f);
    f.dialog = None;

    f.do_act(Act::CompareFiles, 0, vec![papers.join("notes.txt"), papers.join("readme.txt")]);
    let id = match &f.dialog {
        Some(Dialog::Tool(Tool::Diff(d))) => d.id,
        _ => panic!("no compare window"),
    };
    f.tool_msg(ToolMsg::Diffed(id, Err("readme.txt could not be read".into())));
    snap("compare-stopped", &f);
    f.dialog = None;

    std::fs::create_dir_all(papers.join("Albums")).unwrap();
    f.update(Msg::Reload);
    f.ask(Act::Properties, 0, vec![papers.join("Albums")]);
    if let Some(Dialog::Properties(p)) = &mut f.dialog {
        if let Some(form) = &mut p.share {
            form.problem = Some(t!("folders-sharing-name-taken").into());
        }
        p.page = 1;
    }
    snap("sharing-refused", &f);
    f.dialog = None;

    let beach = papers.join("beach.jpg");
    image::RgbImage::from_fn(64, 48, |x, y| image::Rgb([(x * 4) as u8, (y * 5) as u8, 150])).save(&beach).unwrap();
    f.update(Msg::Reload);
    f.ask(Act::Properties, 0, vec![beach]);
    {
        use super::details::{DetailMsg, Note};
        f.update(Msg::Ask(AskMsg::Detail(DetailMsg::Note(Note::Taken, "2026-01-02 03:04".into()))));
        f.update(Msg::Ask(AskMsg::Detail(DetailMsg::SavePhoto)));
        if let Some(Dialog::Properties(p)) = &mut f.dialog {
            p.page = 1;
        }
    }
    snap("details-refused", &f);
    f.dialog = None;

    let huge = papers.join("holiday.png");
    std::fs::write(&huge, header_of_a_huge_png()).unwrap();
    f.update(Msg::Reload);
    f.do_act(Act::Preview, 0, vec![huge]);
    for _ in 0..40 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        f.update(Msg::Reload);
    }
    snap("file-unread", &f);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
#[ignore = "draws pictures for the help site; run by hand"]
fn page_shots_drive() {
    use crate::common::volumes::{Kind, Volume};
    use super::drives::DriveMsg;
    let (dir, mut f) = papers("drive");
    f.set_drives(std::rc::Rc::default(), |_| {});
    let gb = 1_000_000_000u64;
    let drive = |id: &str, label: &str, at: Option<&str>, kind: Kind, fixed: bool, size: u64| Volume {
        id: id.into(), dev: id.into(), label: label.into(), size: size * gb, drive: format!("/drive/{id}"), drive_name: "Example".into(), kind,
        mounts: at.iter().map(|m| m.to_string()).collect(), locked: false, container: None, can_eject: !fixed, can_power_off: !fixed,
        fixed, system: fixed && at == Some("/"), key: format!("fs:uuid-{id}"), drive_key: format!("drive:Example-{id}"),
    };
    f.computer.reader = Some(std::sync::Arc::new(move |at: &Path| match at.to_str() {
        Some("/") => Some((100 * gb, 400 * gb)),
        _ => None,
    }));
    f.update(Msg::Drive(DriveMsg::Listed(vec![drive("nvme0n1p3", "", Some("/"), Kind::Other, true, 500), drive("sdb1", "HOLIDAY", None, Kind::Usb, false, 32)])));
    f.go(0, Where::Computer);
    f.update(Msg::Drive(DriveMsg::Mounted("sdb1".into(), Err("Not authorized to perform operation".into()))));
    snap("drive-open-failed", &f);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
#[ignore = "draws pictures for the help site; run by hand"]
fn page_shots_more() {
    let mut homes = Homes::default();
    more_work_failures(&mut homes);
    more_place_failures(&mut homes);
    more_share_and_file_failures(&mut homes);
    homes.clean();
}

fn more_work_failures(h: &mut Homes) {
    use super::archive::{Place, Trouble};
    use super::locked::Then;
    let mut f = h.fresh("m-settings");
    f.update(Msg::Own(super::settings::OwnMsg::Open));
    snap("settings", &f);

    let mut f = h.fresh("m-failure");
    ended_with(&mut f, Kind::Move, &["Papers/invoice 0042.pdf"], "Documents", &[("invoice 0042.pdf", work::changed_why())]);
    snap("failure-foot", &f);

    let mut f = h.fresh("m-away");
    f.set_drives(std::rc::Rc::default(), |_| {});
    f.say_help(t!("folders-drives-away-failed", name = "HOLIDAY", why = "Target is busy"), DRIVE);
    snap("failure-drive", &f);

    let mut f = h.fresh("m-pw");
    let zip = papers_dir().join("holiday.zip");
    std::fs::write(&zip, b"made up").unwrap();
    f.update(Msg::Reload);
    f.unlock_ask(zip.clone(), Then::Unpack(Place::Here, papers_dir()), Some(Trouble::WrongOrBad));
    snap("archive-password-or-damaged", &f);

    let mut f = h.fresh("m-notarchive");
    let bad = papers_dir().join("notes.zip");
    std::fs::write(&bad, b"just some words").unwrap();
    f.go(0, Where::Inside(bad, String::new()));
    let text = t!("folders-locked-could-not-read", name = "notes.zip", why = Trouble::Unknown.to_string());
    say(&mut f, "folders-locked-could-not-read", text);
    snap("archive-not-an-archive", &f);

    let mut f = h.fresh("m-nowrite");
    ended_with(&mut f, Kind::Other(Extra::Unpack(Place::Here)), &["big.7z"], "Papers", &[("big.7z", t!("folders-archive-why-read-only"))]);
    snap("archive-no-write", &f);

    let mut f = h.fresh("m-notall");
    ended_with(&mut f, Kind::Copy, &["Papers/Letters"], "Documents", &[("Letters", work::not_all())]);
    snap("work-not-all", &f);

    let mut f = h.fresh("m-notkept");
    {
        let home = std::env::var_os("HOME").map(PathBuf::from).unwrap();
        let items = vec![PathBuf::from("Papers/report.pdf")];
        let mut job = Job::sample(1, "x", 300, 300);
        job.words = words(&Kind::Copy, &items, Path::new("Documents"), &home);
        f.jobs.push(job);
        f.update(Msg::Work(1, News::Over(Over { not_kept: vec!["HOLIDAY".into()], failed: vec![work::trouble("report.pdf", work::not_kept())], ..Over::default() })));
    }
    snap("work-not-kept", &f);

    let mut f = h.fresh("m-views");
    say(&mut f, "folders-app-panes-not-kept", t!("folders-app-panes-not-kept").to_string());
    snap("choice-panes-not-kept", &f);
}

fn more_place_failures(h: &mut Homes) {
    use super::far::{Question, Why};
    use super::shares::Tried;
    let mut f = h.fresh("m-far");
    connect(&mut f, "ftp://jo@nas.example.com", Why::Words(t!("folders-remote-ftp-no-answer", host = "nas.example.com"), SERVER_REACH), "server-did-not-answer");
    connect(&mut f, "sftp://jo@nas.example.com", Why::Ask(Question::Changed("nas.example.com".into())), "server-identity-changed");
    connect(&mut f, "sftp://jo@nas.example.com", Why::Ask(Question::NeedsKey), "server-needs-key");
    connect(&mut f, "ftps://jo@nas.example.com", Why::Words(t!("folders-remote-tls-setup-failed", why = "the server closed the link"), SERVER_REFUSED), "server-secure-failed");
    connect(&mut f, "dav://jo@nas.example.com", Why::Words(t!("folders-remote-dav-full").to_string(), SERVER_REFUSED), "server-full");

    let mut f = h.fresh("m-phone");
    f.pane_mut(0).at = Where::Remote("mtp://Pixel/".into());
    f.pane_mut(0).entries.clear();
    f.pane_mut(0).all.clear();
    say(&mut f, "folders-remote-mod-denied", t!("folders-remote-mod-denied", what = "the phone's USB connection").to_string());
    snap("phone-refused", &f);

    let mut f = h.fresh("m-empty");
    let at = f.cols[0].entries.iter().position(|e| e.name == "notes.txt").unwrap();
    f.update(Msg::Choose(0, at));
    f.update(Msg::Pick(0, vec![at]));
    f.update(Msg::Rename);
    f.update(Msg::RenameText(String::new()));
    f.update(Msg::RenameDone);
    snap("name-empty", &f);

    let mut f = h.fresh("m-login");
    f.update(Msg::Share(super::shares::ShareMsg::Mounted { t: 0, p: 0, server: "NAS-HOME".into(), share: "Photos".into(), rest: String::new(),
        tried: Tried { address: Some("192.168.1.20".into()), ..Tried::default() }, got: Err(crate::common::smb::Problem::NeedsLogin) }));
    snap("share-login-first", &f);

    let mut f = h.fresh("m-noshare");
    f.say_help(crate::common::smb::Problem::NoShare.words("NAS-HOME", Some("Photos")), SHARE_NOT_OPENED);
    snap("share-not-found", &f);
}

fn more_share_and_file_failures(h: &mut Homes) {
    let mut f = h.fresh("m-sharemine");
    let albums = papers_dir().join("Albums");
    std::fs::create_dir_all(&albums).unwrap();
    f.update(Msg::Reload);
    f.ask(Act::Properties, 0, vec![albums]);
    if let Some(Dialog::Properties(p)) = &mut f.dialog {
        if let Some(form) = &mut p.share {
            form.on = false;
            form.cannot = Some(t!("folders-sharing-only-yours").into());
        }
        p.page = 1;
    }
    snap("sharing-not-yours", &f);
    f.dialog = None;

    let mut f = h.fresh("m-pdf");
    std::fs::write(papers_dir().join("broken.pdf"), b"not a pdf").unwrap();
    f.update(Msg::Reload);
    f.do_act(Act::Preview, 0, vec![papers_dir().join("broken.pdf")]);
    for _ in 0..20 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        f.update(Msg::Reload);
    }
    snap("file-broken", &f);

    let mut f = h.fresh("m-unshown");
    say(&mut f, "folders-outside-unshown", t!("folders-outside-unshown").to_string());
    snap("drag-unshown", &f);
}

fn papers_dir() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").unwrap()).join("Papers")
}

#[test]
#[ignore = "draws pictures for the help site; run by hand"]
fn page_shots_extra() {
    let mut homes = Homes::default();
    extra_work_failures(&mut homes);
    extra_place_failures(&mut homes);
    extra_share_and_file_failures(&mut homes);
    homes.clean();
}

fn extra_work_failures(h: &mut Homes) {
    let mut f = h.fresh("x-drag");
    say(&mut f, "folders-outside-short-copy", t!("folders-outside-short-copy").to_string());
    snap("drag-short-copy", &f);

    let mut f = h.fresh("x-7z");
    let bad = papers_dir().join("old.7z");
    std::fs::write(&bad, b"7z made up").unwrap();
    f.go(0, Where::Inside(bad, String::new()));
    let why = t!("folders-archive-damaged-7z", why = "a part ends too soon");
    say(&mut f, "folders-locked-could-not-read", t!("folders-locked-could-not-read", name = "old.7z", why = &why));
    snap("archive-damaged-7z", &f);

    let mut f = h.fresh("x-trash");
    ended_with(&mut f, Kind::Trash, &["Papers/budget.ods"], "Papers", &[("budget.ods", t!("kit-proof-not-to-trash"))]);
    snap("work-no-trash", &f);
}

fn extra_place_failures(h: &mut Homes) {
    use super::far::{Question, Why};
    use super::shares::{ShareMsg, Tried};
    use crate::common::smb::Problem;
    let mut f = h.fresh("x-far");
    connect(&mut f, "sftp://jo@nas.example.com", Why::Words(t!("folders-remote-sftp-lost").to_string(), SERVER_REACH), "server-lost");
    connect(&mut f, "ftps://jo@nas.example.com", Why::Ask(Question::CertChanged { host: "nas.example.com".into(), fingerprint: "AB:CD:12:34:56:78".into() }), "server-cert-changed");
    connect(&mut f, "ftp://jo@nas.example.com", Why::Words(t!("folders-remote-ftp-no-welcome", said = "421 Too many users").to_string(), SERVER_REFUSED), "server-no-welcome");

    let mut f = h.fresh("x-phone");
    f.pane_mut(0).at = Where::Remote("mtp://Pixel/".into());
    f.pane_mut(0).entries.clear();
    f.pane_mut(0).all.clear();
    say(&mut f, "folders-remote-mtp-full", t!("folders-remote-mtp-full").to_string());
    snap("phone-full", &f);

    let mounted = |f: &mut Files, tried: Tried, got: Problem, name: &str| {
        f.update(Msg::Share(ShareMsg::Mounted { t: 0, p: 0, server: "NAS-HOME".into(), share: "Photos".into(), rest: String::new(), tried, got: Err(got) }));
        snap(name, f);
        f.dialog = None;
    };
    let mut f = h.fresh("x-share");
    let at = || Tried { address: Some("192.168.1.20".into()), ..Tried::default() };
    mounted(&mut f, Tried { held: Some(vec!["192.168.1.7".into()]), ..at() }, Problem::NeedsLogin, "share-login-held");
    mounted(&mut f, Tried { keyring: Some(t!("common-keyring-silent").to_string()), ..at() }, Problem::NeedsLogin, "share-keyring");

    let mut f = h.fresh("x-noshare2");
    f.say_help(Problem::Missing("smbclient").words("NAS-HOME", None), SHARE_NOT_OPENED);
    snap("share-program-missing", &f);
}

fn extra_share_and_file_failures(h: &mut Homes) {
    let mut f = h.fresh("x-pw");
    let albums = papers_dir().join("Albums");
    std::fs::create_dir_all(&albums).unwrap();
    f.update(Msg::Reload);
    f.ask(Act::Properties, 0, vec![albums]);
    if let Some(Dialog::Properties(p)) = &mut f.dialog {
        if let Some(form) = &mut p.share {
            form.problem = Some(t!("folders-sharing-password-short", n = 8));
        }
        p.page = 1;
    }
    snap("sharing-password-short", &f);
    f.dialog = None;

    let mut f = h.fresh("x-font");
    std::fs::write(papers_dir().join("broken.ttf"), b"not a font at all").unwrap();
    f.update(Msg::Reload);
    f.do_act(Act::Preview, 0, vec![papers_dir().join("broken.ttf")]);
    for _ in 0..20 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        f.update(Msg::Reload);
    }
    snap("file-font", &f);
}
