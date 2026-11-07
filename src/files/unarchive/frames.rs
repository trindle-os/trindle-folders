// The child's frames as they arrive, each checked.

use std::io::{self, BufReader, Read};

use crate::files::archive::{self, trouble, Item, Kind, Trouble};
use crate::files::meter::Meter;
use crate::files::unarchive::{CHUNK, HAS_BYTES, HAS_DATE, HAS_MODE, LOCKED, MARK, MARK_WITHIN, MISSING, NAMELESS, NAME_MOST, NO_MEMORY, UNKNOWN, UNSUPPORTED};
use crate::toolkit::t;

pub(super) struct Frames<'m, R> {
    pub(super) r: BufReader<R>,
    pub(super) meter: &'m Meter<'m>,
    pub(super) eaten: u64,
}

pub(super) fn bad_frame() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, t!("folders-unarchive-bad-frame"))
}

pub(super) fn ended(code: u8) -> io::Error {
    match code {
        LOCKED => trouble(Trouble::Locked),
        UNKNOWN => trouble(Trouble::Unknown),
        NO_MEMORY => trouble(Trouble::GaveUp),
        MISSING => trouble(Trouble::NoReader),
        UNSUPPORTED => trouble(Trouble::Foreign),
        _ => io::Error::new(io::ErrorKind::InvalidData, t!("folders-unarchive-damaged")),
    }
}

impl<R: Read> Frames<'_, R> {
    pub(super) fn byte(&mut self) -> io::Result<Option<u8>> {
        let mut b = [0u8; 1];
        loop {
            match self.r.read(&mut b) {
                Ok(0) => return Ok(None),
                Ok(_) => return Ok(Some(b[0])),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
    }

    pub(super) fn exact<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut b = [0u8; N];
        self.r.read_exact(&mut b)?;
        Ok(b)
    }

    fn text(&mut self, len: u32) -> io::Result<String> {
        let len = usize::try_from(len).ok().filter(|l| *l <= NAME_MOST).ok_or_else(bad_frame)?;
        let mut b = vec![0u8; len];
        self.r.read_exact(&mut b)?;
        Ok(String::from_utf8_lossy(&b).into_owned())
    }

    pub(super) fn find_mark(&mut self) -> io::Result<()> {
        let mut found = 0;
        for _ in 0..MARK_WITHIN {
            let b = self.byte()?.ok_or_else(|| trouble(Trouble::NoReader))?;
            found = if b == MARK[found] { found + 1 } else { usize::from(b == MARK[0]) };
            if found == MARK.len() {
                return Ok(());
            }
        }
        Err(bad_frame())
    }

    pub(super) fn progress(&mut self) -> io::Result<()> {
        let now = u64::from_le_bytes(self.exact()?);
        let more = now.saturating_sub(self.eaten);
        self.eaten = self.eaten.max(now);
        self.meter.add(more)
    }

    pub(super) fn head(&mut self, nameless: &str) -> io::Result<(Item, bool)> {
        let [kind, flags] = self.exact()?;
        let mode = u32::from_le_bytes(self.exact()?);
        let date = i64::from_le_bytes(self.exact()?);
        let size = u64::from_le_bytes(self.exact()?);
        let (name_len, link_len) = (u32::from_le_bytes(self.exact()?), u32::from_le_bytes(self.exact()?));
        let name = self.text(name_len)?;
        let link = self.text(link_len)?;
        let kind = match kind {
            0 => Kind::File,
            1 => Kind::Folder,
            2 => Kind::Link(link),
            3 => Kind::Hard(archive::tidy(&link)),
            _ => return Err(bad_frame()),
        };
        let bytes = flags & HAS_BYTES != 0;
        if bytes && !matches!(kind, Kind::File) {
            return Err(bad_frame());
        }
        let item = Item {
            name: if flags & NAMELESS != 0 { nameless.to_string() } else { archive::tidy(&name) },
            size: if matches!(kind, Kind::File) { size } else { 0 },
            kind,
            mode: (flags & HAS_MODE != 0).then_some(mode),
            modified: (flags & HAS_DATE != 0).then_some(date),
            locked: false,
        };
        Ok((item, bytes))
    }
}

pub(super) struct Body<'f, 'm, R> {
    pub(super) frames: &'f mut Frames<'m, R>,
    pub(super) left: usize,
    pub(super) over: bool,
}

impl<R: Read> Read for Body<'_, '_, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        while self.left == 0 && !self.over {
            match self.frames.byte()? {
                Some(b'D') => {
                    let len = u32::from_le_bytes(self.frames.exact()?) as usize;
                    if len == 0 || len > CHUNK {
                        return Err(bad_frame());
                    }
                    self.left = len;
                }
                Some(b'P') => self.frames.progress()?,
                Some(b'Z') => self.over = true,
                Some(b'E') => return Err(ended(self.frames.exact::<1>()?[0])),
                Some(_) => return Err(bad_frame()),
                None => return Err(trouble(Trouble::GaveUp)),
            }
        }
        if self.over || buf.is_empty() {
            return Ok(0);
        }
        let room = buf.len().min(self.left);
        let n = self.frames.r.read(&mut buf[..room])?;
        if n == 0 {
            return Err(trouble(Trouble::GaveUp));
        }
        self.left -= n;
        Ok(n)
    }
}
