//! What Folders tells the toolkit about itself: its words, the person's choices read from the files Trindle keeps them in, and what only the person may press.

use crate::common::{files, person_only};
use crate::toolkit::host::{Act, Host};
use crate::toolkit::ui::Labels;

struct Folders;

fn helper_on(word: &str) -> bool {
    std::fs::read_to_string(files::home().join(".local/state/trindle/own-helpers")).is_ok_and(|t| t.lines().any(|l| l.trim() == word))
}

impl Host for Folders {
    fn labels(&self) -> Labels {
        if helper_on("screen-reader") {
            Labels::Always
        } else if files::setting("labels_stay") == "on" {
            Labels::WhilePointing
        } else {
            Labels::Brief
        }
    }

    fn moving(&self) -> bool {
        std::env::var("TRINDLE_DRAWING").ok().as_deref() != Some("light")
    }

    fn asks_only_the_person(&self, name: &str) -> bool {
        person_only::asks(name)
    }

    fn act(&self, act: &str) -> Act {
        if person_only::ACTS.contains(&act) { Act::PersonsOwn } else { Act::Unknown }
    }

    fn reader_switched_on(&self) -> bool {
        helper_on("screen-reader")
    }

    fn words(&self) -> &'static [(&'static str, &'static str)] {
        include!(concat!(env!("OUT_DIR"), "/english.rs"))
    }

    fn language(&self) -> Option<String> {
        if cfg!(test) {
            return Some("C".into());
        }
        Some(files::setting("language")).filter(|s| !s.trim().is_empty())
    }
}

pub extern "C" fn set() {
    crate::toolkit::host::set(&Folders);
}
