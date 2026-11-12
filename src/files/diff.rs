//! Two files compared side by side.

use std::io::{self, BufReader, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use tiny_skia::{FillRule, PixmapMut, Transform};

use crate::toolkit::colour::paint;
use crate::toolkit::t;
use crate::toolkit::ui::rounded;
use super::viewer::{self, Kind};
use crate::toolkit::text::Style;
use crate::toolkit::theme::{self, alpha};
use crate::toolkit::widget::{column, drawing, Length, MenuRow, Node, Pen, Rect};

const MOST_TEXT: u64 = 4 << 20;
const MOST_PAIRS: usize = 4_000_000;
const CONTEXT: usize = 3;
const STRIP: usize = 64;
const ROW: f32 = 20.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    Same,
    Changed,
    LeftOnly,
    RightOnly,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    Pair { left: Option<(usize, String)>, right: Option<(usize, String)>, mark: Mark },
    Fold(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Text { lines: Vec<Line>, changes: usize, cut: bool, whole: bool },
    Bytes { first: Option<u64>, sizes: (u64, u64) },
}

pub fn compare(left: &Path, right: &Path, stop: &AtomicBool) -> Result<Outcome, String> {
    let text = |p: &Path| viewer::kind(p) == Kind::Text;
    if !(text(left) && text(right)) {
        return bytes(left, right, stop).map_err(|e| e.to_string());
    }
    let read = |p: &Path| viewer::read_text(p, MOST_TEXT).map_err(|e| e.to_string());
    let ((a, _, cut_a, _), (b, _, cut_b, _)) = (read(left)?, read(right)?);
    let (a, b): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    let (ops, whole) = match_lines(&a, &b, stop).ok_or(t!("folders-diff-stopped"))?;
    let lines = side_by_side(&a, &b, &ops);
    let changes = lines.iter().filter(|l| matches!(l, Line::Pair { mark, .. } if *mark != Mark::Same)).count();
    Ok(Outcome::Text { lines: fold(lines), changes, cut: cut_a || cut_b, whole })
}

fn bytes(left: &Path, right: &Path, stop: &AtomicBool) -> io::Result<Outcome> {
    let sizes = (std::fs::metadata(left)?.len(), std::fs::metadata(right)?.len());
    let (mut a, mut b) = (BufReader::new(crate::common::files::open_file(left)?), BufReader::new(crate::common::files::open_file(right)?));
    let (mut x, mut y) = (vec![0u8; 1 << 16], vec![0u8; 1 << 16]);
    let mut at = 0u64;
    loop {
        if stop.load(Ordering::Relaxed) {
            return Err(io::Error::from(io::ErrorKind::Interrupted));
        }
        let n = a.read(&mut x)?;
        let m = fill(&mut b, &mut y[..n.max(1)])?;
        if let Some(i) = x[..n.min(m)].iter().zip(&y[..n.min(m)]).position(|(p, q)| p != q) {
            return Ok(Outcome::Bytes { first: Some(at + i as u64), sizes });
        }
        if n != m {
            return Ok(Outcome::Bytes { first: Some(at + n.min(m) as u64), sizes });
        }
        if n == 0 {
            return Ok(Outcome::Bytes { first: None, sizes });
        }
        at += n as u64;
    }
}

fn fill(r: &mut impl Read, buf: &mut [u8]) -> io::Result<usize> {
    let mut got = 0;
    while got < buf.len() {
        match r.read(&mut buf[got..])? {
            0 => break,
            n => got += n,
        }
    }
    Ok(got)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Same,
    Gone,
    Added,
}

fn match_lines(a: &[&str], b: &[&str], stop: &AtomicBool) -> Option<(Vec<Op>, bool)> {
    let head = a.iter().zip(b).take_while(|(x, y)| x == y).count();
    let tail = a[head..].iter().rev().zip(b[head..].iter().rev()).take_while(|(x, y)| x == y).count();
    let (am, bm) = (&a[head..a.len() - tail], &b[head..b.len() - tail]);
    let mut ops = vec![Op::Same; head];
    let whole = am.len().saturating_mul(bm.len()) <= MOST_PAIRS;
    if whole {
        let w = bm.len() + 1;
        let mut best = vec![0u32; (am.len() + 1) * w];
        for i in (0..am.len()).rev() {
            if stop.load(Ordering::Relaxed) {
                return None;
            }
            for j in (0..bm.len()).rev() {
                best[i * w + j] = if am[i] == bm[j] { best[(i + 1) * w + j + 1] + 1 } else { best[(i + 1) * w + j].max(best[i * w + j + 1]) };
            }
        }
        let (mut i, mut j) = (0, 0);
        while i < am.len() || j < bm.len() {
            if i < am.len() && j < bm.len() && am[i] == bm[j] {
                ops.push(Op::Same);
                (i, j) = (i + 1, j + 1);
            } else if j == bm.len() || (i < am.len() && best[(i + 1) * w + j] >= best[i * w + j + 1]) {
                ops.push(Op::Gone);
                i += 1;
            } else {
                ops.push(Op::Added);
                j += 1;
            }
        }
    } else {
        ops.extend(std::iter::repeat_n(Op::Gone, am.len()));
        ops.extend(std::iter::repeat_n(Op::Added, bm.len()));
    }
    ops.extend(std::iter::repeat_n(Op::Same, tail));
    Some((ops, whole))
}

fn side_by_side(a: &[&str], b: &[&str], ops: &[Op]) -> Vec<Line> {
    let mut out = Vec::new();
    let (mut i, mut j, mut k) = (0, 0, 0);
    while k < ops.len() {
        if ops[k] == Op::Same {
            out.push(Line::Pair { left: Some((i + 1, a[i].to_string())), right: Some((j + 1, b[j].to_string())), mark: Mark::Same });
            (i, j, k) = (i + 1, j + 1, k + 1);
            continue;
        }
        let gone = ops[k..].iter().take_while(|o| **o == Op::Gone).count();
        let added = ops[k + gone..].iter().take_while(|o| **o == Op::Added).count();
        for n in 0..gone.max(added) {
            let left = (n < gone).then(|| (i + n + 1, a[i + n].to_string()));
            let right = (n < added).then(|| (j + n + 1, b[j + n].to_string()));
            let mark = match (&left, &right) {
                (Some(_), Some(_)) => Mark::Changed,
                (Some(_), None) => Mark::LeftOnly,
                _ => Mark::RightOnly,
            };
            out.push(Line::Pair { left, right, mark });
        }
        (i, j, k) = (i + gone, j + added, k + gone + added);
    }
    out
}

fn fold(lines: Vec<Line>) -> Vec<Line> {
    let same = |l: &Line| matches!(l, Line::Pair { mark: Mark::Same, .. });
    let mut out = Vec::new();
    let mut k = 0;
    while k < lines.len() {
        if !same(&lines[k]) {
            out.push(lines[k].clone());
            k += 1;
            continue;
        }
        let run = lines[k..].iter().take_while(|l| same(l)).count();
        let keep_before = if k == 0 { 0 } else { CONTEXT };
        let keep_after = if k + run == lines.len() { 0 } else { CONTEXT };
        if run > keep_before + keep_after + 1 {
            out.extend(lines[k..k + keep_before].iter().cloned());
            out.push(Line::Fold(run - keep_before - keep_after));
            out.extend(lines[k + run - keep_after..k + run].iter().cloned());
        } else {
            out.extend(lines[k..k + run].iter().cloned());
        }
        k += run;
    }
    out
}

pub fn view<M: 'static>(lines: &[Line]) -> Node<M> {
    let strips = lines.chunks(STRIP).map(|chunk| {
        let rows = chunk.to_vec();
        let h = rows.len() as f32 * ROW;
        let said = chunk.to_vec();
        drawing((0.0, h), "", move |canvas, pen| paint_rows(canvas, pen, &rows))
            .menu(move |_, y| copy_rows(&said, y))
            .width(Length::Fill).height(Length::Fixed(h)).only_when_seen()
    }).collect();
    column(strips).gap(0.0).width(Length::Fill)
}

fn copy_rows<M>(rows: &[Line], y: f32) -> Vec<MenuRow<M>> {
    let Some(Line::Pair { left, right, .. }) = rows.get((y / ROW).max(0.0) as usize) else { return Vec::new() };
    let mut out = Vec::new();
    for (words, line) in [(t!("folders-diff-copy-left"), left), (t!("folders-diff-copy-right"), right)] {
        if let Some((_, text)) = line {
            let mut row = MenuRow::copy(text.clone()).words(words);
            row.keys = String::new();
            out.push(row);
        }
    }
    out
}

fn paint_rows(canvas: &mut PixmapMut, pen: &Pen, rows: &[Line]) {
    let (t, s) = (pen.theme, pen.scale);
    let mono = Style { family: Some("DejaVu Sans Mono".into()), ..Style::new((theme::sheet().note.0 * s).round()) };
    let dots = pen.fonts.layout("\u{2026}", &mono, None);
    let half = canvas.width() as f32 / 2.0;
    let number_w = 44.0 * s;
    let h = ROW * s;
    for (n, line) in rows.iter().enumerate() {
        let y = n as f32 * h;
        match line {
            Line::Fold(count) => {
                let words = t!("folders-diff-same-lines", n = *count);
                let laid = pen.fonts.layout(&words, &Style { align: crate::toolkit::text::Align::Centre, ..mono.clone() }, None);
                pen.fonts.draw(canvas, &laid, (canvas.width() as f32 - laid.width) / 2.0, y + (h - laid.height) / 2.0, t.muted);
            }
            Line::Pair { left, right, mark } => {
                let tint = match mark {
                    Mark::Same => None,
                    Mark::Changed => Some(alpha(t.accent, 0.14)),
                    Mark::LeftOnly | Mark::RightOnly => Some(alpha(t.accent, 0.26)),
                };
                for (side, cell) in [(0.0, left), (half, right)] {
                    let r = Rect { x: side + 2.0 * s, y: y + 1.0 * s, w: half - 4.0 * s, h: h - 2.0 * s };
                    let corner = crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Row, r.w.min(r.h), s);
                    match (tint, cell) {
                        (Some(c), Some(_)) => {
                            if let Some(path) = rounded(r, corner) {
                                canvas.fill_path(&path, &paint(c), FillRule::Winding, Transform::identity(), None);
                            }
                        }
                        (Some(_), None) => {
                            if let Some(path) = rounded(r, corner) {
                                canvas.fill_path(&path, &paint(alpha(t.muted, 0.08)), FillRule::Winding, Transform::identity(), None);
                            }
                        }
                        _ => {}
                    }
                    let Some((number, text)) = cell else { continue };
                    let num = pen.fonts.layout(&number.to_string(), &mono, None);
                    pen.fonts.draw(canvas, &num, side + number_w - num.width - 6.0 * s, y + (h - num.height) / 2.0, t.muted);
                    let shown = text.replace('\t', "    ");
                    let laid = pen.fonts.layout(&shown, &mono, None).cut(half - number_w - 8.0 * s, &dots);
                    pen.fonts.draw(canvas, &laid, side + number_w + 2.0 * s, y + (h - laid.height) / 2.0, t.fg);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("trindle-diff-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn changed_lines_are_paired_and_the_same_folded() {
        let d = dir("text");
        let a: Vec<String> = (1..=20).map(|n| format!("line {n}")).collect();
        let mut b = a.clone();
        b[9] = "line ten".into();
        b.insert(15, "a new line".into());
        b.remove(1);
        std::fs::write(d.join("a.txt"), a.join("\n")).unwrap();
        std::fs::write(d.join("b.txt"), b.join("\n")).unwrap();
        let Ok(Outcome::Text { lines, changes, cut, whole }) = compare(&d.join("a.txt"), &d.join("b.txt"), &AtomicBool::new(false)) else { panic!() };
        assert_eq!((changes, cut, whole), (3, false, true));
        let marks: Vec<String> = lines.iter().map(|l| match l {
            Line::Fold(n) => format!("~{n}"),
            Line::Pair { left, right, mark } => format!("{mark:?} {} {}",
                left.as_ref().map_or(0, |l| l.0), right.as_ref().map_or(0, |r| r.0)),
        }).collect();
        assert_eq!(marks, [
            "Same 1 1", "LeftOnly 2 0", "Same 3 2", "Same 4 3", "Same 5 4", "Same 6 5", "Same 7 6", "Same 8 7", "Same 9 8",
            "Changed 10 9", "Same 11 10", "Same 12 11", "Same 13 12", "Same 14 13", "Same 15 14", "RightOnly 0 15",
            "Same 16 16", "Same 17 17", "Same 18 18", "~2",
        ]);
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn other_files_are_compared_byte_by_byte() {
        let d = dir("bytes");
        std::fs::write(d.join("a.bin"), [0u8, 1, 2, 3, 0xff]).unwrap();
        std::fs::write(d.join("b.bin"), [0u8, 1, 9, 3, 0xff]).unwrap();
        std::fs::write(d.join("c.bin"), [0u8, 1, 2, 3, 0xff]).unwrap();
        std::fs::write(d.join("d.bin"), [0u8, 1, 2]).unwrap();
        let go = |x: &str, y: &str| compare(&d.join(x), &d.join(y), &AtomicBool::new(false)).unwrap();
        assert_eq!(go("a.bin", "b.bin"), Outcome::Bytes { first: Some(2), sizes: (5, 5) });
        assert_eq!(go("a.bin", "c.bin"), Outcome::Bytes { first: None, sizes: (5, 5) });
        assert_eq!(go("a.bin", "d.bin"), Outcome::Bytes { first: Some(3), sizes: (5, 3) });
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_middle_too_big_is_shown_as_changed_whole() {
        let a: Vec<String> = (0..3000).map(|n| format!("a{n}")).collect();
        let b: Vec<String> = (0..3000).map(|n| format!("b{n}")).collect();
        let (a, b): (Vec<&str>, Vec<&str>) = (a.iter().map(String::as_str).collect(), b.iter().map(String::as_str).collect());
        let (ops, whole) = match_lines(&a, &b, &AtomicBool::new(false)).unwrap();
        assert!(!whole);
        assert_eq!(ops.len(), 6000);
    }
}
