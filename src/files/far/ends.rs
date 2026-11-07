// The two kinds of end a thing can be at, and what is known of a thing found there.

use std::collections::HashSet;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::files::proof::FarSeal;
use crate::files::remote::{self, Address, Error};
use crate::files::work::{self, Which};
use super::{address, is_far, path_of, root};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum End {
    Here(PathBuf),
    Far(Address),
}

impl End {
    pub(super) fn of(p: &Path) -> Option<End> {
        if is_far(p) { address(p).map(End::Far) } else { Some(End::Here(p.to_path_buf())) }
    }

    pub(super) fn join(&self, name: &str) -> End {
        match self {
            End::Here(p) => End::Here(p.join(name)),
            End::Far(a) => End::Far(a.join(name)),
        }
    }

    pub(super) fn name(&self) -> String {
        match self {
            End::Here(p) => work::name_of(p),
            End::Far(a) => match remote::name_of(path_of(a)) {
                "" => a.host.clone(),
                name => name.to_string(),
            },
        }
    }

    pub(super) fn within(&self, dir: &End) -> bool {
        match (self, dir) {
            (End::Here(p), End::Here(d)) => p.starts_with(d),
            (End::Far(p), End::Far(d)) => {
                let (p, d) = (path_of(p), path_of(d).trim_end_matches('/'));
                root_of(self) == root_of(dir) && (p == d || p.starts_with(&format!("{d}/")) || d.is_empty())
            }
            _ => false,
        }
    }
}

pub(super) fn root_of(e: &End) -> Option<String> {
    match e {
        End::Far(a) => Some(root(a)),
        End::Here(_) => None,
    }
}

pub(super) fn missing(e: &Error) -> bool {
    match e {
        Error::NotFound(_) => true,
        Error::Io(e) => e.kind() == io::ErrorKind::NotFound,
        _ => false,
    }
}

pub(super) struct Info {
    pub(super) folder: bool,
    pub(super) size: u64,
    pub(super) modified: Option<i64>,
    pub(super) which: Option<Which>,
}

impl Info {
    pub(super) fn seal(&self) -> FarSeal {
        FarSeal { size: self.size, modified: self.modified }
    }
}

pub(super) struct Found {
    pub(super) folder: bool,
    pub(super) size: u64,
    pub(super) modified: Option<i64>,
    pub(super) here: Option<(File, fs::Metadata)>,
}

#[derive(Clone, Copy)]
pub(super) struct Up<'a> {
    pub(super) dir: Option<&'a File>,
    pub(super) known: Option<&'a HashSet<Which>>,
    pub(super) shared: Option<&'a Arc<File>>,
    pub(super) top: bool,
}

pub(super) enum Child {
    Go(String),
    Left(String, &'static str),
}

#[derive(Clone)]
pub(super) enum Got {
    Done(End),
    Part(End, bool),
    Stayed,
    Skipped,
    Failed,
    Later(usize),
}

pub(super) fn claim_key(e: &End) -> String {
    match e {
        End::Here(p) => p.to_string_lossy().into_owned(),
        End::Far(a) => a.to_string(),
    }
}
