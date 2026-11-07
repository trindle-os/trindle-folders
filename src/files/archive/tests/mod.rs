// Tests for archives.

mod boxed;
mod locked;
mod pipes_and_races;
mod round;
mod sevenz;
mod stranger;
mod sz_bytes;

use std::cell::Cell;
use std::fs::{self, File, FileTimes};
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{symlink, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use flate2::{Compression, Crc};
use flate2::write::GzEncoder;
use sha2::{Digest, Sha256};

use crate::files::archive::{find, inside, list, list_up_to, read, sniff, stem, tar_sum_holds, unlocks, Entry, Format, Need, MOST_THINGS, ROOM_KEPT};
use crate::files::archive::pack::{compress, pack, put_new};
use crate::files::archive::sevenz::{SzCoder, SzFolder, SzStreams, SZ_MAGIC};
use crate::files::archive::sevenz::lock::{sz_key, SzKeys};
use crate::files::archive::sevenz::unpack::sz_unpack;
use crate::files::archive::trouble::{trouble, trouble_of, wants_key, why, Key, Trouble};
use crate::files::archive::unpack::{extract, take_out, time_of, unpack, Most, Place, Roots, Taking};
use crate::files::archive::zip::{CENTRAL, END, LOCAL, S_IFDIR, S_IFLNK, S_IFMT, S_IFREG};
use crate::files::archive::zip::dos::{cp437, days_from_civil, from_dos, to_dos, CP437_HIGH};
use crate::files::archive::zip::lock::ZipOld;
use crate::files::archive::zip::write::{Body, ZipWriter};
use crate::files::meter::{is_stopped, Meter};
use crate::toolkit::t;
use super::super::unarchive::tests::{ar, cab, compress_z, cpio, lha, lz4, lzip, lzma_alone, rar4_split, rar5, rpm, xar, zstd, Thing};

const S_IFCHR: u32 = 0o020_000;
const S_IFIFO: u32 = 0o010_000;

fn place(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-files-archive-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn noise(n: usize) -> Vec<u8> {
    let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8
        })
        .collect()
}

fn quiet(_: u64, _: u64) {}

fn sample(dir: &Path) -> PathBuf {
    let top = dir.join("src");
    fs::create_dir_all(top.join("deep/deeper")).unwrap();
    fs::create_dir_all(top.join("empty")).unwrap();
    fs::write(top.join("empty.txt"), b"").unwrap();
    fs::write(top.join("noise.bin"), noise(200_000)).unwrap();
    fs::write(top.join("deep/words.txt"), "John Smith wrote this line again.\n".repeat(2000)).unwrap();
    fs::write(top.join("deep/deeper/café ünïcode.txt"), "hello").unwrap();
    symlink("deep/words.txt", top.join("link")).unwrap();
    symlink("../words.txt", top.join("deep/deeper/up")).unwrap();
    fs::set_permissions(top.join("empty.txt"), fs::Permissions::from_mode(0o755)).unwrap();
    File::open(top.join("noise.bin")).unwrap().set_times(FileTimes::new().set_modified(time_of(1_600_000_000))).unwrap();
    top
}

fn same(a: &Path, b: &Path) {
    let ma = fs::symlink_metadata(a).unwrap();
    let mb = fs::symlink_metadata(b).unwrap();
    assert_eq!(ma.file_type().is_dir(), mb.file_type().is_dir(), "{b:?}");
    assert_eq!(ma.file_type().is_symlink(), mb.file_type().is_symlink(), "{b:?}");
    if ma.file_type().is_symlink() {
        assert_eq!(fs::read_link(a).unwrap(), fs::read_link(b).unwrap());
    } else if ma.is_dir() {
        let mut na: Vec<_> = fs::read_dir(a).unwrap().map(|e| e.unwrap().file_name()).collect();
        let mut nb: Vec<_> = fs::read_dir(b).unwrap().map(|e| e.unwrap().file_name()).collect();
        na.sort();
        nb.sort();
        assert_eq!(na, nb, "{b:?}");
        for n in na {
            same(&a.join(&n), &b.join(&n));
        }
    } else {
        assert_eq!(fs::read(a).unwrap(), fs::read(b).unwrap(), "{b:?}");
        assert_eq!(ma.mode() & 0o777, mb.mode() & 0o777, "{b:?}");
        assert_eq!(ma.mtime(), mb.mtime(), "{b:?}");
    }
}

fn leftovers(dir: &Path) -> Vec<String> {
    fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).filter(|n| n.starts_with(".trindle-")).collect()
}

fn evil_zip(path: &Path, entries: &[(&str, u32, &[u8])]) {
    let stop = AtomicBool::new(false);
    let meter = Meter::new(&stop, &quiet);
    let mut z = ZipWriter::new(File::create(path).unwrap(), 0xFFFF_FFFF);
    for (name, mode, data) in entries {
        let body = if mode & S_IFMT == S_IFDIR { Body::Folder } else { Body::Bytes(data) };
        z.add(name.as_bytes(), body, *mode, 1_700_000_000, &meter).unwrap();
    }
    z.finish().unwrap();
}

fn evil_tar(path: &Path, entries: &[(&str, u32, &[u8])]) {
    let things: Vec<Thing> = entries.iter().map(|(name, mode, data)| (name.as_bytes(), *mode, *data)).collect();
    fs::write(path, tar_of(&things)).unwrap();
}

fn tar_of(things: &[Thing]) -> Vec<u8> {
    let mut b = tar::Builder::new(Vec::new());
    for (name, mode, data) in things {
        let mut h = tar::Header::new_old();
        let old = h.as_old_mut();
        old.name[..name.len()].copy_from_slice(name);
        let kind = match mode & S_IFMT {
            S_IFLNK => tar::EntryType::Symlink,
            S_IFDIR => tar::EntryType::Directory,
            S_IFCHR => tar::EntryType::Char,
            S_IFIFO => tar::EntryType::Fifo,
            _ => tar::EntryType::Regular,
        };
        if kind == tar::EntryType::Symlink {
            h.as_old_mut().linkname[..data.len()].copy_from_slice(data);
        }
        let bytes: &[u8] = if kind == tar::EntryType::Regular { data } else { &[] };
        h.set_entry_type(kind);
        h.set_size(bytes.len() as u64);
        h.set_uid(0);
        h.set_gid(0);
        h.set_mode(mode & 0o7777);
        h.set_mtime(1_700_000_000);
        h.set_cksum();
        b.append(&h, bytes).unwrap();
    }
    b.into_inner().unwrap()
}

fn evil_cpio(path: &Path, entries: &[(&str, u32, &[u8])]) {
    let things: Vec<Thing> = entries.iter().map(|(name, mode, data)| (name.as_bytes(), *mode, *data)).collect();
    fs::write(path, cpio(&things)).unwrap();
}

const FIXTURES: [(&str, &[u8]); 12] = [
    ("rar5.rar", include_bytes!("../../../../tests/fixtures/archives/rar5.rar")),
    ("rar4.rar", include_bytes!("../../../../tests/fixtures/archives/rar4.rar")),
    ("rar5-locked.rar", include_bytes!("../../../../tests/fixtures/archives/rar5-locked.rar")),
    ("rar5-lockedhead.rar", include_bytes!("../../../../tests/fixtures/archives/rar5-lockedhead.rar")),
    ("rar4-locked.rar", include_bytes!("../../../../tests/fixtures/archives/rar4-locked.rar")),
    ("rar4-lockedhead.rar", include_bytes!("../../../../tests/fixtures/archives/rar4-lockedhead.rar")),
    ("ppmd.7z", include_bytes!("../../../../tests/fixtures/archives/ppmd.7z")),
    ("bzip2.zip", include_bytes!("../../../../tests/fixtures/archives/bzip2.zip")),
    ("deflate64.zip", include_bytes!("../../../../tests/fixtures/archives/deflate64.zip")),
    ("split.part01.rar", include_bytes!("../../../../tests/fixtures/archives/split.part01.rar")),
    ("split.part02.rar", include_bytes!("../../../../tests/fixtures/archives/split.part02.rar")),
    ("split.part03.rar", include_bytes!("../../../../tests/fixtures/archives/split.part03.rar")),
];

fn fixture(dir: &Path, name: &str) -> PathBuf {
    let bytes = FIXTURES.iter().find(|f| f.0 == name).unwrap().1;
    fs::write(dir.join(name), bytes).unwrap();
    dir.join(name)
}

const LOCKED_KEY: &str = "john-smith-test";
const LOCKED: [(&str, &[u8]); 6] = [
    ("locked-old.zip", include_bytes!("../../../../tests/fixtures/archives/locked-old.zip")),
    ("locked-aes.zip", include_bytes!("../../../../tests/fixtures/archives/locked-aes.zip")),
    ("locked.7z", include_bytes!("../../../../tests/fixtures/archives/locked.7z")),
    ("locked-names.7z", include_bytes!("../../../../tests/fixtures/archives/locked-names.7z")),
    ("locked-odd.7z", include_bytes!("../../../../tests/fixtures/archives/locked-odd.7z")),
    ("locked-evil.7z", include_bytes!("../../../../tests/fixtures/archives/locked-evil.7z")),
];

fn locked(dir: &Path, name: &str) -> PathBuf {
    let bytes = LOCKED.iter().find(|(n, _)| *n == name).unwrap().1;
    let path = dir.join(name);
    fs::write(&path, bytes).unwrap();
    path
}

fn lock_old(data: &[u8], password: &str, check: u8) -> Vec<u8> {
    let mut z = ZipOld { inner: io::empty(), keys: [0x1234_5678, 0x2345_6789, 0x3456_7890] };
    for &b in password.as_bytes() {
        z.stir(b);
    }
    let mut plain = vec![7u8; 11];
    plain.push(check);
    plain.extend_from_slice(data);
    plain.iter().map(|&b| {
        let t = (z.keys[2] | 2) & 0xffff;
        z.stir(b);
        b ^ (t.wrapping_mul(t ^ 1) >> 8) as u8
    }).collect()
}

fn locked_zip(path: &Path, entries: &[(&str, u32, &[u8])], password: &str, method: u16) {
    let (mut out, mut central) = (Vec::new(), Vec::new());
    for (name, mode, data) in entries {
        let folder = mode & S_IFMT == S_IFDIR;
        let mut crc = Crc::new();
        crc.update(data);
        let body = if folder { Vec::new() } else { lock_old(data, password, (crc.sum() >> 24) as u8) };
        let at = out.len() as u32;
        let mut head = Vec::new();
        for v in [20u16, if folder { 0x800 } else { 0x801 }, if folder { 0 } else { method }, 0, 0x21] {
            head.extend(v.to_le_bytes());
        }
        for v in [crc.sum(), body.len() as u32, data.len() as u32] {
            head.extend(v.to_le_bytes());
        }
        head.extend((name.len() as u16).to_le_bytes());
        head.extend([0u8; 2]);
        out.extend(LOCAL.to_le_bytes());
        out.extend(&head);
        out.extend(name.as_bytes());
        out.extend(&body);
        central.extend(CENTRAL.to_le_bytes());
        central.extend(0x031e_u16.to_le_bytes());
        central.extend(&head);
        central.extend([0u8; 6]);
        central.extend((mode << 16).to_le_bytes());
        central.extend(at.to_le_bytes());
        central.extend(name.as_bytes());
    }
    let (n, size, at) = (entries.len() as u16, central.len() as u32, out.len() as u32);
    out.extend(&central);
    out.extend(END.to_le_bytes());
    out.extend([0u8; 4]);
    out.extend(n.to_le_bytes());
    out.extend(n.to_le_bytes());
    out.extend(size.to_le_bytes());
    out.extend(at.to_le_bytes());
    out.extend([0u8; 2]);
    fs::write(path, out).unwrap();
}

const SZ_HELLO: &[u8] = b"Hello from John Smith.\n";
const SZ_TIME: i64 = 1_767_323_045;

fn sz_note() -> Vec<u8> {
    "Second note, a little longer than the first one.\n".repeat(2).into_bytes()
}
