// What the window does with each message about files and work.

use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;

use crate::files::app::{Files, Msg};
use crate::files::earlier;
use crate::files::inplace::End;
use crate::files::work::{Answer, Clash, Kind, News, Plan};
use crate::toolkit::t;
use super::types::{Asking, Land, Turn};
use super::text::words;

impl Files {
    pub(in crate::files) fn act(&mut self, msg: Msg) {
        if self.earlier_copy_refuses(&msg) {
            return;
        }
        match msg {
            Msg::Copy => self.across(false),
            Msg::Move => self.across(true),
            Msg::Cut => self.copy_clip(true),
            Msg::CopyClip => self.copy_clip(false),
            Msg::Paste => self.paste(),
            Msg::Pasted(got) => self.pasted(got),
            Msg::Trash => self.trash_key(),
            Msg::Delete | Msg::DeleteSure | Msg::NoDelete => self.delete_msg(msg),
            Msg::EmptyTrash => self.empty_trash(),
            Msg::Restore => self.restore(),
            Msg::Rename | Msg::RenameText(_) | Msg::RenameDone | Msg::RenameKeep | Msg::RenameNext(_) | Msg::RenameStop | Msg::SlowRename(_) => self.rename_msg(msg),
            Msg::NewFolder => self.make(true),
            Msg::NewDocument => self.make(false),
            Msg::Undo => self.turn_back(false),
            Msg::Redo => self.turn_back(true),
            Msg::Work(..) | Msg::WorkGone(_) | Msg::Waiting | Msg::Pause(_) | Msg::Cancel(_) | Msg::Answer(..) | Msg::AllSame(..) => self.work_msg(msg),
            Msg::DropOn(_) | Msg::DropStop | Msg::Landed(_) | Msg::Escape => self.drop_msg(msg),
            _ => {}
        }
    }

    fn earlier_copy_refuses(&mut self, msg: &Msg) -> bool {
        let (here, there) = (self.earlier_pane(false), self.earlier_pane(true));
        match msg {
            Msg::SlowRename(_) if here => return true,
            Msg::Rename | Msg::Cut | Msg::Paste | Msg::Pasted(_) | Msg::Trash | Msg::Delete | Msg::NewFolder | Msg::NewDocument | Msg::Move if here => {
                self.say(earlier::shut(), false);
                return true;
            }
            Msg::Copy | Msg::Move if there => {
                self.say(earlier::shut(), false);
                return true;
            }
            _ => {}
        }
        false
    }

    fn delete_msg(&mut self, msg: Msg) {
        match msg {
            Msg::Delete => {
                let items = self.picked_paths(self.side());
                self.ask_delete(items);
            }
            Msg::DeleteSure => {
                let Some(items) = self.confirm.take() else { return };
                let words = words(&Kind::Delete, &items, Path::new(""), &self.home);
                self.forget_undo(&items);
                self.start(Plan { kind: Kind::Delete, items, to: PathBuf::new(), key: None }, words, Turn::New);
            }
            Msg::NoDelete => {
                self.confirm = None;
                self.asked.clear();
            }
            _ => {}
        }
    }

    fn rename_msg(&mut self, msg: Msg) {
        match msg {
            Msg::Rename => self.rename_start(),
            Msg::RenameText(t) => {
                if let Some(r) = &mut self.renaming {
                    r.text = t;
                }
            }
            Msg::RenameDone => self.rename_end(End::Enter),
            Msg::RenameKeep => self.rename_end(End::Away),
            Msg::RenameNext(forward) => self.rename_end(End::Next(forward)),
            Msg::RenameStop => {
                self.renaming = None;
                if self.bad {
                    self.say("", false);
                }
            }
            Msg::SlowRename(n) => self.slow_due(n, std::time::Instant::now()),
            _ => {}
        }
    }

    fn work_msg(&mut self, msg: Msg) {
        match msg {
            Msg::Work(id, News::Over(over)) => self.finished(id, over),
            Msg::Work(id, News::Clash(clash)) => self.job_asks(id, clash),
            Msg::Work(id, news) => self.job_news(id, news),
            Msg::WorkGone(id) => {
                let Some(k) = self.ended.iter().position(|e| e.id == id) else { return };
                let e = self.ended.remove(k);
                if !e.stays && self.said == e.words {
                    self.said.clear();
                }
            }
            Msg::Waiting => self.say(t!("folders-act-closing-waits"), false),
            Msg::Pause(id) => {
                let Some(job) = self.job_mut(id) else { return };
                job.paused = !job.paused;
                job.pace.rest();
                job.pause.store(job.paused, Ordering::SeqCst);
                self.tell_jobs();
            }
            Msg::Cancel(id) => {
                let Some(job) = self.job_mut(id) else { return };
                job.stop.store(true, Ordering::SeqCst);
                job.pause.store(false, Ordering::SeqCst);
                job.paused = false;
                if job.asking.take().is_some() {
                    if let Some(tx) = &job.answers {
                        let _ = tx.send((Answer::Skip, false));
                    }
                }
                self.tell_jobs();
            }
            Msg::Answer(id, a) => {
                let Some(job) = self.job_mut(id) else { return };
                if job.asking.take().is_some() {
                    if let Some(tx) = &job.answers {
                        let _ = tx.send((a, job.all));
                    }
                }
                self.tell_jobs();
            }
            Msg::AllSame(id, on) => {
                if let Some(job) = self.job_mut(id) {
                    job.all = on;
                }
                self.tell_jobs();
            }
            _ => {}
        }
    }

    fn job_asks(&mut self, id: u64, clash: Clash) {
        let asking = Asking::new(clash, &self.mime);
        let Some(job) = self.job_mut(id) else { return };
        job.asking = Some(asking);
        job.all = false;
        job.pace.rest();
        self.tell_jobs();
    }

    fn job_news(&mut self, id: u64, news: News) {
        let Some(job) = self.job_mut(id) else { return };
        match news {
            News::Total(t) => job.total = t,
            News::Done(d) => {
                job.done = d;
                if !job.paused && job.asking.is_none() {
                    job.pace.note(std::time::Instant::now(), d, job.total);
                }
            }
            News::Files(done, of) => job.pace.files(done, of),
            News::Clash(_) | News::Over(_) => {}
        }
        self.tell_jobs();
    }

    fn drop_msg(&mut self, msg: Msg) {
        match msg {
            Msg::DropOn(i) => {
                let Some((items, way)) = self.dropping.take() else { return };
                if let Some((_, at)) = self.drop_places().into_iter().nth(i) {
                    self.drop_into(items, at, way);
                }
            }
            Msg::DropStop => self.dropping = None,
            Msg::Landed(land) => {
                if self.drag.as_ref().is_some_and(|d| d.right || matches!(land, Land::Part(..))) {
                    self.landed = self.land_place(land);
                }
            }
            Msg::Escape => {
                self.landed = None;
                self.pin_at = None;
                self.drag = None;
                self.dropping = None;
                self.confirm = None;
                self.renaming = None;
                self.said.clear();
            }
            _ => {}
        }
    }
}
