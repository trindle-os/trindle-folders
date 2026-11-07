// Reading and writing a song's ID3 tag, versions 1 and 2.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::unix::fs::FileExt;
use std::path::Path;

use super::{change, opened};
use crate::toolkit::t;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Id3 {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: String,
    pub track: String,
    pub genre: String,
}

pub fn read_id3(path: &Path) -> io::Result<Option<Id3>> {
    let mut file = crate::common::files::open_file(path)?;
    if let Some(tag) = read_v2(&mut file)? {
        return Ok(Some(tag.details()));
    }
    Ok(read_v1(&file)?.map(|b| v1_details(&b)))
}

pub fn write_id3(path: &Path, id3: &Id3) -> io::Result<()> {
    let mut file = opened(path)?;
    let old = read_v2(&mut file)?;
    let major = if old.as_ref().is_some_and(|t| t.major == 4) { 4 } else { 3 };
    let year = if major == 4 { "TDRC" } else { "TYER" };
    let ours = [("TIT2", &id3.title), ("TPE1", &id3.artist), ("TALB", &id3.album), (year, &id3.year), ("TRCK", &id3.track), ("TCON", &id3.genre)];
    let mut body = Vec::new();
    for (id, value) in ours {
        if !value.is_empty() {
            put_frame(&mut body, major, id, [0, 0], &encode(major, value));
        }
    }
    if let Some(tag) = old.as_ref().filter(|t| t.major >= 3) {
        for f in tag.frames.iter().filter(|f| !REPLACED.contains(&f.id.as_str())) {
            put_frame(&mut body, major, &f.id, f.flags, &f.data);
        }
    }
    let room = old.as_ref().map_or(0, |t| t.room);
    let fits = old.is_some() && body.len() as u64 + 10 <= room;
    body.resize(if fits { room as usize - 10 } else { body.len() + PADDING }, 0);
    let mut tag = header(major, body.len())?;
    tag.extend_from_slice(&body);
    let len = file.metadata()?.len();
    let last = read_v1(&file)?.map(|mut b| {
        set_v1(&mut b, id3);
        b
    });
    let song = len.saturating_sub(if last.is_some() { 128 } else { 0 }).saturating_sub(room);
    change(path, tag.len() as u64 + len.saturating_sub(room), &mut |out| {
        out.write_all(&tag)?;
        file.seek(SeekFrom::Start(room))?;
        io::copy(&mut Read::take(&file, song), out)?;
        match &last {
            Some(b) => out.write_all(b),
            None => Ok(()),
        }
    })
}

const REPLACED: [&str; 7] = ["TIT2", "TPE1", "TALB", "TYER", "TDRC", "TRCK", "TCON"];

const PADDING: usize = 2048;

fn header(major: u8, size: usize) -> io::Result<Vec<u8>> {
    if size >= 1 << 28 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, t!("folders-meta-tag-too-big")));
    }
    let mut h = b"ID3".to_vec();
    h.extend_from_slice(&[major, 0, 0]);
    h.extend_from_slice(&syncsafe_bytes(size as u32));
    Ok(h)
}

fn put_frame(out: &mut Vec<u8>, major: u8, id: &str, flags: [u8; 2], data: &[u8]) {
    out.extend_from_slice(id.as_bytes());
    let size = data.len() as u32;
    out.extend_from_slice(&if major == 4 { syncsafe_bytes(size) } else { size.to_be_bytes() });
    out.extend_from_slice(&flags);
    out.extend_from_slice(data);
}

fn encode(major: u8, s: &str) -> Vec<u8> {
    if major == 4 {
        let mut out = vec![3];
        out.extend_from_slice(s.as_bytes());
        out
    } else if s.chars().all(|c| (c as u32) < 0x100) {
        std::iter::once(0).chain(s.chars().map(|c| c as u8)).collect()
    } else {
        let mut out = vec![1, 0xff, 0xfe];
        for unit in s.encode_utf16() {
            out.extend_from_slice(&unit.to_le_bytes());
        }
        out
    }
}

fn syncsafe_bytes(n: u32) -> [u8; 4] {
    [(n >> 21) as u8 & 0x7f, (n >> 14) as u8 & 0x7f, (n >> 7) as u8 & 0x7f, n as u8 & 0x7f]
}

fn syncsafe(b: &[u8]) -> u32 {
    b.iter().fold(0, |n, x| (n << 7) | (*x as u32 & 0x7f))
}

fn be(b: &[u8]) -> u32 {
    b.iter().fold(0, |n, x| (n << 8) | *x as u32)
}

fn resync(b: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        out.push(b[i]);
        i += if b[i] == 0xff && b.get(i + 1) == Some(&0) { 2 } else { 1 };
    }
    out
}

struct Frame {
    id: String,
    flags: [u8; 2],
    data: Vec<u8>,
}

struct V2 {
    major: u8,
    frames: Vec<Frame>,
    room: u64,
}

fn read_v2(file: &mut File) -> io::Result<Option<V2>> {
    let mut head = [0u8; 10];
    file.seek(SeekFrom::Start(0))?;
    if file.read_exact(&mut head).is_err() || &head[..3] != b"ID3" || !(2..=4).contains(&head[3]) || head[6..].iter().any(|b| b & 0x80 != 0) {
        return Ok(None);
    }
    let (major, flags) = (head[3], head[5]);
    let size = syncsafe(&head[6..]) as u64;
    let footer = if major == 4 && flags & 0x10 != 0 { 10 } else { 0 };
    let mut body = Vec::new();
    Read::by_ref(file).take(size).read_to_end(&mut body)?;
    let room = 10 + size + footer;
    if major < 4 && flags & 0x80 != 0 {
        body = resync(&body);
    }
    if major == 2 && flags & 0x40 != 0 {
        return Ok(Some(V2 { major, frames: Vec::new(), room }));
    }
    let mut pos = 0usize;
    if major >= 3 && flags & 0x40 != 0 && body.len() >= 4 {
        pos = if major == 3 { 4 + be(&body[..4]) as usize } else { syncsafe(&body[..4]) as usize };
    }
    let (idlen, hlen) = if major == 2 { (3, 6) } else { (4, 10) };
    let mut frames = Vec::new();
    while pos + hlen <= body.len() {
        let id = &body[pos..pos + idlen];
        if !id.iter().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
            break;
        }
        let size = match major {
            2 => be(&body[pos + 3..pos + 6]) as usize,
            3 => be(&body[pos + 4..pos + 8]) as usize,
            _ => size24(&body, pos),
        };
        let fl = if major == 2 { [0, 0] } else { [body[pos + 8], body[pos + 9]] };
        let end = pos + hlen + size;
        if end > body.len() {
            break;
        }
        frames.push(Frame { id: String::from_utf8_lossy(id).into_owned(), flags: fl, data: body[pos + hlen..end].to_vec() });
        pos = end;
    }
    Ok(Some(V2 { major, frames, room }))
}

fn size24(body: &[u8], pos: usize) -> usize {
    let b = &body[pos + 4..pos + 8];
    let plain = be(b) as usize;
    if b.iter().any(|x| x & 0x80 != 0) {
        return plain;
    }
    let safe = syncsafe(b) as usize;
    let lands = |at: usize| {
        at == body.len()
            || body.get(at) == Some(&0)
            || body.get(at..at + 4).is_some_and(|id| id.iter().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()))
    };
    if safe != plain && !lands(pos + 10 + safe) && lands(pos + 10 + plain) {
        plain
    } else {
        safe
    }
}

impl V2 {
    fn details(&self) -> Id3 {
        let names: [[&str; 3]; 6] = [["TIT2", "TT2", ""], ["TPE1", "TP1", ""], ["TALB", "TAL", ""], ["TDRC", "TYER", "TYE"], ["TRCK", "TRK", ""], ["TCON", "TCO", ""]];
        let mut out: [String; 6] = Default::default();
        for (slot, ids) in out.iter_mut().zip(names) {
            *slot = ids.iter().filter(|id| !id.is_empty()).find_map(|id| self.frames.iter().find(|f| f.id == *id).and_then(|f| self.text(f))).unwrap_or_default();
        }
        let [title, artist, album, year, track, genre] = out;
        let year = year.chars().take(4).collect();
        Id3 { title, artist, album, year, track, genre: genre_name(&genre) }
    }

    fn text(&self, f: &Frame) -> Option<String> {
        let mut data: &[u8] = &f.data;
        let owned;
        match self.major {
            3 => {
                if f.flags[1] & 0xc0 != 0 {
                    return None;
                }
                if f.flags[1] & 0x20 != 0 {
                    data = data.get(1..)?;
                }
            }
            4 => {
                if f.flags[1] & 0x0c != 0 {
                    return None;
                }
                if f.flags[1] & 0x40 != 0 {
                    data = data.get(1..)?;
                }
                if f.flags[1] & 0x01 != 0 {
                    data = data.get(4..)?;
                }
                if f.flags[1] & 0x02 != 0 {
                    owned = resync(data);
                    data = &owned;
                }
            }
            _ => {}
        }
        let s = decode(data);
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}

fn decode(data: &[u8]) -> String {
    let Some((enc, rest)) = data.split_first() else { return String::new() };
    let text: String = match enc {
        0 => rest.iter().map(|b| *b as char).collect(),
        1 | 2 => {
            let mut little = *enc == 1;
            let units = rest.chunks_exact(2).filter_map(|p| {
                let unit = if little { u16::from_le_bytes([p[0], p[1]]) } else { u16::from_be_bytes([p[0], p[1]]) };
                match unit {
                    0xfeff => None,
                    0xfffe => {
                        little = !little;
                        None
                    }
                    u => Some(u),
                }
            });
            char::decode_utf16(units.collect::<Vec<_>>()).map(|c| c.unwrap_or('\u{fffd}')).collect()
        }
        _ => String::from_utf8_lossy(rest).into_owned(),
    };
    text.split('\0').map(str::trim).filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" / ")
}

const GENRES: [&str; 80] = [
    "Blues", "Classic Rock", "Country", "Dance", "Disco", "Funk", "Grunge", "Hip-Hop", "Jazz", "Metal", "New Age", "Oldies", "Other", "Pop", "R&B", "Rap",
    "Reggae", "Rock", "Techno", "Industrial", "Alternative", "Ska", "Death Metal", "Pranks", "Soundtrack", "Euro-Techno", "Ambient", "Trip-Hop", "Vocal",
    "Jazz+Funk", "Fusion", "Trance", "Classical", "Instrumental", "Acid", "House", "Game", "Sound Clip", "Gospel", "Noise", "AlternRock", "Bass", "Soul",
    "Punk", "Space", "Meditative", "Instrumental Pop", "Instrumental Rock", "Ethnic", "Gothic", "Darkwave", "Techno-Industrial", "Electronic", "Pop-Folk",
    "Eurodance", "Dream", "Southern Rock", "Comedy", "Cult", "Gangsta", "Top 40", "Christian Rap", "Pop/Funk", "Jungle", "Native American", "Cabaret",
    "New Wave", "Psychadelic", "Rave", "Showtunes", "Trailer", "Lo-Fi", "Tribal", "Acid Punk", "Acid Jazz", "Polka", "Retro", "Musical", "Rock & Roll",
    "Hard Rock",
];

fn genre_name(s: &str) -> String {
    let by_code = |code: &str| match code {
        "RX" => Some("Remix".to_string()),
        "CR" => Some("Cover".to_string()),
        n => n.parse::<usize>().ok().and_then(|n| GENRES.get(n)).map(|g| g.to_string()),
    };
    if let Some(inner) = s.strip_prefix('(') {
        if let Some((code, after)) = inner.split_once(')') {
            if !after.trim().is_empty() {
                return after.trim().to_string();
            }
            if let Some(name) = by_code(code) {
                return name;
            }
        }
    }
    by_code(s.trim()).unwrap_or_else(|| s.to_string())
}

fn read_v1(file: &File) -> io::Result<Option<[u8; 128]>> {
    let len = file.metadata()?.len();
    if len < 128 {
        return Ok(None);
    }
    let mut b = [0u8; 128];
    file.read_exact_at(&mut b, len - 128)?;
    Ok(if &b[..3] == b"TAG" { Some(b) } else { None })
}

fn v1_details(b: &[u8; 128]) -> Id3 {
    let field = |r: std::ops::Range<usize>| {
        let raw = &b[r];
        let end = raw.iter().position(|c| *c == 0).unwrap_or(raw.len());
        raw[..end].iter().map(|c| *c as char).collect::<String>().trim().to_string()
    };
    let track = if b[125] == 0 && b[126] != 0 { b[126].to_string() } else { String::new() };
    let genre = GENRES.get(b[127] as usize).map(|g| g.to_string()).unwrap_or_default();
    Id3 { title: field(3..33), artist: field(33..63), album: field(63..93), year: field(93..97), track, genre }
}

fn set_v1(b: &mut [u8; 128], id3: &Id3) {
    let mut put = |at: usize, width: usize, s: &str| {
        let bytes: Vec<u8> = s.chars().map(|c| if (c as u32) < 0x100 { c as u8 } else { b'?' }).take(width).collect();
        b[at..at + width].fill(0);
        b[at..at + bytes.len()].copy_from_slice(&bytes);
    };
    put(3, 30, &id3.title);
    put(33, 30, &id3.artist);
    put(63, 30, &id3.album);
    put(93, 4, &id3.year);
    match id3.track.split('/').next().unwrap_or("").trim().parse::<u8>() {
        Ok(n) if n > 0 => {
            b[125] = 0;
            b[126] = n;
        }
        _ if b[125] == 0 => b[126] = 0,
        _ => {}
    }
    b[127] = GENRES.iter().position(|g| g.eq_ignore_ascii_case(id3.genre.trim())).map_or(255, |n| n as u8);
}

#[cfg(test)]
mod tests;
