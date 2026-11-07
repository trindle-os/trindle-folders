// What only the person may press in Folders: Share with AI programs, and the password window. Made-up presses (a plugin's) never reach them.

pub const PASSWORD_WINDOW: &str = "trindle-keyring";

pub const FOLDERS_AI_SHARE: &str = "folders-ai-share";

pub const ACTS: [&str; 1] = [FOLDERS_AI_SHARE];

pub fn asks(name: &str) -> bool {
    name == PASSWORD_WINDOW
}

#[cfg(test)]
mod tests {
    #[test]
    fn share_with_ai_asks_whether_the_press_was_the_persons_own() {
        let menu = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/files/menu/act.rs")).unwrap();
        assert!(menu.contains("if !crate::toolkit::window::persons_own(crate::common::person_only::FOLDERS_AI_SHARE) {"));
    }

    #[test]
    fn the_password_window_is_given_the_name_kept_from_made_up_presses() {
        let prompt = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/common/prompt.rs")).unwrap();
        assert!(prompt.contains(&format!("app_id: \"{}\".into()", super::PASSWORD_WINDOW)));
        assert!(super::asks(super::PASSWORD_WINDOW) && !super::asks("trindle-folders"));
    }
}
