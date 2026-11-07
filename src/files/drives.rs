// Drives plugged in, in Files' sidebar.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::app::Files;
use super::places::Where;
use crate::toolkit::t;
use crate::common::volumes::{self, Step, Volume, Watching};

pub enum Cmd {
    Mount(Volume),
    Unlock(Volume),
    Eject(String, Vec<Step>),
}

#[derive(Default)]
pub struct Drives {
    pub list: Vec<Volume>,
    pub fixed: Vec<Volume>,
    keep: Option<Rc<RefCell<Option<Watching>>>>,
    hook: Option<Box<dyn Fn(Cmd)>>,
    busy: HashSet<String>,
}

#[derive(Clone, Debug)]
pub enum DriveMsg {
    Listed(Vec<Volume>),
    Show(usize),
    Eject(usize),
    Mounted(String, Result<String, String>),
    Ejected(String, Result<(), String>),
}

#[derive(Debug, PartialEq)]
pub struct Row {
    pub name: String,
    pub icon: &'static str,
    pub chosen: bool,
    pub eject: String,
}

pub fn rows(list: &[Volume], here: Option<&Path>) -> Vec<Row> {
    let mut count: HashMap<String, usize> = HashMap::new();
    for v in list {
        *count.entry(v.shown()).or_default() += 1;
    }
    list.iter().map(|v| {
        let mut name = v.shown();
        if count.get(&name).is_some_and(|n| *n > 1) {
            name = t!("folders-drives-name-on", name = &name, drive = &v.drive_name);
        }
        Row {
            icon: if v.locked { "changes-prevent-symbolic" } else { "drive-removable-media-symbolic" },
            chosen: here.is_some_and(|h| v.mounts.iter().any(|m| h.starts_with(m))),
            eject: t!("folders-drives-eject", name = &name),
            name,
        }
    }).collect()
}

impl Drives {
    pub fn is_busy(&self, id: &str) -> bool {
        self.busy.contains(id)
    }

    pub fn busy() -> &'static str {
        t!("folders-drives-busy")
    }

    fn find(&self, id: &str) -> Option<&Volume> {
        self.list.iter().chain(&self.fixed).find(|v| v.id == id)
    }
}

impl Files {
    pub fn set_drives(&mut self, keep: Rc<RefCell<Option<Watching>>>, hook: impl Fn(Cmd) + 'static) {
        self.drives.keep = Some(keep);
        self.drives.hook = Some(Box::new(hook));
    }

    fn leave_gone(&mut self, gone: &[String]) {
        let home = self.home.clone();
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                if self.cols[p].tabs[t].at.folder().is_some_and(|f| gone.iter().any(|m| f.starts_with(m))) {
                    if t == self.cols[p].tab {
                        self.go(p, Where::Folder(home.clone()));
                    } else {
                        self.cols[p].tabs[t].at = Where::Folder(home.clone());
                    }
                }
            }
        }
    }

    pub(super) fn drive_msg(&mut self, m: DriveMsg) {
        match m {
            DriveMsg::Listed(list) => {
                let now: HashSet<&String> = list.iter().flat_map(|v| &v.mounts).collect();
                let gone: Vec<String> = self.drives.list.iter().chain(&self.drives.fixed).flat_map(|v| &v.mounts)
                    .filter(|m| !now.contains(m)).cloned().collect();
                self.drives.busy.retain(|id| list.iter().any(|v| v.id == *id));
                let (fixed, plugged): (Vec<Volume>, Vec<Volume>) = list.into_iter().partition(|v| v.fixed);
                self.drives.list = plugged;
                self.drives.fixed = fixed;
                self.leave_gone(&gone);
            }
            DriveMsg::Show(i) => {
                if let Some(id) = self.drives.list.get(i).map(|v| v.id.clone()) {
                    self.drive_show(&id);
                }
            }
            DriveMsg::Mounted(id, got) => {
                self.drives.busy.remove(&id);
                let name = self.drives.find(&id).map(Volume::name).unwrap_or_default();
                match got {
                    Ok(at) => {
                        self.say(String::new(), false);
                        self.go(self.side(), Where::Folder(PathBuf::from(at)));
                    }
                    Err(why) if why.is_empty() => self.say(String::new(), false),
                    Err(why) => self.say_help(t!("folders-drives-open-failed", name = &name, why = why.trim_end_matches('.')), super::help::DRIVE),
                }
            }
            DriveMsg::Eject(i) => {
                if let Some(id) = self.drives.list.get(i).map(|v| v.id.clone()) {
                    self.drive_eject(&id);
                }
            }
            DriveMsg::Ejected(id, got) => {
                self.drives.busy.remove(&id);
                let name = self.drives.find(&id).map(Volume::name).unwrap_or_default();
                match got {
                    Ok(()) => {
                        self.say(String::new(), false);
                        let name = if name.is_empty() { t!("folders-drives-the-drive").to_string() } else { name };
                        self.notify(t!("folders-drives-taken-out", name = &name));
                    }
                    Err(why) => self.say_help(t!("folders-drives-away-failed", name = &name, why = why.trim_end_matches('.')), super::help::DRIVE),
                }
            }
        }
    }

    pub(super) fn drive_show(&mut self, id: &str) {
        let Some(v) = self.drives.find(id).cloned() else { return };
        if self.drives.is_busy(&v.id) {
            self.say_help(Drives::busy().to_string(), super::help::DRIVE);
            return;
        }
        if let Some(m) = v.mounted() {
            self.go(self.side(), Where::Folder(PathBuf::from(m)));
            return;
        }
        self.drives.busy.insert(v.id.clone());
        let cmd = if v.locked { Cmd::Unlock(v.clone()) } else { Cmd::Mount(v.clone()) };
        if !self.drive_do(cmd) {
            self.drives.busy.remove(&v.id);
            self.say_help(t!("folders-drives-unreachable"), super::help::DRIVE);
        } else if !v.locked {
            self.say(t!("folders-drives-opening", name = &v.name()), false);
        }
    }

    pub(super) fn drive_eject(&mut self, id: &str) {
        let Some(v) = self.drives.list.iter().find(|v| v.id == id).cloned() else { return };
        if !self.drives.busy.insert(v.id.clone()) {
            self.say_help(Drives::busy().to_string(), super::help::DRIVE);
            return;
        }
        let mounts = v.mounts.clone();
        self.leave_gone(&mounts);
        let steps = volumes::eject_steps(&v, &self.drives.list);
        self.say(t!("folders-drives-putting-away", name = &v.name()), false);
        if !self.drive_do(Cmd::Eject(v.id.clone(), steps)) {
            self.drives.busy.remove(&v.id);
            self.say_help(t!("folders-drives-unreachable"), super::help::DRIVE);
        }
    }

    fn drive_do(&self, cmd: Cmd) -> bool {
        match &self.drives.hook {
            Some(hook) => {
                hook(cmd);
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::app::Msg;
    use crate::toolkit::ui::App;
    use crate::common::volumes::Kind;

    fn vol(id: &str, label: &str, mounts: &[&str]) -> Volume {
        Volume {
            id: id.into(), dev: id.into(), label: label.into(), size: 8_000_000_000,
            drive: format!("/drive/{id}"), drive_name: format!("Stick {id}"), kind: Kind::Usb,
            mounts: mounts.iter().map(|m| m.to_string()).collect(), locked: false, container: None,
            can_eject: false, can_power_off: true, fixed: false, system: false, key: String::new(), drive_key: String::new(),
        }
    }

    fn files() -> Files {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.set_drives(Rc::default(), |_| {});
        f
    }

    #[test]
    fn the_sidebar_lists_each_part_and_marks_the_one_a_pane_is_in() {
        let list = [vol("sdb1", "HOLIDAY", &["/run/user/1000/media/HOLIDAY"]), vol("sdb2", "", &[])];
        let r = rows(&list, Some(Path::new("/run/user/1000/media/HOLIDAY/photos")));
        assert_eq!(r.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(), vec!["HOLIDAY", "8.0 GB"]);
        assert_eq!(r.iter().map(|r| r.chosen).collect::<Vec<_>>(), vec![true, false]);
        assert_eq!(r[0].eject, "Eject HOLIDAY");
        assert!(rows(&[], None).is_empty(), "no drives, no list");
    }

    #[test]
    fn two_drives_called_alike_are_told_apart() {
        let r = rows(&[vol("a1", "BACKUP", &[]), vol("b1", "BACKUP", &[])], None);
        assert_eq!(r[0].name, "BACKUP on Stick a1");
        assert_eq!(r[1].name, "BACKUP on Stick b1");
        let mut locked = vol("c1", "", &[]);
        locked.locked = true;
        assert_eq!(rows(&[locked], None)[0].name, "Locked: 8.0 GB");
    }

    #[test]
    fn a_new_drive_appears_and_an_unplugged_one_takes_its_pane_home() {
        let mut f = files();
        let at = std::env::temp_dir();
        f.update(Msg::Drive(DriveMsg::Listed(vec![vol("sdb1", "HOLIDAY", &[at.to_str().unwrap()])])));
        assert_eq!(f.drives.list.len(), 1);
        let p = f.side();
        f.go(p, Where::Folder(at.clone()));
        f.update(Msg::Drive(DriveMsg::Listed(Vec::new())));
        assert!(f.drives.list.is_empty());
        assert_eq!(f.cols[p].tabs[0].at, Where::Folder(f.home.clone()), "a pane inside a drive that is gone goes Home");
    }

    #[test]
    fn pressing_a_drive_that_is_not_mounted_asks_for_it_to_be_mounted_once() {
        let mut f = files();
        f.update(Msg::Drive(DriveMsg::Listed(vec![vol("sdb1", "HOLIDAY", &[])])));
        f.update(Msg::Drive(DriveMsg::Show(0)));
        assert!(f.drives.busy.contains("sdb1") && !f.bad, "{}", f.said);
        assert!(f.said.contains("Opening HOLIDAY"), "{}", f.said);
        f.said.clear();
        f.update(Msg::Drive(DriveMsg::Show(0)));
        assert_eq!(f.said, Drives::busy());
        assert!(f.bad && f.drives.busy.contains("sdb1"));
        let at = std::env::temp_dir();
        f.update(Msg::Drive(DriveMsg::Mounted("sdb1".into(), Ok(at.to_string_lossy().into_owned()))));
        let p = f.side();
        assert_eq!(f.cols[p].tabs[0].at, Where::Folder(at));
        assert!(!f.drives.busy.contains("sdb1") && f.said.is_empty());
    }

    #[test]
    fn a_drive_that_will_not_open_says_why_in_plain_words() {
        let mut f = files();
        f.update(Msg::Drive(DriveMsg::Listed(vec![vol("sdb1", "HOLIDAY", &[])])));
        f.update(Msg::Drive(DriveMsg::Mounted("sdb1".into(), Err("Something is still using it.".into()))));
        assert!(f.bad && f.said == "HOLIDAY could not be opened: Something is still using it.", "{}", f.said);
        f.update(Msg::Drive(DriveMsg::Mounted("sdb1".into(), Err(String::new()))));
        assert!(!f.bad && f.said.is_empty());
    }

    #[test]
    fn a_locked_part_asks_for_its_passphrase_and_ejecting_leaves_the_folder_first() {
        let mut f = files();
        let mut locked = vol("sdb2", "", &[]);
        locked.locked = true;
        locked.container = Some("sdb2".into());
        f.update(Msg::Drive(DriveMsg::Listed(vec![locked, vol("sdb1", "HOLIDAY", &[std::env::temp_dir().to_str().unwrap()])])));
        f.update(Msg::Drive(DriveMsg::Show(0)));
        assert!(f.drives.busy.contains("sdb2"), "a locked part asks for its passphrase");
        let p = f.side();
        f.go(p, Where::Folder(std::env::temp_dir()));
        f.update(Msg::Drive(DriveMsg::Eject(1)));
        assert_eq!(f.cols[p].tabs[0].at, Where::Folder(f.home.clone()), "nothing in the window holds it");
        assert!(f.said.contains("Putting HOLIDAY away"), "{}", f.said);
        f.update(Msg::Drive(DriveMsg::Ejected("sdb1".into(), Err("Something is still using it.".into()))));
        assert!(f.bad && f.said.starts_with("HOLIDAY could not be put away"), "{}", f.said);
    }

    #[test]
    fn a_drive_put_away_says_so_in_the_notice_and_a_locked_one_shows_a_padlock() {
        let mut f = files();
        let mut locked = vol("sdb2", "", &[]);
        locked.locked = true;
        let rows = rows(&[vol("sdb1", "HOLIDAY", &[]), locked], None);
        assert_eq!((rows[0].icon, rows[1].icon), ("drive-removable-media-symbolic", "changes-prevent-symbolic"));
        f.update(Msg::Drive(DriveMsg::Listed(vec![vol("sdb1", "HOLIDAY", &[])])));
        f.update(Msg::Drive(DriveMsg::Ejected("sdb1".into(), Ok(()))));
        let (n, words) = f.notice.clone().expect("the notice says it can be taken out");
        assert_eq!(words, "HOLIDAY can be taken out now");
        f.notify("Another");
        f.update(Msg::NoticeGone(n));
        assert!(f.notice.is_some());
        let (n, _) = f.notice.clone().unwrap();
        f.update(Msg::NoticeGone(n));
        assert!(f.notice.is_none());
    }
}
