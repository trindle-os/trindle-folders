//! What Files does to files: copying, moving, the Trash, deleting for

use crate::toolkit::t;

mod acts;
mod drop;
mod jobs;
mod msg;
mod steps;
mod text;
pub(in crate::files) use text::names;
#[cfg(test)]
mod tests;
mod types;
mod undo;
mod window;

#[cfg(test)]
pub use text::words;
pub use types::{Ask, Asking, Busy, Clip, Drag, Job, Land, Pick, Way, Words};
pub(super) use drop::open_to_write;
#[cfg(test)]
pub(super) use steps::make_new;
pub(super) use steps::{picked_up, pinnable};
pub(super) use text::bad_name;
pub(super) use types::Turn;

#[cfg(test)]
pub(super) const SOURCE: &[&str] = &[include_str!("mod.rs"), include_str!("acts.rs"), include_str!("drop.rs"), include_str!("jobs.rs"), include_str!("msg.rs"), include_str!("steps.rs"), include_str!("text.rs"), include_str!("types.rs"), include_str!("undo.rs"), include_str!("window.rs")];

pub const UNDO_KEPT: usize = 10;
const NOTICE_FOR: std::time::Duration = std::time::Duration::from_secs(3);
pub const DRAG_SLOP: f32 = 6.0;
pub fn panes_left() -> f32 {
    2.0 * crate::toolkit::theme::sheet().panel_gap + super::view::side_panel() + crate::toolkit::theme::sheet().card_padding
}
pub fn between() -> f32 {
    2.0 * crate::toolkit::theme::sheet().card_padding + super::view::strip() + 2.0 * super::view::strip_gap()
}
const NAMES_SAID: usize = 3;
pub(super) const CLIP_LIMIT: u64 = 1 << 20;
fn far_clip() -> &'static str {
    t!("folders-act-far-clip")
}
fn shut() -> &'static str {
    t!("folders-act-shut")
}

const QUIT_WAIT: std::time::Duration = std::time::Duration::from_secs(5);

fn cut_elsewhere() -> &'static str {
    t!("folders-act-cut-elsewhere")
}

pub fn being_moved() -> &'static str {
    t!("folders-act-being-moved")
}
pub fn arriving() -> &'static str {
    t!("folders-act-arriving")
}
