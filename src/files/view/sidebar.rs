//! The side bar of the Files window: Places, Pinned, Devices, Network and Shares.

use super::super::act::Land;
use super::super::app::{Files, Msg};
use super::super::drives::{self, DriveMsg};
use super::super::far::FarMsg;
use super::super::icons::stand_in;
use super::super::menu::MenuMsg;
use super::super::places::Where;
use super::super::shares::ShareMsg;
use super::{lands, side_panel, SIDEBAR};
use crate::common::smb::Mounted;
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::widget::{button, card, column, entry, item, list, note, row, scroll, section_rule, side_heading, space, typing_over, InPlace, Item, Length, Node, Rect};

fn side_menu(pinned: bool, row: usize, x: f32, y: f32) -> Msg {
    Msg::Menu(MenuMsg::Side { pinned, row, x, y })
}

const SIDE_HEAD_GAP: f32 = 4.0;
const SIDE_GAP: f32 = 12.0;
const SIDE_BUTTONS_IN: f32 = 4.0;
const SIDE_ROOM: f32 = 8.0;

fn side_group(name: &str, body: Node<Msg>) -> Node<Msg> {
    column(vec![side_head(name), body]).gap(SIDE_HEAD_GAP)
}

fn side_head(name: &str) -> Node<Msg> {
    row(vec![space().width(Length::Fixed(theme::sheet().entry_padding)), side_heading(name.to_string())]).gap(0.0).width(Length::Fill)
}

pub(super) fn sidebar(f: &Files) -> Node<Msg> {
    let side_row = f.room_now().side_row + SIDE_ROOM;
    let at = &f.pane().at;
    let places = f.place_list();
    let (rest, far) = places[1..].split_at(2);
    let mut groups = vec![places_group(f, side_row, rest)];
    groups.push(side_group(t!("folders-view-pinned"), pinned_with_down_shares(f, side_row)));
    groups.push(devices_group(f, side_row, &places[0]));
    groups.push(network_group(f, side_row, far));
    let shares = f.share_places();
    if !shares.is_empty() {
        groups.push(open_shares_group(f, side_row, at.folder(), shares));
    }
    let last = groups.len().saturating_sub(1);
    let groups: Vec<Node<Msg>> = groups.into_iter().enumerate()
        .flat_map(|(i, g)| std::iter::once(g).chain((i < last).then(section_rule))).collect();
    card(scroll("side", column(groups).gap(SIDE_GAP).padding(theme::sheet().sidebar_room))).snug().flat()
        .width(Length::Fixed(side_panel())).height(Length::Fill)
}

fn places_group(f: &Files, side_row: f32, rest: &[Where]) -> Node<Msg> {
    let at = &f.pane().at;
    let at_home = at.folder().is_some_and(|h| h == f.home.as_path());
    let off = 1;
    let rows: Vec<Item> = rest.iter().map(|w| item(f.place_name(w))).collect();
    let chosen = rest.iter().position(|w| w == at);
    side_group(t!("folders-view-places"), column(vec![
        lands(f, list(vec![item(t!("folders-view-home"))], at_home.then_some(0), |_| Msg::Home).here().row_height(side_row), |_| Land::Home),
        lands(f, list(rows, chosen, move |i| Msg::Place(i + off)).here().type_ahead().row_height(side_row)
            .on_menu(move |row, x, y| side_menu(false, row + off, x, y)).on_middle(move |row| Msg::Middle(Land::Place(row + off))),
              move |i| Land::Place(i + off)),
    ]).gap(0.0))
}

fn pinned_with_down_shares(f: &Files, side_row: f32) -> Node<Msg> {
    let pinned = pinned_list(f, side_row);
    let down = f.down_shares();
    if down.is_empty() {
        return pinned;
    }
    let mut lines = vec![pinned];
    for &i in &down {
        lines.push(down_share_line(f, i));
    }
    if down.len() > 1 {
        lines.push(row(vec![
            space().width(Length::Fixed(SIDE_BUTTONS_IN)),
            button(stand_in::RECONNECT, t!("folders-view-reconnect-all")).on_press(Msg::Share(ShareMsg::ReconnectAll)),
            note(t!("folders-view-shares-down", n = down.len())).width(Length::Fill),
        ]).gap(8.0).width(Length::Fill).centred());
    }
    column(lines).gap(SIDE_HEAD_GAP)
}

fn down_share_line(f: &Files, i: usize) -> Node<Msg> {
    let name = f.shares.remembered[i].name();
    let mut line = vec![
        space().width(Length::Fixed(SIDE_BUTTONS_IN)),
        button(stand_in::RECONNECT, t!("folders-view-reconnect", name = &name)).on_press(Msg::Share(ShareMsg::Reconnect(i))),
    ];
    if !f.pinned.iter().any(|p| f.shares.remembered[i].holds(p)) {
        line.push(button(stand_in::FORGET_SHARE, t!("folders-view-forget-share", name = &name)).on_press(Msg::Share(ShareMsg::ForgetShare(i))));
    }
    line.push(note(t!("folders-view-share-down", name = &name)).width(Length::Fill));
    row(line).gap(8.0).width(Length::Fill).centred()
}

fn pinned_list(f: &Files, side_row: f32) -> Node<Msg> {
    if f.pinned.is_empty() {
        return note(t!("folders-view-pinned-none")).width(Length::Fill);
    }
    let here = f.pane().at.folder();
    let rows: Vec<Item> = f.pinned.iter().map(|p| {
        let row = item(f.pin_name(p));
        let row = if f.pin_down(p).is_some() { row.detail(t!("folders-view-not-connected")) } else { row };
        match f.tint_of(p) {
            Some(c) => row.mark(c),
            None => row,
        }
    }).collect();
    let chosen = here.and_then(|h| f.pinned.iter().position(|p| p == h));
    let rows = list(rows, chosen, Msg::Pinned).here().type_ahead().row_height(side_row).on_menu(|row, x, y| side_menu(true, row, x, y))
        .on_middle(|row| Msg::Middle(Land::Pinned(row))).on_reorder(Msg::PinMove).key("pinned");
    let rows = if f.drag.as_ref().is_some_and(|d| d.moving && d.pins) { rows.on_drop_between(Msg::PinAt) } else { rows };
    typing_over(lands(f, rows, Land::Pinned), pin_name_box(f, side_row))
}

fn pin_name_box(f: &Files, side_row: f32) -> Option<InPlace<Msg>> {
    f.pin_rename.as_ref().and_then(|(folder, text)| {
        let at = f.pinned.iter().position(|p| p == folder)?;
        let sheet = theme::sheet();
        let h = sheet.entry_height.min(side_row - 2.0).max(1.0);
        let name = entry(text.clone(), t!("folders-view-pin-name-hint", was = super::super::kinds::shown_name(folder)), Msg::PinNameText)
            .key("pin-name").grab(f.turn).selects(0..text.len()).on_submit(Msg::PinNameDone).on_escape(Msg::PinNameStop).on_leave(Msg::PinNameDone);
        Some(InPlace { place: Rect { x: 0.0, y: at as f32 * side_row + (side_row - h) / 2.0, w: SIDEBAR, h }, entry: name, problem: None })
    })
}

fn devices_group(f: &Files, side_row: f32, storage: &Where) -> Node<Msg> {
    let at = &f.pane().at;
    let mut devices = vec![lands(f, list(vec![item(f.place_name(storage))], (at == storage).then_some(0), |_| Msg::Place(0)).here().row_height(side_row)
        .on_menu(|_, x, y| side_menu(false, 0, x, y)).on_middle(|_| Msg::Middle(Land::Place(0))), |_| Land::Place(0))];
    for (i, r) in drives::rows(&f.drives.list, at.folder()).iter().enumerate() {
        devices.push(row(vec![
            lands(f, list(vec![item(r.name.clone())], r.chosen.then_some(0), move |_| Msg::Drive(DriveMsg::Show(i)))
                .here().row_height(side_row).width(Length::Fill).on_middle(move |_| Msg::Middle(Land::Drive(i))), move |_| Land::Drive(i)),
            button(stand_in::EJECT, r.eject.clone()).on_press(Msg::Drive(DriveMsg::Eject(i))),
        ]).gap(4.0).width(Length::Fill));
    }
    side_group(t!("folders-view-drives"), column(devices).gap(0.0))
}

fn network_group(f: &Files, side_row: f32, far: &[Where]) -> Node<Msg> {
    let at = &f.pane().at;
    let off = 3;
    let rows: Vec<Item> = far.iter().map(|w| item(match w {
        Where::Network => t!("folders-view-network-nearby").to_string(),
        _ => f.place_name(w),
    })).collect();
    let chosen = far.iter().position(|w| w == at);
    side_group(t!("folders-places-network"), column(vec![
        lands(f, list(rows, chosen, move |i| Msg::Place(i + off)).here().row_height(side_row)
            .on_menu(move |row, x, y| side_menu(false, row + off, x, y)).on_middle(move |row| Msg::Middle(Land::Place(row + off))),
              move |i| Land::Place(i + off)),
        list(vec![item(t!("folders-view-connect"))], None, |_| Msg::Far(FarMsg::Open)).here().row_height(side_row),
    ]).gap(0.0))
}

fn open_shares_group(f: &Files, side_row: f32, here: Option<&std::path::Path>, shares: &[Mounted]) -> Node<Msg> {
    let mut open = Vec::new();
    for (i, m) in shares.iter().enumerate() {
        let chosen = (here == Some(m.at.as_path())).then_some(0);
        let name = m.name();
        open.push(row(vec![
            lands(f, list(vec![item(name.clone())], chosen, move |_| Msg::Share(ShareMsg::Show(i)))
                .here().row_height(side_row).width(Length::Fill).on_middle(move |_| Msg::Middle(Land::Share(i))), move |_| Land::Share(i)),
            button(stand_in::EJECT, t!("folders-view-share-close", name = &name)).on_press(Msg::Share(ShareMsg::Eject(i))),
        ]).gap(4.0).width(Length::Fill));
    }
    side_group(t!("folders-view-shares"), column(open).gap(0.0))
}
