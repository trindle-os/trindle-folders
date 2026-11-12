//! No tick glyph and no green state dot: a yes is a word, an accept pill

#[cfg(test)]
pub const GO_ICON: &str = "go-next-symbolic";
pub const MARK_ICON: &str = "media-record-symbolic";

#[cfg(test)]
mod tests {
    use super::*;

    fn banned() -> Vec<String> {
        vec![
            ["object", "select"].join("-"),
            ["TICK", "ICON"].join("_"),
            ["TICK", "SECONDS"].join("_"),
            ["Tone", "Good"].join("::"),
            ["theme", "good()"].join("::"),
        ]
    }

    const ALLOWED: [(&str, &str); 2] = [
        ("common/app_theme.rs", "solid(theme::good())"),
        ("settings/mod.rs", "let status = [theme::good()"),
    ];

    #[test]
    fn no_tick_glyph_or_green_state_dot_is_drawn() {
        let found: Vec<_> = super::super::save_is_a_word::all_lines().into_iter()
            .filter(|(file, line)| file != "common/no_tick.rs" && !file.starts_with("toolkit/") && banned().iter().any(|b| line.contains(b.as_str()))
                && !ALLOWED.iter().any(|(f, w)| file.ends_with(f) && line.contains(w)))
            .collect();
        assert!(found.is_empty(), "a tick or a green state dot is drawn: {found:#?}");
    }

    #[test]
    fn the_marks_are_icons_the_sets_have() {
        for icon in [GO_ICON, MARK_ICON] {
            assert!(icon.ends_with("-symbolic"));
        }
    }
}
