// Storage: a page of every drive and partition a person would open.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::app::{Files, Msg};
use super::icons::stand_in;
use super::places::{root_name, Where};
use crate::common::smb::Mounted;
use crate::common::volumes::{size_words, Kind, Volume};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::tiles::{fill_bar, WORRY};
use crate::toolkit::widget::{button, card, column, even_grid, heading, icon_circle, note, row, scroll, space, Length, Node};

#[derive(Clone, Debug, PartialEq)]
pub enum What {
    System,
    Volume(String),
    Share,
    Phone(String),
    Mount,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub id: String,
    pub what: What,
    pub name: String,
    pub kind: &'static str,
    pub icon: &'static str,
    pub at: Option<PathBuf>,
    pub size: Option<u64>,
    pub locked: bool,
    pub eject: bool,
}

type Full = HashMap<PathBuf, (u64, u64)>;
type Reader = Arc<dyn Fn(&Path) -> Option<(u64, u64)> + Send + Sync>;

#[derive(Default)]
pub struct Computer {
    pub cards: Vec<Card>,
    places: Vec<String>,
    full: Full,
    asked: Vec<PathBuf>,
    id: u64,
    again: bool,
    #[cfg(test)]
    pub reader: Option<Reader>,
    #[cfg(test)]
    pub network: Vec<PathBuf>,
}

#[derive(Clone, Debug)]
pub enum ComputerMsg {
    Open(usize, String),
    Mount(usize, String),
    Eject(String),
    Full(u64, Vec<(PathBuf, Option<(u64, u64)>)>),
}

fn kind_of(v: &Volume) -> (&'static str, &'static str) {
    let kind = if v.system {
        t!("folders-computer-kind-system")
    } else if v.fixed {
        t!("folders-computer-kind-inside")
    } else {
        match v.kind {
            Kind::Usb => t!("folders-computer-kind-usb"),
            Kind::Card => t!("folders-computer-kind-card"),
            Kind::Disc => t!("folders-computer-kind-disc"),
            Kind::Other => t!("folders-computer-kind-other"),
        }
    };
    let icon = if v.locked {
        "changes-prevent-symbolic"
    } else if v.fixed {
        "drive-harddisk-symbolic"
    } else {
        match v.kind {
            Kind::Card => "media-flash-symbolic",
            Kind::Disc => "media-optical-symbolic",
            Kind::Usb | Kind::Other => "drive-removable-media-symbolic",
        }
    };
    (kind, icon)
}

pub fn cards(volumes: &[Volume], shares: &[Mounted], phones: &[(String, String)], network: &[PathBuf]) -> Vec<Card> {
    let mut out = Vec::new();
    if !volumes.iter().any(|v| v.system) {
        out.push(Card { id: "system".into(), what: What::System, name: root_name().into(), kind: t!("folders-computer-kind-system"),
                        icon: "drive-harddisk-symbolic", at: Some(PathBuf::from("/")), size: None, locked: false, eject: false });
    }
    let rank = |v: &Volume| if v.system { 0 } else if v.fixed { 1 } else { 2 };
    let mut drives: Vec<&Volume> = volumes.iter().collect();
    drives.sort_by_key(|v| rank(v));
    for v in drives {
        let (kind, icon) = kind_of(v);
        let name = if v.system && v.label.trim().is_empty() { root_name().to_string() } else { v.name() };
        let at = if v.system { Some(PathBuf::from("/")) } else { v.mounted().map(PathBuf::from) };
        out.push(Card { id: v.id.clone(), what: What::Volume(v.id.clone()), name, kind, icon, at, size: Some(v.size).filter(|s| *s > 0),
                        locked: v.locked, eject: !v.fixed });
    }
    for (address, name) in phones {
        out.push(Card { id: format!("phone:{address}"), what: What::Phone(address.clone()), name: name.clone(), kind: t!("folders-computer-kind-phone"),
                        icon: stand_in::PHONE, at: None, size: None, locked: false, eject: false });
    }
    for m in shares {
        out.push(Card { id: format!("share:{}", m.at.display()), what: What::Share, name: m.name(), kind: t!("folders-computer-kind-share"),
                        icon: stand_in::SERVER, at: Some(m.at.clone()), size: None, locked: false, eject: true });
    }
    for at in network.iter().filter(|at| !shares.iter().any(|m| m.at == **at)) {
        out.push(Card { id: format!("mount:{}", at.display()), what: What::Mount, name: super::kinds::shown_name(at), kind: t!("folders-computer-kind-share"),
                        icon: stand_in::SERVER, at: Some(at.clone()), size: None, locked: false, eject: false });
    }
    let names: Vec<String> = out.iter().map(|c| c.name.clone()).collect();
    for c in &mut out {
        if names.iter().filter(|n| **n == c.name).count() > 1 {
            if let Some(v) = volumes.iter().find(|v| v.id == c.id) {
                c.name = t!("folders-computer-name-on", name = &c.name, drive = &v.drive_name);
            }
        }
    }
    out
}

pub fn words(card: &Card, full: Option<(u64, u64)>) -> String {
    let size = card.size.map(size_words);
    match (full, &card.at) {
        (Some((used, free)), _) => {
            let said = t!("folders-computer-free-of", free = size_words(free), all = size_words(used.saturating_add(free)));
            match share(used, free) {
                s if s >= WORRY.1 => t!("folders-computer-almost-full", said = &said),
                s if s >= WORRY.0 => t!("folders-computer-nearly-full", said = &said),
                _ => said,
            }
        }
        (None, Some(_)) => size.unwrap_or_default(),
        (None, None) if card.locked => size.map_or_else(|| t!("folders-computer-locked").into(), |s| t!("folders-computer-locked-size", size = s)),
        (None, None) if matches!(card.what, What::Phone(_)) => t!("folders-computer-phone-hint").into(),
        (None, None) => size.map_or_else(|| t!("folders-computer-not-mounted").into(), |s| t!("folders-computer-not-mounted-size", size = s)),
    }
}

fn share(used: u64, free: u64) -> f64 {
    let all = used.saturating_add(free);
    if all == 0 { 0.0 } else { used as f64 / all as f64 }
}

fn place(places: &mut Vec<String>, now: &[Card]) {
    for c in now {
        if !places.contains(&c.id) {
            places.push(c.id.clone());
        }
    }
    while places.last().is_some_and(|id| !now.iter().any(|c| c.id == *id)) {
        places.pop();
    }
}

pub fn count_words(n: usize) -> String {
    t!("folders-computer-drive-count", n = n)
}

impl Files {
    fn computer_shown(&self) -> bool {
        (0..2).any(|p| self.shown_pane(p) && self.cols[p].at == Where::Computer)
    }

    pub(super) fn computer_again(&mut self) {
        self.computer.again = true;
    }

    fn computer_network(&self) -> Vec<PathBuf> {
        #[cfg(test)]
        {
            self.computer.network.clone()
        }
        #[cfg(not(test))]
        {
            super::places::network(&std::fs::read_to_string("/proc/self/mounts").unwrap_or_default())
        }
    }

    pub(super) fn computer_wanted(&mut self) {
        if !self.computer_shown() {
            self.computer.places.clear();
            self.computer.cards.clear();
            self.computer.asked.clear();
            return;
        }
        let volumes: Vec<Volume> = self.drives.fixed.iter().chain(&self.drives.list).cloned().collect();
        let phones: Vec<(String, String)> = self.far.places().into_iter().filter_map(|w| match &w {
            Where::Remote(a) if a.starts_with("mtp:") => Some((a.clone(), self.place_name(&w))),
            _ => None,
        }).collect();
        let now = cards(&volumes, self.share_places(), &phones, &self.computer_network());
        place(&mut self.computer.places, &now);
        self.computer.cards = now;
        let mounted: Vec<PathBuf> = self.computer.cards.iter().filter_map(|c| c.at.clone()).collect();
        if !std::mem::take(&mut self.computer.again) && mounted == self.computer.asked {
            return;
        }
        self.computer.asked = mounted.clone();
        self.computer.id += 1;
        let id = self.computer.id;
        #[cfg(test)]
        let reader: Reader = self.computer.reader.clone().unwrap_or_else(|| Arc::new(|_| None));
        #[cfg(not(test))]
        let reader: Reader = Arc::new(crate::common::files::how_full);
        let read = move || mounted.into_iter().map(|at| { let full = reader(&at); (at, full) }).collect::<Vec<_>>();
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name("folders-computer".into()).spawn(move || {
                    post.send(Msg::Computer(ComputerMsg::Full(id, read())));
                });
            }
            None => self.computer.full = read().into_iter().filter_map(|(at, full)| Some((at, full?))).collect(),
        }
    }

    pub(super) fn computer_msg(&mut self, m: ComputerMsg) {
        let card = |f: &Files, id: &str| f.computer.cards.iter().find(|c| c.id == id).cloned();
        match m {
            ComputerMsg::Full(id, read) => {
                if id == self.computer.id {
                    self.computer.full = read.into_iter().filter_map(|(at, full)| Some((at, full?))).collect();
                }
            }
            ComputerMsg::Open(p, id) => {
                let Some(c) = card(self, &id) else { return };
                if matches!(&c.what, What::Volume(v) if self.drives.is_busy(v)) {
                    self.say_help(super::drives::Drives::busy().to_string(), super::help::DRIVE);
                    return;
                }
                match (&c.what, c.at) {
                    (What::Phone(address), _) => self.go(p, Where::Remote(address.clone())),
                    (_, Some(at)) => self.go(p, Where::Folder(at)),
                    (_, None) => {}
                }
            }
            ComputerMsg::Mount(p, id) => {
                let Some(c) = card(self, &id) else { return };
                let What::Volume(volume) = c.what else { return };
                self.side = p;
                self.drive_show(&volume);
            }
            ComputerMsg::Eject(id) => {
                let Some(c) = card(self, &id).filter(|c| c.eject) else { return };
                match c.what {
                    What::Volume(volume) => self.drive_eject(&volume),
                    What::Share => {
                        if let Some(i) = self.share_places().iter().position(|m| Some(&m.at) == c.at.as_ref()) {
                            self.share_msg(super::shares::ShareMsg::Eject(i));
                        }
                    }
                    What::System | What::Phone(_) | What::Mount => {}
                }
            }
        }
    }

    pub(super) fn computer_words(&self) -> String {
        count_words(self.computer.cards.len())
    }

    pub(super) fn computer_laid(&self) -> Vec<Option<&Card>> {
        self.computer.places.iter().map(|id| self.computer.cards.iter().find(|c| c.id == *id)).collect()
    }

    #[cfg(test)]
    pub(super) fn computer_full(&self, at: &Path) -> Option<(u64, u64)> {
        self.computer.full.get(at).copied()
    }
}

pub fn page(f: &Files, p: usize) -> Node<Msg> {
    let cards: Vec<Node<Msg>> = f.computer_laid().into_iter().map(|c| match c {
        Some(c) => one(f, p, c),
        None => gone(),
    }).collect();
    let grid = even_grid(theme::sheet().drive_card, cards);
    scroll(format!("computer{p}:{}", f.cols[p].tab), grid).width(Length::Fill).height(Length::Fill)
}

fn gone() -> Node<Msg> {
    let badge = theme::sheet().badge;
    card(column(vec![
        row(vec![
            space().width(Length::Fixed(badge)).height(Length::Fixed(badge)),
            column(vec![heading(" ").cut().width(Length::Fill), note(" ").cut().width(Length::Fill)]).gap(2.0).width(Length::Fill),
        ]).gap(8.0).width(Length::Fill).centred(),
        fill_bar(None, ""),
        note("").room_for(1).width(Length::Fill),
    ]).gap(theme::sheet().gap).width(Length::Fill)).width(Length::Fill).opacity(0.0)
}

fn one(f: &Files, p: usize, c: &Card) -> Node<Msg> {
    let full = c.at.as_ref().and_then(|at| f.computer.full.get(at).copied());
    let busy = matches!(&c.what, What::Volume(id) if f.drives.is_busy(id));
    let mut head = vec![
        icon_circle(c.icon),
        column(vec![heading(c.name.clone()).cut().width(Length::Fill), note(c.kind).cut().width(Length::Fill)]).gap(2.0).width(Length::Fill),
    ];
    let opens = c.at.is_some() || matches!(c.what, What::Phone(_));
    if opens {
        head.push(button("folder-open-symbolic", t!("folders-computer-open", name = &c.name)).on_press(Msg::Computer(ComputerMsg::Open(p, c.id.clone()))).off_because(crate::common::ui::wait_while(busy)));
    } else if c.locked {
        head.push(button(stand_in::UNLOCK, t!("folders-computer-unlock", name = &c.name)).on_press(Msg::Computer(ComputerMsg::Mount(p, c.id.clone()))).off_because(crate::common::ui::wait_while(busy)));
    } else {
        head.push(button(stand_in::MOUNT, t!("folders-computer-mount", name = &c.name)).on_press(Msg::Computer(ComputerMsg::Mount(p, c.id.clone()))).off_because(crate::common::ui::wait_while(busy)));
    }
    if c.eject {
        let tip = if matches!(c.what, What::Share) { t!("folders-computer-close", name = &c.name) } else { t!("folders-computer-eject", name = &c.name) };
        head.push(button(stand_in::EJECT, tip).on_press(Msg::Computer(ComputerMsg::Eject(c.id.clone()))).off_because(crate::common::ui::wait_while(busy)));
    }
    let said = if busy { super::drives::Drives::busy().to_string() } else { words(c, full) };
    let bar_tip = match full {
        Some((used, free)) => t!("folders-computer-percent-full", pct = (share(used, free) * 100.0).round().to_string()),
        None => String::new(),
    };
    card(column(vec![
        row(head).gap(8.0).width(Length::Fill).centred(),
        fill_bar(full.map(|(used, free)| share(used, free)), bar_tip),
        note(said).room_for(1).width(Length::Fill),
    ]).gap(theme::sheet().gap).width(Length::Fill)).width(Length::Fill)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::drives::DriveMsg;
    use crate::toolkit::ui::App;

    fn vol(id: &str, label: &str, mounts: &[&str], kind: Kind, fixed: bool) -> Volume {
        Volume {
            id: id.into(), dev: id.into(), label: label.into(), size: 32_000_000_000,
            drive: format!("/drive/{id}"), drive_name: format!("Acme {id}"), kind,
            mounts: mounts.iter().map(|m| m.to_string()).collect(), locked: false, container: None,
            can_eject: !fixed, can_power_off: !fixed, fixed, system: fixed && mounts.contains(&"/"), key: String::new(), drive_key: String::new(),
        }
    }

    fn drives(stick_at: &str) -> Vec<Volume> {
        let mut locked = vol("sdd1", "", &[], Kind::Usb, false);
        locked.locked = true;
        vec![vol("sdb1", "HOLIDAY", &[stick_at], Kind::Usb, false), vol("nvme0n1p3", "Trindle", &["/home", "/"], Kind::Other, true),
             vol("sda1", "Films", &[], Kind::Other, true), vol("mmc0p1", "PHOTOS", &[], Kind::Card, false), locked]
    }

    fn files() -> Files {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.set_drives(std::rc::Rc::default(), |_| {});
        f
    }

    #[test]
    fn the_page_is_called_storage_and_its_old_name_shows_nowhere() {
        use crate::files::places::page_name;
        assert_eq!(page_name(), "Storage");
        let mut f = files();
        f.update(Msg::Drive(DriveMsg::Listed(drives("/run/media/john/HOLIDAY"))));
        let shown = |f: &Files| {
            let mut all = crate::files::view::view(f).said();
            for col in &f.cols {
                for tab in &col.tabs {
                    all.push(f.tab_name(&tab.at));
                    all.extend(crate::files::places::crumbs(&tab.at, &f.home).into_iter().map(|(name, _)| name));
                }
            }
            all.join("\n")
        };
        assert!(shown(&f).lines().any(|l| l.trim() == page_name()), "the sidebar's row: {}", shown(&f));
        for two in [false, true] {
            f.update(Msg::PanesAre(two));
            f.go(0, Where::Computer);
            f.update(Msg::NewTab(Some(0)));
            f.kept.set_panel(true, None);
            let said = shown(&f);
            assert!(said.lines().filter(|l| *l == page_name()).count() >= 3, "the sidebar, the tab and the path say it: {said}");
            let rest = said.replace("This computer's disk", "").replace("A disk in this computer", "");
            assert!(!rest.to_lowercase().contains("this computer"), "the old name is still shown: {rest}");
            assert!(!said.lines().any(|l| l.trim() == "Computer"), "nor the name before that: {said}");
        }
        assert_eq!(f.tab_name(&Where::Computer), page_name());
        assert_eq!(t!("folders-view-drives"), "Devices");
        assert_eq!(f.place_list()[0], Where::Computer);
    }

    fn names(f: &Files) -> Vec<&str> {
        f.computer_laid().into_iter().map(|c| c.map_or("", |c| c.name.as_str())).collect()
    }

    #[test]
    fn a_hidden_drive_is_gone_from_the_sidebar_and_from_storage_and_comes_back() {
        use std::collections::BTreeSet;
        use crate::common::volumes::unhidden;
        let mut list = drives("/run/media/john/HOLIDAY");
        for v in &mut list {
            (v.key, v.drive_key) = (format!("fs:uuid-{}", v.dev), format!("drive:Acme-{}", v.dev.trim_end_matches(char::is_numeric)));
        }
        let side = |f: &Files| -> Vec<String> { super::super::drives::rows(&f.drives.list, None).into_iter().map(|r| r.name).collect() };
        let page = |f: &Files| -> Vec<String> {
            let every: Vec<Volume> = f.drives.fixed.iter().chain(&f.drives.list).cloned().collect();
            cards(&every, &[], &[], &[]).into_iter().map(|c| c.name).collect()
        };
        let mut f = files();
        f.update(Msg::Drive(DriveMsg::Listed(unhidden(list.clone(), &BTreeSet::new()))));
        assert_eq!(side(&f), ["HOLIDAY", "PHOTOS", "Locked: 32.0 GB"]);
        assert_eq!(page(&f), ["Trindle", "Films", "HOLIDAY", "PHOTOS", "32.0 GB"]);
        let hidden = BTreeSet::from(["drive:Acme-sdb".to_string(), "fs:uuid-sda1".to_string()]);
        f.update(Msg::Drive(DriveMsg::Listed(unhidden(list.clone(), &hidden))));
        assert_eq!(side(&f), ["PHOTOS", "Locked: 32.0 GB"]);
        f.go(0, Where::Computer);
        let said = super::super::view::view(&f).said();
        assert!(!said.iter().any(|s| s.contains("HOLIDAY") || s.contains("Films")), "{said:?}");
        assert!(said.iter().any(|s| s.contains("PHOTOS")), "{said:?}");
        f.update(Msg::Drive(DriveMsg::Listed(unhidden(list, &BTreeSet::new()))));
        assert_eq!(side(&f), ["HOLIDAY", "PHOTOS", "Locked: 32.0 GB"]);
        let said = super::super::view::view(&f).said();
        assert!(said.iter().any(|s| s.contains("HOLIDAY")) && said.iter().any(|s| s.contains("Films")), "{said:?}");
        assert!(crate::common::source::code(include_str!("mod.rs")).contains("volumes::watch_all(&system, events,"));
    }

    #[test]
    fn every_kind_of_drive_is_a_card_with_the_system_first() {
        let shares = [Mounted { server: "nas".into(), share: "Photos".into(), at: "/run/media/john/Photos on nas".into() }];
        let phones = [("mtp://1-2/".to_string(), "John's phone".to_string())];
        let net = [PathBuf::from("/mnt/backup"), PathBuf::from("/run/media/john/Photos on nas")];
        let c = cards(&drives("/run/media/john/HOLIDAY"), &shares, &phones, &net);
        let said: Vec<(&str, &str)> = c.iter().map(|c| (c.name.as_str(), c.kind)).collect();
        assert_eq!(said, [("Trindle", "This computer's disk"), ("Films", "A disk in this computer"), ("HOLIDAY", "USB drive"),
                          ("PHOTOS", "Memory card"), ("32.0 GB", "USB drive"), ("John's phone", "Phone or tablet"),
                          ("Photos on nas", "Network share"), ("backup", "Network share")]);
        assert_eq!(c[0].at.as_deref(), Some(Path::new("/")));
        assert_eq!(c.iter().map(|c| c.eject).collect::<Vec<_>>(), [false, false, true, true, true, false, true, false]);
        assert!(c[4].locked && c[4].icon == "changes-prevent-symbolic");
        let alone = cards(&[], &[], &[], &[]);
        assert_eq!(alone.len(), 1);
        assert_eq!((alone[0].name.as_str(), alone[0].at.as_deref(), &alone[0].what), ("System disk", Some(Path::new("/")), &What::System));
        let twins = cards(&[vol("a1", "BACKUP", &[], Kind::Usb, false), vol("b1", "BACKUP", &[], Kind::Usb, false)], &[], &[], &[]);
        assert_eq!((twins[1].name.as_str(), twins[2].name.as_str()), ("BACKUP on Acme a1", "BACKUP on Acme b1"));
    }

    #[test]
    fn a_card_says_how_full_it_is_in_words_as_well_as_by_its_bar() {
        let c = cards(&drives("/run/media/john/HOLIDAY"), &[], &[], &[]);
        let gb = 1_000_000_000u64;
        assert_eq!(words(&c[0], Some((244 * gb, 12 * gb))), "Almost full: 12.0 GB free of 256.0 GB");
        assert_eq!(words(&c[0], Some((100 * gb, 156 * gb))), "156.0 GB free of 256.0 GB");
        assert_eq!(words(&c[2], Some((28 * gb, 4 * gb))), "Nearly full: 4.0 GB free of 32.0 GB");
        assert_eq!(words(&c[2], None), "32.0 GB");
        assert_eq!(words(&c[1], None), "Not mounted, 32.0 GB");
        assert_eq!(words(&c[4], None), "Locked, 32.0 GB");
        assert_eq!((count_words(1), count_words(5)), ("1 drive".to_string(), "5 drives".to_string()));
    }

    #[test]
    fn the_page_reads_how_full_things_are_and_keeps_its_cards_in_place() {
        let stick = std::env::temp_dir().join(format!("trindle-files-computer-{}", std::process::id()));
        std::fs::create_dir_all(&stick).unwrap();
        let at = stick.to_string_lossy().into_owned();
        let mut f = files();
        let gb = 1_000_000_000u64;
        let there = stick.clone();
        f.computer.reader = Some(Arc::new(move |p: &Path| if p == there { Some((28 * gb, 4 * gb)) } else if p == Path::new("/") { Some((100 * gb, 156 * gb)) } else { None }));
        f.update(Msg::Drive(DriveMsg::Listed(drives(&at))));
        assert!(f.computer.cards.is_empty() && f.computer_full(Path::new("/")).is_none());
        assert_eq!(f.drives.list.len(), 3);
        f.go(0, Where::Computer);
        assert_eq!(names(&f), ["Trindle", "Films", "HOLIDAY", "PHOTOS", "32.0 GB"]);
        assert_eq!(f.computer_full(Path::new("/")), Some((100 * gb, 156 * gb)));
        assert_eq!(f.computer_full(&stick), Some((28 * gb, 4 * gb)));
        assert_eq!(f.computer_words(), "5 drives");
        let mut fewer = drives(&at);
        fewer.retain(|v| v.id != "mmc0p1");
        f.update(Msg::Drive(DriveMsg::Listed(fewer.clone())));
        assert_eq!(names(&f), ["Trindle", "Films", "HOLIDAY", "", "32.0 GB"]);
        assert_eq!(f.computer_words(), "4 drives");
        fewer.insert(0, vol("sdz1", "AAA FIRST", &[], Kind::Usb, false));
        fewer.insert(0, vol("sdy1", "Inside", &[], Kind::Other, true));
        f.update(Msg::Drive(DriveMsg::Listed(fewer.clone())));
        assert_eq!(names(&f), ["Trindle", "Films", "HOLIDAY", "", "32.0 GB", "Inside", "AAA FIRST"]);
        fewer.push(vol("mmc0p1", "PHOTOS", &[], Kind::Card, false));
        f.update(Msg::Drive(DriveMsg::Listed(fewer.clone())));
        assert_eq!(names(&f), ["Trindle", "Films", "HOLIDAY", "PHOTOS", "32.0 GB", "Inside", "AAA FIRST"]);
        fewer.retain(|v| v.id != "sdz1");
        f.update(Msg::Drive(DriveMsg::Listed(fewer.clone())));
        assert_eq!(names(&f), ["Trindle", "Films", "HOLIDAY", "PHOTOS", "32.0 GB", "Inside"]);
        f.go(0, Where::Folder(std::env::temp_dir()));
        assert!(f.computer.cards.is_empty());
        f.go(0, Where::Computer);
        assert_eq!(names(&f), ["Trindle", "Inside", "Films", "HOLIDAY", "32.0 GB", "PHOTOS"]);
        let old = f.computer.id;
        f.update(Msg::Reload);
        assert!(f.computer.id > old, "reading the page again asks the disks again");
        f.update(Msg::Computer(ComputerMsg::Full(old, vec![(PathBuf::from("/"), Some((1, 1)))])));
        assert_eq!(f.computer_full(Path::new("/")), Some((100 * gb, 156 * gb)));
        let _ = std::fs::remove_dir_all(stick);
    }

    #[test]
    fn a_card_opens_mounts_or_ejects_and_the_top_of_the_disk_leads_back_to_the_page() {
        let stick = std::env::temp_dir().join(format!("trindle-files-computer-acts-{}", std::process::id()));
        std::fs::create_dir_all(&stick).unwrap();
        let mut f = files();
        f.update(Msg::Drive(DriveMsg::Listed(drives(&stick.to_string_lossy()))));
        f.go(0, Where::Computer);
        let id = |f: &Files, name: &str| f.computer.cards.iter().find(|c| c.name == name).unwrap().id.clone();
        f.update(Msg::Computer(ComputerMsg::Open(0, id(&f, "HOLIDAY"))));
        assert_eq!(f.cols[0].at, Where::Folder(stick.clone()));
        f.go(0, Where::Computer);
        f.update(Msg::Computer(ComputerMsg::Open(0, id(&f, "Trindle"))));
        assert_eq!(f.cols[0].at, Where::Folder(PathBuf::from("/")));
        assert_eq!(f.cols[0].at.name(&f.home), "System disk");
        f.update(Msg::Up);
        assert_eq!(f.cols[0].at, Where::Computer);
        assert_eq!(f.cols[0].at.name(&f.home), crate::files::places::page_name());
        f.update(Msg::Back);
        assert_eq!(f.cols[0].at, Where::Folder(PathBuf::from("/")));
        f.go(0, Where::Computer);
        f.update(Msg::Computer(ComputerMsg::Mount(0, id(&f, "Films"))));
        assert!(f.drives.is_busy("sda1") && f.said.contains("Opening Films"), "{}", f.said);
        f.update(Msg::Drive(DriveMsg::Mounted("sda1".into(), Ok(stick.to_string_lossy().into_owned()))));
        assert_eq!(f.cols[0].at, Where::Folder(stick.clone()));
        f.go(0, Where::Computer);
        f.update(Msg::Computer(ComputerMsg::Eject(id(&f, "PHOTOS"))));
        assert!(f.drives.is_busy("mmc0p1") && f.said.contains("Putting PHOTOS away"), "{}", f.said);
        f.said.clear();
        f.update(Msg::Computer(ComputerMsg::Eject(id(&f, "PHOTOS"))));
        assert_eq!(f.said, super::super::drives::Drives::busy());
        f.said.clear();
        f.drive_show("mmc0p1");
        assert_eq!(f.said, super::super::drives::Drives::busy());
        f.said.clear();
        f.update(Msg::Computer(ComputerMsg::Eject(id(&f, "Films"))));
        assert!(!f.drives.is_busy("sda1") && f.said.is_empty(), "{}", f.said);
        assert_eq!(f.place_list()[0], Where::Computer);
        assert!(!crate::files::inplace::can_rename(&Where::Computer) && !f.searchable(0));
        assert!(crate::files::menu::lines_for(&Where::Computer, &[], false).is_empty());
        let _ = std::fs::remove_dir_all(stick);
    }
}
