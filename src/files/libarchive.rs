//! The system's libarchive, used for archives Trindle does not read itself.

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::OnceLock;

const EOF: c_int = 1;
const RETRY: c_int = -10;
const WARN: c_int = -20;
const FORMAT_RAW: c_int = 0x90000;
const FORMAT_MASK: c_int = 0xff0000;
const IFMT: u32 = 0o170_000;
const IFREG: u32 = 0o100_000;
const IFDIR: u32 = 0o040_000;
const IFLNK: u32 = 0o120_000;
const NAME_MAX: usize = 1 << 16;

type Handle = *mut c_void;
type Setup = unsafe extern "C" fn(Handle) -> c_int;
type Text = unsafe extern "C" fn(Handle) -> *const c_char;

struct Lib {
    read_new: unsafe extern "C" fn() -> Handle,
    read_free: Setup,
    filter_all: Setup,
    formats: Vec<Setup>,
    open_names: unsafe extern "C" fn(Handle, *const *const c_char, usize) -> c_int,
    next_header: unsafe extern "C" fn(Handle, *mut Handle) -> c_int,
    read_data: unsafe extern "C" fn(Handle, *mut c_void, usize) -> isize,
    error_string: Text,
    format: Setup,
    filter_count: Setup,
    filter_bytes: unsafe extern "C" fn(Handle, c_int) -> i64,
    pathname: Text,
    pathname_utf8: Text,
    symlink: Text,
    hardlink: Text,
    filetype: unsafe extern "C" fn(Handle) -> u32,
    perm: unsafe extern "C" fn(Handle) -> u32,
    size: unsafe extern "C" fn(Handle) -> i64,
    size_is_set: Setup,
    mtime: unsafe extern "C" fn(Handle) -> i64,
    mtime_is_set: Setup,
    data_encrypted: Setup,
}

const FORMATS: [&CStr; 11] = [
    c"archive_read_support_format_rar",
    c"archive_read_support_format_rar5",
    c"archive_read_support_format_tar",
    c"archive_read_support_format_cpio",
    c"archive_read_support_format_iso9660",
    c"archive_read_support_format_cab",
    c"archive_read_support_format_ar",
    c"archive_read_support_format_lha",
    c"archive_read_support_format_xar",
    c"archive_read_support_format_zip",
    c"archive_read_support_format_7zip",
];
const FORMAT_RAW_NAME: &CStr = c"archive_read_support_format_raw";

fn lib() -> Option<&'static Lib> {
    static LIB: OnceLock<Option<Lib>> = OnceLock::new();
    LIB.get_or_init(load).as_ref()
}

#[cfg(test)]
pub fn there() -> bool {
    lib().is_some()
}

fn load() -> Option<Lib> {
    let so = unsafe { libc::dlopen(c"libarchive.so.13".as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
    if so.is_null() {
        return None;
    }
    let find = |name: &CStr| -> Option<*mut c_void> {
        let at = unsafe { libc::dlsym(so, name.as_ptr()) };
        (!at.is_null()).then_some(at)
    };
    unsafe {
        let mut formats: Vec<Setup> = Vec::new();
        for name in FORMATS {
            formats.push(std::mem::transmute::<*mut c_void, Setup>(find(name)?));
        }
        formats.push(std::mem::transmute::<*mut c_void, Setup>(find(FORMAT_RAW_NAME)?));
        Some(Lib {
            read_new: std::mem::transmute::<*mut c_void, unsafe extern "C" fn() -> Handle>(find(c"archive_read_new")?),
            read_free: std::mem::transmute::<*mut c_void, Setup>(find(c"archive_read_free")?),
            filter_all: std::mem::transmute::<*mut c_void, Setup>(find(c"archive_read_support_filter_all")?),
            formats,
            open_names: std::mem::transmute::<*mut c_void, unsafe extern "C" fn(Handle, *const *const c_char, usize) -> c_int>(find(c"archive_read_open_filenames")?),
            next_header: std::mem::transmute::<*mut c_void, unsafe extern "C" fn(Handle, *mut Handle) -> c_int>(find(c"archive_read_next_header")?),
            read_data: std::mem::transmute::<*mut c_void, unsafe extern "C" fn(Handle, *mut c_void, usize) -> isize>(find(c"archive_read_data")?),
            error_string: std::mem::transmute::<*mut c_void, Text>(find(c"archive_error_string")?),
            format: std::mem::transmute::<*mut c_void, Setup>(find(c"archive_format")?),
            filter_count: std::mem::transmute::<*mut c_void, Setup>(find(c"archive_filter_count")?),
            filter_bytes: std::mem::transmute::<*mut c_void, unsafe extern "C" fn(Handle, c_int) -> i64>(find(c"archive_filter_bytes")?),
            pathname: std::mem::transmute::<*mut c_void, Text>(find(c"archive_entry_pathname")?),
            pathname_utf8: std::mem::transmute::<*mut c_void, Text>(find(c"archive_entry_pathname_utf8")?),
            symlink: std::mem::transmute::<*mut c_void, Text>(find(c"archive_entry_symlink")?),
            hardlink: std::mem::transmute::<*mut c_void, Text>(find(c"archive_entry_hardlink")?),
            filetype: std::mem::transmute::<*mut c_void, unsafe extern "C" fn(Handle) -> u32>(find(c"archive_entry_filetype")?),
            perm: std::mem::transmute::<*mut c_void, unsafe extern "C" fn(Handle) -> u32>(find(c"archive_entry_perm")?),
            size: std::mem::transmute::<*mut c_void, unsafe extern "C" fn(Handle) -> i64>(find(c"archive_entry_size")?),
            size_is_set: std::mem::transmute::<*mut c_void, Setup>(find(c"archive_entry_size_is_set")?),
            mtime: std::mem::transmute::<*mut c_void, unsafe extern "C" fn(Handle) -> i64>(find(c"archive_entry_mtime")?),
            mtime_is_set: std::mem::transmute::<*mut c_void, Setup>(find(c"archive_entry_mtime_is_set")?),
            data_encrypted: std::mem::transmute::<*mut c_void, Setup>(find(c"archive_entry_is_data_encrypted")?),
        })
    }
}

pub fn speak_utf8() {
    unsafe {
        libc::setlocale(libc::LC_CTYPE, c"C.UTF-8".as_ptr());
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Fault {
    Missing,
    Locked,
    Unknown,
    Memory,
    Unsupported,
    Damaged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum What {
    File,
    Folder,
    Symlink,
    Hardlink,
    Other,
}

#[derive(Debug)]
pub struct Head {
    pub name: Vec<u8>,
    pub what: What,
    pub mode: Option<u32>,
    pub size: Option<u64>,
    pub modified: Option<i64>,
    pub link: Vec<u8>,
    pub locked: bool,
}

pub struct Reader {
    lib: &'static Lib,
    a: Handle,
}

impl Reader {
    pub fn open(parts: &[PathBuf]) -> Result<Reader, Fault> {
        let lib = lib().ok_or(Fault::Missing)?;
        let names: Vec<CString> = parts.iter().filter_map(|p| CString::new(p.as_os_str().as_bytes()).ok()).collect();
        if names.is_empty() || names.len() != parts.len() {
            return Err(Fault::Unknown);
        }
        let mut list: Vec<*const c_char> = names.iter().map(|n| n.as_ptr()).collect();
        list.push(std::ptr::null());
        let a = unsafe { (lib.read_new)() };
        if a.is_null() {
            return Err(Fault::Memory);
        }
        let reader = Reader { lib, a };
        unsafe {
            (lib.filter_all)(a);
            for format in &lib.formats {
                format(a);
            }
        }
        let r = unsafe { (lib.open_names)(a, list.as_ptr(), 1 << 16) };
        if r < WARN {
            return Err(reader.fault());
        }
        Ok(reader)
    }

    fn fault(&self) -> Fault {
        let said = unsafe {
            let s = (self.lib.error_string)(self.a);
            if s.is_null() { String::new() } else { CStr::from_ptr(s).to_string_lossy().into_owned() }
        };
        let low = said.to_lowercase();
        if low.contains("encrypt") || low.contains("passphrase") || low.contains("password") {
            Fault::Locked
        } else if low.contains("unrecognized archive format") {
            Fault::Unknown
        } else if low.contains("unsupported") {
            Fault::Unsupported
        } else if low.contains("memory") || low.contains("allocate") {
            Fault::Memory
        } else {
            Fault::Damaged
        }
    }

    pub fn next(&mut self) -> Result<Option<Head>, Fault> {
        Ok(self.header()?.map(|entry| self.head_of(entry)))
    }

    fn header(&mut self) -> Result<Option<Handle>, Fault> {
        let mut entry: Handle = std::ptr::null_mut();
        let mut tries = 0;
        let r = loop {
            let r = unsafe { (self.lib.next_header)(self.a, &mut entry) };
            tries += 1;
            if r != RETRY || tries > 8 {
                break r;
            }
        };
        if r == EOF {
            return Ok(None);
        }
        if r < WARN || entry.is_null() {
            return Err(self.fault());
        }
        if self.raw() && self.filters() <= 1 {
            return Err(Fault::Unknown);
        }
        Ok(Some(entry))
    }

    fn head_of(&self, entry: Handle) -> Head {
        let lib = self.lib;
        let text = |get: Text| -> Vec<u8> {
            unsafe {
                let s = get(entry);
                if s.is_null() { Vec::new() } else { CStr::from_ptr(s).to_bytes().iter().take(NAME_MAX).copied().collect() }
            }
        };
        let mut name = text(lib.pathname_utf8);
        if name.is_empty() {
            name = text(lib.pathname);
        }
        let hard = text(lib.hardlink);
        let (kind, perm, size, modified, locked) = unsafe {
            (
                (lib.filetype)(entry),
                (lib.perm)(entry),
                ((lib.size_is_set)(entry) != 0).then(|| (lib.size)(entry)),
                ((lib.mtime_is_set)(entry) != 0).then(|| (lib.mtime)(entry)),
                (lib.data_encrypted)(entry) == 1,
            )
        };
        let (what, link) = if !hard.is_empty() {
            (What::Hardlink, hard)
        } else {
            match kind & IFMT {
                IFDIR => (What::Folder, Vec::new()),
                IFLNK => (What::Symlink, text(lib.symlink)),
                IFREG | 0 => (What::File, Vec::new()),
                _ => (What::Other, Vec::new()),
            }
        };
        let raw = self.raw();
        Head {
            name,
            what,
            mode: (!raw).then_some(perm & 0o7777),
            size: size.and_then(|s| u64::try_from(s).ok()),
            modified,
            link,
            locked,
        }
    }

    pub fn data(&mut self, buf: &mut [u8]) -> Result<usize, Fault> {
        for _ in 0..=8 {
            let n = unsafe { (self.lib.read_data)(self.a, buf.as_mut_ptr().cast(), buf.len()) };
            if n >= 0 {
                return Ok(n as usize);
            }
            if n != RETRY as isize {
                break;
            }
        }
        Err(self.fault())
    }

    pub fn eaten(&self) -> u64 {
        u64::try_from(unsafe { (self.lib.filter_bytes)(self.a, -1) }).unwrap_or(0)
    }

    pub fn raw(&self) -> bool {
        (unsafe { (self.lib.format)(self.a) }) & FORMAT_MASK == FORMAT_RAW
    }

    fn filters(&self) -> c_int {
        unsafe { (self.lib.filter_count)(self.a) }
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            (self.lib.read_free)(self.a);
        }
    }
}
