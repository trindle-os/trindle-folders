// What Files does with each message: the routing to the topic that answers it, and the rules that hold after every one.

use std::path::PathBuf;

use super::*;

impl Files {
    pub(super) fn pick_in_one_pane(&mut self) {
        let here = self.side();
        if self.cols[here].picked.is_empty() {
            return;
        }
        let other = &mut self.cols[1 - here];
        if !other.picked.is_empty() {
            other.picked.clear();
            other.anchor = None;
        }
    }

    pub(super) fn handle(&mut self, msg: Msg) {
        self.unlock_forget_closed();
        if matches!(msg, Msg::Pin) && self.pane().at.folder().is_none() {
            return;
        }
        let to_list = matches!(msg, Msg::Choose(..) | Msg::Pick(..) | Msg::Icon(..) | Msg::Step(..) | Msg::Typed(_));
        let typed = matches!(msg, Msg::Find(crate::files::found::FindMsg::Text(_)));
        let list_own = keeps_the_keyboard(&msg);
        self.dispatch(msg);
        self.outside_tidy();
        self.rename_follow();
        self.locate_follow();
        self.hand_back_the_keyboard(to_list, typed, list_own);
        self.sizes_wanted();
        self.pictures_wanted();
        self.media_wanted();
        self.computer_wanted();
        self.panel_wanted();
    }

    fn dispatch(&mut self, msg: Msg) {
        match msg {
            m @ (Msg::Choose(..) | Msg::Activate(_) | Msg::Pick(..) | Msg::All | Msg::TickAll(_) | Msg::Unpick | Msg::Invert | Msg::SameInOther
                 | Msg::SwapPanes | Msg::ComparePanes | Msg::PickDiffering | Msg::Icon(..) | Msg::Head(..) | Msg::Step(..) | Msg::Typed(_)) => self.pick_msg(m),
            m @ (Msg::Part(..) | Msg::Place(_) | Msg::Pinned(_) | Msg::Travel(..) | Msg::Back | Msg::Forward | Msg::Chooser(_) | Msg::Reload
                 | Msg::GiveUp | Msg::Home | Msg::Up | Msg::Enter | Msg::Add(_) | Msg::Reveal(..) | Msg::Props | Msg::Terminal) => self.go_msg(m),
            m @ (Msg::Locate | Msg::LocateLine(_) | Msg::LocateText(_) | Msg::LocateDone | Msg::LocateStop) => self.locate_msg(m),
            m @ (Msg::Tab(..) | Msg::NewTab(_) | Msg::CloseTab(_) | Msg::NewWindow | Msg::Middle(_) | Msg::PartTab(..) | Msg::NextTab(_)
                 | Msg::OtherPane | Msg::Panes | Msg::PanesAre(_) | Msg::Link) => self.tab_msg(m),
            m @ (Msg::PinAt(_) | Msg::PinMove(..) | Msg::PinNameText(_) | Msg::PinNameDone | Msg::PinNameStop | Msg::Pin) => self.sidebar_msg(m),
            m @ (Msg::Settings | Msg::Own(_) | Msg::Hidden | Msg::Look(_) | Msg::SortBy(_) | Msg::GroupBy(_) | Msg::Zoom(..) | Msg::Flip
                 | Msg::Changed | Msg::Resized) => self.look_msg(m),
            m @ (Msg::Said(..) | Msg::Idle | Msg::Listed | Msg::NoticeGone(_) | Msg::Help(_)) => self.said_msg(m),
            m @ (Msg::Below(_) | Msg::Menu(_) | Msg::Ask(_) | Msg::Tool(_) | Msg::Far(_) | Msg::Share(_) | Msg::Drive(_) | Msg::Sharing(_)
                 | Msg::Inside(_) | Msg::Read(_) | Msg::ReadLate(_) | Msg::Unlock(_) | Msg::Find(_) | Msg::Pictured(..) | Msg::Preview(_)
                 | Msg::Panel(_) | Msg::Computer(_) | Msg::Earlier(_) | Msg::Sized(..) | Msg::Media(..) | Msg::Neighbours(_) | Msg::Announced(_)
                 | Msg::Looked(_)) => self.part_msg(m),
            m @ (Msg::Outside(..) | Msg::OutsideAnswer(_) | Msg::OutEnded(_) | Msg::OutShort(_) | Msg::OutUnshown(_) | Msg::OutsideLooked(..)
                 | Msg::OutsideLate(_) | Msg::OutLeft(..)) => self.outside_msg(m),
            m @ (Msg::Copy | Msg::Move | Msg::Rename | Msg::Cut | Msg::CopyClip | Msg::Paste | Msg::Pasted(_) | Msg::Trash
                 | Msg::Delete | Msg::DeleteSure | Msg::NoDelete | Msg::EmptyTrash | Msg::Restore | Msg::RenameText(_) | Msg::RenameDone
                 | Msg::RenameKeep | Msg::RenameNext(_) | Msg::SlowRename(_) | Msg::RenameStop | Msg::NewFolder | Msg::NewDocument | Msg::Undo | Msg::Redo | Msg::Work(..) | Msg::WorkGone(_) | Msg::Waiting | Msg::Pause(_)
                 | Msg::Cancel(_) | Msg::Answer(..) | Msg::AllSame(..) | Msg::DropOn(_) | Msg::DropStop | Msg::Landed(_) | Msg::Escape) => self.act(m),
        }
    }

    fn hand_back_the_keyboard(&mut self, to_list: bool, typed: bool, list_own: bool) {
        if !list_own {
            self.turn += 1;
        }
        if to_list {
            self.find.typing = false;
        } else if typed || !list_own {
            self.find.typing = true;
        }
    }

    pub(super) fn look_msg(&mut self, m: Msg) {
        match m {
            Msg::Settings => {
                self.read_settings();
                self.earlier_look();
            }
            Msg::Own(m) => self.own_msg(m),
            Msg::Hidden => {
                self.hidden = !self.hidden;
                self.below_forget();
                self.reload_all();
            }
            Msg::Look(v) => self.look(v),
            Msg::SortBy(i) => self.sort_on(i.checked_sub(1).and_then(|k| crate::files::columns::Col::ALL.get(k).copied())),
            Msg::GroupBy(i) => self.group_by(i),
            Msg::Zoom(way, pointed) => self.zoom(way, pointed),
            Msg::Flip => {
                self.up = !self.up;
                self.reload_all();
            }
            Msg::Changed => self.changed_on_disk(),
            Msg::Resized => {}
            _ => unreachable!("not a look message"),
        }
    }

    fn look(&mut self, v: View) {
        self.view = v;
        let at = self.pane().at.clone();
        if self.kept.view(&at, v).is_err() {
            self.say_help(t!("folders-app-view-not-kept"), crate::files::help::NOT_KEPT);
        }
    }

    fn group_by(&mut self, i: usize) {
        let by = crate::files::groups::GroupBy::ALL[i.min(crate::files::groups::GroupBy::ALL.len() - 1)];
        let at = self.pane().at.clone();
        if self.kept.set_group(&at, by).is_err() {
            self.say_help(t!("folders-app-grouping-not-kept"), crate::files::help::NOT_KEPT);
        }
        self.reload_all();
    }

    fn changed_on_disk(&mut self) {
        let news: Vec<PathBuf> = self.news.borrow_mut().drain(..).collect();
        self.far_saved(&news);
        for n in &news {
            self.sizes_changed(n);
        }
        for p in 0..2 {
            for t in 0..self.cols[p].tabs.len() {
                if news.iter().any(|n| super::listing::touches(&self.cols[p].tabs[t].at, n, &self.trash_dirs)) {
                    self.load(t, p);
                }
            }
        }
    }

    fn said_msg(&mut self, m: Msg) {
        match m {
            Msg::Said(words, bad) => self.say(words, bad),
            Msg::Idle => {}
            Msg::Listed => self.reasons_open = !self.reasons_open && self.reasons.as_ref().is_some_and(|(w, _)| *w == self.said),
            Msg::NoticeGone(n) => {
                if self.notice.as_ref().is_some_and(|(k, _)| *k == n) {
                    self.notice = None;
                }
            }
            Msg::Help(address) => self.help_opens(address),
            _ => unreachable!("not a saying"),
        }
    }

    fn part_msg(&mut self, m: Msg) {
        match m {
            Msg::Below(m) => self.below_msg(m),
            Msg::Menu(m) => self.menu_msg(m),
            Msg::Ask(m) => self.ask_msg(m),
            Msg::Tool(m) => self.tool_msg(m),
            Msg::Far(m) => self.far_msg(m),
            Msg::Share(m) => self.share_msg(m),
            Msg::Drive(m) => self.drive_msg(m),
            Msg::Sharing(m) => self.sharing_msg(m),
            Msg::Inside(m) => self.inside_msg(m),
            Msg::Read(got) => self.read_arrived(*got),
            Msg::ReadLate(id) => self.read_late(id),
            Msg::Unlock(m) => self.unlock_msg(m),
            Msg::Find(m) => self.find_msg(m),
            Msg::Pictured(file, thumb) => self.pictured(file, thumb),
            Msg::Preview(m) => self.preview_msg(m),
            Msg::Panel(m) => self.panel_msg(m),
            Msg::Computer(m) => self.computer_msg(m),
            Msg::Earlier(m) => self.earlier_msg(m),
            Msg::Sized(folder, t, done) => self.sized(folder, t, done),
            Msg::Media(path, when, m) => self.media_read(path, when, m),
            Msg::Neighbours(found) => self.neighbours_found(found),
            Msg::Announced(found) => self.neighbours_announced(found),
            Msg::Looked(which) => self.neighbours_looked(which),
            _ => unreachable!("not for another part"),
        }
    }

    fn outside_msg(&mut self, m: Msg) {
        match m {
            Msg::Outside(onto, given, way) => self.outside_drop(onto, given, way),
            Msg::OutsideAnswer(bring) => self.outside_answer(bring),
            Msg::OutEnded(took) => self.outside_ended(took),
            Msg::OutShort(way) => self.outside_short(way),
            Msg::OutUnshown(way) => self.outside_unshown(way),
            Msg::OutsideLooked(n, looked) => self.outside_looked(n, *looked),
            Msg::OutsideLate(n) => self.outside_late(n),
            Msg::OutLeft(left, taken) => self.outside_left(left, taken),
            _ => unreachable!("not about things held"),
        }
    }
}

fn keeps_the_keyboard(msg: &Msg) -> bool {
    matches!(msg, Msg::Choose(..) | Msg::Activate(_) | Msg::Pick(..) | Msg::Icon(..) | Msg::Step(..) | Msg::Typed(_) | Msg::Head(..) | Msg::TickAll(_) | Msg::Zoom(..) | Msg::Changed | Msg::Settings | Msg::Own(_) | Msg::Resized | Msg::Work(..) | Msg::NoticeGone(_) | Msg::Listed | Msg::Idle
                    | Msg::WorkGone(_) | Msg::Waiting | Msg::Landed(_) | Msg::PinAt(_) | Msg::Below(crate::files::below::BelowMsg::Open(..) | crate::files::below::BelowMsg::Got(..)) | Msg::Pause(_) | Msg::Cancel(_) | Msg::Answer(..) | Msg::AllSame(..) | Msg::RenameText(_) | Msg::LocateText(_) | Msg::PinNameText(_) | Msg::PinNameDone | Msg::PinNameStop | Msg::Said(..)
                    | Msg::RenameKeep | Msg::SlowRename(_)
                    | Msg::Pasted(_) | Msg::Menu(crate::files::menu::MenuMsg::Pop(PopupMsg::Hover(_) | PopupMsg::Stay | PopupMsg::Move(_) | PopupMsg::Point { .. } | PopupMsg::SubHover(_) | PopupMsg::Into | PopupMsg::Out | PopupMsg::Due(_))) | Msg::Tool(_)
                    | Msg::Far(crate::files::far::FarMsg::Listed { .. } | crate::files::far::FarMsg::Fetched { .. } | crate::files::far::FarMsg::SentBack(..) | crate::files::far::FarMsg::Plugged | crate::files::far::FarMsg::Renamed { .. } | crate::files::far::FarMsg::Made { .. }
                               | crate::files::far::FarMsg::Address(_) | crate::files::far::FarMsg::Password(_) | crate::files::far::FarMsg::Key(_))
                    | Msg::Share(crate::files::shares::ShareMsg::Listed { got: Ok(_), .. } | crate::files::shares::ShareMsg::Mounted { got: Ok(_), .. } | crate::files::shares::ShareMsg::Ejected(..)
                                 | crate::files::shares::ShareMsg::User(_) | crate::files::shares::ShareMsg::Password(_) | crate::files::shares::ShareMsg::Remember(_))
                    | Msg::Drive(crate::files::drives::DriveMsg::Listed(_) | crate::files::drives::DriveMsg::Mounted(..) | crate::files::drives::DriveMsg::Ejected(..))
                    | Msg::Sharing(crate::files::sharing::SharingMsg::Name(_) | crate::files::sharing::SharingMsg::Password(_)
                                   | crate::files::sharing::SharingMsg::Listed(_) | crate::files::sharing::SharingMsg::SetUp(..)
                                   | crate::files::sharing::SharingMsg::Made(..) | crate::files::sharing::SharingMsg::PasswordSaved(_)
                                   | crate::files::sharing::SharingMsg::Connecting(_))
                    | Msg::Inside(_) | Msg::Read(_) | Msg::ReadLate(_) | Msg::Unlock(crate::files::locked::UnlockMsg::Password(_)) | Msg::Sized(..) | Msg::Media(..) | Msg::Neighbours(_) | Msg::Announced(_) | Msg::Looked(_) | Msg::Pictured(..) | Msg::Preview(crate::files::preview::PreviewMsg::Loaded(..) | crate::files::preview::PreviewMsg::Drawn(..) | crate::files::preview::PreviewMsg::Tick(_)
                                   | crate::files::preview::PreviewMsg::Seek(_) | crate::files::preview::PreviewMsg::Volume(_))
                    | Msg::Panel(crate::files::panel::PanelMsg::Got(..) | crate::files::panel::PanelMsg::Edge(_))
                    | Msg::Computer(crate::files::computer::ComputerMsg::Full(..))
                    | Msg::Earlier(crate::files::earlier::EarlierMsg::Any(_) | crate::files::earlier::EarlierMsg::Listed(..)
                                   | crate::files::earlier::EarlierMsg::PutBackDone(..) | crate::files::earlier::EarlierMsg::Compared(..))
                    | Msg::Find(crate::files::found::FindMsg::Text(_) | crate::files::found::FindMsg::Found(..))
                    | Msg::Ask(crate::files::dialogs::AskMsg::SumsFar(..) | crate::files::dialogs::AskMsg::SumsGot(..) | crate::files::dialogs::AskMsg::SumsCheck(_)
                               | crate::files::dialogs::AskMsg::Always(_) | crate::files::dialogs::AskMsg::Page(_)))
}
