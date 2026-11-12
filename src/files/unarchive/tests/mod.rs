//! Tests for archives read through libarchive in its box.

mod frames;
mod writers;

pub use self::writers::{ar, cab, compress_z, cpio, lha, lz4, lzip, lzma_alone, rar4_split, rar5, rpm, xar, zstd, Thing};

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use crate::files::libarchive;
use crate::files::archive::Kind;
use crate::files::unarchive::child::{asked, child};
use super::*;

pub const CHILD: &str = "TRINDLE_UNARCHIVE_CHILD";

#[test]
fn the_boxed_child() {
    if let Some((need, parts)) = std::env::var(CHILD).ok().and_then(|w| asked(&w)) {
        std::process::exit(child(need, parts));
    }
}

fn place(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-files-unarchive-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
