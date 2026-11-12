//! Facts about the things an act is done to, and the steps that undo it.

use std::fs::{self, OpenOptions};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::files::inside;
use crate::files::trash;
use crate::files::work::{self, Extra, Kind, Plan, Step};
use crate::toolkit::t;
use super::UNDO_KEPT;

pub(super) fn keep(steps: &mut Vec<(String, Vec<Step>)>, label: String, taken: Vec<Step>) {
    if label.is_empty() || taken.is_empty() {
        return;
    }
    steps.push((label, taken));
    if steps.len() > UNDO_KEPT {
        steps.remove(0);
    }
}

pub(super) fn touched(plan: &Plan, bin: Option<&trash::Bin>) -> Option<Vec<PathBuf>> {
    let named = |p: &PathBuf| p.file_name().map(|n| plan.to.join(n));
    let mut out: Vec<PathBuf> = match &plan.kind {
        Kind::Copy | Kind::Other(Extra::TakeOut(_)) => plan.items.iter().filter_map(named).collect(),
        Kind::Move => plan.items.iter().filter_map(named).chain(plan.items.iter().cloned()).collect(),
        Kind::Other(Extra::Pack(_, name)) => vec![plan.to.join(name)],
        Kind::Other(Extra::Unpack(_)) => vec![plan.to.clone()],
        Kind::Delete | Kind::Other(Extra::Wipe(_)) => plan.items.clone(),
        Kind::Other(Extra::Picture(_)) => plan.items.iter().cloned().chain(plan.items.iter().filter_map(|i| i.parent().map(Path::to_path_buf))).collect(),
        Kind::Trash => {
            let mut v = plan.items.clone();
            for item in &plan.items {
                match bin {
                    Some(b) => v.push(b.files.clone()),
                    None => v.extend(trash::bin_for(item).ok().map(|b| b.files)),
                }
            }
            v
        }
        Kind::Restore => plan.items.iter().cloned().chain(plan.items.iter().filter_map(|i| trash::origin(i))).collect(),
        Kind::Undo(steps) => {
            let mut v = Vec::new();
            for step in steps {
                match step {
                    Step::Remove(p) => v.push(p.clone()),
                    Step::Back { now, was } | Step::Retrash { now, was, .. } => v.extend([now.clone(), was.clone()]),
                    Step::Restore(_) | Step::Other(_) => return None,
                }
            }
            v
        }
    };
    out.sort();
    out.dedup();
    Some(out)
}

pub(in crate::files) fn picked_up(items: &[PathBuf], aside: bool) -> Vec<Option<crate::files::proof::Picked>> {
    items.iter().map(|i| if crate::files::far::is_far(i) || inside::within(i).is_some() { None } else { crate::files::proof::picked_up(i, aside) }).collect()
}

pub(in crate::files) fn pinnable(items: &[PathBuf]) -> bool {
    !items.is_empty() && items.iter().all(|i| !crate::files::far::is_far(i) && inside::within(i).is_none() && crate::files::reach::is_dir(i))
}

pub(super) fn same_drive(a: &Path, b: &Path) -> bool {
    match (crate::files::reach::meta(a, false).ok_or(()), crate::files::reach::meta(b, true).ok_or(())) {
        (Ok(x), Ok(y)) => x.dev() == y.dev(),
        _ => false,
    }
}

pub(super) fn trashed(path: &Path) -> bool {
    path.parent().and_then(Path::file_name).is_some_and(|n| n == "files") && trash::origin(path).is_some()
}

pub(in crate::files) fn make_new(dir: &Path, folder: bool) -> Result<PathBuf, String> {
    let base = if folder { t!("folders-act-new-folder") } else { t!("folders-act-new-document") };
    let plain = dir.join(base);
    if !crate::files::reach::answers(dir) {
        return Err(crate::files::reach::not_answering().into());
    }
    let path = if fs::symlink_metadata(&plain).is_err() { plain } else { work::free_name(dir, base.as_ref(), folder) };
    let made = if folder { fs::create_dir(&path) } else { OpenOptions::new().write(true).create_new(true).open(&path).map(drop) };
    made.map(|()| path).map_err(|e| t!("folders-act-make-failed", base = base, why = &e.to_string()))
}
