// Reading the snapshots of the home folder, so that an earlier version of a file can be opened.

use std::path::{Path, PathBuf};

#[cfg(not(test))]
pub const HOME: &str = "/home";

#[cfg(not(test))]
pub const SNAPSHOTS: &str = "/home/.snapshots";

pub const DESCRIPTION: &str = "trindle-home";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kept {
    pub number: u32,
    pub taken: i64,
    pub ours: bool,
}

pub fn unix_of(date: &str) -> Option<i64> {
    let num = |r: std::ops::Range<usize>| date.get(r)?.parse::<i64>().ok();
    let (y, m, d, h, mi, s) = (num(0..4)?, num(5..7)?, num(8..10)?, num(11..13)?, num(14..16)?, num(17..19)?);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || h > 23 || mi > 59 || s > 60 {
        return None;
    }
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some((era * 146_097 + doe - 719_468) * 86_400 + h * 3600 + mi * 60 + s)
}

pub fn kept_in(dir: &Path) -> Vec<Kept> {
    let mut out: Vec<Kept> = std::fs::read_dir(dir).map(|d| d.flatten().filter_map(|e| {
        let number: u32 = e.file_name().to_str()?.parse().ok().filter(|n| *n > 0)?;
        let base = e.path();
        if !std::fs::symlink_metadata(base.join("snapshot")).is_ok_and(|m| m.is_dir()) {
            return None;
        }
        let info = std::fs::read_to_string(base.join("info.xml")).unwrap_or_default();
        Some(Kept { number, taken: unix_of(&xml_field(&info, "date")).unwrap_or(0),
                    ours: xml_field(&info, "description") == DESCRIPTION })
    }).collect()).unwrap_or_default();
    out.sort_by_key(|k| k.number);
    out
}

pub fn tree(snapshots: &Path, number: u32) -> PathBuf {
    snapshots.join(number.to_string()).join("snapshot")
}

fn xml_field(xml: &str, tag: &str) -> String {
    let open = format!("<{tag}>");
    xml.split(&open).nth(1).and_then(|r| r.split(&format!("</{tag}>")).next()).unwrap_or("")
        .replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'")
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn snappers_dates_are_read() {
        assert_eq!(unix_of("1970-01-01 00:00:00"), Some(0));
        assert_eq!(unix_of("2026-10-04 09:34:12"), Some(1_791_106_452));
        assert_eq!(unix_of("2024-02-29 12:00:00"), Some(1_709_208_000));
        assert_eq!(unix_of("2026-13-04 09:34:12"), None);
        assert_eq!(unix_of("yesterday"), None);
        assert_eq!(unix_of(""), None);
    }

    pub(crate) fn made_up_snapshots(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-home-snapshots-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    pub(crate) fn add_snapshot(dir: &Path, number: u32, date: &str, description: &str) -> PathBuf {
        let base = dir.join(number.to_string());
        std::fs::create_dir_all(base.join("snapshot")).unwrap();
        std::fs::write(base.join("info.xml"), format!(
            "<?xml version=\"1.0\"?>\n<snapshot>\n  <type>single</type>\n  <num>{number}</num>\n  \
             <date>{date}</date>\n  <description>{description}</description>\n  <cleanup>number</cleanup>\n</snapshot>\n")).unwrap();
        base.join("snapshot")
    }

    #[test]
    fn the_snapshots_are_read_from_their_folder() {
        let dir = made_up_snapshots("read");
        add_snapshot(&dir, 2, "2026-10-04 09:34:12", DESCRIPTION);
        add_snapshot(&dir, 10, "2026-10-05 09:34:12", "by hand");
        add_snapshot(&dir, 1, "2026-10-03 09:34:12", DESCRIPTION);
        std::fs::create_dir_all(dir.join("7")).unwrap();
        std::fs::create_dir_all(dir.join("notes/snapshot")).unwrap();
        std::fs::create_dir_all(dir.join("0/snapshot")).unwrap();
        let kept = kept_in(&dir);
        assert_eq!(kept.iter().map(|k| k.number).collect::<Vec<_>>(), [1, 2, 10]);
        assert_eq!(kept[1], Kept { number: 2, taken: 1_791_106_452, ours: true });
        assert!(!kept[2].ours);
        assert!(kept_in(&dir.join("none")).is_empty());
        assert_eq!(tree(&dir, 2), dir.join("2/snapshot"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
