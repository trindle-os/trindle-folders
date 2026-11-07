// The parts of a split RAR, and what a RAR's headers say before it is unpacked.

use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::files::archive::{self, trouble, Trouble};

pub(super) const PARTS_MOST: usize = 1000;

pub(in crate::files) fn rar_part(stem: &str) -> Option<(&str, &str)> {
    let dot = stem.to_ascii_lowercase().rfind(".part").filter(|d| *d > 0)?;
    let (head, digits) = stem.split_at(dot + 5);
    (!digits.is_empty() && digits.len() <= 5 && digits.bytes().all(|b| b.is_ascii_digit())).then_some((head, digits))
}

pub(super) fn volumes(archive: &Path) -> Vec<PathBuf> {
    let alone = || vec![archive.to_path_buf()];
    let (Some(dir), Some(name)) = (archive.parent(), archive.file_name().and_then(|n| n.to_str())) else { return alone() };
    if !name.to_ascii_lowercase().ends_with(".rar") {
        return alone();
    }
    let (stem, ending) = name.split_at(name.len() - 4);
    if let Some((head, digits)) = rar_part(stem) {
        let all: Vec<PathBuf> = (1..=PARTS_MOST)
            .map(|k| dir.join(format!("{head}{k:0width$}{ending}", width = digits.len())))
            .take_while(|p| p.is_file())
            .collect();
        return if all.iter().any(|p| p == archive) { all } else { alone() };
    }
    let mut all = alone();
    for k in 0..100 {
        match [format!("{stem}.r{k:02}"), format!("{stem}.R{k:02}")].into_iter().map(|n| dir.join(n)).find(|p| p.is_file()) {
            Some(next) => all.push(next),
            None => break,
        }
    }
    all
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Look {
    locked: bool,
    later: bool,
    more: bool,
}

pub(super) fn whole(parts: &[PathBuf]) -> io::Result<()> {
    for (i, part) in parts.iter().enumerate() {
        let look = match rar5_look(part)? {
            Some(look) => look,
            None => rar4_look(part).unwrap_or_default(),
        };
        if look.locked {
            return Err(trouble(Trouble::Locked));
        }
        if (i == 0 && look.later) || (i + 1 == parts.len() && look.more) {
            return Err(trouble(Trouble::Parts));
        }
    }
    Ok(())
}

pub(super) const RAR4_MARK: [u8; 7] = *b"Rar!\x1a\x07\x00";

fn rar4_look(part: &Path) -> Option<Look> {
    use std::io::{Seek, SeekFrom};
    let mut file = archive::opened(part).ok()?;
    let mut mark = [0u8; 7];
    file.read_exact(&mut mark).ok()?;
    if mark != RAR4_MARK {
        return None;
    }
    let len = file.metadata().ok()?.len();
    let mut tail = vec![0u8; len.min(13) as usize];
    file.seek(SeekFrom::Start(len - tail.len() as u64)).ok()?;
    file.read_exact(&mut tail).ok()?;
    let more = [7usize, 9, 11, 13].into_iter().filter(|n| *n <= tail.len()).any(|n| {
        let block = &tail[tail.len() - n..];
        let mut crc = flate2::Crc::new();
        crc.update(&block[2..]);
        let sound = block[2] == 0x7b && usize::from(u16::from_le_bytes([block[5], block[6]])) == n && crc.sum() as u16 == u16::from_le_bytes([block[0], block[1]]);
        sound && block[3] & 1 != 0
    });
    Some(Look { more, ..Look::default() })
}

const RAR5_MARK: [u8; 8] = *b"Rar!\x1a\x07\x01\x00";
const RAR5_HEAD_MOST: u64 = 2 << 20;

fn rar5_number(b: &mut &[u8]) -> Option<u64> {
    let mut n = 0u64;
    for i in 0..10 {
        let (&byte, rest) = b.split_first()?;
        *b = rest;
        n |= u64::from(byte & 0x7f) << (7 * i);
        if byte & 0x80 == 0 {
            return Some(n);
        }
    }
    None
}

fn rar5_look(archive: &Path) -> io::Result<Option<Look>> {
    use std::io::{Seek, SeekFrom};
    let cut = || io::Error::from(io::ErrorKind::InvalidData);
    let mut file = archive::opened(archive)?;
    let mut mark = [0u8; 8];
    if file.read_exact(&mut mark).is_err() || mark != RAR5_MARK {
        return Ok(None);
    }
    let mut look = Look::default();
    let mut at = 8u64;
    for _ in 0..2 * archive::MOST_THINGS {
        file.seek(SeekFrom::Start(at))?;
        let mut start = [0u8; 7];
        file.read_exact(&mut start)?;
        let mut b = &start[4..];
        let size = rar5_number(&mut b).filter(|s| (1..=RAR5_HEAD_MOST).contains(s)).ok_or_else(cut)?;
        let size_len = 3 - b.len() as u64;
        file.seek(SeekFrom::Start(at + 4 + size_len))?;
        let mut head = vec![0u8; size as usize];
        file.read_exact(&mut head)?;
        let mut h = &head[..];
        let kind = rar5_number(&mut h).ok_or_else(cut)?;
        let flags = rar5_number(&mut h).ok_or_else(cut)?;
        let extra = if flags & 1 != 0 { rar5_number(&mut h).ok_or_else(cut)? } else { 0 };
        let data = if flags & 2 != 0 { rar5_number(&mut h).ok_or_else(cut)? } else { 0 };
        match kind {
            1 => {
                let flags = rar5_number(&mut h).ok_or_else(cut)?;
                look.later = flags & 2 != 0 && rar5_number(&mut h).ok_or_else(cut)? > 0;
            }
            4 => {
                look.locked = true;
                return Ok(Some(look));
            }
            5 => {
                look.more = rar5_number(&mut h).ok_or_else(cut)? & 1 != 0;
                return Ok(Some(look));
            }
            2 => {
                let from = head.len().checked_sub(usize::try_from(extra).map_err(|_| cut())?).ok_or_else(cut)?;
                let mut records = &head[from..];
                while !records.is_empty() {
                    let len = usize::try_from(rar5_number(&mut records).ok_or_else(cut)?).map_err(|_| cut())?;
                    let mut record = records.get(..len).ok_or_else(cut)?;
                    look.locked |= rar5_number(&mut record) == Some(1);
                    records = &records[len..];
                }
            }
            _ => {}
        }
        at = at.checked_add(4 + size_len + size).and_then(|a| a.checked_add(data)).ok_or_else(cut)?;
    }
    Ok(Some(look))
}
