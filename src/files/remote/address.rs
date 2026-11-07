// Addresses of remote places, such as sftp, ftp, ftps, ftpes and mtp.

use std::fmt;

use super::Error;
use crate::toolkit::t;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scheme {
    Sftp,
    Ftp,
    Ftps,
    Ftpes,
    Mtp,
    Dav,
    Davs,
    Smb,
    Nfs,
}

impl Scheme {
    pub fn name(self) -> &'static str {
        match self {
            Scheme::Sftp => "sftp",
            Scheme::Ftp => "ftp",
            Scheme::Ftps => "ftps",
            Scheme::Ftpes => "ftpes",
            Scheme::Mtp => "mtp",
            Scheme::Dav => "dav",
            Scheme::Davs => "davs",
            Scheme::Smb => "smb",
            Scheme::Nfs => "nfs",
        }
    }

    pub fn port(self) -> u16 {
        match self {
            Scheme::Sftp => 22,
            Scheme::Ftp | Scheme::Ftpes => 21,
            Scheme::Ftps => 990,
            Scheme::Mtp => 0,
            Scheme::Dav => 80,
            Scheme::Davs => 443,
            Scheme::Smb => 445,
            Scheme::Nfs => 2049,
        }
    }

    pub fn is_ftp(self) -> bool {
        matches!(self, Scheme::Ftp | Scheme::Ftps | Scheme::Ftpes)
    }

    pub fn takes_password(self) -> bool {
        self.is_ftp() || matches!(self, Scheme::Dav | Scheme::Davs | Scheme::Smb)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address {
    pub scheme: Scheme,
    pub user: Option<String>,
    pub password: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
}

impl Address {
    pub fn parse(text: &str) -> Result<Address, Error> {
        let bad = |why: &str| Error::BadAddress(why.into());
        let text = text.trim();
        let (scheme, rest) = text.split_once("://").ok_or_else(|| bad(t!("folders-remote-address-no-scheme")))?;
        let scheme = match scheme.to_ascii_lowercase().as_str() {
            "sftp" | "ssh" => Scheme::Sftp,
            "ftp" => Scheme::Ftp,
            "mtp" => Scheme::Mtp,
            "ftps" => Scheme::Ftps,
            "ftpes" => Scheme::Ftpes,
            "dav" | "webdav" => Scheme::Dav,
            "davs" | "webdavs" => Scheme::Davs,
            "smb" => Scheme::Smb,
            "nfs" => Scheme::Nfs,
            _ => return Err(bad(t!("folders-remote-address-bad-scheme"))),
        };
        let (authority, path) = match rest.find('/') {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, ""),
        };
        let (login, hostport) = match authority.rfind('@') {
            Some(i) => (Some(&authority[..i]), &authority[i + 1..]),
            None => (None, authority),
        };
        let (user, password) = match login {
            Some(l) => match l.split_once(':') {
                Some((u, p)) => (Some(unpercent(u)), Some(unpercent(p))),
                None => (Some(unpercent(l)), None),
            },
            None => (None, None),
        };
        let (host, port) = if scheme == Scheme::Mtp {
            (unpercent(hostport), None)
        } else if let Some(inner) = hostport.strip_prefix('[') {
            let (h, after) = inner.split_once(']').ok_or_else(|| bad(t!("folders-remote-address-no-bracket")))?;
            (h.to_string(), port_of(after)?)
        } else {
            match hostport.rsplit_once(':') {
                Some((h, p)) => (h.to_string(), port_of(&format!(":{p}"))?),
                None => (hostport.to_string(), None),
            }
        };
        if host.is_empty() {
            return Err(bad(t!("folders-remote-address-no-server")));
        }
        if host.starts_with('-') || host.chars().any(|c| c.is_whitespace() || c.is_control()) {
            return Err(bad(t!("folders-remote-address-bad-name")));
        }
        let path = unpercent(path);
        if path.contains(['\r', '\n', '\0']) {
            return Err(bad(t!("folders-remote-address-line-break")));
        }
        let user = user.filter(|u| !u.is_empty());
        Ok(Address { scheme, user, password, host, port, path })
    }

    pub fn join(&self, name: &str) -> Address {
        let base = if self.path.is_empty() { "/" } else { &self.path };
        Address { path: super::join(base, name), ..self.clone() }
    }

    pub fn parent(&self) -> Address {
        Address { path: super::parent(&self.path), ..self.clone() }
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(self.scheme.port())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}://", self.scheme.name())?;
        if let Some(user) = &self.user {
            write!(f, "{}@", percent(user, false))?;
        }
        if self.host.contains(':') {
            write!(f, "[{}]", self.host)?;
        } else if self.scheme == Scheme::Mtp {
            f.write_str(&percent(&self.host, false))?;
        } else {
            f.write_str(&self.host)?;
        }
        if let Some(port) = self.port {
            write!(f, ":{port}")?;
        }
        f.write_str(&percent(&self.path, true))
    }
}

fn port_of(after: &str) -> Result<Option<u16>, Error> {
    match after.strip_prefix(':') {
        None if after.is_empty() => Ok(None),
        None => Err(Error::BadAddress(t!("folders-remote-address-after-name").into())),
        Some("") => Ok(None),
        Some(p) => p.parse().map(Some).map_err(|_| Error::BadAddress(t!("folders-remote-address-not-port", port = p))),
    }
}

fn unpercent(text: &str) -> String {
    let (mut out, b) = (Vec::new(), text.as_bytes());
    let mut i = 0;
    while i < b.len() {
        let hex = |c: u8| (c as char).to_digit(16);
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(h), Some(l)) = (hex(b[i + 1]), hex(b[i + 2])) {
                out.push((h * 16 + l) as u8);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn percent(text: &str, slash: bool) -> String {
    let mut out = String::new();
    for &c in text.as_bytes() {
        if c.is_ascii_alphanumeric() || b"-._~".contains(&c) || (slash && c == b'/') {
            out.push(c as char);
        } else {
            out.push_str(&format!("%{c:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sftp() {
        let a = Address::parse("sftp://john@files.example.com:2222/home/john/My%20Files").unwrap();
        assert_eq!(a.scheme, Scheme::Sftp);
        assert_eq!(a.user.as_deref(), Some("john"));
        assert_eq!(a.host, "files.example.com");
        assert_eq!(a.port, Some(2222));
        assert_eq!(a.path, "/home/john/My Files");
        assert_eq!(a.to_string(), "sftp://john@files.example.com:2222/home/john/My%20Files");
        assert_eq!(a.parent().path, "/home/john");
        assert_eq!(a.join("x").path, "/home/john/My Files/x");
    }

    #[test]
    fn ftp_and_ipv6() {
        let a = Address::parse("ftp://[2001:db8::1]:21").unwrap();
        assert_eq!(a.host, "2001:db8::1");
        assert_eq!(a.port(), 21);
        assert_eq!(a.path, "");
        assert_eq!(a.to_string(), "ftp://[2001:db8::1]:21");
        let b = Address::parse("ftp://anna:p%40ss@ftp.example.com/pub").unwrap();
        assert_eq!(b.password.as_deref(), Some("p@ss"));
        assert_eq!(b.to_string(), "ftp://anna@ftp.example.com/pub");
        assert_eq!(b.port(), 21);
    }

    #[test]
    fn secure_ftp() {
        let a = Address::parse("ftps://jo@files.example.com/in").unwrap();
        assert_eq!((a.scheme, a.port()), (Scheme::Ftps, 990));
        assert_eq!(a.to_string(), "ftps://jo@files.example.com/in");
        let b = Address::parse("FTPES://files.example.com:2121").unwrap();
        assert_eq!((b.scheme, b.port()), (Scheme::Ftpes, 2121));
        assert!(a.scheme.is_ftp() && b.scheme.is_ftp() && !Scheme::Sftp.is_ftp());
    }

    #[test]
    fn mtp() {
        let a = Address::parse("mtp://ABC123/Internal%20shared%20storage/DCIM").unwrap();
        assert_eq!(a.scheme, Scheme::Mtp);
        assert_eq!(a.host, "ABC123");
        assert_eq!(a.path, "/Internal shared storage/DCIM");
    }

    #[test]
    fn webdav() {
        let a = Address::parse("davs://jo@cloud.example.com/remote.php/dav/files/jo/").unwrap();
        assert_eq!((a.scheme, a.port(), a.path.as_str()), (Scheme::Davs, 443, "/remote.php/dav/files/jo/"));
        assert!(a.scheme.takes_password() && !Scheme::Sftp.takes_password());
        let b = Address::parse("webdav://nas.local:8080/share").unwrap();
        assert_eq!((b.scheme, b.port()), (Scheme::Dav, 8080));
        assert_eq!(b.to_string(), "dav://nas.local:8080/share");
    }

    #[test]
    fn windows_shares() {
        let a = Address::parse("smb://NAS/My%20Photos/2026").unwrap();
        assert_eq!((a.scheme, a.host.as_str(), a.path.as_str()), (Scheme::Smb, "NAS", "/My Photos/2026"));
        assert_eq!(a.to_string(), "smb://NAS/My%20Photos/2026");
        assert!(a.scheme.takes_password());
        let b = Address::parse("SMB://WORKGROUP;jo@desk-pc").unwrap();
        assert_eq!((b.user.as_deref(), b.host.as_str(), b.path.as_str()), (Some("WORKGROUP;jo"), "desk-pc", ""));
    }

    #[test]
    fn refused() {
        assert!(matches!(Address::parse("http://example.com"), Err(Error::BadAddress(_))));
        assert!(matches!(Address::parse("sftp://-oProxyCommand=x/"), Err(Error::BadAddress(_))));
        assert!(matches!(Address::parse("sftp://host:99999/"), Err(Error::BadAddress(_))));
        assert!(matches!(Address::parse("ftp://host/a%0Db"), Err(Error::BadAddress(_))));
        assert!(matches!(Address::parse("example.com"), Err(Error::BadAddress(_))));
    }
}
