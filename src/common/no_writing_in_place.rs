// Nothing a person or the computer would miss is written over where it lies.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const ALLOWED: [(&str, usize, &str); 10] = [
    ("toolkit/common/file_work/gentle/mod.rs", 1, "copy_named, which copies gently into a folder its caller made for that alone: a part of a package put together in a folder of its own"),
    ("toolkit/sys/gbm.rs", 1, "the graphics card's render node in /dev/dri, opened to draw with: a device, not a file on a drive, with nothing in it to lose"),
    ("files/index.rs", 1, "the search index is written to a part file and renamed; it is built again when missing"),
    ("files/thumbs.rs", 2, "a thumbnail is written to a part file, opened over two lines that are each counted, and renamed; it is made again from its picture"),
    ("common/sharing.rs", 1, "an empty mark"),
    ("common/keyring_said.rs", 1, "a lock file in the session's own folder, opened and never written"),
    ("files/remote/usb.rs", 1, "a phone's or camera's USB device, which is talked to and is no file on a drive"),
    ("files/far/window.rs", 1, "a file fetched from a server to look at, still on the server"),
    ("files/preview.rs", 1, "a file read out of an archive to look at, still in the archive"),
    ("files/wipe.rs", 3, "Wipe securely, which is asked to write over the file where it lies and says so before it does"),
];

const TAKES: [(&str, usize, &str); 21] = [
    ("common/earlier.rs", 2, "putting an earlier version back, as asked: its own spare copy cleared (the renames into place and back go through common::files::rename_new)"),
    ("files/archive/pack.rs", 2, "spare names of this work's own beside what is made: the spare archive after a failure or once it has its real name"),
    ("files/archive/unpack.rs", 4, "spare names of this work's own beside what is made: the folder an archive is unpacked in, and an earlier entry in that folder a later one replaces"),
    ("files/askpass.rs", 1, "a link to this very program in the session's own folder, made again when it leads elsewhere"),
    ("toolkit/common/file_work/clip.rs", 1, "Folders' own note of what it last cut, in the session's own folder, once the cut is pasted or something else is copied"),
    ("files/far/copy.rs", 1, "the name of a spare file once it is open"),
    ("files/far/hands.rs", 2, "an empty folder a move has emptied, by the call that cannot remove one with anything in it, and a spare file this work made itself and could not finish"),
    ("files/far/window.rs", 1, "a half-fetched copy made to be looked at"),
    ("files/imaging.rs", 2, "a picture turned as asked: the turned copy, made new beside it under a name of the turn's own, is renamed over it whole, or removed if that could not be done"),
    ("files/index.rs", 1, "the search index's part file renamed over the old index; it is built again when missing"),
    ("files/inside.rs", 2, "copies taken out of archives to be opened, in Trindle's own cache: all of them as it starts, and an older copy before a fresh one; each is still in its archive"),
    ("files/locked.rs", 2, "copies taken out of a locked archive to be opened, in Trindle's own cache, removed as it closes so nothing unlocked stays; each is still in its archive"),
    ("files/preview.rs", 2, "the copy of a file from a locked archive made to be looked at, in Trindle's own cache, removed when the viewer closes; it is still in the archive"),
    ("toolkit/common/file_work/proof.rs", 1, "the half-made copy this work made itself, while its name still leads to it: the third of the rule's own three, whose other two are in toolkit/sys/disk.rs"),
    ("toolkit/sys/disk.rs", 2, "the rule's own three: a thing given a name of this work's own within its opened folder, one name removed within an opened folder once it is proved to be the thing copied or asked about, and a half-made copy while its name still leads to it"),
    ("toolkit/sys/os.rs", 1, "the rename that takes no name already taken, which common::files::rename_new calls"),
    ("files/thumbs.rs", 2, "a thumbnail's part file, renamed into place or removed after a failure; it is made again from its picture"),
    ("toolkit/common/file_work/trash.rs", 3, "the Trash's own note of an item: one made for a name that turned out taken or for a move that failed, and one whose item has left the Trash"),
    ("files/wipe.rs", 2, "a secure wipe, which the person asks for and is told cannot be undone: a file written over, a link or an empty folder is given a name that says nothing, looked at under it, and removed only if it is the thing that was listed"),
    ("toolkit/common/file_work/work/finish.rs", 3, "folders a move has emptied, by the call that cannot remove one with anything in it, and what this work made under a spare name of its own and could not put in place; Delete itself goes by proof::Asked"),
    ("toolkit/common/file_work/work/leave.rs", 1, "a folder a move has emptied, by the call that cannot remove one with anything in it"),
];

fn ways() -> [String; 8] {
    [["fs::", "write("].concat(), ["File::", "create("].concat(), [".truncate", "(true)"].concat(), ["fs::", "copy("].concat(),
     ["set_", "len("].concat(), [".append", "(true)"].concat(), ["libc::O_", "WRONLY"].concat(), ["libc::O_", "RDWR"].concat()]
}

fn opened_to_write() -> String {
    [".write", "(true)"].concat()
}

fn made_new() -> String {
    ["create_", "new("].concat()
}

const NEAR: usize = 2;

fn takes() -> [String; 6] {
    [["fs::", "rename("].concat(), ["remove_", "file("].concat(), ["remove_", "dir_all("].concat(), ["remove_", "dir("].concat(),
     ["libc::", "rename"].concat(), ["libc::", "unlink"].concat()]
}

fn for_tests() -> String {
    ["#[cfg(", "test)]"].concat()
}

fn for_tests_too() -> String {
    ["#[cfg(any(", "test, feature = \"testing\"))]"].concat()
}

pub(crate) fn code(text: &str) -> Vec<&str> {
    let lines: Vec<&str> = text.lines().collect();
    let (mark, too) = (for_tests(), for_tests_too());
    let depth = |l: &str| l.len() - l.trim_start().len();
    let mut out = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        if !lines[i].trim_start().starts_with(&mark) && !lines[i].trim_start().starts_with(&too) {
            out.push(lines[i]);
            i += 1;
            continue;
        }
        let deep = depth(lines[i]);
        let mut first = i + 1;
        while lines.get(first).is_some_and(|l| l.trim_start().starts_with("#[")) {
            first += 1;
        }
        let begun = lines.get(first).map_or("", |l| l.trim_end());
        let whole = begun.ends_with(';') || begun.ends_with(',') || (begun.ends_with('}') && begun.contains('{'));
        let block = begun.ends_with('{');
        let ends = |l: &str| l.trim_start().starts_with('}') || (!block && (l.trim_end().ends_with(';') || l.trim_end().ends_with(',')));
        let end = if whole { first } else { (first + 1..lines.len()).find(|&k| depth(lines[k]) == deep && ends(lines[k])).unwrap_or(lines.len() - 1) };
        let end = end.min(lines.len() - 1);
        out.extend(std::iter::repeat_n("", end + 1 - i));
        i = end + 1;
    }
    out
}

fn count(text: &str, ways: &[String]) -> usize {
    let lines = code(text);
    let (opened, new) = (opened_to_write(), made_new());
    let said = |l: &str| !l.trim_start().starts_with("//");
    (0..lines.len())
        .filter(|&i| said(lines[i]))
        .filter(|&i| {
            let near = &lines[i.saturating_sub(NEAR)..(i + NEAR + 1).min(lines.len())];
            ways.iter().any(|w| lines[i].contains(w.as_str())) || (lines[i].contains(&opened) && !near.iter().any(|l| l.contains(&new)))
        })
        .count()
}

fn taken(text: &str, takes: &[String]) -> usize {
    code(text).iter().filter(|l| !l.trim_start().starts_with("//") && takes.iter().any(|w| l.contains(w.as_str()))).count()
}

pub(crate) fn all_code() -> Vec<(String, String)> {
    let mut files = Vec::new();
    let mut names = std::collections::HashMap::new();
    for (src, prefix) in crate::common::source::trees() {
        let before = files.len();
        sources(&src, &mut files);
        for f in &files[before..] {
            names.insert(f.clone(), format!("{prefix}{}", f.strip_prefix(&src).unwrap_or(f).to_string_lossy()));
        }
    }
    let texts: Vec<(PathBuf, String)> = files.into_iter().filter_map(|p| std::fs::read_to_string(&p).ok().map(|t| (p, t))).collect();
    let mut tests = BTreeSet::new();
    for (path, text) in &texts {
        only_for_tests(path, text, &mut tests);
    }
    texts.into_iter()
        .filter(|(path, _)| !path.ancestors().any(|a| tests.contains(a)))
        .map(|(path, text)| (names[&path].clone(), text))
        .collect()
}

fn only_for_tests(path: &Path, text: &str, out: &mut BTreeSet<PathBuf>) {
    let lines: Vec<&str> = text.lines().collect();
    let own = matches!(path.file_name().and_then(|n| n.to_str()), Some("mod.rs" | "main.rs" | "lib.rs"));
    let Some(dir) = path.parent() else { return };
    let dir = if own { dir.to_path_buf() } else { dir.join(path.file_stem().unwrap_or_default()) };
    for pair in lines.windows(2) {
        let name = pair[1].trim().trim_start_matches("pub ").strip_prefix("mod ").and_then(|m| m.strip_suffix(';'));
        if let (true, Some(name)) = (pair[0].trim().starts_with("#[cfg(test)]") || pair[0].trim().starts_with(&for_tests_too()), name) {
            out.insert(dir.join(format!("{name}.rs")));
            out.insert(dir.join(name));
        }
    }
}

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(list) = std::fs::read_dir(dir) else { return };
    for entry in list.flatten() {
        let path = entry.path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|x| x == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn a_write_is_counted_outside_comments_and_the_files_own_tests() {
    let ways = ways();
    let [write, create, truncate, copy, length, append, ..] = &ways;
    let text = format!("fn a() {{ std::{write}p, b) }}\n// std::{write}p, b)\nlet f = {create}p);\nopen{truncate}.open(p);\n{copy}a, b);\n\
                        fs::read(p);\n#[cfg(test)]\nmod tests {{\n    std::{write}p, b);\n}}\n");
    assert_eq!(count(&text, &ways), 4);
    let (opened, new) = (opened_to_write(), made_new());
    let unseen = format!("let f = OpenOptions::new(){opened}.open(p)?;\nf.{length}0)?;\nlet g = OpenOptions::new(){append}.open(q)?;\n\
                          let h = File::options().read(true){opened}.create(true).truncate(false).open(r)?;\nfs::read(p);\n");
    assert_eq!(count(&unseen, &ways), 4, "a file opened to be written, given a length or added to is written where it lies");
    let fresh = format!("let f = OpenOptions::new(){opened}.{new}true).open(p)?;\nlet g = OpenOptions::new()\n    {opened}\n    .{new}true)\n    .open(q)?;\n");
    assert_eq!(count(&fresh, &ways), 0);
    let far = format!("let f = OpenOptions::new(){opened}.open(p)?;\n\n\n\nlet g = OpenOptions::new().{new}true).open(q)?;\n");
    assert_eq!(count(&far, &ways), 1, "one made new some lines off excuses nothing");
    let [rename, file, all, dir, ..] = &takes();
    let gone = format!("{rename}a, b)?;\nstd::fs::{file}a)?;\n// fs::{file}a)\nfs::{all}d)?;\nfs::{dir}d)?;\nfs::read(p);\n#[cfg(test)]\nmod tests {{\n    fs::{file}a);\n}}\n");
    assert_eq!(taken(&gone, &takes()), 4);
    let mark = for_tests();
    let amid = format!("{rename}a, b)?;\n{mark}\nmod tests {{\n    fs::{file}a);\n    fn f() {{\n    }}\n}}\n\nfn later() {{\n    fs::{file}a)?;\n    {mark}\n    fs::{file}t);\n    \
                        {mark}\n    if testing() {{\n        fs::{all}d);\n    }}\n    fs::{dir}d)?;\n}}\n{mark}\npub fn only_tests() {{\n    fs::{file}a);\n}}\n{mark}\n#[allow(dead_code)]\n\
                        fn long(\n    a: u8,\n) {{\n    fs::{file}a);\n}}\nfn last() {{ fs::{all}d); }}\n{mark}\nmod at_the_end {{\n    fs::{file}a);\n}}\n");
    assert_eq!(taken(&amid, &takes()), 4, "the code after a module of tests is counted, and the tests are not");
    assert_eq!(code(&amid).len(), amid.lines().count(), "each line stays where it was");
    let raw = format!("let fd = libc::open(p, {} | libc::O_APPEND, 0o600);\n", ways[6]);
    assert_eq!(count(&raw, &ways), 1, "a file opened to be written by the kernel's own flag is written where it lies");
    let mut found = BTreeSet::new();
    only_for_tests(Path::new("src/files/mod.rs"), "mod act;\n#[cfg(test)]\nmod shots;\n#[cfg(test)]\npub mod rig;\n", &mut found);
    assert!(found.contains(Path::new("src/files/shots.rs")) && found.contains(Path::new("src/files/rig")) && !found.contains(Path::new("src/files/act.rs")));
}

#[test]
fn nothing_is_written_over_where_it_lies_but_what_is_listed() {
    let ways = ways();
    let found: BTreeMap<String, usize> = all_code().into_iter().map(|(file, text)| (file, count(&text, &ways))).filter(|(_, n)| *n > 0).collect();
    let listed: BTreeMap<String, usize> = ALLOWED.iter().map(|(file, n, _)| (file.to_string(), *n)).collect();
    assert_eq!(listed.len(), ALLOWED.len(), "a file is listed twice");
    assert!(ALLOWED.iter().all(|(_, _, why)| !why.is_empty()), "each one listed says why");
    let wrong: Vec<String> = found.iter().filter(|(file, n)| listed.get(*file) != Some(n))
        .map(|(file, n)| format!("{file} writes a file where it lies {n} times, and {} are listed", listed.get(file).copied().unwrap_or(0)))
        .chain(listed.keys().filter(|file| !found.contains_key(*file)).map(|file| format!("{file} is listed and no longer writes any")))
        .collect();
    assert!(wrong.is_empty(), "use common::files::write_whole, or list it in ALLOWED with its reason: {wrong:#?}");
}

#[test]
fn nothing_is_taken_away_or_named_anew_but_where_it_is_counted() {
    let takes = takes();
    let found: BTreeMap<String, usize> = all_code().into_iter().map(|(file, text)| (file, taken(&text, &takes))).filter(|(_, n)| *n > 0).collect();
    let listed: BTreeMap<String, usize> = TAKES.iter().map(|(file, n, _)| (file.to_string(), *n)).collect();
    assert_eq!(listed.len(), TAKES.len(), "a file is listed twice");
    assert!(TAKES.iter().all(|(_, _, why)| !why.is_empty()), "each one counted says why");
    let wrong: Vec<String> = found.iter().filter(|(file, n)| listed.get(*file) != Some(n))
        .map(|(file, n)| format!("{file} takes a thing away or names it anew {n} times, and {} are counted", listed.get(file).copied().unwrap_or(0)))
        .chain(listed.keys().filter(|file| !found.contains_key(*file)).map(|file| format!("{file} is counted and no longer takes any away")))
        .collect();
    assert!(wrong.is_empty(), "a person's own thing goes only by its proof or into the Trash, and is named only by common::files::rename_new; \
                               when the new one has been looked at, count it in TAKES and say there why it may stay: {wrong:#?}");
}
