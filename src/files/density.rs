// How close together Folders' rows sit.

use super::app::Files;
use crate::toolkit::t;
use crate::toolkit::theme::{self, FoldersRoom};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Density {
    #[default]
    Compact,
    Normal,
    Roomy,
}

impl Density {
    pub const ALL: [Density; 3] = [Density::Compact, Density::Normal, Density::Roomy];

    pub fn word(self) -> &'static str {
        match self {
            Density::Compact => t!("folders-density-compact"),
            Density::Normal => t!("folders-density-normal"),
            Density::Roomy => t!("folders-density-roomy"),
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Density::Compact => "compact",
            Density::Normal => "normal",
            Density::Roomy => "roomy",
        }
    }

    pub fn named(word: &str) -> Density {
        Density::ALL.into_iter().find(|d| d.key() == word.trim()).unwrap_or_default()
    }

    pub fn room(self) -> &'static FoldersRoom {
        let rooms = &theme::sheet().folders;
        match self {
            Density::Compact => theme::compact_room(),
            Density::Normal => &rooms[1],
            Density::Roomy => &rooms[2],
        }
    }
}

impl Files {
    pub(super) fn room_now(&self) -> &'static FoldersRoom {
        self.density.room()
    }

    pub(super) fn set_density(&mut self, to: Density) {
        if to == self.density {
            return;
        }
        let mut held: [Option<(usize, f32)>; 2] = [None, None];
        for (q, held) in held.iter_mut().enumerate() {
            let pane = &self.cols[q];
            let grid = super::view::grid_of(self, pane, q);
            let anchor = pane.chosen.or(pane.seen.top()).filter(|i| *i < pane.entries.len());
            *held = anchor.map(|i| (i, grid.row_top(grid.place(i).0)));
        }
        self.density = to;
        self.nudging += 1;
        let turn = self.nudging;
        for (q, held) in held.into_iter().enumerate() {
            let Some((i, was)) = held else { continue };
            let grid = super::view::grid_of(self, &self.cols[q], q);
            if grid.across() {
                self.seek(q);
                continue;
            }
            let by = grid.row_top(grid.place(i).0) - was;
            self.pane_mut(q).nudge = (turn, by);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::app::{Msg, View};
    use crate::files::columns::Col;
    use crate::files::icons::{LINE_ICON, TICK};
    use crate::toolkit::ui::App;
    use std::path::PathBuf;

    fn scratch(tag: &str, files: usize) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-folders-density-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for i in 0..files {
            std::fs::write(dir.join(format!("made up {i:03}.txt")), b"made up").unwrap();
        }
        dir
    }

    #[test]
    fn everyone_starts_on_compact_and_a_level_is_read_by_its_word() {
        assert_eq!(Density::default(), Density::Compact);
        assert_eq!((Density::named("compact"), Density::named("roomy"), Density::named("normal")), (Density::Compact, Density::Roomy, Density::Normal));
        assert_eq!((Density::named(""), Density::named("enormous")), (Density::Compact, Density::Compact), "anything else is Compact");
        assert_eq!(Density::ALL.map(Density::word), ["Compact", "Normal", "Roomy"]);
        let dir = scratch("start", 1);
        assert_eq!(Files::new(vec![dir.clone()]).density, Density::Compact);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn each_level_is_tighter_than_the_next_and_none_is_too_small() {
        let (c, n, r) = (Density::Compact.room(), Density::Normal.room(), Density::Roomy.room());
        let sheet = theme::sheet();
        assert_eq!((r.row, r.rich_row, r.line, r.side_row, r.gap, r.icon_room), (54.0, 54.0, 28.0, 34.0, 8.0, (60.0, 52.0)), "Roomy is how it was");
        for (tight, loose) in [(c, n), (n, r)] {
            assert!(tight.row < loose.row && tight.rich_row < loose.rich_row && tight.line < loose.line);
            assert!(tight.side_row < loose.side_row && tight.gap < loose.gap);
            assert!(tight.icon_room.0 < loose.icon_room.0 && tight.icon_room.1 < loose.icon_room.1);
        }
        assert!(n.row <= r.row - 16.0, "Normal is clearly tighter than Folders was");
        let (line, small) = (sheet.text.0 * 1.21, sheet.note.0 * 1.21);
        for room in [c, n, r] {
            for h in [room.row, room.rich_row, room.line, room.side_row] {
                assert!(h >= theme::TARGET_LEAST, "{h} is too small to press");
            }
            assert!(room.row >= line + 2.0 && room.line >= line + 2.0 && room.side_row >= line + 2.0, "words clear the row's edges");
            assert!(room.rich_row >= line + sheet.rich_gap + small + 2.0, "both lines of a rich row fit: {}", room.rich_row);
            assert!(room.rich_row >= sheet.rich_icon + 2.0 && room.line >= LINE_ICON + 2.0 && room.line >= TICK + 2.0);
            assert!(room.icon_room.1 >= 8.0 + 6.0 + line + 8.0, "the name fits under its icon: {}", room.icon_room.1);
        }
    }

    #[test]
    fn the_level_sets_the_rows_of_every_view_in_both_panes() {
        let dir = scratch("views", 6);
        let mut f = Files::new(vec![dir.clone()]);
        f.update(Msg::PanesAre(true));
        f.go(1, crate::files::places::Where::Folder(dir.clone()));
        for d in Density::ALL {
            f.set_density(d);
            let room = d.room();
            for p in 0..2 {
                let pane = &f.cols[p];
                f.view = View::List;
                f.kept.columns = Some(vec![Col::Size, Col::Type, Col::Date]);
                pane.width.set(900.0);
                assert_eq!(crate::files::view::grid_of(&f, pane, p).h, room.row + crate::files::view::ROW_ROOM, "{d:?}: plain rows, pane {p}");
                f.kept.columns = Some(vec![Col::Size, Col::Date]);
                assert_eq!(crate::files::view::grid_of(&f, pane, p).h, room.row + crate::files::view::ROW_ROOM, "{d:?}: one line with no Type column too, pane {p}");
                f.view = View::Compact;
                assert_eq!(crate::files::view::grid_of(&f, pane, p).h, room.line + crate::files::view::ROW_ROOM, "{d:?}: the Compact view, pane {p}");
                f.view = View::Icons;
                let g = crate::files::view::grid_of(&f, pane, p);
                assert_eq!((g.h, g.cols), (f.kept.icon() + room.icon_room.1, (900.0 / (f.kept.icon() + room.icon_room.0)).floor() as usize), "{d:?}: icons, pane {p}");
            }
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_chosen_thing_stays_in_view_when_the_level_changes() {
        let dir = scratch("keep", 60);
        let mut f = Files::new(vec![dir.clone()]);
        f.update(Msg::PanesAre(false));
        for view in [View::List, View::Compact, View::Icons] {
            f.view = view;
            f.set_density(Density::Roomy);
            f.update(Msg::Choose(0, 40));
            let before = crate::files::view::grid_of(&f, &f.cols[0], 0);
            let was = before.row_top(before.place(40).0);
            f.set_density(Density::Compact);
            let after = crate::files::view::grid_of(&f, &f.cols[0], 0);
            let now = after.row_top(after.place(40).0);
            if after.across() {
                assert!(after.h < before.h, "{view:?}: the lines did get tighter");
                assert_eq!(f.cols[0].seek, f.seeking, "{view:?}: brought into sight");
                continue;
            }
            assert!(now < was, "{view:?}: the rows did get tighter");
            assert_eq!(f.cols[0].nudge.1, now - was, "{view:?}: scrolled on by as far as the row moved");
            assert_eq!(f.cols[0].nudge.0, f.nudging);
        }
        let turn = f.nudging;
        f.set_density(Density::Compact);
        assert_eq!(f.nudging, turn);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
