//! How the Files window looks: a toolbar, the sidebar, one or two panes

mod sidebar;
#[cfg(test)]
mod tests;

use sidebar::sidebar;
use super::app::{Files, Msg, Pane, View};
use super::below::BelowMsg;
use super::found::FindMsg;
use super::icons::{self, stand_in, Cellular};
use super::dialogs::{foot, head_to};
use super::places::{self, Where};
use crate::toolkit::t;
use crate::toolkit::text::Style;
use crate::toolkit::theme;
use crate::toolkit::tiles::{empty_state, error_state, loading_state};
use super::menu::MenuMsg;
use crate::toolkit::widget::{button, card, choice, column, drawing, dropdown, entry, item, label, layers, list, main_action, note, placed, row, section_rule, word_button,
                             scroll, scroll_x, space, typing_over, Area, InPlace, Item, Length, Node, Rect, Sub, Tone};

const SIDEBAR: f32 = 200.0;
pub(super) fn side_panel() -> f32 {
    SIDEBAR + 2.0 * (theme::sheet().snug_padding + theme::sheet().sidebar_room)
}
pub(super) const LINES_PER_STRIP: usize = 16;
pub(super) const FOOT_LINES: f32 = 1.0;
const BACKDROP: f32 = 0.55;
const QUESTION_W: f32 = 360.0;
pub(super) const STATUS_H: f32 = 22.0;

pub fn view(f: &Files) -> Node<Msg> {
    let mut over = super::progress::layers(f);
    if let Some((_, words)) = &f.notice {
        over.push(notice(words));
    }
    let page = match reasons_now(f).filter(|_| f.reasons_open) {
        Some(lines) => crate::toolkit::popup::dismissing(window(f), listed_panel(lines), Msg::Listed, Msg::Idle),
        None => window(f),
    };
    if over.is_empty() {
        return page;
    }
    over.insert(0, page);
    layers(over)
}

fn reasons_now(f: &Files) -> Option<&Vec<String>> {
    f.reasons.as_ref().filter(|(words, _)| *words == f.said && !f.said.is_empty()).map(|(_, lines)| lines)
}

const LISTED_LINES: usize = 12;
const LISTED_W: f32 = 420.0;

fn listed_panel(lines: &[String]) -> Node<Msg> {
    let rows: Vec<Node<Msg>> = lines.iter().map(|l| note(l.clone()).width(Length::Fill)).collect();
    let shown = rows.len().min(LISTED_LINES) as f32 * (STATUS_H + super::progress::GAP);
    let panel = card(scroll("listed files", column(rows).gap(super::progress::GAP).width(Length::Fill)).height(Length::Fixed(shown))).floats().width(Length::Fixed(LISTED_W));
    placed(panel, |(_, h), (_, ah)| (16.0, ah - h - 2.0 * STATUS_H - 16.0))
}

fn notice(words: &str) -> Node<Msg> {
    use crate::toolkit::pill::{paint_pill, BAND, FOOT, HEIGHT, ROOM};
    let said = words.to_string();
    let pill = drawing((0.0, BAND), words.to_string(), move |canvas, pen| paint_pill(canvas, pen, &said))
        .width(Length::Fill)
        .height(Length::Fixed(BAND))
        .passes();
    placed(pill, |_, (_, ah)| (0.0, ah - FOOT - ROOM - HEIGHT))
}

fn window(f: &Files) -> Node<Msg> {
    let (cards, feet) = if f.two {
        let one = |p: usize| {
            let card = card(pane(f, p)).current(f.side() == p).width(Length::Fill).height(Length::Fill);
            if f.presses_activate() { card.on_press_within(Msg::Activate(p)) } else { card }
        };
        (vec![one(0), middle(f), one(1)], vec![pane_foot(f, 0), space().width(Length::Fixed(strip())), pane_foot(f, 1)])
    } else {
        let p = f.side();
        (vec![card(pane(f, p)).width(Length::Fill).height(Length::Fill)], vec![pane_foot(f, p)])
    };
    let gap = if f.two { strip_gap() } else { 0.0 };
    let (mut cards, mut feet) = (row(cards).gap(gap).width(Length::Fill).height(Length::Fill), row(feet).gap(gap).width(Length::Fill));
    if let Some(wide) = f.panel_room().filter(|_| f.kept.panel) {
        cards = row(vec![cards, super::panel::edge(), super::panel::view(f, wide)]).gap(0.0).width(Length::Fill).height(Length::Fill);
        feet = row(vec![feet, space().width(Length::Fixed(theme::sheet().panel_gap + wide))]).gap(0.0).width(Length::Fill);
    }
    let work = column(vec![cards, feet]).gap(f.room_now().gap).width(Length::Fill).height(Length::Fill);
    let main = row(vec![sidebar(f), work])
        .width(Length::Fill)
        .height(Length::Fill)
        .gap(theme::sheet().panel_gap);
    let gap = theme::sheet().panel_gap;
    let top = card(top_bar(f)).area(Area::Top, [gap, gap, gap / 4.0, gap]).width(Length::Fill);
    let mut all = vec![top, section_rule(), main];
    all.extend(super::choose::bar(f));
    let all = column(all).padding(gap).gap(gap / 2.0);
    let question = if let Some(d) = &f.dialog {
        Some((format!("dialog:{:?}", std::mem::discriminant(d)), super::dialogs::view(d, f.turn)))
    } else if let Some(items) = &f.confirm {
        Some(("dialog:delete".to_string(), confirm(items.len(), f.emptying).on_escape(Msg::NoDelete)))
    } else {
        f.dropping.as_ref().map(|_| ("dialog:drop".to_string(), drop_places(f).on_escape(Msg::DropStop)))
    };
    match question {
        Some((name, q)) => {
            let backdrop = drawing((0.0, 0.0), "", |canvas, pen| {
                let mut paint = tiny_skia::Paint::default();
                paint.set_color(theme::alpha(pen.theme.shadow, BACKDROP).to_skia());
                if let Some(all) = tiny_skia::Rect::from_xywh(0.0, 0.0, canvas.width() as f32, canvas.height() as f32) {
                    canvas.fill_rect(all, &paint, tiny_skia::Transform::identity(), None);
                }
            }).width(Length::Fill).height(Length::Fill);
            let centred = placed(q.floats(), |(w, h), (aw, ah)| (((aw - w) / 2.0).max(0.0), ((ah - h) / 2.0).max(0.0))).stays(name);
            layers(vec![all.off(true), backdrop, centred])
        }
        None => match &f.menu {
            Some(menu) => super::menu::over(all, menu, f.room.1),
            None => all,
        },
    }
}

fn confirm(n: usize, emptying: bool) -> Node<Msg> {
    let (name, body, act) = if emptying {
        (t!("folders-view-empty-trash-title").to_string(), t!("folders-view-empty-trash-body", n = n), t!("folders-view-empty-trash"))
    } else {
        (t!("folders-view-delete-title", n = n), t!("folders-view-delete-body").to_string(), t!("folders-view-delete-permanently"))
    };
    card(column(vec![
        head_to(stand_in::SHRED, name, t!("folders-view-keep-them"), Msg::NoDelete),
        label(body).dialog_message().width(Length::Fill),
        foot(Vec::new(), main_action(stand_in::SHRED, act).cannot_be_undone().on_press(Msg::DeleteSure)),
    ]).gap(12.0).width(Length::Fill)).width(Length::Fixed(QUESTION_W))
}

fn drop_places(f: &Files) -> Node<Msg> {
    let rows: Vec<Item> = f.drop_places().into_iter().map(|(name, at)| {
        if at == Where::Trash { item(name).icon("user-trash-symbolic") } else { item(name).icon("folder-symbolic").folder() }
    }).collect();
    let n = rows.len();
    card(column(vec![
        head_to("folder-symbolic", t!("folders-view-drop-title"), t!("folders-view-drop-leave"), Msg::DropStop),
        super::dialogs::dialog_list("drop-places", list(rows, None, Msg::DropOn).grab(f.turn), n),
    ]).gap(12.0).width(Length::Fill)).width(Length::Fixed(QUESTION_W))
}

fn top_bar(f: &Files) -> Node<Msg> {
    let p = f.side();
    let pane = &*f.cols[p];
    let mut lines = Vec::new();
    if !f.two && f.cols[p].tabs.len() > 1 {
        lines.push(tab_strip(f, p));
    }
    lines.push(toolbar(f, pane, p));
    if f.find.open == Some(p) {
        lines.push(narrowing(pane, p));
    }
    column(lines).gap(f.room_now().gap).width(Length::Fill)
}

fn toolbar(f: &Files, pane: &Pane, p: usize) -> Node<Msg> {
    let panes_tip = if f.two { t!("folders-view-panes-one") } else { t!("folders-view-panes-two") };
    let new = word_button(t!("folders-view-new")).on_press(Msg::Menu(MenuMsg::New))
        .off(!super::inplace::can_rename(&pane.at) || f.earlier_at(&pane.at));
    let sheet = theme::sheet();
    let room = f.room.0 - 2.0 * (sheet.panel_gap + sheet.card_padding) - 9.0 * (theme::icon_button() + BAR_GAP) - pill_guess(t!("folders-view-new")) - BAR_GAP;
    let searching = f.find.open == Some(p);
    let nothing = pane.at == Where::Computer;
    let search_tip = if nothing { t!("folders-view-search-none") } else if searching { t!("folders-view-search-close") } else { t!("folders-view-search") };
    let fits = f.panel_room().is_some();
    let panel_tip = if !fits { super::panel::too_narrow() } else if f.kept.panel { t!("folders-view-panel-hide") } else { t!("folders-view-panel-show") };
    let (hidden_icon, hidden_tip) = if f.hidden { ("view-reveal-symbolic", t!("folders-view-hidden-hide")) } else { ("view-conceal-symbolic", t!("folders-view-hidden-show")) };
    let view_icon = match f.view_of(p) { View::Icons => "view-grid-symbolic", View::List => "view-list-symbolic", View::Compact => "view-continuous-symbolic" };
    row(vec![
        button("go-previous-symbolic", t!("folders-view-back")).on_press(Msg::Back).off(pane.back.is_empty()),
        button("go-next-symbolic", t!("folders-view-forward")).on_press(Msg::Forward).off(pane.ahead.is_empty()),
        button("go-up-symbolic", t!("folders-view-up")).on_press(Msg::Up).off(pane.at.up().is_none()),
        button("system-search-symbolic", search_tip).on(searching).spoken(t!("folders-view-search-spoken")).on_press(Msg::Find(FindMsg::Open(Some(p)))).off(nothing),
        bar_middle(f, pane, p, room),
        new,
        button(stand_in::TWO_PANES, panes_tip).on(f.two).spoken(t!("folders-view-two-panes")).on_press(Msg::PanesAre(!f.two)),
        button(stand_in::PREVIEW_PANEL, panel_tip).on(f.kept.panel && fits).spoken(t!("folders-view-panel-spoken"))
            .on_press(Msg::Panel(super::panel::PanelMsg::Switch)).off(!fits),
        button(hidden_icon, hidden_tip).on(f.hidden).spoken(t!("folders-view-hidden-spoken")).on_press(Msg::Hidden),
        button(view_icon, t!("folders-view-view")).on_press(Msg::Menu(MenuMsg::View)),
        button("view-more-symbolic", t!("folders-view-more")).on_press(Msg::Menu(MenuMsg::More)),
    ]).gap(BAR_GAP).width(Length::Fill)
}

pub(super) fn top_bar_height(f: &Files) -> f32 {
    let sheet = theme::sheet();
    let tabs = if !f.two && f.cols[f.side()].tabs.len() > 1 { theme::icon_button() + f.room_now().gap } else { 0.0 };
    sheet.panel_gap + theme::icon_button() + 2.0 * sheet.card_padding + tabs
}

fn lands(f: &Files, node: Node<Msg>, to: impl Fn(usize) -> super::act::Land + 'static) -> Node<Msg> {
    let to: std::rc::Rc<dyn Fn(usize) -> super::act::Land> = std::rc::Rc::new(to);
    let node = super::outside::onto_rows(f, node, to.clone());
    if f.drag.as_ref().is_some_and(|d| d.right) { node.on_drop(move |i| Msg::Landed(to(i))) } else { node }
}

fn tab_strip(f: &Files, p: usize) -> Node<Msg> {
    let col = &f.cols[p];
    let most = if f.two { 14 } else { 24 };
    let names: Vec<String> = col.tabs.iter().map(|t| short_name(&f.tab_name(&t.at), most)).collect();
    let which = if !f.two { t!("folders-view-tabs") } else if p == 0 { t!("folders-view-tabs-left") } else { t!("folders-view-tabs-right") };
    row(vec![
        lands(f, choice(names, col.tab, which, move |i| Msg::Tab(p, i)).on_middle(move |i| Msg::Middle(super::act::Land::Tab(p, i))),
              move |i| super::act::Land::Tab(p, i)),
        space().width(Length::Fill),
        button("tab-new-symbolic", t!("folders-view-tab-new")).on_press(Msg::NewTab(Some(p))),
        button("window-close-symbolic", t!("folders-view-tab-close")).on_press(Msg::CloseTab(Some(p))).off(col.tabs.len() < 2),
    ]).gap(8.0).width(Length::Fill)
}

fn short_name(name: &str, most: usize) -> String {
    if name.chars().count() <= most {
        return name.to_string();
    }
    let mut out: String = name.chars().take(most - 1).collect();
    out.push('\u{2026}');
    out
}

pub(super) fn strip() -> f32 {
    theme::sheet().pane_strip.max(theme::icon_button() + 2.0 * STRIP_ROOM)
}
const STRIP_ROOM: f32 = 4.0;
pub(super) fn strip_gap() -> f32 {
    theme::sheet().panel_gap / 2.0
}
const STRIP_GROUPS: f32 = 12.0;

fn middle(f: &Files) -> Node<Msg> {
    let (arrow, way) = if f.side() == 0 { ("go-next-symbolic", "right") } else { ("go-previous-symbolic", "left") };
    let same = f.cols[0].at == f.cols[1].at;
    let folders = f.cols.iter().all(|q| q.at.folder().is_some());
    let apart = || space().width(Length::Shrink).height(Length::Fixed(STRIP_GROUPS));
    let link = match &f.link_note {
        Some(note) if f.kept.linked => t!("folders-view-link-noted", note = note),
        _ => t!("folders-view-link").to_string(),
    };
    let buttons = column(vec![
        space().width(Length::Shrink).height(Length::Fill),
        button("edit-copy-symbolic", t!("folders-view-copy-to", way = way)).on_press(Msg::Copy),
        button(arrow, t!("folders-view-move-to", way = way)).on_press(Msg::Move),
        apart(),
        button(stand_in::SAME_FOLDER, t!("folders-view-same-in", way = way)).on_press(Msg::SameInOther).off(same),
        button(stand_in::SWAP_PANES, t!("folders-view-swap")).on_press(Msg::SwapPanes),
        button(stand_in::LINK_PANES, link).on(f.kept.linked).on_press(Msg::Link),
        apart(),
        button(stand_in::PICK_DIFFERING, t!("folders-view-pick-differing", way = way)).on_press(Msg::PickDiffering).off(same),
        button(stand_in::COMPARE, t!("folders-view-compare", way = way)).on_press(Msg::ComparePanes).off(!folders),
        space().width(Length::Shrink).height(Length::Fill),
    ])
    .gap(8.0)
    .centred()
    .width(Length::Fill)
    .height(Length::Fill);
    buttons.key(BETWEEN).width(Length::Fixed(strip())).height(Length::Fill)
}

pub(super) const BETWEEN: &str = "between the panes";

pub(super) fn shown_widths(f: &Files, pane: &Pane, p: usize) -> Vec<(super::columns::Col, f32)> {
    match f.view_of(p) {
        View::List => {
            let rich = f.widths_rich();
            let cols: Vec<_> = f.kept.columns().into_iter().map(|c| (c, f.kept.width(c, rich))).collect();
            super::columns::fitting(&cols, pane.width.get())
        }
        View::Icons | View::Compact => Vec::new(),
    }
}

pub(super) fn shown_columns(f: &Files, pane: &Pane, p: usize) -> Vec<super::columns::Col> {
    shown_widths(f, pane, p).into_iter().map(|c| c.0).collect()
}

fn pill_guess(name: &str) -> f32 {
    name.chars().count() as f32 * 7.6 + 2.0 * theme::sheet().entry_padding + ARROW_ROOM
}

const ARROW_ROOM: f32 = 16.0;

fn fit_crumbs(crumbs: Vec<String>, room: f32) -> (Vec<String>, usize) {
    let total: f32 = crumbs.iter().map(|c| pill_guess(c)).sum();
    if total <= room || crumbs.len() < 2 {
        return (crumbs, 0);
    }
    let dots = pill_guess("\u{2026}");
    let mut used = dots;
    let mut keep = 0;
    for c in crumbs.iter().rev() {
        let w = pill_guess(c);
        if keep > 0 && used + w > room {
            break;
        }
        used += w;
        keep += 1;
    }
    let skipped = crumbs.len() - keep;
    if skipped == 0 {
        return (crumbs, 0);
    }
    if keep == 1 && used > room {
        let last = crumbs.into_iter().next_back().into_iter().collect();
        return (last, skipped);
    }
    let mut out = vec!["\u{2026}".to_string()];
    out.extend(crumbs.into_iter().skip(skipped));
    (out, skipped)
}

const BAR_GAP: f32 = 8.0;

fn path(f: &Files, pane: &Pane, p: usize, room: f32) -> Node<Msg> {
    let parts = places::crumbs(&pane.at, &f.home);
    let crumbs: Vec<String> = parts.iter().map(|(name, _)| name.clone()).collect();
    let (crumbs, skipped) = fit_crumbs(crumbs, room);
    let (last, crumbs_len) = (crumbs.len().saturating_sub(1), crumbs.len());
    let dots = usize::from(skipped > 0 && crumbs.first().is_some_and(|c| c == "\u{2026}"));
    let part_of = move |i: usize| (i + skipped).saturating_sub(dots);
    let below = super::below::subs(f, &parts);
    let subs: Vec<Sub> = (0..crumbs.len()).map(|i| if i < dots { Sub::None } else { below.get(part_of(i)).cloned().unwrap_or(Sub::None) }).collect();
    let places_of: Vec<Where> = (0..crumbs.len()).map(|i| parts.get(part_of(i)).map_or_else(|| pane.at.clone(), |c| c.1.clone())).collect();
    let picked_of = places_of.clone();
    let place = choice(crumbs, last, t!("folders-view-where"), move |i| Msg::Part(p, part_of(i)))
        .on_middle(move |i| Msg::PartTab(p, part_of(i)))
        .arrows(subs, move |i| Msg::Below(BelowMsg::Open(p, places_of[i].clone())), move |i, row| Msg::Below(BelowMsg::Pick(p, picked_of[i].clone(), row)))
        .key(format!("path{p}"));
    let kinds: Vec<Option<Where>> = (0..crumbs_len).map(|i| if i < dots { None } else { f.pill_place(p, part_of(i)) }).collect();
    let ok = std::rc::Rc::new(move |i: usize| kinds.get(i).and_then(Option::as_ref).is_some_and(super::act::open_to_write));
    let to: std::rc::Rc<dyn Fn(usize) -> super::act::Land> = std::rc::Rc::new(move |i| super::act::Land::Part(p, i.saturating_add(skipped).saturating_sub(dots)));
    let place = super::outside::onto_names(place, { let ok = ok.clone(); move |i| ok(i) }, to.clone());
    let place = if f.drag.as_ref().is_some_and(|d| d.moving) { place.on_drop_where(move |i| ok(i), move |i| Msg::Landed(to(i))) } else { place };
    row(vec![place, space().width(Length::Fill)]).width(Length::Fill)
}

fn pane(f: &Files, p: usize) -> Node<Msg> {
    let pane = &*f.cols[p];
    let active = f.side == p;
    let body = if pane.at == Where::Computer {
        super::computer::page(f, p)
    } else if pane.entries.is_empty() {
        let words = vec![space().width(Length::Shrink).height(Length::Fixed(EMPTY_TOP)), empty_pane(f, pane, active),
                         space().width(Length::Shrink).height(Length::Fixed(EMPTY_UNDER))];
        items(f, pane, p, words)
    } else {
        items(f, pane, p, Vec::new())
    };
    let mut controls = Vec::new();
    if f.two && f.cols[p].tabs.len() > 1 {
        controls.push(tab_strip(f, p));
    }
    controls.push(pane_line(f, pane, p));
    if let Some(words) = f.earlier_note(&pane.at) {
        controls.push(note(words).cut().width(Length::Fill));
    }
    let mut parts = controls;
    if !parts.is_empty() {
        parts.push(section_rule());
    }
    if let Some(head) = f.head_of(pane, p).filter(|_| !pane.entries.is_empty()) {
        let h = grid_of(f, pane, p).h;
        let headings = pane.groups.len() as f32 * icons::HEAD_H;
        let scrolls = pane.tall.get() > 0.0 && pane.entries.len() as f32 * h + headings + FOOT_LINES * h > pane.tall.get() + 0.5;
        let sheet = theme::sheet();
        let gutter = if scrolls { sheet.scrollbar_hover + 2.0 * sheet.scrollbar_inset } else { 0.0 };
        parts.push(super::heading::view(head, gutter, move |at| Msg::Head(p, at)));
    }
    parts.push(body);
    column(parts).gap(f.room_now().gap).width(Length::Fill).height(Length::Fill)
}

fn narrowing(pane: &Pane, p: usize) -> Node<Msg> {
    use super::narrow::{Changed, Kind, Size};
    let n = pane.narrow;
    let words = |all: &[&'static str]| all.iter().map(|w| w.to_string()).collect::<Vec<_>>();
    row(vec![
        dropdown(words(&Kind::ALL.map(Kind::word)), Kind::ALL.iter().position(|k| *k == n.kind), t!("folders-view-find-kind"), |i| Msg::Find(FindMsg::Kind(i)))
            .key(format!("find-kind{p}")).width(Length::Fill),
        dropdown(words(&Size::ALL.map(Size::word)), Size::ALL.iter().position(|s| *s == n.size), t!("folders-view-find-size"), |i| Msg::Find(FindMsg::Size(i)))
            .key(format!("find-size{p}")).width(Length::Fill),
        dropdown(words(&Changed::ALL.map(Changed::word)), Changed::ALL.iter().position(|c| *c == n.changed), t!("folders-view-find-changed"), |i| Msg::Find(FindMsg::Changed(i)))
            .key(format!("find-changed{p}")).width(Length::Fill),
    ]).gap(8.0).width(Length::Fill)
}

fn bar_middle(f: &Files, pane: &Pane, p: usize, room: f32) -> Node<Msg> {
    if f.find.open == Some(p) {
        let deep = f.searchable(p);
        return row(vec![
            entry(pane.filter.clone(), t!("folders-view-filter-hint"), |t| Msg::Find(FindMsg::Text(t)))
                .key("find").grab(f.turn).on_submit(Msg::Find(FindMsg::Search(false))).on_escape(Msg::Find(FindMsg::Close))
                .width(Length::Fill),
            button("system-search-symbolic", t!("folders-view-find-name")).on_press(Msg::Find(FindMsg::Search(false))).off(!deep),
            button(stand_in::SEARCH_INSIDE, t!("folders-view-find-inside")).on_press(Msg::Find(FindMsg::Search(true))).off(!deep),
        ]).gap(BAR_GAP).width(Length::Fill);
    }
    match f.locate.as_ref().filter(|(at, _)| *at == p && !f.locate_line) {
        Some((_, text)) => entry(text.clone(), t!("folders-view-locate-hint"), Msg::LocateText)
            .key("locate").grab(f.turn).on_submit(Msg::LocateDone).on_escape(Msg::LocateStop).width(Length::Fill),
        None => path(f, pane, p, room),
    }
}

fn pane_line(f: &Files, pane: &Pane, p: usize) -> Node<Msg> {
    match f.locate.as_ref().filter(|(at, _)| *at == p && f.locate_line) {
        Some((_, text)) => entry(text.clone(), t!("folders-view-locate-hint"), Msg::LocateText)
            .key(format!("locate-line{p}")).grab(f.turn).on_submit(Msg::LocateDone).on_escape(Msg::LocateStop).on_leave(Msg::LocateStop).width(Length::Fill),
        None => card(typed_line(location_words(f, &pane.at))).flat().snug().on_press(Msg::LocateLine(p)).width(Length::Fill),
    }
}

fn typed_line(words: String) -> Node<Msg> {
    line_drawn(words, true, false, None).named(true)
}

fn location_words(f: &Files, at: &Where) -> String {
    match at {
        Where::Folder(path) => path.to_string_lossy().into_owned(),
        Where::Remote(address) => address.clone(),
        _ => f.tab_name(at),
    }
}

pub(super) const ROW_ROOM: f32 = 10.0;

pub(super) fn grid_of(f: &Files, pane: &Pane, p: usize) -> icons::Grid {
    let room = f.room_now();
    match f.view_of(p) {
        View::Icons => icons::Grid::icons_in(pane.width.get(), f.kept.icon(), room.icon_room),
        View::List => icons::Grid::lines(room.row + ROW_ROOM),
        View::Compact if !pane.entries.is_empty() => {
            let names: Vec<f32> = pane.entries.iter().map(|e| icons::name_width(f.shown_name(e))).collect();
            let grid = icons::Grid::columns(room.line + ROW_ROOM, pane.tall.get(), &names, &pane.groups, f.ticks);
            if grid.breadth() <= pane.width.get() {
                return grid;
            }
            let bar = theme::sheet().scrollbar_hover + 2.0 * theme::sheet().scrollbar_inset;
            return icons::Grid::columns(room.line + ROW_ROOM, pane.tall.get() - bar, &names, &pane.groups, f.ticks);
        }
        View::Compact => icons::Grid::lines(room.line + ROW_ROOM),
    }.ticked(f.ticks).grouped(&pane.groups)
}

fn items(f: &Files, pane: &Pane, p: usize, above: Vec<Node<Msg>>) -> Node<Msg> {
    let grid = grid_of(f, pane, p);
    pane.grid.set(grid.clone());
    if grid.across() && above.is_empty() {
        return across(f, pane, p, grid);
    }
    if !grid.lines {
        let in_sight = grid.cols * ((pane.tall.get() / grid.h).ceil() as usize + 1);
        pane.seen.keep(super::pictures::window_for(grid.icon, in_sight));
    }
    let n = pane.entries.len();
    let band = pane.band.as_ref().map(|b| icons::band_rect(b.from, b.to));
    let per = if grid.lines { LINES_PER_STRIP } else { 1 };
    let take = grid.cols * per;
    let widths = shown_widths(f, pane, p);
    let shown: Vec<super::columns::Col> = widths.iter().map(|c| c.0).collect();
    let noted = icons::Noted { name: pane.name_at.clone(), tall: f.lines_tall.clone() };
    let renaming = f.renaming.as_ref().filter(|r| r.side == p && r.tab == f.cols[p].tab)
        .and_then(|r| Some((pane.entries.iter().position(|e| e.path == r.path)?, r)));
    let mut strips = Vec::new();
    let in_sight = grid.cols * ((pane.tall.get() / grid.h).ceil() as usize + 1);
    let also: Vec<usize> = pane.chosen.into_iter().chain(pane.hover).chain(renaming.map(|(at, _)| at)).collect();
    let made = pane.seen.made(n, in_sight.max(take), &also);
    let picked = super::app::Picked::of(&pane.picked);
    for (run_start, run_len, run_row) in grid.runs(n) {
        for within in (0..run_len).step_by(take) {
            let start = run_start + within;
            let first = run_row + within / grid.cols;
            let end = (start + take).min(run_start + run_len);
            let near = made.iter().any(|(a, b)| start < *b && end > *a);
            let cells: Vec<Cellular> = (start..end).filter(|_| near)
                .map(|i| cellular(f, pane, i, &picked, renaming.map(|(at, _)| at), &grid, &shown, &widths)).collect();
            let rows = (end - start).div_ceil(grid.cols);
            let tip = pane.entries[start..end].iter().map(|e| f.shown_name(e)).collect::<Vec<_>>().join(", ");
            let top = grid.strip_top(first);
            let tall = grid.row_top(first) - top + rows as f32 * grid.h;
            let strip = icons::strip(first, cells, grid.clone(), tall, band, pane.width.clone(), noted.clone(), tip,
                                     move |at| Msg::Icon(p, first, at), seen_at(f, pane, start, end));
            let strip = super::outside::onto_icons(f, strip, p, first, 0.0);
            strips.push(match pane.chosen.filter(|c| pane.seek > 0 && (start..end).contains(c)) {
                Some(c) => strip.seek(pane.seek, grid.cell(c, pane.width.get()).y - top, grid.h),
                None => strip,
            });
        }
    }
    let empty = !above.is_empty();
    let last = foot_strip(f, pane, p, &grid, band, &noted, empty);
    if empty {
        strips.splice(0..0, above);
        strips.push(last.height(Length::Fill));
    } else {
        strips.push(last);
    }
    nudge_first(pane, &mut strips);
    let typing = renaming.map(|(i, r)| typing_box(f, p, &grid, i, r, &widths, &noted, 0.0));
    let area = if empty { empty_area(p, f.cols[p].tab, &pane.at) } else { area_name(f, pane, p) };
    layers(vec![sizer(f, pane, false), scroll(area, typing_over(column(strips).gap(0.0).height(Length::Fill), typing)).height(Length::Fill).width(Length::Fill)])
}

fn nudge_first(pane: &Pane, strips: &mut [Node<Msg>]) {
    if pane.nudge.0 > 0 {
        if let Some(first) = strips.first_mut() {
            *first = std::mem::replace(first, space()).nudge(pane.nudge.0, pane.nudge.1);
        }
    }
}

fn foot_strip(f: &Files, pane: &Pane, p: usize, grid: &icons::Grid, band: Option<Rect>, noted: &icons::Noted, empty: bool) -> Node<Msg> {
    let n = pane.entries.len();
    let foot = grid.rows(n);
    let least = FOOT_LINES * grid.h;
    let room = if empty { least } else { (pane.tall.get() - grid.row_top(foot) - 1.0).max(least).max(pane.floor.take() - grid.row_top(foot)) };
    super::outside::onto_icons(f, icons::strip(foot, Vec::new(), grid.clone(), room, band, pane.width.clone(), noted.clone(), String::new(),
                             move |at| Msg::Icon(p, foot, at), seen_at(f, pane, n, n)), p, foot, 0.0)
}

fn area_name(f: &Files, pane: &Pane, p: usize) -> String {
    format!("pane{p}:{}:{:?}:{:?}", f.cols[p].tab, f.view_of(p), pane.at)
}

#[allow(clippy::too_many_arguments)]
fn cellular(f: &Files, pane: &Pane, i: usize, picked: &super::app::Picked, renaming: Option<usize>, grid: &icons::Grid,
            shown: &[super::columns::Col], widths: &[(super::columns::Col, f32)]) -> Cellular {
    let e = &pane.entries[i];
    let cols = f.cache.cells(e, shown).into_iter().zip(widths.iter().map(|c| c.1)).collect();
    Cellular { name: f.shown_name(e).to_string(), icon: e.icon(), folder: e.folder_icon(), picked: picked.has(i),
               chosen: pane.chosen == Some(i), hover: pane.hover == Some(i), dim: f.is_cut(&e.path) || f.is_busy(&e.path) || e.arriving(),
               tint: f.tint_of(&e.path), cols, emblems: marks(f, &e.path),
               picture: if grid.lines { None } else { f.picture_of(&e.path) },
               editing: renaming == Some(i),
               tick: f.ticks && (f.touch.get() || picked.has(i) || pane.hover == Some(i)) }
}

#[allow(clippy::too_many_arguments)]
fn typing_box(f: &Files, p: usize, grid: &icons::Grid, i: usize, r: &super::inplace::Renaming, widths: &[(super::columns::Col, f32)],
              noted: &icons::Noted, by: f32) -> InPlace<Msg> {
    let cols: f32 = widths.iter().map(|c| c.1).sum();
    let mut cell = grid.cell(i, f.cols[p].width.get());
    cell.x -= by;
    let room = icons::name_room(grid, cell, cols, noted.line(), 1.0);
    let name = entry(r.text.clone(), t!("folders-view-rename-hint", was = r.was()), Msg::RenameText)
        .key(format!("rename{p}:{}", r.path.display())).grab(f.turn).selects(r.selected())
        .on_submit(Msg::RenameDone).on_escape(Msg::RenameStop).on_leave(Msg::RenameKeep).on_tab(Msg::RenameNext);
    InPlace { place: room.typing_box(grid.lines), entry: if grid.lines { name } else { name.centred() }, problem: r.why.clone() }
}

fn across(f: &Files, pane: &Pane, p: usize, grid: icons::Grid) -> Node<Msg> {
    let n = pane.entries.len();
    let runs = grid.runs_across();
    let narrowest = runs.iter().map(|r| r.3).fold(f32::MAX, f32::min).max(1.0);
    let screen = grid.rows(n) * ((pane.width.get() / narrowest).ceil() as usize + 1);
    let renaming = f.renaming.as_ref().filter(|r| r.side == p && r.tab == f.cols[p].tab)
        .and_then(|r| Some((pane.entries.iter().position(|e| e.path == r.path)?, r)));
    let also: Vec<usize> = pane.chosen.into_iter().chain(pane.hover).chain(renaming.map(|(at, _)| at)).collect();
    let made = pane.seen.made(n, screen, &also);
    let picked = super::app::Picked::of(&pane.picked);
    let band = pane.band.as_ref().map(|b| icons::band_rect(b.from, b.to));
    let noted = icons::Noted { name: pane.name_at.clone(), tall: f.lines_tall.clone() };
    let mut cols: Vec<Node<Msg>> = runs.into_iter().map(|(start, len, x, w)| {
        let end = start + len;
        let near = made.iter().any(|(a, b)| start < *b && end > *a);
        let cells = (start..end).filter(|_| near)
            .map(|i| (i, cellular(f, pane, i, &picked, renaming.map(|(at, _)| at), &grid, &[], &[]))).collect();
        let tip = pane.entries[start..end].iter().map(|e| f.shown_name(e)).collect::<Vec<_>>().join(", ");
        let col = icons::column(cells, grid.clone(), (x, w), band, pane.width.clone(), noted.clone(), tip,
                                move |at| Msg::Icon(p, 0, icons::along(at, x)), seen_at(f, pane, start, end));
        let col = match pane.chosen.filter(|c| pane.seek > 0 && (start..end).contains(c)) {
            Some(_) => col.seek(pane.seek, 0.0, w),
            None => col,
        };
        super::outside::onto_icons(f, col, p, 0, x)
    }).collect();
    let breadth = grid.breadth();
    let rest = if breadth <= pane.width.get() { (pane.width.get() - breadth - 1.0).max(0.0) } else { FOOT_LINES * grid.h };
    let last = icons::column(Vec::new(), grid.clone(), (breadth, rest), band, pane.width.clone(), noted.clone(), String::new(),
                             move |at| Msg::Icon(p, 0, icons::along(at, breadth)), seen_at(f, pane, n, n));
    cols.push(super::outside::onto_icons(f, last, p, 0, breadth));
    let typing = renaming.map(|(i, r)| typing_box(f, p, &grid, i, r, &[], &noted, 0.0));
    let items = typing_over(row(cols).gap(0.0).width(Length::Fixed(breadth + rest)).height(Length::Fill), typing);
    layers(vec![sizer(f, pane, true), scroll_x(area_name(f, pane, p), items).height(Length::Fill).width(Length::Fill)])
}

fn sizer(f: &Files, pane: &Pane, wide: bool) -> Node<Msg> {
    let (tall, width) = (pane.tall.clone(), pane.width.clone());
    let post = f.post.clone();
    let seen = pane.seen.clone();
    drawing((0.0, 0.0), "", move |canvas, pen| {
        seen.start();
        if wide {
            width.set(canvas.width() as f32 / pen.scale);
        }
        let now = canvas.height() as f32 / pen.scale;
        if (now - tall.get()).abs() > 0.5 {
            tall.set(now);
            if let Some(post) = &post {
                post.send(Msg::Resized);
            }
        }
    })
        .width(Length::Fill).height(Length::Fill)
}

fn marks(f: &Files, path: &std::path::Path) -> Vec<String> {
    let mut out = Vec::new();
    if f.is_shared(path) {
        out.push(stand_in::SHARED_MARK.to_string());
    }
    out
}

fn seen_at(f: &Files, pane: &Pane, start: usize, end: usize) -> impl Fn() + 'static {
    let (seen, post) = (pane.seen.clone(), f.post.clone());
    move || {
        let moved = seen.painted(start);
        let unmade = end > start && seen.strip_painted(start, end);
        if moved || unmade {
            if let Some(post) = &post {
                post.send(Msg::Resized);
            }
        }
    }
}

pub(super) fn free_words(pane: &Pane) -> Option<String> {
    pane.free.filter(|_| pane.at != Where::Computer).map(|free| t!("folders-view-status-free", size = size_words(free)))
}

pub(super) fn status_words(f: &Files, pane: &Pane, p: usize, active: bool) -> String {
    let (count, size, short) = f.picked_total(p);
    let mut words = if pane.at == Where::Computer {
        f.computer_words()
    } else if count == 0 {
        items_words(pane.entries.len() as u64)
    } else if short {
        t!("folders-view-status-selected-working", count = count)
    } else {
        t!("folders-view-status-selected", count = count, size = size_words(size))
    };
    if let Some(w) = super::found::words(pane, count) {
        words = if count == 0 { w } else { t!("folders-view-status-then", first = words, then = w) };
    }
    if let Some(w) = Files::not_answering(pane) {
        words = t!("folders-view-status-then", first = words, then = w);
    }
    if active && says(f) {
        words = t!("folders-view-status-then", first = words, then = &f.said);
    }
    words
}

fn says(f: &Files) -> bool {
    !f.said.is_empty() && !f.ended.iter().any(|e| e.words == f.said)
}

pub(super) fn pane_foot(f: &Files, p: usize) -> Node<Msg> {
    status(f, &f.cols[p], p, !f.two || f.side() == p)
}

fn status(f: &Files, pane: &Pane, p: usize, active: bool) -> Node<Msg> {
    let words = status_words(f, pane, p, active);
    let tone = if active && f.bad && says(f) { Tone::Problem } else if active { Tone::Plain } else { Tone::Calm };
    let out = theme::sheet().card_padding / 2.0;
    let line = one_line(words, tone);
    let line = if active && says(f) && reasons_now(f).is_some() {
        row(vec![line, button("dialog-information-symbolic", t!("folders-view-listed")).size(theme::sheet().info).on(f.reasons_open).on_press(Msg::Listed)])
            .gap(8.0).width(Length::Fill)
    } else {
        line
    };
    let more = match f.said_help.filter(|_| active && f.bad && says(f)) {
        Some(address) => crate::common::more::button(Msg::Help(address)).size(STATUS_H),
        None => crate::toolkit::widget::space().width(Length::Fixed(STATUS_H)),
    };
    let mut line = vec![space().width(Length::Fixed(out)), line, more];
    if let Some(free) = free_words(pane) {
        line.push(end_line(free));
    }
    line.push(space().width(Length::Fixed(out)));
    let line = row(line).gap(8.0).width(Length::Fill);
    column(super::progress::rows(f, p, line)).gap(super::progress::GAP).width(Length::Fill)
}

fn empty_pane(f: &Files, pane: &Pane, active: bool) -> Node<Msg> {
    let reading = match &pane.at {
        Where::Remote(a) => f.far.waiting.contains(a),
        Where::Inside(a, _) => f.inside.waiting.contains(a),
        Where::Found(..) => super::found::looking(pane),
        _ => pane.reading,
    };
    let retry = || button("view-refresh-symbolic", t!("folders-view-try-again")).on_press(Msg::Reload);
    if let Some(words) = Files::not_answering(pane) {
        let cancel = button("process-stop-symbolic", t!("folders-view-cancel")).on_press(Msg::GiveUp);
        return row(vec![note(words).width(Length::Fill), cancel, retry(), crate::common::more::button(Msg::Help(super::help::PLACE_UNREAD))]).gap(12.0).centred();
    }
    if reading {
        return loading_state(EMPTY_ROOM);
    }
    if let Some(why) = &pane.error {
        return error_state(why.clone(), row(vec![retry(), crate::common::more::button(Msg::Help(super::help::PLACE_UNREAD))]).gap(theme::sheet().gap));
    }
    if active && f.bad && !f.said.is_empty() && matches!(pane.at, Where::Remote(_) | Where::Inside(..)) {
        let next = match f.said_help {
            Some(address) => row(vec![retry(), crate::common::more::button(Msg::Help(address))]).gap(theme::sheet().gap),
            None => retry(),
        };
        return error_state(f.said.clone(), next);
    }
    let network = f.near.words();
    let (words, next) = if !pane.all.is_empty() {
        (t!("folders-view-none-matches"), vec![("edit-clear-symbolic", t!("folders-view-show-everything"), Msg::Find(FindMsg::Close))])
    } else {
        match &pane.at {
            Where::Recent => (t!("folders-view-none-recent"), vec![("go-home-symbolic", t!("folders-view-go-home"), Msg::Home)]),
            Where::Trash => (t!("folders-view-none-trash"), vec![]),
            Where::Computer => (t!("folders-view-none-drives"), vec![]),
            Where::Network => (network.as_str(), vec![(stand_in::SERVER, t!("folders-view-connect"), Msg::Far(super::far::FarMsg::Open))]),
            Where::Found(..) => (t!("folders-view-none-found"), vec![("edit-clear-symbolic", t!("folders-view-clear-search"), Msg::Find(FindMsg::Close))]),
            Where::Inside(..) => (t!("folders-view-none-archive"), vec![("go-up-symbolic", t!("folders-view-go-up"), Msg::Up)]),
            Where::Folder(_) | Where::Remote(_) => (t!("folders-view-none-folder"), vec![("folder-new-symbolic", t!("folders-view-new-folder"), Msg::NewFolder),
                                                                                  ("document-new-symbolic", t!("folders-view-new-document"), Msg::NewDocument)]),
        }
    };
    let next: Vec<Node<Msg>> = next.into_iter().map(|(icon, words, msg)| main_action(icon, words).on_press(msg)).collect();
    let next = (!next.is_empty()).then(|| column(next).gap(theme::sheet().gap));
    let wide = EMPTY_WORDS.min((pane.width.get() - 2.0 * EMPTY_SIDES).max(EMPTY_LEAST));
    row(vec![space(), empty_state(words, next).centred().width(Length::Fixed(wide)), space()]).width(Length::Fill)
}

pub(super) fn empty_area(p: usize, tab: usize, at: &Where) -> String {
    format!("empty{p}:{tab}:{at:?}")
}

const EMPTY_ROOM: f32 = 96.0;
const EMPTY_TOP: f32 = 48.0;
const EMPTY_UNDER: f32 = 8.0;
const EMPTY_WORDS: f32 = 420.0;
const EMPTY_SIDES: f32 = 12.0;
const EMPTY_LEAST: f32 = 120.0;

fn end_line(words: String) -> Node<Msg> {
    line_drawn(words, false, true, None)
}

fn one_line(words: String, tone: Tone) -> Node<Msg> {
    line_drawn(words, false, false, Some(tone))
}

fn line_drawn(words: String, fixed: bool, at_end: bool, tone: Option<Tone>) -> Node<Msg> {
    let (tip, copied) = (words.clone(), words.clone());
    drawing((0.0, STATUS_H), tip, move |canvas, pen| {
        let size = (icons::line_size() * pen.scale).round();
        let style = if fixed { Style::monospace(size) } else { Style::new(size) };
        let dots = pen.fonts.layout("\u{2026}", &style, None);
        let laid = pen.fonts.layout(&words, &style, None).cut(canvas.width() as f32, &dots);
        let y = ((canvas.height() as f32 - laid.height) / 2.0).round();
        let x = if at_end { (canvas.width() as f32 - laid.width).max(0.0).round() } else { 0.0 };
        let ink = tone.map_or(pen.theme.muted, |t| crate::toolkit::ui::toned(pen.theme, t, pen.theme.fg));
        pen.fonts.draw(canvas, &laid, x, y, ink);
    }).copies(copied).width(Length::Fill)
}

pub fn items_words(n: u64) -> String {
    t!("folders-view-items", n = n)
}

pub fn size_words(bytes: u64) -> String {
    trindle_toolkit::lang::format::size_exact(bytes)
}
