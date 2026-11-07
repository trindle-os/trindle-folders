// How a name is shown when file endings are hidden: without its last ending (".pdf"), except where leaving it out would mislead.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use super::far;
use super::places::Entry;

const HEAD: usize = 16;
const INNER_MOST: usize = 6;

pub fn ending_at(name: &str, folder: bool) -> Option<usize> {
    if folder || name.starts_with('.') {
        return None;
    }
    let dot = name.rfind('.').filter(|&i| i > 0 && i + 1 < name.len())?;
    let stem = &name[..dot];
    let inner = stem.rfind('.').filter(|&i| i > 0).map(|i| &stem[i + 1..]);
    if inner.is_some_and(|e| !e.is_empty() && e.chars().count() <= INNER_MOST && e.chars().all(|c| c.is_ascii_alphanumeric())) {
        return None;
    }
    Some(dot)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    Program,
    Pdf,
    Png,
    Jpeg,
    Gif,
    Zip,
    Gzip,
    SevenZip,
    Rar,
}

pub fn start_of(head: &[u8]) -> Option<Start> {
    const STARTS: [(&[u8], Start); 12] = [
        (b"\x7fELF", Start::Program), (b"MZ", Start::Program), (b"#!", Start::Program),
        (b"%PDF-", Start::Pdf), (b"\x89PNG\r\n\x1a\n", Start::Png), (b"\xff\xd8\xff", Start::Jpeg),
        (b"GIF87a", Start::Gif), (b"GIF89a", Start::Gif), (b"PK\x03\x04", Start::Zip),
        (b"\x1f\x8b", Start::Gzip), (b"7z\xbc\xaf\x27\x1c", Start::SevenZip), (b"Rar!\x1a\x07", Start::Rar),
    ];
    STARTS.iter().find(|(magic, _)| head.starts_with(magic)).map(|(_, s)| *s)
}

fn promised(ending: &str) -> Option<Start> {
    Some(match ending.to_ascii_lowercase().as_str() {
        "pdf" => Start::Pdf,
        "png" => Start::Png,
        "jpg" | "jpeg" => Start::Jpeg,
        "gif" => Start::Gif,
        "zip" => Start::Zip,
        "gz" | "tgz" => Start::Gzip,
        "7z" => Start::SevenZip,
        "rar" => Start::Rar,
        _ => return None,
    })
}

const PROGRAM_ENDINGS: [&str; 22] = ["exe", "dll", "com", "msi", "scr", "sys", "so", "bin", "run", "appimage", "elf", "o", "ko",
                                     "sh", "bash", "zsh", "fish", "py", "pl", "rb", "js", "awk"];

pub fn misleads(ending: &str, head: &[u8]) -> bool {
    if head.is_empty() {
        return false;
    }
    let start = start_of(head);
    if start == Some(Start::Program) {
        return !PROGRAM_ENDINGS.contains(&ending.to_ascii_lowercase().as_str());
    }
    promised(ending).is_some_and(|p| start != Some(p))
}

#[derive(Default)]
pub struct Shown {
    kept: RefCell<HashMap<PathBuf, (i64, bool)>>,
}

impl Shown {
    pub fn name<'a>(&self, e: &'a Entry, endings: bool) -> &'a str {
        match self.hidden_from(e, endings) {
            Some(at) => &e.name[..at],
            None => &e.name,
        }
    }

    pub fn hidden_from(&self, e: &Entry, endings: bool) -> Option<usize> {
        if endings {
            return None;
        }
        let at = ending_at(&e.name, e.folder)?;
        let known = self.kept.borrow().get(e.path.as_path()).filter(|(when, _)| *when == e.when).map(|(_, keep)| *keep);
        let keep = known.or_else(|| {
            let keep = must_show(e, &e.name[at + 1..])?;
            self.kept.borrow_mut().insert(e.path.clone(), (e.when, keep));
            Some(keep)
        }).unwrap_or(true);
        (!keep).then_some(at)
    }

    pub fn forget(&self) {
        self.kept.borrow_mut().clear();
    }
}

fn must_show(e: &Entry, ending: &str) -> Option<bool> {
    if far::is_far(&e.path) {
        return Some(false);
    }
    let (path, ending) = (e.path.clone(), ending.to_string());
    super::reach::within(&e.path, move || {
        let Ok(meta) = std::fs::metadata(&path) else { return false };
        if !meta.is_file() {
            return false;
        }
        if meta.mode() & 0o111 != 0 {
            return true;
        }
        let mut head = [0u8; HEAD];
        let n = crate::common::files::open_file(&path).and_then(|mut f| f.read(&mut head)).unwrap_or(0);
        misleads(&ending, &head[..n])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_ending_is_hidden_only_where_the_name_alone_allows() {
        fn hidden(name: &str) -> Option<&str> {
            ending_at(name, false).map(|at| &name[..at])
        }
        assert_eq!(hidden("report.pdf"), Some("report"));
        assert_eq!(hidden("Holiday plans.TXT"), Some("Holiday plans"));
        assert_eq!(hidden("report v1.2 final.pdf"), Some("report v1.2 final"));
        assert_eq!(hidden("photo.jpg.exe"), None);
        assert_eq!(hidden("a.tar.gz"), None);
        assert_eq!(hidden(".bashrc"), None);
        assert_eq!(hidden(".config.bak"), None);
        assert_eq!(hidden("README"), None);
        assert_eq!(hidden("odd."), None);
        assert_eq!(ending_at("my.folder", true), None);
    }

    #[test]
    fn a_file_that_is_not_what_its_ending_says_misleads() {
        assert!(misleads("jpg", b"\x7fELF\x02\x01\x01"));
        assert!(misleads("txt", b"#!/bin/sh\n"));
        assert!(misleads("pdf", b"MZ\x90\x00"));
        assert!(!misleads("sh", b"#!/bin/sh\n"));
        assert!(!misleads("exe", b"MZ\x90\x00"));
        assert!(misleads("pdf", b"plain words"));
        assert!(misleads("jpg", b"\x89PNG\r\n\x1a\n"));
        assert!(!misleads("pdf", b"%PDF-1.7"));
        assert!(!misleads("JPG", b"\xff\xd8\xff\xe0"));
        assert!(!misleads("docx", b"PK\x03\x04"));
        assert!(!misleads("txt", b"plain words"));
        assert!(!misleads("pdf", b""));
    }

    #[test]
    fn the_file_itself_can_keep_its_ending_on_show() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("trindle-files-shown-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let make = |name: &str, bytes: &[u8], run: bool| {
            let path = dir.join(name);
            std::fs::write(&path, bytes).unwrap();
            if run {
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
            }
            Entry { name: name.into(), path, folder: false, size: bytes.len() as u64, when: 1, kind: None }
        };
        let shown = Shown::default();
        let honest = make("report.pdf", b"%PDF-1.7 made up", false);
        assert_eq!(shown.name(&honest, true), "report.pdf");
        assert_eq!(shown.name(&honest, false), "report");
        assert_eq!(shown.name(&make("tool.txt", b"made up", true), false), "tool.txt");
        assert_eq!(shown.name(&make("invoice.pdf", b"\x7fELF made up", false), false), "invoice.pdf");
        assert_eq!(shown.name(&make("cat.jpg", b"GIF89a made up", false), false), "cat.jpg");
        assert_eq!(shown.name(&make("notes.txt", b"made up", false), false), "notes");
        assert_eq!(shown.name(&make("empty.pdf", b"", false), false), "empty");
        let folder = Entry { name: "my.folder".into(), path: dir.join("my.folder"), folder: true, size: 0, when: 1, kind: None };
        assert_eq!(shown.name(&folder, false), "my.folder");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
