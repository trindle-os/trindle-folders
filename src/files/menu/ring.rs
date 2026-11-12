//! Pinning a folder or a file to the ring, beside the favourite apps.

use std::path::Path;

use crate::files::app::Files;
use crate::files::places::Where;
use crate::files::work;
use crate::toolkit::t;

impl Files {
    pub(in crate::files) fn ring_has(&self, at: &Where, path: &Path) -> Option<bool> {
        let file = self.ring.as_ref()?;
        let here = matches!(at, Where::Folder(_) | Where::Recent | Where::Found(..));
        let kept = crate::common::favourites::line_of(path).is_some();
        (here && kept && !crate::files::far::is_far(path) && crate::files::inside::within(path).is_none())
            .then(|| crate::common::favourites::has(file, path))
    }

    pub(in crate::files) fn ring_toggle(&mut self, path: &Path) {
        use crate::common::favourites::{self, Pin};
        let Some(file) = self.ring.clone() else { return };
        let name = work::name_of(path);
        let done = if favourites::has(&file, path) {
            favourites::unpin(&file, path).map(|_| (t!("folders-menu-ring-off", name = &name), false))
        } else {
            favourites::pin(&file, path).map(|how| match how {
                Pin::Pinned => (t!("folders-menu-ring-pinned", name = &name), false),
                Pin::Already => (t!("folders-menu-ring-already", name = &name), false),
                Pin::Full => (t!("folders-menu-ring-full", n = favourites::MAX), true),
                Pin::Unkept => (t!("folders-menu-ring-unkept", name = &name), true),
            })
        };
        match done {
            Ok((words, true)) => self.say_help(words, crate::files::help::NOT_KEPT),
            Ok((words, false)) => self.say(words, false),
            Err(_) => self.say_help(t!("folders-menu-ring-not-saved"), crate::files::help::NOT_KEPT),
        }
    }
}
