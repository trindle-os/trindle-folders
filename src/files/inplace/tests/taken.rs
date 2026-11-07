// A name taken by the other kind says which is there.

use super::*;

#[test]
fn a_name_taken_by_the_other_kind_says_which_is_there() {
    let dir = folder("kinds");
    let here = dir.join("left");
    fs::write(here.join("ABC"), b"made up").unwrap();
    fs::create_dir(here.join("XYZ")).unwrap();
    let file_taken = "A file called ABC is already here.";
    let folder_taken = "A folder called XYZ is already here.";
    assert_eq!(rename_here(&here.join("Albums"), "ABC"), Err(file_taken.to_string()));
    assert_eq!(rename_here(&here.join("notes.txt"), "XYZ"), Err(folder_taken.to_string()));
    assert_eq!(rename_here(&here.join("report.pdf"), "ABC"), Err(file_taken.to_string()));
    assert_eq!(rename_here(&here.join("Letters"), "XYZ"), Err(folder_taken.to_string()));
    assert_eq!(rename_here(&here.join("Albums"), "ABC"), Err(file_taken.to_string()));
    assert_eq!(rename_here(&here.join("notes.txt"), "XYZ"), Err(folder_taken.to_string()));
    assert!(here.join("Albums").is_dir() && here.join("notes.txt").is_file(), "nothing changed");
    for view in VIEWS {
        let (dir, mut b) = bench(&format!("kinds-box-{view:?}"), view);
        for p in [0, 1] {
            let here = side_dir(&dir, p);
            fs::write(here.join("ABC"), b"made up").unwrap();
            fs::create_dir(here.join("XYZ")).unwrap();
            b.f.update(Msg::Reload);
            b.show();
            for (item, typed, why) in [("Albums", "ABC", file_taken), ("notes.txt", "XYZ", folder_taken)] {
                b.pick(p, item);
                b.key(Keysym::F2);
                b.key_with(Keysym::a, select_all());
                b.f.update(Msg::RenameText(typed.to_string()));
                b.show();
                b.key(Keysym::Return);
                let r = b.f.renaming.as_ref().unwrap_or_else(|| panic!("{view:?} {p}: the box stays open"));
                assert_eq!(r.why.as_deref(), Some(why), "{view:?} {p}");
                assert!(b.f.bad && b.f.said == why, "{view:?} {p}: also in the bottom line");
                b.key(Keysym::Escape);
            }
            for made in ["ABC", "XYZ"] {
                let _ = fs::remove_file(here.join(made)).or_else(|_| fs::remove_dir(here.join(made)));
            }
            b.f.update(Msg::Reload);
            b.show();
        }
        fs::remove_dir_all(&dir).unwrap();
    }
    let (made, _) = (crate::files::act::make_new(&here, true).unwrap(), 0);
    let again = crate::files::act::make_new(&here, true).unwrap();
    let doc = crate::files::act::make_new(&here, false).unwrap();
    assert!(made.is_dir() && again.is_dir() && doc.is_file() && made != again, "{made:?} {again:?} {doc:?}");
    fs::remove_dir_all(&dir).unwrap();
}
