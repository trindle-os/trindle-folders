// The password a person typed and the troubles an archive can give.

use std::io::{self, Read};

use zeroize::Zeroizing;

use crate::files::meter::is_stopped;
use crate::toolkit::t;

#[derive(Clone)]
pub struct Key(Zeroizing<String>);

impl Key {
    pub fn new(typed: &str) -> Key {
        Key(Zeroizing::new(typed.to_string()))
    }

    pub(super) fn text(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trouble {
    Locked,
    WrongKey,
    WrongOrBad,
    LockedRar,
    LockedForeign,
    Unknown,
    TooMany,
    NoRoom,
    GaveUp,
    NoReader,
    Foreign,
    Parts,
}

impl std::fmt::Display for Trouble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Trouble::Locked => t!("folders-archive-trouble-locked"),
            Trouble::WrongKey => t!("folders-archive-trouble-wrong-key"),
            Trouble::WrongOrBad => t!("folders-archive-trouble-wrong-or-bad"),
            Trouble::LockedRar => t!("folders-archive-trouble-locked-rar"),
            Trouble::LockedForeign => t!("folders-archive-trouble-locked-foreign"),
            Trouble::Unknown => t!("folders-archive-trouble-unknown"),
            Trouble::TooMany => t!("folders-archive-trouble-too-many"),
            Trouble::NoRoom => t!("folders-archive-trouble-no-room"),
            Trouble::GaveUp => t!("folders-archive-trouble-gave-up"),
            Trouble::NoReader => t!("folders-archive-trouble-no-reader"),
            Trouble::Foreign => t!("folders-archive-trouble-foreign"),
            Trouble::Parts => t!("folders-archive-trouble-parts"),
        })
    }
}

impl std::error::Error for Trouble {}

pub(in crate::files) fn trouble(t: Trouble) -> io::Error {
    let kind = match t {
        Trouble::Locked | Trouble::WrongKey | Trouble::WrongOrBad => io::ErrorKind::PermissionDenied,
        Trouble::LockedRar | Trouble::LockedForeign | Trouble::Foreign | Trouble::NoReader => io::ErrorKind::Unsupported,
        Trouble::NoRoom => io::ErrorKind::StorageFull,
        Trouble::GaveUp => io::ErrorKind::OutOfMemory,
        Trouble::Unknown | Trouble::TooMany => io::ErrorKind::InvalidData,
        Trouble::Parts => io::ErrorKind::NotFound,
    };
    io::Error::new(kind, t)
}

pub fn trouble_of(e: &io::Error) -> Option<Trouble> {
    e.get_ref().and_then(|inner| inner.downcast_ref::<Trouble>()).copied()
}

pub fn why(e: &io::Error) -> String {
    if let Some(t) = trouble_of(e) {
        return t.to_string();
    }
    match e.kind() {
        io::ErrorKind::InvalidData | io::ErrorKind::UnexpectedEof => t!("folders-archive-why-damaged"),
        io::ErrorKind::StorageFull => t!("folders-archive-why-no-room"),
        io::ErrorKind::PermissionDenied | io::ErrorKind::ReadOnlyFilesystem => t!("folders-archive-why-read-only"),
        io::ErrorKind::NotFound => t!("folders-archive-why-gone"),
        io::ErrorKind::OutOfMemory => t!("folders-archive-why-memory"),
        _ => t!("folders-archive-why-other"),
    }
    .to_string()
}

pub(super) fn key_err(e: io::Error, t: Trouble) -> io::Error {
    if trouble_of(&e).is_some() || is_stopped(&e) {
        return e;
    }
    match e.kind() {
        io::ErrorKind::InvalidData | io::ErrorKind::UnexpectedEof | io::ErrorKind::InvalidInput => trouble(t),
        _ => e,
    }
}

pub(super) struct Sealed<R>(pub(super) R);

impl<R: Read> Read for Sealed<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf).map_err(|e| key_err(e, Trouble::WrongOrBad))
    }
}

pub fn wants_key(e: &io::Error) -> Option<Trouble> {
    trouble_of(e).filter(|t| matches!(t, Trouble::Locked | Trouble::WrongKey | Trouble::WrongOrBad))
}
