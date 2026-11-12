//! The keys Files answers, and the input it looks at before the toolkit does.

use std::time::Instant;

use super::*;

impl Files {
    pub(super) fn input_msg(&self, input: &Input, at: Keyboard) -> Option<Msg> {
        match input {
            Input::PointerIn { x, y } | Input::PointerMove { x, y } | Input::Press { x, y, .. } | Input::Release { x, y, .. }
            | Input::TouchDown { x, y, .. } | Input::TouchMove { x, y, .. } => self.pointer_input(input, *x, *y),
            Input::Scroll { y, steps, .. } if self.mods.get().ctrl && *y != 0.0 && self.nothing_floats() => self.wheel_zoom(*y, *steps),
            Input::DragIn { x, y, .. } | Input::DragOver { x, y } | Input::Dropped { x, y, .. } => {
                self.pointer.set((*x, *y));
                None
            }
            Input::DragEnded { short: Some(crate::toolkit::dnd::Short::Cut(way)), .. } => Some(Msg::OutShort(*way)),
            Input::DragEnded { short: Some(crate::toolkit::dnd::Short::Unshown(way)), .. } => Some(Msg::OutUnshown(*way)),
            Input::DragEnded { took, .. } => Some(Msg::OutEnded(*took)),
            Input::Focus(false) => {
                self.mods.set(Mods::default());
                None
            }
            Input::KeyUp { sym, mods } => {
                self.mods.set(held(*mods, *sym, false));
                None
            }
            Input::Mods(mods) => {
                self.mods.set(*mods);
                None
            }
            Input::Key { sym, mods, text, .. } => self.key_input(*sym, *mods, text.as_deref(), at),
            _ => None,
        }
    }

    fn nothing_floats(&self) -> bool {
        self.menu.is_none() && self.dialog.is_none() && self.confirm.is_none() && self.dropping.is_none()
    }

    fn pointer_input(&self, input: &Input, x: f32, y: f32) -> Option<Msg> {
        self.pointer.set((x, y));
        match input {
            Input::TouchDown { .. } => self.touch.set(true),
            Input::Press { .. } => self.touch.set(false),
            _ => {}
        }
        if let Input::Press { button: Button::Back | Button::Forward, .. } = input {
            if self.menu.is_none() && self.dialog.is_none() {
                return Some(Msg::Travel(self.pane_at(x), matches!(input, Input::Press { button: Button::Forward, .. })));
            }
        }
        None
    }

    fn wheel_zoom(&self, y: f32, steps: bool) -> Option<Msg> {
        use crate::files::zoom::{Zoom, WHEEL_STEP};
        let p = self.pane_at(self.pointer.get().0);
        if self.view_of(p) != View::Icons {
            return None;
        }
        let sum = if steps { y.signum() * WHEEL_STEP } else { self.wheel.get() + y };
        if sum.abs() < WHEEL_STEP {
            self.wheel.set(sum);
            return Some(Msg::Resized);
        }
        self.wheel.set(0.0);
        Some(Msg::Zoom(if sum < 0.0 { Zoom::In } else { Zoom::Out }, Some(p)))
    }

    fn key_input(&self, sym: Keysym, mods: Mods, text: Option<&str>, at: Keyboard) -> Option<Msg> {
        self.mods.set(held(mods, sym, true));
        if self.drag.is_some() && sym == Keysym::Escape {
            return Some(Msg::Escape);
        }
        if self.menu.is_some() {
            use crate::files::menu::MenuMsg;
            return Some(Msg::Menu(MenuMsg::Pop(crate::toolkit::popup::key(sym))));
        }
        if sym == Keysym::Escape {
            if let Some(closes) = self.escape_closes() {
                return Some(closes);
            }
        }
        let ctrl_f = mods.ctrl && matches!(sym, Keysym::f | Keysym::F);
        if self.find.open.is_some() && self.find.typing && !ctrl_f {
            return None;
        }
        if self.locate.is_some() || self.pin_rename.is_some() {
            return None;
        }
        if matches!(self.dialog, Some(crate::files::dialogs::Dialog::Preview(_))) {
            return viewer_key(sym);
        }
        if self.renaming.is_some() || self.confirm.is_some() || self.dropping.is_some() || self.dialog.is_some() {
            return None;
        }
        if sym == Keysym::Menu || (sym == Keysym::F10 && mods.shift) {
            return Some(Msg::Menu(crate::files::menu::MenuMsg::Open));
        }
        if let Some(t) = text.filter(|_| !mods.ctrl && !mods.alt && !mods.logo && !at.text && !at.keys && !at.arrows) {
            if self.ahead.takes(t, Instant::now()) {
                return Some(Msg::Typed(t.to_string()));
            }
        }
        key(sym, mods, at)
    }

    fn escape_closes(&self) -> Option<Msg> {
        if self.dialog.is_some() {
            return Some(Msg::Ask(crate::files::dialogs::AskMsg::Close));
        }
        if self.confirm.is_some() {
            return Some(Msg::NoDelete);
        }
        if self.dropping.is_some() {
            return Some(Msg::DropStop);
        }
        if self.reasons_open {
            return Some(Msg::Listed);
        }
        self.choose_escapes().then_some(Msg::Chooser(crate::files::choose::ChooseMsg::Cancel))
    }
}

fn viewer_key(sym: Keysym) -> Option<Msg> {
    use crate::files::preview::PreviewMsg;
    match sym {
        Keysym::space => Some(Msg::Preview(PreviewMsg::Space)),
        Keysym::Left | Keysym::KP_Left | Keysym::Up | Keysym::KP_Up => Some(Msg::Preview(PreviewMsg::Step(-1))),
        Keysym::Right | Keysym::KP_Right | Keysym::Down | Keysym::KP_Down => Some(Msg::Preview(PreviewMsg::Step(1))),
        Keysym::Page_Up | Keysym::KP_Page_Up => Some(Msg::Preview(PreviewMsg::Page(-1))),
        Keysym::Page_Down | Keysym::KP_Page_Down => Some(Msg::Preview(PreviewMsg::Page(1))),
        _ => None,
    }
}

pub(super) fn key(sym: Keysym, mods: Mods, at: Keyboard) -> Option<Msg> {
    let editing = matches!(sym, Keysym::a | Keysym::A | Keysym::x | Keysym::X | Keysym::c | Keysym::C | Keysym::v | Keysym::V | Keysym::z | Keysym::Z
                                | Keysym::y | Keysym::Y);
    if at.text && (matches!(sym, Keysym::BackSpace | Keysym::Delete | Keysym::KP_Delete) || (mods.ctrl && editing)) {
        return None;
    }
    if mods.ctrl && !mods.alt && mods.shift {
        return ctrl_shift_key(sym);
    }
    if mods.ctrl && !mods.alt {
        return ctrl_key(sym);
    }
    if mods.alt && !mods.ctrl {
        return if mods.shift { None } else { alt_key(sym) };
    }
    if mods.ctrl {
        return None;
    }
    plain_key(sym, mods, at)
}

fn ctrl_shift_key(sym: Keysym) -> Option<Msg> {
    match sym {
        Keysym::Tab | Keysym::ISO_Left_Tab => Some(Msg::NextTab(false)),
        Keysym::a | Keysym::A => Some(Msg::Unpick),
        Keysym::z | Keysym::Z => Some(Msg::Redo),
        Keysym::n | Keysym::N => Some(Msg::NewFolder),
        Keysym::plus | Keysym::equal | Keysym::KP_Add => Some(Msg::Zoom(crate::files::zoom::Zoom::In, None)),
        _ => None,
    }
}

fn ctrl_key(sym: Keysym) -> Option<Msg> {
    match sym {
        Keysym::Tab => Some(Msg::NextTab(true)),
        Keysym::ISO_Left_Tab => Some(Msg::NextTab(false)),
        Keysym::Page_Down | Keysym::KP_Page_Down => Some(Msg::NextTab(true)),
        Keysym::Page_Up | Keysym::KP_Page_Up => Some(Msg::NextTab(false)),
        Keysym::_1 | Keysym::KP_1 => Some(Msg::Look(View::Icons)),
        Keysym::_2 | Keysym::KP_2 => Some(Msg::Look(View::List)),
        Keysym::_3 | Keysym::KP_3 => Some(Msg::Look(View::Compact)),
        Keysym::l | Keysym::L => Some(Msg::Locate),
        Keysym::r | Keysym::R => Some(Msg::Reload),
        Keysym::d | Keysym::D => Some(Msg::Pin),
        Keysym::t | Keysym::T => Some(Msg::NewTab(None)),
        Keysym::w | Keysym::W => Some(Msg::CloseTab(None)),
        Keysym::n | Keysym::N => Some(Msg::NewWindow),
        Keysym::h | Keysym::H => Some(Msg::Hidden),
        Keysym::a | Keysym::A => Some(Msg::All),
        Keysym::i | Keysym::I => Some(Msg::Invert),
        Keysym::x | Keysym::X => Some(Msg::Cut),
        Keysym::c | Keysym::C => Some(Msg::CopyClip),
        Keysym::v | Keysym::V => Some(Msg::Paste),
        Keysym::z | Keysym::Z => Some(Msg::Undo),
        Keysym::y | Keysym::Y => Some(Msg::Redo),
        Keysym::f | Keysym::F => Some(Msg::Find(crate::files::found::FindMsg::Open(None))),
        Keysym::plus | Keysym::equal | Keysym::KP_Add => Some(Msg::Zoom(crate::files::zoom::Zoom::In, None)),
        Keysym::minus | Keysym::KP_Subtract => Some(Msg::Zoom(crate::files::zoom::Zoom::Out, None)),
        Keysym::_0 | Keysym::KP_0 => Some(Msg::Zoom(crate::files::zoom::Zoom::Usual, None)),
        Keysym::comma => Some(Msg::Own(crate::files::settings::OwnMsg::Open)),
        _ => None,
    }
}

fn alt_key(sym: Keysym) -> Option<Msg> {
    match sym {
        Keysym::Up | Keysym::KP_Up => Some(Msg::Up),
        Keysym::Left | Keysym::KP_Left => Some(Msg::Back),
        Keysym::Right | Keysym::KP_Right => Some(Msg::Forward),
        Keysym::Home | Keysym::KP_Home => Some(Msg::Home),
        Keysym::Return | Keysym::KP_Enter => Some(Msg::Props),
        Keysym::d | Keysym::D => Some(Msg::Locate),
        Keysym::p | Keysym::P => Some(Msg::Panel(crate::files::panel::PanelMsg::Switch)),
        _ => None,
    }
}

fn plain_key(sym: Keysym, mods: Mods, at: Keyboard) -> Option<Msg> {
    match sym {
        Keysym::BackSpace => return Some(Msg::Up),
        Keysym::F2 => return Some(Msg::Rename),
        Keysym::F3 => return Some(Msg::Panes),
        Keysym::F4 => return Some(Msg::Terminal),
        Keysym::F5 => return Some(Msg::Reload),
        Keysym::F6 => return Some(Msg::OtherPane),
        Keysym::Delete | Keysym::KP_Delete if mods.shift => return Some(Msg::Delete),
        Keysym::Delete | Keysym::KP_Delete => return Some(Msg::Trash),
        _ => {}
    }
    if at.keys || at.arrows {
        return None;
    }
    let step = match sym {
        Keysym::Left | Keysym::KP_Left => Step::Left,
        Keysym::Right | Keysym::KP_Right => Step::Right,
        Keysym::Up | Keysym::KP_Up => Step::Up,
        Keysym::Down | Keysym::KP_Down => Step::Down,
        Keysym::Home | Keysym::KP_Home => Step::First,
        Keysym::End | Keysym::KP_End => Step::Last,
        Keysym::Return | Keysym::KP_Enter => return Some(Msg::Enter),
        Keysym::space => return Some(Msg::Preview(crate::files::preview::PreviewMsg::Toggle)),
        _ => return None,
    };
    Some(Msg::Step(step, mods.shift))
}

fn held(mut mods: Mods, sym: Keysym, down: bool) -> Mods {
    match sym {
        Keysym::Control_L | Keysym::Control_R => mods.ctrl = down,
        Keysym::Shift_L | Keysym::Shift_R => mods.shift = down,
        _ => {}
    }
    mods
}
