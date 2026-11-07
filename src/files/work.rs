// Folders' side of the file work the toolkit does: its own kinds of work, the servers it reaches, and what it calls them.

use std::path::{Path, PathBuf};

use crate::toolkit::common::file_work::work as shared;
pub use shared::*;

use super::{archive, far, imaging, jobs, remote, rename};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Own;

pub type Kind = shared::Kind<Own>;
pub type Plan = shared::Plan<Own>;
pub type Step = shared::Step<Own>;
pub type News = shared::News<Own>;
pub type Over = shared::Over<Own>;
pub type Ctl = shared::Ctl<Own>;

#[derive(Clone, Debug)]
pub enum Extra {
    Pack(archive::Format, String),
    Unpack(archive::Place),
    Wipe(u32),
    TakeOut(PathBuf),
    Picture(imaging::Work),
}

impl Side for Own {
    type Job = Extra;
    type Key = archive::Key;
    type Links = far::Links;
    type Undo = rename::Undo;
    type Locked = archive::Trouble;

    fn elsewhere(plan: &Plan, ctl: &Ctl, tell: &dyn Fn(News)) -> Option<Over> {
        far::involved(plan).then(|| far::run(plan, ctl, tell))
    }

    fn job(plan: &Plan, ctl: &Ctl, tell: &dyn Fn(News)) -> Over {
        jobs::run(plan, ctl, tell)
    }

    fn undo(done: &rename::Undo) -> Result<rename::Undo, Option<PathBuf>> {
        rename::undo(done).map_err(|_| done.moves.first().map(|(_, now)| now.clone()))
    }

    fn name_of(path: &Path) -> String {
        name_of(path)
    }
}

pub fn name_of(path: &Path) -> String {
    if let Some(a) = far::address(path) {
        let name = remote::name_of(&a.path);
        return if name.is_empty() { a.host } else { name.to_string() };
    }
    name_on_disk(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn undoing_many_names_answers_how_to_do_them_again() {
        let dir = std::env::temp_dir().join(format!("trindle-files-work-names-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let ctl = Ctl::default();
        let undo = |steps: Vec<Step>| {
            let over = run(Plan { kind: Kind::Undo(steps), items: Vec::new(), to: dir.clone(), key: None }, &ctl, &|_| {});
            assert!(over.failed.is_empty(), "{:?}", over.failed);
            over.undo
        };
        let (two, three) = (dir.join("two.txt"), dir.join("three.txt"));
        fs::write(&three, b"two").unwrap();
        let renamed = vec![Step::Other(rename::Undo { moves: vec![(two.clone(), three.clone())] })];
        let again = undo(renamed.clone());
        assert!(two.exists() && !three.exists());
        let back = undo(again);
        assert!(three.exists() && !two.exists());
        assert_eq!(back, renamed);
        fs::remove_dir_all(&dir).unwrap();
    }
}
