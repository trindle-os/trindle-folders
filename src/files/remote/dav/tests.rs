//! Tests of the WebDAV link: its listings and a made-up server.

use super::*;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

#[test]
fn listings_dates_and_challenges_are_read() {
    let xml = r#"<?xml version="1.0"?><d:multistatus xmlns:d="DAV:"><d:response><d:href>/dav/Holiday%20photos/</d:href>
        <d:propstat><d:prop><d:resourcetype><d:collection/></d:resourcetype><d:getlastmodified>Sun, 06 Nov 1994 08:49:37 GMT</d:getlastmodified></d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat>
        <d:propstat><d:prop><d:getcontentlength/></d:prop><d:status>HTTP/1.1 404 Not Found</d:status></d:propstat></d:response>
        <d:response><d:href>https://example.com/dav/Holiday%20photos/sea%20%26%20sand.jpg</d:href><d:propstat><d:prop><d:resourcetype/><d:getcontentlength>2048</d:getcontentlength></d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>
        <d:response><d:href>/dav/Holiday%20photos/A&amp;B.txt</d:href><d:propstat><d:prop><d:getcontentlength>5</d:getcontentlength></d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response></d:multistatus>"#;
    let got = parse_listing(xml).unwrap();
    assert_eq!(got.len(), 3);
    assert_eq!((got[0].0.as_str(), got[0].1.name.as_str(), got[0].1.kind, got[0].1.modified),
               ("/dav/Holiday photos/", "Holiday photos", Kind::Folder, Some(784111777)));
    assert_eq!((got[1].0.as_str(), got[1].1.size, got[1].1.kind), ("/dav/Holiday photos/sea & sand.jpg", 2048, Kind::File));
    assert_eq!(got[2].1.name, "A&B.txt");
    assert_eq!(encode("/dav/sea & sand.jpg"), "/dav/sea%20%26%20sand.jpg");
    assert_eq!(base64(b"jo:secret"), "am86c2VjcmV0");
    assert_eq!(base64(b"ab"), "YWI=");
    let digest = choose_auth(&["Basic realm=\"x\"".into(), "Digest realm=\"files@example.com\", qop=\"auth,auth-int\", nonce=\"dcd98b\", opaque=\"5ccc\"".into()]);
    assert_eq!(digest, Some(Auth::Digest { realm: "files@example.com".into(), nonce: "dcd98b".into(), opaque: Some("5ccc".into()), qop: true, count: 0 }));
    assert_eq!(choose_auth(&["Basic realm=\"x\"".into()]), Some(Auth::Basic));
    assert_eq!(choose_auth(&["Bearer".into()]), None);
}

fn sends(bytes: Vec<u8>) -> Link {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        if let Ok((mut conn, _)) = listener.accept() {
            let _ = conn.write_all(&bytes);
        }
    });
    Link::Plain(TcpStream::connect(("127.0.0.1", port)).unwrap())
}

#[test]
fn what_a_server_sends_without_end_is_stopped() {
    let too_much = t!("folders-remote-mod-too-much");
    let why = read_head(sends(vec![b'a'; 64 << 20])).err().expect("a status line without end").to_string();
    assert_eq!(why, too_much);
    let mut heads = b"HTTP/1.1 207 Multi-Status\r\n".to_vec();
    heads.extend(b"X-Filler: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\r\n".repeat(700_000));
    let why = read_head(sends(heads)).err().expect("headers without end").to_string();
    assert_eq!(why, too_much);
    let mut body = Body { from: BufReader::new(sends(vec![b'f'; 64 << 20])), left: Framing::Chunked(0, false) };
    let why = body.read(&mut [0u8; 16]).unwrap_err().to_string();
    assert_eq!(why, too_much);
    let reply = b"HTTP/1.1 207 Multi-Status\r\nContent-Type: text/xml\r\nTransfer-Encoding: chunked\r\n\r\n5;ext=1\r\nhello\r\n0\r\n\r\n".to_vec();
    let mut got = read_head(sends(reply)).unwrap();
    let mut text = String::new();
    got.body.read_to_string(&mut text).unwrap();
    assert_eq!((got.status, text.as_str()), (207, "hello"));
}

type Held = Arc<Mutex<BTreeMap<String, Vec<u8>>>>;

fn server(digest: bool) -> (u16, Held) {
    let files = Arc::new(Mutex::new(BTreeMap::from([("/dav/a.txt".to_string(), b"hello".to_vec()), ("/dav/sub/".to_string(), Vec::new())])));
    let store = files.clone();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for conn in listener.incoming() {
            let Ok(conn) = conn else { continue };
            answer(conn, &store, digest);
        }
    });
    (port, files)
}

struct Asked {
    method: String,
    target: String,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

fn read_request(conn: &TcpStream) -> Asked {
    let mut r = BufReader::new(conn.try_clone().unwrap());
    let mut line = String::new();
    r.read_line(&mut line).unwrap();
    let mut parts = line.split_whitespace();
    let (method, target) = (parts.next().unwrap_or("").to_string(), href_path(parts.next().unwrap_or("/")));
    let mut headers = BTreeMap::new();
    loop {
        let mut h = String::new();
        r.read_line(&mut h).unwrap();
        if h.trim().is_empty() {
            break;
        }
        let (k, v) = h.split_once(':').unwrap();
        headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
    }
    let len: usize = headers.get("content-length").and_then(|n| n.parse().ok()).unwrap_or(0);
    let mut body = vec![0u8; len];
    r.read_exact(&mut body).unwrap();
    Asked { method, target, headers, body }
}

fn is_authed(asked: &Asked, digest: bool) -> bool {
    asked.headers.get("authorization").is_some_and(|a| if digest {
        a.starts_with("Digest") && a.contains("username=\"jo\"") && {
            let f = fields(a);
            let get = |k: &str| f.iter().find(|(x, _)| x == k).map(|(_, v)| v.clone()).unwrap_or_default();
            let ha1 = md5_hex("jo:test:secret");
            let ha2 = md5_hex(&format!("{}:{}", asked.method, get("uri")));
            get("response") == md5_hex(&format!("{ha1}:abc:{}:{}:auth:{ha2}", get("nc"), get("cnonce")))
        }
    } else {
        a == &format!("Basic {}", base64(b"jo:secret"))
    })
}

fn send(conn: &mut TcpStream, status: &str, extra: &str, body: &[u8]) {
    let _ = write!(conn, "HTTP/1.1 {status}\r\n{extra}Content-Length: {}\r\n\r\n", body.len());
    let _ = conn.write_all(body);
}

fn tag(files: &BTreeMap<String, Vec<u8>>, p: &str) -> String {
    let sum = |d: &[u8]| d.iter().map(|b| u64::from(*b)).sum::<u64>();
    if p.ends_with('/') {
        format!("\"d{}\"", files.keys().filter(|k| k.starts_with(p) && k.as_str() != p).count())
    } else {
        format!("\"f{}-{}\"", files.get(p).map_or(0, Vec::len), files.get(p).map_or(0, |d| sum(d)))
    }
}

fn answer(mut conn: TcpStream, store: &Held, digest: bool) {
    let asked = read_request(&conn);
    if !is_authed(&asked, digest) {
        let ask = if digest { "WWW-Authenticate: Digest realm=\"test\", nonce=\"abc\", qop=\"auth\"\r\n" } else { "WWW-Authenticate: Basic realm=\"test\"\r\n" };
        send(&mut conn, "401 Unauthorized", ask, b"");
        return;
    }
    let mut files = store.lock().unwrap();
    let Asked { method, target, headers, body } = asked;
    match method.as_str() {
        "PROPFIND" => propfind(&mut conn, &mut files, &target, headers.get("depth").map_or("", String::as_str)),
        "DELETE" if headers.get("if-match").is_some_and(|want| {
            let folder = format!("{}/", target.trim_end_matches('/'));
            *want != tag(&files, if files.contains_key(&folder) { &folder } else { &target })
        }) => send(&mut conn, "412 Precondition Failed", "", b""),
        "GET" => match files.get(&target) {
            Some(d) if target.contains("unsized") => {
                let _ = write!(conn, "HTTP/1.1 200 OK\r\n\r\n");
                let _ = conn.write_all(&d[..d.len() / 2]);
            }
            Some(d) => send(&mut conn, "200 OK", "", d),
            None => send(&mut conn, "404 Not Found", "", b""),
        },
        "PUT" if headers.get("if-none-match").is_some_and(|h| h == "*") && files.contains_key(&target) => {
            send(&mut conn, "412 Precondition Failed", "", b"");
        }
        "PUT" => {
            files.insert(target, body);
            send(&mut conn, "201 Created", "", b"");
        }
        "MKCOL" => {
            files.insert(format!("{}/", target.trim_end_matches('/')), Vec::new());
            send(&mut conn, "201 Created", "", b"");
        }
        "DELETE" => {
            files.retain(|p, _| !p.starts_with(&target));
            send(&mut conn, "204 No Content", "", b"");
        }
        "MOVE" => {
            let to = href_path(headers.get("destination").unwrap());
            if files.contains_key(&to) {
                send(&mut conn, "412 Precondition Failed", "", b"");
            } else if let Some(d) = files.remove(&target) {
                files.insert(to, d);
                send(&mut conn, "201 Created", "", b"");
            }
        }
        _ => send(&mut conn, "405 Method Not Allowed", "", b""),
    }
}

fn propfind(conn: &mut TcpStream, files: &mut BTreeMap<String, Vec<u8>>, target: &str, depth: &str) {
    let dir = format!("{}/", target.trim_end_matches('/'));
    let (xml, any) = multistatus(files, target, depth, &dir);
    if any {
        let half = xml.len() / 2;
        let _ = write!(conn, "HTTP/1.1 207 Multi-Status\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n{}\r\n{:x}\r\n{}\r\n0\r\n\r\n",
                       half, &xml[..half], xml.len() - half, &xml[half..]);
    } else {
        send(conn, "404 Not Found", "", b"");
    }
    if target.contains("busy") {
        if files.contains_key(&dir) {
            files.insert(format!("{dir}late.txt"), b"put there meanwhile".to_vec());
        } else if let Some(d) = files.get_mut(target) {
            *d = b"saved again meanwhile".to_vec();
        }
    }
}

fn multistatus(files: &BTreeMap<String, Vec<u8>>, target: &str, depth: &str, dir: &str) -> (String, bool) {
    let mut xml = String::from(r#"<?xml version="1.0"?><D:multistatus xmlns:D="DAV:">"#);
    let mut any = false;
    for (p, data) in files.iter() {
        let folder = p.ends_with('/');
        let me = p.trim_end_matches('/') == target.trim_end_matches('/');
        let child = depth == "1" && p.starts_with(dir) && !p[dir.len()..].trim_end_matches('/').contains('/') && p.len() > dir.len();
        if me || child || (depth == "1" && dir == "/dav/" && p == "/dav/") {
            any = true;
            if p.contains("forbidden") {
                xml.push_str(&format!("<D:response><D:href>{}</D:href><D:propstat><D:prop><D:resourcetype/><D:getcontentlength/></D:prop><D:status>HTTP/1.1 403 Forbidden</D:status></D:propstat></D:response>", encode(p)));
                continue;
            }
            let kind = if folder { "<D:collection/>" } else { "" };
            let etag = match tag(files, p) {
                _ if p.contains("untagged") => String::new(),
                t if p.contains("weak") => format!("<D:getetag>W/{}</D:getetag>", t.replace('"', "&quot;")),
                t => format!("<D:getetag>{}</D:getetag>", t.replace('"', "&quot;")),
            };
            xml.push_str(&format!("<D:response><D:href>{}</D:href><D:propstat><D:prop><D:resourcetype>{kind}</D:resourcetype><D:getcontentlength>{}</D:getcontentlength>{etag}</D:prop><D:status>HTTP/1.1 200 OK</D:status></D:propstat></D:response>", encode(p), data.len()));
        }
    }
    if target.trim_end_matches('/') == "/dav" {
        any = true;
        xml.push_str("<D:response><D:href>/dav/</D:href><D:propstat><D:prop><D:resourcetype><D:collection/></D:resourcetype></D:prop><D:status>HTTP/1.1 200 OK</D:status></D:propstat></D:response>");
    }
    xml.push_str("</D:multistatus>");
    (xml, any)
}

fn open(port: u16, password: Option<&str>) -> Result<Dav, Error> {
    use crate::files::leave;
    let address = Address::parse(&format!("dav://jo@127.0.0.1:{port}/dav/")).unwrap();
    let to = leave::Reached::now(&address.host, None).map_or(leave::Way::Nowhere, leave::Way::At);
    let to = leave::leave(Login { password: password.map(|p| String::from(p).into()), ..Login::default() }, &to).expect("an address that was typed is given");
    Dav::connect(&address, &to)
}

#[test]
fn a_link_goes_to_the_one_address_it_was_cleared_for() {
    use crate::files::leave;
    let (port, _files) = server(false);
    let shown = leave::Way::At(leave::Confirmed::made_up("127.0.0.1").reached());
    let address = Address::parse(&format!("dav://jo:secret@no-such-name.invalid:{port}/dav/")).unwrap();
    let to = leave::leave(Login { password: Some(String::from("secret").into()), ..Login::default() }, &shown).expect("shown on the card");
    let mut dav = Dav::connect(&address, &to).expect("the address shown was not the one written to");
    assert_eq!(dav.list("/dav/").unwrap().len(), 2);
    let mut got = Vec::new();
    assert_eq!(dav.read("/dav/a.txt", &mut got, &mut |_| true).unwrap(), 5);
    let to = leave::leave(Login::default(), &shown).expect("nothing in it");
    assert!(matches!(Dav::connect(&address, &to).map(drop), Err(Error::NeedsPassword)));
}

#[test]
fn a_webdav_folder_is_listed_read_written_and_renamed() {
    for digest in [false, true] {
        let (port, files) = server(digest);
        assert!(matches!(open(port, None), Err(Error::NeedsPassword)));
        assert!(matches!(open(port, Some("wrong")), Err(Error::LoginRefused)));
        let mut dav = open(port, Some("secret")).unwrap();
        assert_eq!(dav.home().unwrap(), "/dav/");
        let mut names: Vec<(String, Kind)> = dav.list("/dav/").unwrap().into_iter().map(|e| (e.name, e.kind)).collect();
        names.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(names, [("a.txt".to_string(), Kind::File), ("sub".to_string(), Kind::Folder)]);
        let mut got = Vec::new();
        assert_eq!(dav.read("/dav/a.txt", &mut got, &mut |_| true).unwrap(), 5);
        assert_eq!(got, b"hello");
        let words = b"made by Folders";
        dav.write_new("/dav/sub/new note.txt", &mut &words[..], words.len() as u64, &mut |_| true).unwrap();
        assert_eq!(files.lock().unwrap().get("/dav/sub/new note.txt").map(Vec::as_slice), Some(&words[..]));
        assert_eq!(dav.stat("/dav/sub/new note.txt").unwrap().size, words.len() as u64);
        dav.rename("/dav/a.txt", "/dav/b.txt").unwrap();
        assert!(matches!(dav.rename("/dav/b.txt", "/dav/sub/new note.txt"), Err(Error::Exists(_))));
        assert!(dav.refuses_taken());
        let got = dav.write_new("/dav/sub/new note.txt", &mut &b"other"[..], 5, &mut |_| true);
        assert!(matches!(&got, Err(Error::Exists(_))), "{got:?}");
        assert_eq!(files.lock().unwrap().get("/dav/sub/new note.txt").map(Vec::as_slice), Some(&words[..]), "a file that was there was written over");
        dav.write_new("/dav/sub/fresh.txt", &mut &b"other"[..], 5, &mut |_| true).unwrap();
        assert_eq!(files.lock().unwrap().get("/dav/sub/fresh.txt").map(Vec::as_slice), Some(&b"other"[..]));
        dav.mkdir("/dav/made").unwrap();
        dav.remove("/dav/b.txt").unwrap();
        assert!(matches!(dav.stat("/dav/b.txt"), Err(Error::NotFound(_))));
        assert!(files.lock().unwrap().contains_key("/dav/made/"));
    }
}

#[test]
fn a_webdav_file_with_no_length_is_never_taken_as_whole() {
    let (port, _files) = server(false);
    let mut dav = open(port, Some("secret")).unwrap();
    let words = b"the whole of it";
    dav.write_new("/dav/unsized.txt", &mut &words[..], words.len() as u64, &mut |_| true).unwrap();
    let mut got = Vec::new();
    assert!(dav.read("/dav/unsized.txt", &mut got, &mut |_| true).is_err(), "a cut-short file was taken as whole: {got:?}");
}

#[test]
fn a_webdav_folder_with_anything_in_it_is_never_removed() {
    let (port, files) = server(false);
    let mut dav = open(port, Some("secret")).unwrap();
    let words = b"still wanted";
    dav.write_new("/dav/sub/kept.txt", &mut &words[..], words.len() as u64, &mut |_| true).unwrap();
    let got = dav.rmdir("/dav/sub");
    assert!(files.lock().unwrap().contains_key("/dav/sub/kept.txt"), "a folder was removed with a file still in it");
    assert!(matches!(&got, Err(Error::Failed(why)) if why == "sub is not empty."), "{got:?}");
    dav.remove("/dav/sub/kept.txt").unwrap();
    dav.rmdir("/dav/sub").unwrap();
    assert!(!files.lock().unwrap().contains_key("/dav/sub/"));
    dav.mkdir("/dav/sub").unwrap();
    dav.write_new("/dav/sub/kept.txt", &mut &words[..], words.len() as u64, &mut |_| true).unwrap();
    assert!(dav.remove("/dav/sub").is_err());
    assert!(files.lock().unwrap().contains_key("/dav/sub/kept.txt"), "a folder was removed as if it were a file");
}

#[test]
fn a_webdav_folder_holding_a_thing_that_cannot_be_read_is_never_removed() {
    let (port, files) = server(false);
    let mut dav = open(port, Some("secret")).unwrap();
    files.lock().unwrap().insert("/dav/sub/forbidden.txt".into(), b"another person's".to_vec());
    let got = dav.rmdir("/dav/sub");
    assert!(files.lock().unwrap().contains_key("/dav/sub/forbidden.txt"), "a folder was removed with a thing in it that could not be read");
    assert!(matches!(&got, Err(Error::Failed(why)) if why == "sub is not empty."), "{got:?}");
    let listed: Vec<(String, Kind)> = dav.list("/dav/sub").unwrap().into_iter().map(|e| (e.name, e.kind)).collect();
    assert_eq!(listed, [("forbidden.txt".to_string(), Kind::Other)]);
    assert_eq!(dav.stat("/dav/sub/forbidden.txt").map(|e| e.kind).ok(), Some(Kind::Other));
    files.lock().unwrap().insert("/dav/top/forbidden/".into(), Vec::new());
    files.lock().unwrap().insert("/dav/top/".into(), Vec::new());
    assert!(dav.rmdir("/dav/top").is_err() && files.lock().unwrap().contains_key("/dav/top/forbidden/"));
}

#[test]
fn what_changes_on_a_webdav_server_in_the_moment_before_a_removal_is_not_removed() {
    let (port, files) = server(false);
    let mut dav = open(port, Some("secret")).unwrap();
    let held = |name: &str| files.lock().unwrap().get(name).cloned();
    files.lock().unwrap().insert("/dav/busy/".into(), Vec::new());
    let got = dav.rmdir("/dav/busy");
    assert_eq!(held("/dav/busy/late.txt").as_deref(), Some(&b"put there meanwhile"[..]), "what was put in the folder after it was looked at went with it");
    assert!(matches!(&got, Err(Error::Failed(why)) if why == "busy (not removed because it had just changed)"), "{got:?}");
    files.lock().unwrap().insert("/dav/busy.txt".into(), b"the one that was moved".to_vec());
    let got = dav.remove_if("/dav/busy.txt", 22, None);
    assert_eq!(held("/dav/busy.txt").as_deref(), Some(&b"saved again meanwhile"[..]), "a file saved again after it was looked at was removed");
    assert!(matches!(got, Ok(false)), "{got:?}");
    assert!(matches!(dav.remove_if("/dav/a.txt", 4, None), Ok(false)) && held("/dav/a.txt").is_some());
    assert!(matches!(dav.remove_if("/dav/a.txt", 5, None), Ok(true)) && held("/dav/a.txt").is_none());
    for name in ["/dav/untagged.txt", "/dav/weak.txt"] {
        files.lock().unwrap().insert(name.into(), b"plain".to_vec());
        assert!(matches!(dav.remove_if(name, 5, None), Ok(true)) && held(name).is_none(), "{name}");
    }
    dav.rmdir("/dav/sub").unwrap();
    assert!(held("/dav/sub/").is_none());
    assert!(matches!(dav.remove_if("/dav/a.txt", 5, None), Err(Error::NotFound(_))));
}
