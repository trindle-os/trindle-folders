// Archives of every kind made by hand, for the tests.

use crate::files::unarchive::rar::RAR4_MARK;

pub type Thing<'a> = (&'a [u8], u32, &'a [u8]);

pub fn cpio(things: &[Thing]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut add = |name: &[u8], mode: u32, data: &[u8]| {
        let fields = [0, mode, 0, 0, 1, 1_700_000_000, data.len() as u32, 0, 0, 1, 3, name.len() as u32 + 1, 0];
        out.extend_from_slice(b"070701");
        for field in fields {
            out.extend_from_slice(format!("{field:08x}").as_bytes());
        }
        out.extend_from_slice(name);
        out.push(0);
        while out.len() % 4 != 0 {
            out.push(0);
        }
        out.extend_from_slice(data);
        while out.len() % 4 != 0 {
            out.push(0);
        }
    };
    for (name, mode, data) in things {
        add(name, *mode, data);
    }
    add(b"TRAILER!!!", 0, b"");
    out
}

pub fn ar(things: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = b"!<arch>\n".to_vec();
    for (name, data) in things {
        out.extend_from_slice(format!("{:<16}{:<12}{:<6}{:<6}{:<8}{:<10}`\n", format!("{name}/"), 1_700_000_000, 0, 0, "100644", data.len()).as_bytes());
        out.extend_from_slice(data);
        if data.len() % 2 == 1 {
            out.push(b'\n');
        }
    }
    out
}

pub fn zstd(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x28, 0xb5, 0x2f, 0xfd, 0xa0];
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    let blocks: Vec<&[u8]> = if data.is_empty() { vec![data] } else { data.chunks(1 << 16).collect() };
    for (i, block) in blocks.iter().enumerate() {
        let last = u32::from(i + 1 == blocks.len());
        out.extend_from_slice(&(last | ((block.len() as u32) << 3)).to_le_bytes()[..3]);
        out.extend_from_slice(block);
    }
    out
}

fn xxh32_short(bytes: &[u8]) -> u32 {
    let mut h = 374_761_393u32.wrapping_add(bytes.len() as u32);
    for &b in bytes {
        h = h.wrapping_add(u32::from(b).wrapping_mul(374_761_393)).rotate_left(11).wrapping_mul(2_654_435_761);
    }
    h ^= h >> 15;
    h = h.wrapping_mul(2_246_822_519);
    h ^= h >> 13;
    h = h.wrapping_mul(3_266_489_917);
    h ^ (h >> 16)
}

fn lz4_block(d: &[u8]) -> Vec<u8> {
    fn long(mut n: usize, out: &mut Vec<u8>) {
        while n >= 255 {
            out.push(255);
            n -= 255;
        }
        out.push(n as u8);
    }
    fn part(out: &mut Vec<u8>, plain: &[u8], back: Option<(usize, usize)>) {
        let run = back.map_or(0, |(_, len)| len - 4);
        out.push(((plain.len().min(15) as u8) << 4) | run.min(15) as u8);
        if plain.len() >= 15 {
            long(plain.len() - 15, out);
        }
        out.extend_from_slice(plain);
        if let Some((far, _)) = back {
            out.extend_from_slice(&(far as u16).to_le_bytes());
            if run >= 15 {
                long(run - 15, out);
            }
        }
    }
    let mut out = Vec::new();
    let mut seen: std::collections::HashMap<[u8; 4], usize> = Default::default();
    let (mut i, mut from) = (0, 0);
    while i + 12 < d.len() {
        let four = [d[i], d[i + 1], d[i + 2], d[i + 3]];
        if let Some(j) = seen.insert(four, i).filter(|j| i - j <= 0xffff) {
            let mut len = 4;
            while i + len < d.len() - 5 && d[j + len] == d[i + len] {
                len += 1;
            }
            part(&mut out, &d[from..i], Some((i - j, len)));
            i += len;
            from = i;
        } else {
            i += 1;
        }
    }
    part(&mut out, &d[from..], None);
    out
}

pub fn lz4(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x04, 0x22, 0x4d, 0x18, 0x60, 0x40];
    out.push((xxh32_short(&out[4..6]) >> 8) as u8);
    assert_eq!(out[6], 0x82, "the check every lz4 frame that starts this way carries");
    for chunk in data.chunks(60_000) {
        let block = lz4_block(chunk);
        if block.len() < chunk.len() {
            out.extend_from_slice(&(block.len() as u32).to_le_bytes());
            out.extend_from_slice(&block);
        } else {
            out.extend_from_slice(&(chunk.len() as u32 | 0x8000_0000).to_le_bytes());
            out.extend_from_slice(chunk);
        }
    }
    out.extend_from_slice(&[0; 4]);
    out
}

pub fn lzma_alone(data: &[u8]) -> Vec<u8> {
    use std::io::Write;
    let options = lzma_rust2::LzmaOptions::with_preset(1);
    let mut w = lzma_rust2::LzmaWriter::new_use_header(Vec::new(), &options, Some(data.len() as u64)).unwrap();
    w.write_all(data).unwrap();
    w.finish().unwrap()
}

pub fn lzip(data: &[u8]) -> Vec<u8> {
    use std::io::Write;
    let mut options = lzma_rust2::LzmaOptions::with_preset(1);
    options.dict_size = 1 << 16;
    let mut w = lzma_rust2::LzmaWriter::new_no_header(Vec::new(), &options, true).unwrap();
    assert_eq!(w.props(), 0x5d, "lzip is always LZMA with these settings");
    w.write_all(data).unwrap();
    let squeezed = w.finish().unwrap();
    let mut out = b"LZIP\x01\x10".to_vec();
    out.extend_from_slice(&squeezed);
    let mut crc = flate2::Crc::new();
    crc.update(data);
    out.extend_from_slice(&crc.sum().to_le_bytes());
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    out.extend_from_slice(&(out.len() as u64 + 8).to_le_bytes());
    out
}

pub fn compress_z(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x1f, 0x9d, 0x90];
    let mut runs: std::collections::HashMap<(u32, u8), u32> = Default::default();
    let (mut next, mut bits, mut held, mut have) = (257u32, 9u32, 0u64, 0u32);
    let mut run: Option<u32> = None;
    let mut put = |code: u32, bits: u32, out: &mut Vec<u8>| {
        held |= u64::from(code) << have;
        have += bits;
        while have >= 8 {
            out.push(held as u8);
            held >>= 8;
            have -= 8;
        }
        have
    };
    for &b in data {
        let Some(so_far) = run else {
            run = Some(u32::from(b));
            continue;
        };
        if let Some(&code) = runs.get(&(so_far, b)) {
            run = Some(code);
            continue;
        }
        put(so_far, bits, &mut out);
        assert!(next < 1 << 16, "too much for this small writer");
        runs.insert((so_far, b), next);
        next += 1;
        if next > 1 << bits && bits < 16 {
            bits += 1;
        }
        run = Some(u32::from(b));
    }
    if let Some(last) = run {
        put(last, bits, &mut out);
    }
    if put(0, 0, &mut out) > 0 {
        put(0, 8, &mut out);
    }
    out
}

const DOS_DATE: u16 = ((2023 - 1980) << 9) | (11 << 5) | 14;
const DOS_TIME: u16 = (22 << 11) | (13 << 5) | 10;

pub fn cab(files: &[(&str, &[u8])]) -> Vec<u8> {
    use std::io::Write;
    let all: Vec<u8> = files.iter().flat_map(|(_, d)| d.iter().copied()).collect();
    assert!(all.len() < 32_768, "one block holds no more");
    let mut z = flate2::write::DeflateEncoder::new(b"CK".to_vec(), flate2::Compression::default());
    z.write_all(&all).unwrap();
    let packed = z.finish().unwrap();
    let mut list = Vec::new();
    let mut at = 0u32;
    for (name, data) in files {
        list.extend_from_slice(&(data.len() as u32).to_le_bytes());
        list.extend_from_slice(&at.to_le_bytes());
        for v in [0u16, DOS_DATE, DOS_TIME, 0x20 | 0x80] {
            list.extend_from_slice(&v.to_le_bytes());
        }
        list.extend_from_slice(name.replace('/', "\\").as_bytes());
        list.push(0);
        at += data.len() as u32;
    }
    let (files_at, data_at) = (36 + 8, 36 + 8 + list.len() as u32);
    let mut out = b"MSCF".to_vec();
    for v in [0, data_at + 8 + packed.len() as u32, 0, files_at, 0] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(&[3, 1]);
    for v in [1u16, files.len() as u16, 0, 0x1234, 0] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(&data_at.to_le_bytes());
    out.extend_from_slice(&[1, 0, 1, 0]);
    out.extend_from_slice(&list);
    let mut sizes = (packed.len() as u16).to_le_bytes().to_vec();
    sizes.extend_from_slice(&(all.len() as u16).to_le_bytes());
    out.extend_from_slice(&cab_sum(&sizes, cab_sum(&packed, 0)).to_le_bytes());
    out.extend_from_slice(&sizes);
    out.extend_from_slice(&packed);
    out
}

fn cab_sum(bytes: &[u8], seed: u32) -> u32 {
    let fours = bytes.chunks_exact(4);
    let odd = fours.remainder().iter().fold(0u32, |sum, &b| (sum << 8) | u32::from(b));
    fours.fold(seed, |sum, f| sum ^ u32::from_le_bytes([f[0], f[1], f[2], f[3]])) ^ odd
}

fn crc16(data: &[u8]) -> u16 {
    let mut crc = 0u16;
    for &b in data {
        crc ^= u16::from(b);
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xa001 } else { crc >> 1 };
        }
    }
    crc
}

fn lh5(data: &[u8]) -> Vec<u8> {
    assert!(!data.is_empty() && data.len() < 65_536, "one block holds no more");
    let (mut out, mut held, mut have) = (Vec::new(), 0u64, 0u32);
    let mut put = |value: u32, bits: u32, out: &mut Vec<u8>| {
        held = (held << bits) | u64::from(value);
        have += bits;
        while have >= 8 {
            out.push((held >> (have - 8)) as u8);
            have -= 8;
        }
        held &= (1 << have) - 1;
        have
    };
    for (value, bits) in [(data.len() as u32, 16), (0, 5), (10, 5), (256, 9), (0, 4), (0, 4)] {
        put(value, bits, &mut out);
    }
    for &b in data {
        put(u32::from(b), 8, &mut out);
    }
    let left = put(0, 0, &mut out);
    if left > 0 {
        put(0, 8 - left, &mut out);
    }
    out
}

pub fn lha(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, (name, data)) in files.iter().enumerate() {
        let (method, packed) = if i == 0 || data.is_empty() { (b"-lh0-", data.to_vec()) } else { (b"-lh5-", lh5(data)) };
        let name = name.replace('/', "\\");
        let mut head = method.to_vec();
        head.extend_from_slice(&(packed.len() as u32).to_le_bytes());
        head.extend_from_slice(&(data.len() as u32).to_le_bytes());
        head.extend_from_slice(&DOS_TIME.to_le_bytes());
        head.extend_from_slice(&DOS_DATE.to_le_bytes());
        head.extend_from_slice(&[0x20, 0, name.len() as u8]);
        head.extend_from_slice(name.as_bytes());
        head.extend_from_slice(&crc16(data).to_le_bytes());
        out.push(head.len() as u8);
        out.push(head.iter().fold(0u8, |sum, b| sum.wrapping_add(*b)));
        out.extend_from_slice(&head);
        out.extend_from_slice(&packed);
    }
    out.push(0);
    out
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn xar(things: &[Thing]) -> Vec<u8> {
    use std::io::Write;
    fn zlib(data: &[u8]) -> Vec<u8> {
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        z.write_all(data).unwrap();
        z.finish().unwrap()
    }
    fn sha1(data: &[u8]) -> String {
        hex(ring::digest::digest(&ring::digest::SHA1_FOR_LEGACY_USE_ONLY, data).as_ref())
    }
    fn escaped(name: &str) -> String {
        name.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    }
    fn under(things: &[Thing], above: &[u8], heap: &mut Vec<u8>, toc: &mut String) {
        for (n, &(path, mode, data)) in things.iter().enumerate() {
            let (parent, name) = match path.iter().rposition(|&b| b == b'/') {
                Some(i) => (&path[..i], &path[i + 1..]),
                None => (&path[..0], path),
            };
            let top = path.starts_with(b"../") || path.starts_with(b"/");
            if (top && !above.is_empty()) || (!top && parent != above) {
                continue;
            }
            let name = String::from_utf8_lossy(if top { path } else { name });
            let folder = mode & 0o170_000 == 0o040_000;
            toc.push_str(&format!("<file id=\"{}\"><name>{}</name><type>{}</type><mode>{:04o}</mode>", n + 1, escaped(&name),
                                  if folder { "directory" } else { "file" }, mode & 0o7777));
            toc.push_str("<uid>0</uid><user>root</user><gid>0</gid><group>root</group><mtime>2023-11-14T22:13:20Z</mtime>");
            if folder {
                under(things, path, heap, toc);
            } else {
                let packed = zlib(data);
                toc.push_str(&format!("<data><length>{}</length><offset>{}</offset><size>{}</size><encoding style=\"application/x-gzip\"/>\
                                       <extracted-checksum style=\"sha1\">{}</extracted-checksum><archived-checksum style=\"sha1\">{}</archived-checksum></data>",
                                      packed.len(), heap.len(), data.len(), sha1(data), sha1(&packed)));
                heap.extend_from_slice(&packed);
            }
            toc.push_str("</file>");
        }
    }
    let mut heap = Vec::new();
    let mut toc = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<xar><toc>");
    under(things, b"", &mut heap, &mut toc);
    toc.push_str("</toc></xar>\n");
    let packed = zlib(toc.as_bytes());
    let mut out = b"xar!".to_vec();
    out.extend_from_slice(&[0, 28, 0, 1]);
    out.extend_from_slice(&(packed.len() as u64).to_be_bytes());
    out.extend_from_slice(&(toc.len() as u64).to_be_bytes());
    out.extend_from_slice(&[0; 4]);
    out.extend_from_slice(&packed);
    out.extend_from_slice(&heap);
    out
}

pub fn rpm(things: &[Thing]) -> Vec<u8> {
    use std::io::Write;
    let part = |tag: u32, text: &[u8]| {
        let mut p = vec![0x8e, 0xad, 0xe8, 0x01, 0, 0, 0, 0];
        for v in [1, text.len() as u32, tag, 6, 0, 1] {
            p.extend_from_slice(&v.to_be_bytes());
        }
        p.extend_from_slice(text);
        p
    };
    let mut out = vec![0xed, 0xab, 0xee, 0xdb, 3, 0, 0, 0, 0, 1];
    let mut name = [0u8; 66];
    name[..11].copy_from_slice(b"album-1.0-1");
    out.extend_from_slice(&name);
    out.extend_from_slice(&[0, 1, 0, 5]);
    out.extend_from_slice(&[0; 16]);
    out.extend_from_slice(&part(1000, b"none\0"));
    while out.len() % 8 != 0 {
        out.push(0);
    }
    out.extend_from_slice(&part(1000, b"album\0"));
    let mut z = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    z.write_all(&cpio(things)).unwrap();
    out.extend_from_slice(&z.finish().unwrap());
    out
}

#[test]
fn the_writers_of_test_archives_write_what_their_formats_say() {
    assert_eq!(xxh32_short(b""), 0x02cc_5d05);
    assert_eq!(crc16(b"123456789"), 0xbb3d);
    assert_eq!(compress_z(b"aaa"), [0x1f, 0x9d, 0x90, 0x61, 0x02, 0x02]);
    let noughts = lz4_block(&[0; 105]);
    assert_eq!(noughts[0], 0x1f, "one byte as it is, then a long run");
    assert!(noughts.len() < 20, "{noughts:?}");
}

fn vint(mut n: u64, out: &mut Vec<u8>) {
    loop {
        let b = (n & 0x7f) as u8;
        n >>= 7;
        out.push(if n > 0 { b | 0x80 } else { b });
        if n == 0 {
            break;
        }
    }
}

pub fn rar5(things: &[Thing]) -> Vec<u8> {
    let mut out = b"Rar!\x1a\x07\x01\x00".to_vec();
    let mut block = |body: &[u8], data: &[u8]| {
        let mut sized = Vec::new();
        vint(body.len() as u64, &mut sized);
        sized.extend_from_slice(body);
        let mut crc = flate2::Crc::new();
        crc.update(&sized);
        out.extend_from_slice(&crc.sum().to_le_bytes());
        out.extend_from_slice(&sized);
        out.extend_from_slice(data);
    };
    block(&[1, 0, 0], b"");
    for (name, mode, data) in things {
        let folder = mode & 0o170_000 == 0o040_000;
        let mut body = Vec::new();
        vint(2, &mut body);
        vint(2, &mut body);
        vint(data.len() as u64, &mut body);
        vint(if folder { 1 | 2 } else { 2 | 4 }, &mut body);
        vint(data.len() as u64, &mut body);
        vint(u64::from(*mode), &mut body);
        body.extend_from_slice(&1_700_000_000u32.to_le_bytes());
        if !folder {
            let mut crc = flate2::Crc::new();
            crc.update(data);
            body.extend_from_slice(&crc.sum().to_le_bytes());
        }
        vint(0, &mut body);
        vint(1, &mut body);
        vint(name.len() as u64, &mut body);
        body.extend_from_slice(name);
        block(&body, data);
    }
    block(&[5, 0, 0], b"");
    out
}

pub fn rar4_split(name: &[u8], data: &[u8], sizes: &[usize]) -> Vec<Vec<u8>> {
    let sum = |bytes: &[u8]| {
        let mut crc = flate2::Crc::new();
        crc.update(bytes);
        crc.sum()
    };
    let block = |kind: u8, flags: u16, body: &[u8], after: &[u8]| {
        let mut rest = vec![kind];
        rest.extend_from_slice(&flags.to_le_bytes());
        rest.extend_from_slice(&(7 + body.len() as u16).to_le_bytes());
        rest.extend_from_slice(body);
        let mut out = (sum(&rest) as u16).to_le_bytes().to_vec();
        out.extend_from_slice(&rest);
        out.extend_from_slice(after);
        out
    };
    let mut cuts: Vec<&[u8]> = Vec::new();
    let mut left = data;
    for size in sizes {
        let (now, later) = left.split_at(*size);
        cuts.push(now);
        left = later;
    }
    cuts.push(left);
    let last = cuts.len() - 1;
    cuts.iter().enumerate().map(|(i, part)| {
        let mut out = RAR4_MARK.to_vec();
        out.extend(block(0x73, 1 | if i == 0 { 0x100 } else { 0 }, &[0; 6], b""));
        let mut body = Vec::new();
        body.extend_from_slice(&(part.len() as u32).to_le_bytes());
        body.extend_from_slice(&(data.len() as u32).to_le_bytes());
        body.push(3);
        body.extend_from_slice(&sum(if i == last { data } else { part }).to_le_bytes());
        body.extend_from_slice(&0x5c22_1862u32.to_le_bytes());
        body.extend_from_slice(&[29, 0x30]);
        body.extend_from_slice(&(name.len() as u16).to_le_bytes());
        body.extend_from_slice(&0o100_644u32.to_le_bytes());
        body.extend_from_slice(name);
        let split = if i > 0 { 1 } else { 0 } | if i < last { 2 } else { 0 };
        out.extend(block(0x74, 0x8000 | split, &body, part));
        out.extend(block(0x7b, 0x4000 | u16::from(i < last), b"", b""));
        out
    }).collect()
}
