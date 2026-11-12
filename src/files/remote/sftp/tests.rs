//! Tests of the SFTP link: its packets, its replies and a made-up server.

use super::*;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

#[test]
fn four_files_at_once_have_no_more_in_flight_than_two() {
    assert_eq!((share_of(0), share_of(1), share_of(2)), (WINDOW, WINDOW, WINDOW));
    assert_eq!((share_of(3), share_of(4)), (21, 16));
    assert!((1..=4).all(|n| n * share_of(n) <= IN_FLIGHT));
    assert_eq!(share_of(100), WINDOW_LEAST);
}

#[test]
fn a_lost_connection_is_its_own_kind_and_a_refusal_is_not() {
    for kind in [io::ErrorKind::UnexpectedEof, io::ErrorKind::BrokenPipe] {
        assert!(matches!(lost(kind.into()), Error::Lost(_)), "{kind:?}");
    }
    assert!(matches!(garbled(), Error::Lost(_)));
    assert!(matches!(status(4, "no", "it"), Error::Failed(_)));
}

#[test]
fn packets() {
    let p = Out::new(OPEN, Some(7)).str("/a").u32(OPEN_READ).attrs(&Attrs::default()).done();
    assert_eq!(p, [0, 0, 0, 19, 3, 0, 0, 0, 7, 0, 0, 0, 2, b'/', b'a', 0, 0, 0, 1, 0, 0, 0, 0]);
    let a = Attrs { size: Some(5), owner: Some((1, 2)), mode: Some(0o100644), times: Some((3, 4)) };
    let p = Out::new(ATTRS, Some(9)).attrs(&a).done();
    assert_eq!(decode(&p[4..]).unwrap(), (9, Reply::Attrs(a.clone())));
    assert_eq!(a.entry("x".into()).kind, Kind::File);
    assert_eq!(a.entry("x".into()).mode, Some(0o644));
}

#[test]
fn replies() {
    let s = [101, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0, 4, b'g', b'o', b'n', b'e', 0, 0, 0, 0];
    assert_eq!(decode(&s).unwrap(), (3, Reply::Status(2, "gone".into())));
    assert_eq!(decode(&[101, 0, 0, 0, 4, 0, 0, 0, 1]).unwrap(), (4, Reply::Status(1, String::new())));
    let mut n = vec![104, 0, 0, 0, 5, 0, 0, 0, 1, 0, 0, 0, 1, b'd', 0, 0, 0, 2, b'l', b'l'];
    n.extend([0x80, 0, 0, 0x04, 0, 0, 0x41, 0xED, 0, 0, 0, 1, 0, 0, 0, 1, b'k', 0, 0, 0, 1, b'v']);
    let (id, r) = decode(&n).unwrap();
    assert_eq!(id, 5);
    let Reply::Name(names) = r else { panic!() };
    assert_eq!(names[0].0, "d");
    assert_eq!(names[0].1.kind(), Kind::Folder);
    assert!(decode(&[103, 0, 0, 0, 1, 0, 0, 0, 9, 1]).is_err());
}

#[test]
fn pieces_out_of_order_and_short() {
    let mut p = Pieces::default();
    let a = p.next().unwrap();
    let b = p.next().unwrap();
    let c = p.next().unwrap();
    assert_eq!((a.0, b.0, c.0), (0, CHUNK as u64, 2 * CHUNK as u64));
    p.came(b.0, b.1, Some(vec![2; CHUNK as usize])).unwrap();
    p.came(c.0, c.1, None).unwrap();
    assert!(p.ready().is_empty());
    p.came(a.0, a.1, Some(vec![1; 100])).unwrap();
    assert_eq!(p.ready().len(), 1);
    let again = p.next().unwrap();
    assert_eq!(again, (100, CHUNK - 100));
    p.came(again.0, again.1, Some(vec![1; (CHUNK - 100) as usize])).unwrap();
    assert_eq!(p.ready().len(), 2);
    assert_eq!(p.written(), 2 * CHUNK as u64);
    assert_eq!(p.next(), None);
    let mut p = Pieces::default();
    let a = p.next().unwrap();
    assert!(p.came(a.0, a.1, Some(vec![1; a.1 as usize + 1])).is_err());
}

#[test]
fn ssh_uses_the_normal_set_up() {
    let a = Address::parse("sftp://john@example.com:2200/x").unwrap();
    let args = ssh_args(&a, &Login::default()).unwrap();
    let joined = args.join(" ");
    for want in ["ClearAllForwardings=yes", "BatchMode=yes", "StrictHostKeyChecking=yes", "-p 2200", "-l john", "-s -- example.com sftp"] {
        assert!(joined.contains(want), "{want} missing from {joined}");
    }
    for not in ["-F none", "IdentityFile=none", "IdentityAgent=none", "UserKnownHostsFile", "IdentitiesOnly"] {
        assert!(!joined.contains(not), "{not} in {joined}");
    }
    let k = Login { key: Some("/no/such/key".into()), ..Login::default() };
    assert!(matches!(ssh_args(&a, &k), Err(Error::Key(_))));
    if std::env::var_os("XDG_RUNTIME_DIR").is_some_and(|d| Path::new(&d).is_dir()) {
        assert!(joined.contains("ControlMaster=auto") && joined.contains("trindle-ssh-%C"), "{joined}");
    }
    let asks = Login { askpass: Some("/x/trindle-askpass".into()), ..Login::default() };
    assert!(!ssh_args(&a, &asks).unwrap().join(" ").contains("BatchMode"));
}

#[test]
fn reasons() {
    let l = Login::default();
    let r = |s| why(s, "h", &l).unwrap();
    assert!(matches!(r("john@h: Permission denied (publickey,password).\r\n"), Error::NeedsPassword));
    assert!(matches!(r("No ED25519 host key is known for h and you have requested strict checking.\r\nHost key verification failed.\r\n"), Error::HostKeyUnknown(_)));
    assert!(matches!(r("@ WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED! @\nHost key verification failed.\n"), Error::HostKeyChanged(_)));
    assert!(matches!(r("ssh: Could not resolve hostname h: Name or service not known\n"), Error::Unreachable(_)));
    assert_eq!(r("subsystem request failed on channel 0\r\n").to_string(), "This server doesn't offer SFTP.");
    assert!(why("", "h", &l).is_none());
}

type Open = (Option<std::fs::File>, Option<Vec<(String, Attrs)>>);

struct Folder {
    root: PathBuf,
    handles: HashMap<Vec<u8>, Open>,
    next: u32,
}

fn attrs_of(m: &std::fs::Metadata) -> Attrs {
    use std::os::unix::fs::MetadataExt;
    Attrs {
        size: Some(m.len()),
        owner: Some((m.uid(), m.gid())),
        mode: Some(m.mode()),
        times: Some((m.atime() as u32, m.mtime() as u32)),
    }
}

fn status_of(id: u32, r: io::Result<()>) -> Vec<u8> {
    let (code, msg) = match r {
        Ok(()) => (FX_OK, ""),
        Err(e) if e.kind() == io::ErrorKind::NotFound => (FX_NO_SUCH_FILE, "No such file"),
        Err(_) => (4, "Failure"),
    };
    Out::new(STATUS, Some(id)).u32(code).str(msg).str("").done()
}

fn serve(mut s: UnixStream, root: PathBuf) {
    let mut folder = Folder { root, handles: HashMap::new(), next: 0 };
    loop {
        let mut len = [0u8; 4];
        if s.read_exact(&mut len).is_err() {
            return;
        }
        let mut body = vec![0; u32::from_be_bytes(len) as usize];
        s.read_exact(&mut body).unwrap();
        let mut i = In::new(&body);
        let kind = i.u8().unwrap();
        if kind == INIT {
            s.write_all(&Out::new(VERSION_REPLY, None).u32(3).done()).unwrap();
            continue;
        }
        let id = i.u32().unwrap();
        let reply = folder.reply(kind, id, &mut i);
        s.write_all(&reply).unwrap();
    }
}

impl Folder {
    fn at(&self, p: &str) -> PathBuf {
        self.root.join(p.trim_start_matches('/'))
    }

    fn reply(&mut self, kind: u8, id: u32, i: &mut In) -> Vec<u8> {
        use std::fs;
        match kind {
            REALPATH => Out::new(NAME, Some(id)).u32(1).str("/home").str("").attrs(&Attrs::default()).done(),
            OPEN | OPENDIR => self.open(kind, id, i),
            READ => self.read(id, i),
            WRITE => self.write(id, i),
            READDIR => self.readdir(id, i),
            CLOSE => {
                self.handles.remove(i.bytes().unwrap());
                status_of(id, Ok(()))
            }
            LSTAT | STAT => match if kind == STAT { fs::metadata(self.at(&i.string().unwrap())) } else { fs::symlink_metadata(self.at(&i.string().unwrap())) } {
                Ok(m) => Out::new(ATTRS, Some(id)).attrs(&attrs_of(&m)).done(),
                Err(e) => status_of(id, Err(e)),
            },
            MKDIR => status_of(id, fs::create_dir(self.at(&i.string().unwrap()))),
            RMDIR => status_of(id, fs::remove_dir(self.at(&i.string().unwrap()))),
            REMOVE => status_of(id, fs::remove_file(self.at(&i.string().unwrap()))),
            RENAME => {
                let (a, b) = (i.string().unwrap(), i.string().unwrap());
                status_of(id, fs::rename(self.at(&a), self.at(&b)))
            }
            READLINK => match fs::read_link(self.at(&i.string().unwrap())) {
                Ok(t) => Out::new(NAME, Some(id)).u32(1).str(&t.to_string_lossy()).str("").attrs(&Attrs::default()).done(),
                Err(e) => status_of(id, Err(e)),
            },
            _ => Out::new(STATUS, Some(id)).u32(FX_OP_UNSUPPORTED).str("").str("").done(),
        }
    }

    fn open(&mut self, kind: u8, id: u32, i: &mut In) -> Vec<u8> {
        use std::fs;
        let path = i.string().unwrap();
        let opened = if kind == OPEN {
            let flags = i.u32().unwrap();
            let f = if flags & OPEN_EXCL != 0 {
                fs::File::options().write(true).create_new(true).open(self.at(&path))
            } else if flags & OPEN_WRITE != 0 {
                fs::File::create(self.at(&path))
            } else {
                fs::File::open(self.at(&path))
            };
            f.map(|f| (Some(f), None))
        } else {
            fs::read_dir(self.at(&path)).map(|d| {
                let list = d.flatten().map(|e| (e.file_name().to_string_lossy().into_owned(), attrs_of(&e.metadata().unwrap()))).collect();
                (None, Some(list))
            })
        };
        match opened {
            Ok(h) => {
                self.next += 1;
                let key = self.next.to_be_bytes().to_vec();
                self.handles.insert(key.clone(), h);
                Out::new(HANDLE, Some(id)).bytes(&key).done()
            }
            Err(e) => status_of(id, Err(e)),
        }
    }

    fn read(&mut self, id: u32, i: &mut In) -> Vec<u8> {
        let h = i.bytes().unwrap().to_vec();
        let (off, len) = (i.u64().unwrap(), i.u32().unwrap());
        let f = self.handles.get_mut(&h).unwrap().0.as_mut().unwrap();
        let mut buf = vec![0; len.min(1000) as usize];
        let n = std::os::unix::fs::FileExt::read_at(f, &mut buf, off).unwrap();
        if n == 0 {
            Out::new(STATUS, Some(id)).u32(FX_EOF).str("EOF").str("").done()
        } else {
            Out::new(DATA, Some(id)).bytes(&buf[..n]).done()
        }
    }

    fn write(&mut self, id: u32, i: &mut In) -> Vec<u8> {
        let h = i.bytes().unwrap().to_vec();
        let off = i.u64().unwrap();
        let data = i.bytes().unwrap();
        let f = self.handles.get_mut(&h).unwrap().0.as_mut().unwrap();
        status_of(id, std::os::unix::fs::FileExt::write_all_at(f, data, off))
    }

    fn readdir(&mut self, id: u32, i: &mut In) -> Vec<u8> {
        let h = i.bytes().unwrap().to_vec();
        let held = self.handles.get_mut(&h).unwrap();
        if held.1.as_ref().is_some_and(|l| l.iter().any(|(n, _)| n == "endless")) {
            let list = held.1.clone().unwrap();
            let mut o = Out::new(NAME, Some(id));
            o.u32(list.len() as u32);
            for (n, a) in &list {
                o.str(n).str(n).attrs(a);
            }
            return o.done();
        }
        match held.1.take() {
            Some(list) => {
                let mut o = Out::new(NAME, Some(id));
                o.u32(list.len() as u32);
                for (n, a) in &list {
                    o.str(n).str(n).attrs(a);
                }
                o.done()
            }
            None => Out::new(STATUS, Some(id)).u32(FX_EOF).str("").str("").done(),
        }
    }
}

#[test]
fn round_trip() {
    let root = std::env::temp_dir().join(format!("trindle-sftp-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let (ours, theirs) = UnixStream::pair().unwrap();
    let served = root.clone();
    let server = std::thread::spawn(move || serve(theirs, served));
    let mut s = Sftp::over(ours.try_clone().unwrap(), ours).unwrap();
    assert_eq!(s.home().unwrap(), "/home");

    let data: Vec<u8> = (0..300_000u32).map(|i| (i % 251) as u8).collect();
    let mut seen = 0;
    let n = s
        .write_new("/big.bin", &mut &data[..], data.len() as u64, &mut |d| {
            seen = d;
            true
        })
        .unwrap();
    assert_eq!((n, seen), (data.len() as u64, data.len() as u64));
    assert_eq!(std::fs::read(root.join("big.bin")).unwrap(), data);

    let mut back = Vec::new();
    assert_eq!(s.read("/big.bin", &mut back, &mut |_| true).unwrap(), data.len() as u64);
    assert!(back == data);

    std::fs::create_dir_all(root.join("loop")).unwrap();
    std::fs::write(root.join("loop/endless"), b"").unwrap();
    assert!(matches!(s.list("/loop"), Err(Error::Lost(_))));
    std::fs::remove_dir_all(root.join("loop")).unwrap();

    assert!(s.refuses_taken());
    let got = s.write_new("/big.bin", &mut &b"other"[..], 5, &mut |_| true);
    assert!(matches!(&got, Err(Error::Exists(name)) if name == "big.bin"), "{got:?}");
    assert_eq!(std::fs::read(root.join("big.bin")).unwrap(), data, "a file that was there was written over");
    assert_eq!(s.write_new("/new.bin", &mut &b"other"[..], 5, &mut |_| true).unwrap(), 5);
    assert_eq!(std::fs::read(root.join("new.bin")).unwrap(), b"other");
    s.remove("/new.bin").unwrap();

    let mut cut = Vec::new();
    assert!(matches!(s.read("/big.bin", &mut cut, &mut |d| d < 5000), Err(Error::Cancelled)));

    s.mkdir("/sub").unwrap();
    s.rename("/big.bin", "/sub/moved.bin").unwrap();
    std::os::unix::fs::symlink("moved.bin", root.join("sub/link")).unwrap();
    let mut list = s.list("/sub").unwrap();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    assert_eq!(list.len(), 2);
    assert_eq!((list[0].name.as_str(), list[0].kind), ("link", Kind::Link));
    assert_eq!((list[1].name.as_str(), list[1].size), ("moved.bin", data.len() as u64));
    assert_eq!(s.readlink("/sub/link").unwrap(), "moved.bin");
    let led = s.follow("/sub/link").unwrap();
    assert_eq!((led.name.as_str(), led.kind, led.size), ("link", Kind::File, data.len() as u64));
    assert!(s.stat("/sub").unwrap().is_folder());
    assert!(matches!(s.stat("/nothing"), Err(Error::NotFound(_))));
    s.remove("/sub/link").unwrap();
    s.remove("/sub/moved.bin").unwrap();
    s.rmdir("/sub").unwrap();
    assert!(s.list("/").unwrap().is_empty());

    drop(s);
    server.join().unwrap();
    std::fs::remove_dir_all(&root).unwrap();
}
