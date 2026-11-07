// For the tests that read Folders' own source: the files under `src`, by the names they have there, and the file work it shares from the toolkit, held to the same rules.

pub use crate::toolkit::common::source::code;

pub fn code_all(texts: &[&'static str]) -> &'static str {
    Box::leak(texts.iter().map(|t| code(t)).collect::<String>().into_boxed_str())
}

pub fn files_in(folder: &str) -> usize {
    std::fs::read_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join(folder)).map_or(0, |d| d.flatten().filter(|e| e.path().extension().is_some_and(|x| x == "rs")).count())
}

pub fn trees() -> Vec<(std::path::PathBuf, &'static str)> {
    let toolkit = crate::toolkit::common::source::toolkit_root().join("src");
    vec![
        (std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), ""),
        (toolkit.join("common/file_work"), "toolkit/common/file_work/"),
        (toolkit.join("sys"), "toolkit/sys/"),
    ]
}
