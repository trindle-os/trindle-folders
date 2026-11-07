// The Connect to a server card: what it asks, and how it is drawn.

use std::path::PathBuf;

use crate::toolkit::t;
use crate::toolkit::tiles::standard_ring;
use crate::toolkit::widget::{button, card, column, entry, main_action, note, row, Length, Node, Tone};

use crate::files::icons;
use crate::files::help;
use crate::files::dialogs;
use crate::files::app::Msg;
use crate::files::leave::Card;
use crate::files::remote::{self, Address, Error};
use super::{ftp_cannot_refuse, warns_cannot_refuse};
use super::window::FarMsg;

#[derive(Clone, Debug, PartialEq)]
pub enum Question {
    NewHost(String),
    Changed(String),
    NeedsKey,
    NewCert { host: String, fingerprint: String, why: String },
    CertChanged { host: String, fingerprint: String },
    Heard { host: String, at: String, port: u16 },
}

#[derive(Clone, Debug)]
pub enum Why {
    Ask(Question),
    Words(String, &'static str),
}

pub(super) fn why(e: Error, phone: bool) -> Why {
    match e {
        Error::HostKeyUnknown(h) => Why::Ask(Question::NewHost(h)),
        Error::HostKeyChanged(h) => Why::Ask(Question::Changed(h)),
        Error::NeedsPassword => Why::Ask(Question::NeedsKey),
        Error::CertUnknown { host, fingerprint, why } => Why::Ask(Question::NewCert { host, fingerprint, why }),
        Error::CertChanged { host, fingerprint } => Why::Ask(Question::CertChanged { host, fingerprint }),
        Error::Heard { host, at, port } => Why::Ask(Question::Heard { host, at, port }),
        e => Why::Words(e.to_string(), help::remote_page(&e, phone)),
    }
}

pub struct Connect {
    pub address: String,
    pub password: String,
    pub key: String,
    pub busy: bool,
    pub question: Option<Question>,
    pub bad: Option<(String, &'static str)>,
    pub trust: bool,
    pub trust_cert: Option<String>,
    pub pane: usize,
}

impl Connect {
    pub fn new(address: String, pane: usize) -> Connect {
        Connect { address, password: String::new(), key: String::new(), busy: false, question: None, bad: None, trust: false, trust_cert: None, pane }
    }
}

impl Card for Connect {
    fn shows(&self) -> Option<String> {
        match &self.question {
            Some(Question::Heard { at, .. }) => Some(at.clone()),
            _ => None,
        }
    }
}

const NARROW: f32 = 440.0;

pub(super) fn far(m: FarMsg) -> Msg {
    Msg::Far(m)
}

pub fn view(c: &Connect, turn: u32) -> Node<Msg> {
    let ftp = typed(&c.address).is_ok_and(|a| a.scheme.takes_password());
    let mut body = vec![
        dialogs::head_to(icons::stand_in::SERVER, t!("folders-far-connect-title"), t!("folders-far-connect-close"), far(FarMsg::Close)),
        entry(c.address.clone(), t!("folders-far-connect-address"), |t| far(FarMsg::Address(t)))
            .key("faraddress").grab(turn).on_submit(far(FarMsg::Go)).on_escape(far(FarMsg::Close)),
    ];
    if ftp {
        body.push(entry(c.password.clone(), t!("folders-far-connect-password"), |t| far(FarMsg::Password(t)))
            .secret().key("farpassword").on_submit(far(FarMsg::Go)).on_escape(far(FarMsg::Close)));
    } else {
        body.push(row(vec![
            entry(c.key.clone(), t!("folders-far-connect-key"), |t| far(FarMsg::Key(t)))
                .key("farkey").on_submit(far(FarMsg::Go)).on_escape(far(FarMsg::Close)).width(Length::Fill),
            button(icons::stand_in::KEY_FILE, t!("folders-far-connect-pick-key")).on_press(far(FarMsg::PickKey)),
        ]).gap(8.0).width(Length::Fill));
        body.push(note(t!("folders-far-connect-ssh-setup")));
    }
    if typed(&c.address).is_ok_and(|a| warns_cannot_refuse(a.scheme)) {
        body.push(note(ftp_cannot_refuse()).key("farftp"));
    }
    let trust = question_notes(c, ftp, &mut body);
    if let Some((b, page)) = &c.bad {
        body.push(with_more(note(b.clone()).tone(Tone::Problem), page));
    }
    if c.busy {
        body.push(row(vec![note(t!("folders-far-connect-connecting")), standard_ring(None, 32.0)]).gap(12.0).centred());
    }
    let main = if trust {
        main_action(icons::stand_in::SECURITY, t!("folders-far-connect-trust")).on_press(far(FarMsg::Trust)).off_because(crate::common::ui::wait_while(c.busy))
    } else {
        main_action(icons::stand_in::SERVER, t!("folders-far-connect-go")).on_press(far(FarMsg::Go)).off(c.address.trim().is_empty()).off_because(crate::common::ui::wait_while(c.busy))
    };
    body.push(dialogs::foot(Vec::new(), main));
    card(column(body).gap(12.0).width(Length::Fill)).width(Length::Fixed(NARROW)).on_escape(far(FarMsg::Close))
}

fn question_notes(c: &Connect, ftp: bool, body: &mut Vec<Node<Msg>>) -> bool {
    let mut trust = false;
    match &c.question {
        Some(Question::NewHost(h)) => {
            body.push(with_more(note(t!("folders-far-connect-new-host", host = h)).tone(Tone::Warn), help::SERVER_LOGIN));
            trust = true;
        }
        Some(Question::Changed(h)) => {
            body.push(with_more(note(t!("folders-far-connect-host-changed", host = h)).tone(Tone::Problem), help::SERVER_LOGIN));
        }
        Some(Question::NeedsKey) if ftp => {
            body.push(with_more(note(t!("folders-far-connect-needs-password")).tone(Tone::Warn), help::SERVER_LOGIN));
        }
        Some(Question::NeedsKey) => {
            body.push(with_more(note(t!("folders-far-connect-needs-key")).tone(Tone::Warn), help::SERVER_LOGIN));
        }
        Some(Question::NewCert { host, fingerprint, why }) => {
            body.push(with_more(note(t!("folders-far-connect-cert-new", host = host, why = why)).tone(Tone::Warn), help::SERVER_LOGIN));
            body.push(note(fingerprint.clone()).key("farfingerprint"));
            trust = true;
        }
        Some(Question::CertChanged { host, fingerprint }) => {
            body.push(with_more(note(t!("folders-far-connect-cert-changed", host = host)).tone(Tone::Warn), help::SERVER_LOGIN));
            body.push(note(fingerprint.clone()).key("farfingerprint"));
            trust = true;
        }
        Some(Question::Heard { host, at, port }) => {
            body.push(with_more(note(remote::heard(host, at, remote::Heard::Card(*port))).tone(Tone::Warn).key("farheard"), help::SERVER_LOGIN));
        }
        None => {}
    }
    trust
}

fn with_more(words: Node<Msg>, page: &'static str) -> Node<Msg> {
    row(vec![words.width(Length::Fill), crate::common::more::button(Msg::Help(page))]).gap(8.0).width(Length::Fill)
}

pub(super) fn tilde(text: &str) -> PathBuf {
    match text.strip_prefix("~/") {
        Some(rest) => crate::common::files::home().join(rest),
        None => PathBuf::from(text),
    }
}

pub(super) fn typed(text: &str) -> Result<Address, Error> {
    let text = text.trim();
    if text.contains("://") { Address::parse(text) } else { Address::parse(&format!("sftp://{text}")) }
}
