//! The computers on the home network that share folders, under Network: Windows computers by WS-Discovery (asked of trindle-wsdd) and others by multicast DNS (nearby.rs), merged into one list.

use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::app::{Files, Msg};
use super::places::{Entry, Where};
use crate::toolkit::t;

const SCHEME: &str = "smb://";
const FIRST_WAIT: Duration = Duration::from_secs(2);
const SECOND_WAIT: Duration = Duration::from_secs(4);
const READ_WAIT: Duration = Duration::from_secs(3);
const REPLY_MOST: u64 = 256 * 1024;
const REPLY_WAIT: Duration = Duration::from_secs(6);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Computer {
    pub name: String,
    pub address: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Way {
    #[default]
    Not,
    Asking,
    Asked,
    Could(String),
}

#[derive(Clone, Debug)]
pub enum Looked {
    Finder(bool),
    Announced(super::nearby::Asked),
}

#[derive(Default)]
pub struct Near {
    wsd: Vec<Computer>,
    mdns: Vec<Computer>,
    pub finder: Way,
    pub announced: Way,
}

impl Near {
    pub fn looking(&self) -> bool {
        self.finder == Way::Asking || self.announced == Way::Asking
    }

    fn all(&self) -> Vec<Computer> {
        let short = |name: &str| name.strip_suffix(".local").unwrap_or(name).to_lowercase();
        let mut out: Vec<Computer> = Vec::new();
        for c in self.wsd.iter().chain(&self.mdns) {
            match out.iter_mut().find(|o| short(&o.name) == short(&c.name)) {
                Some(o) if o.address.is_none() && c.address.is_some() => *o = c.clone(),
                Some(_) => {}
                None => out.push(c.clone()),
            }
        }
        out.sort_by_key(|c| c.name.to_lowercase());
        out
    }

    pub fn words(&self) -> String {
        if self.looking() {
            return t!("folders-neighbours-looking").to_string();
        }
        let mut out = vec![t!("folders-neighbours-none").to_string()];
        match (&self.finder, &self.announced) {
            (Way::Asked, Way::Asked) => out.push(t!("folders-neighbours-asked", ways = "both")),
            (Way::Asked, _) => out.push(t!("folders-neighbours-asked", ways = "windows")),
            (_, Way::Asked) => out.push(t!("folders-neighbours-asked", ways = "other")),
            _ => {}
        }
        for way in [&self.finder, &self.announced] {
            if let Way::Could(why) = way {
                out.push(why.clone());
            }
        }
        out.push(t!("folders-neighbours-quiet").to_string());
        out.join(" ")
    }
}

pub fn is_computer(path: &Path) -> bool {
    let b = path.as_os_str().as_encoded_bytes();
    b.strip_prefix(SCHEME.as_bytes()).is_some_and(|rest| !rest.is_empty() && !rest.strip_suffix(b"/").unwrap_or(rest).contains(&b'/'))
}

fn socket() -> Option<PathBuf> {
    Some(PathBuf::from(std::env::var_os("XDG_RUNTIME_DIR")?).join("gvfsd/wsdd"))
}

pub fn parse(reply: &str) -> Vec<Computer> {
    let mut out: Vec<Computer> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for line in reply.lines().take_while(|l| l.trim() != ".") {
        if out.len() == super::nearby::COMPUTERS_MOST {
            break;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        let Some(name) = fields.get(1).and_then(|n| super::nearby::plain_host(n.trim())) else { continue };
        let all: Vec<&str> = fields.get(4).map(|a| a.split('\'').skip(1).step_by(2).collect()).unwrap_or_default();
        let usable = |a: &&str| crate::common::smb::valid_address(a).is_some();
        let address = all.iter().find(|a| usable(a) && a.parse::<std::net::Ipv4Addr>().is_ok()).or_else(|| all.iter().find(|a| usable(a)))
            .map(|a| a.to_string());
        if seen.insert(name.to_lowercase()) {
            out.push(Computer { name: name.to_string(), address });
        }
    }
    out.sort_by_key(|c| c.name.to_lowercase());
    out
}

fn talk(socket: &Path, line: &str) -> io::Result<String> {
    let mut conn = UnixStream::connect(socket)?;
    conn.set_read_timeout(Some(READ_WAIT))?;
    conn.write_all(format!("{line}\n").as_bytes())?;
    if line != "list" {
        return Ok(String::new());
    }
    read_reply(conn, REPLY_WAIT)
}

#[derive(Default)]
struct Ending {
    line: usize,
    seen: usize,
    #[cfg(test)]
    looked: usize,
}

impl Ending {
    fn reached(&mut self, reply: &[u8]) -> bool {
        while let Some(at) = reply[self.seen..].iter().position(|b| *b == b'\n') {
            let end = self.seen + at;
            let line = &reply[self.line..end];
            #[cfg(test)]
            {
                self.looked += at + 1 + line.len();
            }
            (self.line, self.seen) = (end + 1, end + 1);
            if line.trim_ascii() == b"." {
                return true;
            }
        }
        #[cfg(test)]
        {
            self.looked += reply.len() - self.seen;
        }
        self.seen = reply.len();
        false
    }
}

fn read_reply(mut conn: UnixStream, wait: Duration) -> io::Result<String> {
    let late = || io::Error::new(io::ErrorKind::TimedOut, "The finder took too long to answer.");
    let (mut reply, mut buf, end) = (Vec::new(), [0u8; 4096], Instant::now() + wait);
    let mut ending = Ending::default();
    loop {
        let left = end.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(late());
        }
        conn.set_read_timeout(Some(left.min(READ_WAIT)))?;
        let n = match conn.read(&mut buf) {
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) if matches!(e.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut) && Instant::now() >= end => return Err(late()),
            Err(e) => return Err(e),
        };
        reply.extend_from_slice(&buf[..n]);
        if ending.reached(&reply) {
            return Ok(String::from_utf8_lossy(&reply).into_owned());
        }
        if n == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "The finder's answer was cut short."));
        }
        if reply.len() as u64 >= REPLY_MOST {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "The finder's answer was too long."));
        }
    }
}

impl Files {
    pub(super) fn find_neighbours(&mut self) {
        let Some(post) = self.post.clone() else { return };
        if self.near.looking() {
            return;
        }
        (self.near.finder, self.near.announced) = (Way::Asking, Way::Asking);
        let (found, over) = (post.clone(), post.clone());
        let _ = std::thread::Builder::new().name("folders-nearby".into()).spawn(move || {
            let asked = super::nearby::look(|computers| {
                found.send(Msg::Announced(computers));
            });
            over.send(Msg::Looked(Looked::Announced(asked)));
        });
        let _ = std::thread::Builder::new().name("folders-neighbours".into()).spawn(move || {
            let there = socket().is_some_and(|socket| {
                if talk(&socket, "probe").is_err() {
                    crate::common::run::spawn(&["trindle-wsdd"]);
                    std::thread::sleep(Duration::from_millis(300));
                    if talk(&socket, "probe").is_err() {
                        return false;
                    }
                }
                for wait in [FIRST_WAIT, SECOND_WAIT] {
                    std::thread::sleep(wait);
                    if let Ok(reply) = talk(&socket, "list") {
                        post.send(Msg::Neighbours(parse(&reply)));
                    }
                }
                true
            });
            post.send(Msg::Looked(Looked::Finder(there)));
        });
    }

    pub(super) fn neighbours_looked(&mut self, which: Looked) {
        use super::nearby::Asked;
        match which {
            Looked::Finder(true) => self.near.finder = Way::Asked,
            Looked::Finder(false) => self.near.finder = Way::Could(t!("folders-neighbours-no-finder").into()),
            Looked::Announced(Asked::Yes) => self.near.announced = Way::Asked,
            Looked::Announced(Asked::NoCard) => self.near.announced = Way::Could(t!("folders-neighbours-no-card").into()),
            Looked::Announced(Asked::Refused(why)) => self.near.announced = Way::Could(t!("folders-neighbours-refused", why = &why)),
        }
    }

    pub(super) fn neighbours_found(&mut self, found: Vec<Computer>) {
        self.near.wsd = found;
        self.neighbours_now();
    }

    pub(super) fn neighbours_announced(&mut self, found: Vec<Computer>) {
        self.near.mdns = found;
        self.neighbours_now();
    }

    fn neighbours_now(&mut self) {
        let found = self.near.all();
        if self.neighbours == found {
            return;
        }
        self.neighbours = found;
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                if self.cols[p].tabs[t].at == Where::Network {
                    self.load(t, p);
                }
            }
        }
    }

    pub(super) fn neighbour_entries(&self) -> Vec<Entry> {
        self.neighbours.iter().map(|c| Entry {
            name: c.name.clone(),
            path: PathBuf::from(format!("{SCHEME}{}/", c.name)),
            folder: true,
            size: 0,
            when: 0,
            kind: None,
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{BufRead, BufReader};

    #[test]
    fn computers_are_read_from_the_finders_reply() {
        let reply = "urn:uuid:1\tNAS\tWORKGROUP\t2026-10-04 12:00:00\teth0, {'fe80::2', '192.168.1.20'}\tpub:Computer\n\
                     urn:uuid:2\tdesk-pc\tWORKGROUP\t2026-10-04 12:00:01\twlan0, {'192.168.1.31'}\tpub:Computer\n\
                     urn:uuid:3\tnas\tWORKGROUP\t2026-10-04 12:00:02\teth0, {'192.168.1.20'}\tpub:Computer\n\
                     urn:uuid:4\t\tWORKGROUP\t2026-10-04 12:00:02\teth0, {'192.168.1.9'}\tpub:Computer\n.\n";
        assert_eq!(parse(reply), [
            Computer { name: "desk-pc".into(), address: Some("192.168.1.31".into()) },
            Computer { name: "NAS".into(), address: Some("192.168.1.20".into()) },
        ]);
        assert!(parse(".\n").is_empty());
        assert!(is_computer(Path::new("smb://NAS/")) && is_computer(Path::new("smb://NAS")) && !is_computer(Path::new("/home/john")));
        assert!(!is_computer(Path::new("smb://NAS/Photos")) && !is_computer(Path::new("smb://")));
    }

    #[test]
    fn what_the_finder_says_is_checked_and_capped() {
        let line = |name: &str| format!("urn:uuid:1\t{name}\tWORKGROUP\tx\teth0, {{'192.168.1.9'}}\tpub:Computer\n");
        for bad in ["../etc", "-rf", "a b", "nas;reboot", "nas/Photos", "nas@evil", "user:pw@nas", "nas.example.com", "nas%2e", "n\u{e4}s"] {
            assert!(parse(&line(bad)).is_empty(), "{bad:?}");
        }
        let good = parse(&(line("DESK-PC") + &line("nas.local") + &line("desk-pc")));
        assert_eq!(good.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(), ["DESK-PC", "nas"]);
        for c in &good {
            assert!(is_computer(Path::new(&format!("{SCHEME}{}/", c.name))));
        }
        let many: String = (0..5000).map(|n| line(&format!("pc{n}"))).collect();
        assert_eq!(parse(&many).len(), super::super::nearby::COMPUTERS_MOST);
        let d = std::env::temp_dir().join(format!("trindle-neighbours-flood-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let path = d.join("wsdd");
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        let serve = std::thread::spawn(move || {
            let (mut conn, _) = listener.accept().unwrap();
            let mut heard = String::new();
            BufReader::new(conn.try_clone().unwrap()).read_line(&mut heard).unwrap();
            let chunk = line("pc").repeat(256);
            for _ in 0..(2 * 1024 * 1024 / chunk.len()) {
                if conn.write_all(chunk.as_bytes()).is_err() {
                    break;
                }
            }
        });
        let began = Instant::now();
        let reply = talk(&path, "list");
        assert_eq!(reply.map_err(|e| e.to_string()), Err("The finder's answer was too long.".to_string()));
        serve.join().unwrap();
        assert!(began.elapsed() < REPLY_WAIT, "{:?}", began.elapsed());
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn computers_found_show_under_network_and_open() {
        use crate::toolkit::ui::App;
        let mut f = Files::new(Vec::new());
        f.go(0, Where::Network);
        f.update(Msg::Neighbours(vec![Computer { name: "NAS".into(), address: Some("10.0.0.2".into()) }]));
        let pane = &f.cols[0].tabs[0];
        let i = pane.entries.iter().position(|e| e.name == "NAS").expect("the computer listed");
        assert_eq!(pane.entries[i].icon(), super::super::icons::stand_in::SERVER);
        assert!(pane.entries[i].folder && !pane.entries[i].folder_icon());
        f.update(Msg::Pick(0, vec![i]));
        assert_eq!(f.picked_total(0), (1, 0, false));
        f.open(0, i);
        assert_eq!(f.cols[0].tabs[0].at, Where::Remote("smb://NAS/".into()));
        assert!(f.said.contains("find a computer called NAS"), "{}", f.said);
        assert_eq!(f.cols[0].tabs[0].at.up(), Some(Where::Network));
    }

    #[test]
    fn a_nas_found_either_way_is_listed_once() {
        use crate::toolkit::ui::App;
        let mut f = Files::new(Vec::new());
        f.go(0, Where::Network);
        let names = |f: &Files| f.cols[0].tabs[0].entries.iter().map(|e| e.name.clone()).collect::<Vec<_>>();
        let mut got = super::super::nearby::Gathered::default();
        got.take(&super::super::nearby::tests::nas());
        f.update(Msg::Announced(got.computers()));
        assert_eq!(names(&f), ["NAS"]);
        assert_eq!(f.neighbours, [Computer { name: "NAS".into(), address: Some("192.168.1.20".into()) }]);
        f.update(Msg::Neighbours(vec![Computer { name: "nas".into(), address: None }, Computer { name: "desk-pc".into(), address: Some("192.168.1.31".into()) }]));
        assert_eq!(names(&f), ["desk-pc", "NAS"]);
        assert_eq!(f.neighbours[1].address.as_deref(), Some("192.168.1.20"), "the one with an address is kept");
        f.update(Msg::Announced(vec![Computer { name: "desk-pc.local".into(), address: None }]));
        assert_eq!(names(&f), ["desk-pc", "nas"]);
        assert_eq!(f.neighbours[0].address.as_deref(), Some("192.168.1.31"));
        f.update(Msg::Neighbours(Vec::new()));
        f.update(Msg::Announced(got.computers()));
        f.open(0, 0);
        assert_eq!(f.cols[0].tabs[0].at, Where::Remote("smb://NAS/".into()));
    }

    #[test]
    fn an_empty_network_says_what_was_looked_for() {
        use crate::toolkit::ui::App;
        use super::super::nearby::Asked;
        let mut f = Files::new(Vec::new());
        f.go(0, Where::Network);
        let last = "you can still open it by its name or address.";
        assert!(f.near.words().starts_with("No computers sharing folders were found on this network.") && f.near.words().ends_with(last));
        (f.near.finder, f.near.announced) = (Way::Asking, Way::Asking);
        assert_eq!(f.near.words(), "Looking for computers on this network that share folders…");
        f.update(Msg::Looked(Looked::Finder(true)));
        assert!(f.near.looking(), "one way is still asking");
        f.update(Msg::Looked(Looked::Announced(Asked::Yes)));
        assert!(!f.near.looking());
        assert!(f.near.words().contains("Folders looked for Windows computers, and for NAS drives, Macs and Linux computers."), "{}", f.near.words());
        f.update(Msg::Looked(Looked::Finder(false)));
        let words = f.near.words();
        assert!(words.contains("Folders looked for NAS drives, Macs and Linux computers.") && words.contains("trindle-wsdd") && words.ends_with(last), "{words}");
        f.update(Msg::Looked(Looked::Announced(Asked::NoCard)));
        assert!(f.near.words().contains("because this computer isn't connected to a network"), "{}", f.near.words());
        assert!(!f.near.words().contains("Folders looked for"), "{}", f.near.words());
        f.update(Msg::Looked(Looked::Announced(Asked::Refused("Operation not permitted".into()))));
        assert!(f.near.words().contains("Folders couldn't look for NAS drives, Macs and Linux computers (Operation not permitted)."), "{}", f.near.words());
        let said = super::super::view::view(&f).said();
        assert!(said.contains(&f.near.words()) && said.contains(&"button: Connect to a server".to_string()), "{said:?}");
        f.update(Msg::Announced(vec![Computer { name: "NAS".into(), address: None }]));
        assert!(!super::super::view::view(&f).said().iter().any(|w| w.starts_with("No computers")));
    }

    #[test]
    fn the_finder_is_asked_over_its_socket() {
        let d = std::env::temp_dir().join(format!("trindle-neighbours-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let path = d.join("wsdd");
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        let serve = std::thread::spawn(move || {
            let mut heard = Vec::new();
            for _ in 0..2 {
                let (conn, _) = listener.accept().unwrap();
                let mut out = conn.try_clone().unwrap();
                let mut line = String::new();
                BufReader::new(conn).read_line(&mut line).unwrap();
                if line.trim() == "list" {
                    out.write_all(b"urn:uuid:1\tNAS\tWORKGROUP\tx\teth0, {'10.0.0.2'}\tpub:Computer\n.\n").unwrap();
                }
                heard.push(line.trim().to_string());
            }
            heard
        });
        assert_eq!(talk(&path, "probe").unwrap(), "");
        let reply = talk(&path, "list").unwrap();
        assert_eq!(parse(&reply)[0].name, "NAS");
        assert_eq!(serve.join().unwrap(), ["probe", "list"]);
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_finder_that_dribbles_and_never_ends_a_line_is_given_up_on() {
        let d = std::env::temp_dir().join(format!("trindle-neighbours-dribble-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let path = d.join("wsdd");
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        let serve = std::thread::spawn(move || {
            let (mut conn, _) = listener.accept().unwrap();
            for _ in 0..150 {
                if conn.write_all(b"x").is_err() {
                    break;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        });
        let began = Instant::now();
        let conn = UnixStream::connect(&path).unwrap();
        let got = read_reply(conn, Duration::from_secs(2));
        let took = began.elapsed();
        assert_eq!(got.as_ref().map_err(|e| e.to_string()), Err("The finder took too long to answer.".to_string()));
        assert!(took >= Duration::from_millis(1900) && took < Duration::from_secs(4), "listened for {took:?}");
        serve.join().unwrap();
        assert_eq!(REPLY_WAIT, Duration::from_secs(6));
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_reply_cut_short_is_an_error_and_never_half_an_address() {
        let cut = b"urn:uuid:1\tNAS\tWORKGROUP\tx\teth0, {'192.168.1.2";
        let (conn, mut finder) = UnixStream::pair().unwrap();
        finder.write_all(cut).unwrap();
        drop(finder);
        let got = read_reply(conn, Duration::from_secs(2));
        assert_eq!(got.as_ref().map_err(|e| e.to_string()), Err("The finder's answer was cut short.".to_string()), "{got:?}");
        assert_eq!(parse(&format!("{}'}}", String::from_utf8_lossy(cut)))[0].address.as_deref(), Some("192.168.1.2"));
        let (conn, mut finder) = UnixStream::pair().unwrap();
        finder.write_all(b"urn:uuid:1\tNAS\tWORKGROUP\tx\teth0, {'10.0.0.2'}\tpub:Computer\n").unwrap();
        drop(finder);
        assert!(read_reply(conn, Duration::from_secs(2)).is_err());
        let (conn, mut finder) = UnixStream::pair().unwrap();
        let fill = std::thread::spawn(move || {
            let line = b"urn:uuid:1\tNAS\tWORKGROUP\tx\teth0, {'10.0.0.2'}\tpub:Computer\n";
            for _ in 0..(REPLY_MOST as usize / line.len() + 2) {
                if finder.write_all(line).is_err() {
                    break;
                }
            }
        });
        let got = read_reply(conn, Duration::from_secs(5));
        assert_eq!(got.as_ref().map_err(|e| e.to_string()), Err("The finder's answer was too long.".to_string()));
        fill.join().unwrap();
        let whole = b"urn:uuid:1\tNAS\tWORKGROUP\tx\teth0, {'10.0.0.20'}\tpub:Computer\n.\n";
        let (conn, mut finder) = UnixStream::pair().unwrap();
        let slow = std::thread::spawn(move || {
            for b in whole {
                finder.write_all(&[*b]).unwrap();
            }
            std::thread::sleep(Duration::from_millis(600));
        });
        let began = Instant::now();
        let reply = read_reply(conn, Duration::from_secs(5)).unwrap();
        assert!(began.elapsed() < Duration::from_millis(500), "it waited past the full stop");
        assert_eq!(parse(&reply)[0].address.as_deref(), Some("10.0.0.20"));
        slow.join().unwrap();
    }

    #[test]
    fn the_end_of_a_reply_is_looked_for_in_the_new_bytes_only() {
        let mut end = Ending::default();
        let mut reply = Vec::new();
        let line = b"urn:uuid:1\tNAS\tWORKGROUP\tx\teth0, {'10.0.0.2'}\tpub:Computer\n";
        for b in line.iter().cycle().take(64 * 1024) {
            reply.push(*b);
            assert!(!end.reached(&reply));
        }
        for (b, last) in [(b'\n', false), (b' ', false), (b'.', false), (b'\r', false), (b'\n', true)] {
            reply.push(b);
            assert_eq!(end.reached(&reply), last);
        }
        assert!(end.looked <= 2 * reply.len(), "looked at {} bytes for a reply of {}", end.looked, reply.len());
        let mut end = Ending::default();
        assert!(!end.reached(b"a.b\n") && !end.reached(b"a.b\n.") && end.reached(b"a.b\n.\n"));
    }
}
