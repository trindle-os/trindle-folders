// The few facts about drives that the volume list needs: which mounts the system runs from, and UDisks values read plainly.

use std::collections::HashSet;

use crate::common::refused::{self, Refused};
use crate::toolkit::bus::{self as dbus, Props};
use crate::toolkit::t;

pub const SYSTEM_MOUNTS: &[&str] = &[
    "/", "/boot", "/boot/efi", "/efi", "/home", "/usr", "/var",
    "/run/live/medium", "/lib/live/mount/medium",
];

#[derive(Debug, PartialEq)]
pub struct Mounted {
    pub dir: String,
    pub major_minor: String,
    pub source: String,
}

pub fn system_mounts(mountinfo: &str) -> Vec<Mounted> {
    mountinfo
        .lines()
        .filter_map(|l| {
            let (left, right) = l.split_once(" - ")?;
            let f: Vec<&str> = left.split(' ').collect();
            let dir = unescape(f.get(4)?);
            let source = right.split(' ').nth(1).unwrap_or("").to_string();
            SYSTEM_MOUNTS.contains(&dir.as_str())
                .then(|| Mounted { dir, major_minor: f.get(2).unwrap_or(&"").to_string(), source })
        })
        .collect()
}

pub fn swap_devices(swaps: &str) -> Vec<String> {
    swaps
        .lines()
        .skip(1)
        .filter_map(|l| {
            let mut w = l.split_whitespace();
            let (dev, kind) = (w.next()?, w.next()?);
            (kind == "partition").then(|| unescape(dev))
        })
        .collect()
}

pub fn unescape(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 3 < b.len() && b[i + 1..i + 4].iter().all(|c| (b'0'..=b'7').contains(c)) {
            out.push((b[i + 1] - b'0') * 64 + (b[i + 2] - b'0') * 8 + (b[i + 3] - b'0'));
            i += 4;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

pub fn underneath(seeds: &[String], below: impl Fn(&str) -> Vec<String>) -> HashSet<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut todo: Vec<String> = seeds.to_vec();
    while let Some(d) = todo.pop() {
        if d.is_empty() || !seen.insert(d.clone()) {
            continue;
        }
        todo.extend(below(&d));
    }
    seen
}

fn busy(e: &str) -> bool {
    let l = e.to_lowercase();
    l.contains("target is busy") || l.contains("device is busy")
}

pub fn plain_error(e: &str) -> String {
    let l = e.to_lowercase();
    if busy(e) {
        t!("folders-volumes-busy").into()
    } else if let Some(heard) = refused::heard_udisks("", e) {
        refusal_words(heard.settled(|| None))
    } else if l.contains("no space left") || l.contains("too small") {
        t!("folders-volumes-no-room").into()
    } else {
        let e = e.trim();
        e.rsplit_once("exit code ").and_then(|(_, rest)| rest.split_once(": ")).map(|(_, why)| why.trim())
            .unwrap_or(e).to_string()
    }
}

fn refusal_words(r: Refused) -> String {
    match r {
        Refused::Cancelled => t!("folders-volumes-no-password").into(),
        other => t!("folders-volumes-nothing-changed", reason = other.sentence()),
    }
}

fn trim_nul(mut b: Vec<u8>) -> String {
    while b.last() == Some(&0) {
        b.pop();
    }
    String::from_utf8_lossy(&b).to_string()
}

pub fn text(p: &Props, k: &str) -> String {
    trim_nul(dbus::get_bytes(p, k))
}

pub fn mount_points(fs: &Props) -> Vec<String> {
    fs.get("MountPoints")
        .and_then(|v| v.try_clone().ok())
        .and_then(|v| Vec::<Vec<u8>>::try_from(v).ok())
        .unwrap_or_default()
        .into_iter()
        .map(trim_nul)
        .filter(|m| !m.is_empty())
        .collect()
}

pub fn num(p: &Props, k: &str) -> u64 {
    let Some(v) = p.get(k) else { return 0 };
    u64::try_from(v).ok()
        .or_else(|| i64::try_from(v).ok().map(|n| n.max(0) as u64))
        .or_else(|| u32::try_from(v).ok().map(u64::from))
        .or_else(|| i32::try_from(v).ok().map(|n| n.max(0) as u64))
        .or_else(|| u16::try_from(v).ok().map(u64::from))
        .or_else(|| u8::try_from(v).ok().map(u64::from))
        .unwrap_or(0)
}
