// A file's checksums, as websites print them beside a download so it can be checked.

use std::io::{self, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use md5::Md5;
use sha2::{Digest, Sha256, Sha512};

use crate::common::gentle;

pub const KINDS: [&str; 4] = ["MD5", "SHA-1", "SHA-256", "SHA-512"];
const CHUNK: usize = 1 << 20;

pub fn work_out(path: &Path, stop: &AtomicBool, tell: &mut dyn FnMut(f32)) -> io::Result<Vec<(&'static str, String)>> {
    let mut file = gentle::In::new(crate::common::files::open_file(path)?);
    let size = file.file().metadata()?.len().max(1);
    let (mut md5, mut sha256, mut sha512) = (Md5::new(), Sha256::new(), Sha512::new());
    let mut sha1 = ring::digest::Context::new(&ring::digest::SHA1_FOR_LEGACY_USE_ONLY);
    let mut buf = vec![0u8; CHUNK];
    let mut read = 0u64;
    loop {
        if stop.load(Ordering::Relaxed) {
            return Err(io::Error::from(io::ErrorKind::Interrupted));
        }
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let part = &buf[..n];
        md5.update(part);
        sha1.update(part);
        sha256.update(part);
        sha512.update(part);
        read += n as u64;
        tell(gentle::share(read, size, None, read >= size));
    }
    Ok(vec![
        (KINDS[0], hex(&md5.finalize())),
        (KINDS[1], hex(sha1.finish().as_ref())),
        (KINDS[2], hex(&sha256.finalize())),
        (KINDS[3], hex(&sha512.finalize())),
    ])
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn matching<'a>(sums: &'a [(&'static str, String)], pasted: &str) -> Option<&'a str> {
    let word = pasted.split_whitespace().find(|w| w.len() >= 32).unwrap_or(pasted.trim());
    let word = word.rsplit_once(['=', ':']).map_or(word, |(_, after)| after);
    let wanted: String = word.chars().filter(char::is_ascii_hexdigit).collect::<String>().to_ascii_lowercase();
    if wanted.len() < 32 {
        return None;
    }
    sums.iter().find(|(_, sum)| *sum == wanted).map(|(kind, _)| *kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_usual_checksums_come_out_right() {
        let dir = std::env::temp_dir().join(format!("trindle-checksum-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("abc.txt");
        std::fs::write(&file, b"abc").unwrap();
        let mut last = 0.0;
        let sums = work_out(&file, &AtomicBool::new(false), &mut |f| last = f).unwrap();
        assert_eq!(sums[0].1, "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(sums[1].1, "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert_eq!(sums[2].1, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert!(sums[3].1.starts_with("ddaf35a193617abacc417349ae204131"));
        assert_eq!(last, 1.0);
        assert!(work_out(&file, &AtomicBool::new(true), &mut |_| {}).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_pasted_checksum_is_matched_however_it_is_written() {
        let sums = vec![("MD5", "900150983cd24fb0d6963f7d28e17f72".to_string()),
                        ("SHA-256", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".to_string())];
        assert_eq!(matching(&sums, " 900150983CD24FB0D6963F7D28E17F72 "), Some("MD5"));
        assert_eq!(matching(&sums, "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"), Some("SHA-256"));
        assert_eq!(matching(&sums, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  trindle.iso"), Some("SHA-256"));
        assert_eq!(matching(&sums, "SHA256 (abc.txt) = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"), Some("SHA-256"));
        assert_eq!(matching(&sums, "900150983cd24fb0d6963f7d28e17f73"), None);
        assert_eq!(matching(&sums, "abc"), None);
    }
}
