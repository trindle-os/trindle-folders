// Which rows of a menu are kept in sub-menus, so the first level is short.

use super::lines::without;
use super::{line, Act, Kind, Line};
use crate::files::icons::stand_in;
use crate::toolkit::t;

struct Held {
    kind: Kind,
    icon: &'static str,
    groups: &'static [&'static [Act]],
}

const SUBS: [Held; 4] = [
    Held { kind: Kind::Select, icon: "edit-select-all-symbolic",
           groups: &[&[Act::SelectAll, Act::SelectNone, Act::Invert]] },
    Held { kind: Kind::Pack, icon: stand_in::SUB_PACK,
           groups: &[&[Act::Compress, Act::ExtractTo], &[Act::SendTo, Act::ShareOnNetwork, Act::ShareWithAi], &[Act::Link]] },
    Held { kind: Kind::Picture, icon: stand_in::SUB_PICTURE,
           groups: &[&[Act::Wallpaper], &[Act::TurnLeft, Act::TurnRight, Act::TurnOver, Act::Resize]] },
    Held { kind: Kind::Tools, icon: stand_in::SUB_TOOLS,
           groups: &[&[Act::Preview, Act::OpenWindow, Act::Terminal, Act::AsRoot], &[Act::Pin, Act::RingPin, Act::PinUp, Act::PinDown, Act::Colour],
                     &[Act::Dupes, Act::CompareFiles, Act::Earlier, Act::DeletedHere], &[Act::Wipe]] },
];

fn sub_words(kind: Kind) -> &'static str {
    match kind {
        Kind::Select => t!("folders-menu-sub-select"),
        Kind::Pack => t!("folders-menu-sub-pack"),
        Kind::Picture => t!("folders-menu-sub-picture"),
        Kind::Tools => t!("folders-menu-sub-tools"),
        Kind::Sort => t!("folders-view-sort-by"),
        Kind::Group => t!("folders-view-group-by"),
    }
}

pub(super) const WHOLE: usize = 9;
pub(super) const FOR_GOOD: [Act; 2] = [Act::Wipe, Act::Delete];

pub fn fold(lines: Vec<Line>) -> Vec<Line> {
    if lines.len() <= WHOLE {
        return lines;
    }
    let alone = !lines.iter().any(|l| l.act == Act::ExtractHere);
    let home = |act: Act| SUBS.iter().position(|s| s.groups.iter().any(|g| g.contains(&act))).filter(|_| !(alone && act == Act::ExtractTo));
    let subs: Vec<Vec<Line>> = SUBS.iter().enumerate().map(|(k, held)| {
        let mut rows = Vec::new();
        for group in held.groups {
            let mut first = !rows.is_empty();
            for act in *group {
                rows.extend(lines.iter().filter(|l| l.act == *act && home(*act) == Some(k)).map(|l| Line { rule: std::mem::take(&mut first), ..l.clone() }));
            }
        }
        let leads_away = rows.first().is_some_and(|l: &Line| FOR_GOOD.contains(&l.act));
        if rows.len() < 2 || leads_away { Vec::new() } else { rows }
    }).collect();
    let made = |act: Act| home(act).is_some_and(|k| !subs[k].is_empty());
    let mut out = without(lines, made);
    let at = out.iter().position(|l| l.act == Act::Trash || FOR_GOOD.contains(&l.act))
        .or_else(|| out.iter().position(|l| l.act == Act::Properties))
        .unwrap_or(out.len());
    let rows: Vec<Line> = SUBS.iter().zip(subs).filter(|(_, rows)| !rows.is_empty())
        .map(|(held, rows)| Line { sub: rows, ..line(Act::Sub(held.kind), sub_words(held.kind), held.icon, "") }).collect();
    if rows.is_empty() {
        return out;
    }
    if let Some(next) = out.get_mut(at) {
        next.rule = true;
    }
    out.splice(at..at, rows);
    out[at].rule = at > 0;
    out
}
