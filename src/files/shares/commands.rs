//! The command lines that list and mount shares, which hold no login, and the words for an NFS server's trouble.

use zeroize::Zeroizing;

use crate::files::remote::Address;
#[cfg(not(test))]
use crate::common::smb;
use crate::common::smb::{Login, Mounted, Problem};
use crate::toolkit::t;

pub fn parts(a: &Address) -> (String, Option<String>, String) {
    let mut bits = a.path.trim_start_matches('/').splitn(2, '/');
    let share = bits.next().filter(|s| !s.is_empty()).map(str::to_string);
    let rest = bits.next().unwrap_or("").trim_end_matches('/').to_string();
    (a.host.clone(), share, rest)
}

pub fn list_argv(server: &str, ip: Option<&str>, guest: bool) -> Vec<String> {
    let mut v: Vec<String> = ["smbclient", "-g", "-t", "10", "-L"].iter().map(|s| s.to_string()).collect();
    v.push(format!("//{server}"));
    if let Some(ip) = ip {
        v.extend(["-I".to_string(), ip.to_string()]);
    }
    if guest {
        v.extend(["-N".to_string(), "-U%".to_string()]);
    }
    v
}

pub fn list_env(login: Option<&Login>) -> Vec<(&'static str, Zeroizing<String>)> {
    match login {
        Some(l) => vec![("USER", Zeroizing::new(l.shown_user())), ("LOGNAME", Zeroizing::new(l.user.clone())), ("PASSWD", l.password.clone())],
        None => Vec::new(),
    }
}

pub fn mount_argv(helper: &str, server: &str, share: &str, ip: Option<&str>) -> Vec<String> {
    let mut v = vec!["pkexec".to_string(), helper.to_string(), "smb-mount".into(), server.to_string(), share.to_string()];
    if let Some(ip) = ip {
        v.push(ip.to_string());
    }
    v
}

pub fn nfs_mount_argv(helper: &str, server: &str, export: &str) -> Vec<String> {
    vec!["pkexec".to_string(), helper.to_string(), "nfs-mount".into(), server.to_string(), export.to_string()]
}

pub fn nfs_words(problem: &Problem, server: &str, export: &str) -> String {
    match problem {
        Problem::Denied => t!("folders-shares-nfs-denied", server = server, export = export),
        Problem::NoShare => t!("folders-shares-nfs-no-share", server = server, export = export),
        other => other.words(server, Some(export)),
    }
}

#[cfg(not(test))]
fn me() -> String {
    let mut buf = vec![0u8; 16 * 1024];
    let name = unsafe {
        let mut entry: libc::passwd = std::mem::zeroed();
        let mut p: *mut libc::passwd = std::ptr::null_mut();
        let ok = libc::getpwuid_r(libc::getuid(), &mut entry, buf.as_mut_ptr().cast(), buf.len(), &mut p) == 0 && !p.is_null();
        ok.then(|| std::ffi::CStr::from_ptr((*p).pw_name).to_string_lossy().into_owned())
    };
    name.unwrap_or_else(crate::common::files::user)
}

#[cfg(not(test))]
pub(super) fn mounted_now() -> Option<Vec<Mounted>> {
    let mounts = std::fs::read_to_string("/proc/self/mounts").ok()?;
    Some(smb::mounted(&mounts, &smb::user_folder(&me())))
}

#[cfg(test)]
pub(super) fn mounted_now() -> Option<Vec<Mounted>> {
    None
}
