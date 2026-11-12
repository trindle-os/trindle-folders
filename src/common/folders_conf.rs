//! Folders' own settings: they are in Folders itself and not in the

use std::path::{Path, PathBuf};

use super::files::{config_path, read_env, read_small, trindle_conf, update_env};

pub const FILE: &str = "folders.conf";

pub const START: &str = "start";
pub const LEFT: &str = "left";
pub const RIGHT: &str = "right";
pub const ENDINGS: &str = "endings";
pub const TICKS: &str = "ticks";
pub const OPEN_DRIVES: &str = "open_drives";
pub const RECONNECT: &str = "reconnect_shares";
pub const DENSITY: &str = "density";

const CARRIED: [(&str, &str); 6] = [
    (START, "files_start"),
    (LEFT, "files_left"),
    (RIGHT, "files_right"),
    (ENDINGS, "files_endings"),
    (TICKS, "files_ticks"),
    (OPEN_DRIVES, "open_drives_when_plugged"),
];

pub fn usual(key: &str) -> &'static str {
    match key {
        LEFT => "~",
        RIGHT => "recent:",
        ENDINGS => "yes",
        TICKS | OPEN_DRIVES | RECONNECT => "no",
        DENSITY => "compact",
        _ => "",
    }
}

pub fn file() -> PathBuf {
    config_path(FILE)
}

pub fn get_in(file: &Path, key: &str) -> String {
    read_env(file).get(key).map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
        .unwrap_or_else(|| usual(key).to_string())
}

pub fn unreadable(file: &Path) -> Option<String> {
    read_small(file).err().map(|e| e.to_string())
}

pub fn set_in(file: &Path, key: &str, value: &str) -> std::io::Result<()> {
    update_env(file, &[(key, Some(value))])
}

pub fn carry_over(old: &Path, new: &Path) -> std::io::Result<()> {
    let was = read_env(old);
    if !CARRIED.iter().any(|(_, o)| was.contains_key(*o)) {
        return Ok(());
    }
    let now = read_env(new);
    let fresh: Vec<(&str, Option<&str>)> = CARRIED.iter()
        .filter(|(n, _)| !now.contains_key(*n))
        .filter_map(|(n, o)| Some((*n, Some(was.get(*o)?.as_str()))))
        .collect();
    if !fresh.is_empty() {
        update_env(new, &fresh)?;
    }
    let gone: Vec<(&str, Option<&str>)> = CARRIED.iter().map(|(_, o)| (*o, None)).collect();
    update_env(old, &gone)
}

#[cfg_attr(test, allow(dead_code))]
pub fn ready() -> PathBuf {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = carry_over(&trindle_conf(), &file());
    });
    file()
}

pub fn get(key: &str) -> String {
    #[cfg(test)]
    return usual(key).to_string();
    #[cfg(not(test))]
    get_in(&ready(), key)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-folders-conf-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn with_no_file_every_setting_has_its_usual_answer() {
        let dir = scratch("usual");
        let file = dir.join(FILE);
        assert_eq!(get_in(&file, START), "");
        assert_eq!((get_in(&file, LEFT), get_in(&file, RIGHT)), ("~".to_string(), "recent:".to_string()));
        assert_eq!((get_in(&file, ENDINGS), get_in(&file, TICKS), get_in(&file, OPEN_DRIVES)), ("yes".to_string(), "no".to_string(), "no".to_string()));
        assert_eq!(get_in(&file, DENSITY), "compact", "new and existing people alike start on Compact");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_setting_is_kept_in_folders_own_file_and_read_back() {
        let dir = scratch("kept");
        let file = dir.join(FILE);
        set_in(&file, TICKS, "yes").unwrap();
        set_in(&file, DENSITY, "roomy").unwrap();
        set_in(&file, LEFT, "~/Work").unwrap();
        assert_eq!((get_in(&file, TICKS), get_in(&file, DENSITY), get_in(&file, LEFT)), ("yes".to_string(), "roomy".to_string(), "~/Work".to_string()));
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "ticks=yes\ndensity=roomy\nleft=~/Work\n");
        set_in(&file, LEFT, "").unwrap();
        assert_eq!(get_in(&file, LEFT), "~");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_file_that_cannot_be_read_keeps_every_setting_and_a_line_break_is_refused() {
        let dir = scratch("honest");
        let file = dir.join(FILE);
        let was = b"density=roomy\nleft=~/Work\nright=trash:\nstart=chosen\n# \xff\n".to_vec();
        std::fs::write(&file, &was).unwrap();
        assert!(unreadable(&file).is_some_and(|why| why.contains("cannot be read")));
        assert!(set_in(&file, TICKS, "yes").is_err());
        assert_eq!(std::fs::read(&file).unwrap(), was, "Density, Left, Right and Start are still there");
        assert_eq!(get_in(&file, DENSITY), "compact");
        let old = dir.join("trindle.conf");
        std::fs::write(&old, "files_ticks=yes\ntheme=dark\n").unwrap();
        assert!(carry_over(&old, &file).is_err());
        assert_eq!(std::fs::read(&file).unwrap(), was);
        assert_eq!(std::fs::read_to_string(&old).unwrap(), "files_ticks=yes\ntheme=dark\n");
        std::fs::write(&file, "density=roomy\nleft=~/Work\n").unwrap();
        assert!(unreadable(&file).is_none() && unreadable(&dir.join("none.conf")).is_none());
        assert!(set_in(&file, LEFT, "~/Work\nstart=last").is_err());
        assert!(set_in(&file, LEFT, "~/Work\rticks=yes").is_err());
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "density=roomy\nleft=~/Work\n");
        assert_eq!((get_in(&file, START), get_in(&file, TICKS)), (String::new(), "no".to_string()));
        set_in(&file, TICKS, "yes").unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "density=roomy\nleft=~/Work\nticks=yes\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn what_was_chosen_in_settings_is_carried_over_once_and_the_old_lines_go() {
        let dir = scratch("carry");
        let (old, new) = (dir.join("trindle.conf"), dir.join(FILE));
        std::fs::write(&old, "# made up\ntheme=dark\nfiles_start=chosen\nfiles_left=~/Documents\nfiles_right=trash:\nfiles_endings=no\nfiles_ticks=yes\nopen_drives_when_plugged=yes\naccent=#fbc711\n").unwrap();
        carry_over(&old, &new).unwrap();
        for (key, want) in [(START, "chosen"), (LEFT, "~/Documents"), (RIGHT, "trash:"), (ENDINGS, "no"), (TICKS, "yes"), (OPEN_DRIVES, "yes")] {
            assert_eq!(get_in(&new, key), want, "{key}");
        }
        assert_eq!(std::fs::read_to_string(&old).unwrap(), "# made up\ntheme=dark\naccent=#fbc711\n");
        assert_eq!(get_in(&new, DENSITY), "compact");
        set_in(&new, TICKS, "no").unwrap();
        let before = std::fs::read_to_string(&new).unwrap();
        carry_over(&old, &new).unwrap();
        assert_eq!(std::fs::read_to_string(&new).unwrap(), before);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_choice_made_in_folders_beats_an_old_line_that_turns_up_later() {
        let dir = scratch("later");
        let (old, new) = (dir.join("trindle.conf"), dir.join(FILE));
        std::fs::write(&new, "endings=yes\n").unwrap();
        std::fs::write(&old, "files_endings=no\nfiles_ticks=yes\n").unwrap();
        carry_over(&old, &new).unwrap();
        assert_eq!((get_in(&new, ENDINGS), get_in(&new, TICKS)), ("yes".to_string(), "yes".to_string()));
        assert_eq!(std::fs::read_to_string(&old).unwrap(), "\n");
        let (a, b) = (dir.join("none.conf"), dir.join("none-folders.conf"));
        carry_over(&a, &b).unwrap();
        assert!(!a.exists() && !b.exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
