//! Packing chosen files and folders into a new archive.

use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use flate2::Compression;
use flate2::write::GzEncoder;

use crate::common::gentle;
use crate::files::archive::{opened_as_found, Format};
use crate::files::archive::zip::write::{Body, ZipWriter};
use crate::files::meter::{Counted, Meter};
use crate::toolkit::t;

pub(super) struct Gathered {
    pub(super) path: PathBuf,
    pub(super) name: Vec<u8>,
    pub(super) meta: fs::Metadata,
}

fn gather(path: &Path, name: Vec<u8>, out: &mut Vec<Gathered>, meter: &Meter) -> io::Result<()> {
    meter.check()?;
    let meta = fs::symlink_metadata(path)?;
    let ft = meta.file_type();
    if ft.is_dir() {
        out.push(Gathered { path: path.to_path_buf(), name: name.clone(), meta });
        let mut kids = fs::read_dir(path)?.map(|e| e.map(|e| e.file_name())).collect::<io::Result<Vec<OsString>>>()?;
        kids.sort();
        for kid in kids {
            let mut n = name.clone();
            n.push(b'/');
            n.extend_from_slice(kid.as_bytes());
            gather(&path.join(&kid), n, out, meter)?;
        }
    } else if ft.is_file() || ft.is_symlink() {
        out.push(Gathered { path: path.to_path_buf(), name, meta });
    }
    Ok(())
}

static TEMP: AtomicU64 = AtomicU64::new(0);

pub(super) fn temp_name(dir: &Path, what: &str) -> PathBuf {
    let n = TEMP.fetch_add(1, Ordering::Relaxed);
    dir.join(format!(".trindle-{what}-{}-{n}", std::process::id()))
}

pub fn compress(items: &[PathBuf], to: &Path, format: Format, stop: &AtomicBool, tell: &dyn Fn(u64, u64)) -> io::Result<()> {
    let meter = Meter::new(stop, tell);
    pack(items, to, format, 0xFFFF_FFFF, &meter).map_err(|e| meter.check().err().unwrap_or(e))
}

pub(super) fn pack(items: &[PathBuf], to: &Path, format: Format, big: u64, meter: &Meter) -> io::Result<()> {
    if fs::symlink_metadata(to).is_ok() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, t!("folders-archive-pack-exists")));
    }
    let mut list = Vec::new();
    for item in items {
        let name = item.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, t!("folders-archive-pack-no-name")))?;
        gather(item, name.as_bytes().to_vec(), &mut list, meter)?;
    }
    meter.total(list.iter().filter(|g| g.meta.is_file()).map(|g| g.meta.len()).sum());
    let dir = to.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let temp = loop {
        let t = temp_name(dir, "pack");
        match OpenOptions::new().write(true).read(true).create_new(true).mode(0o644).open(&t) {
            Ok(f) => break (t, f),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    };
    let (temp, file) = temp;
    let written = write_archive(file, &list, format, big, meter).and_then(|()| put_new(&temp, to));
    if written.is_err() {
        let _ = fs::remove_file(&temp);
    }
    meter.end();
    written
}

fn write_archive(file: File, list: &[Gathered], format: Format, big: u64, meter: &Meter) -> io::Result<()> {
    let w = BufWriter::with_capacity(1 << 18, gentle::Tide::new(file));
    let w = match format {
        Format::Zip => {
            let mut z = ZipWriter::new(w, big);
            for g in list {
                let ft = g.meta.file_type();
                let body = if ft.is_dir() {
                    Body::Folder
                } else if ft.is_symlink() {
                    let target = fs::read_link(&g.path)?;
                    z.add(&g.name, Body::Bytes(target.as_os_str().as_bytes()), g.meta.mode(), g.meta.mtime(), meter)?;
                    continue;
                } else {
                    Body::File(g)
                };
                z.add(&g.name, body, g.meta.mode(), g.meta.mtime(), meter)?;
            }
            z.finish()?
        }
        Format::Tar => tar_into(w, list, meter)?,
        Format::TarGz => tar_into(GzEncoder::new(w, Compression::default()), list, meter)?.finish()?,
        Format::TarBz2 => tar_into(bzip2::write::BzEncoder::new(w, bzip2::Compression::default()), list, meter)?.finish()?,
        Format::TarXz => {
            let xz = lzma_rust2::XzWriter::new(w, lzma_rust2::XzOptions::with_preset(3))?;
            tar_into(xz, list, meter)?.finish()?
        }
        Format::SevenZ | Format::Gz | Format::Bz2 | Format::Xz | Format::Other => {
            return Err(io::Error::new(io::ErrorKind::Unsupported, t!("folders-archive-pack-cannot-make")));
        }
    };
    let file = w.into_inner().map_err(|e| e.into_error())?.finish()?;
    file.sync_all()
}

fn tar_into<W: Write>(w: W, list: &[Gathered], meter: &Meter) -> io::Result<W> {
    let mut b = tar::Builder::new(w);
    b.follow_symlinks(false);
    for g in list {
        meter.check()?;
        let mut h = tar::Header::new_gnu();
        h.set_metadata_in_mode(&g.meta, tar::HeaderMode::Complete);
        let name = Path::new(OsStr::from_bytes(&g.name));
        let ft = g.meta.file_type();
        if ft.is_symlink() {
            b.append_link(&mut h, name, fs::read_link(&g.path)?)?;
        } else if ft.is_dir() {
            b.append_data(&mut h, name, io::empty())?;
        } else {
            let f = opened_as_found(g)?;
            let len = g.meta.len();
            b.append_data(&mut h, name, Exact { inner: Counted::new(gentle::In::new(f), meter).take(len), left: len })?;
        }
    }
    b.into_inner()
}

struct Exact<R> {
    inner: R,
    left: u64,
}

impl<R: Read> Read for Exact<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let room = buf.len().min(usize::try_from(self.left).unwrap_or(usize::MAX));
        if room == 0 {
            return Ok(0);
        }
        let mut n = self.inner.read(&mut buf[..room])?;
        if n == 0 {
            buf[..room].fill(0);
            n = room;
        }
        self.left -= n as u64;
        Ok(n)
    }
}

pub(super) fn put_new(from: &Path, to: &Path) -> io::Result<()> {
    match fs::hard_link(from, to) {
        Ok(()) => {
            let _ = fs::remove_file(from);
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Err(e),
        Err(_) => crate::common::files::rename_new(from, to),
    }
}
