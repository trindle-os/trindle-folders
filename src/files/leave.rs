// The one rule for logins that leave: what is kept and what is dropped when a login ends.

use crate::common::smb;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trust {
    Given,
    Heard,
    Shown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reached {
    at: String,
    trust: Trust,
}

impl Reached {
    pub fn now(name: &str, found: Option<String>) -> Option<Reached> {
        let (at, trust) = reach(name, found);
        at.map(|at| Reached { at, trust })
    }

    pub fn address(&self) -> &str {
        &self.at
    }

    pub fn trust(&self) -> Trust {
        self.trust
    }
}

pub trait Card {
    fn shows(&self) -> Option<String>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Confirmed(String);

impl Confirmed {
    pub fn pressed(card: &impl Card) -> Option<Confirmed> {
        card.shows().map(Confirmed)
    }

    pub fn address(&self) -> &str {
        &self.0
    }

    pub fn reached(&self) -> Reached {
        Reached { at: self.0.clone(), trust: Trust::Shown }
    }

    #[cfg(test)]
    pub fn made_up(address: &str) -> Confirmed {
        Confirmed(address.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Way {
    Nowhere,
    At(Reached),
    Proved,
}

pub trait Carries {
    fn secret(&self) -> bool;
}

impl Carries for Option<smb::Login> {
    fn secret(&self) -> bool {
        self.is_some()
    }
}

#[derive(Debug)]
pub struct Leave<L> {
    login: L,
    at: Option<String>,
}

impl<L> Leave<L> {
    pub fn login(&self) -> &L {
        &self.login
    }

    pub fn address(&self) -> Option<&str> {
        self.at.as_deref()
    }
}

pub fn leave<L: Carries>(login: L, to: &Way) -> Result<Leave<L>, L> {
    let at = match to {
        Way::At(reached) => Some(reached.at.clone()),
        Way::Nowhere | Way::Proved => None,
    };
    if !login.secret() {
        return Ok(Leave { login, at });
    }
    match to {
        Way::Proved => Ok(Leave { login, at }),
        Way::At(reached) if reached.trust != Trust::Heard => Ok(Leave { login, at }),
        Way::At(_) | Way::Nowhere => Err(login),
    }
}

pub fn real_name(server: &str) -> bool {
    let name = server.trim_end_matches('.').to_ascii_lowercase();
    name.contains('.') && !name.ends_with(".local")
}

pub fn reach(server: &str, found: Option<String>) -> (Option<String>, Trust) {
    let bare = server.strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or(server);
    if bare.parse::<std::net::IpAddr>().is_ok() {
        return (Some(bare.to_string()), Trust::Given);
    }
    if real_name(server) {
        if let Some(at) = resolve(server) {
            return (Some(at), Trust::Given);
        }
        return (found, Trust::Heard);
    }
    (found.or_else(|| resolve(server)), Trust::Heard)
}

#[cfg(not(test))]
fn resolve(server: &str) -> Option<String> {
    use std::net::ToSocketAddrs;
    let all: Vec<std::net::IpAddr> = (server, 0).to_socket_addrs().ok()?.map(|a| a.ip()).collect();
    let usable = |a: &&std::net::IpAddr| smb::valid_address(&a.to_string()).is_some();
    all.iter().find(|a| a.is_ipv4() && usable(a)).or_else(|| all.iter().find(usable)).map(|a| a.to_string())
}

#[cfg(test)]
fn resolve(server: &str) -> Option<String> {
    pretend::looked_up(server)
}

#[cfg(test)]
pub mod pretend {
    use std::collections::HashMap;

    thread_local! {
        static DNS: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
    }

    pub fn dns(name: &str, at: &str) {
        DNS.with_borrow_mut(|d| d.insert(name.to_lowercase(), at.to_string()));
    }

    pub fn dns_forgets(name: &str) {
        DNS.with_borrow_mut(|d| d.remove(&name.to_lowercase()));
    }

    pub fn looked_up(name: &str) -> Option<String> {
        DNS.with_borrow(|d| d.get(&name.to_lowercase()).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::source::code;

    fn way(name: &str, found: Option<&str>) -> Way {
        Reached::now(name, found.map(str::to_string)).map_or(Way::Nowhere, Way::At)
    }

    #[test]
    fn a_secret_never_leaves_for_an_address_only_heard() {
        let jo = || Some(smb::Login::typed("jo", "right"));
        let guest = || -> Option<smb::Login> { None };
        pretend::dns("files.example.com", "192.0.2.5");
        pretend::dns("nas.local", "10.0.0.2");
        pretend::dns("nas", "10.0.0.2");
        for (name, at) in [("10.0.0.9", "10.0.0.9"), ("fd00::20", "fd00::20"), ("[fd00::20]", "fd00::20"), ("files.example.com", "192.0.2.5")] {
            let gone = leave(jo(), &way(name, Some("10.0.0.66"))).expect(name);
            assert_eq!(gone.address(), Some(at), "what was found on the network is not used in place of what was given");
            assert_eq!(gone.login(), &jo());
        }
        for (name, found) in [("nas", Some("10.0.0.2")), ("nas", None), ("nas.local", Some("10.0.0.2")), ("nas.local", None), ("NAS.LOCAL.", Some("10.0.0.2")),
                              ("files.invalid", Some("10.0.0.2"))] {
            assert!(matches!(&way(name, found), Way::At(r) if r.trust() == Trust::Heard && r.address() == "10.0.0.2"), "{name}");
            assert_eq!(leave(jo(), &way(name, found)).map(|_| ()), Err(jo()), "{name}: a secret left for an address only heard");
            assert_eq!(leave(guest(), &way(name, found)).expect(name).address(), Some("10.0.0.2"));
        }
        assert_eq!(way("nowhere", None), Way::Nowhere);
        assert_eq!(leave(jo(), &Way::Nowhere).map(|_| ()), Err(jo()));
        assert_eq!(leave(guest(), &Way::Nowhere).expect("a guest").address(), None);
        struct Up(Option<&'static str>);
        impl Card for Up {
            fn shows(&self) -> Option<String> {
                self.0.map(str::to_string)
            }
        }
        assert_eq!(Confirmed::pressed(&Up(None)), None, "a card that shows no address confirms none");
        let pressed = Confirmed::pressed(&Up(Some("10.0.0.2"))).unwrap();
        assert_eq!(pressed.reached().trust(), Trust::Shown);
        assert_eq!(leave(jo(), &Way::At(pressed.reached())).expect("shown").address(), Some("10.0.0.2"));
        for name in ["10.0.0.9", "files.example.com", "nas", "nas.local"] {
            assert!(!matches!(way(name, Some("10.0.0.2")), Way::At(r) if r.trust() == Trust::Shown));
        }
        assert_eq!(leave(jo(), &Way::Proved).expect("proved").address(), None);
        for (name, real) in [("files.example.com", true), ("nas.lan.", true), ("nas", false), ("NAS.local", false), ("nas.local.", false), ("nas.", false)] {
            assert_eq!(real_name(name), real, "{name}");
        }
    }

    fn before_tests(text: &'static str) -> &'static str {
        text.find("mod tests {").map_or(text, |at| &text[..at])
    }

    fn parts_are_its_own(code: &str) -> bool {
        let parts = code.split("pub struct Leave<L>").nth(1).and_then(|rest| rest.split_once('{')).and_then(|(_, rest)| rest.split_once('}')).map(|(parts, _)| parts);
        let names: Option<Vec<&str>> = parts.map(|p| p.split(',').map(str::trim).filter(|part| !part.is_empty()).collect());
        names.is_some_and(|names| names.len() == 2 && names.iter().all(|part| !part.starts_with("pub") && part.contains(':')))
    }

    #[test]
    fn the_parts_of_a_leave_are_read_however_they_are_laid_out() {
        assert!(parts_are_its_own("pub struct Leave<L> {\n    login: L,\n    at: Option<String>,\n}"));
        assert!(parts_are_its_own("pub struct Leave<L> { login: L, at: Option<String> }"));
        assert!(parts_are_its_own("pub struct Leave<L>\n{\n\tlogin: L,\n\n\tat: Option<String>,\n}\n"));
        for open in ["pub login: L, at: Option<String>", "login: L, pub(crate) at: Option<String>", "login: L,\n    pub(super) at: Option<String>,", "login: L"] {
            assert!(!parts_are_its_own(&format!("pub struct Leave<L> {{ {open} }}")), "{open}");
        }
        assert!(!parts_are_its_own("pub struct Leave<L>(pub L, pub Option<String>);"));
        assert!(!parts_are_its_own("struct Other { a: u8 }"));
    }

    fn calls(code: &str, name: &str) -> usize {
        code.matches(&format!("{name}(")).count() - code.matches(&format!("fn {name}(")).count()
    }

    #[test]
    fn a_login_is_sent_only_by_way_of_the_one_rule() {
        let shares = before_tests(crate::common::source::code_all(crate::files::shares::SOURCE));
        let far = before_tests(crate::common::source::code_all(crate::files::far::SOURCE));
        let connect = before_tests(code(include_str!("remote/mod.rs")));
        let ftp = before_tests(code(include_str!("remote/ftp.rs")));
        let dav = before_tests(code(include_str!("remote/dav.rs")));
        let sftp = before_tests(code(include_str!("remote/sftp.rs")));
        let here = before_tests(code(include_str!("leave.rs")));
        assert!(parts_are_its_own(here), "the parts of a Leave can be reached from outside");
        let made = here.matches("Leave { login, at }").count();
        let rule = here.split("pub fn leave<").nth(1).and_then(|s| s.split("\n}\n").next()).unwrap_or_default();
        assert!(made > 0 && rule.matches("Leave { login, at }").count() == made, "a Leave is made outside the one rule");
        assert_eq!(here.matches("Trust::Shown").count(), 1, "an address is said to be shown somewhere other than from a card");
        for (file, text, sender, times) in [
            ("shares", shares, "fn list_on(server: &str, to: &Leave<Option<Login>>)", 2),
            ("shares", shares, "fn mount_on(server: &str, share: &str, to: &Leave<Option<Login>>)", 2),
            ("far", far, "fn dial(a: &Address, to: &Leave<Login>)", 2),
            ("remote/mod.rs", connect, "pub fn connect(address: &Address, to: &Leave<Login>)", 1),
            ("remote/ftp.rs", ftp, "pub fn connect(address: &Address, to: &Leave<Login>)", 1),
            ("remote/dav.rs", dav, "pub fn connect(address: &Address, to: &Leave<Login>)", 1),
        ] {
            assert_eq!(text.matches(sender).count(), times, "{file}: `{sender}` must take the Leave the one rule made, the real one and any pretend one alike");
        }
        for (file, text) in [("remote/ftp.rs", ftp), ("remote/dav.rs", dav), ("remote/sftp.rs", sftp), ("remote/mod.rs", connect)] {
            assert!(!text.contains("address.password") && !text.contains("a.password"), "{file} reads a password from an address");
        }
        assert_eq!(calls(shares, "list_env"), 1, "a share's login is put in a command's surroundings somewhere new");
        assert_eq!(shares.matches("Login::lines(").count(), 1, "a share's login is handed to the mounting helper somewhere new");
        assert!(shares.contains("list_env(to.login().as_ref())") && shares.contains("Login::lines(to.login().as_ref())"));
        assert_eq!(calls(shares, "kept_login"), 1, "the keyring's login is fetched somewhere other than in `settle`");
        assert_eq!(shares.matches("kept_login(server, share)").count(), 1);
        assert_eq!(calls(shares, "keyring::find"), 1, "the keyring is asked for a login somewhere new");
        assert_eq!(shares.matches(".shares.logins.get(").count(), 1, "a login held in memory is fetched somewhere other than in `held_for`");
        assert_eq!(calls(shares, "settle"), 1, "an opening is settled somewhere other than in `share_send`");
        assert_eq!(calls(shares, "list_on"), 1, "a computer's shares are listed somewhere new");
        assert_eq!(calls(shares, "mount_on") - shares.matches("unmount_on(").count(), 1, "a share is mounted somewhere new");
        assert_eq!(far.matches(".logins.get(").count(), 1, "a server's held login is fetched somewhere other than in `Links::dial`");
        assert_eq!(calls(far, "dial") - 1, 4, "a server is dialled somewhere other than by `Links::dial`, from `open`, `claim` and `look`");
        assert_eq!(far.matches("self.dial(").count(), 4);
        assert_eq!(calls(far, "cleared"), 1, "a server's login is cleared to leave somewhere other than in `Links::dial`");
        assert_eq!(far.matches("remote::connect(a, to)").count(), 2);
        assert_eq!(far.matches("remote::connect(").count(), 2);
        for (file, text) in [
            ("app.rs", code(crate::files::app::SOURCE)), ("act", crate::common::source::code_all(crate::files::act::SOURCE)), ("menu.rs", code(crate::files::menu::SOURCE)),
            ("work.rs", code(include_str!("work.rs"))), ("inplace.rs", code(include_str!("inplace.rs"))), ("preview.rs", code(include_str!("preview.rs"))),
            ("neighbours.rs", code(include_str!("neighbours.rs"))), ("nearby.rs", code(include_str!("nearby.rs"))), ("sharing.rs", code(include_str!("sharing.rs"))),
            ("kept.rs", code(include_str!("kept.rs"))), ("places.rs", code(include_str!("places.rs"))), ("computer.rs", code(include_str!("computer.rs"))),
            ("dialogs.rs", code(include_str!("dialogs.rs"))), ("askpass.rs", code(include_str!("askpass.rs"))), ("mod.rs", code(include_str!("mod.rs"))),
        ] {
            for call in ["remote::connect(", "Ftp::connect(", "Dav::connect(", "kept_login(", "keyring::find(", "list_on(", "mount_on(", "list_env(", "Login::lines(", "leave::leave(", "Confirmed::pressed("] {
                assert!(!before_tests(text).contains(call), "{file} has `{call}`: a login leaves only by shares.rs or far.rs, through leave::leave");
            }
        }
        assert_eq!(shares.matches("Confirmed::pressed(").count(), 1);
        assert_eq!(far.matches("Confirmed::pressed(").count(), 1);
    }
}
