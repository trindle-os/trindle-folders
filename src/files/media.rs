//! What the List view's music, photo and PDF columns show about a file.

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};

use super::app::{Files, Msg};
use super::places::Where;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Media {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track: Option<String>,
    pub length: Option<u64>,
    pub camera: Option<String>,
    pub dimensions: Option<(u32, u32)>,
    pub pages: Option<u32>,
}

#[derive(Default)]
pub struct Reading {
    ask: Option<Sender<(PathBuf, i64)>>,
    asked: HashSet<(PathBuf, i64)>,
}

impl Files {
    pub(super) fn media_wanted(&mut self) {
        let mut want = Vec::new();
        for (p, pane) in self.cols.iter().enumerate() {
            if !self.shown_pane(p) || matches!(pane.at, Where::Remote(_) | Where::Inside(..)) {
                continue;
            }
            if !super::view::shown_columns(self, pane, p).iter().any(|c| c.media()) {
                continue;
            }
            for e in pane.entries.iter().filter(|e| !e.folder && has_any(&e.path)) {
                let key = (e.path.clone(), e.when);
                if self.cache.media(e).is_none() && !self.media.asked.contains(&key) {
                    want.push(key);
                }
            }
        }
        for key in want {
            self.media.asked.insert(key.clone());
            let Some(post) = self.post.clone() else {
                let m = read(&key.0);
                self.media_read(key.0, key.1, m);
                continue;
            };
            let ask = self.media.ask.get_or_insert_with(|| {
                let (tx, rx) = channel::<(PathBuf, i64)>();
                let _ = std::thread::Builder::new().name("folders-media".into()).spawn(move || {
                    for (path, when) in rx {
                        let m = read(&path);
                        post.send(Msg::Media(path, when, m));
                    }
                });
                tx
            });
            let _ = ask.send(key);
        }
    }

    pub(super) fn media_read(&mut self, path: PathBuf, when: i64, m: Media) {
        self.media.asked.remove(&(path.clone(), when));
        self.cache.set_media(path, when, m);
        if self.media.asked.is_empty() && self.by.is_some_and(|c| c.media()) {
            self.resort_all();
        }
    }
}

pub fn has_any(path: &Path) -> bool {
    !matches!(kind(path), Kind::None)
}

enum Kind {
    Mp3,
    Flac,
    Ogg,
    Photo,
    Picture,
    Pdf,
    None,
}

fn kind(path: &Path) -> Kind {
    let end = path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).unwrap_or_default();
    match end.as_str() {
        "mp3" => Kind::Mp3,
        "flac" => Kind::Flac,
        "ogg" | "oga" | "opus" => Kind::Ogg,
        "jpg" | "jpeg" | "tif" | "tiff" | "webp" => Kind::Photo,
        "png" | "gif" | "bmp" => Kind::Picture,
        "pdf" => Kind::Pdf,
        _ => Kind::None,
    }
}

fn dimensions(path: &Path) -> Option<(u32, u32)> {
    crate::toolkit::picture::reader(path).ok()?.into_dimensions().ok()
}

pub fn read(path: &Path) -> Media {
    let got = match kind(path) {
        Kind::Mp3 => mp3(path),
        Kind::Flac => flac(path),
        Kind::Ogg => ogg(path),
        Kind::Photo => Ok(Media { camera: camera(path), dimensions: dimensions(path), ..Media::default() }),
        Kind::Picture => Ok(Media { dimensions: dimensions(path), ..Media::default() }),
        Kind::Pdf => Ok(Media { pages: pdf_pages(path), ..Media::default() }),
        Kind::None => Ok(Media::default()),
    };
    got.unwrap_or_default()
}

pub fn length_words(secs: u64) -> String {
    match secs {
        s if s >= 3600 => format!("{}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60),
        s => format!("{}:{:02}", s / 60, s % 60),
    }
}

fn some(text: &str) -> Option<String> {
    let t = text.trim();
    (!t.is_empty()).then(|| t.to_string())
}

fn camera(path: &Path) -> Option<String> {
    let file = crate::common::files::open_file(path).ok()?;
    let exif = exif::Reader::new().read_from_container(&mut BufReader::new(file)).ok()?;
    let text = |tag| exif.get_field(tag, exif::In::PRIMARY).map(|f| f.display_value().to_string().trim_matches(['"', ' ']).to_string());
    let make = text(exif::Tag::Make).unwrap_or_default();
    let model = text(exif::Tag::Model).unwrap_or_default();
    let first = make.split_whitespace().next().unwrap_or("");
    if model.to_lowercase().starts_with(&first.to_lowercase()) {
        some(&model)
    } else {
        some(&format!("{make} {model}"))
    }
}

fn mp3(path: &Path) -> io::Result<Media> {
    let mut m = Media::default();
    if let Some(id3) = super::meta::read_id3(path)? {
        m.title = some(&id3.title);
        m.artist = some(&id3.artist);
        m.album = some(&id3.album);
        m.track = some(&id3.track);
    }
    m.length = mp3_length(path).ok().flatten();
    Ok(m)
}

fn mp3_length(path: &Path) -> io::Result<Option<u64>> {
    let mut file = crate::common::files::open_file(path)?;
    let size = file.metadata()?.len();
    let mut head = [0u8; 10];
    file.read_exact(&mut head)?;
    let mut start = 0u64;
    if &head[..3] == b"ID3" {
        let n = head[6..10].iter().fold(0u64, |a, &b| (a << 7) | u64::from(b & 0x7f));
        start = 10 + n + if head[5] & 0x10 != 0 { 10 } else { 0 };
    }
    file.seek(SeekFrom::Start(start))?;
    let mut buf = vec![0u8; 64 * 1024];
    let got = read_up_to(&mut file, &mut buf)?;
    Ok(mp3_frames(&buf[..got], size.saturating_sub(start)))
}

fn mp3_frames(buf: &[u8], audio: u64) -> Option<u64> {
    const BITRATES: [[u32; 15]; 2] = [
        [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320],
        [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160],
    ];
    let at = buf.windows(4).position(|w| {
        w[0] == 0xff && w[1] & 0xe0 == 0xe0 && (w[1] >> 1) & 3 == 1 && (w[2] >> 4) != 15 && (w[2] >> 4) != 0 && (w[2] >> 2) & 3 != 3
    })?;
    let h = &buf[at..];
    let version = (h[1] >> 3) & 3;
    let mpeg1 = version == 3;
    let rate = [44100u32, 48000, 32000][usize::from((h[2] >> 2) & 3)] / match version { 3 => 1, 2 => 2, _ => 4 };
    let bitrate = BITRATES[usize::from(!mpeg1)][usize::from(h[2] >> 4)] * 1000;
    let mono = h[3] >> 6 == 3;
    let per_frame = if mpeg1 { 1152 } else { 576 };
    let side = match (mpeg1, mono) {
        (true, false) => 32,
        (true, true) => 17,
        (false, false) => 17,
        (false, true) => 9,
    };
    let be32 = |b: &[u8]| u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
    let xing = h.get(4 + side..4 + side + 12).filter(|x| &x[..4] == b"Xing" || &x[..4] == b"Info");
    let frames = match (xing, h.get(36..36 + 18)) {
        (Some(x), _) if be32(&x[4..8]) & 1 != 0 => Some(be32(&x[8..12])),
        (_, Some(v)) if &v[..4] == b"VBRI" => Some(be32(&v[14..18])),
        _ => None,
    };
    match frames {
        Some(n) => Some(u64::from(n) * per_frame / u64::from(rate)),
        None if bitrate > 0 => Some(audio * 8 / u64::from(bitrate)),
        None => None,
    }
}

fn flac(path: &Path) -> io::Result<Media> {
    let mut file = BufReader::new(crate::common::files::open_file(path)?);
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    if &magic != b"fLaC" {
        return Ok(Media::default());
    }
    let mut m = Media::default();
    loop {
        let mut head = [0u8; 4];
        file.read_exact(&mut head)?;
        let (last, kind) = (head[0] & 0x80 != 0, head[0] & 0x7f);
        let len = u32::from_be_bytes([0, head[1], head[2], head[3]]) as usize;
        match kind {
            0 | 4 => {
                let mut block = vec![0u8; len];
                file.read_exact(&mut block)?;
                if kind == 0 && len >= 18 {
                    let rate = (u32::from(block[10]) << 12) | (u32::from(block[11]) << 4) | u32::from(block[12] >> 4);
                    let samples = (u64::from(block[13] & 0x0f) << 32) | u64::from(u32::from_be_bytes([block[14], block[15], block[16], block[17]]));
                    if rate > 0 && samples > 0 {
                        m.length = Some(samples / u64::from(rate));
                    }
                } else if kind == 4 {
                    comments(&block, &mut m);
                }
            }
            _ => {
                file.seek_relative(len as i64)?;
            }
        }
        if last {
            break;
        }
    }
    Ok(m)
}

fn comments(b: &[u8], m: &mut Media) {
    let le = |at: usize| b.get(at..at + 4).map(|x| u32::from_le_bytes([x[0], x[1], x[2], x[3]]) as usize);
    let Some(vendor) = le(0) else { return };
    let mut at = 4 + vendor;
    let Some(count) = le(at) else { return };
    at += 4;
    for _ in 0..count.min(10_000) {
        let Some(n) = le(at) else { return };
        let Some(line) = b.get(at + 4..at + 4 + n) else { return };
        at += 4 + n;
        let line = String::from_utf8_lossy(line);
        let Some((key, value)) = line.split_once('=') else { continue };
        let slot = match key.to_ascii_uppercase().as_str() {
            "TITLE" => &mut m.title,
            "ARTIST" => &mut m.artist,
            "ALBUM" => &mut m.album,
            "TRACKNUMBER" => &mut m.track,
            _ => continue,
        };
        if slot.is_none() {
            *slot = some(value);
        }
    }
}

fn ogg(path: &Path) -> io::Result<Media> {
    let mut file = crate::common::files::open_file(path)?;
    let size = file.metadata()?.len();
    let mut buf = vec![0u8; 512 * 1024];
    let got = read_up_to(&mut file, &mut buf)?;
    let packets = ogg_packets(&buf[..got], 2);
    let mut m = Media::default();
    let (rate, skip) = match packets.first() {
        Some(p) if p.starts_with(b"\x01vorbis") && p.len() >= 16 => (u32::from_le_bytes([p[12], p[13], p[14], p[15]]), 0),
        Some(p) if p.starts_with(b"OpusHead") && p.len() >= 12 => (48_000, u64::from(u16::from_le_bytes([p[10], p[11]]))),
        _ => return Ok(m),
    };
    if let Some(p) = packets.get(1) {
        if let Some(rest) = p.strip_prefix(b"\x03vorbis").or_else(|| p.strip_prefix(b"OpusTags")) {
            comments(rest, &mut m);
        }
    }
    let tail = size.min(64 * 1024);
    file.seek(SeekFrom::Start(size - tail))?;
    let mut end = vec![0u8; tail as usize];
    file.read_exact(&mut end)?;
    if let Some(at) = end.windows(4).rposition(|w| w == b"OggS") {
        if let Some(g) = end.get(at + 6..at + 14) {
            let granule = u64::from_le_bytes([g[0], g[1], g[2], g[3], g[4], g[5], g[6], g[7]]);
            if rate > 0 && granule != u64::MAX {
                m.length = Some(granule.saturating_sub(skip) / u64::from(rate));
            }
        }
    }
    Ok(m)
}

fn ogg_packets(buf: &[u8], most: usize) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut now = Vec::new();
    let mut at = 0;
    while out.len() < most && buf.get(at..at + 4) == Some(b"OggS") {
        let Some(&count) = buf.get(at + 26) else { break };
        let table = at + 27;
        let Some(sizes) = buf.get(table..table + usize::from(count)) else { break };
        let mut data = table + usize::from(count);
        for &s in sizes {
            let s = usize::from(s);
            let Some(part) = buf.get(data..data + s) else { return out };
            now.extend_from_slice(part);
            data += s;
            if s < 255 {
                out.push(std::mem::take(&mut now));
                if out.len() == most {
                    return out;
                }
            }
        }
        at = data;
    }
    out
}

const PDF_WHOLE: u64 = 16 << 20;
const PDF_PART: u64 = 4 << 20;
const PDF_INFLATED: u64 = 32 << 20;
const PDF_BACK: usize = 4096;

fn pdf_pages(path: &Path) -> Option<u32> {
    let mut file = crate::common::files::open_file(path).ok()?;
    let size = file.metadata().ok()?.len();
    let mut bytes = Vec::new();
    if size <= PDF_WHOLE {
        file.read_to_end(&mut bytes).ok()?;
    } else {
        (&mut file).take(PDF_PART).read_to_end(&mut bytes).ok()?;
        file.seek(SeekFrom::Start(size - PDF_PART)).ok()?;
        file.read_to_end(&mut bytes).ok()?;
    }
    if !bytes.starts_with(b"%PDF") {
        return None;
    }
    let mut most = pages_count(&bytes);
    object_streams(&bytes, &mut |stream| most = most.max(pages_count(stream)));
    most.filter(|&n| n > 0)
}

fn pages_count(b: &[u8]) -> Option<u32> {
    let mut best = None;
    let mut from = 0;
    while let Some(i) = find(&b[from..], b"/Type") {
        let at = from + i + 5;
        from = at;
        let rest = skip_space(&b[at..]);
        if !rest.starts_with(b"/Pages") || rest.get(6).is_some_and(|c| c.is_ascii_alphanumeric()) {
            continue;
        }
        let lo = at.saturating_sub(PDF_BACK);
        let open = b[lo..at].windows(2).rposition(|w| w == b"<<").map_or(lo, |p| lo + p);
        let close = find(&b[at..], b">>").map_or(b.len(), |e| at + e);
        let dict = &b[open..close];
        if let Some(c) = find(dict, b"/Count") {
            let digits: Vec<u8> = skip_space(&dict[c + 6..]).iter().copied().take_while(u8::is_ascii_digit).collect();
            if let Some(n) = std::str::from_utf8(&digits).ok().and_then(|s| s.parse::<u32>().ok()) {
                best = Some(best.map_or(n, |m: u32| m.max(n)));
            }
        }
    }
    best
}

fn object_streams(b: &[u8], each: &mut dyn FnMut(&[u8])) -> u64 {
    let mut total = 0;
    let mut from = 0;
    while let Some(i) = find(&b[from..], b"/ObjStm") {
        let at = from + i;
        let Some(s) = find(&b[at..], b"stream") else { break };
        let mut data = at + s + 6;
        if b.get(data) == Some(&b'\r') {
            data += 1;
        }
        if b.get(data) == Some(&b'\n') {
            data += 1;
        }
        let end = find(&b[data..], b"endstream").map_or(b.len(), |e| data + e);
        let left = PDF_INFLATED - total;
        if left == 0 {
            break;
        }
        let mut inflated = Vec::new();
        if flate2::read::ZlibDecoder::new(&b[data..end]).take(PDF_PART.min(left)).read_to_end(&mut inflated).is_ok() || !inflated.is_empty() {
            total += inflated.len() as u64;
            each(&inflated);
        }
        from = end;
    }
    total
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn skip_space(b: &[u8]) -> &[u8] {
    let n = b.iter().take_while(|c| c.is_ascii_whitespace()).count();
    &b[n..]
}

fn read_up_to(file: &mut File, buf: &mut [u8]) -> io::Result<usize> {
    let mut got = 0;
    while got < buf.len() {
        match file.read(&mut buf[got..])? {
            0 => break,
            n => got += n,
        }
    }
    Ok(got)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn dir(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("trindle-media-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn comment_block(lines: &[&str]) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&5u32.to_le_bytes());
        b.extend_from_slice(b"maker");
        b.extend_from_slice(&(lines.len() as u32).to_le_bytes());
        for l in lines {
            b.extend_from_slice(&(l.len() as u32).to_le_bytes());
            b.extend_from_slice(l.as_bytes());
        }
        b
    }

    #[test]
    fn a_flac_gives_its_comments_and_length() {
        let d = dir("flac");
        let mut f = b"fLaC".to_vec();
        let mut info = vec![0u8; 34];
        let rate = 44100u32;
        info[10] = (rate >> 12) as u8;
        info[11] = (rate >> 4) as u8;
        info[12] = ((rate & 0xf) << 4) as u8 | 0x02;
        info[14..18].copy_from_slice(&441_000u32.to_be_bytes());
        f.extend_from_slice(&[0, 0, 0, 34]);
        f.extend_from_slice(&info);
        let c = comment_block(&["TITLE=Morning", "artist=Jo Bloggs", "ALBUM=Days", "TRACKNUMBER=3"]);
        f.push(0x84);
        f.extend_from_slice(&(c.len() as u32).to_be_bytes()[1..]);
        f.extend_from_slice(&c);
        std::fs::write(d.join("song.flac"), &f).unwrap();
        let m = read(&d.join("song.flac"));
        assert_eq!((m.title.as_deref(), m.artist.as_deref(), m.album.as_deref(), m.track.as_deref()),
                   (Some("Morning"), Some("Jo Bloggs"), Some("Days"), Some("3")));
        assert_eq!(m.length, Some(10));
        std::fs::remove_dir_all(&d).unwrap();
    }

    fn page(granule: u64, packets: &[&[u8]]) -> Vec<u8> {
        let mut p = b"OggS\0\0".to_vec();
        p.extend_from_slice(&granule.to_le_bytes());
        p.extend_from_slice(&[0; 12]);
        let mut table = Vec::new();
        for pk in packets {
            let mut n = pk.len();
            while n >= 255 {
                table.push(255);
                n -= 255;
            }
            table.push(n as u8);
        }
        p.push(table.len() as u8);
        p.extend_from_slice(&table);
        for pk in packets {
            p.extend_from_slice(pk);
        }
        p
    }

    #[test]
    fn an_opus_file_gives_its_comments_and_length() {
        let d = dir("opus");
        let mut head = b"OpusHead\x01\x02".to_vec();
        head.extend_from_slice(&312u16.to_le_bytes());
        head.extend_from_slice(&48000u32.to_le_bytes());
        head.extend_from_slice(&[0, 0, 0]);
        let mut tags = b"OpusTags".to_vec();
        let long = format!("COMMENT={}", "x".repeat(400));
        tags.extend_from_slice(&comment_block(&["TITLE=Evening", &long, "ARTIST=Jo"]));
        let mut f = page(0, &[&head]);
        f.extend(page(0, &[&tags]));
        f.extend(page(48000 * 125 + 312, &[b"audio"]));
        std::fs::write(d.join("talk.opus"), &f).unwrap();
        let m = read(&d.join("talk.opus"));
        assert_eq!((m.title.as_deref(), m.artist.as_deref(), m.length), (Some("Evening"), Some("Jo"), Some(125)));
        assert_eq!(length_words(125), "2:05");
        assert_eq!(length_words(3725), "1:02:05");
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn an_mp3_gives_its_tag_and_length() {
        let d = dir("mp3");
        let path = d.join("tune.mp3");
        let mut f = Vec::new();
        let mut frame = vec![0xff, 0xfb, 0x90, 0x00];
        frame.resize(4 + 32, 0);
        frame.extend_from_slice(b"Xing");
        frame.extend_from_slice(&1u32.to_be_bytes());
        frame.extend_from_slice(&1000u32.to_be_bytes());
        frame.resize(417, 0);
        f.extend_from_slice(&frame);
        std::fs::write(&path, &f).unwrap();
        let id3 = super::super::meta::Id3 { title: "Tune".into(), artist: "Band".into(), track: "7".into(), ..Default::default() };
        super::super::meta::write_id3(&path, &id3).unwrap();
        let m = read(&path);
        assert_eq!((m.title.as_deref(), m.artist.as_deref(), m.track.as_deref()), (Some("Tune"), Some("Band"), Some("7")));
        assert_eq!(m.length, Some(26));
        assert_eq!(mp3_frames(&[0xff, 0xfb, 0x90, 0x00, 0, 0], 1_600_000), Some(100));
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_pdf_counts_its_pages_plain_or_packed() {
        let d = dir("pdf");
        let plain = b"%PDF-1.4\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R 4 0 R] /Count 12 >> endobj\n3 0 obj << /Type /Pages /Count 5 /Parent 2 0 R >> endobj\n4 0 obj << /Type /Page /Parent 2 0 R >> endobj\n%%EOF\n";
        std::fs::write(d.join("plain.pdf"), plain).unwrap();
        assert_eq!(read(&d.join("plain.pdf")).pages, Some(12));
        let inner = b"2 0 3 40 << /Type/Pages /Count 31 /Kids [] >> << /Type /Page >>";
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        z.write_all(inner).unwrap();
        let packed = z.finish().unwrap();
        let mut f = b"%PDF-1.5\n5 0 obj << /Type /ObjStm /N 2 /First 8 /Filter /FlateDecode /Length 99 >>\nstream\n".to_vec();
        f.extend_from_slice(&packed);
        f.extend_from_slice(b"\nendstream\nendobj\n%%EOF\n");
        std::fs::write(d.join("packed.pdf"), &f).unwrap();
        assert_eq!(read(&d.join("packed.pdf")).pages, Some(31));
        std::fs::write(d.join("fake.pdf"), b"not a pdf").unwrap();
        assert_eq!(read(&d.join("fake.pdf")).pages, None);
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        z.write_all(&vec![b' '; PDF_PART as usize]).unwrap();
        let swell = z.finish().unwrap();
        let mut f = b"%PDF-1.5\n".to_vec();
        for _ in 0..40 {
            f.extend_from_slice(b"1 0 obj << /Type /ObjStm >>\nstream\n");
            f.extend_from_slice(&swell);
            f.extend_from_slice(b"\nendstream\nendobj\n");
        }
        let mut held = 0;
        let total = object_streams(&f, &mut |s| held = held.max(s.len()));
        assert!(total <= PDF_INFLATED && held as u64 <= PDF_PART, "{total} inflated");
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn the_list_view_shows_what_is_read_inside_files() {
        use crate::toolkit::ui::App;
        use super::super::columns::Col;
        let d = dir("columns");
        image::DynamicImage::new_rgb8(30, 20).save(d.join("a.png")).unwrap();
        std::fs::write(d.join("notes.txt"), "words").unwrap();
        let mut f = Files::new(vec![d.clone()]);
        f.go(0, Where::Folder(d.clone()));
        f.view = super::super::app::View::List;
        f.kept.column(Col::Type, false).unwrap();
        f.kept.column(Col::Dimensions, true).unwrap();
        f.cols[0].tabs[0].width.set(900.0);
        f.update(Msg::Reload);
        let pane = &f.cols[0].tabs[0];
        let cells = |name: &str| {
            let e = pane.entries.iter().find(|e| e.name == name).unwrap();
            f.cache.cells(e, &[Col::Dimensions])
        };
        assert_eq!(cells("a.png"), ["30 \u{d7} 20"]);
        assert_eq!(cells("notes.txt"), [""]);
        let mut f = Files::new(vec![d.clone()]);
        f.go(1, Where::Folder(d.clone()));
        f.go(0, Where::Recent);
        f.view = super::super::app::View::List;
        f.kept.column(Col::Dimensions, true).unwrap();
        f.cols[1].tabs[0].width.set(900.0);
        f.update(Msg::Panes);
        f.side = 1;
        f.cols[1].tabs[0].width.set(900.0);
        f.update(Msg::Reload);
        let e = f.cols[1].tabs[0].entries.iter().find(|e| e.name == "a.png").unwrap().clone();
        assert_eq!(f.cache.cells(&e, &[Col::Dimensions]), ["30 \u{d7} 20"]);
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_picture_gives_its_size() {
        let d = dir("picture");
        image::DynamicImage::new_rgb8(30, 20).save(d.join("a.png")).unwrap();
        assert_eq!(read(&d.join("a.png")).dimensions, Some((30, 20)));
        assert!(has_any(&d.join("a.png")) && !has_any(&d.join("a.txt")));
        std::fs::remove_dir_all(&d).unwrap();
    }
}
