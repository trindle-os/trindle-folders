//! Files as the toolkit's App.

use super::*;

impl App for Files {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        self.handle(msg);
        self.pick_in_one_pane();
        self.choose_follow();
    }

    fn view(&self) -> Node<Msg> {
        crate::files::view::view(self)
    }

    fn focused(&self) -> Option<Msg> {
        Some(Msg::Settings)
    }

    fn resized(&mut self, w: f32, h: f32) -> bool {
        let panel = self.panel_room();
        self.room = (w, h);
        let wrapped = panel != self.panel_room();
        self.panel_wanted();
        match &mut self.dialog {
            Some(crate::files::dialogs::Dialog::Preview(pv)) => {
                pv.room = (w, h);
                true
            }
            _ => wrapped,
        }
    }

    fn intercept(&self, input: &Input, keys: bool) -> Option<Msg> {
        self.intercept_with(input, Keyboard { keys, text: false, arrows: keys })
    }

    fn dragging(&self) -> Option<crate::toolkit::ui::Lifted> {
        self.outside_held()
    }

    fn drop_end(&mut self, id: u64, way: crate::toolkit::dnd::Way) -> crate::toolkit::ui::DropEnd {
        self.outside_end(id, way)
    }

    fn intercept_with(&self, input: &Input, at: Keyboard) -> Option<Msg> {
        self.input_msg(input, at)
    }
}
