//! Hidden file endings in the list and in the rename box.

use super::*;

fn drawn(b: &Bench, p: usize, name: &str) -> String {
    b.f.shown_name(&b.f.cols[p].entries[index(&b.f, p, name)]).to_string()
}

fn endings_cases(b: &mut Bench, dir: &Path, p: usize, what: &str) {
    endings_are_hidden(b, dir, p, what);
    endings_are_kept_in_the_box(b, dir, p, what);
}

fn endings_are_hidden(b: &mut Bench, dir: &Path, p: usize, what: &str) {
    use std::os::unix::fs::PermissionsExt;
    let here = side_dir(dir, p);
    fs::write(here.join("letter.txt"), b"made up").unwrap();
    fs::write(here.join("plan.pdf"), b"%PDF-1.7 made up").unwrap();
    fs::write(here.join("tool.txt"), b"made up").unwrap();
    fs::set_permissions(here.join("tool.txt"), fs::Permissions::from_mode(0o755)).unwrap();
    b.f.side = p;
    b.f.update(Msg::Reload);
    b.f.endings = false;
    b.show();
    assert_eq!(drawn(b, p, "letter.txt"), "letter", "{what}");
    assert_eq!(drawn(b, p, "plan.pdf"), "plan", "{what}");
    assert_eq!(drawn(b, p, "archive.tar.gz"), "archive.tar.gz", "{what}: two endings show in full");
    assert_eq!(drawn(b, p, "tool.txt"), "tool.txt", "{what}: something that can be run shows in full");
    assert_eq!(drawn(b, p, "report.pdf"), "report.pdf", "{what}: not the PDF its ending says");
    assert_eq!(drawn(b, p, "Albums"), "Albums", "{what}");
    b.f.ahead = Default::default();
    b.f.typed_at("letter.t", Instant::now());
    assert_eq!(chosen(&b.f, p).as_deref(), Some("letter.txt"), "{what}");
    b.f.update(Msg::Find(crate::files::found::FindMsg::Open(Some(p))));
    b.f.update(Msg::Find(crate::files::found::FindMsg::Text("letter.txt".into())));
    assert_eq!(names(&b.f, p), ["letter.txt"], "{what}");
    b.f.update(Msg::Find(crate::files::found::FindMsg::Close));
    b.show();
}

fn endings_are_kept_in_the_box(b: &mut Bench, dir: &Path, p: usize, what: &str) {
    let here = side_dir(dir, p);
    b.pick(p, "letter.txt");
    let cell = b.cell(p, "letter.txt");
    b.key(Keysym::F2);
    let r = b.name_box().unwrap_or_else(|| panic!("{what}: the box is in the list"));
    assert!(r.x >= cell.x && r.y >= cell.y && r.y + r.h <= cell.y + cell.h + 0.5, "{what}: the box {r:?} is in its row {cell:?}");
    assert_eq!(b.typed(), ("letter".to_string(), 0.."letter".len()), "{what}");
    b.type_in("memo");
    assert_eq!(b.typed().0, "memo", "{what}");
    b.key(Keysym::Return);
    assert!(here.join("memo.txt").is_file() && !here.join("letter.txt").exists() && !here.join("memo").exists(), "{what}: the ending is kept");
    assert_eq!(chosen(&b.f, p).as_deref(), Some("memo.txt"), "{what}");
    b.f.update(Msg::Undo);
    b.show();
    assert!(here.join("letter.txt").is_file(), "{what}: Undo");
    b.pick(p, "letter.txt");
    b.key(Keysym::F2);
    b.f.update(Msg::RenameText(String::new()));
    b.f.update(Msg::RenameDone);
    b.show();
    assert_eq!(b.f.renaming.as_ref().and_then(|r| r.why.as_deref()), Some("Type a name."), "{what}");
    assert!(here.join("letter.txt").is_file() && !here.join(".txt").exists(), "{what}");
    b.key(Keysym::Escape);
    b.pick(p, "archive.tar.gz");
    b.key(Keysym::F2);
    assert_eq!(b.typed(), ("archive.tar.gz".to_string(), 0.."archive.tar".len()), "{what}");
    b.key(Keysym::Escape);
    b.f.endings = true;
    b.show();
    assert_eq!(drawn(b, p, "letter.txt"), "letter.txt", "{what}");
    b.pick(p, "letter.txt");
    b.key(Keysym::F2);
    assert_eq!(b.typed(), ("letter.txt".to_string(), 0.."letter".len()), "{what}");
    b.key(Keysym::Escape);
    for made in ["letter.txt", "plan.pdf", "tool.txt"] {
        fs::remove_file(here.join(made)).unwrap();
    }
    b.f.update(Msg::Reload);
    b.show();
}

#[test]
fn hidden_endings_are_drawn_and_renamed_without_them_in_every_view() {
    every_pane_and_tab("endings", endings_cases);
}
