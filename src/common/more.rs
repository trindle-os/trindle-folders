//! The one way any Trindle program offers a "More" beside a failure

use crate::toolkit::t;
use crate::toolkit::widget::{button as round, Node};

const PROGRAM: &str = "trindle-help";

pub fn valid(address: &str) -> bool {
    let word = |w: &str| !w.is_empty() && !w.starts_with('-') && w.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
    match address.split_once('#') {
        Some((page, section)) => word(page) && word(section),
        None => word(address),
    }
}

pub fn open(address: &str) -> bool {
    valid(address) && super::run::spawn(&[PROGRAM, address])
}

pub fn button<M>(press: M) -> Node<M> {
    round("help-browser-symbolic", t!("common-more")).spoken(t!("common-more-spoken")).on_press(press)
}

#[cfg(test)]
pub type Entry = (&'static str, &'static str);

#[cfg(test)]
pub const TABLES: &[(&str, &[Entry])] = &[
    ("Folders", crate::files::help::MESSAGES),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn keys() -> std::collections::HashSet<String> {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang/en");
        let own = std::fs::read_dir(dir).unwrap().flatten().map(|f| std::fs::read_to_string(f.path()).unwrap_or_default());
        let toolkit = crate::toolkit::lang::KIT_ENGLISH.iter().map(|(_, text)| text.to_string());
        let mut out = std::collections::HashSet::new();
        for text in own.chain(toolkit) {
            for line in text.lines() {
                if let Some((k, _)) = line.split_once(" =") {
                    if !k.starts_with(|c: char| c == '#' || c.is_whitespace()) {
                        out.insert(k.to_string());
                    }
                }
            }
        }
        out
    }

    #[test]
    fn only_a_page_and_a_section_are_handed_to_help() {
        for good in ["folders", "folders#archive-password", "a-b#c-1"] {
            assert!(valid(good), "{good}");
        }
        for bad in ["", "#x", "x#", "-x", "x#-y", "--help", "Folders", "a b", "a/b", "a#b#c", "a;b", "../x", "x\n"] {
            assert!(!valid(bad), "{bad:?}");
        }
        assert!(!open("--version"));
    }

    #[test]
    fn every_message_with_a_more_is_in_the_word_files_and_names_an_address_help_can_be_given() {
        let keys = keys();
        for (program, table) in TABLES {
            assert!(!table.is_empty(), "{program} has an empty table");
            let mut seen = std::collections::HashSet::new();
            for (key, address) in *table {
                assert!(seen.insert(key), "{program}: {key} is listed twice");
                assert!(keys.contains(*key), "{program}: {key} is not in the word files");
                assert!(valid(address), "{program}: {key} names {address:?}, which Help cannot be given");
            }
        }
    }

    #[test]
    fn the_button_says_more_and_is_pressable() {
        let b: Node<u8> = button(1);
        let said = b.said();
        assert_eq!(said, vec![format!("button: {}", t!("common-more"))]);
    }
}
