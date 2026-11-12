//! Undo and redo.

use std::path::{Path, PathBuf};

use crate::files::app::Files;
use crate::files::work::{Kind, Plan, Step};
use crate::toolkit::t;
use super::types::{Turn, Words};
use super::text::label_what;

impl Files {
    pub(in crate::files) fn forget_undo(&mut self, items: &[PathBuf]) {
        let hit = |p: &Path| items.iter().any(|i| p.starts_with(i));
        let touches = |steps: &[Step]| steps.iter().any(|s| match s {
            Step::Remove(p) | Step::Restore(p) => hit(p),
            Step::Back { now, .. } | Step::Retrash { now, .. } => hit(now),
            Step::Other(done) => done.moves.iter().any(|(_, now)| hit(now)),
        });
        self.undo.retain(|(_, steps)| !touches(steps));
        self.redo.retain(|(_, steps)| !touches(steps));
    }

    pub(super) fn follow_moves(&mut self, steps: &[Step], back: bool) {
        let mut moves: Vec<(PathBuf, PathBuf)> = Vec::new();
        for step in steps {
            match step {
                Step::Back { now, was } => moves.push((was.clone(), now.clone())),
                Step::Other(done) => moves.extend(done.moves.iter().cloned()),
                _ => {}
            }
        }
        let failed = moves.into_iter().any(|(was, now)| {
            let (from, to) = if back { (now, was) } else { (was, now) };
            self.kept.moved(&from, &to).is_err()
        });
        if failed {
            self.say_help(t!("folders-act-colours-lost"), crate::files::help::COLOURS_LOST);
        }
    }

    pub(super) fn turn_back(&mut self, again: bool) {
        let last = if again { self.redo.pop() } else { self.undo.pop() };
        let Some((label, steps)) = last else {
            return self.say(if again { t!("folders-act-nothing-redo") } else { t!("folders-act-nothing-undo") }, false);
        };
        self.follow_moves(&steps, true);
        let what = label_what(&label, again);
        let (words, turn) = if again {
            (Words { doing: t!("folders-act-redoing", what = &what), done: t!("folders-act-redid", what = &what), verb: t!("folders-act-verb-done-again"), help: crate::files::help::WORK_NOT_DONE, undo: String::new() }, Turn::Redo(what))
        } else {
            (Words { doing: t!("folders-act-undoing", what = &what), done: t!("folders-act-undid", what = &what), verb: t!("folders-act-verb-put-back"), help: crate::files::help::WORK_NOT_DONE, undo: String::new() }, Turn::Undo(what))
        };
        self.start(Plan { kind: Kind::Undo(steps), items: Vec::new(), to: PathBuf::new(), key: None }, words, turn);
    }
}
