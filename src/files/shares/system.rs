//! The programs that list a computer's shares and mount and unmount them, with the login the one rule let go.

#[cfg(not(test))]
use std::path::{Path, PathBuf};

#[cfg(not(test))]
use crate::common::smb::{self, Login, Problem};
#[cfg(not(test))]
use crate::files::leave::Leave;
#[cfg(not(test))]
use crate::toolkit::t;
#[cfg(not(test))]
use super::commands::{list_argv, list_env, mount_argv, nfs_mount_argv};

#[cfg(test)]
pub(super) use super::pretend::{list_on, mount_on, nfs_export_on, unmount_on};

#[cfg(not(test))]
pub(super) fn list_on(server: &str, to: &Leave<Option<Login>>) -> Result<Vec<String>, Problem> {
    use std::process::{Command, Stdio};
    let (ip, login) = (to.address(), to.login().as_ref());
    let argv = list_argv(server, ip, login.is_none());
    let mut cmd = Command::new(&argv[0]);
    cmd.args(&argv[1..]).env("LC_ALL", "C").stdin(Stdio::null());
    for k in ["USER", "LOGNAME", "PASSWD", "PASSWD_FD", "PASSWD_FILE"] {
        cmd.env_remove(k);
    }
    for (k, v) in list_env(to.login().as_ref()) {
        cmd.env(k, v.as_str());
    }
    let out = cmd.output().map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => Problem::Missing("smbclient"),
        _ => Problem::Other(e.to_string()),
    })?;
    let listed = String::from_utf8_lossy(&out.stdout);
    let found = smb::shares(&listed);
    if out.status.success() || !found.is_empty() {
        return Ok(found);
    }
    Err(smb::list_problem(&format!("{listed}\n{}", String::from_utf8_lossy(&out.stderr)), login.is_none()))
}

#[cfg(not(test))]
pub(super) fn nfs_export_on(server: &str, export: &str) -> Result<PathBuf, Problem> {
    if !crate::files::alone::helper_here() {
        return Err(Problem::Other(crate::files::alone::no_helper_nfs().into()));
    }
    let helper = crate::common::run::admin_helper();
    let (ok, out, err) = crate::common::run::captured(&nfs_mount_argv(&helper, server, export), None);
    if ok {
        return out.lines().last().map(PathBuf::from).ok_or_else(|| Problem::Other(t!("folders-shares-nfs-opened-unknown").into()));
    }
    let l = err.to_lowercase();
    if l.contains("not installed") {
        return Err(Problem::Missing("nfs-common"));
    }
    if l.contains("mount.nfs") {
        return Err(smb::nfs_problem(&err));
    }
    if l.contains("not authorized") || l.contains("dismissed") || err.trim().is_empty() {
        return Err(Problem::NotAllowed(crate::common::run::admin_error(&err)));
    }
    Err(Problem::Other(crate::common::run::admin_error(&err)))
}

#[cfg(not(test))]
pub(super) fn mount_on(server: &str, share: &str, to: &Leave<Option<Login>>) -> Result<PathBuf, Problem> {
    if !crate::files::alone::helper_here() {
        return Err(Problem::Other(crate::files::alone::no_helper_mount().into()));
    }
    let login = to.login().as_ref();
    let helper = crate::common::run::admin_helper();
    let argv = mount_argv(&helper, server, share, to.address());
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    let (ok, out, err) = crate::common::run::captured_secret(&argv, Login::lines(to.login().as_ref()));
    if ok {
        return out.lines().last().map(PathBuf::from).ok_or_else(|| Problem::Other(t!("folders-shares-opened-unknown").into()));
    }
    if err.contains("mount error") {
        return Err(smb::mount_problem(&err, login.is_none()));
    }
    let l = err.to_lowercase();
    if l.contains("not authorized") || l.contains("dismissed") || err.trim().is_empty() {
        return Err(Problem::NotAllowed(crate::common::run::admin_error(&err)));
    }
    if l.contains("not installed") {
        return Err(Problem::Missing("cifs-utils"));
    }
    Err(Problem::Other(crate::common::run::admin_error(&err)))
}

#[cfg(not(test))]
pub(super) fn unmount_on(at: &Path) -> Result<(), String> {
    let helper = crate::common::run::admin_helper();
    let argv = ["pkexec".to_string(), helper, "smb-unmount".into(), at.to_string_lossy().into_owned()];
    let (ok, _, err) = crate::common::run::captured(&argv, None);
    if ok { Ok(()) } else { Err(crate::common::run::admin_error(&err)) }
}
