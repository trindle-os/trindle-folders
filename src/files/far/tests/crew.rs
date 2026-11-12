//! Files going to and from a server together: how many at once, in what order, and what a stop leaves.

use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::time::Instant;

use crate::files::remote::{Address, Login};
use crate::files::work::{self, Answer, Ctl, Kind, News, Plan, Step};
use crate::common::gentle;

use super::*;

#[test]
fn files_go_to_and_from_a_server_four_at_once_and_arrive_whole() {
    let (here, far, base) = place("slow-four");
    fs::create_dir_all(here.join("many")).unwrap();
    for n in 0..24 {
        fs::write(here.join(format!("many/f{n:02}.bin")), vec![n as u8; 3000 + n * 7]).unwrap();
    }
    let told = std::cell::RefCell::new(Vec::new());
    let ctl = Ctl { fixed: Some(Answer::Skip), ..Ctl::default() };
    let over = work::run(Plan { kind: Kind::Copy, items: vec![here.join("many")], to: PathBuf::from(&base), key: None }, &ctl, &|n| told.borrow_mut().push(n));
    assert!(over.failed.is_empty() && !over.stopped, "{:?}", over.failed);
    for n in 0..24 {
        assert_eq!(fs::read(far.join(format!("many/f{n:02}.bin"))).unwrap(), vec![n as u8; 3000 + n * 7]);
    }
    let most = fake::most_at_once(&far);
    assert!((2..=gentle::AT_ONCE).contains(&most), "{most} files went at once");
    assert!(spares(&far).is_empty());
    let told = told.into_inner();
    let total: u64 = (0..24).map(|n| 3000 + n * 7).sum();
    assert!(matches!(told.first(), Some(News::Total(t)) if *t == total), "{:?}", told.first());
    let done: Vec<u64> = told.iter().filter_map(|n| if let News::Done(d) = n { Some(*d) } else { None }).collect();
    assert!(done.windows(2).all(|w| w[0] <= w[1]) && done.last() == Some(&total), "{done:?}");
    assert!(told.iter().any(|n| matches!(n, News::Files(24, 24))));
    let back = here.join("back");
    fs::create_dir_all(&back).unwrap();
    let over = go(Kind::Copy, vec![PathBuf::from(format!("{base}/many"))], back.clone(), Answer::Skip);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    for n in 0..24 {
        assert_eq!(fs::read(back.join(format!("many/f{n:02}.bin"))).unwrap(), vec![n as u8; 3000 + n * 7]);
    }
    assert_eq!(over.undo, vec![Step::Remove(back.join("many"))]);
    assert_eq!(lock(&ctl.links.0).open.values().map(Vec::len).max(), Some(1));
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn what_each_of_the_four_said_is_put_back_in_the_order_it_was_met() {
    let (here, far, base) = place("slow-order");
    let mut items = Vec::new();
    for n in 0..16 {
        let name = if n % 3 == 0 { format!("fails-{n:02}.bin") } else { format!("good-{n:02}.bin") };
        fs::write(here.join(&name), vec![1u8; 20_000 - n * 1000]).unwrap();
        items.push(here.join(name));
    }
    let over = go(Kind::Copy, items, PathBuf::from(&base), Answer::Skip);
    let said: Vec<String> = over.failed.iter().map(|f| f.split(' ').next().unwrap().to_string()).collect();
    let wanted: Vec<String> = (0..16).filter(|n| n % 3 == 0).map(|n| format!("fails-{n:02}.bin")).collect();
    assert_eq!(said, wanted);
    assert!(spares(&far).is_empty(), "{:?}", spares(&far));
    for n in 0..12 {
        fs::write(far.join(format!("u{n:02}.bin")), vec![2u8; 12_000 - n * 500]).unwrap();
    }
    let items: Vec<PathBuf> = (0..12).map(|n| PathBuf::from(format!("{base}/u{n:02}.bin"))).collect();
    let over = go(Kind::Copy, items, here.join("box"), Answer::Skip);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert_eq!(over.undo, (0..12).map(|n| Step::Remove(here.join(format!("box/u{n:02}.bin")))).collect::<Vec<_>>());
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
fn a_server_that_gives_fewer_links_is_used_with_those_it_gave() {
    let (here, far, base) = place("slow-few");
    let user = far.file_name().unwrap().to_string_lossy().into_owned();
    fake::links_most(&user, 2);
    let items: Vec<PathBuf> = (0..12).map(|n| {
        fs::write(here.join(format!("f{n}.bin")), vec![3u8; 5000]).unwrap();
        here.join(format!("f{n}.bin"))
    }).collect();
    let over = go(Kind::Copy, items, PathBuf::from(&base), Answer::Skip);
    assert!(over.failed.is_empty() && !over.stopped, "{:?}", over.failed);
    assert!((0..12).all(|n| fs::read(far.join(format!("f{n}.bin"))).is_ok_and(|b| b == vec![3u8; 5000])));
    assert_eq!(fake::links_given(&user), 2);
    assert_eq!(fake::most_at_once(&far), 2);
    let (here2, far2, base2) = place("slow-none");
    fake::links_most(&far2.file_name().unwrap().to_string_lossy(), 0);
    let over = go(Kind::Copy, vec![here2.join("box")], PathBuf::from(&base2), Answer::Skip);
    assert_eq!(over.failed.len(), 1, "{:?}", over.failed);
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far), fs::remove_dir_all(here2), fs::remove_dir_all(far2));
}

#[test]
fn a_stop_leaves_no_spare_file_from_any_of_the_four() {
    for to_server in [true, false] {
        let (here, far, base) = place(if to_server { "slow-stop-up" } else { "slow-stop-down" });
        let from = if to_server { &here } else { &far };
        for n in 0..12 {
            fs::write(from.join(format!("f{n:02}.bin")), vec![4u8; 400_000]).unwrap();
        }
        let ctl = Ctl { fixed: Some(Answer::Skip), ..Ctl::default() };
        let stop = ctl.stop.clone();
        fake::meanwhile(&far, move || stop.store(true, Ordering::Relaxed));
        let (items, to): (Vec<PathBuf>, PathBuf) = if to_server {
            ((0..12).map(|n| here.join(format!("f{n:02}.bin"))).collect(), PathBuf::from(&base))
        } else {
            ((0..12).map(|n| PathBuf::from(format!("{base}/f{n:02}.bin"))).collect(), here.join("box"))
        };
        let over = work::run(Plan { kind: Kind::Copy, items, to, key: None }, &ctl, &|_| {});
        assert!(over.stopped && over.failed.is_empty(), "{:?}", over.failed);
        assert!(spares(&far).is_empty() && spares(&here).is_empty(), "{:?} {:?}", spares(&far), spares(&here));
        let made = if to_server { far.clone() } else { here.join("box") };
        for e in fs::read_dir(&made).unwrap().flatten().filter(|e| e.file_name().to_string_lossy().ends_with(".bin")) {
            assert_eq!(e.metadata().unwrap().len(), 400_000, "{:?} was left half made", e.path());
        }
        assert!(fs::read_dir(&made).unwrap().flatten().filter(|e| e.file_name().to_string_lossy().ends_with(".bin")).count() < 12);
        let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
    }
}

#[test]
fn two_of_one_name_in_one_go_are_asked_about_as_one_at_a_time() {
    let (here, far, base) = place("slow-twins");
    fs::create_dir_all(here.join("other")).unwrap();
    fs::write(here.join("twin.txt"), b"first").unwrap();
    fs::write(here.join("other/twin.txt"), b"second").unwrap();
    let over = go(Kind::Copy, vec![here.join("twin.txt"), here.join("other/twin.txt")], PathBuf::from(&base), Answer::Both);
    assert!(over.failed.is_empty(), "{:?}", over.failed);
    assert_eq!(fs::read(far.join("twin.txt")).unwrap(), b"first");
    assert_eq!(fs::read(far.join("twin (2).txt")).unwrap(), b"second");
    let _ = (fs::remove_dir_all(here), fs::remove_dir_all(far));
}

#[test]
#[ignore = "needs a real SFTP server named by TRINDLE_TEST_SFTP: run by hand"]
fn timing_against_a_real_sftp_server() {
    let port = std::env::var("TRINDLE_TEST_SFTP").unwrap();
    let key = PathBuf::from(std::env::var("TRINDLE_TEST_SFTP_KEY").unwrap());
    let room = PathBuf::from(std::env::var("TRINDLE_TEST_SFTP_ROOM").unwrap());
    let (small, large) = (room.join("small"), room.join("large"));
    let (served, back) = (room.join("served"), room.join("back"));
    for d in [&small, &large, &served, &back] {
        let _ = fs::remove_dir_all(d);
        fs::create_dir_all(d).unwrap();
    }
    let mut noise = [0u8; 1 << 20];
    for n in 0..2000 {
        getrandom::fill(&mut noise[..4096]).unwrap();
        fs::write(small.join(format!("s{n:04}.bin")), &noise[..4096]).unwrap();
    }
    for n in 0..4 {
        let mut f = File::create(large.join(format!("l{n}.bin"))).unwrap();
        for _ in 0..64 {
            getrandom::fill(&mut noise).unwrap();
            std::io::Write::write_all(&mut f, &noise).unwrap();
        }
    }
    let base = format!("sftp://127.0.0.1:{port}{}", served.display());
    let login = Login { key: Some(key), trust_new_host: true, ..Login::default() };
    for at_once in ["1", "4"] {
        std::env::set_var("TRINDLE_TEST_AT_ONCE", at_once);
        for what in ["small", "large"] {
            let ctl = Ctl { fixed: Some(Answer::Skip), ..Ctl::default() };
            ctl.links.open(&Address::parse(&base).unwrap(), &login, None).unwrap();
            let _ = fs::remove_dir_all(served.join(what));
            let _ = fs::remove_dir_all(back.join(what));
            let t = Instant::now();
            let over = work::run(Plan { kind: Kind::Copy, items: vec![room.join(what)], to: PathBuf::from(&base), key: None }, &ctl, &|_| {});
            let up = t.elapsed();
            assert!(over.failed.is_empty() && !over.stopped, "{:?}", over.failed);
            let t = Instant::now();
            let over = work::run(Plan { kind: Kind::Copy, items: vec![PathBuf::from(format!("{base}/{what}"))], to: back.clone(), key: None }, &ctl, &|_| {});
            let down = t.elapsed();
            assert!(over.failed.is_empty() && !over.stopped, "{:?}", over.failed);
            let n = fs::read_dir(back.join(what)).unwrap().count();
            println!("TIMING at_once={at_once} {what}: to the server {:.2}s, from the server {:.2}s, {n} files", up.as_secs_f64(), down.as_secs_f64());
        }
    }
    for d in [&small, &large, &served, &back] {
        let _ = fs::remove_dir_all(d);
    }
}
