//! What Files and the file chooser both need: media types and their icons,

use std::collections::HashMap;
use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

pub use crate::toolkit::common::file_work::address::{file_uri, uri_path};

use crate::common::clock::Local;
use crate::common::files::data_home;
use crate::toolkit::t;

#[derive(Default)]
pub struct Mime {
    endings: HashMap<String, (u32, String)>,
    others: Vec<(u32, String, String, bool)>,
    parents: HashMap<String, Vec<String>>,
    aliases: HashMap<String, String>,
}

type Lists = [String; 3];

fn lists_in(mime: &Path) -> Lists {
    ["globs2", "subclasses", "aliases"].map(|name| crate::common::files::read_string(mime.join(name)).unwrap_or_default())
}

const ELSEWHERE_WAIT: std::time::Duration = std::time::Duration::from_millis(40);

static ELSEWHERE_READS: crate::common::run::Slots = crate::common::run::Slots::new(2);

impl Mime {
    pub fn load() -> Mime {
        let mut dirs = vec![data_home()];
        let shared = std::env::var("XDG_DATA_DIRS").ok().filter(|s| !s.is_empty())
            .unwrap_or_else(|| "/usr/local/share:/usr/share".into());
        dirs.extend(shared.split(':').map(PathBuf::from));
        Mime::load_from(dirs, crate::common::files::own_place, lists_in, ELSEWHERE_WAIT)
    }

    fn load_from(dirs: Vec<PathBuf>, own: fn(&Path) -> bool, read: fn(&Path) -> Lists, wait: std::time::Duration) -> Mime {
        let dirs: Vec<PathBuf> = dirs.iter().map(|d| d.join("mime")).collect();
        let here = |mime: &PathBuf| ["globs2", "subclasses", "aliases"].iter().all(|name| own(&mime.join(name)));
        let mut lists: Vec<Option<Lists>> = dirs.iter().map(|mime| here(mime).then(|| read(mime))).collect();
        let elsewhere: Vec<(usize, PathBuf)> = dirs.iter().enumerate().filter(|(n, _)| lists[*n].is_none()).map(|(n, d)| (n, d.clone())).collect();
        if !elsewhere.is_empty() {
            if let Some(place) = ELSEWHERE_READS.take() {
                let (back, came) = std::sync::mpsc::channel();
                std::thread::spawn(move || {
                    let _place = place;
                    let found: Vec<(usize, Lists)> = elsewhere.into_iter().map(|(n, mime)| (n, read(&mime))).collect();
                    let _ = back.send(found);
                });
                for (n, read) in came.recv_timeout(wait).unwrap_or_default() {
                    lists[n] = Some(read);
                }
            }
        }
        let mut mime = Mime::default();
        for [globs, subclasses, aliases] in lists.iter().rev().flatten() {
            mime.add(globs, subclasses, aliases);
        }
        mime
    }

    pub(crate) fn add(&mut self, globs: &str, subclasses: &str, aliases: &str) {
        for line in globs.lines().filter(|l| !l.starts_with('#')) {
            let mut bits = line.splitn(4, ':');
            let (Some(weight), Some(kind), Some(glob)) = (bits.next(), bits.next(), bits.next()) else { continue };
            let Ok(weight) = weight.parse::<u32>() else { continue };
            let cs = bits.next().is_some_and(|f| f.split(',').any(|f| f == "cs"));
            match glob.strip_prefix("*.") {
                Some(end) if !cs && !end.contains(['*', '?', '[']) => {
                    let end = end.to_lowercase();
                    if self.endings.get(&end).is_none_or(|(w, _)| *w < weight) {
                        self.endings.insert(end, (weight, kind.to_string()));
                    }
                }
                _ => self.others.push((weight, glob.to_string(), kind.to_string(), cs)),
            }
        }
        for line in subclasses.lines() {
            if let Some((kind, parent)) = line.split_once(' ') {
                self.parents.entry(kind.to_string()).or_default().push(parent.to_string());
            }
        }
        for line in aliases.lines() {
            if let Some((alias, kind)) = line.split_once(' ') {
                self.aliases.insert(alias.to_string(), kind.to_string());
            }
        }
    }

    pub fn of_path(&self, path: &Path) -> String {
        self.of(&path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default())
            .unwrap_or_else(|| if reads_as_text(path) { "text/plain".into() } else { "application/octet-stream".into() })
    }

    pub fn of(&self, name: &str) -> Option<String> {
        let lower = name.to_lowercase();
        let mut best: Option<(u32, usize, &str)> = None;
        for (at, _) in lower.match_indices('.') {
            if let Some((w, kind)) = self.endings.get(&lower[at + 1..]) {
                let len = lower.len() - at + 1;
                if best.is_none_or(|(bw, bl, _)| (*w, len) > (bw, bl)) {
                    best = Some((*w, len, kind));
                }
            }
        }
        for (w, glob, kind, cs) in &self.others {
            let hit = if *cs { glob_matches(glob, name) } else { glob_matches(&glob.to_lowercase(), &lower) };
            if hit && best.is_none_or(|(bw, bl, _)| (*w, glob.len()) > (bw, bl)) {
                best = Some((*w, glob.len(), kind));
            }
        }
        best.map(|(_, _, k)| k.to_string())
    }

    pub fn is_a(&self, kind: &str, wanted: &str) -> bool {
        let wanted = self.aliases.get(wanted).map_or(wanted, |s| s.as_str());
        let kind = self.aliases.get(kind).map_or(kind, |s| s.as_str());
        if let Some(group) = wanted.strip_suffix("/*") {
            return kind.split('/').next() == Some(group);
        }
        if wanted == "text/plain" && kind.starts_with("text/") {
            return true;
        }
        let (mut seen, mut todo) = (std::collections::HashSet::new(), vec![kind.to_string()]);
        while let Some(k) = todo.pop() {
            if k == wanted {
                return true;
            }
            if seen.insert(k.clone()) {
                todo.extend(self.parents.get(&k).into_iter().flatten().cloned());
            }
        }
        false
    }
}

pub fn reads_as_text(path: &Path) -> bool {
    use std::io::Read;
    let Ok(file) = crate::common::files::open_file(path) else { return false };
    let mut head = Vec::new();
    if file.take(8192).read_to_end(&mut head).is_err() {
        return false;
    }
    looks_like_text(&head)
}

fn looks_like_text(head: &[u8]) -> bool {
    if head.contains(&0) {
        return false;
    }
    match std::str::from_utf8(head) {
        Ok(_) => true,
        Err(e) => e.error_len().is_none() && head.len() - e.valid_up_to() < 4,
    }
}

pub fn glob_matches(glob: &str, name: &str) -> bool {
    let (Ok(g), Ok(n)) = (CString::new(glob), CString::new(name)) else { return false };
    unsafe { libc::fnmatch(g.as_ptr(), n.as_ptr(), 0) == 0 }
}

pub fn icon_for(kind: Option<&str>) -> &'static str {
    let Some(kind) = kind else { return "text-x-generic-symbolic" };
    match kind.split('/').next().unwrap_or("") {
        "image" => "image-x-generic-symbolic",
        "audio" => "audio-x-generic-symbolic",
        "video" => "video-x-generic-symbolic",
        _ if kind == "application/pdf" || kind.contains("document") || kind.contains("opendocument") => "x-office-document-symbolic",
        _ if ["zip", "tar", "compress", "archive", "x-7z", "rar", "xz", "bzip", "gzip", "zstd", "debian-package", "efi.iso", "cd-image", "iso9660-image", "x-cpio", "x-rpm", "x-lha", "x-xar", "x-lz"]
            .iter().any(|w| kind.contains(w)) => {
            "package-x-generic-symbolic"
        }
        _ => "text-x-generic-symbolic",
    }
}

pub fn kind_name(mime: &str) -> String {
    let text = crate::common::files::read_string(Path::new("/usr/share/mime").join(format!("{mime}.xml"))).unwrap_or_default();
    match text.split_once("<comment>").and_then(|(_, r)| r.split_once("</comment>")) {
        Some((words, _)) if !words.trim().is_empty() => words.trim().to_string(),
        _ => mime.to_string(),
    }
}

pub fn recent(xbel: &str) -> Vec<PathBuf> {
    let mut found: Vec<(String, PathBuf)> = Vec::new();
    for tag in xbel.split("<bookmark ").skip(1) {
        let tag = &tag[..tag.find('>').unwrap_or(tag.len())];
        let attr = |name: &str| {
            let at = tag.find(&format!(" {name}=\"")).or_else(|| tag.starts_with(&format!("{name}=\"")).then_some(0))?;
            let from = tag[at..].find('"')? + at + 1;
            let len = tag[from..].find('"')?;
            Some(tag[from..from + len].replace("&amp;", "&").replace("&apos;", "'").replace("&quot;", "\""))
        };
        let Some(path) = attr("href").as_deref().and_then(uri_path) else { continue };
        let when = attr("modified").or_else(|| attr("visited")).unwrap_or_default();
        found.retain(|(_, p)| *p != path);
        found.push((when, path));
    }
    found.sort_by(|a, b| b.0.cmp(&a.0));
    found.into_iter().map(|(_, p)| p).collect()
}

pub fn add_recent(xbel: &str, path: &Path, kind: &str, now: &str) -> String {
    let href = file_uri(path);
    let start = format!("<bookmark href=\"{href}\"");
    if let Some(at) = xbel.find(&start) {
        let end = xbel[at..].find('>').map_or(xbel.len(), |e| at + e);
        let mut tag = xbel[at..end].to_string();
        for name in ["modified", "visited"] {
            let key = format!(" {name}=\"");
            match tag.find(&key) {
                Some(k) => {
                    let from = k + key.len();
                    let len = tag[from..].find('"').unwrap_or(tag.len() - from);
                    tag.replace_range(from..from + len, now);
                }
                None => tag.push_str(&format!("{key}{now}\"")),
            }
        }
        return format!("{}{tag}{}", &xbel[..at], &xbel[end..]);
    }
    let mark = format!(
        "  <bookmark href=\"{href}\" added=\"{now}\" modified=\"{now}\" visited=\"{now}\">\n    <info>\n      \
         <metadata owner=\"http://freedesktop.org\">\n        <mime:mime-type type=\"{kind}\"/>\n        \
         <bookmark:applications>\n          <bookmark:application name=\"Folders\" exec=\"&apos;xdg-open %u&apos;\" \
         modified=\"{now}\" count=\"1\"/>\n        </bookmark:applications>\n      </metadata>\n    </info>\n  </bookmark>\n"
    );
    match xbel.rfind("</xbel>") {
        Some(end) => format!("{}{mark}{}", &xbel[..end], &xbel[end..]),
        None => format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<xbel version=\"1.0\"\n      \
             xmlns:bookmark=\"http://www.freedesktop.org/standards/desktop-bookmarks\"\n      \
             xmlns:mime=\"http://www.freedesktop.org/standards/shared-mime-info\"\n>\n{mark}</xbel>\n"
        ),
    }
}

pub fn shown_name(path: &Path) -> String {
    path.file_name().map_or_else(|| path.to_string_lossy().into_owned(), |n| n.to_string_lossy().into_owned())
}

pub fn parts(folder: &Path, home: &Path) -> Vec<(String, PathBuf)> {
    let (mut out, rest) = match folder.strip_prefix(home) {
        Ok(rest) if home != Path::new("/") => (vec![(t!("folders-kinds-home").to_string(), home.to_path_buf())], rest),
        _ => (vec![(crate::files::places::root_name().to_string(), PathBuf::from("/"))], folder.strip_prefix("/").unwrap_or(folder)),
    };
    let mut at = out[0].1.clone();
    for part in rest.components() {
        at.push(part);
        out.push((part.as_os_str().to_string_lossy().into_owned(), at.clone()));
    }
    out
}

pub fn when_text(secs: i64) -> String {
    if secs <= 0 {
        return String::new();
    }
    let at = Local::at(secs);
    let now = Local::at(std::time::SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64));
    if at.format("%F") == now.format("%F") { at.line(trindle_toolkit::lang::format::Pattern::Clock) } else { at.line(trindle_toolkit::lang::format::Pattern::DayShortYear) }
}

pub fn said_kind(folder: bool, kind: impl FnOnce() -> String) -> String {
    if folder { String::new() } else { kind() }
}

pub fn ending_of(name: &str) -> Option<&str> {
    let at = name.rfind('.').filter(|at| *at > 0)?;
    let end = &name[at + 1..];
    (!end.is_empty() && end.len() <= 12 && end.chars().all(|c| c.is_ascii_alphanumeric())).then_some(end)
}

pub fn said_kind_beside(folder: bool, name: &str, kind: impl FnOnce() -> String) -> String {
    if folder || ending_of(name).is_some() { String::new() } else { kind() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_with_no_known_name_is_judged_by_what_is_in_it() {
        assert!(looks_like_text(b""));
        assert!(looks_like_text(b"token=abc\nmore\n"));
        assert!(looks_like_text("caf\u{e9}".as_bytes()));
        assert!(looks_like_text(&"\u{e9}".as_bytes()[..1]));
        assert!(!looks_like_text(b"\x7fELF\x02\x01\x01\0\0"));
        assert!(!looks_like_text(&[0xff, 0xfe, 0xfd, 0x80, 0x81]));
        let dir = std::env::temp_dir().join(format!("trindle-kinds-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mime = Mime::default();
        for (name, body, kind) in [("fetch-sample", &b""[..], "text/plain"), ("notes", b"hello\n", "text/plain"),
                                   ("blob", b"\0\x01\x02", "application/octet-stream")] {
            let file = dir.join(name);
            std::fs::write(&file, body).unwrap();
            assert_eq!(mime.of_path(&file), kind, "{name}");
        }
        assert_eq!(mime.of_path(&dir.join("not-there")), "application/octet-stream");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn every_open_in_folders_goes_through_open_file() {
        let files = [("app.rs", crate::common::source::code(crate::files::app::SOURCE)),
                     ("menu.rs", crate::common::source::code(crate::files::menu::SOURCE)),
                     ("inside.rs", crate::common::source::code(include_str!("inside.rs"))),
                     ("earlier.rs", crate::common::source::code(include_str!("earlier.rs"))),
                     ("far", crate::common::source::code_all(crate::files::far::SOURCE)),
                     ("dialogs.rs", crate::common::source::code(include_str!("dialogs.rs")))];
        for (name, code) in files {
            assert!(!code.contains("\"xdg-open\""), "{name} opens a file without open_file, so one with no known kind opens in nothing");
        }
    }

    fn made_up(mime: &Path) -> Lists {
        let dir = mime.parent().unwrap().file_name().unwrap().to_string_lossy().into_owned();
        if dir.contains("slow") {
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
        [format!("50:application/x-{dir}:*.{dir}\n"), String::new(), String::new()]
    }

    fn own(path: &Path) -> bool {
        path.starts_with("/here")
    }

    #[test]
    fn the_kinds_of_files_are_never_waited_for_long_from_a_folder_kept_elsewhere() {
        let wait = std::time::Duration::from_millis(500);
        let began = std::time::Instant::now();
        let all = Mime::load_from(vec!["/far/quick".into(), "/here/own".into()], own, made_up, wait);
        assert!(began.elapsed() < wait, "a folder that answered at once was waited for");
        assert_eq!(all.of("a.quick").as_deref(), Some("application/x-quick"));
        assert_eq!(all.of("a.own").as_deref(), Some("application/x-own"));
        let began = std::time::Instant::now();
        let some = Mime::load_from(vec!["/far/slow".into(), "/here/own".into()], own, made_up, std::time::Duration::from_millis(60));
        assert!(began.elapsed() < std::time::Duration::from_secs(2), "the start waited on a folder that had stopped answering");
        assert_eq!(some.of("a.own").as_deref(), Some("application/x-own"));
        assert_eq!(some.of("a.slow"), None);
        let code = crate::common::source::code(include_str!("kinds.rs"));
        let code = code.split("\n#[cfg(test)]").next().unwrap();
        assert!(code.contains("here(mime).then(|| read(mime))"));
        assert_eq!(code.matches("read(mime)").count(), 1);
        assert_eq!(code.matches("read(&mime)").count(), 1, "the lists kept elsewhere are read in one place, on the thread");
        assert!(code.contains("Mime::load_from(dirs, crate::common::files::own_place, lists_in, ELSEWHERE_WAIT)"));
    }
}
