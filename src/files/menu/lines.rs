//! The rows each place and each kind of pick offers, before the least used

use std::path::PathBuf;

use super::{line, Act, Kind, Line, Sel};
use crate::files::app::View;
use crate::files::icons::stand_in;
use crate::files::places::Where;
use crate::toolkit::t;

pub fn lines_for(at: &Where, sel: &[Sel], pinned: bool) -> Vec<Line> {
    let groups: Vec<Vec<Line>> = match (at, sel) {
        (Where::Network | Where::Computer, _) => Vec::new(),
        (Where::Inside(..), []) => vec![pick()],
        (Where::Inside(..), _) => vec![vec![open(), line(Act::ExtractHere, t!("folders-menu-take-out-next-to-the-archive"), "archive-extract-symbolic", "")], pick()],
        (Where::Remote(_), []) => vec![make(), pick(), vec![path()]],
        (Where::Remote(_), sel) => {
            let rename = if sel.len() == 1 { t!("folders-menu-rename") } else { t!("folders-menu-rename-them") };
            vec![make(), vec![open(), path(), line(Act::Rename, rename, stand_in::RENAME_MANY, "F2")], pick(), vec![shred()]]
        }
        (Where::Trash, []) => vec![pick(), vec![line(Act::EmptyTrash, t!("folders-menu-empty-the-trash"), stand_in::SHRED, "")]],
        (Where::Trash, _) => vec![vec![line(Act::Restore, t!("folders-menu-put-back"), "document-revert-symbolic", "")], vec![shred()], vec![props()]],
        (Where::Recent | Where::Found(..), []) => vec![pick()],
        (Where::Found(..), sel) => found_groups(sel),
        (Where::Recent, _) => vec![vec![open(), open_with(), preview()], vec![path(), copy()], vec![props()]],
        (Where::Folder(_), []) => vec![
            make(),
            vec![line(Act::Paste, t!("folders-menu-paste"), "edit-paste-symbolic", "Ctrl+V")],
            pick(),
            vec![dupes()],
            vec![terminal(), root()],
            vec![pin(pinned), colour()],
            vec![props()],
        ],
        (Where::Folder(_), sel) => folder_groups(sel, pinned),
    };
    let mut out = Vec::new();
    for g in groups.into_iter().filter(|g| !g.is_empty()) {
        let rule = !out.is_empty();
        for (k, l) in g.into_iter().enumerate() {
            out.push(Line { rule: rule && k == 0, ..l });
        }
    }
    out
}

fn open() -> Line {
    line(Act::Open, t!("folders-menu-open"), "document-open-symbolic", "Return")
}

fn props() -> Line {
    line(Act::Properties, t!("folders-menu-properties"), "document-properties-symbolic", "Alt+Return")
}

fn pick() -> Vec<Line> {
    vec![line(Act::SelectAll, t!("folders-menu-select-all"), "edit-select-all-symbolic", "Ctrl+A"),
         line(Act::SelectNone, t!("folders-menu-select-none"), stand_in::SELECT_NONE, "Ctrl+Shift+A"),
         line(Act::Invert, t!("folders-menu-invert-selection"), stand_in::INVERT, "Ctrl+I")]
}

fn copy() -> Line {
    line(Act::Copy, t!("folders-menu-copy"), "edit-copy-symbolic", "Ctrl+C")
}

fn cut() -> Line {
    line(Act::Cut, t!("folders-menu-cut"), "edit-cut-symbolic", "Ctrl+X")
}

fn path() -> Line {
    line(Act::CopyPath, t!("folders-menu-copy-location"), "edit-copy-symbolic", "")
}

fn pin(pinned: bool) -> Line {
    if pinned { line(Act::Pin, t!("folders-menu-unpin"), "view-pin-symbolic", "") } else { line(Act::Pin, t!("folders-menu-pin-to-the-sidebar"), "view-pin-symbolic", "") }
}

fn colour() -> Line {
    line(Act::Colour, t!("folders-menu-folder-colour"), stand_in::FOLDER_COLOUR, "")
}

fn terminal() -> Line {
    line(Act::Terminal, t!("folders-menu-open-in-a-terminal"), "utilities-terminal-symbolic", "F4")
}

fn root() -> Line {
    line(Act::AsRoot, t!("folders-menu-open-as-administrator"), stand_in::AS_ROOT, "")
}

fn shred() -> Line {
    line(Act::Delete, t!("folders-menu-delete-permanently"), stand_in::SHRED, "Shift+Delete")
}

fn trash() -> Line {
    line(Act::Trash, t!("folders-menu-move-to-trash"), "user-trash-symbolic", "Delete")
}

fn dupes() -> Line {
    line(Act::Dupes, t!("folders-menu-find-duplicates"), stand_in::DUPES, "")
}

fn open_with() -> Line {
    line(Act::OpenWith, t!("folders-menu-open-with-another-app"), "view-app-grid-symbolic", "")
}

fn preview() -> Line {
    line(Act::Preview, t!("folders-menu-preview"), stand_in::PREVIEW, "Space")
}

fn make() -> Vec<Line> {
    vec![line(Act::NewFolder, t!("folders-menu-new-folder"), "folder-new-symbolic", "Ctrl+Shift+N"),
         line(Act::NewDocument, t!("folders-menu-new-document"), "document-new-symbolic", "")]
}

fn found_groups(sel: &[Sel]) -> Vec<Vec<Line>> {
    let mut first = vec![open()];
    if sel.len() == 1 {
        if !sel[0].folder {
            first.push(open_with());
            first.push(preview());
        }
        first.push(line(Act::ShowFolder, t!("folders-menu-show-in-its-folder"), "folder-open-symbolic", ""));
    }
    vec![first, vec![cut(), copy(), path()], vec![trash()], vec![props()]]
}

#[derive(Clone, Copy)]
struct Mix {
    one: bool,
    folders: bool,
    files: bool,
}

fn folder_groups(sel: &[Sel], pinned: bool) -> Vec<Vec<Line>> {
    let mix = Mix { one: sel.len() == 1, folders: sel.iter().all(|s| s.folder), files: sel.iter().all(|s| !s.folder) };
    vec![make(), open_group(mix), edit_group(mix), pack_group(sel, mix), more_group(sel, mix, pinned), gone_group(mix), vec![props()]]
}

fn open_group(mix: Mix) -> Vec<Line> {
    let mut first = vec![open()];
    if mix.one && mix.folders {
        first.push(line(Act::OpenTab, t!("folders-menu-open-in-a-new-tab"), "tab-new-symbolic", ""));
        first.push(line(Act::OpenWindow, t!("folders-menu-open-in-a-new-window"), "window-new-symbolic", ""));
    }
    if mix.one && mix.files {
        first.push(open_with());
        first.push(preview());
    }
    first
}

fn edit_group(mix: Mix) -> Vec<Line> {
    let mut edit = vec![cut(), copy(), path()];
    if mix.one {
        edit.push(line(Act::Rename, t!("folders-menu-rename"), "document-edit-symbolic", "F2"));
        edit.push(line(Act::Link, t!("folders-menu-make-a-link"), stand_in::LINK, ""));
    } else {
        edit.push(line(Act::Rename, t!("folders-menu-rename-them"), stand_in::RENAME_MANY, "F2"));
    }
    edit.push(line(Act::SendTo, t!("folders-menu-send-to"), "send-to-symbolic", ""));
    edit
}

fn pack_group(sel: &[Sel], mix: Mix) -> Vec<Line> {
    let mut pack = Vec::new();
    if mix.one && sel[0].archive {
        pack.push(line(Act::ExtractHere, t!("folders-menu-extract-here"), "archive-extract-symbolic", ""));
        pack.push(line(Act::ExtractTo, t!("folders-menu-extract-to-a-folder"), "archive-extract-symbolic", ""));
    } else if !mix.one && sel.iter().all(|s| s.archive) {
        pack.push(line(Act::ExtractTo, t!("folders-menu-extract-each-to-its-own-folder"), "archive-extract-symbolic", ""));
    }
    if !sel.is_empty() && sel.iter().all(|s| s.font) {
        pack.push(line(Act::InstallFont, if sel.len() == 1 { t!("folders-menu-install-the-font-for-you") } else { t!("folders-menu-install-the-fonts-for-you") }, stand_in::FONT, ""));
    }
    pack.push(line(Act::Compress, t!("folders-menu-compress"), "package-x-generic-symbolic", ""));
    pack
}

fn gone_group(mix: Mix) -> Vec<Line> {
    let mut gone = vec![trash(), shred()];
    if mix.files {
        gone.push(line(Act::Wipe, t!("folders-menu-wipe-securely"), stand_in::WIPE, ""));
    }
    gone
}

fn more_group(sel: &[Sel], mix: Mix, pinned: bool) -> Vec<Line> {
    let mut more = Vec::new();
    if mix.one && mix.folders {
        more.extend([pin(pinned), colour(), terminal(), root()]);
        more.push(line(Act::ShareOnNetwork, t!("folders-menu-share-on-the-network"), stand_in::SHARE_FOLDER, ""));
        more.push(line(Act::ShareWithAi, t!("folders-menu-share-with-ai-programs"), "security-high-symbolic", ""));
    } else if mix.folders {
        more.extend([colour(), terminal()]);
    } else {
        more.push(terminal());
    }
    if mix.folders {
        more.push(dupes());
    }
    more.extend(picture_rows(sel, mix));
    if sel.len() == 2 && mix.files {
        more.push(line(Act::CompareFiles, t!("folders-menu-compare-the-two"), stand_in::COMPARE, ""));
    }
    more
}

fn picture_rows(sel: &[Sel], mix: Mix) -> Vec<Line> {
    let mut rows = Vec::new();
    if mix.one && sel[0].image {
        rows.push(line(Act::Wallpaper, t!("folders-menu-set-as-wallpaper"), "preferences-desktop-wallpaper-symbolic", ""));
    }
    if !sel.is_empty() && sel.iter().all(|s| s.turnable) {
        rows.push(line(Act::TurnLeft, t!("folders-menu-turn-left"), stand_in::TURN_LEFT, ""));
        rows.push(line(Act::TurnRight, t!("folders-menu-turn-right"), stand_in::TURN_RIGHT, ""));
        rows.push(line(Act::TurnOver, t!("folders-menu-turn-upside-down"), stand_in::TURN_OVER, ""));
    }
    if !sel.is_empty() && sel.iter().all(|s| s.resizable) {
        rows.push(line(Act::Resize, t!("folders-menu-resize"), stand_in::RESIZE, ""));
    }
    rows
}

pub fn view_lines(view: View, sort: usize, group: usize, up: bool) -> Vec<Line> {
    use crate::files::columns::Col;
    use crate::files::groups::GroupBy;
    let mark = |on: bool| if on { stand_in::TICKED } else { "" };
    let mut out: Vec<Line> = [(View::Icons, t!("folders-view-icons"), "Ctrl+1"), (View::List, t!("folders-view-list"), "Ctrl+2"), (View::Compact, t!("folders-view-compact"), "Ctrl+3")]
        .into_iter().map(|(v, words, keys)| line(Act::Look(v), words, mark(v == view), keys)).collect();
    if view == View::List {
        out.push(line(Act::Columns, t!("folders-view-columns"), stand_in::COLUMNS, ""));
    }
    let mut sorts = vec![t!("folders-view-sort-name")];
    sorts.extend(Col::ALL.iter().map(|c| c.word()));
    let sorts = sorts.into_iter().enumerate().map(|(i, w)| line(Act::SortBy(i), w, mark(i == sort), "")).collect();
    let groups = GroupBy::ALL.iter().enumerate().map(|(i, g)| line(Act::GroupBy(i), g.word(), mark(i == group), "")).collect();
    let way = if up { "view-sort-ascending-symbolic" } else { "view-sort-descending-symbolic" };
    out.push(Line { rule: true, sub: sorts, ..line(Act::Sub(Kind::Sort), t!("folders-view-sort-by"), way, "") });
    out.push(Line { sub: groups, ..line(Act::Sub(Kind::Group), t!("folders-view-group-by"), "", "") });
    out.push(line(Act::Flip, t!("folders-view-sort-other-way"), way, ""));
    out
}

pub fn more_lines(undo: bool, redo: bool) -> Vec<Line> {
    let mut out = Vec::new();
    if undo {
        out.push(line(Act::Undo, t!("folders-view-more-undo"), "edit-undo-symbolic", "Ctrl+Z"));
    }
    if redo {
        out.push(line(Act::Redo, t!("folders-view-more-redo"), "edit-redo-symbolic", "Ctrl+Y"));
    }
    out.push(Line { rule: undo || redo, ..line(Act::Settings, crate::files::settings::name(), stand_in::SETTINGS, "Ctrl+,") });
    out
}

pub fn column_lines(shown: &[crate::files::columns::Col]) -> Vec<Line> {
    use crate::files::columns::Col;
    let first_inside = Col::ALL.iter().position(|c| c.media());
    Col::ALL.iter().enumerate().map(|(i, c)| Line {
        act: Act::Column(*c), words: c.word(), icon: if shown.contains(c) { stand_in::TICKED } else { "" }, keys: "",
        rule: Some(i) == first_inside, sub: Vec::new(),
    }).collect()
}

pub fn drop_lines(items: &[PathBuf], target: &Where) -> Vec<Line> {
    let shut = items.first().is_some_and(|i| crate::files::inside::within(i).is_some());
    let far = items.iter().any(|i| crate::files::far::is_far(i));
    let copy = line(Act::CopyHere, t!("folders-menu-copy-here"), "edit-copy-symbolic", "");
    let moved = line(Act::MoveHere, t!("folders-menu-move-here"), "go-jump-symbolic", "");
    let mut lines = match target {
        Where::Folder(dir) => {
            let mut lines = vec![copy];
            if !shut && items.iter().any(|i| i.parent() != Some(dir.as_path())) {
                lines.push(moved);
            }
            if !shut && !far {
                lines.push(line(Act::LinkHere, t!("folders-menu-make-a-link-here"), stand_in::LINK, ""));
            }
            lines
        }
        Where::Remote(_) if !shut => vec![copy, moved],
        Where::Trash if !shut && !far => vec![line(Act::MoveHere, t!("folders-menu-move-to-trash"), "user-trash-symbolic", "")],
        _ => Vec::new(),
    };
    if !lines.is_empty() {
        lines.push(Line { rule: true, ..line(Act::LeaveThem, t!("folders-menu-cancel"), "window-close-symbolic", "") });
    }
    lines
}

const SIDE: [Act; 9] = [Act::Open, Act::OpenTab, Act::OpenWindow, Act::Terminal, Act::AsRoot, Act::CopyPath, Act::Pin, Act::Colour, Act::Properties];

pub fn side_lines(at: &Where, pinned: bool) -> Vec<Line> {
    let mut lines: Vec<Line> = match at {
        Where::Folder(f) => {
            let parent = Where::Folder(f.parent().unwrap_or(f).to_path_buf());
            let folder = Sel { folder: true, ..Sel::default() };
            lines_for(&parent, &[folder], pinned).into_iter().filter(|l| SIDE.contains(&l.act)).collect()
        }
        Where::Trash => lines_for(&Where::Trash, &[], false).into_iter().filter(|l| l.act == Act::EmptyTrash).collect(),
        _ => Vec::new(),
    };
    if let Some(first) = lines.first_mut() {
        first.rule = false;
    }
    lines
}

pub fn pinned_lines(mut lines: Vec<Line>, row: usize, count: usize) -> Vec<Line> {
    let Some(at) = lines.iter().position(|l| l.act == Act::Pin) else { return lines };
    let mut moves = vec![line(Act::RenamePin, t!("folders-menu-rename"), "document-edit-symbolic", "")];
    if row > 0 {
        moves.push(line(Act::PinUp, t!("folders-menu-move-up"), "go-up-symbolic", ""));
    }
    if row + 1 < count {
        moves.push(line(Act::PinDown, t!("folders-menu-move-down"), "go-down-symbolic", ""));
    }
    lines.splice(at + 1..at + 1, moves);
    lines
}

pub fn with_ring(mut lines: Vec<Line>, on_ring: bool) -> Vec<Line> {
    let row = if on_ring {
        line(Act::RingPin, t!("folders-menu-unpin-from-the-ring"), "starred-symbolic", "")
    } else {
        line(Act::RingPin, t!("folders-menu-pin-to-the-ring"), "non-starred-symbolic", "")
    };
    let at = lines.iter().position(|l| l.act == Act::Pin).map(|i| i + 1)
        .or_else(|| lines.iter().position(|l| l.act == Act::Properties));
    match at {
        Some(at) if lines[at..].first().is_some_and(|l| l.act == Act::Properties && l.rule) => {
            lines[at].rule = false;
            lines.insert(at, Line { rule: true, ..row });
        }
        Some(at) => lines.insert(at, row),
        None => lines.push(row),
    }
    lines
}

pub fn without(lines: Vec<Line>, gone: impl Fn(Act) -> bool) -> Vec<Line> {
    let mut out: Vec<Line> = Vec::new();
    let mut rule = false;
    for l in lines {
        if gone(l.act) {
            rule |= l.rule;
            continue;
        }
        out.push(Line { rule: l.rule || (rule && !out.is_empty()), ..l });
        rule = false;
    }
    if let Some(first) = out.first_mut() {
        first.rule = false;
    }
    out
}
