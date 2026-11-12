//! Files on the clipboard: the toolkit's rules, and where Folders notes what it last cut.

use std::path::PathBuf;

pub use crate::toolkit::common::file_work::clip::*;

pub fn cut_note() -> PathBuf {
    crate::common::files::scratch().join("trindle-folders-cut")
}
