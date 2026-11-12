//! The ring's favourites: the apps round its middle and the folders and files pinned beside them, kept one per line in a text file.

#[cfg(test)]
use super::apps::Entry;
use std::path::{Path, PathBuf};

use super::files::{config_path, read_text, write_text};

pub const MAX: usize = 12;

pub const STARTING: &[&str] = &["trindle-folders.desktop", "trindle-drives.desktop"];

pub const HEADER: &str = "# Trindle ring favourites, in order round the ring from twelve o'clock.";

const RENAMED: &[(&str, &str)] = &[
    ("trindle-disks.desktop", "trindle-drives.desktop"),
    ("trindle-store.desktop", "trindle-apps.desktop"),
    ("trindle-files.desktop", "trindle-folders.desktop"),
];

pub fn renamed(id: &str) -> &str {
    RENAMED.iter().find(|(old, _)| *old == id).map_or(id, |(_, new)| new)
}

#[cfg_attr(test, allow(dead_code))]
pub fn file() -> PathBuf {
    config_path("favourites")
}

pub fn from_text(text: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for line in text.lines() {
        let line = renamed(line.trim());
        if !line.is_empty() && !line.starts_with('#') && !ids.iter().any(|i| i == line) {
            ids.push(line.to_string());
        }
    }
    if !text.lines().any(|l| l.starts_with("# Trindle ring favo")) {
        for (n, p) in STARTING.iter().enumerate() {
            if !ids.iter().any(|i| i == p) {
                ids.insert(n.min(ids.len()), p.to_string());
            }
        }
    }
    ids
}

pub fn read(file: &Path) -> Vec<String> {
    from_text(&read_text(file))
}

fn read_kept(file: &Path) -> std::io::Result<Vec<String>> {
    Ok(from_text(&crate::common::files::read_small(file)?))
}

pub fn write(file: &Path, ids: &[String]) -> std::io::Result<()> {
    let mut text = format!("{HEADER}\n");
    for id in ids {
        text.push_str(id);
        text.push('\n');
    }
    write_text(file, &text)
}

pub fn is_path(id: &str) -> bool {
    id.starts_with('/')
}

pub fn line_of(path: &Path) -> Option<String> {
    let line = path.to_str()?;
    (is_path(line) && !line.contains(['\n', '\r']) && line.trim() == line).then(|| line.to_string())
}

#[cfg_attr(test, allow(dead_code))]
pub fn ring_there(file: &Path) -> bool {
    file.is_file() || super::run::find("trindle-ring").is_some()
}

pub fn has(file: &Path, path: &Path) -> bool {
    line_of(path).is_some_and(|line| read(file).contains(&line))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin {
    Pinned,
    Already,
    Full,
    Unkept,
}

pub fn pin(file: &Path, path: &Path) -> std::io::Result<Pin> {
    let Some(line) = line_of(path) else { return Ok(Pin::Unkept) };
    let mut ids = read_kept(file)?;
    if ids.contains(&line) {
        return Ok(Pin::Already);
    }
    if ids.len() >= MAX {
        return Ok(Pin::Full);
    }
    ids.push(line);
    write(file, &ids)?;
    Ok(Pin::Pinned)
}

pub fn unpin(file: &Path, path: &Path) -> std::io::Result<bool> {
    let Some(line) = line_of(path) else { return Ok(false) };
    let mut ids = read_kept(file)?;
    let before = ids.len();
    ids.retain(|i| *i != line);
    if ids.len() == before {
        return Ok(false);
    }
    write(file, &ids)?;
    Ok(true)
}

#[cfg(test)]
pub fn unpin_app(file: &Path, id: &str) -> std::io::Result<bool> {
    let mut ids = read_kept(file)?;
    let before = ids.len();
    ids.retain(|i| i != id);
    if ids.len() == before {
        return Ok(false);
    }
    write(file, &ids)?;
    Ok(true)
}

#[cfg(test)]
pub fn path_entry(path: &Path, icon: Option<String>) -> Entry {
    let name = path.file_name().map_or_else(|| path.to_string_lossy().into_owned(), |n| n.to_string_lossy().into_owned());
    Entry {
        id: path.to_string_lossy().into_owned(), name, icon, exec: String::new(), kinds: Vec::new(), path: path.to_path_buf(),
        flatpak: None, appimage: None, terminal: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-favourites-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_folder_and_a_file_are_pinned_beside_the_apps_and_taken_off_again() {
        let dir = scratch("pin");
        let file = dir.join("favourites");
        std::fs::write(&file, format!("{HEADER}\nfirefox-esr.desktop\n")).unwrap();
        let (photos, letter) = (dir.join("Photos"), dir.join("letter to John Smith.odt"));
        assert!(!has(&file, &photos));
        assert_eq!(pin(&file, &photos).unwrap(), Pin::Pinned);
        assert_eq!(pin(&file, &letter).unwrap(), Pin::Pinned);
        assert_eq!(pin(&file, &photos).unwrap(), Pin::Already);
        assert!(has(&file, &photos) && has(&file, &letter));
        assert_eq!(read(&file), ["firefox-esr.desktop".to_string(), photos.to_string_lossy().into_owned(), letter.to_string_lossy().into_owned()]);
        assert!(read_text(&file).starts_with(HEADER));
        assert!(is_path(&read(&file)[1]) && !is_path("firefox-esr.desktop"));
        assert!(unpin(&file, &photos).unwrap());
        assert!(!unpin(&file, &photos).unwrap());
        assert_eq!(read(&file).len(), 2);
        let before = read_text(&file);
        assert_eq!(pin(&file, Path::new("/made/up/two\nlines")).unwrap(), Pin::Unkept);
        assert_eq!(pin(&file, Path::new("not/a/whole/path")).unwrap(), Pin::Unkept);
        assert_eq!(read_text(&file), before);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn an_app_taken_off_twice_is_not_a_fault() {
        let dir = scratch("twice");
        let file = dir.join("favourites");
        write(&file, &["krita.desktop".to_string(), "obsidian.desktop".to_string()]).unwrap();
        assert!(unpin_app(&file, "obsidian.desktop").unwrap());
        assert!(!unpin_app(&file, "obsidian.desktop").unwrap());
        assert_eq!(read(&file), ["krita.desktop".to_string()]);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_middle_of_the_ring_holds_twelve_and_no_more() {
        let dir = scratch("full");
        let file = dir.join("favourites");
        let ids: Vec<String> = (0..MAX - 1).map(|n| format!("app{n}.desktop")).collect();
        write(&file, &ids).unwrap();
        assert_eq!(pin(&file, &dir.join("one")).unwrap(), Pin::Pinned);
        assert_eq!(pin(&file, &dir.join("two")).unwrap(), Pin::Full);
        assert_eq!(read(&file).len(), MAX);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_person_with_no_list_yet_keeps_the_starting_favourites_when_something_is_pinned() {
        let dir = scratch("first");
        let file = dir.join("favourites");
        assert_eq!(pin(&file, &dir.join("Photos")).unwrap(), Pin::Pinned);
        let ids = read(&file);
        assert_eq!(&ids[..STARTING.len()], STARTING);
        assert_eq!(ids.len(), STARTING.len() + 1);
        assert!(unpin(&file, &dir.join("Photos")).unwrap());
        assert_eq!(read(&file), STARTING);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_path_shows_in_the_ring_under_its_name_and_in_no_piece_of_all_apps() {
        let e = path_entry(Path::new("/home/john/Photos"), Some("folder-symbolic".into()));
        assert_eq!((e.name.as_str(), e.id.as_str()), ("Photos", "/home/john/Photos"));
        assert!(is_path(&e.id) && e.exec.is_empty());
        assert!(e.icons().contains(&"folder-symbolic".to_string()));
    }
}
