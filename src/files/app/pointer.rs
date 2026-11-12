//! The pointer over a pane's items, and the steps and letters that move the choice.

use std::path::PathBuf;

use super::*;

impl Files {
    pub(super) fn pick_msg(&mut self, m: Msg) {
        match m {
            Msg::Choose(p, i) => {
                self.side = p;
                self.pane_mut(p).chosen = Some(i);
            }
            Msg::Activate(p) => self.side = p.min(self.cols.len() - 1),
            Msg::Pick(p, picked) => {
                self.side = p;
                self.pane_mut(p).picked = picked;
            }
            Msg::All => {
                let pane = self.pane_mut(self.side());
                pane.picked = (0..pane.entries.len()).collect();
            }
            Msg::TickAll(p) => {
                self.side = p;
                let pane = self.pane_mut(p);
                pane.picked = if pane.picked.len() == pane.entries.len() { Vec::new() } else { (0..pane.entries.len()).collect() };
            }
            Msg::Unpick => {
                let pane = self.pane_mut(self.side());
                pane.picked.clear();
                pane.anchor = pane.chosen;
            }
            Msg::Invert => self.pane_mut(self.side()).invert(),
            Msg::SameInOther => self.same_in_other(),
            Msg::SwapPanes => self.swap_panes(),
            Msg::ComparePanes => self.compare_panes(),
            Msg::PickDiffering => self.pick_differing(),
            Msg::Icon(p, row, at) => self.icon_pointer(p, row, at),
            Msg::Head(p, at) => self.head_pointer(p, at),
            Msg::Step(step, run) => self.step(step, run),
            Msg::Typed(text) => self.typed_at(&text, Instant::now()),
            _ => unreachable!("not about picking"),
        }
    }

    fn icon_pointer(&mut self, p: usize, row: usize, at: Pointer) {
        self.icon_pointer_at(p, row, at, Instant::now());
    }

    pub(in crate::files) fn icon_pointer_at(&mut self, p: usize, row: usize, at: Pointer, now: Instant) {
        let mods = self.mods.get();
        if !matches!(at, Pointer::Hover(..) | Pointer::Leave) {
            self.side = p;
        }
        let top = self.cols[p].grid.get().strip_top(row);
        if self.drag_pointer(p, at, top) {
            return;
        }
        if let Pointer::Middle(x, y) = at {
            let pane = &self.cols[p];
            let hit = pane.grid.get().hit(x, y + top, pane.width.get(), pane.entries.len());
            if let Some(to) = hit.and_then(|i| self.tab_place(p, i)) {
                self.tab_for(p, to, mods.shift);
            }
            return;
        }
        let (held_down, renaming) = (self.drag.is_some(), self.renaming.is_some());
        let named = crate::files::inplace::can_rename(&self.cols[p].at);
        let aim = Aim { top, mods, now, held_down, can_slow: named && !renaming };
        let pressed = self.pane_mut(p).pointer(at, &aim);
        self.after_pointer(p, pressed, &aim);
    }

    fn drag_pointer(&mut self, p: usize, at: Pointer, top: f32) -> bool {
        match at {
            Pointer::Move(..) if self.drag.is_some() => self.drag_moved(),
            Pointer::Up(x, y) if self.drag.is_some() => self.drag_ended(p, (x, y + top)),
            Pointer::RightDown(x, y) => self.hold_right(p, (x, y + top)),
            Pointer::RightMove(..) => {}
            Pointer::RightUp(x, y) => {
                if let Some(d) = self.drag.take().filter(|d| d.right) {
                    self.let_go(d, (x, y + top));
                }
                self.landed = None;
            }
            Pointer::Down(..) => {
                self.drag = None;
                self.slow = None;
                self.pin_at = None;
                if !self.bad {
                    self.said.clear();
                }
                return false;
            }
            _ => return false,
        }
        true
    }

    fn drag_moved(&mut self) {
        let (wx, wy) = self.pointer.get();
        let Some(d) = &mut self.drag else { return };
        if !d.moving && ((wx - d.start.0).abs() > DRAG_SLOP || (wy - d.start.1).abs() > DRAG_SLOP) {
            d.moving = true;
            d.was = crate::files::act::picked_up(&d.items, self.post.is_some());
            self.slow = None;
            self.said = t!("folders-app-dragging", what = &items_named(&d.items));
            self.bad = false;
        }
    }

    fn drag_ended(&mut self, p: usize, at: (f32, f32)) {
        let Some(d) = self.drag.take() else { return };
        if d.moving {
            self.let_go(d, at);
        } else if let Some(i) = d.then {
            self.pane_mut(p).pick(i, false);
        }
        self.landed = None;
        self.slow_let_go();
    }

    fn hold_right(&mut self, p: usize, (x, y): (f32, f32)) {
        self.drag = None;
        self.slow = None;
        self.landed = None;
        self.pin_at = None;
        let pane = self.pane_mut(p);
        let Some(i) = pane.grid.get().hit(x, y, pane.width.get(), pane.entries.len()) else { return };
        if !pane.picked.contains(&i) {
            pane.pick(i, false);
        }
        let items = pane.picked_paths();
        self.said = t!("folders-app-holding", what = &items_named(&items));
        self.bad = false;
        self.drag = Some(Drag { start: self.pointer.get(), pane: p, pins: crate::files::act::pinnable(&items), was: crate::files::act::picked_up(&items, self.post.is_some()), items, moving: true, then: None,
                                right: true });
    }

    fn after_pointer(&mut self, p: usize, pressed: Pressed, aim: &Aim) {
        let Pressed { opened, tabbed, held, menu, slow } = pressed;
        if let Some((items, then)) = held {
            self.drag = Some(Drag { start: self.pointer.get(), pane: p, pins: crate::files::act::pinnable(&items), was: Vec::new(), items, moving: false, then, right: false });
        }
        if let Some(path) = slow {
            self.slow_arm(p, path, aim.now);
        }
        if let Some(i) = opened {
            self.open(p, i);
        }
        if let Some(to) = tabbed.and_then(|i| self.tab_place(p, i)) {
            self.drag = None;
            self.tab_for(p, to, aim.mods.shift);
        }
        if menu {
            self.open_menu(p);
        }
    }

    fn step(&mut self, step: Step, run: bool) {
        let p = self.side();
        let pane = self.pane_mut(p);
        let n = pane.entries.len();
        if n == 0 {
            return;
        }
        let to = match (pane.chosen, step) {
            (None, Step::Last) => n - 1,
            (None, _) => 0,
            (Some(i), step) => pane.grid.get().step(i.min(n - 1), step, n),
        };
        pane.pick(to, run);
        self.seek(p);
    }

    pub(in crate::files) fn typed_at(&mut self, text: &str, now: Instant) {
        let p = self.side();
        let pane = &self.cols[p];
        let to = self.ahead.jump(text, now, pane.chosen, pane.entries.len(), |i| pane.entries[i].name.clone());
        if let Some(i) = to {
            self.pane_mut(p).pick(i, false);
            self.seek(p);
        }
    }

    pub(in crate::files) fn seek(&mut self, p: usize) {
        self.seeking += 1;
        let turn = self.seeking;
        self.pane_mut(p).seek = turn;
    }
}

struct Aim {
    top: f32,
    mods: Mods,
    now: Instant,
    held_down: bool,
    can_slow: bool,
}

#[derive(Default)]
struct Pressed {
    opened: Option<usize>,
    tabbed: Option<usize>,
    held: Option<(Vec<PathBuf>, Option<usize>)>,
    menu: bool,
    slow: Option<PathBuf>,
}

fn items_named(items: &[PathBuf]) -> String {
    match items {
        [one] => work::name_of(one),
        many => t!("folders-app-items", n = many.len()),
    }
}

impl Pane {
    fn picked_paths(&self) -> Vec<PathBuf> {
        self.picked.iter().filter_map(|&j| self.entries.get(j)).map(|e| e.path.clone()).collect()
    }

    fn invert(&mut self) {
        let mut was = vec![false; self.entries.len()];
        for i in &self.picked {
            if let Some(w) = was.get_mut(*i) {
                *w = true;
            }
        }
        self.picked = (0..was.len()).filter(|i| !was[*i]).collect();
    }

    fn pointer(&mut self, at: Pointer, aim: &Aim) -> Pressed {
        let (grid, width, n) = (self.grid.get(), self.width.get(), self.entries.len());
        let top = aim.top;
        let mut out = Pressed::default();
        match at {
            Pointer::Down(x, y) if grid.tick_hit(x, y + top, width, n).is_some() => {
                if let Some(i) = grid.tick_hit(x, y + top, width, n) {
                    self.toggle(i);
                    self.last = None;
                }
            }
            Pointer::Down(x, y) => self.press(grid.hit(x, y + top, width, n), (x, y + top), aim, &mut out),
            Pointer::Move(x, y) => {
                if let Some(b) = &mut self.band {
                    b.to = (x, y + top);
                    let mut picked = b.base.clone();
                    picked.extend(grid.in_band(icons::band_rect(b.from, b.to), width, n));
                    picked.sort_unstable();
                    picked.dedup();
                    self.picked = picked;
                }
            }
            Pointer::Up(..) => self.band = None,
            Pointer::Hover(x, y) => {
                self.hover = grid.hit(x, y + top, width, n);
                self.pointed = Some((x, y + top));
                if self.last.is_some_and(|(_, j)| self.hover != Some(j)) {
                    self.stayed = false;
                }
            }
            Pointer::Leave => {
                self.hover = None;
                self.pointed = None;
                self.stayed &= aim.held_down;
            }
            Pointer::Menu(x, y) => {
                match grid.hit(x, y + top, width, n) {
                    Some(i) if !self.picked.contains(&i) => self.pick(i, false),
                    Some(_) => {}
                    None => {
                        self.before = std::mem::take(&mut self.picked);
                        self.clear();
                    }
                }
                out.menu = true;
            }
            Pointer::Middle(..) | Pointer::RightDown(..) | Pointer::RightMove(..) | Pointer::RightUp(..) => {}
        }
        out
    }

    fn toggle(&mut self, i: usize) {
        match self.picked.iter().position(|&j| j == i) {
            Some(k) => {
                self.picked.remove(k);
            }
            None => self.picked.push(i),
        }
        self.picked.sort_unstable();
        self.chosen = Some(i);
        self.anchor = Some(i);
    }

    fn press(&mut self, hit: Option<usize>, at: (f32, f32), aim: &Aim, out: &mut Pressed) {
        let (x, y) = at;
        let (mods, now) = (aim.mods, aim.now);
        let Some(i) = hit else {
            let base = if mods.ctrl { self.picked.clone() } else { Vec::new() };
            if !mods.ctrl {
                self.picked.clear();
                self.chosen = None;
            }
            self.band = Some(Band { from: (x, y), to: (x, y), base });
            self.last = None;
            return;
        };
        let soon = self.last.is_some_and(|(t, j)| j == i && now.saturating_duration_since(t) < TWICE);
        let twice = !mods.ctrl && !mods.shift && soon;
        let tab_twice = mods.ctrl && !mods.alt && soon;
        let on_name = self.name_at.get().is_some_and(|(j, r)| j == i && r.contains(x, y));
        if aim.can_slow && mods == Mods::default() && self.picked == [i] && self.chosen == Some(i) && self.stayed
            && on_name && slow_second(self.last, i, now) {
            out.slow = self.entries.get(i).map(|e| e.path.clone());
        }
        self.stayed = true;
        let mut then = None;
        if mods.ctrl {
            self.toggle(i);
        } else if !mods.shift && self.picked.len() > 1 && self.picked.contains(&i) {
            self.chosen = Some(i);
            then = Some(i);
        } else {
            self.pick(i, mods.shift);
        }
        self.last = if twice || tab_twice { None } else { Some((now, i)) };
        if twice {
            out.opened = Some(i);
        } else if tab_twice {
            out.tabbed = Some(i);
        } else if self.picked.contains(&i) {
            out.held = Some((self.picked_paths(), then));
        }
    }
}
