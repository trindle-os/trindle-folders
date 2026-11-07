// Red is for an act that cannot be undone, and for a failure, and for nothing else (the rule for every design).

use super::save_is_a_word::all_lines;

const MARK: &str = ".cannot_be_undone()";

const FOR_GOOD: [(&str, &str, &str); 3] = [
    ("files/view.rs", "main_action(stand_in::SHRED, act).cannot_be_undone()", "Delete permanently, and Empty the Trash: the files are gone"),
    ("files/tools.rs", "t!(\"folders-tools-wipe-button\"), ToolMsg::Wipe", "each file is written over before it goes"),
    ("files/settings.rs", "let forget = button(stand_in::SHRED, t!(\"folders-settings-forget\", server = &r.server)).cannot_be_undone()", "Forget a kept login: the login and the addresses kept with it are let go for good, and the password must be typed again"),
];

const TELLS: [&str; 1] = ["cannot be undone"];

const NO_BUTTON: [(&str, &str); 2] = [
    ("common/no_writing_in_place.rs", "a list for a test, which says what the wipe tells the person"),
    ("common/red_for_good.rs", "this file, which names the words"),
];

#[test]
fn every_red_main_action_is_on_the_list_with_what_cannot_be_undone() {
    let marked: Vec<(String, String)> = all_lines().into_iter()
        .filter(|(file, line)| line.contains(MARK) && !file.starts_with("toolkit/") && file != "common/red_for_good.rs").collect();
    let listed = |file: &str, line: &str| FOR_GOOD.iter().any(|(f, words, _)| *f == file && line.contains(words));
    let unlisted: Vec<_> = marked.iter().filter(|(file, line)| !listed(file, line)).collect();
    assert!(
        unlisted.is_empty(),
        "a main action is marked as one that cannot be undone, and so drawn red, and is not on the list in common/red_for_good.rs. \
         Red is only for an act that cannot be undone: add it there with what that is, or take the mark away: {unlisted:#?}"
    );
    for (file, words, what) in FOR_GOOD {
        let lines: Vec<_> = marked.iter().filter(|(f, line)| f == file && line.contains(words)).collect();
        assert_eq!(lines.len(), 1, "{file}: the main action for \"{what}\" is no longer marked on a line with {words}");
    }
}

#[test]
fn every_file_that_says_an_act_cannot_be_undone_marks_its_main_action() {
    let all = all_lines();
    let mut wrong = Vec::new();
    let mut says: Vec<&str> = all.iter().filter(|(_, line)| TELLS.iter().any(|t| line.to_lowercase().contains(t))).map(|(f, _)| f.as_str()).collect();
    says.dedup();
    for file in says {
        let marks = all.iter().any(|(f, line)| f == file && line.contains(MARK));
        let excused = NO_BUTTON.iter().any(|(f, _)| *f == file);
        if !marks && !excused {
            wrong.push(file);
        }
    }
    assert!(wrong.is_empty(), "these files tell a person that something cannot be undone, and no main action in them is marked with {MARK}, so the button is the highlight and not red: {wrong:?}");
    for must in ["folders-view.ftl", "folders-tools.ftl"] {
        let words = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang/en").join(must)).unwrap_or_default();
        assert!(TELLS.iter().any(|t| words.to_lowercase().contains(t)), "Folders' file {must} no longer tells a person that something cannot be undone");
    }
}
