// Windows and Samba shares, mounted by the kernel's cifs so every program can open the files on them.

use crate::toolkit::t;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use zeroize::Zeroizing;

pub const MEDIA: &str = "/run/media";
pub const KINDS: &[&str] = &["cifs", "smb3"];
pub const NFS_KINDS: &[&str] = &["nfs", "nfs4"];
const MAX_EXPORT: usize = 255;
const MAX_SERVER: usize = 253;
const MAX_SHARE: usize = 80;
const MAX_LOGIN: usize = 256;
const MAX_SHOWN_SERVER: usize = 100;

pub fn valid_server(s: &str) -> bool {
    !s.is_empty() && s.len() <= MAX_SERVER && !s.starts_with(['-', '.'])
        && s.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
}

pub fn valid_share(s: &str) -> bool {
    !s.is_empty() && s.chars().count() <= MAX_SHARE && s != "." && s != ".." && !s.starts_with('-')
        && !s.chars().any(|c| matches!(c, '/' | '\\' | ',') || c.is_control())
}

pub fn valid_export(s: &str) -> bool {
    s.starts_with('/') && s.len() <= MAX_EXPORT && !s.chars().any(|c| matches!(c, ',' | ':' | '\\') || c.is_control())
        && (s == "/" || s[1..].split('/').all(|part| !part.is_empty() && part != "." && part != ".."))
}

pub fn valid_address(s: &str) -> Option<IpAddr> {
    let ip: IpAddr = s.parse().ok()?;
    match ip {
        IpAddr::V6(v6) if (v6.segments()[0] & 0xffc0) == 0xfe80 => None,
        IpAddr::V6(v6) if v6.is_unspecified() || v6.is_multicast() => None,
        IpAddr::V4(v4) if v4.is_unspecified() || v4.is_broadcast() || v4.is_multicast() => None,
        ip => Some(ip),
    }
}

pub fn user_folder(user: &str) -> PathBuf {
    Path::new(MEDIA).join(user)
}

pub fn folder_name(server: &str, share: &str) -> String {
    let server: String = server.chars().take(MAX_SHOWN_SERVER).collect();
    let name = format!("{share} on {server}");
    match name.strip_prefix('.') {
        Some(rest) => format!("_{rest}"),
        None => name,
    }
}

pub fn nfs_folder_name(server: &str, export: &str) -> String {
    let last = export.rsplit('/').find(|p| !p.is_empty()).unwrap_or("everything");
    folder_name(server, last)
}

#[cfg(test)]
pub fn in_user_folder(at: &Path, user: &str) -> bool {
    let Some(name) = at.file_name().and_then(|n| n.to_str()) else { return false };
    name != "." && name != ".." && !name.contains('/') && at.as_os_str() == user_folder(user).join(name).as_os_str()
}

#[cfg(test)]
pub fn options(uid: u32, gid: u32, ip: Option<IpAddr>, login: Option<&Path>) -> String {
    let mut o = vec![format!("uid={uid}"), format!("gid={gid}"), "forceuid".into(), "forcegid".into(), "file_mode=0600".into(),
                     "dir_mode=0700".into(), "nosuid".into(), "nodev".into(), "iocharset=utf8".into()];
    match login {
        Some(file) => o.push(format!("credentials={}", file.display())),
        None => o.extend(["guest".to_string(), "username=guest".to_string()]),
    }
    if let Some(ip) = ip {
        o.push(format!("ip={ip}"));
    }
    o.join(",")
}

#[cfg(test)]
pub fn mount_argv(server: &str, share: &str, at: &Path, options: &str) -> Vec<String> {
    vec!["mount.cifs".into(), format!("//{server}/{share}"), at.to_string_lossy().into_owned(), "-o".into(), options.into()]
}

#[cfg(test)]
pub fn nfs_mount_argv(server: &str, export: &str, at: &Path) -> Vec<String> {
    vec!["mount.nfs".into(), format!("{server}:{export}"), at.to_string_lossy().into_owned(), "-o".into(), NFS_OPTIONS.into()]
}

#[cfg(test)]
pub fn tidy_unit(uid: u32) -> String {
    format!("trindle-shares@{uid}.service")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mounted {
    pub server: String,
    pub share: String,
    pub at: PathBuf,
}

impl Mounted {
    pub fn name(&self) -> String {
        if self.is_nfs() {
            return nfs_folder_name(&self.server, &self.share);
        }
        format!("{} on {}", self.share, self.server)
    }

    pub fn is_nfs(&self) -> bool {
        self.share.starts_with('/')
    }
}

pub fn mounted(mounts: &str, folder: &Path) -> Vec<Mounted> {
    mounts.lines().filter_map(|line| {
        let mut f = line.split(' ');
        let (source, at, kind) = (f.next()?, f.next()?, f.next()?);
        let nfs = NFS_KINDS.contains(&kind);
        if !nfs && !KINDS.contains(&kind) {
            return None;
        }
        let at = PathBuf::from(unescape(at));
        if at.parent() != Some(folder) {
            return None;
        }
        let source = unescape(source);
        if nfs {
            let (server, export) = source.split_once(":/")?;
            if !valid_server(server) {
                return None;
            }
            return Some(Mounted { server: server.to_string(), share: format!("/{export}"), at });
        }
        let (server, share) = source.strip_prefix("//")?.split_once('/')?;
        Some(Mounted { server: server.to_string(), share: share.trim_end_matches('/').to_string(), at })
    }).collect()
}

fn unescape(field: &str) -> String {
    let b = field.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 3 < b.len() && b[i + 1..i + 4].iter().all(|c| (b'0'..=b'7').contains(c)) {
            out.push((b[i + 1] - b'0') * 64 + (b[i + 2] - b'0') * 8 + (b[i + 3] - b'0'));
            i += 4;
            continue;
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Login {
    pub user: String,
    pub domain: String,
    pub password: Zeroizing<String>,
}

impl std::fmt::Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Login").field("user", &self.user).field("domain", &self.domain).finish_non_exhaustive()
    }
}

impl Login {
    pub fn typed(user: &str, password: &str) -> Login {
        let user = user.trim();
        let (domain, user) = match user.split_once('\\') {
            Some((d, u)) => (d.to_string(), u.to_string()),
            None => (String::new(), user.to_string()),
        };
        Login { user, domain, password: Zeroizing::new(password.to_string()) }
    }

    pub fn shown_user(&self) -> String {
        if self.domain.is_empty() { self.user.clone() } else { format!("{}\\{}", self.domain, self.user) }
    }

    pub fn check(&self) -> Result<(), String> {
        let name_ok = |s: &str| s.len() <= MAX_LOGIN && !s.chars().any(|c| c.is_control() || matches!(c, '%' | '/' | '\\' | ','));
        if self.user.is_empty() || !name_ok(&self.user) {
            return Err("That is not a user name a share can take.".into());
        }
        if !name_ok(&self.domain) {
            return Err("That is not a domain a share can take.".into());
        }
        if self.password.len() > MAX_LOGIN || self.password.chars().any(|c| matches!(c, '\n' | '\r' | '\0')) {
            return Err("A share's password cannot hold a line break.".into());
        }
        Ok(())
    }

    pub fn lines(login: Option<&Login>) -> Zeroizing<String> {
        Zeroizing::new(match login {
            Some(l) => format!("{}\n{}\n{}\n", l.user, l.domain, l.password.as_str()),
            None => String::new(),
        })
    }

    #[cfg(test)]
    pub fn read(text: &str) -> Result<Option<Login>, String> {
        if text.trim().is_empty() {
            return Ok(None);
        }
        let mut lines = text.split('\n');
        let user = lines.next().unwrap_or_default().to_string();
        let domain = lines.next().unwrap_or_default().to_string();
        let password = Zeroizing::new(lines.next().unwrap_or_default().to_string());
        if lines.any(|l| !l.is_empty()) {
            return Err("A share's password cannot hold a line break.".into());
        }
        let l = Login { user, domain, password };
        l.check()?;
        Ok(Some(l))
    }

    #[cfg(test)]
    pub fn credentials(&self) -> Zeroizing<String> {
        let mut text = format!("username={}\npassword={}\n", self.user, self.password.as_str());
        if !self.domain.is_empty() {
            text.push_str(&format!("domain={}\n", self.domain));
        }
        Zeroizing::new(text)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    NeedsLogin,
    WrongLogin,
    Account,
    NoShare,
    Denied,
    Asleep,
    NoComputer,
    Smb1,
    Nfs4,
    Missing(&'static str),
    NotAllowed(String),
    Other(String),
}

impl Problem {
    pub fn words(&self, server: &str, share: Option<&str>) -> String {
        let it = share.map_or_else(|| server.to_string(), |s| format!("{s} on {server}"));
        match self {
            Problem::NeedsLogin => format!("{server} wants a user name and password for {it}."),
            Problem::WrongLogin => format!("The user name or password was not right for {it}."),
            Problem::Account => format!("Your account on {server} is locked or switched off, or its password has run out."),
            Problem::NoShare => format!("{server} has no shared folder called {}.", share.unwrap_or("that")),
            Problem::Denied => format!("That account may not open {it}."),
            Problem::Asleep => format!("{server} did not answer. It may be asleep or switched off: wake it and try again."),
            Problem::NoComputer => format!("No computer called {server} could be found on this network."),
            Problem::Smb1 => format!("{server} only speaks SMB1, an old way of sharing that is no longer safe, so Trindle does not use it."),
            Problem::Nfs4 => t!("folders-shares-nfs-version", server = server),
            Problem::Missing(package) => format!("This needs Debian's {package}, which is not installed."),
            Problem::NotAllowed(w) | Problem::Other(w) => w.clone(),
        }
    }

    pub fn ask(&self) -> bool {
        matches!(self, Problem::NeedsLogin | Problem::WrongLogin | Problem::Denied)
    }
}

pub fn mount_problem(said: &str, guest: bool) -> Problem {
    let code = said.split("mount error(").nth(1).and_then(|r| r.split(')').next()).and_then(|c| c.parse::<i32>().ok());
    let refused = if guest { Problem::NeedsLogin } else { Problem::WrongLogin };
    match code {
        Some(13 | 1 | 126) => refused,
        Some(127 | 128) => Problem::Account,
        Some(2 | 6) => Problem::NoShare,
        Some(101 | 110 | 111 | 112 | 113 | 115) => Problem::Asleep,
        Some(95) => Problem::Smb1,
        _ if said.contains("could not resolve address") || said.contains("resolve") => Problem::NoComputer,
        _ => Problem::Other(last_line(said, "The share could not be opened.")),
    }
}

pub fn nfs_problem(said: &str) -> Problem {
    let has = |words: &str| said.to_lowercase().contains(words);
    if has("access denied") || has("permission denied") || has("operation not permitted") {
        Problem::Denied
    } else if has("protocol not supported") || has("requested nfs version") {
        Problem::Nfs4
    } else if has("no such file or directory") {
        Problem::NoShare
    } else if has("timed out") || has("no route to host") || has("connection refused") || has("network is unreachable") || has("host is down") {
        Problem::Asleep
    } else if has("failed to resolve") || has("can't get address") || has("name or service not known") || has("unknown host") {
        Problem::NoComputer
    } else {
        Problem::Other(last_line(said, "The shared folder could not be opened."))
    }
}

pub fn list_problem(said: &str, guest: bool) -> Problem {
    let has = |code: &str| said.contains(code);
    let refused = if guest { Problem::NeedsLogin } else { Problem::WrongLogin };
    if ["NT_STATUS_HOST_UNREACHABLE", "NT_STATUS_IO_TIMEOUT", "NT_STATUS_NETWORK_UNREACHABLE", "NT_STATUS_CONNECTION_REFUSED"]
        .iter().any(|c| has(c)) {
        Problem::Asleep
    } else if has("NT_STATUS_LOGON_FAILURE") || has("NT_STATUS_WRONG_PASSWORD") || has("NT_STATUS_NO_SUCH_USER") {
        refused
    } else if has("NT_STATUS_ACCESS_DENIED") {
        if guest { Problem::NeedsLogin } else { Problem::Denied }
    } else if ["NT_STATUS_ACCOUNT_LOCKED_OUT", "NT_STATUS_ACCOUNT_DISABLED", "NT_STATUS_PASSWORD_EXPIRED", "NT_STATUS_PASSWORD_MUST_CHANGE",
               "NT_STATUS_ACCOUNT_EXPIRED"].iter().any(|c| has(c)) {
        Problem::Account
    } else if has("NT_STATUS_BAD_NETWORK_NAME") {
        Problem::NoShare
    } else if has("NT_STATUS_RESOURCE_NAME_NOT_FOUND") || has("NT_STATUS_BAD_NETWORK_PATH") || has("Unable to resolve") {
        Problem::NoComputer
    } else if has("protocol negotiation failed") || has("NT_STATUS_CONNECTION_RESET") {
        Problem::Smb1
    } else {
        Problem::Other(last_line(said, "Its shared folders could not be listed."))
    }
}

fn last_line(said: &str, otherwise: &str) -> String {
    said.lines().map(str::trim).rfind(|l| !l.is_empty()).map_or_else(|| otherwise.to_string(), str::to_string)
}

pub fn shares(listed: &str) -> Vec<String> {
    let mut out: Vec<String> = listed.lines().filter_map(|l| {
        let mut f = l.split('|');
        (f.next()? == "Disk").then_some(())?;
        let name = f.next()?.trim();
        (!name.is_empty() && !name.ends_with('$') && valid_share(name)).then(|| name.to_string())
    }).collect();
    out.sort_by_key(|s| s.to_lowercase());
    out.dedup();
    out
}

#[cfg(test)]
pub const NFS_OPTIONS: &str = "vers=4,nosuid,nodev,retry=0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_that_are_let_through() {
        for ok in ["NAS", "desk-pc", "nas.local", "192.168.1.20", "files_1"] {
            assert!(valid_server(ok), "{ok}");
        }
        for bad in ["", "-oopt", ".hidden", "a b", "a/b", "fe80::1", "x,y", "a\nb", &"a".repeat(254)] {
            assert!(!valid_server(bad), "{bad}");
        }
        for ok in ["Photos", "My Music", "C$", "Ünïcødé"] {
            assert!(valid_share(ok), "{ok}");
        }
        for bad in ["", ".", "..", "a/b", "a\\b", "a\nb", "-o", "x,uid=0", &"a".repeat(81)] {
            assert!(!valid_share(bad), "{bad}");
        }
        assert_eq!(valid_address("192.168.1.20"), Some("192.168.1.20".parse().unwrap()));
        assert_eq!(valid_address("fd00::2"), Some("fd00::2".parse().unwrap()));
        assert_eq!(valid_address("fe80::2"), None);
        assert_eq!(valid_address("0.0.0.0"), None);
        assert_eq!(valid_address("nas"), None);
    }

    #[test]
    fn folders_belong_to_their_person() {
        assert_eq!(folder_name("NAS", "Photos"), "Photos on NAS");
        assert_eq!(folder_name("NAS", ".dots"), "_dots on NAS");
        assert!(folder_name(&"n".repeat(253), &"s".repeat(80)).len() < 255);
        let jo = user_folder("jo");
        assert_eq!(jo, Path::new("/run/media/jo"));
        assert!(in_user_folder(&jo.join("Photos on NAS"), "jo"));
        assert!(!in_user_folder(&jo, "jo"));
        assert!(!in_user_folder(Path::new("/run/media/jo/a/b"), "jo"));
        assert!(!in_user_folder(Path::new("/run/media/jo/../ann/x"), "jo"));
        assert!(!in_user_folder(Path::new("/run/media/jo//x"), "jo"));
        assert!(!in_user_folder(Path::new("/run/media/jo/./x"), "jo"));
        assert!(!in_user_folder(Path::new("/run/media/ann/x"), "jo"));
        assert!(!in_user_folder(Path::new("/etc"), "jo"));
        assert!(!in_user_folder(Path::new("run/media/jo/x"), "jo"));
    }

    #[test]
    fn the_mount_command_holds_no_secrets() {
        let login = Login::typed("WORKGROUP\\jo", "s3cret, pass");
        assert_eq!((login.user.as_str(), login.domain.as_str()), ("jo", "WORKGROUP"));
        let file = Path::new("/run/trindle-admin/cifs-1");
        let o = options(1000, 1000, Some("192.168.1.20".parse().unwrap()), Some(file));
        assert_eq!(o, "uid=1000,gid=1000,forceuid,forcegid,file_mode=0600,dir_mode=0700,nosuid,nodev,iocharset=utf8,\
                       credentials=/run/trindle-admin/cifs-1,ip=192.168.1.20");
        let argv = mount_argv("NAS", "My Photos", Path::new("/run/media/jo/My Photos on NAS"), &o);
        assert_eq!(argv[..4], ["mount.cifs", "//NAS/My Photos", "/run/media/jo/My Photos on NAS", "-o"]);
        for a in &argv {
            assert!(!a.contains("s3cret") && !a.contains("jo\\") && !a.contains("username=jo") && !a.contains("password"), "{a}");
        }
        let guest = options(1000, 100, None, None);
        assert!(guest.ends_with(",guest,username=guest") && !guest.contains("credentials"));
        let creds = login.credentials();
        assert_eq!(creds.as_str(), "username=jo\npassword=s3cret, pass\ndomain=WORKGROUP\n");
        assert_eq!(tidy_unit(1000), "trindle-shares@1000.service");
    }

    #[test]
    fn a_login_travels_on_standard_input() {
        let login = Login::typed("jo", "pa ss");
        let lines = Login::lines(Some(&login));
        assert_eq!(lines.as_str(), "jo\n\npa ss\n");
        assert_eq!(Login::read(&lines).unwrap(), Some(login.clone()));
        assert_eq!(Login::read("").unwrap(), None);
        assert_eq!(Login::lines(None).as_str(), "");
        assert!(Login::read("jo\n\npass\nmore\n").is_err());
        assert!(Login::read("j%o\n\npass\n").is_err(), "a % would split the name");
        assert!(Login::read("\n\npass\n").is_err());
        assert!(Login::read("jo\nA,B\npass\n").is_err());
        assert_eq!(Login::typed("DOM\\jo", "x").shown_user(), "DOM\\jo");
        assert!(!format!("{login:?}").contains("pa ss"), "never printed");
    }

    #[test]
    fn mounted_shares_are_read() {
        let mounts = "/dev/sda1 / ext4 rw 0 0\n\
                      //NAS/My\\040Photos /run/media/jo/My\\040Photos\\040on\\040NAS cifs rw,relatime 0 0\n\
                      //desk/Work /run/media/jo/Work\\040on\\040desk smb3 rw 0 0\n\
                      //NAS/Other /run/media/ann/Other\\040on\\040NAS cifs rw 0 0\n\
                      //NAS/Deep /run/media/jo/a/b cifs rw 0 0\n\
                      server:/x /run/media/jo/x nfs4 rw 0 0\n";
        let got = mounted(mounts, &user_folder("jo"));
        assert_eq!(got, [
            Mounted { server: "NAS".into(), share: "My Photos".into(), at: PathBuf::from("/run/media/jo/My Photos on NAS") },
            Mounted { server: "desk".into(), share: "Work".into(), at: PathBuf::from("/run/media/jo/Work on desk") },
            Mounted { server: "server".into(), share: "/x".into(), at: PathBuf::from("/run/media/jo/x") },
        ]);
        assert_eq!(got[0].name(), "My Photos on NAS");
    }

    #[test]
    fn complaints_are_put_in_plain_words() {
        assert_eq!(mount_problem("mount error(13): Permission denied\nRefer to the mount.cifs(8) manual page", false), Problem::WrongLogin);
        assert_eq!(mount_problem("mount error(13): Permission denied", true), Problem::NeedsLogin);
        assert_eq!(mount_problem("mount error(2): No such file or directory", false), Problem::NoShare);
        assert_eq!(mount_problem("mount error(112): Host is down", false), Problem::Asleep);
        assert_eq!(mount_problem("mount error(113): could not connect to 10.0.0.2No route to host", false), Problem::Asleep);
        assert_eq!(mount_problem("mount error: could not resolve address for NAS: Unknown error", false), Problem::NoComputer);
        assert_eq!(mount_problem("mount error(95): Operation not supported", false), Problem::Smb1);
        assert!(matches!(mount_problem("mount error(16): Device or resource busy", false), Problem::Other(w) if w.contains("busy")));
        assert_eq!(list_problem("session setup failed: NT_STATUS_LOGON_FAILURE", false), Problem::WrongLogin);
        assert_eq!(list_problem("session setup failed: NT_STATUS_ACCESS_DENIED", true), Problem::NeedsLogin);
        assert_eq!(list_problem("do_connect: Connection to NAS failed (Error NT_STATUS_IO_TIMEOUT)", true), Problem::Asleep);
        assert_eq!(list_problem("protocol negotiation failed: NT_STATUS_IO_TIMEOUT", true), Problem::Asleep);
        assert_eq!(list_problem("do_connect: Connection to NAS failed (Error NT_STATUS_RESOURCE_NAME_NOT_FOUND)", true), Problem::NoComputer);
        assert_eq!(list_problem("protocol negotiation failed: NT_STATUS_CONNECTION_RESET", true), Problem::Smb1);
        assert_eq!(list_problem("session setup failed: NT_STATUS_ACCOUNT_LOCKED_OUT", false), Problem::Account);
        assert!(Problem::WrongLogin.ask() && Problem::NeedsLogin.ask() && !Problem::Asleep.ask());
        assert_eq!(Problem::NoShare.words("NAS", Some("Photos")), "NAS has no shared folder called Photos.");
        assert!(Problem::Asleep.words("NAS", None).contains("asleep"));
        assert_eq!(Problem::Missing("smbclient").words("NAS", None), "This needs Debian's smbclient, which is not installed.");
        assert_eq!(Problem::NotAllowed("Not done.".into()).words("NAS", None), "Not done.");
    }

    #[test]
    fn a_computers_shares_are_listed() {
        let listed = "Disk|Photos|Family photos\nIPC|IPC$|IPC Service (Samba)\nDisk|print$|Printer Drivers\nDisk|C$|Default share\n\
                      Printer|HP|Office printer\nDisk|music|\nServer|NAS|\nWorkgroup|WORKGROUP|NAS\nSMB1 disabled -- no workgroup available\n";
        assert_eq!(shares(listed), ["music", "Photos"]);
        assert!(shares("").is_empty());
    }

    #[test]
    fn an_nfs_server_without_version_4_is_said_so() {
        for said in ["mount.nfs: Protocol not supported", "mount.nfs: requested NFS version or transport protocol is not supported"] {
            assert_eq!(nfs_problem(said), Problem::Nfs4, "{said}");
        }
        assert!(Problem::Nfs4.words("nas", Some("/data")).contains("version 4"));
        assert!(!Problem::Nfs4.ask());
    }

    #[test]
    fn nfs_names_and_exports_that_are_let_through() {
        for ok in ["/", "/srv/media", "/export/My Photos", "/a/b/c", "/home/jo"] {
            assert!(valid_export(ok), "{ok}");
        }
        for bad in ["", "srv/media", "/srv//media", "/srv/media/", "/srv/../etc", "/./x", "/x,vers=3", "/x:/y", "/x\\y", "/a\nb", &format!("/{}", "a".repeat(255))] {
            assert!(!valid_export(bad), "{bad:?}");
        }
    }

    #[test]
    fn nfs_is_mounted_with_nothing_but_its_own_options() {
        let at = Path::new("/run/media/jo/media on nas");
        let argv = nfs_mount_argv("nas", "/srv/media", at);
        assert_eq!(argv, ["mount.nfs", "nas:/srv/media", "/run/media/jo/media on nas", "-o", "vers=4,nosuid,nodev,retry=0"]);
        assert_eq!(nfs_folder_name("nas", "/srv/media"), "media on nas");
        assert_eq!(nfs_folder_name("nas", "/"), "everything on nas");
        assert_eq!(nfs_folder_name("nas", "/srv/.hidden"), "_hidden on nas");
    }

    #[test]
    fn nfs_exports_are_found_among_the_mounts_beside_shares() {
        let mounts = "//NAS/Photos /run/media/jo/Photos\\040on\\040NAS cifs rw 0 0\n\
                      nas:/srv/media /run/media/jo/media\\040on\\040nas nfs4 rw 0 0\n\
                      nas:/srv/old /run/media/ann/old\\040on\\040nas nfs rw 0 0\n\
                      [fe80::1]:/x /run/media/jo/x nfs4 rw 0 0\n\
                      nas:/srv/deep /run/media/jo/a/b nfs4 rw 0 0\n\
                      /dev/sda1 / ext4 rw 0 0\n";
        let got = mounted(mounts, Path::new("/run/media/jo"));
        assert_eq!(got.len(), 2, "{got:?}");
        assert!(!got[0].is_nfs() && got[0].name() == "Photos on NAS");
        assert!(got[1].is_nfs());
        assert_eq!((got[1].server.as_str(), got[1].share.as_str(), got[1].name().as_str()), ("nas", "/srv/media", "media on nas"));
    }

    #[test]
    fn what_mount_nfs_says_is_put_in_plain_words() {
        let said = |t: &str| nfs_problem(t);
        assert_eq!(said("mount.nfs: mounting nas:/x failed, reason given by server: No such file or directory"), Problem::NoShare);
        assert_eq!(said("mount.nfs: access denied by server while mounting nas:/x"), Problem::Denied);
        assert_eq!(said("mount.nfs: Connection timed out"), Problem::Asleep);
        assert_eq!(said("mount.nfs: Failed to resolve server nope: Name or service not known"), Problem::NoComputer);
        assert!(matches!(said("mount.nfs: something odd\nthe last line"), Problem::Other(w) if w == "the last line"));
    }
}
