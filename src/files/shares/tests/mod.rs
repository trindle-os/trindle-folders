// Tests of Files' side of Windows and Samba shares.

use crate::files::app::{Files, Msg};
use crate::common::smb::Login;
use crate::toolkit::widget::{Node, Tone};

use super::card::view;
use super::commands::{list_argv, list_env, mount_argv, nfs_mount_argv, nfs_words, parts};
use super::opening::typed_login;
use super::kept::{keep_login, kept_login, seen_for, unkept_words, worked};
use super::pretend;
use super::SOURCE;
use super::rule::{again_words, unshown_words, LAPSE, SEEN_MOST, Sends, choose, held_words, mark, now};
use super::types::{Ask, Confirmed, ShareMsg, Trust, Used};
use crate::files::leave::{reach, real_name};

mod card;
mod commands;
mod keeping;
mod opening;
mod reconnect;
mod rule;

fn names(f: &Files, p: usize) -> Vec<String> {
    f.cols[p].entries.iter().map(|e| e.name.clone()).collect()
}

#[allow(clippy::too_many_arguments)]
fn rule(server: &str, address: Option<&str>, trust: Trust, typed: Option<Login>, used: Option<Used>,
        kept: impl FnOnce() -> Option<Login>, seen: impl FnOnce(&Login) -> Vec<(String, u64)>, now: u64) -> Sends {
    choose(server, address, trust, typed, used, kept, seen, now).0
}

fn keyed(node: &Node<Msg>, key: &str) -> Option<(String, bool)> {
    if let (Some(k), crate::toolkit::widget::What::Label { text, tone, .. }) = (&node.key, &node.what) {
        if k == key {
            return Some((text.clone(), matches!(tone, Tone::Problem)));
        }
    }
    node.items().iter().find_map(|i| keyed(i, key))
}

#[test]
fn the_source_list_names_every_file_of_the_folder() {
    assert_eq!(SOURCE.len(), crate::common::source::files_in("files/shares"));
}
