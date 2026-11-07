// The Icons view's icon size.

use crate::toolkit::t;

use super::app::{Files, View};
use super::icons::{ICON_SIZES, ICON_USUAL};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zoom {
    In,
    Out,
    Usual,
}

pub const WHEEL_STEP: f32 = 40.0;

pub fn stepped(now: f32, way: Zoom) -> f32 {
    let at = ICON_SIZES.iter().position(|s| *s >= now).unwrap_or(ICON_SIZES.len() - 1);
    match way {
        Zoom::In => ICON_SIZES[(at + 1).min(ICON_SIZES.len() - 1)],
        Zoom::Out => ICON_SIZES[at.saturating_sub(1)],
        Zoom::Usual => ICON_USUAL,
    }
}

impl Files {
    pub(super) fn zoom(&mut self, way: Zoom, pointed: Option<usize>) {
        if self.view_of(pointed.unwrap_or(self.side())) != View::Icons {
            return;
        }
        let to = stepped(self.kept.icon(), way);
        if to == self.kept.icon() {
            return;
        }
        let mut held: [Option<(usize, f32, f32)>; 2] = [None, None];
        for (q, held) in held.iter_mut().enumerate() {
            if self.view_of(q) != View::Icons {
                continue;
            }
            let pane = &self.cols[q];
            let (grid, width, n) = (pane.grid.get(), pane.width.get(), pane.entries.len());
            let under = pane.pointed.filter(|_| pointed == Some(q)).and_then(|(x, y)| {
                let row = grid.row_under(y, n)?;
                let i = grid.hit(x, y, width, n).unwrap_or(grid.row_items(row, n).0);
                Some((i, (y - grid.row_top(row)) / grid.h))
            });
            let anchor = under.or(pane.chosen.map(|i| (i, 0.0))).or(pane.seen.top().map(|i| (i, 0.0))).filter(|(i, _)| *i < n);
            *held = anchor.map(|(i, share)| (i, share, grid.row_top(grid.place(i).0) + share * grid.h));
            pane.floor.set(grid.row_top(grid.rows(n)) + super::view::FOOT_LINES * grid.h);
        }
        if self.kept.set_icon(to).is_err() {
            self.say_help(t!("folders-zoom-not-kept"), super::help::NOT_KEPT);
        }
        self.pictures_resized();
        self.nudging += 1;
        let turn = self.nudging;
        for (q, held) in held.into_iter().enumerate() {
            let Some((i, share, was)) = held else { continue };
            let grid = super::view::grid_of(self, &self.cols[q], q);
            let by = grid.row_top(grid.place(i).0) + share * grid.h - was;
            self.pane_mut(q).nudge = (turn, by);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_size_steps_one_at_a_time_and_stops_at_the_ends() {
        assert_eq!(stepped(48.0, Zoom::In), 64.0);
        assert_eq!(stepped(48.0, Zoom::Out), 32.0);
        assert_eq!(stepped(32.0, Zoom::Out), 32.0);
        assert_eq!(stepped(256.0, Zoom::In), 256.0);
        assert_eq!(stepped(192.0, Zoom::Usual), ICON_USUAL);
        assert_eq!((stepped(50.0, Zoom::In), stepped(50.0, Zoom::Out), stepped(999.0, Zoom::Out)), (96.0, 48.0, 192.0));
    }
}
