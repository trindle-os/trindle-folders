//! Installed applications from .desktop files, searched in XDG order so user

use crate::toolkit::t;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::files::home;

pub fn app_dirs() -> Vec<PathBuf> {
    let h = home();
    vec![
        h.join(".local/share/applications"),
        h.join(".local/share/flatpak/exports/share/applications"),
        PathBuf::from("/var/lib/flatpak/exports/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
        PathBuf::from("/usr/share/trindle/applications"),
        PathBuf::from("/usr/share/applications"),
    ]
}

fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let p = e.path();
        if p.is_dir() {
            walk(base, &p, out);
        } else if p.extension().is_some_and(|x| x == "desktop") {
            if let Ok(rel) = p.strip_prefix(base) {
                out.push((rel.to_string_lossy().replace('/', "-"), p.clone()));
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub exec: String,
    pub kinds: Vec<String>,
    pub path: PathBuf,
    pub flatpak: Option<String>,
    pub appimage: Option<String>,
    pub terminal: bool,
}

impl Entry {
    pub fn key(&self) -> &str {
        self.id.strip_suffix(".desktop").unwrap_or(&self.id)
    }

    #[cfg(test)]
    pub fn icons(&self) -> Vec<String> {
        let mut all = vec![format!("trindle-app-{}", self.key())];
        all.extend(self.icon.clone());
        all.push(generic_icon_for(self.key(), &self.name));
        all
    }
}

pub const GENERIC_ICONS: [&str; 20] = [
    "trindle-generic-01", "trindle-generic-02", "trindle-generic-03", "trindle-generic-04", "trindle-generic-05",
    "trindle-generic-06", "trindle-generic-07", "trindle-generic-08", "trindle-generic-09", "trindle-generic-10",
    "trindle-generic-11", "trindle-generic-12", "trindle-generic-13", "trindle-generic-14", "trindle-generic-15",
    "trindle-generic-16", "trindle-generic-17", "trindle-generic-18", "trindle-generic-19", "trindle-generic-20",
];

const OLD_STAND_INS: [&str; 10] = [
    "application-x-executable-symbolic", "package-x-generic-symbolic", "application-x-addon-symbolic",
    "text-x-generic-symbolic", "image-x-generic-symbolic", "computer-symbolic", "video-display-symbolic",
    "audio-speakers-symbolic", "starred-symbolic", "preferences-system-symbolic",
];

fn is_old_stand_in(icon: &str) -> bool {
    OLD_STAND_INS.contains(&icon) || icon.starts_with("trindle-generic-")
}

pub fn generic_icon_for(id: &str, name: &str) -> String {
    let clean = |s: &str| s.chars().map(|c| if c.is_control() { ' ' } else { c }).collect::<String>();
    format!("{}?{}\t{}", generic_icon(id), clean(id), clean(name))
}

pub fn generic_icon(key: &str) -> &'static str {
    let key = key.strip_suffix(".desktop").unwrap_or(key).to_lowercase();
    let mut h: u32 = 0x811c_9dc5;
    for b in key.bytes() {
        h = (h ^ b as u32).wrapping_mul(0x0100_0193);
    }
    GENERIC_ICONS[(h % GENERIC_ICONS.len() as u32) as usize]
}

fn languages() -> Vec<String> {
    let lang = ["LC_ALL", "LC_MESSAGES", "LANG"].iter()
        .filter_map(|v| std::env::var(v).ok()).find(|v| !v.is_empty()).unwrap_or_default();
    locale_names(&lang)
}

fn locale_names(lang: &str) -> Vec<String> {
    let (rest, modifier) = match lang.split_once('@') {
        Some((r, m)) => (r, Some(m)),
        None => (lang, None),
    };
    let rest = rest.split('.').next().unwrap_or("");
    if rest.is_empty() || rest == "C" || rest == "POSIX" {
        return Vec::new();
    }
    let (ll, cc) = match rest.split_once('_') {
        Some((l, c)) => (l, Some(c)),
        None => (rest, None),
    };
    let mut names = Vec::new();
    if let (Some(c), Some(m)) = (cc, modifier) {
        names.push(format!("{ll}_{c}@{m}"));
    }
    if let Some(c) = cc {
        names.push(format!("{ll}_{c}"));
    }
    if let Some(m) = modifier {
        names.push(format!("{ll}@{m}"));
    }
    names.push(ll.to_string());
    names
}

fn bus_start(key: &str, path: &str, files: &[String]) -> Vec<String> {
    let mut argv: Vec<String> = ["busctl", "--user", "call", key, path, "org.freedesktop.Application"].map(String::from).to_vec();
    if files.is_empty() {
        argv.extend(["Activate", "a{sv}", "0"].map(String::from));
    } else {
        argv.extend(["Open".into(), "asa{sv}".into(), files.len().to_string()]);
        argv.extend(files.iter().map(|f| if is_link(f) { f.clone() } else { file_uri(Path::new(f)) }));
        argv.push("0".into());
    }
    argv
}

fn runs(argv: &[String]) -> bool {
    let Some(first) = argv.first() else { return false };
    if !on_path(first) {
        return false;
    }
    if Path::new(first).file_name().is_some_and(|n| n == "env") {
        return argv[1..].iter().find(|a| !a.contains('=') && !a.starts_with('-')).is_none_or(|p| on_path(p));
    }
    true
}

fn on_path(program: &str) -> bool {
    let p = Path::new(program);
    if p.is_absolute() {
        return if super::files::own_place(p) { p.is_file() } else { far_there(p, FAR_GRACE, |p| p.is_file()) };
    }
    std::env::var_os("PATH").is_some_and(|path| std::env::split_paths(&path).any(|d| d.join(program).is_file()))
}

const FAR_GRACE: std::time::Duration = std::time::Duration::from_millis(40);
const FAR_FRESH: std::time::Duration = std::time::Duration::from_secs(2);
const FAR_SHY: std::time::Duration = std::time::Duration::from_secs(10);
static FAR_LOOKS: super::run::Slots = super::run::Slots::new(8);

#[derive(Clone, Copy)]
enum Far {
    Out,
    Seen(bool, std::time::Instant),
}

struct Fars {
    known: HashMap<PathBuf, Far>,
    late: Option<std::time::Instant>,
    tell: Vec<crate::toolkit::event::Post<()>>,
}

fn fars() -> &'static std::sync::Mutex<Fars> {
    static FARS: std::sync::OnceLock<std::sync::Mutex<Fars>> = std::sync::OnceLock::new();
    FARS.get_or_init(|| std::sync::Mutex::new(Fars { known: HashMap::new(), late: None, tell: Vec::new() }))
}

fn far_there(path: &Path, grace: std::time::Duration, look: fn(&Path) -> bool) -> bool {
    use std::time::Instant;
    let Ok(mut fars) = fars().lock() else { return false };
    match fars.known.get(path) {
        Some(Far::Seen(there, when)) if when.elapsed() < FAR_FRESH => return *there,
        Some(Far::Out) => return false,
        _ => {}
    }
    let Some(slot) = FAR_LOOKS.take() else { return false };
    let shy = fars.late.is_some_and(|late| late.elapsed() < FAR_SHY);
    fars.known.insert(path.to_path_buf(), Far::Out);
    drop(fars);
    let (back, answer) = std::sync::mpsc::channel();
    let at = path.to_path_buf();
    let started = std::thread::Builder::new().name("app-there".into()).stack_size(128 << 10).spawn(move || {
        let _slot = slot;
        let there = look(&at);
        let Ok(mut fars) = self::fars().lock() else { return };
        fars.known.insert(at, Far::Seen(there, Instant::now()));
        if back.send(there).is_err() && there {
            fars.tell.retain(|tell| tell.send(()));
        }
    });
    if started.is_err() {
        if let Ok(mut fars) = self::fars().lock() {
            fars.known.remove(path);
        }
        return false;
    }
    let heard = if shy { answer.try_recv().ok() } else { answer.recv_timeout(grace).ok() };
    drop(answer);
    if let Some(there) = heard {
        return there;
    }
    let Ok(mut fars) = self::fars().lock() else { return false };
    if let Some(Far::Seen(there, _)) = fars.known.get(path) {
        return *there;
    }
    fars.late = Some(Instant::now());
    false
}

fn entry_from(id: &str, file_path: &Path, file: &super::keyfile::KeyFile, langs: &[String], desktops: &[String],
              found: impl Fn(&str) -> bool) -> Option<Entry> {
    const GROUP: &str = "Desktop Entry";
    let get = |key: &str| file.string(GROUP, key);
    let yes = |key: &str| get(key).is_some_and(|v| v == "true");
    if get("Type").as_deref() != Some("Application") || yes("Hidden") || yes("NoDisplay") {
        return None;
    }
    let listed = |key: &str| file.list(GROUP, key).map(|l| l.iter().any(|d| desktops.iter().any(|x| x.eq_ignore_ascii_case(d))));
    if listed("OnlyShowIn") == Some(false) || listed("NotShowIn") == Some(true) {
        return None;
    }
    let try_exec = get("TryExec").filter(|t| !t.is_empty());
    if try_exec.as_deref().is_some_and(|t| !found(t)) {
        return None;
    }
    let exec = get("Exec").unwrap_or_default();
    if exec.is_empty() && !yes("DBusActivatable") {
        return None;
    }
    let local = |key: &str| langs.iter().find_map(|l| get(&format!("{key}[{l}]"))).or_else(|| get(key));
    let name = local("Name")?;
    Some(Entry {
        id: id.to_string(),
        name,
        icon: get("Icon").filter(|i| !i.is_empty() && !is_old_stand_in(i)),
        exec,
        kinds: file.list(GROUP, "MimeType").unwrap_or_default(),
        path: file_path.to_path_buf(),
        flatpak: get("X-Flatpak").filter(|f| !f.is_empty()),
        appimage: get("X-Trindle-AppImage").filter(|f| !f.is_empty()),
        terminal: yes("Terminal"),
    })
}

pub fn entries() -> Vec<Entry> {
    let langs = languages();
    let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        .split(':').filter(|d| !d.is_empty()).map(str::to_string).collect();
    let mut seen = HashSet::new();
    let mut all = Vec::new();
    for base in app_dirs() {
        let mut found = Vec::new();
        walk(&base, &base, &mut found);
        for (id, path) in found {
            if !seen.insert(id.clone()) {
                continue;
            }
            let Ok(file) = super::keyfile::KeyFile::load(&path) else { continue };
            all.extend(entry_from(&id, &path, &file, &langs, &desktops, on_path));
        }
    }
    all.sort_by_key(|a| a.name.to_lowercase());
    all
}

#[cfg(test)]
pub fn command_of(file: &super::keyfile::KeyFile, path: &Path) -> Option<(Vec<String>, Option<PathBuf>)> {
    command_with(file, path, &[])
}

fn command_with(file: &super::keyfile::KeyFile, path: &Path, files: &[String]) -> Option<(Vec<String>, Option<PathBuf>)> {
    const ENTRY: &str = "Desktop Entry";
    let line = file.string(ENTRY, "Exec")?;
    let link = |f: &String| match is_link(f) {
        true => f.clone(),
        false => file_uri(Path::new(f)),
    };
    let mut argv = Vec::new();
    for word in super::keyfile::command_words(&line)? {
        match word.as_str() {
            "%f" => argv.extend(files.first().cloned()),
            "%F" => argv.extend(files.iter().cloned()),
            "%u" => argv.extend(files.first().map(link)),
            "%U" => argv.extend(files.iter().map(link)),
            "%d" | "%D" | "%n" | "%N" | "%v" | "%m" => {}
            "%i" => {
                if let Some(icon) = file.string(ENTRY, "Icon") {
                    argv.extend(["--icon".to_string(), icon]);
                }
            }
            _ => argv.push(
                word.replace("%%", "\u{0}")
                    .replace("%c", &file.string(ENTRY, "Name").unwrap_or_default())
                    .replace("%k", &path.to_string_lossy())
                    .replace('\u{0}', "%"),
            ),
        }
    }
    let folder = file.string(ENTRY, "Path").filter(|p| !p.is_empty()).map(PathBuf::from);
    (!argv.is_empty()).then_some((argv, folder))
}

fn is_link(f: &str) -> bool {
    let Some((scheme, _)) = f.split_once(':') else { return false };
    let mut chars = scheme.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
        && !f.starts_with('/')
}

pub fn with_own_x11(argv: Vec<String>) -> Vec<String> {
    let flatpak = argv.first().is_some_and(|p| Path::new(p).file_name().is_some_and(|n| n == "flatpak"))
        && argv.get(1).is_some_and(|a| a == "run");
    let Some(id) = argv.iter().skip(2).find(|a| !a.starts_with('-')).filter(|_| flatpak).cloned() else { return argv };
    ["trindle-boxed".to_string(), "--own-x11".to_string(), id].into_iter().chain(argv).collect()
}

pub fn launch_boxed(app: &Entry, files: &[String]) -> bool {
    launch_inner(app, files, None, true)
}

pub fn loose_argv(app: &Entry, argv: &[String]) -> Option<Vec<String>> {
    let first = Path::new(argv.first()?).file_name()?.to_string_lossy().to_lowercase();
    let kept = app.terminal
        || app.flatpak.is_some()
        || app.appimage.is_some()
        || app.key().starts_with("trindle")
        || first == "trindle"
        || first.starts_with("trindle-")
        || ["flatpak", "bwrap", "snap"].contains(&first.as_str());
    if kept {
        return None;
    }
    let id = box_id(app.key());
    Some(["trindle-boxed".to_string(), "--loose".to_string(), id].into_iter().chain(argv.iter().cloned()).collect())
}

fn launch_inner(app: &Entry, files: &[String], token: Option<String>, boxed: bool) -> bool {
    let Ok(file) = super::keyfile::KeyFile::load(&app.path) else { return false };
    if app.exec.is_empty() {
        let path = format!("/{}", app.key().replace(['.', '-'], "/"));
        return super::run::spawn(&bus_start(app.key(), &path, files).iter().map(String::as_str).collect::<Vec<_>>());
    }
    let one_each = ["%f", "%u"].iter().any(|c| app.exec.contains(c)) && files.len() > 1;
    let rounds: Vec<&[String]> = match one_each {
        true => files.chunks(1).collect(),
        false => vec![files],
    };
    let mut started = false;
    for some in rounds {
        let Some((mut argv, folder)) = command_with(&file, &app.path, some) else { return false };
        if !runs(&argv) {
            return false;
        }
        if app.terminal {
            if let Some((_, inside)) = terminal_argv(&argv) {
                argv = inside;
            }
        }
        if box_here() {
            argv = with_own_x11(argv);
        }
        if boxed && box_here() {
            argv = loose_argv(app, &argv).unwrap_or(argv);
        }
        let desktop = app.path.clone();
        let go = move |token: Option<String>| {
            let mut command = std::process::Command::new(&argv[0]);
            command.args(&argv[1..]).stdin(std::process::Stdio::null()).env("GIO_LAUNCHED_DESKTOP_FILE", &desktop);
            if let Some(folder) = folder {
                command.current_dir(folder);
            }
            if let Some(token) = token {
                command.env("XDG_ACTIVATION_TOKEN", token);
            }
            super::run::child(command, |_| {});
        };
        match token.clone() {
            Some(token) => go(Some(token)),
            None => super::run::vouched(go),
        }
        started = true;
    }
    started
}

pub fn opener_for(kind: &str) -> Option<Entry> {
    let all = entries();
    chosen_of(&all, kind).or_else(|| openers_of(&all, kind).into_iter().next())
}

#[cfg(test)]
pub fn start_file(path: &Path) -> Result<(), String> {
    start_file_saying(path, std::process::Stdio::null())
}

pub fn opens(app: &Entry, kind: &str) -> bool {
    let family = kind.split('/').next().map(|f| format!("{f}/*"));
    app.kinds.iter().any(|t| t == kind || Some(t.as_str()) == family.as_deref())
}

pub fn openers_of(all: &[Entry], kind: &str) -> Vec<Entry> {
    all.iter().filter(|a| opens(a, kind)).cloned().collect()
}

fn mimeapps() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()).map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config")).join("mimeapps.list")
}

fn mimeapps_lists() -> Vec<PathBuf> {
    let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        .split(':').filter(|d| !d.is_empty()).map(str::to_lowercase).collect();
    let config = mimeapps();
    let mut dirs = vec![config.parent().map(Path::to_path_buf).unwrap_or_default()];
    let system = std::env::var("XDG_CONFIG_DIRS").ok().filter(|v| !v.is_empty()).unwrap_or_else(|| "/etc/xdg".into());
    dirs.extend(system.split(':').filter(|d| !d.is_empty()).map(PathBuf::from));
    dirs.extend(app_dirs());
    lists_in(&dirs, &desktops)
}

fn lists_in(dirs: &[PathBuf], desktops: &[String]) -> Vec<PathBuf> {
    dirs.iter().flat_map(|d| {
        desktops.iter().map(|n| d.join(format!("{n}-mimeapps.list"))).chain(std::iter::once(d.join("mimeapps.list")))
    }).collect()
}

pub fn chosen_of(all: &[Entry], kind: &str) -> Option<Entry> {
    let found = |id: &str| all.iter().find(|a| a.id == id && opens(a, kind)).cloned();
    for list in mimeapps_lists() {
        let file = super::keyfile::KeyFile::load_or_empty(&list);
        for group in ["Default Applications", "Added Associations"] {
            if let Some(app) = file.list(group, kind).unwrap_or_default().iter().find_map(|id| found(id)) {
                return Some(app);
            }
        }
    }
    openers_of(all, kind).into_iter().next()
}

pub fn choose_opener(id: &str, kinds: &[&str]) -> Result<(), String> {
    choose_opener_in(&mimeapps(), id, kinds)
}

pub(crate) fn choose_opener_in(path: &std::path::Path, id: &str, kinds: &[&str]) -> Result<(), String> {
    let mut file = super::keyfile::KeyFile::load_kept(path).map_err(|e| t!("folders-app-opener-unreadable", why = e.to_string()))?;
    for kind in kinds {
        file.set("Default Applications", kind, id);
        let mut added = vec![id.to_string()];
        added.extend(file.list("Added Associations", kind).unwrap_or_default().into_iter().filter(|a| a != id));
        file.set("Added Associations", kind, &format!("{};", added.join(";")));
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    super::files::write_text(path, &file.text()).map_err(|e| e.to_string())
}

pub fn terminals(all: &[Entry]) -> Vec<Entry> {
    all.iter().filter(|e| is_terminal(&super::keyfile::KeyFile::load_or_empty(&e.path))).cloned().collect()
}

fn is_terminal(file: &super::keyfile::KeyFile) -> bool {
    file.list("Desktop Entry", "Categories").unwrap_or_default().iter().any(|c| c == "TerminalEmulator")
}

pub fn default_terminal(all: &[Entry]) -> Option<Entry> {
    let found = terminals(all);
    let lists: Vec<String> = terminal_lists().iter().filter_map(|l| std::fs::read_to_string(l).ok()).collect();
    chosen_terminal(&found, &lists).or_else(|| found.into_iter().next())
}

fn terminal_lists() -> Vec<PathBuf> {
    let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        .split(':').filter(|d| !d.is_empty()).map(str::to_lowercase).collect();
    let config = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from).unwrap_or_else(|| home().join(".config"));
    let dirs = std::env::var("XDG_CONFIG_DIRS").ok().filter(|d| !d.is_empty()).unwrap_or_else(|| "/etc/xdg".into());
    let mut all = vec![config];
    all.extend(dirs.split(':').filter(|d| !d.is_empty()).map(PathBuf::from));
    let mut lists = Vec::new();
    for dir in all {
        lists.extend(desktops.iter().map(|d| dir.join(format!("{d}-xdg-terminals.list"))));
        lists.push(dir.join("xdg-terminals.list"));
    }
    lists
}

fn chosen_terminal(found: &[Entry], lists: &[String]) -> Option<Entry> {
    lists.iter().flat_map(|t| t.lines()).map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with('-'))
        .map(|l| l.trim_start_matches('+'))
        .map(|l| l.split_once(':').map_or(l, |(id, _)| id))
        .find_map(|id| found.iter().find(|e| e.id == id || e.key() == id).cloned())
}

pub fn terminal_argv(command: &[String]) -> Option<(Entry, Vec<String>)> {
    let term = default_terminal(&entries())?;
    let file = super::keyfile::KeyFile::load(&term.path).ok()?;
    let (argv, _) = command_with(&file, &term.path, &[])?;
    Some((term, with_command(argv, &file, command)))
}

fn with_command(mut argv: Vec<String>, file: &super::keyfile::KeyFile, command: &[String]) -> Vec<String> {
    if !command.is_empty() {
        let flag = file.string("Desktop Entry", "X-TerminalArgExec").unwrap_or_else(|| "-e".into());
        argv.extend(flag.split_whitespace().map(str::to_string));
        argv.extend(command.iter().cloned());
    }
    argv
}

pub fn start_terminal(folder: Option<&Path>, command: &[String]) -> bool {
    let Some((term, argv)) = terminal_argv(command) else { return false };
    let (desktop, folder) = (term.path.clone(), folder.map(Path::to_path_buf));
    super::run::vouched(move |token| {
        let mut command = std::process::Command::new(&argv[0]);
        command.args(&argv[1..]).stdin(std::process::Stdio::null()).env("GIO_LAUNCHED_DESKTOP_FILE", &desktop);
        if let Some(folder) = folder {
            command.current_dir(folder);
        }
        if let Some(token) = token {
            command.env("XDG_ACTIVATION_TOKEN", token);
        }
        super::run::child(command, |_| {});
    });
    true
}

#[cfg(test)]
pub fn start_file_saying(path: &Path, stderr: std::process::Stdio) -> Result<(), String> {
    let file = super::keyfile::KeyFile::load(path).map_err(|e| e.to_string())?;
    let (argv, folder) = command_of(&file, path).ok_or("It has no command to run.")?;
    let argv = if box_here() { with_own_x11(argv) } else { argv };
    let mut command = std::process::Command::new(&argv[0]);
    command.args(&argv[1..]).stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null()).stderr(stderr);
    if let Some(folder) = folder {
        command.current_dir(folder);
    }
    let mut child = command.spawn().map_err(|e| format!("could not start {}: {e}", argv[0]))?;
    std::thread::spawn(move || child.wait());
    Ok(())
}

fn box_here() -> bool {
    super::run::find("trindle-boxed").is_some()
}

fn box_id(name: &str) -> String {
    let s: String = name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    let s = s.split('-').filter(|p| !p.is_empty()).collect::<Vec<_>>().join("-");
    let s: String = s.chars().take(64).collect();
    if s.is_empty() { "app".into() } else { s }
}

pub fn file_uri(path: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    let mut url = String::from("file://");
    for &b in path.as_os_str().as_bytes() {
        if b.is_ascii_alphanumeric() || b"!$&'()*+,-./:=@_~".contains(&b) {
            url.push(b as char);
        } else {
            url.push_str(&format!("%{b:02X}"));
        }
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flathub_apps_start_through_their_own_xwayland() {
        let v = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let run = v(&["/usr/bin/flatpak", "run", "--branch=stable", "--command=jd", "org.jdownloader.JDownloader", "@@u", "x", "@@"]);
        assert_eq!(with_own_x11(run.clone()), [v(&["trindle-boxed", "--own-x11", "org.jdownloader.JDownloader"]), run].concat());
        for other in [v(&["flatpak", "list"]), v(&["alacritty"]), v(&["/opt/flatpak-run", "run", "x"]), v(&["flatpak", "run"]), v(&[])] {
            assert_eq!(with_own_x11(other.clone()), other);
        }
    }

    #[test]
    fn the_chosen_terminal_wins_else_the_first_found() {
        let dir = std::env::temp_dir().join(format!("trindle-terminals-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let entry = |id: &str, cats: &str| {
            let path = dir.join(id);
            std::fs::write(&path, format!("[Desktop Entry]\nType=Application\nName={id}\nExec={id}\nCategories={cats}\n")).unwrap();
            Entry { id: id.into(), name: id.into(), icon: None, exec: id.into(), kinds: Vec::new(), path, flatpak: None, appimage: None, terminal: false }
        };
        let all = vec![entry("aterm.desktop", "System;TerminalEmulator;"), entry("editor.desktop", "Utility;TextEditor;"),
                       entry("org.example.Zterm.desktop", "System;TerminalEmulator;")];
        let ids = |v: Vec<Entry>| v.into_iter().map(|e| e.id).collect::<Vec<_>>();
        assert_eq!(ids(terminals(&all)), ["aterm.desktop", "org.example.Zterm.desktop"]);
        let pick = |lists: &[&str]| chosen_terminal(&terminals(&all), &lists.iter().map(|s| s.to_string()).collect::<Vec<_>>()).map(|e| e.id);
        assert_eq!(pick(&["# mine\n-aterm.desktop\ngone.desktop\n+org.example.Zterm.desktop:new-window\n"]).as_deref(), Some("org.example.Zterm.desktop"));
        assert_eq!(pick(&["", "aterm"]).as_deref(), Some("aterm.desktop"));
        assert_eq!(pick(&["editor.desktop"]), None);
        assert_eq!(pick(&[]), None);
        assert!(terminals(&all[1..2]).is_empty());
        let own = crate::common::keyfile::KeyFile::parse("[Desktop Entry]\nExec=zterm\nX-TerminalArgExec=--\n");
        let plain = crate::common::keyfile::KeyFile::parse("[Desktop Entry]\nExec=aterm\n");
        let cmd = ["htop".to_string()];
        assert_eq!(with_command(vec!["zterm".into()], &own, &cmd), ["zterm", "--", "htop"]);
        assert_eq!(with_command(vec!["aterm".into()], &plain, &cmd), ["aterm", "-e", "htop"]);
        assert_eq!(with_command(vec!["aterm".into()], &plain, &[]), ["aterm"]);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_desktops_own_list_of_defaults_comes_first() {
        let lists = lists_in(&[PathBuf::from("/home/john/.config"), PathBuf::from("/usr/share/applications")], &["trindle".into()]);
        assert_eq!(lists, [
            PathBuf::from("/home/john/.config/trindle-mimeapps.list"), PathBuf::from("/home/john/.config/mimeapps.list"),
            PathBuf::from("/usr/share/applications/trindle-mimeapps.list"), PathBuf::from("/usr/share/applications/mimeapps.list"),
        ]);
    }

    #[test]
    fn links_are_told_from_paths() {
        assert!(is_link("mailto:john@example.com?subject=Hi"));
        assert!(is_link("https://example.com/a"));
        assert!(!is_link("/home/john/notes:1.txt"));
        assert!(!is_link("notes.txt"));
        assert!(!is_link("1x:y"));
        let file = super::super::keyfile::KeyFile::parse("[Desktop Entry]\nExec=mail %u\n");
        let (argv, _) = command_with(&file, Path::new("/x.desktop"), &["mailto:john@example.com".into()]).expect("command");
        assert_eq!(argv, vec!["mail".to_string(), "mailto:john@example.com".into()]);
    }

    #[test]
    fn an_app_with_no_icon_gets_the_same_generic_one_every_time() {
        assert_eq!(GENERIC_ICONS.len(), 20);
        let distinct: std::collections::HashSet<_> = GENERIC_ICONS.iter().collect();
        assert_eq!(distinct.len(), 20, "the twenty generic icons must differ");
        for (n, name) in GENERIC_ICONS.iter().enumerate() {
            assert_eq!(*name, format!("trindle-generic-{:02}", n + 1));
        }
        assert_eq!(generic_icon("org.x.Y"), generic_icon("org.x.Y.desktop"));
        assert_eq!(generic_icon("Firefox"), generic_icon("firefox"));
        assert_eq!(generic_icon(""), GENERIC_ICONS[(0x811c_9dc5u32 % 20) as usize]);
        assert_eq!(generic_icon("a"), GENERIC_ICONS[((0x811c_9dc5u32 ^ 0x61).wrapping_mul(0x0100_0193) % 20) as usize]);
        let used: std::collections::HashSet<_> = (0..400).map(|n| generic_icon(&format!("org.example.App{n}"))).collect();
        assert_eq!(used.len(), 20, "a spread of names uses all twenty");
    }

    #[test]
    fn every_place_that_shows_an_app_goes_through_the_one_function() {
        fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() { rust_files(&p, out) } else if p.extension().is_some_and(|x| x == "rs") { out.push(p) }
            }
        }
        let mut files = Vec::new();
        for (src, _) in crate::common::source::trees() {
            rust_files(&src, &mut files);
        }
        let entry = Entry { id: "org.x.Y.desktop".into(), name: "Y".into(), icon: Some("y".into()), exec: "y".into(), kinds: Vec::new(), path: PathBuf::new(), flatpak: None, appimage: None, terminal: false };
        assert_eq!(entry.icons().last().map(String::as_str), Some(generic_icon_for("org.x.Y", "Y").as_str()));
        assert_eq!(generic_icon_for("org.x.Y", "Why\tnot"), format!("{}?org.x.Y\tWhy not", generic_icon("org.x.Y")));
        for p in files {
            let text = std::fs::read_to_string(&p).unwrap();
            let code = text.split("#[cfg(test)]").next().unwrap();
            for (n, line) in code.lines().enumerate() {
                let t = line.trim_start();
                if t.starts_with("//") || p.ends_with("common/apps.rs") { continue; }
                assert!(!(line.contains("\"application-x-executable\"") && !line.contains("defaults_to")),
                    "{}:{}: the old stand-in icon must not be shown for a program", p.display(), n + 1);
                assert!(!line.contains("generic_icon("),
                    "{}:{}: an app's stand-in icon must come from common::apps::generic_icon_for, which puts the program on the list of those that need an icon", p.display(), n + 1);
            }
        }
    }

    #[test]
    fn launcher_files_are_read_without_gio() {
        use crate::common::keyfile::KeyFile;
        let file = |text: &str| KeyFile::parse(&format!("[Desktop Entry]\nType=Application\n{text}"));
        let langs = locale_names("en_GB.UTF-8");
        assert_eq!(langs, ["en_GB", "en"]);
        assert_eq!(locale_names("sr_RS@latin"), ["sr_RS@latin", "sr_RS", "sr@latin", "sr"]);
        assert!(locale_names("C.UTF-8").is_empty());
        let desk = vec!["Trindle".to_string()];
        let read = |text: &str| entry_from("org.x.Y.desktop", Path::new("/x"), &file(text), &langs, &desk, |p| p == "there");
        let e = read("Name=Color\nName[en_GB]=Colour\nExec=env A=1 /usr/bin/Ycolour %U\nIcon=y\nStartupWMClass=YC").unwrap();
        assert_eq!((e.name.as_str(), e.icon.as_deref(), e.key()), ("Colour", Some("y"), "org.x.Y"));
        assert_eq!(e.icons(), ["trindle-app-org.x.Y".to_string(), "y".to_string(), generic_icon_for("org.x.Y", "Colour")]);
        let old = read("Name=Old\nExec=old\nIcon=application-x-executable-symbolic").unwrap();
        assert_eq!(old.icon, None, "a launcher that names one of the old stand-ins has no icon of its own");
        assert_eq!(read("Name=Old\nExec=old\nIcon=trindle-generic-03").unwrap().icon, None);
        assert!(read("Name=A\nExec=a\nNoDisplay=true").is_none());
        assert!(read("Name=A\nExec=a\nHidden=true").is_none());
        assert!(read("Name=A\nExec=a\nOnlyShowIn=KDE;").is_none());
        assert!(read("Name=A\nExec=a\nOnlyShowIn=KDE;trindle;").is_some());
        assert!(read("Name=A\nExec=a\nNotShowIn=Trindle;").is_none());
        assert!(read("Name=A\nExec=a\nTryExec=gone").is_none());
        assert!(on_path("/usr/bin/env") && !on_path("/usr/bin/trindle-test-not-there"));
        assert_eq!(file_uri(Path::new("/home/user/a;b&c.txt")), "file:///home/user/a%3Bb&c.txt");
        assert_eq!(bus_start("org.x.Y", "/org/x/Y", &["/home/user/a b.txt".into()])[6..], ["Open", "asa{sv}", "1", "file:///home/user/a%20b.txt", "0"]);
        assert_eq!(bus_start("org.x.Y", "/org/x/Y", &[])[6..], ["Activate", "a{sv}", "0"]);
        let dir = std::env::temp_dir().join(format!("trindle-apps-missing-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for (exec, starts) in [("/usr/bin/trindle-test-not-there %f", false), ("env A=1 trindle-test-not-there %f", false)] {
            let text = format!("[Desktop Entry]\nType=Application\nName=A\nExec={exec}\n");
            let path = dir.join("a.desktop");
            std::fs::write(&path, &text).unwrap();
            let e = entry_from("a.desktop", &path, &KeyFile::parse(&text), &langs, &desk, |_| true).unwrap();
            assert_eq!(launch_boxed(&e, &[]), starts, "{exec}");
        }
        let _ = std::fs::remove_dir_all(dir);
        assert!(read("Name=A\nExec=a\nTryExec=there").is_some());
        assert!(read("Name=A").is_none());
        assert!(entry_from("l.desktop", Path::new("/x"), &KeyFile::parse("[Desktop Entry]\nType=Link\nName=L\nExec=a"), &langs, &desk, |_| true).is_none());
    }

    #[test]
    fn a_program_that_may_be_on_a_share_is_looked_for_on_a_thread_and_is_missing_until_seen() {
        use std::time::{Duration, Instant};
        assert!(!on_path("/mnt/share/trindle-test-not-there") && !on_path("/home/sam/Applications/trindle-test-not-there"));
        let here = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        assert!(on_path(here), "a program that is there was not found");
        let looked_for = fars().lock().unwrap().known.contains_key(Path::new(here));
        assert_eq!(looked_for, !crate::common::files::own_place(Path::new(here)));
        fn slow(_: &Path) -> bool {
            std::thread::sleep(Duration::from_millis(400));
            true
        }
        let share = Path::new("/mnt/share/trindle-test-slow");
        let since = Instant::now();
        assert!(!far_there(share, Duration::from_millis(20), slow));
        assert!(since.elapsed() < Duration::from_millis(300), "the list of apps waited on a share");
        assert!(!far_there(share, Duration::from_secs(5), |_| panic!("looked twice")));
        assert!(since.elapsed() < Duration::from_millis(300));
        let back = || matches!(fars().lock().unwrap().known.get(share), Some(Far::Seen(true, _)));
        while !back() && since.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(far_there(share, Duration::from_millis(20), |_| panic!("looked again while the answer was fresh")));
        assert!(fars().lock().unwrap().late.take().is_some());
        let held: Vec<_> = std::iter::from_fn(|| FAR_LOOKS.take()).collect();
        assert!((1..=8).contains(&held.len()), "{} looks may be out at once", held.len());
        assert!(!far_there(Path::new("/mnt/share/trindle-test-one-more"), Duration::from_secs(5), |_| true));
        drop(held);
        assert!(far_there(Path::new("/mnt/share/trindle-test-one-more"), Duration::from_secs(5), |_| true));
    }

    #[test]
    fn a_program_folders_starts_is_boxed_unless_it_already_is() {
        let app = |id: &str, exec: &str, flatpak: Option<&str>, appimage: bool, terminal: bool| Entry {
            id: format!("{id}.desktop"), name: id.into(), icon: None, exec: exec.into(), kinds: Vec::new(),
            path: PathBuf::new(), flatpak: flatpak.map(str::to_string),
            appimage: appimage.then(|| "/home/john/.appimage/x.AppImage|1".to_string()), terminal,
        };
        let words = |w: &[&str]| w.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let gimp = app("gimp", "gimp %U", None, false, false);
        assert_eq!(loose_argv(&gimp, &words(&["gimp", "/home/john/a.png"])).unwrap(), words(&["trindle-boxed", "--loose", "gimp", "gimp", "/home/john/a.png"]));
        assert!(loose_argv(&gimp, &words(&["env", "A=1", "gimp"])).is_some(), "a wrapper is boxed with the program");
        for (entry, argv) in [
            (app("org.gimp.GIMP", "flatpak run org.gimp.GIMP", Some("org.gimp.GIMP"), false, false), words(&["flatpak", "run", "org.gimp.GIMP"])),
            (app("x", "trindle-boxed --appimage /a %U", None, true, false), words(&["trindle-boxed", "--appimage", "/a"])),
            (app("alacritty", "trindle-boxed alacritty alacritty", None, false, false), words(&["trindle-boxed", "alacritty", "alacritty"])),
            (app("trindle-text", "trindle-text %F", None, false, false), words(&["trindle-text"])),
            (app("htop", "htop", None, false, true), words(&["htop"])),
            (app("other", "/usr/bin/trindle-thing", None, false, false), words(&["/usr/bin/trindle-thing"])),
        ] {
            assert!(loose_argv(&entry, &argv).is_none(), "{} was boxed twice or boxed when it is Trindle's", entry.id);
        }
    }

    #[test]
    fn a_launcher_command_drops_its_file_codes() {
        let file = crate::common::keyfile::KeyFile::parse(
            "[Desktop Entry]\nName=Paint\nIcon=paint\nPath=/tmp\nExec=env A=1 \"/opt/my paint/paint\" %U --title=%c %i 100%%\n");
        let (argv, folder) = command_of(&file, Path::new("/x/paint.desktop")).unwrap();
        assert_eq!(argv, ["env", "A=1", "/opt/my paint/paint", "--title=Paint", "--icon", "paint", "100%"]);
        assert_eq!(folder.as_deref(), Some(Path::new("/tmp")));
        let none = crate::common::keyfile::KeyFile::parse("[Desktop Entry]\nName=Nothing\nExec=%f\n");
        assert_eq!(command_of(&none, Path::new("/x")), None);
    }

    #[test]
    fn a_launcher_file_starts_its_program_in_its_folder() {
        let dir = std::env::temp_dir().join(format!("trindle-start-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let launcher = dir.join("touch.desktop");
        std::fs::write(&launcher, format!("[Desktop Entry]\nName=Touch\nPath={}\nExec=touch started %F\n", dir.display())).unwrap();
        start_file(&launcher).unwrap();
        let started = dir.join("started");
        for _ in 0..50 {
            if started.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(40));
        }
        assert!(started.exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
