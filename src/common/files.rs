//! The person's folders and files as the toolkit gives them, with Trindle's `trindle.conf` read when it is there.

pub use trindle_toolkit::common::files::*;

pub fn setting(key: &str) -> String {
    read_env(&trindle_conf()).get(key).cloned().unwrap_or_default()
}
