// Tests of the phone link: its containers, names and a made-up phone.

use super::*;
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};

const INTERNAL: u32 = 0x0001_0001;
const CARD: u32 = 0x0002_0001;
const PACKET: usize = 512;

#[test]
fn containers_and_strings() {
    assert_eq!(
        container(COMMAND, OPEN_SESSION, 0, &[1]),
        [0x10, 0, 0, 0, 1, 0, 0x02, 0x10, 0, 0, 0, 0, 1, 0, 0, 0]
    );
    let mut p = Put::default();
    p.str("Hi").str("");
    assert_eq!(p.0, [3, b'H', 0, b'i', 0, 0, 0, 0]);
    let mut t = Take::new(&p.0);
    assert_eq!(t.str().unwrap(), "Hi");
    assert_eq!(t.str().unwrap(), "");
    assert!(t.u8().is_err());
    assert!(Take::new(&[9, 0, 0, 0, 1, 0, 0, 0]).array().is_err());
    assert_eq!(Take::new(&[2, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0]).array().unwrap(), [7, 8]);
}

#[test]
fn object_info() {
    let info = ObjectInfo {
        storage: INTERNAL,
        format: UNDEFINED,
        size: 1234,
        parent: 7,
        name: "Notes.txt".into(),
        modified: Some(1_791_030_605),
    };
    let b = info.encode();
    assert_eq!(&b[..4], &INTERNAL.to_le_bytes());
    assert_eq!(&b[8..12], &1234u32.to_le_bytes());
    assert_eq!(&b[38..42], &7u32.to_le_bytes());
    assert_eq!(b[52], 10);
    assert_eq!(ObjectInfo::decode(&b).unwrap(), info);
    let big = ObjectInfo { size: 5_000_000_000, ..info.clone() };
    assert_eq!(ObjectInfo::decode(&big.encode()).unwrap().size, u32::MAX as u64);
    let folder = ObjectInfo { format: ASSOCIATION, ..info };
    let b = folder.encode();
    assert_eq!(&b[42..44], &[1, 0]);
    assert!(ObjectInfo::decode(&b).unwrap().folder());
    assert!(ObjectInfo::decode(&b[..40]).is_err());
}

fn storage_info(description: &str, label: &str) -> Vec<u8> {
    let mut p = Put::default();
    p.u16(3).u16(2).u16(0).u32(0).u32(1).u32(0).u32(1).u32(100);
    p.str(description).str(label);
    p.0
}

#[test]
fn storage_names_and_dates() {
    assert_eq!(storage_name(&storage_info("Internal shared storage", "")).unwrap(), "Internal shared storage");
    assert_eq!(storage_name(&storage_info("", "SD/card")).unwrap(), "SD-card");
    assert_eq!(storage_name(&storage_info(" ", "")).unwrap(), "Storage");
    assert!(storage_name(&[1, 2]).is_err());
    assert_eq!(date(1_791_030_605), "20261003T123005");
    assert_eq!(undate("20261003T123005"), Some(1_791_030_605));
    assert_eq!(undate("20261003T123005.0Z"), Some(1_791_030_605));
    assert_eq!(undate(""), None);
    assert_eq!(undate("yesterday at noon"), None);
    assert!(matches!(refused(0x2009, "a"), Error::NotFound(_)));
    assert!(matches!(refused(0x200F, "a"), Error::Denied(_)));
    assert!(matches!(refused(0x2019, "a"), Error::Busy(_)));
    assert_eq!(refused(0x200C, "a").to_string(), "The phone is full.");
    assert_eq!(refused(0x2002, "a").to_string(), "The phone refused that (code 2002).");
}

#[test]
fn phone_names() {
    let f = Found {
        port: "1-2".into(),
        serial: "ABC123".into(),
        maker: "Google".into(),
        product: "Pixel 9".into(),
        node: std::path::PathBuf::from("/dev/bus/usb/001/007"),
        interface: 0,
        alt: 0,
        ep_in: 0x81,
        ep_out: 0x01,
        max_packet: PACKET,
    };
    assert_eq!(phone(&f), Phone { id: "ABC123".into(), name: "Google Pixel 9".into() });
    let g = Found { serial: "-oops".into(), maker: "Fairphone".into(), product: "Fairphone 5".into(), ..f.clone() };
    assert_eq!((phone(&g).id.as_str(), phone(&g).name.as_str()), ("1-2", "Fairphone 5"));
    let h = Found { serial: "a/b".into(), maker: String::new(), product: String::new(), ..f };
    assert_eq!((phone(&h).id.as_str(), phone(&h).name.as_str()), ("1-2", "Phone"));
}

#[derive(Default)]
struct State {
    storages: Vec<(u32, String)>,
    objects: BTreeMap<u32, (ObjectInfo, Vec<u8>)>,
    next: u32,
    out: VecDeque<Vec<u8>>,
    waiting: Option<(u16, u32, Vec<u32>)>,
    incoming: Vec<u8>,
    sent: Option<u32>,
    session: bool,
    glue: bool,
    ptp_only: bool,
    no_length: bool,
    cancels: u32,
    empties: u32,
    ragged: u32,
    padding: usize,
}

struct Fake(Arc<Mutex<State>>);

impl State {
    fn add(&mut self, storage: u32, parent: u32, name: &str, data: Option<Vec<u8>>) -> u32 {
        self.next += 1;
        let format = if data.is_some() { UNDEFINED } else { ASSOCIATION };
        let data = data.unwrap_or_default();
        let info = ObjectInfo { storage, format, size: data.len() as u64, parent, name: name.into(), modified: Some(1_791_030_605) };
        self.objects.insert(self.next, (info, data));
        self.next
    }

    fn queue(&mut self, code: u16, id: u32, data: Option<Vec<u8>>, params: &[u32]) {
        let mut transfers = Vec::new();
        let no_length = self.no_length && data.as_ref().is_some_and(|d| d.len() > 1000);
        if let Some(d) = data {
            let mut p = Put::default();
            let len = if no_length { u32::MAX } else { 12 + d.len() as u32 };
            p.u32(len).u16(DATA).u16(code).u32(id);
            p.0.extend_from_slice(&d);
            transfers.push(p.0);
        }
        let mut r = container(RESPONSE, code, id, params);
        if self.padding > 0 {
            r.resize(r.len() + self.padding, 0);
            let len = r.len() as u32;
            r[..4].copy_from_slice(&len.to_le_bytes());
        }
        if self.glue && !no_length && !transfers.is_empty() {
            transfers[0].extend_from_slice(&r);
        } else {
            transfers.push(r);
        }
        for t in transfers {
            let whole = t.len() % PACKET == 0;
            self.out.push_back(t);
            if whole {
                self.out.push_back(Vec::new());
            }
        }
    }

    fn command(&mut self, code: u16, id: u32, p: Vec<u32>) {
        let arg = |i: usize| p.get(i).copied().unwrap_or(0);
        match code {
            OPEN_SESSION => {
                assert_eq!(id, 0, "a session opens with transaction 0");
                if self.session {
                    return self.queue(SESSION_ALREADY_OPEN, id, None, &[]);
                }
                self.session = true;
                self.queue(OK, id, None, &[]);
            }
            CLOSE_SESSION => {
                self.session = false;
                self.queue(OK, id, None, &[]);
            }
            GET_STORAGE_IDS => self.storage_ids(id),
            GET_STORAGE_INFO => self.storage_info_of(id, arg(0)),
            GET_OBJECT_HANDLES => self.object_handles(id, arg(0), arg(2)),
            GET_OBJECT_INFO | GET_OBJECT => self.object_info_or_data(code, id, arg(0)),
            DELETE_OBJECT => {
                let code = if self.objects.remove(&arg(0)).is_some() { OK } else { 0x2009 };
                self.queue(code, id, None, &[]);
            }
            MOVE_OBJECT => {
                let into = if arg(2) == 0 { ROOT } else { arg(2) };
                let o = self.objects.get_mut(&arg(0)).unwrap();
                o.0.storage = arg(1);
                o.0.parent = into;
                self.queue(OK, id, None, &[]);
            }
            GET_OBJECT_PROP_VALUE if self.ptp_only => self.queue(0x2005, id, None, &[]),
            SEND_OBJECT_INFO | SEND_OBJECT | SET_OBJECT_PROP_VALUE => self.waiting = Some((code, id, p)),
            _ => self.queue(0x2005, id, None, &[]),
        }
    }

    fn storage_ids(&mut self, id: u32) {
        let mut d = Put::default();
        d.u32(self.storages.len() as u32 + 1);
        for (s, _) in &self.storages {
            d.u32(*s);
        }
        d.u32(0x0003_0000);
        self.queue(OK, id, Some(d.0), &[]);
    }

    fn storage_info_of(&mut self, id: u32, storage: u32) {
        let name = self.storages.iter().find(|(s, _)| *s == storage).map(|(_, n)| n.clone());
        match name {
            Some(n) => self.queue(OK, id, Some(storage_info(&n, "")), &[]),
            None => self.queue(0x2013, id, None, &[]),
        }
    }

    fn object_handles(&mut self, id: u32, storage: u32, parent: u32) {
        let found: Vec<u32> = self
            .objects
            .iter()
            .filter(|(_, (i, _))| i.storage == storage && i.parent == parent)
            .map(|(h, _)| *h)
            .collect();
        let mut d = Put::default();
        d.u32(found.len() as u32);
        for h in found {
            d.u32(h);
        }
        self.queue(OK, id, Some(d.0), &[]);
    }

    fn object_info_or_data(&mut self, code: u16, id: u32, handle: u32) {
        let got = self.objects.get(&handle).map(|(i, d)| if code == GET_OBJECT { d.clone() } else { i.encode() });
        match got {
            Some(d) => self.queue(OK, id, Some(d), &[]),
            None => self.queue(0x2009, id, None, &[]),
        }
    }

    fn data(&mut self, code: u16, id: u32, p: Vec<u32>, d: Vec<u8>) {
        match code {
            SEND_OBJECT_INFO => {
                let info = ObjectInfo::decode(&d).unwrap();
                assert_eq!((info.storage, info.parent), (p[0], p[1]));
                let data = if info.folder() { None } else { Some(Vec::new()) };
                let h = self.add(p[0], p[1], &info.name, data);
                self.sent = Some(h);
                self.queue(OK, id, None, &[p[0], p[1], h]);
            }
            SEND_OBJECT => {
                let h = self.sent.take().unwrap();
                let o = self.objects.get_mut(&h).unwrap();
                o.0.size = d.len() as u64;
                o.1 = d;
                self.queue(OK, id, None, &[]);
            }
            _ if self.ptp_only => self.queue(0x2005, id, None, &[]),
            _ => {
                assert_eq!(p[1], PROP_NAME as u32);
                let name = Take::new(&d).str().unwrap();
                self.objects.get_mut(&p[0]).unwrap().0.name = name;
                self.queue(OK, id, None, &[]);
            }
        }
    }
}

impl Bulk for Fake {
    fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        let mut s = self.0.lock().unwrap();
        if data.is_empty() {
            s.empties += 1;
            return Ok(());
        }
        if s.waiting.is_some() {
            s.incoming.extend_from_slice(data);
            let total = u32::from_le_bytes(s.incoming[..4].try_into().unwrap()) as usize;
            if s.incoming.len() < total {
                if data.len() % PACKET != 0 {
                    s.ragged += 1;
                }
                return Ok(());
            }
            let (code, id, p) = s.waiting.take().unwrap();
            let got = std::mem::take(&mut s.incoming);
            assert_eq!(u16::from_le_bytes([got[4], got[5]]), DATA);
            s.data(code, id, p, got[12..total].to_vec());
            return Ok(());
        }
        let mut t = Take::new(data);
        let (len, kind, code, id) = (t.u32()?, t.u16()?, t.u16()?, t.u32()?);
        assert_eq!((len as usize, kind), (data.len(), COMMAND));
        let p = (0..(data.len() - 12) / 4).map(|_| t.u32().unwrap()).collect();
        s.command(code, id, p);
        Ok(())
    }

    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut s = self.0.lock().unwrap();
        let mut t = s.out.pop_front().ok_or_else(|| Error::Failed("the made-up phone has nothing to say".into()))?;
        let n = t.len().min(buf.len());
        buf[..n].copy_from_slice(&t[..n]);
        if n < t.len() {
            s.out.push_front(t.split_off(n));
        }
        Ok(n)
    }

    fn max_packet(&self) -> usize {
        PACKET
    }

    fn cancel(&mut self, _id: u32) {
        let mut s = self.0.lock().unwrap();
        s.cancels += 1;
        s.out.clear();
        s.waiting = None;
        s.incoming.clear();
    }
}

fn pattern(n: usize) -> Vec<u8> {
    (0..n).map(|i| (i * 7 % 251) as u8).collect()
}

fn made_up(glue: bool, no_length: bool) -> (Arc<Mutex<State>>, Mtp) {
    let mut s = State { glue, no_length, session: true, ..State::default() };
    s.storages = vec![(INTERNAL, "Internal shared storage".into()), (CARD, "SD card".into())];
    let dcim = s.add(INTERNAL, ROOT, "DCIM", None);
    s.add(INTERNAL, dcim, "Holiday.jpg", Some(pattern(300_000)));
    s.add(INTERNAL, ROOT, "Notes.txt", Some(b"Ring John Smith".to_vec()));
    s.add(CARD, ROOT, "Music", None);
    let state = Arc::new(Mutex::new(s));
    let m = Mtp::over(Box::new(Fake(state.clone()))).unwrap();
    (state, m)
}

fn names(m: &mut Mtp, path: &str) -> Vec<String> {
    let mut n: Vec<String> = m.list(path).unwrap().into_iter().map(|e| e.name).collect();
    n.sort();
    n
}

fn fetch(m: &mut Mtp, path: &str) -> Vec<u8> {
    let mut got = Vec::new();
    m.read(path, &mut got, &mut |_| true).unwrap();
    got
}

fn walk(glue: bool, no_length: bool) {
    let (state, mut m) = made_up(glue, no_length);
    assert_eq!(m.home().unwrap(), "/");
    assert_eq!(names(&mut m, "/"), ["Internal shared storage", "SD card"]);
    assert_eq!(names(&mut m, "/Internal shared storage"), ["DCIM", "Notes.txt"]);
    let photo = m.stat("/Internal shared storage/DCIM/Holiday.jpg").unwrap();
    assert_eq!((photo.kind, photo.size, photo.modified), (Kind::File, 300_000, Some(1_791_030_605)));

    let mut seen = Vec::new();
    let mut got = Vec::new();
    let n = m.read("/Internal shared storage/DCIM/Holiday.jpg", &mut got, &mut |d| {
        seen.push(d);
        true
    });
    assert_eq!(n.unwrap(), 300_000);
    assert!(got == pattern(300_000));
    assert_eq!(seen.last(), Some(&300_000));
    assert_eq!(fetch(&mut m, "/Internal shared storage/Notes.txt"), b"Ring John Smith");

    let big = pattern(280_000);
    assert_eq!(m.write("/SD card/Music/Song.ogg", &mut &big[..], big.len() as u64, &mut |_| true).unwrap(), 280_000);
    assert!(fetch(&mut m, "/SD card/Music/Song.ogg") == big);
    let exact = pattern(PACKET * 3 - 12);
    let before = state.lock().unwrap().empties;
    m.write("/SD card/Music/Song.ogg", &mut &exact[..], exact.len() as u64, &mut |_| true).unwrap();
    assert_eq!(state.lock().unwrap().empties, before + 1);
    assert!(fetch(&mut m, "/SD card/Music/Song.ogg") == exact);
    assert_eq!(names(&mut m, "/SD card/Music"), ["Song.ogg"]);
    assert_eq!(state.lock().unwrap().ragged, 0);

    m.mkdir("/SD card/Podcasts").unwrap();
    assert!(matches!(m.mkdir("/SD card/Podcasts"), Err(Error::Exists(_))));
    assert!(m.mkdir("/New storage").is_err());
    assert!(m.stat("/SD card/Podcasts").unwrap().is_folder());

    m.rename("/Internal shared storage/Notes.txt", "/Internal shared storage/Shopping.txt").unwrap();
    m.rename("/Internal shared storage/Shopping.txt", "/SD card/Podcasts/List.txt").unwrap();
    assert_eq!(names(&mut m, "/Internal shared storage"), ["DCIM"]);
    assert_eq!(fetch(&mut m, "/SD card/Podcasts/List.txt"), b"Ring John Smith");
    assert!(matches!(m.rename("/SD card/Music", "/SD card/Podcasts"), Err(Error::Exists(_))));
    m.rename("/SD card/Music", "/Internal shared storage/Music").unwrap();
    assert_eq!(names(&mut m, "/Internal shared storage"), ["DCIM", "Music"]);

    assert!(m.rmdir("/SD card/Podcasts").is_err());
    assert!(m.remove("/SD card/Podcasts").is_err());
    m.remove("/SD card/Podcasts/List.txt").unwrap();
    m.rmdir("/SD card/Podcasts").unwrap();
    assert!(names(&mut m, "/SD card").is_empty());
    assert!(matches!(m.stat("/SD card/Podcasts"), Err(Error::NotFound(_))));
    assert!(matches!(m.stat("/Nowhere"), Err(Error::NotFound(_))));
    assert!(m.read("/Internal shared storage/DCIM", &mut Vec::new(), &mut |_| true).is_err());
    assert_eq!(state.lock().unwrap().cancels, 0);

    drop(m);
    assert!(!state.lock().unwrap().session, "the session is closed at the end");
}

#[test]
fn walk_a_made_up_phone() {
    walk(false, false);
}

#[test]
fn walk_a_phone_that_glues_its_replies() {
    walk(true, false);
}

#[test]
fn walk_a_phone_that_sends_no_lengths() {
    walk(false, true);
}

#[test]
fn what_a_phone_sends_without_end_is_stopped() {
    let (state, mut m) = made_up(false, false);
    let big = state.lock().unwrap().add(INTERNAL, ROOT, "Big.bin", Some(vec![0; 65 << 20]));
    let got = m.run(GET_OBJECT, &[big], None, "it");
    assert!(matches!(&got, Err(Error::Lost(why)) if why == t!("folders-remote-mod-too-much")), "{:?}", got.map(|(d, _)| d.len()));
    let (state, mut m) = made_up(false, false);
    state.lock().unwrap().padding = 8 << 20;
    let got = m.run(GET_OBJECT_INFO, &[1], None, "it");
    assert!(matches!(got, Err(Error::Lost(_))), "{:?}", got.map(|(d, _)| d.len()));
}

#[test]
fn cancelling() {
    for glue in [false, true] {
        let (state, mut m) = made_up(glue, false);
        let photo = "/Internal shared storage/DCIM/Holiday.jpg";
        let r = m.read(photo, &mut Vec::new(), &mut |done| done == 0);
        assert!(matches!(r, Err(Error::Cancelled)));
        assert_eq!(state.lock().unwrap().cancels, 1);
        assert_eq!(fetch(&mut m, photo).len(), 300_000);

        let big = pattern(600_000);
        let r = m.write("/SD card/Big.bin", &mut &big[..], big.len() as u64, &mut |done| done < 300_000);
        assert!(matches!(r, Err(Error::Cancelled)));
        assert_eq!(state.lock().unwrap().cancels, 2);
        assert_eq!(names(&mut m, "/SD card"), ["Music"]);

        let r = m.write("/SD card/Short.bin", &mut &b"abc"[..], 10, &mut |_| true);
        assert!(r.is_err());
        assert_eq!(names(&mut m, "/SD card"), ["Music"]);
    }
}

#[test]
fn a_camera_that_speaks_only_ptp() {
    let mut s = State { ptp_only: true, ..State::default() };
    s.storages = vec![(INTERNAL, "Memory card".into())];
    let dcim = s.add(INTERNAL, ROOT, "DCIM", None);
    s.add(INTERNAL, dcim, "IMG_0001.JPG", Some(pattern(5000)));
    let state = Arc::new(Mutex::new(s));
    let mut m = Mtp::over(Box::new(Fake(state))).unwrap();
    assert_eq!(names(&mut m, "/Memory card/DCIM"), ["IMG_0001.JPG"]);
    assert_eq!(fetch(&mut m, "/Memory card/DCIM/IMG_0001.JPG").len(), 5000);
    m.write("/Memory card/note.txt", &mut &b"hi"[..], 2, &mut |_| true).unwrap();
    m.rename("/Memory card/note.txt", "/Memory card/DCIM/note.txt").unwrap();
    m.remove("/Memory card/DCIM/note.txt").unwrap();
    let r = m.rename("/Memory card/DCIM/IMG_0001.JPG", "/Memory card/DCIM/Sea.jpg");
    assert!(matches!(r, Err(Error::NotSupported(_))), "{r:?}");
    assert_eq!(names(&mut m, "/Memory card/DCIM"), ["IMG_0001.JPG"]);
}

#[test]
fn a_locked_phone_shows_no_storage() {
    let state = Arc::new(Mutex::new(State::default()));
    let mut m = Mtp::over(Box::new(Fake(state))).unwrap();
    assert_eq!(m.list("/").unwrap_err().to_string(), no_storage());
}

#[test]
fn storages_with_the_same_name() {
    let s = State { storages: vec![(INTERNAL, "Card".into()), (CARD, "Card".into())], ..State::default() };
    let mut m = Mtp::over(Box::new(Fake(Arc::new(Mutex::new(s))))).unwrap();
    assert_eq!(names(&mut m, "/"), ["Card (00010001)", "Card (00020001)"]);
}

#[test]
#[ignore = "needs a real phone plugged in: run by hand"]
fn a_real_phone() {
    let Some(p) = phones().into_iter().next() else {
        eprintln!("No phone is plugged in.");
        return;
    };
    let mut m = Mtp::open(&p.id).unwrap();
    for s in m.list("/").unwrap() {
        eprintln!("{} / {}: {:?}", p.name, s.name, m.list(&format!("/{}", s.name)).map(|l| l.len()));
    }
}
