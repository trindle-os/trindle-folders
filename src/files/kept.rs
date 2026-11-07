// What Files remembers about single folders and files: the colour a folder was given and the view a folder was last shown in.

use std::collections::HashMap;
use std::ffi::OsString;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

use super::app::View;
use super::columns::{self, Col};
use super::groups::GroupBy;
use super::heading::{LEAST, MOST};
use super::icons::{ICON_SIZES, ICON_USUAL};
use super::places::Where;
use crate::common::files::{config_path, write_whole};

pub const COLOURS: &str = "files-colours";
pub const VIEWS: &str = "files-views";
#[cfg_attr(test, allow(dead_code))]
pub const LAST: &str = "files-last";
pub const COLUMNS: &str = "files-columns";
pub const WIDTHS: &str = "files-widths";
pub const GROUPS: &str = "files-groups";
pub const ICON_SIZE: &str = "files-icon-size";
pub const LINKED: &str = "files-linked";
pub const PANES: &str = "files-panes";
pub const PANEL: &str = "files-panel";

#[derive(Default)]
pub struct Kept {
    pub colours: HashMap<PathBuf, String>,
    pub views: HashMap<PathBuf, View>,
    pub columns: Option<Vec<Col>>,
    widths: HashMap<(bool, Col), f32>,
    groups: HashMap<&'static str, GroupBy>,
    icon: Option<f32>,
    pub linked: bool,
    pub dual: bool,
    pub panel: bool,
    panel_width: Option<f32>,
    other: HashMap<&'static str, Vec<Vec<u8>>>,
    dir: Option<PathBuf>,
}

impl Kept {
    #[cfg_attr(test, allow(dead_code))]
    pub fn load() -> Kept {
        Kept::from(Some(crate::common::files::trindle_config()))
    }

    pub fn from(dir: Option<PathBuf>) -> Kept {
        let Some(d) = &dir else { return Kept::default() };
        let mut other: HashMap<&'static str, Vec<Vec<u8>>> = HashMap::new();
        let mut lines = |name: &'static str, known: &mut dyn FnMut(PathBuf, String) -> bool| {
            let bytes = crate::common::files::read_file(d.join(name)).unwrap_or_default();
            for line in bytes.split(|&b| b == b'\n').filter(|l| !l.is_empty()) {
                if !parsed(line).is_some_and(|(path, word)| known(path, word)) {
                    other.entry(name).or_default().push(line.to_vec());
                }
            }
        };
        let mut colours = HashMap::new();
        lines(COLOURS, &mut |path, word| {
            colours.insert(path, word);
            true
        });
        let mut views = HashMap::new();
        lines(VIEWS, &mut |path, word| view_named(&word).map(|v| views.insert(path, v)).is_some());
        let mut groups = HashMap::new();
        lines(GROUPS, &mut |kind, word| {
            let known = PLACE_KINDS.into_iter().find(|k| Path::new(k) == kind).zip(GroupBy::named(&word));
            known.map(|(kind, by)| groups.insert(kind, by)).is_some()
        });
        let columns_text = crate::common::files::read_string(d.join(COLUMNS)).ok();
        let columns = columns_text.as_deref().map(columns::read);
        let words: Vec<Vec<u8>> = columns_text.iter().flat_map(|t| t.split_whitespace()).filter(|w| Col::named(w).is_none()).map(|w| w.as_bytes().to_vec()).collect();
        if !words.is_empty() {
            other.insert(COLUMNS, words);
        }
        let widths_text = crate::common::files::read_string(d.join(WIDTHS)).unwrap_or_default();
        let widths = read_widths(&widths_text);
        let strange: Vec<Vec<u8>> = widths_text.lines().filter(|l| !l.trim().is_empty() && read_widths(l).is_empty()).map(|l| l.as_bytes().to_vec()).collect();
        if !strange.is_empty() {
            other.insert(WIDTHS, strange);
        }
        let icon = crate::common::files::read_string(d.join(ICON_SIZE)).ok().and_then(|t| t.trim().parse::<f32>().ok()).filter(|s| s.is_finite())
            .map(|s| ICON_SIZES.into_iter().min_by(|a, b| (a - s).abs().total_cmp(&(b - s).abs())).unwrap_or(ICON_USUAL));
        let linked = crate::common::files::read_string(d.join(LINKED)).is_ok_and(|t| t.trim() == "on");
        let dual = crate::common::files::read_string(d.join(PANES)).is_ok_and(|t| dual_named(&t));
        let (panel, panel_width) = read_panel(&crate::common::files::read_string(d.join(PANEL)).unwrap_or_default());
        Kept { colours, views, columns, widths, groups, icon, linked, dual, panel, panel_width, other, dir }
    }

    pub fn colour(&mut self, folder: &Path, name: Option<&str>) -> std::io::Result<()> {
        match name {
            Some(n) => self.colours.insert(folder.to_path_buf(), n.to_string()),
            None => self.colours.remove(folder),
        };
        let lines: Vec<(PathBuf, String)> = self.colours.iter().map(|(p, n)| (p.clone(), n.clone())).collect();
        self.write(COLOURS, &lines)
    }

    pub fn moved(&mut self, was: &Path, now: &Path) -> std::io::Result<()> {
        fn shift<V>(map: &mut HashMap<PathBuf, V>, was: &Path, now: &Path) -> bool {
            let hit: Vec<PathBuf> = map.keys().filter(|p| p.starts_with(was)).cloned().collect();
            for old in &hit {
                if let (Some(v), Ok(rest)) = (map.remove(old), old.strip_prefix(was)) {
                    map.insert(if rest.as_os_str().is_empty() { now.to_path_buf() } else { now.join(rest) }, v);
                }
            }
            !hit.is_empty()
        }
        if shift(&mut self.colours, was, now) {
            let lines: Vec<(PathBuf, String)> = self.colours.iter().map(|(p, n)| (p.clone(), n.clone())).collect();
            self.write(COLOURS, &lines)?;
        }
        Ok(())
    }

    pub fn view(&mut self, at: &Where, v: View) -> std::io::Result<()> {
        let Some(key) = key(at) else { return Ok(()) };
        if self.views.get(&key) == Some(&v) {
            return Ok(());
        }
        self.answered(VIEWS, &key);
        self.views.insert(key, v);
        let lines: Vec<(PathBuf, String)> = self.views.iter().map(|(p, v)| (p.clone(), view_word(*v).to_string())).collect();
        self.write(VIEWS, &lines)
    }

    pub fn columns(&self) -> Vec<Col> {
        self.columns.clone().unwrap_or_else(|| Col::USUAL.to_vec())
    }

    pub fn width(&self, col: Col, rich: bool) -> f32 {
        self.widths.get(&(rich, col)).or_else(|| self.widths.get(&(!rich, col))).copied().unwrap_or_else(|| col.width())
    }

    pub fn set_width(&mut self, col: Col, rich: bool, width: f32) {
        self.widths.insert((rich, col), width.clamp(LEAST, MOST));
    }

    pub fn save_widths(&self) -> std::io::Result<()> {
        let Some(d) = &self.dir else { return Ok(()) };
        std::fs::create_dir_all(d)?;
        whole_was_read(&d.join(WIDTHS))?;
        let mut lines: Vec<String> = self.widths.iter()
            .map(|((rich, col), w)| format!("{} {} {w}\n", if *rich { "rich" } else { "list" }, col.key())).collect();
        lines.sort();
        let mut out = lines.concat().into_bytes();
        self.and_other(WIDTHS, &mut out);
        write_whole(&d.join(WIDTHS), &out)
    }

    fn and_other(&self, name: &str, out: &mut Vec<u8>) {
        for line in self.other.get(name).into_iter().flatten() {
            out.extend_from_slice(line);
            out.push(b'\n');
        }
    }

    fn answered(&mut self, name: &'static str, path: &Path) {
        if let Some(lines) = self.other.get_mut(name) {
            lines.retain(|line| !parsed(line).is_some_and(|(p, _)| p == path));
        }
    }

    pub fn group(&self, at: &Where) -> GroupBy {
        self.groups.get(place_kind(at)).copied().unwrap_or_default()
    }

    pub fn set_group(&mut self, at: &Where, by: GroupBy) -> std::io::Result<()> {
        self.answered(GROUPS, Path::new(place_kind(at)));
        self.groups.insert(place_kind(at), by);
        let lines: Vec<(PathBuf, String)> = self.groups.iter().map(|(kind, by)| (PathBuf::from(kind), by.key().to_string())).collect();
        self.write(GROUPS, &lines)
    }

    pub fn icon(&self) -> f32 {
        self.icon.unwrap_or(ICON_USUAL)
    }

    pub fn set_icon(&mut self, size: f32) -> std::io::Result<()> {
        self.icon = Some(size);
        let Some(d) = &self.dir else { return Ok(()) };
        std::fs::create_dir_all(d)?;
        write_whole(&d.join(ICON_SIZE), format!("{size}\n").as_bytes())
    }

    pub fn link(&mut self, on: bool) -> std::io::Result<()> {
        self.linked = on;
        let Some(d) = &self.dir else { return Ok(()) };
        std::fs::create_dir_all(d)?;
        write_whole(&d.join(LINKED), if on { b"on\n" } else { b"off\n" })
    }

    pub fn panes(&mut self, dual: bool) -> std::io::Result<()> {
        self.dual = dual;
        let Some(d) = &self.dir else { return Ok(()) };
        std::fs::create_dir_all(d)?;
        write_whole(&d.join(PANES), if dual { b"dual\n" } else { b"single\n" })
    }

    pub fn panel_width(&self) -> f32 {
        self.panel_width.unwrap_or_else(|| crate::toolkit::theme::sheet().preview_panel)
    }

    pub fn set_panel(&mut self, on: bool, width: Option<f32>) {
        self.panel = on;
        if let Some(w) = width.filter(|w| w.is_finite()) {
            self.panel_width = Some(w);
        }
    }

    pub fn save_panel(&self) -> std::io::Result<()> {
        let Some(d) = &self.dir else { return Ok(()) };
        std::fs::create_dir_all(d)?;
        whole_was_read(&d.join(PANEL))?;
        let width = self.panel_width.map(|w| format!(" {w:.0}")).unwrap_or_default();
        write_whole(&d.join(PANEL), format!("{}{width}\n", if self.panel { "on" } else { "off" }).as_bytes())
    }

    pub fn column(&mut self, col: Col, on: bool) -> std::io::Result<()> {
        let now = self.columns();
        let cols: Vec<Col> = Col::ALL.into_iter().filter(|c| if *c == col { on } else { now.contains(c) }).collect();
        let Some(d) = &self.dir else {
            self.columns = Some(cols);
            return Ok(());
        };
        std::fs::create_dir_all(d)?;
        whole_was_read(&d.join(COLUMNS))?;
        self.columns = Some(cols.clone());
        let mut words = columns::write(&cols).into_bytes();
        for word in self.other.get(COLUMNS).into_iter().flatten() {
            words.push(b' ');
            words.extend_from_slice(word);
        }
        write_whole(&d.join(COLUMNS), &words)
    }

    pub fn view_of(&self, at: &Where) -> Option<View> {
        key(at).and_then(|k| self.views.get(&k).copied())
    }

    fn write(&self, name: &str, lines: &[(PathBuf, String)]) -> std::io::Result<()> {
        let Some(d) = &self.dir else { return Ok(()) };
        std::fs::create_dir_all(d)?;
        crate::common::files::may_rewrite(&d.join(name))?;
        let mut out = Vec::new();
        for (path, word) in lines.iter().filter(|(p, _)| !p.as_os_str().as_bytes().contains(&b'\n')) {
            out.extend_from_slice(word.as_bytes());
            out.push(b'\t');
            out.extend_from_slice(path.as_os_str().as_bytes());
            out.push(b'\n');
        }
        self.and_other(name, &mut out);
        write_whole(&d.join(name), &out)
    }
}

const PLACE_KINDS: [&str; 8] = ["folder", "recent", "trash", "server", "network", "archive", "found", "computer"];

fn place_kind(at: &Where) -> &'static str {
    match at {
        Where::Folder(_) => PLACE_KINDS[0],
        Where::Recent => PLACE_KINDS[1],
        Where::Trash => PLACE_KINDS[2],
        Where::Remote(_) => PLACE_KINDS[3],
        Where::Network => PLACE_KINDS[4],
        Where::Inside(..) => PLACE_KINDS[5],
        Where::Found(..) => PLACE_KINDS[6],
        Where::Computer => PLACE_KINDS[7],
    }
}

fn key(at: &Where) -> Option<PathBuf> {
    match at {
        Where::Folder(p) => Some(p.clone()),
        Where::Recent => Some(PathBuf::from("recent:")),
        Where::Trash => Some(PathBuf::from("trash:")),
        Where::Computer => Some(PathBuf::from("computer:")),
        Where::Remote(a) => Some(PathBuf::from(a)),
        Where::Network | Where::Inside(..) | Where::Found(..) => None,
    }
}

#[cfg_attr(test, allow(dead_code))]
fn read(file: &Path) -> Vec<(PathBuf, String)> {
    let bytes = crate::common::files::read_file(file).unwrap_or_default();
    bytes.split(|&b| b == b'\n').filter_map(parsed).collect()
}

fn parsed(line: &[u8]) -> Option<(PathBuf, String)> {
    let tab = line.iter().position(|&b| b == b'\t')?;
    let word = String::from_utf8(line[..tab].to_vec()).ok()?;
    let path = &line[tab + 1..];
    (!path.is_empty()).then(|| (PathBuf::from(OsString::from_vec(path.to_vec())), word))
}

fn whole_was_read(file: &Path) -> std::io::Result<()> {
    crate::common::files::read_small(file).map(drop)
}

fn read_widths(text: &str) -> HashMap<(bool, Col), f32> {
    text.lines().filter_map(|l| {
        let mut words = l.split_whitespace();
        let rich = match words.next()? {
            "rich" => true,
            "list" => false,
            _ => return None,
        };
        let col = Col::named(words.next()?)?;
        let w: f32 = words.next()?.parse().ok().filter(|w: &f32| w.is_finite())?;
        Some(((rich, col), w.clamp(LEAST, MOST)))
    }).collect()
}

pub fn read_panel(text: &str) -> (bool, Option<f32>) {
    let mut words = text.split_whitespace();
    let on = words.next() == Some("on");
    (on, words.next().and_then(|w| w.parse::<f32>().ok()).filter(|w| w.is_finite() && *w > 0.0))
}

pub fn dual_named(text: &str) -> bool {
    text.trim() == "dual"
}

pub fn view_word(v: View) -> &'static str {
    match v {
        View::Icons => "icons",
        View::List => "list",
        View::Compact => "compact",
    }
}

pub fn view_named(word: &str) -> Option<View> {
    match word {
        "icons" => Some(View::Icons),
        "list" => Some(View::List),
        "compact" => Some(View::Compact),
        _ => None,
    }
}

#[cfg_attr(test, allow(dead_code))]
pub fn start_places(home: &Path) -> (Where, Where) {
    use crate::common::folders_conf::{get, LEFT, RIGHT, START};
    let last = read(&config_path(LAST));
    start_from(home, &get(START), [get(LEFT), get(RIGHT)], &last)
}

pub fn start_from(home: &Path, start: &str, chosen: [String; 2], last: &[(PathBuf, String)]) -> (Where, Where) {
    let fallback = (Where::Folder(home.to_path_buf()), Where::Recent);
    match start {
        "chosen" => (place_named(&chosen[0], home).unwrap_or(fallback.0),
                     place_named(&chosen[1], home).unwrap_or(fallback.1)),
        "last" => {
            let find = |side: &str| last.iter().find(|(_, w)| w == side).and_then(|(p, _)| place_named(&p.to_string_lossy(), home));
            (find("left").unwrap_or(fallback.0), find("right").unwrap_or(fallback.1))
        }
        _ => fallback,
    }
}

pub fn place_named(text: &str, home: &Path) -> Option<Where> {
    match text.trim() {
        "" => None,
        "recent:" => Some(Where::Recent),
        "trash:" => Some(Where::Trash),
        "computer:" => Some(Where::Computer),
        "~" => Some(Where::Folder(home.to_path_buf())),
        t => {
            let p = match t.strip_prefix("~/") {
                Some(rest) => home.join(rest),
                None => PathBuf::from(t),
            };
            p.is_dir().then_some(Where::Folder(p))
        }
    }
}

#[cfg_attr(test, allow(dead_code))]
pub fn save_last(left: &Where, right: &Where) {
    let file = config_path(LAST);
    let mut out = Vec::new();
    for (side, at) in [("left", left), ("right", right)] {
        if let Some(k) = key(at).filter(|k| !k.as_os_str().as_bytes().contains(&b'\n')) {
            out.extend_from_slice(side.as_bytes());
            out.push(b'\t');
            out.extend_from_slice(k.as_os_str().as_bytes());
            out.push(b'\n');
        }
    }
    let _ = write_whole(&file, &out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kept_file_that_cannot_be_read_is_never_written_over() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("trindle-files-kept-unread-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let was: [(&str, &[u8]); 4] = [(COLOURS, b"Red\t/home/john/Work\nBlue\t/home/john/Play\n"),
                                       (VIEWS, b"icons\t/home/john/Pictures\n"), (GROUPS, b"kind\tfolder\n"), (WIDTHS, b"list size 120\n")];
        for (name, bytes) in was {
            crate::common::files::tests::write_only(&dir.join(name), bytes);
        }
        let mut k = Kept::from(Some(dir.clone()));
        assert!(k.colour(Path::new("/home/john/New"), Some("Green")).is_err());
        assert!(k.save_widths().is_err());
        for (name, bytes) in was {
            std::fs::set_permissions(dir.join(name), std::fs::Permissions::from_mode(0o600)).unwrap();
            assert_eq!(std::fs::read(dir.join(name)).unwrap(), bytes, "{name} is as it was");
        }
        let mut k = Kept::from(Some(dir.clone()));
        k.colour(Path::new("/home/john/New"), Some("Green")).unwrap();
        assert_eq!(std::fs::read_to_string(dir.join(COLOURS)).unwrap().lines().count(), 3);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn colours_and_views_are_kept_a_line_each() {
        let dir = std::env::temp_dir().join(format!("trindle-files-kept-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut k = Kept::from(Some(dir.clone()));
        let folder = PathBuf::from("/tmp/my photos");
        k.colour(&folder, Some("Teal")).unwrap();
        k.view(&Where::Folder(folder.clone()), View::Icons).unwrap();
        k.view(&Where::Trash, View::Compact).unwrap();
        let back = Kept::from(Some(dir.clone()));
        assert_eq!(back.colours.get(&folder).map(String::as_str), Some("Teal"));
        assert_eq!(back.view_of(&Where::Folder(folder.clone())), Some(View::Icons));
        assert_eq!(back.view_of(&Where::Trash), Some(View::Compact));
        assert_eq!(back.view_of(&Where::Recent), None);
        k.colour(&folder, None).unwrap();
        assert!(Kept::from(Some(dir.clone())).colours.is_empty());
        assert_eq!(k.columns(), Col::USUAL);
        k.column(Col::Owner, true).unwrap();
        k.column(Col::Type, false).unwrap();
        assert_eq!(Kept::from(Some(dir.clone())).columns(), [Col::Size, Col::Date, Col::Owner]);
        assert_eq!(k.group(&Where::Folder(folder.clone())), GroupBy::None);
        k.set_group(&Where::Folder(folder.clone()), GroupBy::Kind).unwrap();
        k.set_group(&Where::Trash, GroupBy::Date).unwrap();
        let back = Kept::from(Some(dir.clone()));
        assert_eq!(back.group(&Where::Folder(PathBuf::from("/tmp/another"))), GroupBy::Kind);
        assert_eq!((back.group(&Where::Trash), back.group(&Where::Recent)), (GroupBy::Date, GroupBy::None));
        k.set_group(&Where::Folder(folder.clone()), GroupBy::None).unwrap();
        assert_eq!(Kept::from(Some(dir.clone())).group(&Where::Folder(folder.clone())), GroupBy::None);
        assert_eq!(k.icon(), ICON_USUAL);
        k.set_icon(128.0).unwrap();
        assert_eq!(Kept::from(Some(dir.clone())).icon(), 128.0);
        std::fs::write(dir.join(ICON_SIZE), "70\n").unwrap();
        assert_eq!(Kept::from(Some(dir.clone())).icon(), 64.0);
        std::fs::write(dir.join(ICON_SIZE), "enormous\n").unwrap();
        assert_eq!(Kept::from(Some(dir.clone())).icon(), ICON_USUAL);
        assert!(!k.linked);
        k.link(true).unwrap();
        assert!(Kept::from(Some(dir.clone())).linked);
        k.link(false).unwrap();
        assert!(!Kept::from(Some(dir.clone())).linked);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn colours_go_where_their_file_goes() {
        let dir = std::env::temp_dir().join(format!("trindle-files-colours-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut k = Kept::from(Some(dir.clone()));
        let (photos, inside) = (PathBuf::from("/tmp/photos"), PathBuf::from("/tmp/photos/cats"));
        k.colour(&inside, Some("Red")).unwrap();
        k.colour(&photos, Some("Teal")).unwrap();
        k.moved(&photos, Path::new("/tmp/pictures")).unwrap();
        let back = Kept::from(Some(dir.clone()));
        assert_eq!(back.colours.get(Path::new("/tmp/pictures/cats")).map(String::as_str), Some("Red"));
        assert_eq!(back.colours.get(Path::new("/tmp/pictures")).map(String::as_str), Some("Teal"));
        assert!(!back.colours.contains_key(&inside));
        k.colour(Path::new("/tmp/pictures2"), Some("Blue")).unwrap();
        k.moved(Path::new("/tmp/pictures"), Path::new("/tmp/p")).unwrap();
        assert!(k.colours.contains_key(Path::new("/tmp/pictures2")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn start_places_are_read_from_their_words() {
        let home = std::env::temp_dir();
        assert_eq!(place_named("recent:", &home), Some(Where::Recent));
        assert_eq!(place_named("trash:", &home), Some(Where::Trash));
        assert_eq!(place_named("~", &home), Some(Where::Folder(home.clone())));
        assert_eq!(place_named("/no/such/folder/here", &home), None);
        assert_eq!(place_named("", &home), None);
    }

    #[test]
    fn widths_that_cannot_be_read_as_text_are_never_written_over() {
        let dir = std::env::temp_dir().join(format!("trindle-files-kept-widths-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let was = b"list size 120\nlist type 150\n\xff\nrich size 110\n".to_vec();
        std::fs::write(dir.join(WIDTHS), &was).unwrap();
        let mut k = Kept::from(Some(dir.clone()));
        k.set_width(Col::Size, false, 90.0);
        let e = k.save_widths().expect_err("written over");
        assert_eq!(e.to_string(), "the file that keeps it is not text and cannot be read");
        assert_eq!(std::fs::read(dir.join(WIDTHS)).unwrap(), was);
        for (name, bytes) in [(PANEL, b"on 31\xff\n".to_vec()), (COLUMNS, b"size \xff date\n".to_vec())] {
            std::fs::write(dir.join(name), &bytes).unwrap();
            let mut k = Kept::from(Some(dir.clone()));
            k.set_panel(false, None);
            assert!(if name == PANEL { k.save_panel().is_err() } else { k.column(Col::Size, false).is_err() }, "{name}");
            assert_eq!(std::fs::read(dir.join(name)).unwrap(), bytes, "{name} is as it was");
        }
        std::fs::write(dir.join(WIDTHS), "list size 120\nlist type 150\n").unwrap();
        let mut k = Kept::from(Some(dir.clone()));
        k.set_width(Col::Size, false, 90.0);
        k.save_widths().unwrap();
        assert_eq!(std::fs::read_to_string(dir.join(WIDTHS)).unwrap(), "list size 90\nlist type 150\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lines_that_are_not_understood_are_kept_through_a_change() {
        let dir = std::env::temp_dir().join(format!("trindle-files-kept-unknown-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let was: [(&str, &[u8]); 4] = [
            (VIEWS, b"icons\t/home/john/Pictures\ngallery\t/home/john/Art\na line with no tab\n\xffodd\t/home/john/Odd\n"),
            (COLOURS, b"Teal\t/home/john/Work\n# a note someone left\n\xff\xfe\t/home/john/Odd\n"),
            (GROUPS, b"kind\tfolder\nshape\tfolder\nkind\tplanet\n"),
            (WIDTHS, b"list size 120\nwide thing 7\nlist flavour 90\n"),
        ];
        for (name, bytes) in was {
            std::fs::write(dir.join(name), bytes).unwrap();
        }
        let mut k = Kept::from(Some(dir.clone()));
        let new = PathBuf::from("/home/john/New");
        k.view(&Where::Folder(new.clone()), View::List).unwrap();
        k.colour(&new, Some("Green")).unwrap();
        k.set_group(&Where::Trash, GroupBy::Date).unwrap();
        k.set_width(Col::Size, false, 90.0);
        k.save_widths().unwrap();
        let lines = |name: &str| -> Vec<Vec<u8>> { std::fs::read(dir.join(name)).unwrap().split(|b| *b == b'\n').filter(|l| !l.is_empty()).map(<[u8]>::to_vec).collect() };
        for (name, bytes) in was {
            let now = lines(name);
            for line in bytes.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
                if line == b"list size 120" {
                    continue;
                }
                assert!(now.iter().any(|l| l == line), "{name} lost the line {:?}", String::from_utf8_lossy(line));
            }
            assert_eq!(now.len(), bytes.split(|b| *b == b'\n').filter(|l| !l.is_empty()).count() + usize::from(name != WIDTHS), "{name}: one line more, and none twice");
        }
        assert!(lines(WIDTHS).contains(&b"list size 90".to_vec()));
        let mut k = Kept::from(Some(dir.clone()));
        assert_eq!(k.view_of(&Where::Folder(new.clone())), Some(View::List));
        assert_eq!(k.colours.get(Path::new("/home/john/Work")).map(String::as_str), Some("Teal"));
        k.colour(&new, None).unwrap();
        assert!(lines(COLOURS).contains(&b"# a note someone left".to_vec()) && lines(COLOURS).len() == 3);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_kept_file_is_never_written_over_where_it_lies() {
        let dir = std::env::temp_dir().join(format!("trindle-files-kept-whole-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let was = [(WIDTHS, "list size 120\n"), (PANEL, "on 300\n"), (COLUMNS, columns::write(&Col::USUAL).leak() as &str), (ICON_SIZE, "64\n"), (LINKED, "off\n"),
                   (PANES, "single\n"), (COLOURS, "Teal\t/home/john/Work\n"), (VIEWS, "icons\t/home/john/Pictures\n")];
        for (name, text) in was {
            std::fs::write(dir.join(name), text).unwrap();
            std::fs::hard_link(dir.join(name), dir.join(format!("{name}.before"))).unwrap();
        }
        let mut k = Kept::from(Some(dir.clone()));
        k.set_width(Col::Size, false, 90.0);
        k.save_widths().unwrap();
        k.set_panel(false, Some(250.0));
        k.save_panel().unwrap();
        k.column(Col::Owner, true).unwrap();
        k.set_icon(128.0).unwrap();
        k.link(true).unwrap();
        k.panes(true).unwrap();
        k.colour(Path::new("/home/john/New"), Some("Green")).unwrap();
        k.view(&Where::Trash, View::Compact).unwrap();
        for (name, text) in was {
            assert_ne!(std::fs::read_to_string(dir.join(name)).unwrap(), text, "{name} was not changed");
            assert_eq!(std::fs::read_to_string(dir.join(format!("{name}.before"))).unwrap(), text, "{name} was written over where it lay");
        }
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2 * was.len());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
