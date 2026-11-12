//! Reading a photo's EXIF notes, and writing changes to them in JPEG and PNG.

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::os::unix::fs::FileExt;
use std::path::Path;

use super::{change, opened};
use crate::toolkit::t;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tag {
    pub name: String,
    pub value: String,
}

pub fn exif(path: &Path) -> Vec<Tag> {
    let Ok(file) = crate::common::files::open_file(path) else { return Vec::new() };
    let Ok(read) = exif::Reader::new().read_from_container(&mut BufReader::new(file)) else { return Vec::new() };
    read.fields()
        .map(|f| {
            Tag { name: f.tag.to_string(), value: f.display_value().with_unit(&read).to_string() }
        })
        .collect()
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExifEdit {
    pub taken: Option<String>,
    pub description: Option<String>,
    pub artist: Option<String>,
    pub copyright: Option<String>,
    pub orientation: Option<u16>,
    pub drop_location: bool,
}

const T_DESCRIPTION: u16 = 0x010e;
const T_ORIENTATION: u16 = 0x0112;
const T_ARTIST: u16 = 0x013b;
const T_THUMB: u16 = 0x0201;
const T_THUMB_LEN: u16 = 0x0202;
const T_COPYRIGHT: u16 = 0x8298;
const T_EXIF: u16 = 0x8769;
const T_GPS: u16 = 0x8825;
const T_VERSION: u16 = 0x9000;
const T_TAKEN: u16 = 0x9003;
const T_MAKER: u16 = 0x927c;
const T_XP_TITLE: u16 = 0x9c9b;
const T_INTEROP: u16 = 0xa005;
const T_LOOSE: [u16; 5] = [0x0111, 0x0117, 0x0144, 0x0145, 0x014a];

const PNG_SIGN: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];

fn exif_bad(what: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, t!("folders-meta-damaged", why = what))
}

fn exif_refused(what: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, what.to_string())
}

#[derive(Clone, Copy)]
struct Order {
    big: bool,
}

impl Order {
    fn u16(self, b: &[u8]) -> u16 {
        let b = [b[0], b[1]];
        if self.big {
            u16::from_be_bytes(b)
        } else {
            u16::from_le_bytes(b)
        }
    }

    fn u32(self, b: &[u8]) -> u32 {
        let b = [b[0], b[1], b[2], b[3]];
        if self.big {
            u32::from_be_bytes(b)
        } else {
            u32::from_le_bytes(b)
        }
    }

    fn put16(self, v: u16) -> [u8; 2] {
        if self.big {
            v.to_be_bytes()
        } else {
            v.to_le_bytes()
        }
    }

    fn put32(self, v: u32) -> [u8; 4] {
        if self.big {
            v.to_be_bytes()
        } else {
            v.to_le_bytes()
        }
    }
}

#[derive(Clone, Debug)]
struct Field {
    tag: u16,
    kind: u16,
    count: u32,
    data: Vec<u8>,
    at: Option<u32>,
}

impl Field {
    fn text(tag: u16, s: &str) -> Field {
        let mut data = s.as_bytes().to_vec();
        data.push(0);
        Field { tag, kind: 2, count: data.len() as u32, data, at: None }
    }

    fn long(o: Order, tag: u16, v: u32) -> Field {
        Field { tag, kind: 4, count: 1, data: o.put32(v).to_vec(), at: None }
    }
}

fn kind_size(kind: u16) -> Option<u32> {
    match kind {
        1 | 2 | 6 | 7 => Some(1),
        3 | 8 => Some(2),
        4 | 9 | 11 | 13 => Some(4),
        5 | 10 | 12 => Some(8),
        _ => None,
    }
}

struct Tiff {
    order: Order,
    ifd0: Vec<Field>,
    exif: Option<Vec<Field>>,
    gps: Option<Vec<Field>>,
    interop: Option<Vec<Field>>,
    ifd1: Option<Vec<Field>>,
    thumb: Option<Vec<u8>>,
}

fn read_ifd(t: &[u8], o: Order, at: u32) -> io::Result<(Vec<Field>, u32)> {
    let at = at as usize;
    let n = usize::from(o.u16(t.get(at..at + 2).ok_or_else(|| exif_bad(t!("folders-meta-bad-out-of-reach")))?));
    let end = at + 2 + 12 * n;
    if end > t.len() {
        return Err(exif_bad(t!("folders-meta-bad-block-past-end")));
    }
    let mut fields = Vec::with_capacity(n);
    for e in t[at + 2..end].chunks_exact(12) {
        let (tag, kind, count) = (o.u16(&e[0..]), o.u16(&e[2..]), o.u32(&e[4..]));
        let Some(len) = kind_size(kind).and_then(|s| s.checked_mul(count)) else { continue };
        if T_LOOSE.contains(&tag) {
            continue;
        }
        if len <= 4 {
            fields.push(Field { tag, kind, count, data: e[8..8 + len as usize].to_vec(), at: None });
        } else {
            let off = o.u32(&e[8..]);
            let Some(data) = t.get(off as usize..).and_then(|r| r.get(..len as usize)) else { continue };
            fields.push(Field { tag, kind, count, data: data.to_vec(), at: Some(off) });
        }
    }
    let next = t.get(end..end + 4).map_or(0, |b| o.u32(b));
    Ok((fields, next))
}

fn take_pointer(o: Order, fields: &mut Vec<Field>, tag: u16) -> Option<u32> {
    let i = fields.iter().position(|f| f.tag == tag)?;
    let f = fields.remove(i);
    (f.data.len() == 4).then(|| o.u32(&f.data))
}

fn parse_tiff(t: &[u8]) -> io::Result<Tiff> {
    let big = match t.get(..4) {
        Some(b"II*\0") if t.len() >= 8 => false,
        Some(b"MM\0*") if t.len() >= 8 => true,
        _ => return Err(exif_bad(t!("folders-meta-bad-no-tiff"))),
    };
    let o = Order { big };
    let (mut ifd0, next) = read_ifd(t, o, o.u32(&t[4..8]))?;
    let sub = |fields: &mut Vec<Field>, tag: u16| -> io::Result<Option<Vec<Field>>> {
        match take_pointer(o, fields, tag) {
            Some(at) => Ok(Some(read_ifd(t, o, at)?.0)),
            None => Ok(None),
        }
    };
    let mut exif = sub(&mut ifd0, T_EXIF)?;
    let gps = sub(&mut ifd0, T_GPS)?;
    let interop = match exif.as_mut() {
        Some(e) => sub(e, T_INTEROP)?,
        None => None,
    };
    let mut ifd1 = if next == 0 { None } else { read_ifd(t, o, next).ok().map(|r| r.0) };
    let mut thumb = None;
    if let Some(f) = ifd1.as_mut() {
        let len = f.iter().find(|f| f.tag == T_THUMB_LEN && f.data.len() == 4).map(|f| o.u32(&f.data) as usize);
        let at = take_pointer(o, f, T_THUMB).map(|a| a as usize);
        thumb = at.zip(len).and_then(|(a, l)| t.get(a..)?.get(..l)).map(<[u8]>::to_vec);
        if thumb.is_none() {
            f.retain(|f| f.tag != T_THUMB_LEN);
        }
    }
    Ok(Tiff { order: o, ifd0, exif, gps, interop, ifd1, thumb })
}

struct Out {
    o: Order,
    buf: Vec<u8>,
    keep: Option<(usize, Vec<u8>)>,
}

impl Out {
    fn put(&mut self, b: &[u8]) -> u32 {
        let mut at = self.buf.len().next_multiple_of(2);
        if let Some((start, note)) = &self.keep {
            if at < start + note.len() && at + b.len() > *start {
                at = (start + note.len()).next_multiple_of(2);
            }
        }
        self.buf.resize(at, 0);
        self.buf.extend_from_slice(b);
        at as u32
    }

    fn ifd(&mut self, fields: &[Field], next: u32) -> u32 {
        let mut fields: Vec<&Field> = fields.iter().collect();
        fields.sort_by_key(|f| f.tag);
        fields.dedup_by_key(|f| f.tag);
        fields.truncate(usize::from(u16::MAX));
        let o = self.o;
        let mut block = o.put16(fields.len() as u16).to_vec();
        for f in fields {
            block.extend(o.put16(f.tag));
            block.extend(o.put16(f.kind));
            block.extend(o.put32(f.count));
            if f.data.len() <= 4 {
                let mut v = f.data.clone();
                v.resize(4, 0);
                block.extend(v);
            } else if f.tag == T_MAKER && self.keep.as_ref().is_some_and(|k| k.1 == f.data) {
                let start = self.keep.as_ref().map_or(0, |k| k.0);
                block.extend(o.put32(start as u32));
            } else {
                let at = self.put(&f.data);
                block.extend(o.put32(at));
            }
        }
        block.extend(o.put32(next));
        self.put(&block)
    }
}

fn write_tiff(t: &Tiff) -> io::Result<Vec<u8>> {
    let o = t.order;
    let header = if o.big { *b"MM\0*" } else { *b"II*\0" };
    let keep = t.exif.as_ref().and_then(|e| e.iter().find(|f| f.tag == T_MAKER)).and_then(|f| Some((f.at? as usize, f.data.clone())));
    let keep = keep.filter(|k| k.0 >= 8);
    let mut out = Out { o, buf: [header, [0; 4]].concat(), keep };
    let mut ifd0 = t.ifd0.clone();
    if let Some(exif) = &t.exif {
        let mut exif = exif.clone();
        if let Some(interop) = &t.interop {
            let at = out.ifd(interop, 0);
            exif.push(Field::long(o, T_INTEROP, at));
        }
        let at = out.ifd(&exif, 0);
        ifd0.push(Field::long(o, T_EXIF, at));
    }
    if let Some(gps) = &t.gps {
        let at = out.ifd(gps, 0);
        ifd0.push(Field::long(o, T_GPS, at));
    }
    let mut next = 0;
    if let Some(ifd1) = &t.ifd1 {
        let mut ifd1 = ifd1.clone();
        if let Some(thumb) = &t.thumb {
            let at = out.put(thumb);
            ifd1.push(Field::long(o, T_THUMB, at));
        }
        next = out.ifd(&ifd1, 0);
    }
    let first = out.ifd(&ifd0, next);
    out.buf[4..8].copy_from_slice(&o.put32(first));
    if let Some((start, note)) = out.keep.take() {
        if out.buf.len() < start + note.len() {
            out.buf.resize(start + note.len(), 0);
        }
        out.buf[start..start + note.len()].copy_from_slice(&note);
    }
    if out.buf.len() > u32::MAX as usize {
        return Err(exif_bad(t!("folders-meta-bad-too-big")));
    }
    Ok(out.buf)
}

fn set_field(fields: &mut Vec<Field>, tag: u16, field: Option<Field>) {
    fields.retain(|f| f.tag != tag);
    fields.extend(field);
}

fn exif_date(s: &str) -> io::Result<String> {
    let b = s.trim().as_bytes();
    let shape = b.len() == 19
        && b.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == b':' || *c == b'-',
            10 => *c == b' ' || *c == b'T',
            13 | 16 => *c == b':',
            _ => c.is_ascii_digit(),
        });
    if !shape {
        return Err(exif_refused(t!("folders-meta-refuse-date")));
    }
    let mut out = b.to_vec();
    out[4] = b':';
    out[7] = b':';
    out[10] = b' ';
    Ok(String::from_utf8_lossy(&out).into_owned())
}

pub fn exif_upright(block: &[u8]) -> io::Result<Vec<u8>> {
    let mut t = parse_tiff(block)?;
    apply_edit(&mut t, &ExifEdit { orientation: Some(1), ..ExifEdit::default() })?;
    write_tiff(&t)
}

fn apply_edit(t: &mut Tiff, e: &ExifEdit) -> io::Result<()> {
    let o = t.order;
    let texts = [(T_DESCRIPTION, &e.description), (T_ARTIST, &e.artist), (T_COPYRIGHT, &e.copyright), (T_TAKEN, &e.taken)];
    if texts.iter().any(|(_, v)| v.as_deref().is_some_and(|s| s.contains('\0'))) {
        return Err(exif_refused(t!("folders-meta-refuse-nul")));
    }
    let taken = match e.taken.as_deref() {
        Some("") => Some(None),
        Some(s) => Some(Some(exif_date(s)?)),
        None => None,
    };
    if let Some(n) = e.orientation {
        if !(1..=8).contains(&n) {
            return Err(exif_refused(t!("folders-meta-refuse-way-up")));
        }
        set_field(&mut t.ifd0, T_ORIENTATION, Some(Field { tag: T_ORIENTATION, kind: 3, count: 1, data: o.put16(n).to_vec(), at: None }));
    }
    for (tag, v) in &texts[..3] {
        if let Some(v) = v {
            set_field(&mut t.ifd0, *tag, (!v.is_empty()).then(|| Field::text(*tag, v)));
        }
    }
    if let Some(d) = &e.description {
        if t.ifd0.iter().any(|f| f.tag == T_XP_TITLE) {
            let data: Vec<u8> = d.encode_utf16().chain([0]).flat_map(u16::to_le_bytes).collect();
            let field = Field { tag: T_XP_TITLE, kind: 1, count: data.len() as u32, data, at: None };
            set_field(&mut t.ifd0, T_XP_TITLE, (!d.is_empty()).then_some(field));
        }
    }
    if let Some(taken) = taken {
        if taken.is_some() && t.exif.is_none() {
            t.exif = Some(vec![Field { tag: T_VERSION, kind: 7, count: 4, data: b"0232".to_vec(), at: None }]);
        }
        if let Some(exif) = t.exif.as_mut() {
            set_field(exif, T_TAKEN, taken.map(|s| Field::text(T_TAKEN, &s)));
        }
    }
    if e.drop_location {
        t.gps = None;
    }
    Ok(())
}

struct Spot {
    png: bool,
    from: u64,
    to: u64,
    tiff: Option<Vec<u8>>,
}

fn jpeg_spot(file: &File, len: u64) -> io::Result<Spot> {
    let (mut pos, mut after_app0, mut first) = (2u64, 2u64, true);
    while pos + 4 <= len {
        let mut b = [0u8; 4];
        file.read_exact_at(&mut b, pos)?;
        if b[0] != 0xff {
            return Err(exif_bad(t!("folders-meta-bad-jpeg-parts")));
        }
        let marker = b[1];
        if marker == 0xff {
            pos += 1;
            continue;
        }
        if marker == 0xda || marker == 0xd9 {
            break;
        }
        if marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
            pos += 2;
            continue;
        }
        let size = u64::from(u16::from_be_bytes([b[2], b[3]]));
        if size < 2 || pos + 2 + size > len {
            return Err(exif_bad(t!("folders-meta-bad-jpeg-past-end")));
        }
        if marker == 0xe1 && size >= 8 {
            let mut p = vec![0; size as usize - 2];
            file.read_exact_at(&mut p, pos + 4)?;
            if p.starts_with(b"Exif\0") {
                return Ok(Spot { png: false, from: pos, to: pos + 2 + size, tiff: Some(p.split_off(6)) });
            }
        }
        if first && marker == 0xe0 {
            after_app0 = pos + 2 + size;
        }
        first = false;
        pos += 2 + size;
    }
    Ok(Spot { png: false, from: after_app0, to: after_app0, tiff: None })
}

fn png_spot(file: &File, len: u64) -> io::Result<Spot> {
    let mut pos = 8u64;
    while pos + 12 <= len {
        let mut b = [0u8; 8];
        file.read_exact_at(&mut b, pos)?;
        let size = u64::from(u32::from_be_bytes([b[0], b[1], b[2], b[3]]));
        if size > i32::MAX as u64 || pos + 12 + size > len {
            return Err(exif_bad(t!("folders-meta-bad-png-past-end")));
        }
        match &b[4..8] {
            b"eXIf" => {
                let mut data = vec![0; size as usize];
                file.read_exact_at(&mut data, pos + 8)?;
                return Ok(Spot { png: true, from: pos, to: pos + 12 + size, tiff: Some(data) });
            }
            b"IDAT" | b"IEND" => return Ok(Spot { png: true, from: pos, to: pos, tiff: None }),
            _ => pos += 12 + size,
        }
    }
    Err(exif_bad(t!("folders-meta-bad-png-empty")))
}

pub fn write_exif(path: &Path, edit: &ExifEdit) -> io::Result<()> {
    let file = opened(path)?;
    let len = file.metadata()?.len();
    let mut head = [0u8; 8];
    let got = file.read_at(&mut head, 0)?;
    let spot = if got >= 3 && head[..3] == [0xff, 0xd8, 0xff] {
        jpeg_spot(&file, len)?
    } else if got == 8 && head == PNG_SIGN {
        png_spot(&file, len)?
    } else {
        return Err(io::Error::new(io::ErrorKind::Unsupported, t!("folders-meta-only-jpeg-png")));
    };
    let mut tiff = match &spot.tiff {
        Some(old) => parse_tiff(old)?,
        None => Tiff { order: Order { big: false }, ifd0: Vec::new(), exif: None, gps: None, interop: None, ifd1: None, thumb: None },
    };
    apply_edit(&mut tiff, edit)?;
    let tiff = write_tiff(&tiff)?;
    let mut part = Vec::with_capacity(tiff.len() + 12);
    if spot.png {
        part.extend((tiff.len() as u32).to_be_bytes());
        part.extend(b"eXIf");
        part.extend(&tiff);
        let mut crc = flate2::Crc::new();
        crc.update(&part[4..]);
        part.extend(crc.sum().to_be_bytes());
    } else {
        let size = u16::try_from(tiff.len() + 8).map_err(|_| exif_refused(t!("folders-meta-refuse-too-long")))?;
        part.extend([0xff, 0xe1]);
        part.extend(size.to_be_bytes());
        part.extend(b"Exif\0\0");
        part.extend(&tiff);
    }
    change(path, spot.from + part.len() as u64 + len.saturating_sub(spot.to), &mut |out| {
        let mut old = &file;
        old.seek(SeekFrom::Start(0))?;
        io::copy(&mut Read::take(old, spot.from), out)?;
        out.write_all(&part)?;
        old.seek(SeekFrom::Start(spot.to))?;
        io::copy(&mut old, out)?;
        Ok(())
    })
}

#[cfg(test)]
mod tests;
