// Cut, copy and paste, in Folders and from other programs.

use std::fs;
use std::path::{Path, PathBuf};

use crate::files::app::Msg;
use crate::files::clip;
use crate::files::kinds;
use crate::toolkit::ui::App;
use super::*;

#[test]
fn cut_and_paste_moves() {
    let dir = folder("paste");
    fs::write(dir.join("left/a.txt"), b"a").unwrap();
    let mut f = files(&dir);
    pick(&mut f, 0, "a.txt");
    f.update(Msg::Cut);
    f.side = 1;
    f.update(Msg::Paste);
    assert!(!dir.join("left/a.txt").exists() && dir.join("right/a.txt").exists());
    assert!(f.clip.is_none());
    let uri = kinds::file_uri(&dir.join("right/a.txt"));
    assert_eq!(clip::paths_in(&format!("copy\n{uri}\r\n")), [dir.join("right/a.txt")]);
    assert!(clip::paths_in("just some words").is_empty());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_cut_by_another_program_is_pasted_in_folders_as_a_copy_and_folders_own_cut_moves() {
    let dir = folder("whose-cut");
    let note = dir.join("cut-note");
    let window = |dir: &Path| {
        let mut f = files(dir);
        f.cut_note = Some(note.clone());
        f
    };
    let on_clipboard = |paths: &[PathBuf], cut: bool| Msg::Pasted(Some(clip::Pasted { paths: paths.to_vec(), cut }));
    fs::write(dir.join("left/theirs.txt"), b"theirs").unwrap();
    let mut f = window(&dir);
    f.side = 1;
    f.update(on_clipboard(&[dir.join("left/theirs.txt")], true));
    assert!(dir.join("right/theirs.txt").exists(), "nothing was pasted");
    assert!(dir.join("left/theirs.txt").exists(), "what another program called cut was moved");
    assert!(f.ended.iter().any(|e| e.words == cut_elsewhere()) || f.said == cut_elsewhere(), "{:?} / {:?}", f.ended.iter().map(|e| e.words.clone()).collect::<Vec<_>>(), f.said);
    fs::write(dir.join("left/mine.txt"), b"mine").unwrap();
    let mut f = window(&dir);
    pick(&mut f, 0, "mine.txt");
    f.update(Msg::Cut);
    f.side = 1;
    f.update(on_clipboard(&[dir.join("left/mine.txt")], true));
    assert!(!dir.join("left/mine.txt").exists() && dir.join("right/mine.txt").exists(), "Folders' own cut was not moved");
    assert!(!note.exists(), "the note of a cut that has been pasted is kept");
    fs::write(dir.join("left/across.txt"), b"across").unwrap();
    let mut one = window(&dir);
    pick(&mut one, 0, "across.txt");
    one.update(Msg::Cut);
    let mut other = window(&dir);
    other.side = 1;
    other.update(on_clipboard(&[dir.join("left/across.txt")], true));
    assert!(!dir.join("left/across.txt").exists() && dir.join("right/across.txt").exists(), "a cut made in another window of Folders was not moved");
    fs::write(dir.join("left/noted.txt"), b"noted").unwrap();
    fs::write(dir.join("left/other.txt"), b"other").unwrap();
    let mut one = window(&dir);
    pick(&mut one, 0, "noted.txt");
    one.update(Msg::Cut);
    let mut other = window(&dir);
    other.side = 1;
    other.update(on_clipboard(&[dir.join("left/other.txt")], true));
    assert!(dir.join("left/other.txt").exists() && dir.join("right/other.txt").exists());
    pick(&mut one, 0, "other.txt");
    one.update(Msg::CopyClip);
    assert!(!note.exists(), "a copy left the note of the cut before it");
    other.update(on_clipboard(&[dir.join("left/noted.txt")], true));
    assert!(dir.join("left/noted.txt").exists() && dir.join("right/noted.txt").exists());
    clip::note_cut(&note, &[dir.join("left/noted.txt")]);
    assert!(clip::moves(&[dir.join("left/noted.txt")], true, None, Some(&note)));
    assert!(!clip::moves(&[dir.join("left/noted.txt")], true, None, None));
    fs::remove_dir_all(&dir).unwrap();
}
