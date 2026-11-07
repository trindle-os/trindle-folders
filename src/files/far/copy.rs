// Copying one file between this computer and a server, with the room a server may fill held to what was listed.

use std::fs::{self, File};
use std::io::{self, Read, Seek, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;
use std::sync::atomic::Ordering;

use crate::toolkit::t;

use crate::files::proof;
use crate::files::remote::{Address, Error};
use crate::files::work;
use crate::common::files::readable;
use crate::common::gentle;
use super::ends::{End, Found};
use super::hands::Hands;
use super::links::Links;
use super::path_of;

fn spare_here() -> io::Result<File> {
    loop {
        let at = std::env::temp_dir().join(format!("trindle-files-far-{}", crate::toolkit::common::file_work::proof::tag()));
        match File::options().read(true).write(true).create_new(true).mode(0o600).open(&at) {
            Ok(file) => {
                let _ = fs::remove_file(&at);
                return Ok(file);
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
}

pub(super) fn copy_file(h: &Hands, from: &End, found: &Found, to: &End, sure: bool) -> Result<(u64, Option<File>), Error> {
    match (from, to) {
        (End::Here(_), End::Far(b)) => {
            let mut file = gentle::In::new(readable(&opened(found)?.0)?).take(found.size);
            send_to(h, b, &mut file, found.size)
        }
        (End::Far(a), End::Here(t)) => {
            let (file, kept) = new_here(t, 0o666)?;
            let reached = reached(h);
            let mut out = gentle::Out::new(file, &reached);
            let got = take_in(h.links, a, found.size, t.parent().and_then(room_in), &mut out, &mut || h.go_on()).and_then(|n| ended(h, out, n, t, sure));
            if got.is_err() {
                proof::unmake(&kept, t);
            }
            got
        }
        (End::Far(a), End::Far(b)) => {
            let reached = reached(h);
            let mut spare = gentle::Out::new(spare_here()?, &reached);
            take_in(h.links, a, found.size, room_in(&std::env::temp_dir()), &mut spare, &mut || h.go_on())?;
            let mut file = spare.finish_now()?;
            file.rewind()?;
            send_to(h, b, &mut file, found.size)
        }
        (End::Here(_), End::Here(t)) => {
            let (named, meta) = opened(found)?;
            let mut file = gentle::In::new(readable(named)?).take(found.size);
            let (made, kept) = new_here(t, meta.mode() & 0o7777)?;
            let reached = reached(h);
            let mut out = gentle::Out::new(made, &reached);
            let got = gentle::pour(&mut file, &mut out, &mut || h.go_on())
                .map_err(|e| if e.kind() == io::ErrorKind::Interrupted { Error::Cancelled } else { e.into() })
                .and_then(|n| ended(h, out, n, t, sure));
            if got.is_err() {
                proof::unmake(&kept, t);
            }
            got
        }
    }
}

pub(super) fn opened(found: &Found) -> Result<&(File, fs::Metadata), Error> {
    found.here.as_ref().ok_or_else(|| Error::Failed(t!("folders-far-not-opened").into()))
}

pub(super) fn reached<'a>(h: &Hands<'a>) -> impl Fn(u64) + 'a {
    let done = h.done;
    move |n| {
        done.fetch_add(n, Ordering::Relaxed);
    }
}

fn send_to(h: &Hands, b: &Address, file: &mut dyn Read, size: u64) -> Result<(u64, Option<File>), Error> {
    let mut told = 0;
    let sent = h.links.with(b, |r| {
        r.write_new(path_of(b), file, size, &mut |n| {
            h.done.fetch_add(n.saturating_sub(told), Ordering::Relaxed);
            told = told.max(n);
            h.go_on()
        })
    })?;
    Ok((sent, None))
}

fn ended(h: &Hands, out: gentle::Out<File>, n: u64, at: &Path, sure: bool) -> Result<(u64, Option<File>), Error> {
    if sure {
        let file = out.finish_now()?;
        file.sync_all()?;
        Ok((n, Some(file)))
    } else {
        out.finish(h.held, at.to_path_buf())?;
        Ok((n, None))
    }
}

fn new_here(t: &Path, mode: u32) -> io::Result<(File, File)> {
    let file = File::options().write(true).create_new(true).mode(mode).open(t)?;
    match file.try_clone() {
        Ok(kept) => Ok((file, kept)),
        Err(e) => {
            proof::unmake(&file, t);
            Err(e)
        }
    }
}

pub(super) const OVER_MOST: u64 = 8 << 20;

pub(super) const UNLISTED_MOST: u64 = 1 << 30;

pub(super) struct Most<'a, W: Write> {
    to: &'a mut W,
    pub(super) left: Option<u64>,
    pub(super) over: bool,
}

impl<'a, W: Write> Most<'a, W> {
    pub(super) fn new(to: &'a mut W, listed: u64, room: Option<u64>) -> Most<'a, W> {
        let by_size = if listed > 0 { listed.saturating_add(OVER_MOST) } else { UNLISTED_MOST };
        Most { to, left: Some(room.map_or(by_size, |r| by_size.min(r))), over: false }
    }
}

impl<W: Write> Write for Most<'_, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(left) = &mut self.left {
            if buf.len() as u64 > *left {
                self.over = true;
                return Err(io::Error::other(work::changed_why()));
            }
            *left -= buf.len() as u64;
        }
        self.to.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.to.flush()
    }
}

pub(super) fn room_in(dir: &Path) -> Option<u64> {
    crate::common::files::how_full(dir).map(|(_, free)| free)
}

pub(super) fn take_in<W: Write>(links: &Links, a: &Address, listed: u64, room: Option<u64>, to: &mut W, go_on: &mut dyn FnMut() -> bool) -> Result<u64, Error> {
    let mut most = Most::new(to, listed, room);
    let got = links.with(a, |r| r.read(path_of(a), &mut most, &mut |_| go_on()));
    if most.over { Err(Error::Failed(work::changed_why().into())) } else { got }
}
