use colored::Color;

pub mod utils;
pub mod commands;

pub trait Command {
    fn name(&self) -> String;
    fn hotkey_pos(&self) -> usize;
    fn exe_msg(&self, path: &String) -> Option<String>;

    fn need_followup(&self) -> bool { false }
    #[allow(unused_variables)]
    fn followup_prompt(&self, path: &String) -> String { String::from("") }

    fn need_wrapup(&self) -> bool { true }
    #[allow(unused_variables)]
    fn wrapup_msg(&self) -> String { String::from("Done!\n") }

    fn should_show_if_path_exists(&self) -> bool { true }
    fn should_show_if_path_exists_not(&self) -> bool { true }

    fn execute(&self, path: &String, followup_input: Option<String>) -> bool;

    fn display_text(&self) -> String {
        let mut name = self.name();
        let pos = self.hotkey_pos();

        let key_char = name.chars().nth(pos).unwrap();
        let indicator = format!("[{}]", utils::color_text(key_char, Color::Red));

        name.replace_range(pos..pos+1, &indicator);
        if self.need_followup() {
            name.push('…');
        }

        return name
    }

    fn matches_hotkey(&self, key: char) -> bool {
        let hotkey = self.name().chars().nth(self.hotkey_pos()).unwrap();
        return key.to_ascii_uppercase() == hotkey || key.to_ascii_lowercase() == hotkey
    }
}
