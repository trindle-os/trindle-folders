// What a pane shows: the places, what is in them and in which order, and the folders the person pinned.

use std::cmp::Ordering;
use std::ffi::OsString;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use super::kinds::{add_recent, icon_for, parts, recent, shown_name, Mime};
use super::remote::{self, Address};
use crate::common::files::{config_path, data_home};
use crate::toolkit::t;

const RECENT: usize = 100;
const CRUMBS: usize = 4;
const NETWORK: &[&str] = &["nfs", "nfs4", "cifs", "smb3", "smbfs", "fuse.sshfs", "sshfs", "davfs", "fuse.rclone", "afpfs", "9p"];

pub fn root_name() -> &'static str {
    t!("folders-places-root-name")
}

pub fn page_name() -> &'static str {
    t!("folders-places-page-name")
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Where {
    Folder(PathBuf),
    Recent,
    Trash,
    Network,
    Computer,
    Remote(String),
    Inside(PathBuf, String),
    Found(PathBuf, String, bool),
}

impl Where {
    pub fn folder(&self) -> Option<&Path> {
        match self {
            Where::Folder(f) => Some(f),
            _ => None,
        }
    }

    pub fn name(&self, home: &Path) -> String {
        match self {
            Where::Folder(f) if f == home => t!("folders-places-home").into(),
            Where::Folder(f) if f == Path::new("/") => root_name().into(),
            Where::Folder(f) => shown_name(f),
            Where::Recent => t!("folders-places-recent").into(),
            Where::Trash => t!("folders-places-trash").into(),
            Where::Network => t!("folders-places-network").into(),
            Where::Computer => page_name().into(),
            Where::Remote(a) => remote_name(a),
            Where::Inside(a, f) if f.is_empty() => shown_name(a),
            Where::Inside(_, f) => remote::name_of(f).to_string(),
            Where::Found(_, text, false) if text.is_empty() => t!("folders-places-found").into(),
            Where::Found(_, text, false) => t!("folders-places-found-text", text = text),
            Where::Found(_, text, true) => t!("folders-places-found-inside", text = text),
        }
    }

    pub fn up(&self) -> Option<Where> {
        if let Where::Remote(a) = self {
            let addr = Address::parse(a).ok()?;
            if top(&addr.path) && addr.scheme == remote::Scheme::Smb {
                return Some(Where::Network);
            }
            return (!top(&addr.path)).then(|| Where::Remote(addr.parent().to_string()));
        }
        if let Where::Inside(a, f) = self {
            if f.is_empty() {
                return a.parent().map(|p| Where::Folder(p.to_path_buf()));
            }
            let up = f.rsplit_once('/').map(|(u, _)| u).unwrap_or("");
            return Some(Where::Inside(a.clone(), up.to_string()));
        }
        if let Where::Found(from, ..) = self {
            return Some(Where::Folder(from.clone()));
        }
        let folder = self.folder()?;
        Some(folder.parent().map_or(Where::Computer, |p| Where::Folder(p.to_path_buf())))
    }
}

fn top(path: &str) -> bool {
    path.is_empty() || path == "/"
}

fn remote_name(a: &str) -> String {
    let Ok(addr) = Address::parse(a) else { return a.to_string() };
    if top(&addr.path) {
        return addr.host;
    }
    remote::name_of(&addr.path).to_string()
}

pub fn crumbs(at: &Where, home: &Path) -> Vec<(String, Where)> {
    let mut all: Vec<(String, Where)> = match at {
        Where::Folder(f) => folder_crumbs(f, home),
        Where::Remote(a) => remote_crumbs(a),
        Where::Inside(a, f) => inside_crumbs(a, f, home),
        Where::Found(from, ..) => {
            let mut out = folder_crumbs(from, home);
            out.push((at.name(home), at.clone()));
            out
        }
        _ => return vec![(at.name(home), at.clone())],
    };
    if all.len() > CRUMBS + 1 {
        all.drain(1..all.len() - CRUMBS);
    }
    all
}

fn folder_crumbs(folder: &Path, home: &Path) -> Vec<(String, Where)> {
    let mut out: Vec<(String, Where)> = parts(folder, home).into_iter().map(|(n, p)| (n, Where::Folder(p))).collect();
    if out.first().is_some_and(|(_, at)| at.folder() == Some(Path::new("/"))) {
        out[0].0 = root_name().to_string();
        out.insert(0, (Where::Computer.name(home), Where::Computer));
    }
    out
}

fn inside_crumbs(a: &Path, f: &str, home: &Path) -> Vec<(String, Where)> {
    let mut out = a.parent().map(|p| folder_crumbs(p, home)).unwrap_or_default();
    out.push((shown_name(a), Where::Inside(a.to_path_buf(), String::new())));
    let mut at = String::new();
    for part in f.split('/').filter(|p| !p.is_empty()) {
        if !at.is_empty() {
            at.push('/');
        }
        at.push_str(part);
        out.push((part.to_string(), Where::Inside(a.to_path_buf(), at.clone())));
    }
    out
}

fn remote_crumbs(a: &str) -> Vec<(String, Where)> {
    let Ok(addr) = Address::parse(a) else { return vec![(a.to_string(), Where::Remote(a.to_string()))] };
    let mut at = Address { path: "/".into(), ..addr.clone() };
    let mut out = vec![(addr.host.clone(), Where::Remote(at.to_string()))];
    for part in addr.path.split('/').filter(|p| !p.is_empty()) {
        at = at.join(part);
        out.push((part.to_string(), Where::Remote(at.to_string())));
    }
    out
}

pub fn among<'a>(entries: &[Entry], paths: impl IntoIterator<Item = &'a PathBuf>) -> Vec<usize> {
    let paths: std::collections::HashSet<&Path> = paths.into_iter().map(PathBuf::as_path).collect();
    entries.iter().enumerate().filter(|(_, e)| paths.contains(e.path.as_path())).map(|(i, _)| i).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub folder: bool,
    pub size: u64,
    pub when: i64,
    pub kind: Option<String>,
}

impl Entry {
    pub fn read(path: PathBuf, mime: &Mime) -> Option<Entry> {
        let name = shown_name(&path);
        let meta = std::fs::metadata(&path).or_else(|_| std::fs::symlink_metadata(&path)).ok()?;
        let when = meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs() as i64);
        let folder = meta.is_dir();
        let kind = if folder { None } else { mime.of(&name) };
        Some(Entry { name, path, folder, size: if folder { 0 } else { meta.len() }, when, kind })
    }

    pub fn icon(&self) -> &'static str {
        if super::neighbours::is_computer(&self.path) {
            return super::icons::stand_in::SERVER;
        }
        if self.folder { "folder-symbolic" } else { icon_for(self.kind.as_deref()) }
    }

    pub fn folder_icon(&self) -> bool {
        self.folder && !super::neighbours::is_computer(&self.path)
    }

    fn hidden(&self) -> bool {
        self.name.starts_with('.')
    }

    pub fn arriving(&self) -> bool {
        self.name.ends_with(crate::common::files::SPARE_END)
    }

    fn sort_type(&self) -> String {
        self.kind.clone().unwrap_or_else(|| {
            self.name.rsplit_once('.').filter(|(a, _)| !a.is_empty()).map(|(_, e)| e.to_lowercase()).unwrap_or_default()
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Name,
    Size,
    Date,
    Type,
}

pub fn sort(entries: &mut [Entry], key: Key, up: bool) {
    entries.sort_by(|a, b| {
        let by = match key {
            Key::Name => Ordering::Equal,
            Key::Size => a.size.cmp(&b.size),
            Key::Date => a.when.cmp(&b.when),
            Key::Type => a.sort_type().cmp(&b.sort_type()),
        };
        let by = by.then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())).then_with(|| a.name.cmp(&b.name));
        b.folder.cmp(&a.folder).then(if up { by } else { by.reverse() })
    });
}

pub fn recent_file() -> PathBuf {
    data_home().join("recently-used.xbel")
}

pub fn note_recent(file: &Path, kind: &str) -> std::io::Result<()> {
    note_recent_in(&recent_file(), file, kind)
}

const RECENT_MOST: u64 = 64 << 20;

pub fn note_recent_in(to: &Path, file: &Path, kind: &str) -> std::io::Result<()> {
    let now = std::time::SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
    let was = crate::common::files::read_within(to, RECENT_MOST)?;
    let text = add_recent(&was, file, kind, &crate::common::clock::utc_iso(now));
    crate::common::files::write_whole(to, text.as_bytes())
}

pub fn unread_words(e: &std::io::Error) -> String {
    match e.kind() {
        std::io::ErrorKind::PermissionDenied => t!("folders-places-unread-denied").into(),
        std::io::ErrorKind::NotFound => t!("folders-places-unread-gone").into(),
        _ => t!("folders-places-unread-other").into(),
    }
}

pub fn list(at: &Where, hidden: bool, key: Key, up: bool, mime: &Mime) -> Vec<Entry> {
    let folder = |f: &Path| -> Vec<Entry> {
        std::fs::read_dir(f).map(|rd| rd.flatten().filter_map(|d| Entry::read(d.path(), mime)).collect()).unwrap_or_default()
    };
    let mut out = match at {
        Where::Folder(f) => folder(f),
        Where::Trash => super::trash::all_files().iter().flat_map(|f| folder(f)).collect(),
        Where::Recent => {
            let text = crate::common::files::read_string(recent_file()).unwrap_or_default();
            recent(&text).into_iter().filter(|p| p.exists()).take(RECENT).filter_map(|p| Entry::read(p, mime)).collect()
        }
        Where::Network => {
            let mounts = std::fs::read_to_string("/proc/self/mounts").unwrap_or_default();
            network(&mounts).into_iter().filter_map(|p| Entry::read(p, mime)).collect()
        }
        Where::Computer => Vec::new(),
        Where::Remote(_) => Vec::new(),
        Where::Inside(..) => Vec::new(),
        Where::Found(..) => Vec::new(),
    };
    arrange(&mut out, at, hidden, key, up);
    out
}

pub fn arrange(out: &mut Vec<Entry>, at: &Where, hidden: bool, key: Key, up: bool) {
    out.retain(|e| hidden || !e.hidden());
    if *at != Where::Recent {
        sort(out, key, up);
    }
}

pub fn network(mounts: &str) -> Vec<PathBuf> {
    mounts.lines().filter_map(|line| {
        let mut bits = line.split(' ');
        let at = bits.nth(1)?;
        let kind = bits.next()?;
        NETWORK.contains(&kind).then(|| super::trash::mount_point(at))
    }).collect()
}

pub fn run(a: usize, b: usize) -> Vec<usize> {
    (a.min(b)..=a.max(b)).collect()
}

#[cfg_attr(test, allow(dead_code))]
pub fn pins_file() -> PathBuf {
    config_path("folders-pinned")
}

pub fn first_pins(home: &Path) -> Vec<PathBuf> {
    ["Documents", "Downloads", "Music", "Pictures", "Videos", "Projects"].iter().map(|n| home.join(n)).filter(|p| p.is_dir()).collect()
}

pub fn load_pins(file: &Path, home: &Path) -> Vec<PathBuf> {
    if file.exists() {
        return read_pins(file);
    }
    let first = first_pins(home);
    let _ = write_pins(file, &first);
    first
}

pub fn read_pins(file: &Path) -> Vec<PathBuf> {
    let bytes = crate::common::files::read_file(file).unwrap_or_default();
    bytes.split(|&b| b == b'\n').filter(|l| !l.iter().all(u8::is_ascii_whitespace))
        .map(|l| PathBuf::from(OsString::from_vec(l.to_vec()))).collect()
}

pub fn write_pins(file: &Path, pins: &[PathBuf]) -> std::io::Result<()> {
    if let Some(dir) = file.parent() {
        std::fs::create_dir_all(dir)?;
    }
    crate::common::files::may_rewrite(file)?;
    let mut out = Vec::new();
    for p in pins.iter().map(|p| p.as_os_str().as_bytes()).filter(|b| !b.contains(&b'\n')) {
        out.extend_from_slice(p);
        out.push(b'\n');
    }
    crate::common::files::write_whole(file, &out)
}

pub fn names_beside(pins: &Path) -> PathBuf {
    let mut name = pins.as_os_str().to_owned();
    name.push("-names");
    PathBuf::from(name)
}

pub fn clean_pin_name(text: &str) -> String {
    text.chars().map(|c| if c.is_control() { ' ' } else { c }).take(80).collect::<String>().trim().to_string()
}

pub fn read_names(file: &Path) -> std::collections::HashMap<PathBuf, String> {
    let bytes = crate::common::files::read_file(file).unwrap_or_default();
    let mut out = std::collections::HashMap::new();
    for line in bytes.split(|&b| b == b'\n') {
        let Some(tab) = line.iter().position(|&b| b == b'\t') else { continue };
        let name = clean_pin_name(&String::from_utf8_lossy(&line[..tab]));
        let path = &line[tab + 1..];
        if !name.is_empty() && !path.is_empty() {
            out.insert(PathBuf::from(OsString::from_vec(path.to_vec())), name);
        }
    }
    out
}

pub fn write_names(file: &Path, names: &std::collections::HashMap<PathBuf, String>, pins: &[PathBuf]) -> std::io::Result<()> {
    let mut out = Vec::new();
    for p in pins.iter().filter(|p| !p.as_os_str().as_bytes().contains(&b'\n')) {
        if let Some(name) = names.get(p).map(|n| clean_pin_name(n)).filter(|n| !n.is_empty()) {
            out.extend_from_slice(name.as_bytes());
            out.push(b'\t');
            out.extend_from_slice(p.as_os_str().as_bytes());
            out.push(b'\n');
        }
    }
    if out.is_empty() && !file.exists() {
        return Ok(());
    }
    if let Some(dir) = file.parent() {
        std::fs::create_dir_all(dir)?;
    }
    crate::common::files::may_rewrite(file)?;
    crate::common::files::write_whole(file, &out)
}

pub fn mirrored(there: &Path, from: &Path, to: &Path) -> Option<PathBuf> {
    let mut common = from.to_path_buf();
    while !to.starts_with(&common) {
        common = common.parent()?.to_path_buf();
    }
    let mut at = there.to_path_buf();
    for up in from.strip_prefix(&common).ok()?.components().rev() {
        if at.file_name() != Some(up.as_os_str()) {
            return None;
        }
        at = at.parent()?.to_path_buf();
    }
    let at = at.join(to.strip_prefix(&common).ok()?);
    super::reach::is_dir(&at).then_some(at)
}

pub fn typed_place(text: &str, here: &Path, home: &Path) -> Option<(Where, Option<PathBuf>)> {
    let text = text.trim();
    let named = |name: &str| text.strip_prefix(name).is_some_and(|rest| rest.trim_start_matches('/').is_empty());
    if text.is_empty() {
        return None;
    }
    if named("recent:") {
        return Some((Where::Recent, None));
    }
    if named("trash:") {
        return Some((Where::Trash, None));
    }
    if named("computer:") {
        return Some((Where::Computer, None));
    }
    if named("network:") {
        return Some((Where::Network, None));
    }
    if super::far::is_far(Path::new(text)) {
        return Some((Where::Remote(text.to_string()), None));
    }
    let path = if let Some(p) = super::kinds::uri_path(text) {
        p
    } else if text == "~" {
        home.to_path_buf()
    } else if let Some(rest) = text.strip_prefix("~/") {
        home.join(rest)
    } else {
        here.join(text)
    };
    let mut tidy = PathBuf::from("/");
    for part in path.components() {
        match part {
            std::path::Component::ParentDir => {
                tidy.pop();
            }
            std::path::Component::Normal(n) => tidy.push(n),
            _ => {}
        }
    }
    if super::reach::is_dir(&tidy) {
        Some((Where::Folder(tidy), None))
    } else if super::reach::meta(&tidy, true).is_some() {
        let folder = tidy.parent()?.to_path_buf();
        Some((Where::Folder(folder), Some(tidy)))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_names_of_places_follow_the_language_and_fall_back_word_by_word() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "folders-places-root-name = Disque du système\nfolders-places-page-name = Stockage\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(root_name(), "Disque du système");
            assert_eq!(page_name(), "Stockage");
            assert_eq!(t!("folders-places-home"), "Home");
        });
    }

    #[test]
    fn pins_and_recent_files_that_cannot_be_read_are_never_written_over() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("trindle-files-places-unread-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let recent = dir.join("recently-used.xbel");
        let was = b"<?xml version=\"1.0\"?>\n<xbel>\n<bookmark href=\"file:///home/john/a.txt\"/>\n\xff\n</xbel>\n".to_vec();
        std::fs::write(&recent, &was).unwrap();
        assert!(note_recent_in(&recent, Path::new("/home/john/b.txt"), "text/plain").is_err());
        assert_eq!(std::fs::read(&recent).unwrap(), was);
        let fresh = dir.join("fresh.xbel");
        note_recent_in(&fresh, Path::new("/home/john/b.txt"), "text/plain").unwrap();
        note_recent_in(&fresh, Path::new("/home/john/c.txt"), "text/plain").unwrap();
        let now = std::fs::read_to_string(&fresh).unwrap();
        assert!(now.contains("b.txt") && now.contains("c.txt"), "{now}");
        let pins = dir.join("pins");
        crate::common::files::tests::write_only(&pins, b"/home/john/Work\n/home/john/Play\n");
        assert!(read_pins(&pins).is_empty(), "for showing, none");
        assert!(write_pins(&pins, &[PathBuf::from("/home/john/New")]).is_err());
        std::fs::set_permissions(&pins, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(std::fs::read_to_string(&pins).unwrap(), "/home/john/Work\n/home/john/Play\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_standard_folders_are_pinned_once_for_a_person_with_no_pins() {
        let dir = std::env::temp_dir().join(format!("trindle-files-firstpins-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (home, file) = (dir.join("home"), dir.join("conf/files-pinned"));
        for d in ["Downloads", "Documents", "Music", "Pictures", "Videos", "Projects", "Other"] {
            std::fs::create_dir_all(home.join(d)).unwrap();
        }
        let two = ["Documents", "Downloads", "Music", "Pictures", "Videos", "Projects"].map(|d| home.join(d));
        assert_eq!(load_pins(&file, &home), two);
        assert!(file.exists(), "written, so they are not given again");
        write_pins(&file, &two[..1]).unwrap();
        assert_eq!(load_pins(&file, &home), [two[0].clone()]);
        write_pins(&file, &[]).unwrap();
        assert!(load_pins(&file, &home).is_empty(), "unpinning all keeps all gone");
        write_pins(&file, &[home.join("Music")]).unwrap();
        assert_eq!(load_pins(&file, &home), [home.join("Music")]);
        std::fs::remove_dir_all(home.join("Documents")).unwrap();
        std::fs::remove_file(&file).unwrap();
        assert_eq!(load_pins(&file, &home), two[1..].to_vec());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_move_is_mirrored_in_the_other_folder() {
        let dir = std::env::temp_dir().join(format!("trindle-files-mirror-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for d in ["l/photos/2026", "l/films", "r/photos/2026", "r/music"] {
            std::fs::create_dir_all(dir.join(d)).unwrap();
        }
        let (l, r) = (dir.join("l"), dir.join("r"));
        assert_eq!(mirrored(&r.join("photos/2026"), &l.join("photos/2026"), &l), Some(r.clone()));
        assert_eq!(mirrored(&r, &l, &l.join("photos/2026")), Some(r.join("photos/2026")));
        assert_eq!(mirrored(&r.join("photos"), &l.join("photos"), &l.join("music")), Some(r.join("music")));
        assert_eq!(mirrored(&r.join("photos"), &l.join("photos"), &l.join("films")), None);
        assert_eq!(mirrored(&r.join("music"), &l.join("photos"), &l), None);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_typed_location_leads_to_its_folder() {
        let home = std::env::temp_dir().join(format!("trindle-files-typed-{}", std::process::id()));
        std::fs::create_dir_all(home.join("Music/Old")).unwrap();
        std::fs::write(home.join("Music/song one.ogg"), b"x").unwrap();
        let music = home.join("Music");
        let at = |t: &str| typed_place(t, &music, &home);
        assert_eq!(at("Old"), Some((Where::Folder(music.join("Old")), None)));
        assert_eq!(at("~"), Some((Where::Folder(home.clone()), None)));
        assert_eq!(at("~/Music/Old/.."), Some((Where::Folder(music.clone()), None)));
        assert_eq!(at(&format!("  {}//Old/  ", music.display())), Some((Where::Folder(music.join("Old")), None)));
        let uri = format!("file://{}/song%20one.ogg", music.display());
        assert_eq!(at(&uri), Some((Where::Folder(music.clone()), Some(music.join("song one.ogg")))));
        assert_eq!(at("trash:///"), Some((Where::Trash, None)));
        assert_eq!(at("recent:"), Some((Where::Recent, None)));
        assert_eq!(at("sftp://example.com/srv"), Some((Where::Remote("sftp://example.com/srv".into()), None)));
        assert_eq!(at("Nowhere"), None);
        assert_eq!(at("   "), None);
        std::fs::remove_dir_all(&home).unwrap();
    }

    fn entry(name: &str, folder: bool, size: u64, when: i64, kind: Option<&str>) -> Entry {
        Entry { name: name.into(), path: PathBuf::from("/x").join(name), folder, size, when, kind: kind.map(String::from) }
    }

    #[test]
    fn what_was_picked_is_found_again_in_a_list_without_a_wait() {
        let list: Vec<Entry> = ["a", "b", "c", "d"].iter().map(|n| entry(n, false, 0, 0, None)).collect();
        let paths = |names: &[&str]| names.iter().map(|n| PathBuf::from("/x").join(n)).collect::<Vec<_>>();
        assert_eq!(among(&list, &paths(&["d", "b", "gone", "b"])), [1, 3]);
        assert_eq!(among(&list, &paths(&[])), Vec::<usize>::new());
        assert_eq!(among(&[], &paths(&["a"])), Vec::<usize>::new());
        let many: Vec<Entry> = (0..150_000).map(|i| entry(&format!("photo-{i:06}.jpg"), false, 0, 0, None)).collect();
        let all: Vec<PathBuf> = many.iter().rev().map(|e| e.path.clone()).collect();
        let began = std::time::Instant::now();
        assert_eq!(among(&many, &all).len(), many.len());
        assert!(began.elapsed() < std::time::Duration::from_secs(5), "finding them again took {:?}", began.elapsed());
        for (file, text) in [("menu.rs", crate::common::source::code(crate::files::menu::SOURCE)), ("app.rs", crate::common::source::code(crate::files::app::SOURCE))] {
            assert_eq!(text.matches("picked = places::among(").count(), 1, "{file}");
            assert!(!text.contains(".contains(&e.path)).map(|(i, _)| i)"), "{file} looks each thing up among the others");
        }
    }

    #[test]
    fn an_opened_file_goes_to_the_top_of_recent() {
        let a = Path::new("/home/john/a b.txt");
        let b = Path::new("/home/john/b&c.png");
        let one = add_recent("", a, "text/plain", "2026-10-04T09:00:00Z");
        assert_eq!(recent(&one), [a.to_path_buf()]);
        assert!(one.starts_with("<?xml") && one.trim_end().ends_with("</xbel>"));
        let two = add_recent(&one, b, "image/png", "2026-10-04T10:00:00Z");
        assert_eq!(recent(&two), [b.to_path_buf(), a.to_path_buf()]);
        let three = add_recent(&two, a, "text/plain", "2026-10-04T11:00:00Z");
        assert_eq!(recent(&three), [a.to_path_buf(), b.to_path_buf()]);
        assert_eq!(three.matches("<bookmark href").count(), 2);
        assert!(three.contains("added=\"2026-10-04T09:00:00Z\""));
    }

    fn names(e: &[Entry]) -> Vec<&str> {
        e.iter().map(|e| e.name.as_str()).collect()
    }

    fn sample() -> Vec<Entry> {
        vec![
            entry("b.png", false, 300, 30, Some("image/png")),
            entry("Zed", true, 0, 10, None),
            entry("a.txt", false, 100, 20, Some("text/plain")),
            entry("apple", true, 0, 40, None),
            entry("C.mp3", false, 200, 10, Some("audio/mpeg")),
        ]
    }

    #[test]
    fn folders_come_first_and_names_ignore_case() {
        let mut e = sample();
        sort(&mut e, Key::Name, true);
        assert_eq!(names(&e), ["apple", "Zed", "a.txt", "b.png", "C.mp3"]);
        sort(&mut e, Key::Name, false);
        assert_eq!(names(&e), ["Zed", "apple", "C.mp3", "b.png", "a.txt"]);
    }

    #[test]
    fn sorts_by_size_date_and_type() {
        let mut e = sample();
        sort(&mut e, Key::Size, true);
        assert_eq!(names(&e), ["apple", "Zed", "a.txt", "C.mp3", "b.png"]);
        sort(&mut e, Key::Size, false);
        assert_eq!(names(&e), ["Zed", "apple", "b.png", "C.mp3", "a.txt"]);
        sort(&mut e, Key::Date, true);
        assert_eq!(names(&e), ["Zed", "apple", "C.mp3", "a.txt", "b.png"]);
        sort(&mut e, Key::Type, true);
        assert_eq!(names(&e), ["apple", "Zed", "C.mp3", "b.png", "a.txt"]);
    }

    #[test]
    fn type_falls_back_to_the_ending() {
        let mut e = vec![entry("x.zzz", false, 0, 0, None), entry("y.aaa", false, 0, 0, None), entry("noend", false, 0, 0, None)];
        sort(&mut e, Key::Type, true);
        assert_eq!(names(&e), ["noend", "y.aaa", "x.zzz"]);
    }

    #[test]
    fn the_path_bar_starts_at_home_or_the_computer() {
        let home = Path::new("/home/john");
        let c = crumbs(&Where::Folder("/home/john/Documents/work".into()), home);
        assert_eq!(c.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), ["Home", "Documents", "work"]);
        assert_eq!(c[1].1, Where::Folder("/home/john/Documents".into()));
        let c = crumbs(&Where::Folder("/usr/share".into()), home);
        assert_eq!(c.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), [page_name(), "System disk", "usr", "share"]);
        assert_eq!((&c[0].1, &c[1].1), (&Where::Computer, &Where::Folder("/".into())));
        assert_eq!(crumbs(&Where::Computer, home), [(page_name().to_string(), Where::Computer)]);
        assert_eq!(crumbs(&Where::Recent, home), [("Recent".to_string(), Where::Recent)]);
        let found = Where::Found("/home/john/Documents".into(), "tax".into(), false);
        let c = crumbs(&found, home);
        assert_eq!(c.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), ["Home", "Documents", "\u{201c}tax\u{201d}"]);
        assert_eq!(found.up(), Some(Where::Folder("/home/john/Documents".into())));
    }

    #[test]
    fn a_long_path_bar_keeps_its_first_and_last_parts() {
        let home = Path::new("/home/john");
        let c = crumbs(&Where::Folder("/home/john/a/b/c/d/e/f".into()), home);
        assert_eq!(c.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), ["Home", "c", "d", "e", "f"]);
        assert_eq!(c[1].1, Where::Folder("/home/john/a/b/c".into()));
    }

    #[test]
    fn a_run_goes_either_way() {
        assert_eq!(run(2, 5), [2, 3, 4, 5]);
        assert_eq!(run(5, 2), [2, 3, 4, 5]);
        assert_eq!(run(3, 3), [3]);
    }

    #[test]
    fn up_stops_at_the_top() {
        assert_eq!(Where::Folder("/home".into()).up(), Some(Where::Folder("/".into())));
        assert_eq!(Where::Folder("/".into()).up(), Some(Where::Computer));
        assert_eq!(Where::Computer.up(), None);
        assert_eq!(Where::Trash.up(), None);
    }

    #[test]
    fn network_mounts_are_found() {
        let mounts = "/dev/sda1 / ext4 rw 0 0\nserver:/share /mnt/share nfs4 rw 0 0\n//nas/music /home/john/Nas\\040Music cifs rw 0 0\n";
        assert_eq!(network(mounts), [PathBuf::from("/mnt/share"), PathBuf::from("/home/john/Nas Music")]);
    }

    #[test]
    fn pins_are_kept_a_line_each() {
        let dir = std::env::temp_dir().join(format!("trindle-files-pins-{}", std::process::id()));
        let file = dir.join("deeper/files-pinned");
        assert!(read_pins(&file).is_empty());
        let pins = vec![PathBuf::from("/home/john/Work"), PathBuf::from("/home/john/a b"), PathBuf::from("/bad\nname")];
        write_pins(&file, &pins).unwrap();
        assert_eq!(read_pins(&file), pins[..2]);
        std::fs::write(&file, "/one\n\n  \n/two\n").unwrap();
        assert_eq!(read_pins(&file), [PathBuf::from("/one"), PathBuf::from("/two")]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
