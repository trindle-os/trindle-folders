// What an act says of the things it did, and the names it refuses.

use std::path::{Path, PathBuf};

use crate::files::far;
use crate::files::places::Where;
use crate::files::work::{self, Extra, Kind, Over};
use crate::toolkit::t;
use super::NAMES_SAID;
use super::types::Words;

pub(super) fn label_what(label: &str, again: bool) -> String {
    let mark = "\u{1}";
    let shape = if again { t!("folders-act-redo-label", what = mark) } else { t!("folders-act-undo-label", what = mark) };
    shape.split_once(mark)
        .and_then(|(before, after)| label.strip_prefix(before)?.strip_suffix(after))
        .unwrap_or(label)
        .to_string()
}

pub(super) fn what(items: &[PathBuf]) -> String {
    match items {
        [one] => work::name_of(one),
        _ => t!("folders-act-items", n = items.len()),
    }
}

pub(in crate::files) fn failed_words(over: &Over, verb: &str) -> String {
    let count = |list: &[String]| t!("folders-act-failed", n = list.len(), verb = verb, names = &names(list));
    match (over.not_kept.is_empty(), over.failed.is_empty()) {
        (true, _) => count(&over.failed),
        (false, true) => count(&over.troubles()),
        (false, false) => t!("folders-act-not-kept-and-failed", kept = &over.not_kept.join(", "), why = work::not_kept(), rest = &count(&over.failed)),
    }
}

pub(super) fn names(list: &[String]) -> String {
    let shown = &list[..list.len().min(NAMES_SAID)];
    let out = shown.join(", ");
    if list.len() > NAMES_SAID {
        return t!("folders-act-names-more", names = &out, more = list.len() - NAMES_SAID);
    }
    out
}

pub fn words(kind: &Kind, items: &[PathBuf], to: &Path, home: &Path) -> Words {
    let w = what(items);
    let place = if far::is_far(to) { Where::Remote(to.to_string_lossy().into_owned()) } else { Where::Folder(to.to_path_buf()) }.name(home);
    let (doing, done, verb, undo) = match kind {
        Kind::Copy => (t!("folders-act-copying", what = &w, place = &place), t!("folders-act-copied", what = &w, place = &place), t!("folders-act-verb-copied"), t!("folders-act-undo-copying", what = &w)),
        Kind::Move => (t!("folders-act-moving", what = &w, place = &place), t!("folders-act-moved", what = &w, place = &place), t!("folders-act-verb-moved"), t!("folders-act-undo-moving", what = &w)),
        Kind::Trash => (t!("folders-act-trashing", what = &w), t!("folders-act-trashed", what = &w), t!("folders-act-verb-trashed"),
                        t!("folders-act-undo-trashing", what = &w)),
        Kind::Delete => (t!("folders-act-deleting", what = &w), t!("folders-act-deleted", what = &w), t!("folders-act-verb-deleted"), String::new()),
        Kind::Restore => (t!("folders-act-restoring", what = &w), t!("folders-act-restored", what = &w), t!("folders-act-verb-put-back"), t!("folders-act-undo-restoring", what = &w)),
        Kind::Undo(_) => (t!("folders-act-undoing-plain").to_string(), t!("folders-act-undone").to_string(), t!("folders-act-verb-put-back"), String::new()),
        Kind::Other(Extra::Pack(format, name)) => (t!("folders-act-compressing", what = &w), t!("folders-act-compressed", file = &format!("{name}{}", format.ending()), place = &place), t!("folders-act-verb-packed"),
                                     t!("folders-act-undo-compressing", what = &w)),
        Kind::Other(Extra::Unpack(_)) => (t!("folders-act-extracting", what = &w), t!("folders-act-extracted", what = &w, place = &place), t!("folders-act-verb-extracted"), t!("folders-act-undo-extracting", what = &w)),
        Kind::Other(Extra::Wipe(_)) => (t!("folders-act-wiping", what = &w), t!("folders-act-wiped", what = &w), t!("folders-act-verb-wiped"), String::new()),
        Kind::Other(Extra::TakeOut(_)) => (t!("folders-act-taking-out", what = &w), t!("folders-act-took-out", what = &w, place = &place), t!("folders-act-verb-taken-out"), t!("folders-act-undo-taking-out", what = &w)),
        Kind::Other(Extra::Picture(crate::files::imaging::Work::Turn(_))) => (t!("folders-act-turning", what = &w), t!("folders-act-turned", what = &w), t!("folders-act-verb-turned"), String::new()),
        Kind::Other(Extra::Picture(crate::files::imaging::Work::Resize(_))) => (t!("folders-act-resizing", what = &w), t!("folders-act-resized", what = &w), t!("folders-act-verb-resized"),
                                                         t!("folders-act-undo-resizing", what = &w)),
    };
    let places: Vec<&Path> = items.iter().map(PathBuf::as_path).chain(std::iter::once(to)).collect();
    let help = match kind {
        Kind::Other(Extra::Unpack(_) | Extra::TakeOut(_)) => crate::files::help::work_page(kind),
        _ => crate::files::help::server_work_page(&places).unwrap_or_else(|| crate::files::help::work_page(kind)),
    };
    Words { doing, done, verb, help, undo }
}

const NAME_MOST: usize = 255;

pub(in crate::files) fn bad_name(name: &str) -> Option<&'static str> {
    if name.trim().is_empty() {
        Some(t!("folders-act-name-empty"))
    } else if name.contains('/') {
        Some(t!("folders-act-name-slash"))
    } else if name == "." || name == ".." {
        Some(t!("folders-act-name-kept"))
    } else if name.len() > NAME_MOST {
        Some(t!("folders-act-name-long"))
    } else {
        None
    }
}
