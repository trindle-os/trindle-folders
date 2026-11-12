//! The shared pieces Folders carries with it, copied small from Trindle and the toolkit.

pub use crate::toolkit::common::file_work::gentle;
pub use crate::toolkit::common::{selection, ui};
pub mod apps;
pub mod charge;
pub mod clock;
pub mod earlier;
pub mod favourites;
pub mod files;
pub mod folders_conf;
pub mod home_snapshots;
pub mod keyring_said;
pub mod locked;
pub mod memory;
pub mod more;
pub mod no_tick;
pub mod person_only;
pub mod run;
pub mod wallpaper_size;
pub mod keyfile;
pub mod prompt;
pub mod hidden_drives;
pub mod drive_bits;
pub mod refused;
pub mod seccomp;
pub mod sharing;
pub mod smb;
pub mod style;
pub mod volumes;
pub fn thousands(n: u64) -> String {
    trindle_toolkit::lang::format::number(n as f64, 0)
}

#[cfg(test)]
mod gentle_guard;
#[cfg(test)]
pub mod nothing_named_is_followed;
#[cfg(test)]
pub mod no_waiting_on_a_name;
#[cfg(test)]
pub mod save_is_a_word;
#[cfg(test)]
pub mod red_for_good;
#[cfg(test)]
pub mod jobs_say_why;
#[cfg(test)]
pub mod no_writing_in_place;
#[cfg(test)]
pub mod info_popup_tests;
#[cfg(test)]
mod languages_agree;
#[cfg(test)]
pub use crate::toolkit::common::needs;
#[cfg(test)]
pub mod source;
