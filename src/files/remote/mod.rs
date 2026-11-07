// Places that are not on this computer's own disks.

mod address;
mod dav;
mod ftp;
mod mtp;
mod screen;
mod sftp;
mod tls;
mod usb;

use std::fmt;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;

use crate::files::leave::Leave;
use crate::toolkit::t;

pub use address::{Address, Scheme};
pub use mtp::{phones, Phone};
pub use screen::{not_a_name, Listing, Screened};
pub use usb::Plugged;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    File,
    Folder,
    Link,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub kind: Kind,
    pub size: u64,
    pub modified: Option<i64>,
    pub mode: Option<u32>,
}

impl Entry {
    pub fn is_folder(&self) -> bool {
        self.kind == Kind::Folder
    }
}

#[derive(Clone, Debug, Default)]
pub struct Login {
    pub password: Option<String>,
    pub key: Option<PathBuf>,
    pub askpass: Option<PathBuf>,
    pub trust_new_host: bool,
    pub trust_cert: Option<String>,
}

pub type Progress<'a> = &'a mut dyn FnMut(u64) -> bool;

pub trait Remote: Send {
    fn home(&mut self) -> Result<String, Error>;
    fn list(&mut self, path: &str) -> Result<Vec<Entry>, Error>;
    fn stat(&mut self, path: &str) -> Result<Entry, Error>;
    fn read(&mut self, path: &str, to: &mut dyn Write, progress: Progress) -> Result<u64, Error>;
    fn write(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error>;
    fn write_new(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        match self.stat(path) {
            Ok(_) => return Err(Error::Exists(name_of(path).to_string())),
            Err(Error::NotFound(_)) => {}
            Err(Error::Io(e)) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        self.write(path, from, size, progress)
    }
    fn refuses_taken(&self) -> bool {
        false
    }
    fn mkdir(&mut self, path: &str) -> Result<(), Error>;
    fn rmdir(&mut self, path: &str) -> Result<(), Error>;
    fn remove(&mut self, path: &str) -> Result<(), Error>;
    fn remove_if(&mut self, path: &str, size: u64, modified: Option<i64>) -> Result<bool, Error> {
        let now = self.stat(path)?;
        if now.is_folder() || now.size != size || now.modified != modified {
            return Ok(false);
        }
        self.remove(path).map(|()| true)
    }
    fn rename(&mut self, from: &str, to: &str) -> Result<(), Error>;
    fn readlink(&mut self, _path: &str) -> Result<String, Error> {
        Err(Error::NotSupported(t!("folders-remote-mod-links").into()))
    }
    fn follow(&mut self, path: &str) -> Result<Entry, Error> {
        let target = self.readlink(path)?;
        let at = if target.starts_with('/') { target } else { format!("{}/{target}", parent(path).trim_end_matches('/')) };
        let mut e = self.stat(&at)?;
        e.name = name_of(path).to_string();
        Ok(e)
    }
}

#[cfg_attr(test, allow(dead_code))]
pub fn connect(address: &Address, to: &Leave<Login>) -> Result<Screened, Error> {
    let far: Box<dyn Remote> = match address.scheme {
        Scheme::Sftp => Box::new(sftp::Sftp::connect(address, to.login())?),
        Scheme::Ftp | Scheme::Ftps | Scheme::Ftpes => Box::new(ftp::Ftp::connect(address, to)?),
        Scheme::Mtp => Box::new(mtp::Mtp::open(&address.host)?),
        Scheme::Dav | Scheme::Davs => Box::new(dav::Dav::connect(address, to)?),
        Scheme::Smb => return Err(Error::NotSupported(t!("folders-remote-mod-copy-share").into())),
        Scheme::Nfs => return Err(Error::NotSupported(t!("folders-remote-mod-copy-nfs").into())),
    };
    Ok(Screened::over(far))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Heard {
    Card(u16),
    Stopped,
}

pub fn heard(host: &str, at: &str, said: Heard) -> String {
    match said {
        Heard::Card(port) => t!("folders-remote-mod-heard-card", host = host, at = at, port = port),
        Heard::Stopped => t!("folders-remote-mod-heard-stopped", host = host, at = at),
    }
}

pub fn heard_words(host: &str, at: &str) -> String {
    heard(host, at, Heard::Stopped)
}

impl crate::files::leave::Carries for Login {
    fn secret(&self) -> bool {
        self.password.is_some()
    }
}

#[derive(Debug)]
pub enum Error {
    BadAddress(String),
    NeedsPassword,
    LoginRefused,
    HostKeyUnknown(String),
    HostKeyChanged(String),
    CertUnknown { host: String, fingerprint: String, why: String },
    CertChanged { host: String, fingerprint: String },
    Key(String),
    Heard { host: String, at: String, port: u16 },
    Unreachable(String),
    NotFound(String),
    Exists(String),
    Denied(String),
    Busy(String),
    NotSupported(String),
    Cancelled,
    Lost(String),
    Failed(String),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadAddress(why) => f.write_str(&t!("folders-remote-mod-bad-address", why = why)),
            Error::NeedsPassword => f.write_str(t!("folders-remote-mod-needs-password")),
            Error::LoginRefused => f.write_str(t!("folders-remote-mod-login-refused")),
            Error::HostKeyUnknown(host) => f.write_str(&t!("folders-remote-mod-host-key-unknown", host = host)),
            Error::HostKeyChanged(host) => f.write_str(&t!("folders-remote-mod-host-key-changed", host = host)),
            Error::CertUnknown { host, why, .. } => f.write_str(&t!("folders-remote-mod-cert-unknown", host = host, why = why)),
            Error::CertChanged { host, .. } => f.write_str(&t!("folders-remote-mod-cert-changed", host = host)),
            Error::Key(why) => f.write_str(why),
            Error::Heard { host, at, .. } => f.write_str(&heard_words(host, at)),
            Error::Unreachable(why) => f.write_str(&t!("folders-remote-mod-unreachable", why = why)),
            Error::NotFound(what) => f.write_str(&t!("folders-remote-mod-not-found", what = what)),
            Error::Exists(what) => f.write_str(&t!("folders-remote-mod-exists", what = what)),
            Error::Denied(what) => f.write_str(&t!("folders-remote-mod-denied", what = what)),
            Error::Busy(what) => f.write_str(&t!("folders-remote-mod-busy", what = what)),
            Error::NotSupported(what) => f.write_str(&t!("folders-remote-mod-not-supported", what = what)),
            Error::Cancelled => f.write_str(t!("folders-remote-mod-cancelled")),
            Error::Lost(why) | Error::Failed(why) => f.write_str(why),
            Error::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

pub(super) fn too_much() -> Error {
    Error::Lost(t!("folders-remote-mod-too-much").into())
}

pub(super) fn too_much_read() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, t!("folders-remote-mod-too-much"))
}

pub(super) fn read_line_most(from: &mut dyn BufRead, most: usize, raw: &mut Vec<u8>) -> io::Result<usize> {
    let got = Read::take(&mut *from, most as u64).read_until(b'\n', raw)?;
    if got == most && raw.last() != Some(&b'\n') {
        return Err(too_much_read());
    }
    Ok(got)
}

pub fn name_of(path: &str) -> &str {
    path.trim_end_matches('/').rsplit('/').next().unwrap_or("")
}

pub fn join(dir: &str, name: &str) -> String {
    if dir.ends_with('/') {
        format!("{dir}{name}")
    } else {
        format!("{dir}/{name}")
    }
}

pub fn parent(path: &str) -> String {
    let path = path.trim_end_matches('/');
    match path.rfind('/') {
        Some(0) | None => "/".into(),
        Some(i) => path[..i].into(),
    }
}

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub(crate) fn unix_time(year: i64, month: u32, day: u32, h: u32, m: u32, s: u32) -> i64 {
    let (year, month, day) = (year.clamp(0, 9999), month.clamp(1, 12), day.clamp(1, 31));
    let (h, m, s) = (i64::from(h.min(23)), i64::from(m.min(59)), i64::from(s.min(60)));
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (month as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    days * 86_400 + h * 3600 + m * 60 + s
}

pub(crate) fn civil(time: i64) -> (i64, u32, u32) {
    let z = time.div_euclid(86_400) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(month <= 2), month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn what_is_said_of_a_heard_address_is_kept_in_one_place() {
        let card = heard("nas.local", "10.0.0.7", Heard::Card(21));
        let stopped = heard("nas.local", "10.0.0.7", Heard::Stopped);
        assert_eq!(stopped, heard_words("nas.local", "10.0.0.7"));
        assert_eq!(Error::Heard { host: "nas.local".into(), at: "10.0.0.7".into(), port: 21 }.to_string(), stopped);
        for words in [&card, &stopped] {
            assert!(words.starts_with("nas.local answers at 10.0.0.7") && words.contains("only heard on the network") && words.contains("so the password has not been sent."), "{words}");
        }
    }

    #[test]
    fn dates() {
        assert_eq!(unix_time(1970, 1, 1, 0, 0, 0), 0);
        assert_eq!(unix_time(2000, 3, 1, 0, 0, 0), 951_868_800);
        assert_eq!(unix_time(2026, 10, 3, 12, 30, 5), 1_791_030_605);
        assert_eq!(civil(1_791_030_605), (2026, 10, 3));
        assert_eq!(civil(951_868_799), (2000, 2, 29));
    }

    #[test]
    fn a_date_a_server_made_up_cannot_overflow() {
        let top = unix_time(i64::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX);
        assert_eq!(top, unix_time(9999, 12, 31, 23, 59, 60));
        assert_eq!(unix_time(i64::MIN, 0, 0, 0, 0, 0), unix_time(0, 1, 1, 0, 0, 0));
    }

    #[test]
    fn paths() {
        assert_eq!(join("/", "a"), "/a");
        assert_eq!(join("/a", "b"), "/a/b");
        assert_eq!(parent("/a/b"), "/a");
        assert_eq!(parent("/a"), "/");
        assert_eq!(parent("/"), "/");
        assert_eq!(name_of("/a/b/"), "b");
    }
}
