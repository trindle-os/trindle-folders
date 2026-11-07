// USB the Linux way, with no libusb.

use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use super::Error;
use crate::toolkit::t;

pub const SYSFS: &str = "/sys/bus/usb/devices";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    pub port: String,
    pub serial: String,
    pub maker: String,
    pub product: String,
    pub node: PathBuf,
    pub interface: u8,
    pub alt: u8,
    pub ep_in: u8,
    pub ep_out: u8,
    pub max_packet: usize,
}

fn read(dir: &Path, name: &str) -> String {
    std::fs::read_to_string(dir.join(name)).map(|s| s.trim().to_string()).unwrap_or_default()
}

fn hex(dir: &Path, name: &str) -> Option<u32> {
    u32::from_str_radix(&read(dir, name), 16).ok()
}

pub fn find(root: &Path) -> Vec<Found> {
    let Ok(dir) = std::fs::read_dir(root) else { return Vec::new() };
    let mut names: Vec<String> = dir.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    let mut out: Vec<Found> = Vec::new();
    for name in names {
        let Some((port, _)) = name.split_once(':') else { continue };
        let iface = root.join(&name);
        let class = hex(&iface, "bInterfaceClass");
        let sub = hex(&iface, "bInterfaceSubClass");
        let label = read(&iface, "interface");
        if !((class == Some(6) && sub == Some(1)) || label.to_ascii_uppercase().contains("MTP")) {
            continue;
        }
        if out.iter().any(|f| f.port == port) {
            continue;
        }
        let (mut ep_in, mut ep_out, mut max_packet) = (None, None, 512);
        let Ok(eps) = std::fs::read_dir(&iface) else { continue };
        for ep in eps.flatten() {
            if !ep.file_name().to_string_lossy().starts_with("ep_") {
                continue;
            }
            let ep = ep.path();
            if !read(&ep, "type").eq_ignore_ascii_case("bulk") {
                continue;
            }
            let Some(addr) = hex(&ep, "bEndpointAddress") else { continue };
            if addr & 0x80 != 0 {
                ep_in = Some(addr as u8);
                max_packet = hex(&ep, "wMaxPacketSize").map(|m| (m & 0x7ff) as usize).filter(|&m| m > 0).unwrap_or(512);
            } else {
                ep_out = Some(addr as u8);
            }
        }
        let (Some(ep_in), Some(ep_out)) = (ep_in, ep_out) else { continue };
        let dev = root.join(port);
        let (Ok(bus), Ok(num)) = (read(&dev, "busnum").parse::<u32>(), read(&dev, "devnum").parse::<u32>()) else {
            continue;
        };
        out.push(Found {
            port: port.to_string(),
            serial: read(&dev, "serial"),
            maker: read(&dev, "manufacturer"),
            product: read(&dev, "product"),
            node: PathBuf::from(format!("/dev/bus/usb/{bus:03}/{num:03}")),
            interface: hex(&iface, "bInterfaceNumber").unwrap_or(0) as u8,
            alt: read(&iface, "bAlternateSetting").parse().unwrap_or(0),
            ep_in,
            ep_out,
            max_packet,
        });
    }
    out
}

pub trait Bulk: Send {
    fn send(&mut self, data: &[u8]) -> Result<(), Error>;
    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
    fn max_packet(&self) -> usize;
    fn cancel(&mut self, _id: u32) {}
}

const CONTROL: u64 = 0xC018_5500;
const BULK: u64 = 0xC018_5502;
const SETINTERFACE: u64 = 0x8008_5504;
const CLAIMINTERFACE: u64 = 0x8004_550F;
const RELEASEINTERFACE: u64 = 0x8004_5510;
const CLEAR_HALT: u64 = 0x8004_5515;

#[repr(C)]
struct BulkTransfer {
    ep: u32,
    len: u32,
    timeout: u32,
    data: *mut libc::c_void,
}

#[repr(C)]
struct CtrlTransfer {
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    length: u16,
    timeout: u32,
    data: *mut libc::c_void,
}

#[repr(C)]
struct SetInterface {
    interface: u32,
    alt: u32,
}

const WAIT_MS: u32 = 30_000;

pub struct Usb {
    file: File,
    interface: u32,
    ep_in: u8,
    ep_out: u8,
    max_packet: usize,
}

impl Usb {
    pub fn open(found: &Found) -> Result<Usb, Error> {
        let node = &found.node;
        let file = OpenOptions::new().read(true).write(true).open(node).map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => {
                Error::Denied(t!("folders-remote-usb-denied", node = node.display().to_string()))
            }
            io::ErrorKind::NotFound => Error::Unreachable(t!("folders-remote-usb-unplugged").into()),
            _ => Error::Io(e),
        })?;
        let usb = Usb {
            file,
            interface: found.interface as u32,
            ep_in: found.ep_in,
            ep_out: found.ep_out,
            max_packet: found.max_packet,
        };
        let mut n = usb.interface;
        if let Err(e) = unsafe { usb.ioctl(CLAIMINTERFACE, &mut n as *mut u32 as *mut libc::c_void) } {
            return Err(match e.raw_os_error() {
                Some(libc::EBUSY) => Error::Busy(t!("folders-remote-usb-busy").into()),
                Some(libc::EACCES) | Some(libc::EPERM) => {
                    Error::Denied(t!("folders-remote-usb-denied", node = node.display().to_string()))
                }
                _ => Error::Io(e),
            });
        }
        if found.alt != 0 {
            let mut s = SetInterface { interface: usb.interface, alt: found.alt as u32 };
            unsafe { usb.ioctl(SETINTERFACE, &mut s as *mut SetInterface as *mut libc::c_void) }?;
        }
        Ok(usb)
    }

    unsafe fn ioctl(&self, request: u64, arg: *mut libc::c_void) -> io::Result<libc::c_int> {
        loop {
            let r = unsafe { libc::ioctl(self.file.as_raw_fd(), request as libc::Ioctl, arg) };
            if r >= 0 {
                return Ok(r);
            }
            let e = io::Error::last_os_error();
            if e.kind() != io::ErrorKind::Interrupted {
                return Err(e);
            }
        }
    }

    unsafe fn bulk(&self, ep: u8, data: *mut u8, len: usize) -> Result<usize, Error> {
        let mut t = BulkTransfer { ep: ep as u32, len: len as u32, timeout: WAIT_MS, data: data as *mut libc::c_void };
        unsafe { self.ioctl(BULK, &mut t as *mut BulkTransfer as *mut libc::c_void) }
            .map(|n| n as usize)
            .map_err(|e| self.failed(e))
    }

    fn failed(&self, e: io::Error) -> Error {
        match e.raw_os_error() {
            Some(libc::ETIMEDOUT) => Error::Lost(t!("folders-remote-usb-stopped").into()),
            Some(libc::ENODEV) | Some(libc::ESHUTDOWN) => Error::Unreachable(t!("folders-remote-usb-unplugged").into()),
            Some(libc::EPIPE) => Error::Lost(t!("folders-remote-usb-refused").into()),
            _ => Error::Io(e),
        }
    }

    fn clear_halt(&self, ep: u8) {
        let mut ep = ep as u32;
        let _ = unsafe { self.ioctl(CLEAR_HALT, &mut ep as *mut u32 as *mut libc::c_void) };
    }
}

impl Bulk for Usb {
    fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        let n = unsafe { self.bulk(self.ep_out, data.as_ptr() as *mut u8, data.len()) }?;
        if n == data.len() {
            Ok(())
        } else {
            Err(Error::Lost(t!("folders-remote-usb-part").into()))
        }
    }

    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        unsafe { self.bulk(self.ep_in, buf.as_mut_ptr(), buf.len()) }
    }

    fn max_packet(&self) -> usize {
        self.max_packet
    }

    fn cancel(&mut self, id: u32) {
        let mut data = [0u8; 6];
        data[..2].copy_from_slice(&0x4001u16.to_le_bytes());
        data[2..].copy_from_slice(&id.to_le_bytes());
        let mut c = CtrlTransfer {
            request_type: 0x21,
            request: 0x64,
            value: 0,
            index: self.interface as u16,
            length: data.len() as u16,
            timeout: 5_000,
            data: data.as_mut_ptr() as *mut libc::c_void,
        };
        let _ = unsafe { self.ioctl(CONTROL, &mut c as *mut CtrlTransfer as *mut libc::c_void) };
        for _ in 0..30 {
            let mut status = [0u8; 20];
            let mut c = CtrlTransfer {
                request_type: 0xA1,
                request: 0x67,
                value: 0,
                index: self.interface as u16,
                length: status.len() as u16,
                timeout: 1_000,
                data: status.as_mut_ptr() as *mut libc::c_void,
            };
            let got = unsafe { self.ioctl(CONTROL, &mut c as *mut CtrlTransfer as *mut libc::c_void) };
            match got {
                Ok(n) if n >= 4 && u16::from_le_bytes([status[2], status[3]]) != 0x2019 => break,
                Err(_) => break,
                _ => std::thread::sleep(std::time::Duration::from_millis(100)),
            }
        }
        self.clear_halt(self.ep_in);
        self.clear_halt(self.ep_out);
    }
}

impl Drop for Usb {
    fn drop(&mut self) {
        let mut n = self.interface;
        let _ = unsafe { self.ioctl(RELEASEINTERFACE, &mut n as *mut u32 as *mut libc::c_void) };
    }
}

pub struct Plugged(std::os::fd::OwnedFd);

impl Plugged {
    pub fn open() -> Option<Plugged> {
        use std::os::fd::FromRawFd;
        let fd = unsafe {
            libc::socket(libc::AF_NETLINK, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK, libc::NETLINK_KOBJECT_UEVENT)
        };
        if fd < 0 {
            return None;
        }
        let fd = unsafe { std::os::fd::OwnedFd::from_raw_fd(fd) };
        let mut addr: libc::sockaddr_nl = unsafe { std::mem::zeroed() };
        addr.nl_family = libc::AF_NETLINK as libc::sa_family_t;
        addr.nl_groups = 1;
        let bound = unsafe {
            libc::bind(fd.as_raw_fd(), &addr as *const libc::sockaddr_nl as *const libc::sockaddr,
                       std::mem::size_of::<libc::sockaddr_nl>() as libc::socklen_t)
        };
        (bound == 0).then_some(Plugged(fd))
    }

    pub fn fd(&self) -> std::os::fd::RawFd {
        self.0.as_raw_fd()
    }

    pub fn drain(&self) -> bool {
        let mut buf = [0u8; 8192];
        let mut any = false;
        loop {
            let n = unsafe { libc::recv(self.0.as_raw_fd(), buf.as_mut_ptr() as *mut libc::c_void, buf.len(), 0) };
            if n <= 0 {
                break;
            }
            any |= usb_change(&buf[..n as usize]);
        }
        any
    }
}

fn usb_change(msg: &[u8]) -> bool {
    let fields: Vec<&[u8]> = msg.split(|b| *b == 0).collect();
    let has = |f: &[u8]| fields.contains(&f);
    has(b"SUBSYSTEM=usb") && [&b"ACTION=add"[..], b"ACTION=remove", b"ACTION=bind", b"ACTION=unbind"].iter().any(|a| has(a))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn put(dir: &Path, name: &str, text: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(dir.join(name), format!("{text}\n")).unwrap();
    }

    fn endpoint(iface: &Path, addr: &str, kind: &str) {
        let ep = iface.join(format!("ep_{addr}"));
        put(&ep, "bEndpointAddress", addr);
        put(&ep, "type", kind);
        put(&ep, "wMaxPacketSize", "0200");
    }

    #[test]
    fn struct_sizes_match_the_kernel() {
        assert_eq!(std::mem::size_of::<BulkTransfer>(), 0x18);
        assert_eq!(std::mem::size_of::<CtrlTransfer>(), 0x18);
        assert_eq!(std::mem::size_of::<SetInterface>(), 8);
        assert_eq!((BULK >> 16) & 0x3fff, 0x18);
        assert_eq!((CONTROL >> 16) & 0x3fff, 0x18);
    }

    #[test]
    fn finds_phones_in_a_made_up_tree() {
        let root = std::env::temp_dir().join(format!("trindle-usb-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let dev = root.join("1-2");
        put(&dev, "busnum", "1");
        put(&dev, "devnum", "7");
        put(&dev, "serial", "ABC123");
        put(&dev, "manufacturer", "Example");
        put(&dev, "product", "Example Phone");
        let i = root.join("1-2:1.0");
        put(&i, "bInterfaceClass", "ff");
        put(&i, "bInterfaceSubClass", "ff");
        put(&i, "bInterfaceNumber", "00");
        put(&i, "bAlternateSetting", " 0");
        put(&i, "interface", "MTP");
        endpoint(&i, "81", "Bulk");
        endpoint(&i, "01", "Bulk");
        endpoint(&i, "82", "Interrupt");
        let dev = root.join("3-1.4");
        put(&dev, "busnum", "3");
        put(&dev, "devnum", "12");
        let i = root.join("3-1.4:1.0");
        put(&i, "bInterfaceClass", "06");
        put(&i, "bInterfaceSubClass", "01");
        put(&i, "bInterfaceNumber", "00");
        endpoint(&i, "83", "Bulk");
        endpoint(&i, "02", "Bulk");
        let i = root.join("1-3:1.0");
        put(&i, "bInterfaceClass", "03");
        put(&i, "bInterfaceSubClass", "01");
        let found = find(&root);
        fs::remove_dir_all(&root).unwrap();
        assert_eq!(found.len(), 2);
        let p = &found[0];
        assert_eq!((p.port.as_str(), p.serial.as_str(), p.product.as_str()), ("1-2", "ABC123", "Example Phone"));
        assert_eq!(p.node, PathBuf::from("/dev/bus/usb/001/007"));
        assert_eq!((p.ep_in, p.ep_out, p.max_packet, p.interface), (0x81, 0x01, 512, 0));
        let c = &found[1];
        assert_eq!((c.port.as_str(), c.serial.as_str()), ("3-1.4", ""));
        assert_eq!(c.node, PathBuf::from("/dev/bus/usb/003/012"));
        assert_eq!((c.ep_in, c.ep_out), (0x83, 0x02));
        assert!(find(Path::new("/nowhere/at/all")).is_empty());
        assert!(usb_change(b"add@/devices/x/usb1/1-2\0ACTION=add\0DEVPATH=/devices/x/usb1/1-2\0SUBSYSTEM=usb\0DEVTYPE=usb_device\0"));
        assert!(usb_change(b"remove@/x\0ACTION=remove\0SUBSYSTEM=usb\0"));
        assert!(!usb_change(b"change@/x\0ACTION=change\0SUBSYSTEM=usb\0"));
        assert!(!usb_change(b"add@/x\0ACTION=add\0SUBSYSTEM=block\0"));
    }
}
