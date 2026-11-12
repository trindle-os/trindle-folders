//! The separate naming row is gone: the sources no longer hold its old words.

#[test]
fn the_separate_naming_row_is_gone() {
    for (file, source) in [("view.rs", crate::common::source::code(include_str!("../../view.rs"))), ("act", crate::common::source::code_all(crate::files::act::SOURCE)), ("app.rs", crate::common::source::code(crate::files::app::SOURCE))] {
        for old in ["Keep the old name", ".key(\"rename\")", "TICK_ICON, \"Rename\""] {
            assert!(!source.contains(old), "{file} still has {old}");
        }
    }
    let view = crate::common::source::code(include_str!("../../view.rs"));
    assert_eq!(view.matches("Msg::RenameText").count(), 1);
    assert!(view.contains("typing_over("));
}
