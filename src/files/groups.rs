//! Grouping what a folder holds under small headings: by kind, by when it

use super::places::Entry;
use crate::common::clock::Local;
use crate::toolkit::t;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GroupBy {
    #[default]
    None,
    Kind,
    Date,
    Size,
}

impl GroupBy {
    pub const ALL: [GroupBy; 4] = [GroupBy::None, GroupBy::Kind, GroupBy::Date, GroupBy::Size];

    pub fn word(self) -> &'static str {
        match self {
            GroupBy::None => t!("folders-groups-none"),
            GroupBy::Kind => t!("folders-groups-kind"),
            GroupBy::Date => t!("folders-groups-date"),
            GroupBy::Size => t!("folders-groups-size"),
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            GroupBy::None => "none",
            GroupBy::Kind => "kind",
            GroupBy::Date => "date",
            GroupBy::Size => "size",
        }
    }

    pub fn named(word: &str) -> Option<GroupBy> {
        GroupBy::ALL.into_iter().find(|g| g.key() == word)
    }
}

const MB: u64 = 1_000_000;
const LARGE: u64 = 100 * MB;

fn kinds() -> [&'static str; 8] {
    [t!("folders-groups-kind-folders"), t!("folders-groups-kind-documents"), t!("folders-groups-kind-pictures"), t!("folders-groups-kind-music"), t!("folders-groups-kind-videos"), t!("folders-groups-kind-archives"), t!("folders-groups-kind-programs"), t!("folders-groups-kind-other-files")]
}
fn dates() -> [&'static str; 7] {
    [t!("folders-groups-date-today"), t!("folders-groups-date-yesterday"), t!("folders-groups-date-earlier-this-week"), t!("folders-groups-date-last-week"), t!("folders-groups-date-earlier-this-month"), t!("folders-groups-date-earlier-this-year"), t!("folders-groups-date-long-ago")]
}
fn sizes() -> [&'static str; 5] {
    [t!("folders-groups-size-folders"), t!("folders-groups-size-large-over-100-mb"), t!("folders-groups-size-medium-1-mb-to-100-mb"), t!("folders-groups-size-small-under-1-mb"), t!("folders-groups-size-empty")]
}

pub fn rank(e: &Entry, by: GroupBy, now: Local) -> (usize, &'static str) {
    let at = match by {
        GroupBy::None => return (0, ""),
        GroupBy::Kind => kind_rank(e),
        GroupBy::Date => date_rank(Local::at(e.when), now),
        GroupBy::Size if e.folder => 0,
        GroupBy::Size => match e.size {
            0 => 4,
            s if s < MB => 3,
            s if s < LARGE => 2,
            _ => 1,
        },
    };
    (at, match by {
        GroupBy::Kind => kinds()[at],
        GroupBy::Date => dates()[at],
        _ => sizes()[at],
    })
}

fn kind_rank(e: &Entry) -> usize {
    kind_of(e.folder, e.kind.as_deref().unwrap_or(""), &e.path)
}

pub(super) fn kind_of(folder: bool, kind: &str, path: &std::path::Path) -> usize {
    if folder {
        return 0;
    }
    let is = |words: &[&str]| words.iter().any(|w| kind.contains(w));
    if kind.starts_with("image/") {
        2
    } else if kind.starts_with("audio/") {
        3
    } else if kind.starts_with("video/") {
        4
    } else if super::archive::Format::of(path).is_some() || is(&["zip", "x-tar", "x-7z", "rar", "x-xz", "x-bzip", "zstd", "x-cpio", "x-archive"]) {
        5
    } else if is(&["x-executable", "x-sharedlib", "x-shellscript", "x-ms-dos-executable", "x-msdownload", "appimage", "x-msi", "debian.binary-package", "x-rpm", "flatpak"]) {
        6
    } else if kind.starts_with("text/") || is(&["pdf", "msword", "officedocument", "opendocument", "rtf", "epub", "ms-excel", "ms-powerpoint", "postscript", "x-abiword"]) {
        1
    } else {
        7
    }
}

pub fn date_rank(when: Local, now: Local) -> usize {
    let (day, today) = (when.day_number(), now.day_number());
    let week_start = today - (today + 3).rem_euclid(7);
    if day >= today {
        0
    } else if day == today - 1 {
        1
    } else if day >= week_start {
        2
    } else if day >= week_start - 7 {
        3
    } else if (when.year(), when.month()) == (now.year(), now.month()) {
        4
    } else if when.year() == now.year() {
        5
    } else {
        6
    }
}

pub fn arrange(entries: &mut [Entry], by: GroupBy, now: Local) {
    if by != GroupBy::None {
        entries.sort_by_key(|e| rank(e, by, now).0);
    }
}

pub fn runs(entries: &[Entry], by: GroupBy, now: Local) -> Vec<(usize, usize, String)> {
    let mut out: Vec<(usize, usize, &'static str)> = Vec::new();
    if by == GroupBy::None {
        return Vec::new();
    }
    for (i, e) in entries.iter().enumerate() {
        let name = rank(e, by, now).1;
        match out.last_mut() {
            Some((_, len, last)) if *last == name => *len += 1,
            _ => out.push((i, 1, name)),
        }
    }
    out.into_iter().map(|(start, len, name)| (start, len, t!("folders-groups-heading", name = name, n = len))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn entry(name: &str, folder: bool, size: u64, when: i64, kind: Option<&str>) -> Entry {
        Entry { name: name.into(), path: PathBuf::from("/made-up").join(name), folder, size, when, kind: kind.map(str::to_string) }
    }

    const NOON: i64 = 1_791_979_200;
    const DAY: i64 = 86_400;

    #[test]
    fn dates_fall_into_groups_counted_from_today() {
        let now = Local::utc(NOON);
        assert_eq!((now.year(), now.month(), now.day()), (2026, 10, 14));
        let at = |secs: i64| dates()[date_rank(Local::utc(secs), now)];
        assert_eq!(at(NOON - 3 * 3600), "Today");
        assert_eq!(at(NOON + 5 * DAY), "Today", "dated ahead of now");
        assert_eq!(at(NOON - DAY), "Yesterday");
        assert_eq!(at(NOON - 2 * DAY), "Earlier this week");
        assert_eq!(at(NOON - 3 * DAY), "Last week");
        assert_eq!(at(NOON - 9 * DAY), "Last week");
        assert_eq!(at(NOON - 10 * DAY), "Earlier this month");
        assert_eq!(at(NOON - 13 * DAY), "Earlier this month");
        assert_eq!(at(NOON - 14 * DAY), "Earlier this year");
        assert_eq!(at(NOON - 400 * DAY), "Older");
        let monday = Local::utc(NOON - 2 * DAY);
        assert_eq!(dates()[date_rank(Local::utc(NOON - 3 * DAY), monday)], "Yesterday");
        assert_eq!(dates()[date_rank(Local::utc(NOON - 4 * DAY), monday)], "Last week");
    }

    #[test]
    fn things_fall_into_groups_by_kind_and_by_size() {
        let now = Local::utc(NOON);
        let kind = |e: &Entry| rank(e, GroupBy::Kind, now).1;
        assert_eq!(kind(&entry("Albums", true, 0, 0, None)), "Folders");
        assert_eq!(kind(&entry("cat.jpg", false, 9, 0, Some("image/jpeg"))), "Pictures");
        assert_eq!(kind(&entry("song.flac", false, 9, 0, Some("audio/flac"))), "Music");
        assert_eq!(kind(&entry("film.mkv", false, 9, 0, Some("video/x-matroska"))), "Videos");
        assert_eq!(kind(&entry("report.pdf", false, 9, 0, Some("application/pdf"))), "Documents");
        assert_eq!(kind(&entry("notes.txt", false, 9, 0, Some("text/plain"))), "Documents");
        assert_eq!(kind(&entry("sums.ods", false, 9, 0, Some("application/vnd.oasis.opendocument.spreadsheet"))), "Documents");
        assert_eq!(kind(&entry("old.tar.gz", false, 9, 0, Some("application/x-compressed-tar"))), "Archives");
        assert_eq!(kind(&entry("box.zip", false, 9, 0, Some("application/zip"))), "Archives");
        assert_eq!(kind(&entry("run.sh", false, 9, 0, Some("application/x-shellscript"))), "Programs");
        assert_eq!(kind(&entry("mystery", false, 9, 0, None)), "Other files");
        let size = |e: &Entry| rank(e, GroupBy::Size, now).1;
        assert_eq!(size(&entry("Albums", true, 0, 0, None)), "Folders");
        assert_eq!(size(&entry("empty", false, 0, 0, None)), "Empty");
        assert_eq!(size(&entry("small", false, 999_999, 0, None)), "Small (under 1 MB)");
        assert_eq!(size(&entry("medium", false, 1_000_000, 0, None)), "Medium (1 MB to 100 MB)");
        assert_eq!(size(&entry("large", false, 100_000_000, 0, None)), "Large (over 100 MB)");
        assert_eq!(rank(&entry("any", false, 5, 0, None), GroupBy::None, now), (0, ""));
    }

    #[test]
    fn groups_come_in_their_own_order_and_keep_the_sort_inside() {
        let now = Local::at(NOON);
        let mut list = vec![
            entry("Albums", true, 0, NOON, None), entry("Zoo", true, 0, NOON, None),
            entry("apple.jpg", false, 5, NOON - DAY, Some("image/jpeg")), entry("bread.txt", false, 0, NOON, Some("text/plain")),
            entry("cherry.png", false, 200 * MB, NOON - 400 * DAY, Some("image/png")), entry("damson.txt", false, 2 * MB, NOON - DAY, Some("text/plain")),
        ];
        let names = |l: &[Entry]| l.iter().map(|e| e.name.clone()).collect::<Vec<_>>();
        let heads = |l: &[Entry], by| runs(l, by, now).into_iter().map(|(start, len, words)| format!("{start}+{len} {words}")).collect::<Vec<_>>();
        arrange(&mut list, GroupBy::Kind, now);
        assert_eq!(names(&list), ["Albums", "Zoo", "bread.txt", "damson.txt", "apple.jpg", "cherry.png"]);
        assert_eq!(heads(&list, GroupBy::Kind), ["0+2 Folders, 2", "2+2 Documents, 2", "4+2 Pictures, 2"]);
        arrange(&mut list, GroupBy::Date, now);
        assert_eq!(names(&list), ["Albums", "Zoo", "bread.txt", "damson.txt", "apple.jpg", "cherry.png"]);
        assert_eq!(heads(&list, GroupBy::Date), ["0+3 Today, 3", "3+2 Yesterday, 2", "5+1 Older, 1"]);
        arrange(&mut list, GroupBy::Size, now);
        assert_eq!(names(&list), ["Albums", "Zoo", "cherry.png", "damson.txt", "apple.jpg", "bread.txt"]);
        assert_eq!(heads(&list, GroupBy::Size), ["0+2 Folders, 2", "2+1 Large (over 100 MB), 1", "3+1 Medium (1 MB to 100 MB), 1", "4+1 Small (under 1 MB), 1", "5+1 Empty, 1"]);
        let before = names(&list);
        arrange(&mut list, GroupBy::None, now);
        assert_eq!(names(&list), before);
        assert!(runs(&list, GroupBy::None, now).is_empty());
        for g in GroupBy::ALL {
            assert_eq!(GroupBy::named(g.key()), Some(g));
            assert!(!g.word().is_empty());
        }
    }
}
