// Archives of the kinds Trindle has no reader of its own for, read by the system's libarchive.

mod child;
mod frames;
mod rar;
mod run;
#[cfg(test)]
pub mod tests;

pub use self::child::main;
pub(super) use self::rar::rar_part;

use std::io::{self, Read};
use std::path::Path;
use std::time::Duration;

use crate::files::walled;
use crate::files::archive::{self, trouble, Item, Need, Trouble};
use crate::files::meter::Meter;
use crate::files::unarchive::rar::{volumes, whole};
use crate::files::unarchive::run::run;

const MARK: [u8; 8] = *b"\x00\xffTRAR\x01\n";
const MARK_WITHIN: usize = 1 << 16;
const CHUNK: usize = 1 << 16;
const NAME_MOST: usize = 1 << 16;
const SELF: &str = walled::SELF;
const MEMORY: u64 = 2 << 30;
const CPU_NAMES: u64 = 120;
const CPU_ALL: u64 = 2 * 60 * 60;
const WATCH: Duration = Duration::from_millis(50);

const FINE: u8 = 0;
const DAMAGED: u8 = 1;
const LOCKED: u8 = 2;
const UNKNOWN: u8 = 3;
const NO_MEMORY: u8 = 4;
const MISSING: u8 = 5;
const UNSUPPORTED: u8 = 6;

const HAS_MODE: u8 = 1;
const HAS_DATE: u8 = 2;
const HAS_BYTES: u8 = 4;
const NAMELESS: u8 = 8;

fn word(need: Need) -> &'static str {
    match need {
        Need::Names => "names",
        Need::All => "all",
        Need::Some => "some",
    }
}

fn need_of(word: &str) -> Option<Need> {
    [Need::Names, Need::All, Need::Some].into_iter().find(|n| self::word(*n) == word)
}

pub(super) fn walk(
    archive: &Path,
    meter: &Meter,
    need: Need,
    want: &dyn Fn(&str) -> bool,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    let parts = volumes(archive);
    for part in &parts {
        archive::opened(part)?;
    }
    if need != Need::Names {
        whole(&parts)?;
    }
    let mut need = need;
    let mut chosen: Vec<u32> = Vec::new();
    if need == Need::Some {
        let (mut number, mut every) = (0u32, true);
        run(&parts, Need::Names, &[], &meter.hushed(), &mut |item, _| {
            if want(&item.name) {
                chosen.push(number);
            } else {
                every = false;
            }
            number = number.saturating_add(1);
            if number as usize > archive::MOST_THINGS { Err(trouble(Trouble::TooMany)) } else { Ok(true) }
        })?;
        if chosen.is_empty() {
            return Ok(());
        }
        if every {
            need = Need::All;
        }
    }
    let mut total = 0u64;
    for part in &parts {
        total = total.saturating_add(std::fs::metadata(part)?.len());
    }
    meter.total(total);
    run(&parts, need, &chosen, meter, &mut |item, r| if want(&item.name) { f(item, r) } else { Ok(true) })?;
    meter.end();
    Ok(())
}
