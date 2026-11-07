// Long disk work in Folders goes through the gentle rule, read from the code itself.

use crate::common::no_writing_in_place::{all_code, code};

const LONG: &[&str] = &[
    "files/act/mod.rs", "files/act/acts.rs", "files/act/drop.rs", "files/act/jobs.rs", "files/act/msg.rs", "files/act/steps.rs", "files/act/text.rs", "files/act/types.rs", "files/act/undo.rs", "files/act/window.rs", "files/dialogs.rs", "toolkit/common/file_work/work/bin.rs", "toolkit/common/file_work/work/copy.rs", "toolkit/common/file_work/work/crew.rs", "toolkit/common/file_work/work/finish.rs", "toolkit/common/file_work/work/leave.rs", "toolkit/common/file_work/work/looks.rs", "toolkit/common/file_work/work/mod.rs", "toolkit/common/file_work/work/plan.rs", "toolkit/common/file_work/work/run.rs", "toolkit/common/file_work/work/said.rs", "toolkit/common/file_work/work/side.rs", "toolkit/common/file_work/work/undo.rs", "toolkit/common/file_work/work/walk.rs", "files/far/mod.rs", "files/far/card.rs", "files/far/copy.rs", "files/far/ends.rs", "files/far/hands.rs", "files/far/links.rs", "files/far/msg.rs", "files/far/put.rs", "files/far/walk.rs", "files/far/window.rs", "files/jobs.rs",
    "files/archive/mod.rs", "files/archive/pack.rs", "files/archive/trouble.rs", "files/archive/unpack.rs", "files/archive/walk.rs",
    "files/archive/sevenz/header.rs", "files/archive/sevenz/lock.rs", "files/archive/sevenz/mod.rs", "files/archive/sevenz/unpack.rs",
    "files/archive/zip/dir.rs", "files/archive/zip/dos.rs", "files/archive/zip/lock.rs", "files/archive/zip/mod.rs", "files/archive/zip/write.rs",
    "files/unarchive/child.rs", "files/unarchive/frames.rs", "files/unarchive/mod.rs", "files/unarchive/rar.rs", "files/unarchive/run.rs",
    "files/wipe.rs", "files/checksum.rs", "files/dupes.rs", "files/queue.rs", "files/index.rs", "files/search.rs", "files/thumbs.rs",
];

const STARTS: [&str; 3] = ["thread::spawn(", "thread::Builder", "s.spawn("];

const NOT_DISK: &[(&str, usize, &str)] = &[
    ("files/act/acts.rs", 2, "the clipboard read and written: none reads or writes files at length"),
    ("files/act/jobs.rs", 1, "the wait before a finished row goes, which reads or writes no file"),
    ("files/act/window.rs", 1, "a notice sent, which only waits and reads or writes no file"),
    ("files/unarchive/run.rs", 1, "watches the switch that stops the walled reader, and reads nothing"),
    ("files/index.rs", 1, "listens on the bus for the person being away, and reads no file"),
    ("files/far/window.rs", 1, "one look at a server for the window, which the person is waiting for"),
];

const POURS: [&str; 4] = ["io::copy(", "fs::copy(", "copy_file_range(", "sendfile("];

const SHORT: &[(&str, usize, &str)] = &[
    ("files/archive/mod.rs", 1, "reads the start of one locked file to nowhere, to try a password"),
    ("files/archive/sevenz/unpack.rs", 1, "reads what is passed over in a block to nowhere"),
    ("files/archive/walk.rs", 1, "reads one squeezed file to nowhere, to count it"),
    ("files/archive/zip/write.rs", 3, "read into an archive being made, which is written out as it grows (Tide)"),
    ("files/meta/photo.rs", 2, "one photo, written again with its notes changed"),
    ("files/meta/song.rs", 1, "one song, written again with its notes changed"),
    ("files/unarchive/run.rs", 1, "reads what is left of one thing to nowhere"),
];

const PROGRAMS: [&str; 7] = ["\"tar\"", "\"rsync\"", "\"cp\"", "\"dd\"", "\"sha256sum\"", "\"unzip\"", "\"dpkg-deb\""];

const STARTED: &[(&str, usize, &str)] = &[
    ("files/kinds.rs", 1, "a kind of file's name, and nothing started"),
];

const SHOWN: &[(&str, &str)] = &[
    ("files/progress.rs", "files/act/types.rs"),
    ("files/tools.rs", "files/tools.rs"),
    ("files/dialogs.rs", "files/checksum.rs"),
];

const NOT_WORK: &[(&str, &str)] = &[
    ("settings/panels/about.rs", "how full the disk is"),
    ("ring/battery_alert.rs", "how full the battery is"),
    ("store/app_page.rs", "what people thought of an app"),
];

fn said(line: &str) -> &str {
    line.split("//").next().unwrap_or(line)
}

fn lines_with(text: &str, any: &[&str]) -> usize {
    code(text).into_iter().map(said).filter(|l| any.iter().any(|a| l.contains(a))).count()
}

#[test]
fn long_disk_work_is_started_only_through_the_gentle_rule() {
    let files = all_code();
    let mut wrong = Vec::new();
    let find = |name: &str| files.iter().find(|(f, _)| f == name).map(|(_, t)| t.as_str());
    for name in LONG {
        let Some(text) = find(name) else {
            wrong.push(format!("{name} is listed in LONG and is not there"));
            continue;
        };
        let plain = lines_with(text, &STARTS);
        let allowed = NOT_DISK.iter().find(|(f, ..)| f == name).map_or(0, |(_, n, _)| *n);
        if plain != allowed {
            wrong.push(format!("{name} starts {plain} threads that are not gentle, and {allowed} are listed in NOT_DISK"));
        }
    }
    for (name, _) in files.iter().filter(|(f, _)| f.starts_with("toolkit/common/file_work/work/") && !LONG.contains(&f.as_str())) {
        wrong.push(format!("{name} is part of the file work and is not listed in LONG"));
    }
    for (name, text) in files.iter().filter(|(f, _)| f != "toolkit/common/file_work/gentle/mod.rs") {
        let pours = lines_with(text, &POURS);
        let listed = SHORT.iter().find(|(f, ..)| f == name).map_or(0, |(_, n, _)| *n);
        if pours != listed {
            wrong.push(format!("{name} moves bytes in bulk on {pours} lines outside common::gentle, and {listed} are listed in SHORT"));
        }
        let programs = lines_with(text, &PROGRAMS);
        let listed = STARTED.iter().find(|(f, ..)| f == name).map_or(0, |(_, n, _)| *n);
        if programs != listed {
            wrong.push(format!("{name} names a program that does long disk work on {programs} lines, and {listed} are listed in STARTED"));
        }
        let rings = lines_with(text, &["standard_ring(Some(", "standard_ring_with(Some("]);
        let shown = SHOWN.iter().find(|(f, _)| f == name);
        if rings > 0 && shown.is_none() && !NOT_WORK.iter().any(|(f, _)| f == name) && !name.starts_with("toolkit/") {
            wrong.push(format!("{name} shows a ring that fills and is in neither SHOWN nor NOT_WORK"));
        }
        if let Some((_, by)) = shown {
            if !find(by).is_some_and(|t| t.contains("gentle::share(")) {
                wrong.push(format!("{name} shows how far disk work has got, and {by} does not work it out with gentle::share"));
            }
        }
    }
    for listed in SHORT.iter().map(|s| s.0).chain(STARTED.iter().map(|s| s.0)).chain(SHOWN.iter().map(|s| s.0)).chain(NOT_DISK.iter().map(|s| s.0)) {
        if find(listed).is_none() {
            wrong.push(format!("{listed} is listed and is not there"));
        }
    }
    let said_why = SHORT.iter().all(|s| s.1 > 0 && !s.2.is_empty()) && NOT_DISK.iter().all(|s| s.1 > 0 && !s.2.is_empty()) && STARTED.iter().all(|s| s.1 > 0 && !s.2.is_empty());
    assert!(said_why, "each one listed says how many and why");
    assert!(files.len() > 100, "Folders' code was not found");
    assert!(wrong.is_empty(), "long disk work goes through common::gentle (see the top of that file): {wrong:#?}");
}

const OUTS: [&str; 12] = ["settle(", "all_out(", "all_out_of(", "folder_out(", "sync_all(", "sync_data(", "syncfs(", "fsync(", "fdatasync(", "written(", "finish_now(", ".flush()"];

const THROWS: [&str; 3] = ["let _ =", ".ok();", "drop("];

const WIRES: [&str; 2] = ["conn.flush()", "queue.flush()"];

const THROWN: &[(&str, usize, &str)] = &[
    ("files/remote/tls.rs", 1, "the goodbye of a secure link; whether the upload is whole is what the server answers next, and that is carried"),
];

fn thrown(text: &str) -> usize {
    code(text).into_iter().map(said).filter(|l| OUTS.iter().any(|o| l.contains(o)) && THROWS.iter().any(|t| l.contains(t)) && !WIRES.iter().any(|w| l.contains(w))).count()
}

#[test]
fn no_answer_of_a_drive_is_thrown_away_uncounted() {
    let files = all_code();
    let mut wrong = Vec::new();
    for (name, text) in &files {
        let found = thrown(text);
        let listed = THROWN.iter().find(|(f, ..)| f == name).map_or(0, |(_, n, _)| *n);
        if found != listed {
            wrong.push(format!("{name} throws away {found} answers of a drive or a flush, and {listed} are listed in THROWN"));
        }
    }
    for (listed, n, why) in THROWN {
        if !files.iter().any(|(f, _)| f == listed) {
            wrong.push(format!("{listed} is listed and is not there"));
        }
        assert!(*n > 0 && !why.is_empty(), "each one listed says how many and why");
    }
    assert!(files.len() > 100, "Folders' code was not found");
    assert!(wrong.is_empty(), "a write the drive refuses is never counted as done: {wrong:#?}");
    let seen = |line: &str| thrown(&format!("fn f() {{\n    {line}\n}}\n"));
    for line in ["let _ = held.settle(&|n| {", "let _ = gentle::all_out(folder);", "file.sync_all().ok();", "drop(out.finish_now());", "let _ = folder_out(dir);", "let _ = out.flush();", "let _ = gentle::all_out_of(&dir);"] {
        assert_eq!(seen(line), 1, "{line}");
    }
    for line in ["gentle::all_out(folder)?;", "let lost = held.settle(&reached);", "let _ = self.conn.flush();", "// let _ = file.sync_all();", "out.finish_now().map(drop)"] {
        assert_eq!(seen(line), 0, "{line}");
    }
}

#[test]
fn the_scan_counts_code_and_not_comments_or_tests() {
    let text = "fn f() {\n    // io::copy(a, b)\n    io::copy(a, b); // fs::copy(\n    std::thread::spawn(|| ());\n}\n\n#[cfg(test)]\nmod tests {\n    fn t() { io::copy(a, b); }\n}\n";
    assert_eq!(lines_with(text, &POURS), 1);
    assert_eq!(lines_with(text, &STARTS), 1);
}
