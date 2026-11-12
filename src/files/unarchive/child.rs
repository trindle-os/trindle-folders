//! The child, in the box: reads the archive and writes frames.

use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::PathBuf;
use std::time::Instant;

use crate::files::walled;
use crate::files::archive::{self, Need};
use crate::files::libarchive::{self, Fault, Head, Reader, What};
use crate::files::meter::GAP;
use crate::files::unarchive::{need_of, word, CHUNK, DAMAGED, FINE, HAS_BYTES, HAS_DATE, HAS_MODE, LOCKED, MARK, MISSING, NAMELESS, NO_MEMORY, UNKNOWN, UNSUPPORTED};
use crate::files::unarchive::rar::PARTS_MOST;

pub fn main(args: Vec<String>) -> i32 {
    match asked(&args[1..].join(" ")) {
        Some((need, parts)) => child(need, parts),
        None => 2,
    }
}

pub(super) fn ask(need: Need, parts: usize) -> String {
    format!("{} {parts}", word(need))
}

pub(super) fn asked(words: &str) -> Option<(Need, usize)> {
    let (need, parts) = words.split_once(' ')?;
    Some((need_of(need)?, parts.parse().ok().filter(|p| (1..=PARTS_MOST).contains(p))?))
}

pub(super) fn inside(part: usize) -> PathBuf {
    PathBuf::from(if part == 0 { walled::INSIDE.to_string() } else { format!("{}.{part}", walled::INSIDE) })
}

pub(super) fn child(need: Need, parts: usize) -> i32 {
    libarchive::speak_utf8();
    let mut chosen = Vec::new();
    if need == Need::Some {
        let mut bytes = Vec::new();
        if io::stdin().lock().take(4 * archive::MOST_THINGS as u64).read_to_end(&mut bytes).is_err() {
            return 1;
        }
        chosen = bytes.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
        chosen.sort_unstable();
    }
    let files: Vec<PathBuf> = (0..parts).map(inside).collect();
    let Ok(out) = std::os::fd::AsFd::as_fd(&io::stdout()).try_clone_to_owned() else { return 1 };
    let mut out = BufWriter::with_capacity(1 << 17, File::from(out));
    match serve(&files, need, &chosen, &mut out) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

enum Halt {
    Pipe(io::Error),
    Fault(Fault),
}

impl From<io::Error> for Halt {
    fn from(e: io::Error) -> Halt {
        Halt::Pipe(e)
    }
}

impl From<Fault> for Halt {
    fn from(f: Fault) -> Halt {
        Halt::Fault(f)
    }
}

pub(super) fn serve(files: &[PathBuf], need: Need, chosen: &[u32], out: &mut dyn Write) -> io::Result<()> {
    out.write_all(&MARK)?;
    let code = match send(files, need, chosen, out) {
        Ok(()) => FINE,
        Err(Halt::Pipe(e)) => return Err(e),
        Err(Halt::Fault(Fault::Locked)) => LOCKED,
        Err(Halt::Fault(Fault::Unknown)) => UNKNOWN,
        Err(Halt::Fault(Fault::Memory)) => NO_MEMORY,
        Err(Halt::Fault(Fault::Missing)) => MISSING,
        Err(Halt::Fault(Fault::Unsupported)) => UNSUPPORTED,
        Err(Halt::Fault(Fault::Damaged)) => DAMAGED,
    };
    out.write_all(&[b'E', code])?;
    out.flush()
}

fn send(files: &[PathBuf], need: Need, chosen: &[u32], out: &mut dyn Write) -> Result<(), Halt> {
    let mut reader = Reader::open(files)?;
    let mut buf = vec![0u8; CHUNK];
    let mut said = Instant::now();
    let (mut number, mut seen) = (0u32, 0u64);
    while let Some(head) = reader.next()? {
        seen += 1;
        tick(&reader, out, &mut said)?;
        let kind = match head.what {
            What::File => 0u8,
            What::Folder => 1,
            What::Symlink => 2,
            What::Hardlink => 3,
            What::Other => continue,
        };
        let this = number;
        number = number.saturating_add(1);
        if need == Need::Some && chosen.binary_search(&this).is_err() {
            continue;
        }
        let bytes = need != Need::Names && head.what == What::File;
        if bytes && head.locked {
            return Err(Fault::Locked.into());
        }
        write_head(out, &head, kind, bytes, reader.raw())?;
        if bytes {
            send_bytes(&mut reader, out, &mut buf, &mut said, head.size.unwrap_or(u64::MAX))?;
        }
        if need == Need::Some && chosen.last() == Some(&this) {
            break;
        }
    }
    if seen == 0 && reader.eaten() == 0 {
        return Err(Fault::Unknown.into());
    }
    out.write_all(b"P")?;
    out.write_all(&reader.eaten().to_le_bytes())?;
    Ok(())
}

fn write_head(out: &mut dyn Write, head: &Head, kind: u8, bytes: bool, nameless: bool) -> io::Result<()> {
    let flags = if head.mode.is_some() { HAS_MODE } else { 0 }
        | if head.modified.is_some() { HAS_DATE } else { 0 }
        | if bytes { HAS_BYTES } else { 0 }
        | if nameless { NAMELESS } else { 0 };
    out.write_all(&[b'H', kind, flags])?;
    out.write_all(&head.mode.unwrap_or(0).to_le_bytes())?;
    out.write_all(&head.modified.unwrap_or(0).to_le_bytes())?;
    out.write_all(&head.size.unwrap_or(0).to_le_bytes())?;
    out.write_all(&(head.name.len() as u32).to_le_bytes())?;
    out.write_all(&(head.link.len() as u32).to_le_bytes())?;
    out.write_all(&head.name)?;
    out.write_all(&head.link)
}

fn send_bytes(reader: &mut Reader, out: &mut dyn Write, buf: &mut [u8], said: &mut Instant, size: u64) -> Result<(), Halt> {
    let mut left = size;
    loop {
        let n = (reader.data(buf)? as u64).min(left) as usize;
        if n == 0 {
            break;
        }
        left -= n as u64;
        out.write_all(b"D")?;
        out.write_all(&(n as u32).to_le_bytes())?;
        out.write_all(&buf[..n])?;
        tick(reader, out, said)?;
    }
    out.write_all(b"Z")?;
    Ok(())
}

fn tick(reader: &Reader, out: &mut dyn Write, said: &mut Instant) -> io::Result<()> {
    if said.elapsed() < GAP {
        return Ok(());
    }
    *said = Instant::now();
    out.write_all(b"P")?;
    out.write_all(&reader.eaten().to_le_bytes())?;
    out.flush()
}
