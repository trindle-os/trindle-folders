// Finds computers that announce shared folders by multicast DNS.

use std::collections::BTreeMap;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};
use std::os::fd::FromRawFd;
use std::time::{Duration, Instant};

use super::neighbours::Computer;

const GROUP4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const GROUP6: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0xfb);
const PORT: u16 = 5353;
const SERVICE: &str = "_smb._tcp.local";
pub(super) const SMB_PORT: u16 = 445;
const A: u16 = 1;
const PTR: u16 = 12;
const AAAA: u16 = 28;
const SRV: u16 = 33;
const UNICAST: u16 = 0x8000;
const IN: u16 = 1;
const PACKET_MOST: usize = 9000;
const RECORDS_MOST: usize = 256;
pub(super) const COMPUTERS_MOST: usize = 64;
const PACKETS_AT_ONCE: usize = 64;
const JUMPS_MOST: usize = 16;
const ASK_AGAIN: Duration = Duration::from_millis(1000);
pub const LISTEN: Duration = Duration::from_millis(3000);
const NAP: Duration = Duration::from_millis(40);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Asked {
    Yes,
    NoCard,
    Refused(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Record {
    Ptr { service: String, instance: String },
    Srv { instance: String, host: String, port: u16 },
    At { host: String, at: IpAddr },
}

fn put_name(out: &mut Vec<u8>, name: &str) {
    for label in name.split('.').filter(|l| !l.is_empty()) {
        let bytes = &label.as_bytes()[..label.len().min(63)];
        out.push(bytes.len() as u8);
        out.extend_from_slice(bytes);
    }
    out.push(0);
}

pub fn question(asks: &[(String, u16)], unicast: bool) -> Vec<u8> {
    let mut out = vec![0, 0, 0, 0];
    out.extend_from_slice(&(asks.len() as u16).to_be_bytes());
    out.extend_from_slice(&[0; 6]);
    for (name, kind) in asks {
        put_name(&mut out, name);
        out.extend_from_slice(&kind.to_be_bytes());
        out.extend_from_slice(&(IN | if unicast { UNICAST } else { 0 }).to_be_bytes());
    }
    out
}

fn name_at(packet: &[u8], at: usize) -> Option<(String, usize)> {
    let (mut i, mut after, mut jumps, mut name) = (at, None, 0, String::new());
    loop {
        let len = *packet.get(i)? as usize;
        match len & 0xc0 {
            0xc0 => {
                let to = ((len & 0x3f) << 8) | *packet.get(i + 1)? as usize;
                jumps += 1;
                if to >= i || jumps > JUMPS_MOST {
                    return None;
                }
                after.get_or_insert(i + 2);
                i = to;
            }
            0 if len == 0 => return Some((name, after.unwrap_or(i + 1))),
            0 => {
                let label = packet.get(i + 1..i + 1 + len)?;
                if !name.is_empty() {
                    name.push('.');
                }
                name.push_str(&String::from_utf8_lossy(label));
                if name.len() > 255 {
                    return None;
                }
                i += 1 + len;
            }
            _ => return None,
        }
    }
}

fn u16_at(packet: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*packet.get(at)?, *packet.get(at + 1)?]))
}

pub fn records(packet: &[u8]) -> Vec<Record> {
    let mut out = Vec::new();
    let _ = read_into(packet, &mut out);
    out
}

fn read_into(packet: &[u8], out: &mut Vec<Record>) -> Option<()> {
    if packet.len() > PACKET_MOST || u16_at(packet, 2)? & 0x8000 == 0 {
        return None;
    }
    let questions = u16_at(packet, 4)? as usize;
    let count = u16_at(packet, 6)? as usize + u16_at(packet, 8)? as usize + u16_at(packet, 10)? as usize;
    let mut i = 12;
    for _ in 0..questions {
        i = name_at(packet, i)?.1 + 4;
    }
    for _ in 0..count.min(RECORDS_MOST) {
        let (name, next) = name_at(packet, i)?;
        let kind = u16_at(packet, next)?;
        let len = u16_at(packet, next + 8)? as usize;
        let data = next + 10;
        let body = packet.get(data..data + len)?;
        match kind {
            PTR => out.push(Record::Ptr { service: name, instance: name_at(packet, data)?.0 }),
            SRV if len > 6 => out.push(Record::Srv { instance: name, port: u16_at(packet, data + 4)?, host: name_at(packet, data + 6)?.0 }),
            A if len == 4 => out.push(Record::At { host: name, at: IpAddr::V4(Ipv4Addr::new(body[0], body[1], body[2], body[3])) }),
            AAAA if len == 16 => {
                let mut b = [0u8; 16];
                b.copy_from_slice(body);
                out.push(Record::At { host: name, at: IpAddr::V6(Ipv6Addr::from(b)) });
            }
            _ => {}
        }
        i = data + len;
    }
    Some(())
}

pub(super) fn plain_host(host: &str) -> Option<&str> {
    let host = host.strip_suffix('.').unwrap_or(host);
    let (label, rest) = host.split_once('.').unwrap_or((host, ""));
    let fine = !label.is_empty() && label.len() <= 63 && !label.starts_with('-') && label.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-');
    (fine && (rest.is_empty() || rest.eq_ignore_ascii_case("local"))).then_some(label)
}

#[derive(Default)]
pub struct Gathered {
    instances: BTreeMap<String, Option<(String, u16)>>,
    hosts: BTreeMap<String, Vec<IpAddr>>,
}

impl Gathered {
    pub fn take(&mut self, packet: &[u8]) {
        for r in records(packet) {
            match r {
                Record::Ptr { service, instance } if service.eq_ignore_ascii_case(SERVICE) => {
                    if self.instances.len() < COMPUTERS_MOST {
                        self.instances.entry(instance.to_lowercase()).or_insert(None);
                    }
                }
                Record::Srv { instance, host, port } => {
                    if let Some(served) = self.instances.get_mut(&instance.to_lowercase()) {
                        *served = Some((host, port));
                    }
                }
                Record::At { host, at } => {
                    if self.hosts.len() < 4 * COMPUTERS_MOST || self.hosts.contains_key(&host.to_lowercase()) {
                        let known = self.hosts.entry(host.to_lowercase()).or_default();
                        if !known.contains(&at) && known.len() < 8 {
                            known.push(at);
                        }
                    }
                }
                Record::Ptr { .. } => {}
            }
        }
    }

    pub fn asks(&self) -> Vec<(String, u16)> {
        let mut out = vec![(SERVICE.to_string(), PTR)];
        for (instance, served) in &self.instances {
            match served {
                None => out.push((instance.clone(), SRV)),
                Some((host, _)) if !self.hosts.contains_key(&host.to_lowercase()) => {
                    out.push((host.clone(), A));
                    out.push((host.clone(), AAAA));
                }
                Some(_) => {}
            }
        }
        out.truncate(24);
        out
    }

    pub fn computers(&self) -> Vec<Computer> {
        let mut out: Vec<Computer> = Vec::new();
        for (host, port) in self.instances.values().flatten() {
            let Some(label) = plain_host(host).filter(|_| *port == SMB_PORT) else { continue };
            let all = self.hosts.get(&host.to_lowercase()).map(Vec::as_slice).unwrap_or_default();
            let usable = |a: &&IpAddr| crate::common::smb::valid_address(&a.to_string()).is_some();
            let address = all.iter().find(|a| a.is_ipv4() && usable(a)).or_else(|| all.iter().find(usable)).map(|a| a.to_string());
            let name = match address {
                Some(_) => label.to_string(),
                None => host.strip_suffix('.').unwrap_or(host).to_string(),
            };
            if !out.iter().any(|c| c.name.eq_ignore_ascii_case(&name)) {
                out.push(Computer { name, address });
            }
        }
        out.sort_by_key(|c| c.name.to_lowercase());
        out
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Card {
    index: u32,
    v4: Option<Ipv4Addr>,
    v6: bool,
}

fn askable(flags: u32) -> bool {
    let has = |f: i32| flags & f as u32 != 0;
    has(libc::IFF_UP) && has(libc::IFF_MULTICAST) && !has(libc::IFF_LOOPBACK) && !has(libc::IFF_POINTOPOINT)
}

fn cards() -> Vec<Card> {
    let mut by_name: BTreeMap<String, Card> = BTreeMap::new();
    let mut head: *mut libc::ifaddrs = std::ptr::null_mut();
    if unsafe { libc::getifaddrs(&mut head) } != 0 {
        return Vec::new();
    }
    let mut p = head;
    while !p.is_null() {
        let ifa = unsafe { &*p };
        p = ifa.ifa_next;
        if ifa.ifa_addr.is_null() || ifa.ifa_name.is_null() || !askable(ifa.ifa_flags) {
            continue;
        }
        let name = unsafe { std::ffi::CStr::from_ptr(ifa.ifa_name) };
        let index = unsafe { libc::if_nametoindex(name.as_ptr()) };
        if index == 0 {
            continue;
        }
        let card = by_name.entry(name.to_string_lossy().into_owned()).or_insert(Card { index, ..Card::default() });
        match i32::from(unsafe { (*ifa.ifa_addr).sa_family }) {
            libc::AF_INET => {
                let sin = unsafe { &*(ifa.ifa_addr as *const libc::sockaddr_in) };
                card.v4.get_or_insert(Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr)));
            }
            libc::AF_INET6 => card.v6 = true,
            _ => {}
        }
    }
    unsafe { libc::freeifaddrs(head) };
    by_name.into_values().filter(|c| c.v4.is_some() || c.v6).collect()
}

fn setopt<T>(fd: i32, level: i32, name: i32, value: &T) -> io::Result<()> {
    let r = unsafe { libc::setsockopt(fd, level, name, (value as *const T).cast(), std::mem::size_of::<T>() as libc::socklen_t) };
    if r == 0 { Ok(()) } else { Err(io::Error::last_os_error()) }
}

fn datagram(family: i32) -> io::Result<UdpSocket> {
    let fd = unsafe { libc::socket(family, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { UdpSocket::from_raw_fd(fd) })
}

fn bind_to<T>(sock: &UdpSocket, at: &T) -> io::Result<()> {
    use std::os::fd::AsRawFd;
    let r = unsafe { libc::bind(sock.as_raw_fd(), (at as *const T).cast(), std::mem::size_of::<T>() as libc::socklen_t) };
    if r == 0 { Ok(()) } else { Err(io::Error::last_os_error()) }
}

fn socket4(card: &Card) -> io::Result<(UdpSocket, bool)> {
    use std::os::fd::AsRawFd;
    let sock = datagram(libc::AF_INET)?;
    let fd = sock.as_raw_fd();
    let on = libc::ip_mreqn { imr_multiaddr: libc::in_addr { s_addr: u32::from(GROUP4).to_be() }, imr_address: libc::in_addr { s_addr: 0 },
                              imr_ifindex: card.index as i32 };
    setopt(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, &1i32)?;
    setopt(fd, libc::IPPROTO_IP, libc::IP_MULTICAST_IF, &on)?;
    setopt(fd, libc::IPPROTO_IP, libc::IP_MULTICAST_TTL, &255i32)?;
    let at = |port: u16| libc::sockaddr_in { sin_family: libc::AF_INET as libc::sa_family_t, sin_port: port.to_be(),
                                             sin_addr: libc::in_addr { s_addr: 0 }, sin_zero: [0; 8] };
    let shared = setopt(fd, libc::IPPROTO_IP, libc::IP_MULTICAST_ALL, &0i32).and_then(|()| bind_to(&sock, &at(PORT)))
        .and_then(|()| setopt(fd, libc::IPPROTO_IP, libc::IP_ADD_MEMBERSHIP, &on)).is_ok();
    if !shared {
        let sock = datagram(libc::AF_INET)?;
        setopt(sock.as_raw_fd(), libc::IPPROTO_IP, libc::IP_MULTICAST_IF, &on)?;
        setopt(sock.as_raw_fd(), libc::IPPROTO_IP, libc::IP_MULTICAST_TTL, &255i32)?;
        bind_to(&sock, &at(0))?;
        return Ok((sock, false));
    }
    Ok((sock, true))
}

fn socket6(card: &Card) -> io::Result<(UdpSocket, bool)> {
    use std::os::fd::AsRawFd;
    let make = || -> io::Result<UdpSocket> {
        let sock = datagram(libc::AF_INET6)?;
        let fd = sock.as_raw_fd();
        setopt(fd, libc::IPPROTO_IPV6, libc::IPV6_V6ONLY, &1i32)?;
        setopt(fd, libc::IPPROTO_IPV6, libc::IPV6_MULTICAST_IF, &card.index)?;
        setopt(fd, libc::IPPROTO_IPV6, libc::IPV6_MULTICAST_HOPS, &255i32)?;
        Ok(sock)
    };
    let mut at: libc::sockaddr_in6 = unsafe { std::mem::zeroed() };
    at.sin6_family = libc::AF_INET6 as libc::sa_family_t;
    let sock = make()?;
    let fd = sock.as_raw_fd();
    let join = libc::ipv6_mreq { ipv6mr_multiaddr: libc::in6_addr { s6_addr: GROUP6.octets() }, ipv6mr_interface: card.index };
    at.sin6_port = PORT.to_be();
    let shared = setopt(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, &1i32).and_then(|()| bind_to(&sock, &at))
        .and_then(|()| setopt(fd, libc::IPPROTO_IPV6, libc::IPV6_ADD_MEMBERSHIP, &join)).is_ok();
    if !shared {
        let sock = make()?;
        at.sin6_port = 0;
        bind_to(&sock, &at)?;
        return Ok((sock, false));
    }
    Ok((sock, true))
}

fn drain(sock: &UdpSocket, got: &mut Gathered, buf: &mut [u8], until: Instant) {
    for _ in 0..PACKETS_AT_ONCE {
        if Instant::now() >= until {
            return;
        }
        match sock.recv_from(buf) {
            Ok((n, _)) => got.take(&buf[..n]),
            Err(_) => return,
        }
    }
}

struct Line {
    sock: UdpSocket,
    to: SocketAddr,
    shared: bool,
}

pub fn look(mut tell: impl FnMut(Vec<Computer>)) -> Asked {
    let cards = cards();
    if cards.is_empty() {
        return Asked::NoCard;
    }
    let (mut lines, mut why) = (Vec::new(), String::new());
    for card in &cards {
        let four = card.v4.map(|_| (socket4(card), SocketAddr::V4(SocketAddrV4::new(GROUP4, PORT))));
        let six = card.v6.then(|| (socket6(card), SocketAddr::V6(SocketAddrV6::new(GROUP6, PORT, 0, card.index))));
        for (made, to) in four.into_iter().chain(six) {
            match made.and_then(|(sock, shared)| sock.set_nonblocking(true).map(|()| (sock, shared))) {
                Ok((sock, shared)) => lines.push(Line { sock, to, shared }),
                Err(e) => why = e.to_string(),
            }
        }
    }
    let mut got = Gathered::default();
    let send = |lines: &mut Vec<Line>, got: &Gathered| -> Option<String> {
        let asks = got.asks();
        let mut why = None;
        lines.retain(|l| match l.sock.send_to(&question(&asks, !l.shared), l.to) {
            Ok(_) => true,
            Err(e) => {
                why = Some(e.to_string());
                false
            }
        });
        why
    };
    if let Some(e) = send(&mut lines, &got) {
        why = e;
    }
    if lines.is_empty() {
        return Asked::Refused(why);
    }
    let (start, mut again, mut buf) = (Instant::now(), false, vec![0u8; PACKET_MOST]);
    while start.elapsed() < LISTEN {
        for line in &lines {
            drain(&line.sock, &mut got, &mut buf, start + LISTEN);
        }
        if !again && start.elapsed() >= ASK_AGAIN {
            again = true;
            send(&mut lines, &got);
            let found = got.computers();
            if !found.is_empty() {
                tell(found);
            }
        }
        std::thread::sleep(NAP);
    }
    tell(got.computers());
    Asked::Yes
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub fn answer(records: &[(&str, u16, Vec<u8>)]) -> Vec<u8> {
        let mut out = vec![0, 0, 0x84, 0, 0, 0];
        out.extend_from_slice(&(records.len() as u16).to_be_bytes());
        out.extend_from_slice(&[0; 4]);
        for (name, kind, data) in records {
            put_name(&mut out, name);
            out.extend_from_slice(&kind.to_be_bytes());
            out.extend_from_slice(&[0x80, 1, 0, 0, 0, 120]);
            out.extend_from_slice(&(data.len() as u16).to_be_bytes());
            out.extend_from_slice(data);
        }
        out
    }

    pub fn name(n: &str) -> Vec<u8> {
        let mut out = Vec::new();
        put_name(&mut out, n);
        out
    }

    pub fn srv(host: &str, port: u16) -> Vec<u8> {
        let mut out = vec![0, 0, 0, 0];
        out.extend_from_slice(&port.to_be_bytes());
        put_name(&mut out, host);
        out
    }

    pub fn nas() -> Vec<u8> {
        answer(&[("_smb._tcp.local", PTR, name("NAS._smb._tcp.local")), ("NAS._smb._tcp.local", SRV, srv("NAS.local", 445)),
                 ("NAS.local", A, vec![192, 168, 1, 20])])
    }

    fn found(packets: &[Vec<u8>]) -> Vec<(String, Option<String>)> {
        let mut got = Gathered::default();
        for p in packets {
            got.take(p);
        }
        got.computers().into_iter().map(|c| (c.name, c.address)).collect()
    }

    #[test]
    fn a_nas_that_answers_is_a_computer_with_its_address() {
        assert_eq!(found(&[nas()]), [("NAS".to_string(), Some("192.168.1.20".to_string()))]);
        let parts = [answer(&[("_smb._tcp.local", PTR, name("NAS._smb._tcp.local"))]), answer(&[("NAS._smb._tcp.local", SRV, srv("NAS.local", 445))]),
                     answer(&[("nas.LOCAL", A, vec![192, 168, 1, 20])]), nas()];
        assert_eq!(found(&parts), [("NAS".to_string(), Some("192.168.1.20".to_string()))]);
        let others = answer(&[("_airplay._tcp.local", PTR, name("Lounge._airplay._tcp.local")), ("Lounge._airplay._tcp.local", SRV, srv("lounge.local", 7000)),
                              ("lounge.local", A, vec![192, 168, 1, 30]), ("_sftp-ssh._tcp.local", PTR, name("box._sftp-ssh._tcp.local"))]);
        assert!(found(&[others]).is_empty());
        let more = answer(&[("_smb._tcp.local", PTR, name("John Smith's laptop._smb._tcp.local")),
                            ("John Smith's laptop._smb._tcp.local", SRV, srv("johns-laptop.local", 445)), ("johns-laptop.local", A, vec![192, 168, 1, 7])]);
        assert_eq!(found(&[nas(), more]).into_iter().map(|c| c.0).collect::<Vec<_>>(), ["johns-laptop", "NAS"]);
    }

    #[test]
    fn addresses_are_chosen_for_use() {
        let with = |addresses: &[(u16, Vec<u8>)]| {
            let mut records = vec![("_smb._tcp.local", PTR, name("NAS._smb._tcp.local")), ("NAS._smb._tcp.local", SRV, srv("NAS.local", 445))];
            records.extend(addresses.iter().map(|(kind, data)| ("NAS.local", *kind, data.clone())));
            found(&[answer(&records)])
        };
        let v6 = |text: &str| text.parse::<Ipv6Addr>().unwrap().octets().to_vec();
        assert_eq!(with(&[(AAAA, v6("fd00::20")), (A, vec![10, 0, 0, 20])]), [("NAS".to_string(), Some("10.0.0.20".to_string()))]);
        assert_eq!(with(&[(AAAA, v6("fe80::1")), (AAAA, v6("fd00::20"))]), [("NAS".to_string(), Some("fd00::20".to_string()))]);
        assert_eq!(with(&[(AAAA, v6("fe80::1"))]), [("NAS.local".to_string(), None)]);
        assert_eq!(with(&[]), [("NAS.local".to_string(), None)]);
        assert_eq!(with(&[(A, vec![0, 0, 0, 0]), (A, vec![224, 0, 0, 251]), (A, vec![10, 0, 0, 20])]), [("NAS".to_string(), Some("10.0.0.20".to_string()))]);
    }

    #[test]
    fn what_is_still_missing_is_asked_for_next() {
        let mut got = Gathered::default();
        assert_eq!(got.asks(), [("_smb._tcp.local".to_string(), PTR)]);
        got.take(&answer(&[("_smb._tcp.local", PTR, name("NAS._smb._tcp.local"))]));
        assert_eq!(got.asks()[1..], [("nas._smb._tcp.local".to_string(), SRV)]);
        got.take(&answer(&[("NAS._smb._tcp.local", SRV, srv("NAS.local", 445))]));
        assert_eq!(got.asks()[1..], [("NAS.local".to_string(), A), ("NAS.local".to_string(), AAAA)]);
        got.take(&answer(&[("NAS.local", A, vec![192, 168, 1, 20])]));
        assert_eq!(got.asks().len(), 1);
        let q = question(&got.asks(), false);
        assert_eq!(q[..12], [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(q[12..], [&name("_smb._tcp.local")[..], &[0, 12, 0, 1]].concat());
        assert_eq!(question(&got.asks(), true)[q.len() - 2..], [0x80, 1]);
        assert!(records(&q).is_empty(), "a question is not an answer");
    }

    #[test]
    fn names_that_point_back_are_followed() {
        let mut p = vec![0, 0, 0x84, 0, 0, 0, 0, 3, 0, 0, 0, 0];
        let service = p.len() as u8;
        p.extend_from_slice(&name("_smb._tcp.local"));
        let local = service + 10;
        p.extend_from_slice(&[0, 12, 0, 1, 0, 0, 0, 120, 0, 6, 3, b'N', b'A', b'S', 0xc0, service]);
        let instance = p.len() as u8 - 6;
        p.extend_from_slice(&[0xc0, instance, 0, 33, 0, 1, 0, 0, 0, 120, 0, 12, 0, 0, 0, 0, 1, 189, 3, b'N', b'A', b'S', 0xc0, local]);
        let host = p.len() as u8 - 6;
        p.extend_from_slice(&[0xc0, host, 0, 1, 0, 1, 0, 0, 0, 120, 0, 4, 192, 168, 1, 20]);
        assert_eq!(records(&p), [Record::Ptr { service: "_smb._tcp.local".into(), instance: "NAS._smb._tcp.local".into() },
                                 Record::Srv { instance: "NAS._smb._tcp.local".into(), host: "NAS.local".into(), port: 445 },
                                 Record::At { host: "NAS.local".into(), at: "192.168.1.20".parse().unwrap() }]);
        assert_eq!(found(&[p]), [("NAS".to_string(), Some("192.168.1.20".to_string()))]);
    }

    #[test]
    fn a_hostile_answer_does_no_harm() {
        for bad in [vec![0xc0, 12], vec![0xc0, 40], vec![0xc0, 14, 0xc0, 12]] {
            let mut p = vec![0, 0, 0x84, 0, 0, 0, 0, 1, 0, 0, 0, 0];
            p.extend_from_slice(&bad);
            p.extend_from_slice(&[0, 12, 0, 1, 0, 0, 0, 120, 0, 2, 0xc0, 12]);
            assert!(records(&p).is_empty(), "{bad:?}");
        }
        let whole = nas();
        for cut in 0..whole.len() {
            let got = records(&whole[..cut]);
            assert!(got.len() <= 3, "cut at {cut}");
        }
        let mut long = answer(&[("_smb._tcp.local", PTR, name("NAS._smb._tcp.local"))]);
        let at = long.len() - name("NAS._smb._tcp.local").len() - 2;
        long[at] = 0xff;
        assert!(records(&long).is_empty());
        let mut many = nas();
        many[6..8].copy_from_slice(&[0xff, 0xff]);
        assert_eq!(records(&many).len(), 3);
        assert!(records(&vec![0xff; PACKET_MOST + 1]).is_empty() && records(&[]).is_empty());
        for (host, port) in [("../etc.local", 445), ("-rf.local", 445), ("a b.local", 445), ("nas.example.com", 445), ("nas;reboot.local", 445),
                             ("", 445), ("NAS.local", 4450), ("NAS.local", 0)] {
            let p = answer(&[("_smb._tcp.local", PTR, name("X._smb._tcp.local")), ("X._smb._tcp.local", SRV, srv(host, port)),
                             (host, A, vec![192, 168, 1, 9])]);
            assert!(found(&[p]).is_empty(), "{host:?} on {port}");
        }
        let mut got = Gathered::default();
        for n in 0..500 {
            let (instance, host) = (format!("pc{n}._smb._tcp.local"), format!("pc{n}.local"));
            got.take(&answer(&[("_smb._tcp.local", PTR, name(&instance)), (&instance, SRV, srv(&host, 445)), (&host, A, vec![10, 0, (n / 250) as u8, (n % 250) as u8])]));
        }
        assert_eq!(got.computers().len(), COMPUTERS_MOST);
        assert!(got.asks().len() <= 24);
    }

    #[test]
    fn a_flood_of_answers_cannot_keep_the_listening_going() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
        sock.set_nonblocking(true).unwrap();
        let to = sock.local_addr().unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let flood = std::thread::spawn(move || {
            let from = UdpSocket::bind("127.0.0.1:0").unwrap();
            let (packet, began) = (nas(), Instant::now());
            while !flag.load(Ordering::SeqCst) && began.elapsed() < Duration::from_secs(4) {
                let _ = from.send_to(&packet, to);
            }
        });
        std::thread::sleep(Duration::from_millis(100));
        let (mut got, mut buf, began) = (Gathered::default(), vec![0u8; PACKET_MOST], Instant::now());
        drain(&sock, &mut got, &mut buf, began + Duration::from_millis(150));
        let first = began.elapsed();
        assert_eq!(got.computers().len(), 1);
        let again = Instant::now();
        while again.elapsed() < Duration::from_millis(150) {
            drain(&sock, &mut got, &mut buf, again + Duration::from_millis(150));
        }
        let second = again.elapsed();
        stop.store(true, Ordering::SeqCst);
        flood.join().unwrap();
        assert!(first < Duration::from_secs(1), "one turn at a flooded card took {first:?}");
        assert!(second < Duration::from_secs(1), "listening ran on for {second:?} past its time");
        let mut none = Gathered::default();
        drain(&sock, &mut none, &mut buf, Instant::now());
        assert!(none.computers().is_empty());
    }

    #[test]
    #[ignore = "needs a real network with other computers on it: run by hand"]
    fn look_on_this_network() {
        let asked = look(|found| println!("answered so far: {found:?}"));
        println!("asked: {asked:?}; cards: {:?}", cards());
    }

    #[test]
    fn only_cards_that_can_be_asked_are() {
        let (up, group, lo, tunnel) = (libc::IFF_UP as u32, libc::IFF_MULTICAST as u32, libc::IFF_LOOPBACK as u32, libc::IFF_POINTOPOINT as u32);
        assert!(askable(up | group), "a cable or Wi-Fi");
        assert!(!askable(group), "one that is down");
        assert!(!askable(up), "one that cannot send to a group");
        assert!(!askable(up | group | lo), "this computer's own loop");
        assert!(!askable(up | group | tunnel), "a VPN's or Tailscale's tunnel");
        assert_eq!(plain_host("NAS.local"), Some("NAS"));
        assert_eq!(plain_host("nas.local."), Some("nas"));
        assert_eq!(plain_host("nas"), Some("nas"));
    }
}
