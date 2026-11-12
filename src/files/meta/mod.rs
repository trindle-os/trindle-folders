//! A file's details beyond its name: dates, the read-only and runnable flags, EXIF notes in a photo, and ID3 tags in a song.

use std::fs::{self, File, FileTimes, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::Path;
use std::time::SystemTime;

use crate::toolkit::t;

mod photo;
mod song;
#[cfg(test)]
mod tests;

pub use photo::{exif, exif_upright, write_exif, ExifEdit, Tag};
pub use song::{read_id3, write_id3, Id3};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dates {
    pub modified: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
}

pub fn dates(path: &Path) -> io::Result<Dates> {
    let m = fs::metadata(path).or_else(|_| fs::symlink_metadata(path))?;
    Ok(Dates { modified: m.modified().ok(), accessed: m.accessed().ok() })
}

pub fn set_dates(path: &Path, modified: Option<SystemTime>, accessed: Option<SystemTime>) -> io::Result<()> {
    let file = open_no_link(path)?;
    let mut times = FileTimes::new();
    if let Some(t) = modified {
        times = times.set_modified(t);
    }
    if let Some(t) = accessed {
        times = times.set_accessed(t);
    }
    file.set_times(times)
}

fn open_no_link(path: &Path) -> io::Result<File> {
    match OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path) {
        Err(e) if e.raw_os_error() == Some(libc::ELOOP) => Err(io::Error::new(io::ErrorKind::InvalidInput, t!("folders-meta-link"))),
        other => other,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flags {
    pub read_only: bool,
    pub executable: bool,
}

pub fn flags(path: &Path) -> io::Result<Flags> {
    let m = fs::metadata(path)?;
    let mode = m.permissions().mode();
    Ok(Flags { read_only: mode & 0o222 == 0, executable: m.is_file() && mode & 0o111 != 0 })
}

pub fn set_flags(path: &Path, want: Flags) -> io::Result<()> {
    let m = fs::symlink_metadata(path)?;
    if m.file_type().is_symlink() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, t!("folders-meta-link")));
    }
    let old = m.permissions().mode() & 0o7777;
    let mut mode = old;
    if want.read_only {
        mode &= !0o222;
    } else if mode & 0o222 == 0 {
        mode |= 0o200;
    }
    if m.is_file() {
        if want.executable {
            mode |= (mode & 0o444) >> 2;
        } else {
            mode &= !0o111;
        }
    }
    if mode == old {
        return Ok(());
    }
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
}

fn opened(path: &Path) -> io::Result<File> {
    use crate::common::files::{open_named, readable, Named};
    match open_named(path, None, None)? {
        Named::File(named, _) => readable(&named),
        Named::Link(_) => Err(io::Error::new(io::ErrorKind::InvalidInput, t!("folders-meta-link"))),
        Named::Folder(..) => Err(io::Error::new(io::ErrorKind::Unsupported, t!("kit-files-not-a-file"))),
    }
}

#[cfg(test)]
thread_local! {
    static ROOM: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
}

fn change(path: &Path, size: u64, write: &mut dyn FnMut(&mut dyn Write) -> io::Result<()>) -> io::Result<()> {
    let room = crate::common::files::room_left(path.parent().unwrap_or(Path::new(".")));
    #[cfg(test)]
    let room = ROOM.get().or(room);
    if room.is_some_and(|room| room < size) {
        return Err(io::Error::new(io::ErrorKind::StorageFull, t!("folders-meta-no-room")));
    }
    crate::common::files::write_whole_by(path, &mut |spare| {
        let mut out = crate::common::gentle::Out::new(spare.try_clone()?, &|_| {});
        write(&mut out)?;
        out.finish_now().map(drop)
    })
}
