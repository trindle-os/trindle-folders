// The sign-in card.

use crate::files::app::Msg;
use crate::toolkit::t;
use crate::toolkit::tiles::standard_ring;
use crate::toolkit::widget::{card, column, entry, label, main_action, note, row, switch, Length, Node, Tone};
use super::rule::where_words;
use super::types::{Ask, ShareMsg, share_msg};

const NARROW: f32 = 440.0;

pub(super) fn remember() -> &'static str {
    t!("folders-shares-remember")
}
fn remember_line() -> &'static str {
    t!("folders-shares-remember-line")
}

fn why_row(a: &Ask) -> Node<Msg> {
    let words = note(a.why.clone()).tone(if a.failed { Tone::Problem } else if a.held || a.heard { Tone::Warn } else { Tone::Plain }).key("sharewhy");
    if a.failed || a.held || a.heard {
        row(vec![words.width(Length::Fill), crate::common::more::button(Msg::Help(crate::files::help::SHARE_LOGIN))]).gap(8.0).width(Length::Fill)
    } else {
        words
    }
}

pub fn view(a: &Ask, turn: u32) -> Node<Msg> {
    let it = a.share.as_ref().map_or_else(|| a.server.clone(), |s| t!("folders-shares-share-on", share = s, server = &a.server));
    let (user_turn, password_turn) = if a.failed && !a.user.trim().is_empty() { (0, turn) } else { (turn, 0) };
    let mut body = vec![
        crate::files::dialogs::head_to(crate::files::icons::stand_in::SERVER, t!("folders-shares-card-title", it = &it), t!("folders-shares-close"), share_msg(ShareMsg::Close)),
        why_row(a),
        entry(a.user.clone(), t!("folders-shares-user-placeholder"), |t| share_msg(ShareMsg::User(t)))
            .key("shareuser").grab(user_turn).on_submit(share_msg(ShareMsg::Go)).on_escape(share_msg(ShareMsg::Close)),
        entry(a.password.to_string(), t!("folders-shares-password"), |t| share_msg(ShareMsg::Password(t)))
            .secret().key("sharepassword").grab(password_turn).on_submit(share_msg(ShareMsg::Go)).on_escape(share_msg(ShareMsg::Close)),
        row(vec![
            label(remember()).width(Length::Fill),
            switch(a.remember, remember(), |on| share_msg(ShareMsg::Remember(on))),
        ]).gap(8.0).width(Length::Fill).centred(),
        note(remember_line()).key("shareremember"),
    ];
    if let Some(keyring) = &a.keyring {
        body.insert(2, row(vec![note(keyring.clone()).tone(Tone::Problem).key("sharekeyring").width(Length::Fill), crate::common::more::button(Msg::Help(crate::files::help::SHARE_LOGIN))]).gap(8.0).width(Length::Fill));
    }
    if a.held && !a.before.is_empty() && a.remember {
        body.push(row(vec![
            label(t!("folders-shares-forget")).width(Length::Fill),
            switch(a.forget, t!("folders-shares-forget-spoken"), |on| share_msg(ShareMsg::Forget(on))),
        ]).gap(8.0).width(Length::Fill).centred());
    }
    if let Some(at) = &a.address {
        body.push(note(where_words(at, a.heard)).key("sharewhere"));
    }
    let mut left = Vec::new();
    if a.busy {
        left.push(row(vec![note(t!("folders-shares-opening")), standard_ring(None, 32.0)]).gap(12.0).centred());
    }
    body.push(crate::files::dialogs::foot(left, main_action(crate::files::icons::stand_in::SERVER, t!("folders-shares-open"))
        .on_press(share_msg(ShareMsg::Go)).off(a.user.trim().is_empty() || a.address.is_none()).off_because(crate::common::ui::wait_while(a.busy))));
    card(column(body).gap(12.0).width(Length::Fill)).width(Length::Fixed(NARROW)).on_escape(share_msg(ShareMsg::Close))
}
