// Tests of the FTP link: replies, listings, and a made-up FTP server.

use super::*;
use std::net::TcpListener;

const NOW: i64 = 1_791_030_605;

#[test]
fn a_lost_connection_is_its_own_kind_and_a_refusal_is_not() {
    use io::ErrorKind as K;
    for kind in [K::UnexpectedEof, K::BrokenPipe, K::ConnectionReset, K::TimedOut, K::WouldBlock] {
        assert!(matches!(lost(kind.into()), Error::Lost(_)), "{kind:?}");
    }
    assert!(matches!(failed(421, "421 Closing.", "it"), Error::Lost(_)));
    assert!(matches!(failed(451, "451 Not now.", "it"), Error::Failed(_)));
    assert!(matches!(failed(534, "534 No protection.", "it"), Error::Failed(_)));
}

#[test]
fn replies() {
    let mut one = &b"220 Welcome\r\n"[..];
    assert_eq!(read_reply(&mut one).unwrap(), (220, "Welcome".into()));
    let mut many = &b"211-Features:\r\n MLST type*;size*;\r\n EPSV\r\n211 End\r\n257 \"/x\"\r\n"[..];
    assert_eq!(read_reply(&mut many).unwrap(), (211, "Features:\n MLST type*;size*;\n EPSV\nEnd".into()));
    assert_eq!(read_reply(&mut many).unwrap().0, 257);
    assert!(read_reply(&mut &b"hello\r\n"[..]).is_err());
    assert!(read_reply(&mut &b""[..]).is_err());
}

#[test]
fn what_a_server_sends_without_end_is_stopped() {
    let too_much = t!("folders-remote-mod-too-much");
    let mut line = BufReader::new(io::repeat(b'a').take(64 << 20));
    let why = read_reply(&mut line).unwrap_err().to_string();
    assert_eq!(why, too_much);
    let again = "211-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\r\n".repeat(1 << 16);
    let why = lost(read_reply(&mut again.as_bytes()).unwrap_err()).to_string();
    assert_eq!(why, too_much);
    let big = io::repeat(b'a').take(MOST_LISTING + 1);
    assert_eq!(read_lines(big).unwrap_err().to_string(), too_much);
    let ordinary = format!("{}\r\n", "a".repeat(400)).repeat(1000);
    assert_eq!(read_lines(ordinary.as_bytes()).unwrap().len(), 1000);
    let long = format!("211-{}\r\n211 End\r\n", "a".repeat(8000));
    assert_eq!(read_reply(&mut long.as_bytes()).unwrap().0, 211);
}

#[test]
fn passive_replies() {
    assert_eq!(epsv_port("Entering Extended Passive Mode (|||6446|)"), Some(6446));
    assert_eq!(epsv_port("Entering Extended Passive Mode (!!!21000!)"), Some(21000));
    assert_eq!(epsv_port("no"), None);
    assert_eq!(pasv_port("Entering Passive Mode (192,0,2,1,25,46)."), Some(25 * 256 + 46));
    assert_eq!(pasv_port("Entering Passive Mode 192,0,2,1,4,1"), Some(1025));
    assert_eq!(pasv_port("Entering Passive Mode (192,0,2,1,4)"), None);
    assert_eq!(quoted("\"/home/john\" is the current folder"), Some("/home/john".into()));
    assert_eq!(quoted("\"/a \"\"b\"\"\" is current"), Some("/a \"b\"".into()));
}

#[test]
fn mlsd() {
    let e = mlsd_line("type=file;size=1234;modify=20261003123005.123;UNIX.mode=0644; My notes.txt").unwrap();
    assert_eq!(e, Entry { name: "My notes.txt".into(), kind: Kind::File, size: 1234, modified: Some(NOW), mode: Some(0o644) });
    assert!(mlsd_line("Type=dir;Modify=20261003123005; Photos").unwrap().is_folder());
    assert_eq!(mlsd_line("type=OS.unix=symlink;size=3; link").unwrap().kind, Kind::Link);
    assert_eq!(mlsd_line("type=OS.unix=slink:/x; link2").unwrap().kind, Kind::Link);
    assert!(mlsd_line("type=cdir; /pub").is_none());
    assert!(mlsd_line("type=pdir; ..").is_none());
}

#[test]
fn a_listing_with_a_made_up_time_does_not_abort() {
    for line in ["-rw-r--r-- 1 u g 1 Jan 1 4294967295:00 file", "-rw-r--r-- 1 u g 1 Jan 1 12:4294967295 file", "-rw-r--r-- 1 u g 1 Jan 99999999999999999 file"] {
        let _ = list_line(line, NOW);
    }
    assert!(list_line("-rw-r--r-- 1 u g 1 Jan 1 4294967295:00 file", NOW).is_some());
}

#[test]
fn unix_list() {
    let (e, t) = list_line("-rw-r--r--    1 1000     1000         5120 Oct  3 12:30 My notes.txt", NOW).unwrap();
    assert_eq!(e.name, "My notes.txt");
    assert_eq!((e.kind, e.size, e.mode), (Kind::File, 5120, Some(0o644)));
    assert_eq!(e.modified, Some(unix_time(2026, 10, 3, 12, 30, 0)));
    assert_eq!(t, None);
    let (e, _) = list_line("drwxr-xr-x 2 john staff 4096 Dec 25 09:00 Christmas", NOW).unwrap();
    assert_eq!(e.modified, Some(unix_time(2025, 12, 25, 9, 0, 0)));
    assert!(e.is_folder());
    let (e, _) = list_line("-rwsr-x--T 1 ftp 77 Jan  1  2019 old file", NOW).unwrap();
    assert_eq!((e.name.as_str(), e.size), ("old file", 77));
    assert_eq!(e.mode, Some(0o4000 | 0o1000 | 0o750));
    assert_eq!(e.modified, Some(unix_time(2019, 1, 1, 0, 0, 0)));
    let (e, t) = list_line("lrwxrwxrwx 1 root root 7 Oct  1 08:00 latest -> v2.1/a b", NOW).unwrap();
    assert_eq!((e.name.as_str(), e.kind), ("latest", Kind::Link));
    assert_eq!(t.as_deref(), Some("v2.1/a b"));
    let (e, _) = list_line("-rw-r--r-- 1 a b 9 Oct  3 12:30 Oct 3 12:30", NOW).unwrap();
    assert_eq!(e.name, "Oct 3 12:30");
    assert!(list_line("total 48", NOW).is_none());
}

#[test]
fn dos_list() {
    let (e, _) = list_line("10-03-26  12:30PM       <DIR>          Photos", NOW).unwrap();
    assert_eq!((e.name.as_str(), e.kind), ("Photos", Kind::Folder));
    assert_eq!(e.modified, Some(unix_time(2026, 10, 3, 12, 30, 0)));
    let (e, _) = list_line("01-02-1999  12:05AM              1234 Read me.txt", NOW).unwrap();
    assert_eq!((e.name.as_str(), e.size), ("Read me.txt", 1234));
    assert_eq!(e.modified, Some(unix_time(1999, 1, 2, 0, 5, 0)));
}

#[test]
fn line_breaks_never_reach_the_server() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().unwrap();
        s.write_all(b"220 hi\r\n").unwrap();
        let mut got = Vec::new();
        let _ = s.read_to_end(&mut got);
        got
    });
    let mut f = Ftp::over(Link::Plain(TcpStream::connect(("127.0.0.1", port)).unwrap())).unwrap();
    assert!(matches!(f.remove("/a\r\nDELE /b"), Err(Error::BadAddress(_))));
    drop(f);
    assert_eq!(server.join().unwrap(), b"QUIT\r\n");
}

fn serve(listener: TcpListener, file: Vec<u8>) -> Vec<u8> {
    let (s, _) = listener.accept().unwrap();
    let mut r = BufReader::new(s.try_clone().unwrap());
    let mut w = s;
    let mut data_listener: Option<TcpListener> = None;
    let mut stored = Vec::new();
    w.write_all(b"220-Welcome\r\n220 Ready\r\n").unwrap();
    loop {
        let mut line = String::new();
        if r.read_line(&mut line).unwrap() == 0 {
            return stored;
        }
        let line = line.trim_end();
        let (cmd, arg) = line.split_once(' ').unwrap_or((line, ""));
        let reply: String = match cmd {
            "USER" => "331 Password please".into(),
            "PASS" if arg == "anonymous@" => "230 In".into(),
            "TYPE" => "200 Binary".into(),
            "PWD" => "257 \"/pub\" is here".into(),
            "EPSV" | "MLSD" | "MLST" => "502 No".into(),
            "PASV" => {
                let l = TcpListener::bind("127.0.0.1:0").unwrap();
                let p = l.local_addr().unwrap().port();
                data_listener = Some(l);
                format!("227 Passive (10,9,8,7,{},{})", p / 256, p % 256)
            }
            "CWD" => "250 Ok".into(),
            "LIST" | "RETR" | "STOR" => {
                w.write_all(b"150 Here it comes\r\n").unwrap();
                let (mut d, _) = data_listener.take().unwrap().accept().unwrap();
                match cmd {
                    "LIST" => d.write_all(b"total 1\r\n-rw-r--r-- 1 ftp ftp 70000 Oct  3 12:30 big file.bin\r\nlrwxrwxrwx 1 ftp ftp 12 Oct  3 12:30 link -> big file.bin\r\n").unwrap(),
                    "RETR" => d.write_all(&file).unwrap(),
                    _ => {
                        d.read_to_end(&mut stored).unwrap();
                    }
                }
                "226 Done".into()
            }
            "QUIT" => return stored,
            _ => "500 What".into(),
        };
        w.write_all(format!("{reply}\r\n").as_bytes()).unwrap();
    }
}

trait Rw: Read + Write + Send {}
impl<T: Read + Write + Send> Rw for T {}

#[derive(Clone, Copy, PartialEq)]
enum Way {
    Implicit,
    Explicit,
    Demanded,
}

fn tls_config(c: &rcgen::CertifiedKey) -> std::sync::Arc<rustls::ServerConfig> {
    let provider = std::sync::Arc::new(rustls::crypto::ring::default_provider());
    let key = rustls::pki_types::PrivateKeyDer::Pkcs8(c.key_pair.serialize_der().into());
    std::sync::Arc::new(
        rustls::ServerConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(vec![c.cert.der().clone()], key)
            .unwrap(),
    )
}

fn accept_tls(s: TcpStream, config: &std::sync::Arc<rustls::ServerConfig>) -> rustls::StreamOwned<rustls::ServerConnection, TcpStream> {
    rustls::StreamOwned::new(rustls::ServerConnection::new(config.clone()).unwrap(), s)
}

fn say(ctl: &mut BufReader<Box<dyn Rw>>, line: &str) -> Option<()> {
    let w = ctl.get_mut();
    w.write_all(format!("{line}\r\n").as_bytes()).ok()?;
    w.flush().ok()
}

fn secure_transfer(ctl: &mut BufReader<Box<dyn Rw>>, data: &mut Option<TcpListener>, cmd: &str, config: &std::sync::Arc<rustls::ServerConfig>, file: &[u8], stored: &mut Vec<u8>) -> Option<String> {
    say(ctl, "150 Here it comes")?;
    let (d, _) = data.take()?.accept().ok()?;
    d.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    let mut d = accept_tls(d, config);
    match cmd {
        "MLSD" => d.write_all(format!("type=file;size={};modify=20261003123005; secret.bin\r\n", file.len()).as_bytes()).ok()?,
        "RETR" => d.write_all(file).ok()?,
        _ => {
            d.read_to_end(stored).ok()?;
        }
    }
    d.conn.send_close_notify();
    let _ = d.flush();
    Some("226 Done".into())
}

fn serve_secure(s: TcpStream, way: Way, config: std::sync::Arc<rustls::ServerConfig>, file: &[u8]) -> Option<Vec<u8>> {
    s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    let raw = s.try_clone().unwrap();
    let mut secure = way == Way::Implicit;
    let first: Box<dyn Rw> = if secure { Box::new(accept_tls(s, &config)) } else { Box::new(s) };
    let mut ctl = BufReader::new(first);
    let (mut data, mut prot, mut stored) = (None::<TcpListener>, false, Vec::new());
    say(&mut ctl, "220 Secure stand-in ready")?;
    loop {
        let mut line = String::new();
        if ctl.read_line(&mut line).ok()? == 0 {
            return None;
        }
        let line = line.trim_end();
        let (cmd, arg) = line.split_once(' ').unwrap_or((line, ""));
        let reply: String = match cmd {
            "AUTH" if !secure && arg == "TLS" => {
                say(&mut ctl, "234 Go ahead")?;
                ctl = BufReader::new(Box::new(accept_tls(raw.try_clone().unwrap(), &config)));
                secure = true;
                continue;
            }
            "USER" if way == Way::Demanded && !secure => "530 Non-anonymous sessions must use encryption.".into(),
            "USER" => "331 Password please".into(),
            "PASS" if arg == "secret" => "230 In".into(),
            "PASS" => "530 No".into(),
            "PBSZ" => "200 PBSZ=0".into(),
            "PROT" if arg == "P" && secure => {
                prot = true;
                "200 Protected".into()
            }
            "TYPE" | "OPTS" => "200 Ok".into(),
            "PWD" => "257 \"/home\" is here".into(),
            "EPSV" => {
                let l = TcpListener::bind("127.0.0.1:0").unwrap();
                let p = l.local_addr().unwrap().port();
                data = Some(l);
                format!("229 Extended (|||{p}|)")
            }
            "MLSD" | "RETR" | "STOR" => {
                if !prot {
                    return None;
                }
                secure_transfer(&mut ctl, &mut data, cmd, &config, file, &mut stored)?
            }
            "QUIT" => {
                let _ = say(&mut ctl, "221 Bye");
                return Some(stored);
            }
            _ => "500 What".into(),
        };
        say(&mut ctl, &reply)?;
    }
}

fn cleared(a: &Address, login: &Login) -> Leave<Login> {
    use crate::files::leave;
    let to = leave::Reached::now(&a.host, None).map_or(leave::Way::Nowhere, leave::Way::At);
    leave::leave(login.clone(), &to).expect("an address that was typed is given")
}

#[test]
fn a_link_goes_to_the_one_address_it_was_cleared_for() {
    use crate::files::leave;
    let serve_once = |listener: TcpListener| {
        std::thread::spawn(move || {
            let (s, _) = listener.accept().unwrap();
            s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
            let mut r = BufReader::new(s.try_clone().unwrap());
            let mut w = s;
            let mut heard = Vec::new();
            w.write_all(b"220 Ready\r\n").unwrap();
            loop {
                let mut line = String::new();
                if r.read_line(&mut line).unwrap_or(0) == 0 {
                    return heard;
                }
                let line = line.trim_end().to_string();
                let reply = match line.split(' ').next().unwrap_or_default() {
                    "USER" => "331 Password please",
                    "PASS" => "230 In",
                    "QUIT" => "221 Bye",
                    _ => "200 Ok",
                };
                heard.push(line);
                if w.write_all(format!("{reply}\r\n").as_bytes()).is_err() {
                    return heard;
                }
            }
        })
    };
    let shown = leave::Way::At(leave::Confirmed::made_up("127.0.0.1").reached());
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = serve_once(listener);
    let a = Address::parse(&format!("ftp://jo:written@no-such-name.invalid:{port}/")).unwrap();
    let held = Login { password: Some("held".into()), ..Login::default() };
    let to = leave::leave(held, &shown).expect("shown on the card");
    drop(Ftp::connect(&a, &to).expect("the address shown was not the one dialled"));
    let heard = server.join().unwrap();
    assert!(heard.contains(&"PASS held".to_string()) && !heard.iter().any(|l| l.contains("written")), "{heard:?}");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = serve_once(listener);
    let a = Address::parse(&format!("ftp://jo:written@no-such-name.invalid:{port}/")).unwrap();
    let to = leave::leave(Login::default(), &shown).expect("nothing in it");
    assert!(matches!(Ftp::connect(&a, &to).map(drop), Err(Error::NeedsPassword)));
    let heard = server.join().unwrap();
    assert!(!heard.iter().any(|l| l.starts_with("PASS")), "{heard:?}");
}

#[test]
fn secure_ftp_asks_once_then_remembers() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let first = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    let second = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    let fp1 = super::super::tls::fingerprint(first.cert.der());
    let fp2 = super::super::tls::fingerprint(second.cert.der());
    let (c1, c2) = (tls_config(&first), tls_config(&second));
    let file: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
    let served = file.clone();
    let configs = vec![c1.clone(), c1.clone(), c1, c2];
    let server = std::thread::spawn(move || {
        configs.into_iter().map(|c| serve_secure(listener.accept().unwrap().0, Way::Explicit, c, &served)).collect::<Vec<_>>()
    });
    let a = Address::parse(&format!("ftpes://john@127.0.0.1:{port}/")).unwrap();
    let pw = Login { password: Some("secret".into()), ..Login::default() };
    match Ftp::connect(&a, &cleared(&a, &pw)).map(drop) {
        Err(Error::CertUnknown { host, fingerprint, .. }) => assert_eq!((host.as_str(), fingerprint.as_str()), ("127.0.0.1", fp1.as_str())),
        other => panic!("expected a certificate question, got {other:?}"),
    }
    let trusted = Login { trust_cert: Some(fp1.clone()), ..pw.clone() };
    let mut f = Ftp::connect(&a, &cleared(&a, &trusted)).unwrap();
    assert_eq!(f.home().unwrap(), "/home");
    let list = f.list("/home").unwrap();
    assert_eq!((list[0].name.as_str(), list[0].size), ("secret.bin", 200_000));
    let mut back = Vec::new();
    assert_eq!(f.read("/home/secret.bin", &mut back, &mut |_| true).unwrap(), 200_000);
    assert!(back == file);
    assert_eq!(f.write("/home/up.bin", &mut &b"hello secure"[..], 12, &mut |_| true).unwrap(), 12);
    drop(f);
    let mut f = Ftp::connect(&a, &cleared(&a, &pw)).unwrap();
    assert_eq!(f.home().unwrap(), "/home");
    drop(f);
    match Ftp::connect(&a, &cleared(&a, &pw)).map(drop) {
        Err(Error::CertChanged { fingerprint, .. }) => assert_eq!(fingerprint, fp2),
        other => panic!("expected a changed certificate, got {other:?}"),
    }
    let got = server.join().unwrap();
    assert_eq!(got, vec![None, Some(b"hello secure".to_vec()), Some(Vec::new()), None]);
}

#[test]
fn implicit_and_demanded_tls() {
    for way in [Way::Implicit, Way::Demanded] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let fp = super::super::tls::fingerprint(cert.cert.der());
        let config = tls_config(&cert);
        let server = std::thread::spawn(move || serve_secure(listener.accept().unwrap().0, way, config, b"tiny"));
        let scheme = if way == Way::Implicit { "ftps" } else { "ftp" };
        let a = Address::parse(&format!("{scheme}://john@127.0.0.1:{port}/")).unwrap();
        let login = Login { password: Some("secret".into()), trust_cert: Some(fp), ..Login::default() };
        let mut f = Ftp::connect(&a, &cleared(&a, &login)).unwrap();
        let mut back = Vec::new();
        assert_eq!(f.read("/home/tiny", &mut back, &mut |_| true).unwrap(), 4);
        assert_eq!(back, b"tiny");
        f.write("/home/up", &mut &b"up"[..], 2, &mut |_| true).unwrap();
        drop(f);
        assert_eq!(server.join().unwrap(), Some(b"up".to_vec()), "{scheme}");
    }
}

#[test]
fn round_trip_on_loopback() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let file: Vec<u8> = (0..70_000u32).map(|i| (i % 253) as u8).collect();
    let served = file.clone();
    let server = std::thread::spawn(move || serve(listener, served));
    let a = Address::parse(&format!("ftp://127.0.0.1:{port}/")).unwrap();
    let mut f = Ftp::connect(&a, &cleared(&a, &Login::default())).unwrap();
    assert_eq!(f.home().unwrap(), "/pub");
    let list = f.list("/pub").unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!((list[0].name.as_str(), list[0].size), ("big file.bin", 70_000));
    assert_eq!(f.readlink("/pub/link").unwrap(), "big file.bin");
    assert_eq!(f.stat("/pub/big file.bin").unwrap().size, 70_000);
    let mut back = Vec::new();
    assert_eq!(f.read("/pub/big file.bin", &mut back, &mut |_| true).unwrap(), 70_000);
    assert!(back == file);
    let n = f.write("/pub/up.bin", &mut &b"hello there"[..], 11, &mut |_| true).unwrap();
    assert_eq!(n, 11);
    drop(f);
    assert_eq!(server.join().unwrap(), b"hello there");
}
