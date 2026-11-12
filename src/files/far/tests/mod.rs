//! Tests of Files' side of servers and phones.

use std::fs;
use std::path::{Path, PathBuf};

use crate::files::app::Files;
use crate::files::work::{self, Answer, Ctl, Kind, Over, Plan};

use super::*;
use super::copy::{Most, OVER_MOST, UNLISTED_MOST};
use super::links::DIALLED;
use super::window::fetch_dir;
use super::card::{Connect, why};
use super::ends::End;
use super::links::Links;
use super::{SPARE, address, is_far, lock};
use super::window::FarMsg;

mod brought;
mod crew;
mod dialling;
mod strangers;
mod window;

fn place(tag: &str) -> (PathBuf, PathBuf, String) {
    let user = format!("trindle-files-far-{tag}-{}", std::process::id());
    let far = std::env::temp_dir().join(&user);
    let here = std::env::temp_dir().join(format!("{user}-here"));
    for d in [&far, &here] {
        let _ = fs::remove_dir_all(d);
        fs::create_dir_all(d.join("box/inner")).unwrap();
        fs::write(d.join("box/inner/one.txt"), b"one").unwrap();
    }
    fs::write(far.join("song.txt"), vec![5u8; 10_000]).unwrap();
    (here, far, format!("sftp://{user}@test.invalid"))
}

fn go(kind: Kind, items: Vec<PathBuf>, to: PathBuf, fixed: Answer) -> Over {
    let ctl = Ctl { fixed: Some(fixed), ..Ctl::default() };
    work::run(Plan { kind, items, to, key: None }, &ctl, &|_| {})
}

fn spares(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for e in fs::read_dir(dir).unwrap().flatten() {
        if e.path().is_dir() {
            found.extend(spares(&e.path()));
        } else if e.file_name().to_string_lossy().ends_with(SPARE) {
            found.push(e.path());
        }
    }
    found
}

fn names(f: &Files, p: usize) -> Vec<String> {
    let mut n: Vec<String> = f.cols[p].entries.iter().map(|e| e.name.clone()).collect();
    n.sort();
    n
}

fn there(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    names
}

fn tree(dir: &Path) -> Vec<String> {
    fn walk(root: &Path, at: &Path, out: &mut Vec<String>) {
        for e in fs::read_dir(at).unwrap().flatten() {
            let rel = e.path().strip_prefix(root).unwrap().display().to_string();
            if e.path().is_dir() {
                out.push(format!("{rel}/"));
                walk(root, &e.path(), out);
            } else {
                out.push(format!("{rel}={}", fs::metadata(e.path()).unwrap().len()));
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort();
    out
}

#[test]
fn the_source_list_names_every_file_of_the_folder() {
    assert_eq!(SOURCE.len(), crate::common::source::files_in("files/far"));
}
