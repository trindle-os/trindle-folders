// Whether this login's screen is locked, as a small file the compositor keeps in the login's runtime folder.

use std::path::PathBuf;

pub const NAME: &str = "trindle-locked";

pub fn path() -> PathBuf {
    crate::common::files::scratch().join(NAME)
}
