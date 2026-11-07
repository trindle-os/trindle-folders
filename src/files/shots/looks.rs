// Pictures for judging a look.

use std::path::{Path, PathBuf};

use super::{index_of, pick, shoot, shoot_at, size};
use crate::files::act::Job;
use crate::files::app::{Files, Msg, View};
use crate::files::places::Where;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::{App, Ui};

fn look_papers() -> (PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-look-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("HOME", &dir);
    let papers = dir.join("Papers");
    for sub in ["Albums", "Letters", "Receipts 2026", "Recipes"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for sub in ["Documents", "Downloads", "Pictures"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    for n in 1..=9 {
        std::fs::write(papers.join("Letters").join(format!("letter {n:02}.txt")), vec![b'x'; 2_400]).unwrap();
    }
    let files: [(&str, usize); 16] = [
        ("holiday plans.txt", 1_800), ("notes.txt", 640), ("budget.ods", 48_000), ("accounts 2025.ods", 96_000), ("report.pdf", 220_000),
        ("invoice 0042.pdf", 81_000), ("old letters.tar.gz", 4_200_000), ("song.mp3", 3_900_000), ("film.mkv", 7_400_000), ("shopping.txt", 210),
        ("minutes of the meeting.odt", 31_000), ("garden plan.odt", 27_500), ("passport scan.pdf", 640_000), ("readme.txt", 90),
        ("seed list.txt", 1_100), ("timetable.ods", 12_000),
    ];
    for (file, bytes) in files {
        std::fs::write(papers.join(file), vec![b'x'; bytes]).unwrap();
    }
    for (file, rgb) in [("beach.png", [222, 184, 120]), ("garden.png", [70, 140, 80]), ("hills.png", [90, 110, 170])] {
        image::RgbImage::from_fn(640, 400, |x, y| image::Rgb(if (x / 80 + y / 80) % 2 == 0 { rgb } else { [rgb[0] / 2, rgb[1] / 2, rgb[2] / 2] }))
            .save(papers.join(file)).unwrap();
    }
    let then = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000);
    let mut all = vec![papers.clone(), papers.join("Letters")];
    all.extend([&papers, &papers.join("Letters")].into_iter().flat_map(|d| std::fs::read_dir(d).unwrap().flatten().map(|e| e.path())));
    for path in all {
        std::fs::File::open(&path).unwrap().set_modified(then).unwrap();
    }
    (dir, papers)
}

fn look_drives(f: &mut Files) {
    use crate::common::volumes::{Kind, Volume};
    let gb = 1_000_000_000u64;
    let drive = |id: &str, label: &str, at: Option<&str>, kind: Kind, fixed: bool, size: u64| Volume {
        id: id.into(), dev: id.into(), label: label.into(), size: size * gb, drive: format!("/drive/{id}"), drive_name: "Acme".into(), kind,
        mounts: at.iter().map(|m| m.to_string()).collect(), locked: false, container: None, can_eject: !fixed, can_power_off: !fixed,
        fixed, system: fixed && at == Some("/"), key: String::new(), drive_key: String::new(),
    };
    let listed = vec![
        drive("nvme0n1p3", "", Some("/"), Kind::Other, true, 256), drive("sda1", "Films", None, Kind::Other, true, 2000),
        drive("sdb1", "HOLIDAY", Some("/run/media/john/HOLIDAY"), Kind::Usb, false, 32),
        drive("mmc0p1", "PHOTOS", Some("/run/media/john/PHOTOS"), Kind::Card, false, 64),
    ];
    f.computer.reader = Some(std::sync::Arc::new(move |at: &std::path::Path| match at.to_str() {
        Some("/") => Some((100 * gb, 156 * gb)),
        Some("/run/media/john/HOLIDAY") => Some((28 * gb, 4 * gb)),
        Some("/run/media/john/PHOTOS") => Some((62 * gb, 2 * gb)),
        _ => None,
    }));
    f.update(Msg::Drive(crate::files::drives::DriveMsg::Listed(listed)));
}

#[test]
#[ignore]
fn shoot_look() {
    use crate::files::density::Density;
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    let level = Density::named(&std::env::var("TRINDLE_LOOK_DENSITY").unwrap_or_default());
    crate::toolkit::theme::reread();
    let (dir, papers) = look_papers();
    let mut f = Files::new(vec![papers.clone()]);
    f.room = size();
    f.set_density(level);
    f.pinned = vec![dir.join("Downloads"), dir.join("Documents"), dir.join("Pictures")];
    f.pictures.inline = Some(crate::files::thumbs::Thumbs::at(&dir.join("thumbnails")));
    f.update(Msg::PanesAre(false));
    look_drives(&mut f);
    the_list_view_and_the_others(&mut f, &name);
    dual_with_the_preview_panel(&mut f, &name, &papers);
    f.go(0, Where::Computer);
    shoot(&format!("{name}-3-storage"), &f);
    folders_own_settings(&mut f, &name, &papers);
    std::fs::remove_dir_all(&dir).unwrap();
}

fn the_list_view_and_the_others(f: &mut Files, name: &str) {
    pick(f, 0, "garden plan.odt");
    f.jobs.push(Job::sample(1, "Copying 3 items to Documents", 300, 120));
    shoot(&format!("{name}-1-list"), f);
    f.jobs.clear();
    f.kept.columns = Some(vec![crate::files::columns::Col::Size, crate::files::columns::Col::Date]);
    shoot(&format!("{name}-1b-rich"), f);
    f.kept.columns = None;
    f.update(Msg::Look(View::Icons));
    shoot(&format!("{name}-1c-icons"), f);
    f.update(Msg::Look(View::Compact));
    shoot(&format!("{name}-1d-compact-view"), f);
    f.update(Msg::Look(View::List));
}

fn dual_with_the_preview_panel(f: &mut Files, name: &str, papers: &Path) {
    use crate::files::panel::PanelMsg;
    f.update(Msg::PanesAre(true));
    f.go(1, Where::Folder(papers.join("Letters")));
    f.update(Msg::Panel(PanelMsg::Switch));
    pick(f, 0, "beach.png");
    shoot(&format!("{name}-2-dual-panel"), f);
    f.update(Msg::Panel(PanelMsg::Switch));
    f.update(Msg::PanesAre(false));
}

fn folders_own_settings(f: &mut Files, name: &str, papers: &Path) {
    use crate::files::settings::OwnMsg;
    f.go(0, Where::Folder(papers.to_path_buf()));
    f.update(Msg::Own(OwnMsg::Open));
    for (page, word) in ["opens", "list", "drives"].iter().enumerate() {
        f.update(Msg::Own(OwnMsg::Page(page)));
        shoot(&format!("{name}-4-settings-{word}"), f);
    }
}

fn dual_papers() -> (PathBuf, PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-dual-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("HOME", &dir);
    let (papers, copy) = (dir.join("Papers"), dir.join("Backup"));
    for sub in ["Albums", "Letters", "Receipts 2026", "Recipes"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for sub in ["Documents", "Downloads", "Pictures", "Backup/Albums", "Backup/Letters"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    let files: [(&str, usize); 10] = [
        ("holiday plans.txt", 1_800), ("notes.txt", 640), ("budget.ods", 48_000), ("report.pdf", 220_000), ("invoice 0042.pdf", 81_000),
        ("old letters.tar.gz", 4_200_000), ("song.mp3", 3_900_000), ("letter to John Smith.odt", 31_000), ("garden plan.odt", 27_500),
        ("timetable.ods", 12_000),
    ];
    for (file, bytes) in files {
        std::fs::write(papers.join(file), vec![b'x'; bytes]).unwrap();
    }
    for (file, bytes) in [("holiday plans.txt", 1_800), ("notes.txt", 320), ("budget.ods", 48_000), ("report.pdf", 220_000), ("song.mp3", 3_900_000)] {
        std::fs::write(copy.join(file), vec![b'x'; bytes]).unwrap();
    }
    for (file, rgb) in [("beach.png", [222, 184, 120]), ("garden.png", [70, 140, 80]), ("hills.png", [90, 110, 170])] {
        image::RgbImage::from_fn(640, 400, |x, y| image::Rgb(if (x / 80 + y / 80) % 2 == 0 { rgb } else { [rgb[0] / 2, rgb[1] / 2, rgb[2] / 2] }))
            .save(papers.join(file)).unwrap();
    }
    let then = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000);
    for folder in [&papers, &copy] {
        for path in std::fs::read_dir(folder).unwrap().flatten().map(|e| e.path()) {
            std::fs::File::open(&path).unwrap().set_modified(then).unwrap();
        }
    }
    (dir, papers, copy)
}

#[test]
#[ignore]
fn shoot_dual() {
    use crate::files::density::Density;
    use crate::files::panel::PanelMsg;
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    let level = Density::named(&std::env::var("TRINDLE_LOOK_DENSITY").unwrap_or_default());
    crate::toolkit::theme::reread();
    let (dir, papers, copy) = dual_papers();
    let mut f = Files::new(vec![papers.clone()]);
    f.room = size();
    f.set_density(level);
    f.pinned = vec![dir.join("Downloads"), dir.join("Documents"), dir.join("Pictures")];
    f.pictures.inline = Some(crate::files::thumbs::Thumbs::at(&dir.join("thumbnails")));
    f.update(Msg::PanesAre(true));
    f.go(1, Where::Folder(copy.clone()));
    let both = |f: &mut Files, word: &str| {
        for (side, which) in ["left", "right"].into_iter().enumerate() {
            f.side = side;
            pick(f, side, "budget.ods");
            f.side = side;
            shoot(&format!("{name}-{word}-{which}"), f);
        }
    };
    both(&mut f, "list");
    f.kept.columns = Some(vec![crate::files::columns::Col::Size, crate::files::columns::Col::Date]);
    both(&mut f, "rich");
    f.kept.columns = None;
    for (view, word) in [(View::Icons, "icons"), (View::Compact, "compact")] {
        for side in 0..2 {
            f.side = side;
            f.update(Msg::Look(view));
        }
        both(&mut f, word);
    }
    for side in 0..2 {
        f.side = side;
        f.update(Msg::Look(View::List));
    }
    f.side = 0;
    f.update(Msg::Panel(PanelMsg::Switch));
    pick(&mut f, 0, "beach.png");
    f.jobs.push(Job::sample(1, "Copying 3 items to Backup", 300, 120));
    shoot(&format!("{name}-panel"), &f);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
#[ignore]
fn shoot_network() {
    use crate::files::nearby::Asked;
    use crate::files::neighbours::{Computer, Looked, Way};
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    crate::toolkit::theme::reread();
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-network-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Documents")).unwrap();
    std::env::set_var("HOME", &dir);
    let mut f = Files::new(vec![dir.clone()]);
    f.room = size();
    f.update(Msg::PanesAre(false));
    f.go(0, Where::Network);
    (f.near.finder, f.near.announced) = (Way::Asking, Way::Asking);
    shoot(&format!("{name}-network-1-looking"), &f);
    f.update(Msg::Looked(Looked::Finder(true)));
    f.update(Msg::Looked(Looked::Announced(Asked::Yes)));
    shoot(&format!("{name}-network-2-nothing"), &f);
    f.update(Msg::Looked(Looked::Finder(false)));
    shoot(&format!("{name}-network-3-no-finder"), &f);
    f.update(Msg::PanesAre(true));
    f.side = 0;
    f.room = (820.0, 520.0);
    shoot_at(&format!("{name}-network-4-dual-narrow"), &f, f.room);
    f.room = size();
    f.update(Msg::PanesAre(false));
    f.update(Msg::Announced(vec![Computer { name: "NAS".into(), address: Some("192.168.1.20".into()) }]));
    shoot(&format!("{name}-network-5-found"), &f);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
#[ignore]
fn shoot_theme_folders() {
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-theme-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for folder in ["Papers/Letters", "Papers/Invoices", "Papers/Holiday pictures"] {
        std::fs::create_dir_all(dir.join(folder)).unwrap();
    }
    for (name, size) in [("report 01.txt", 900), ("report 02.txt", 4_200), ("budget.ods", 38_000), ("garden plan.pdf", 512_000),
                         ("meeting notes.md", 2_300), ("holiday.mp4", 1_800_000), ("recipe.txt", 640), ("letter to John Smith.odt", 21_000)] {
        std::fs::write(dir.join("Papers").join(name), vec![b'a'; size]).unwrap();
    }
    let mut f = Files::new(vec![dir.join("Papers")]);
    f.room = size();
    f.update(Msg::PanesAre(false));
    let picked: Vec<usize> = ["budget.ods", "garden plan.pdf", "holiday.mp4"].iter().map(|n| index_of(&f, 0, n)).collect();
    f.update(Msg::Choose(0, picked[0]));
    f.update(Msg::Pick(0, picked));
    f.jobs.push(Job::sample(1, "Copying 3 items to Documents", 300, 120));
    let start = std::time::Instant::now();
    for s in 0..4 {
        f.jobs[0].pace.note(start + std::time::Duration::from_secs(s), s * 24_000_000, 3_000_000_000);
    }
    f.jobs[0].pace.files(1_240, 9_000);
    shoot("theme-folders", &f);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
#[ignore]
fn shoot_rows() {
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-rows-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("HOME", &dir);
    let papers = dir.join("Papers");
    for sub in ["Albums", "Letters", "Recipes"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for (file, bytes) in [("notes.md", 640), ("read me first.md", 2_100), ("letter to John Smith.txt", 1_800), ("shopping.txt", 210), ("budget.ods", 12_000),
                          ("report.pdf", 220_000), ("minutes of the meeting.odt", 31_000), ("song.mp3", 3_900_000), ("old letters.tar.gz", 4_200_000)] {
        std::fs::write(papers.join(file), vec![b'x'; bytes]).unwrap();
    }
    image::RgbImage::from_pixel(64, 40, image::Rgb([222, 184, 120])).save(papers.join("beach.png")).unwrap();
    let then = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000);
    for e in std::fs::read_dir(&papers).unwrap().flatten() {
        std::fs::File::open(e.path()).unwrap().set_modified(then).unwrap();
    }
    for (light, look) in [(false, "dark"), (true, "light")] {
        if light {
            std::env::set_var("TRINDLE_SHOT_LIGHT", "1");
        } else {
            std::env::remove_var("TRINDLE_SHOT_LIGHT");
        }
        for (room, wide) in [((1200.0, 760.0), "wide"), ((620.0, 760.0), "narrow")] {
            let mut f = Files::new(vec![papers.clone()]);
            f.room = room;
            f.update(Msg::PanesAre(false));
            f.view = View::List;
            shoot_at(&format!("rows-{name}-{look}-{wide}"), &f, room);
        }
    }
    std::env::remove_var("TRINDLE_SHOT_LIGHT");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn folders_draws_only_pairs_the_table_holds() {
    use crate::files::panel::PanelMsg;
    use crate::toolkit::theme::drawn;
    let dir = std::env::temp_dir().join(format!("trindle-files-pairs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let papers = dir.join("Papers");
    for sub in ["Albums", "Letters", "Receipts 2026", "Recipes"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for (file, bytes) in [("holiday plans.txt", 1_800), ("budget.ods", 48_000), ("report.pdf", 220_000), ("letter to John Smith.odt", 21_000),
                          ("song.mp3", 3_900), (".hidden notes.txt", 40)] {
        std::fs::write(papers.join(file), vec![b'x'; bytes]).unwrap();
    }
    let paint = |f: &Files, name: &str| {
        drawn::page(name);
        let mut pix = tiny_skia::Pixmap::new(1200, 760).unwrap();
        for _ in 0..2 {
            Ui::new(crate::files::view::view(f), Theme::chosen()).paint(&mut pix.as_mut(), 1.0);
        }
    };
    let faults = drawn::in_every_look("Folders", 300, || {
        let mut f = Files::new(vec![papers.clone()]);
        f.room = (1200.0, 760.0);
        f.pinned = vec![papers.join("Albums"), papers.join("Letters")];
        f.update(Msg::PanesAre(false));
        let picked: Vec<usize> = ["Letters", "budget.ods", "report.pdf"].iter().map(|n| index_of(&f, 0, n)).collect();
        f.update(Msg::Choose(0, picked[0]));
        f.update(Msg::Pick(0, picked));
        f.jobs.push(Job::sample(1, "Copying 3 items to Documents", 300, 120));
        for (view, name) in [(View::List, "list"), (View::Icons, "icons"), (View::Compact, "compact")] {
            f.update(Msg::Look(view));
            paint(&f, name);
        }
        f.clip = Some(crate::files::act::Clip { paths: ["song.mp3", "Recipes", "report.pdf"].iter().map(|n| papers.join(n)).collect(), cut: true });
        for (view, name) in [(View::List, "list with cut items"), (View::Icons, "icons with cut items"), (View::Compact, "compact with cut items")] {
            f.update(Msg::Look(view));
            paint(&f, name);
        }
        f.clip = None;
        f.update(Msg::Look(View::List));
        f.update(Msg::Panel(PanelMsg::Switch));
        paint(&f, "preview panel");
    });
    let _ = std::fs::remove_dir_all(&dir);
    drawn::none(faults);
}

#[test]
#[ignore]
fn shoot_folder_colour() {
    use crate::files::panel::PanelMsg;
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    let out = PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-c".into()));
    std::fs::create_dir_all(&out).unwrap();
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-folder-colour-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("HOME", &dir);
    let papers = dir.join("Papers");
    for sub in ["Albums", "Letters", "Receipts 2026", "Recipes"] {
        std::fs::create_dir_all(papers.join(sub)).unwrap();
    }
    for sub in ["Documents", "Downloads", "Pictures"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    for (file, bytes) in [("holiday plans.txt", 1_800), ("budget.ods", 48_000), ("report.pdf", 220_000), ("letter to John Smith.odt", 21_000),
                          ("song.mp3", 3_900_000), ("old letters.tar.gz", 4_200_000)] {
        std::fs::write(papers.join(file), vec![b'x'; bytes]).unwrap();
    }
    let then = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000);
    for path in std::fs::read_dir(&papers).unwrap().flatten().map(|e| e.path()) {
        std::fs::File::open(&path).unwrap().set_modified(then).unwrap();
    }
    let mut f = Files::new(vec![papers.clone()]);
    f.room = size();
    f.pinned = vec![dir.join("Downloads"), dir.join("Documents"), dir.join("Pictures")];
    f.update(Msg::PanesAre(false));
    pick(&mut f, 0, "Letters");
    let snap = |f: &Files, view: &str| {
        for (colour, accent) in [("yellow", crate::common::style::DEFAULT_ACCENT), ("blue", "#2295f1")] {
            let mut pix = tiny_skia::Pixmap::new(size().0 as u32, size().1 as u32).unwrap();
            for _ in 0..2 {
                Ui::new(crate::files::view::view(f), Theme::new(false, accent)).paint(&mut pix.as_mut(), 1.0);
            }
            pix.save_png(out.join(format!("{name}-{colour}-{view}.png"))).unwrap();
        }
    };
    snap(&f, "1-list");
    f.update(Msg::Look(View::Icons));
    snap(&f, "2-icons");
    f.update(Msg::Look(View::Compact));
    snap(&f, "3-compact");
    f.update(Msg::Look(View::List));
    f.update(Msg::Panel(PanelMsg::Switch));
    snap(&f, "4-preview-panel");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
#[ignore]
fn shoot_hidden_drive() {
    use crate::common::volumes::{unhidden, Kind, Volume};
    use std::collections::BTreeSet;
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-hidden-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Documents")).unwrap();
    std::env::set_var("HOME", &dir);
    let mut f = Files::new(vec![dir.clone()]);
    f.room = size();
    f.set_drives(std::rc::Rc::default(), |_| {});
    let gb = 1_000_000_000u64;
    let drive = |id: &str, label: &str, at: Option<&str>, kind: Kind, fixed: bool, size: u64| Volume {
        id: id.into(), dev: id.into(), label: label.into(), size: size * gb, drive: format!("/drive/{id}"), drive_name: "Example".into(), kind,
        mounts: at.iter().map(|m| m.to_string()).collect(), locked: false, container: None, can_eject: !fixed, can_power_off: !fixed,
        fixed, system: fixed && at == Some("/"), key: format!("fs:uuid-{id}"), drive_key: format!("drive:Example-{id}"),
    };
    let listed = vec![
        drive("nvme0n1p3", "", Some("/"), Kind::Other, true, 500), drive("nvme1n1p3", "other_system", None, Kind::Other, true, 1997),
        drive("sdb1", "HOLIDAY", Some("/run/media/john/HOLIDAY"), Kind::Usb, false, 32),
        drive("sdc1", "BACKUP", None, Kind::Usb, false, 500),
    ];
    f.computer.reader = Some(std::sync::Arc::new(move |at: &std::path::Path| match at.to_str() {
        Some("/") => Some((100 * gb, 400 * gb)),
        Some("/run/media/john/HOLIDAY") => Some((20 * gb, 12 * gb)),
        _ => None,
    }));
    f.update(Msg::Drive(crate::files::drives::DriveMsg::Listed(unhidden(listed.clone(), &BTreeSet::new()))));
    f.go(0, Where::Computer);
    shoot("folders-drives-shown", &f);
    let hidden = BTreeSet::from(["drive:Example-nvme1n1p3".to_string(), "drive:Example-sdc1".to_string()]);
    f.update(Msg::Drive(crate::files::drives::DriveMsg::Listed(unhidden(listed, &hidden))));
    shoot("folders-drives-hidden", &f);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
#[ignore]
fn shoot_foot() {
    let name = std::env::var("TRINDLE_LOOK").unwrap_or_else(|_| "now".into());
    crate::toolkit::theme::reread();
    let dir = std::env::temp_dir().join(format!("trindle-files-shoot-foot-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::env::set_var("HOME", &dir);
    let papers = dir.join("Papers");
    std::fs::create_dir_all(papers.join("Letters")).unwrap();
    std::fs::create_dir_all(dir.join("Documents")).unwrap();
    for (file, bytes) in [("holiday plans.txt", 1_800), ("notes.txt", 640), ("budget.ods", 48_000), ("report.pdf", 220_000),
                          ("invoice 0042.pdf", 81_000), ("song.mp3", 3_900_000), ("garden plan.odt", 27_500), ("readme.txt", 90)] {
        std::fs::write(papers.join(file), vec![b'x'; bytes]).unwrap();
    }
    let then = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000);
    for path in std::fs::read_dir(&papers).unwrap().flatten().map(|e| e.path()).chain([papers.clone()]) {
        std::fs::File::open(&path).unwrap().set_modified(then).unwrap();
    }
    let mut f = Files::new(vec![papers.clone()]);
    f.room = size();
    f.update(Msg::PanesAre(false));
    let going = |f: &mut Files| {
        let mut job = Job::sample(1, "Copying 3 items to Documents", 300, 120);
        job.pane = 1;
        f.jobs.push(job);
    };
    let ended = |f: &mut Files| f.update(Msg::Work(1, crate::files::work::News::Over(crate::files::work::Over::default())));
    let idle = |f: &mut Files| f.update(Msg::WorkGone(1));
    going(&mut f);
    shoot(&format!("{name}-1-one-pane-copy-going"), &f);
    ended(&mut f);
    shoot(&format!("{name}-2-one-pane-copy-complete"), &f);
    idle(&mut f);
    shoot(&format!("{name}-3-one-pane-idle"), &f);
    f.update(Msg::PanesAre(true));
    f.go(1, Where::Folder(dir.join("Documents")));
    f.side = 0;
    going(&mut f);
    shoot(&format!("{name}-4-two-panes-copy-going"), &f);
    ended(&mut f);
    shoot(&format!("{name}-5-two-panes-copy-complete"), &f);
    idle(&mut f);
    shoot(&format!("{name}-6-two-panes-idle"), &f);
    let _ = std::fs::remove_dir_all(&dir);
}
