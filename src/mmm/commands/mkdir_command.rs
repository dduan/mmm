use crate::mmm::Command;
use crate::mmm::utils;
use std::fs;
use std::path::Path;

pub struct MkdirCommand {}

impl Command for MkdirCommand {
    fn name(&self) -> String { String::from("Make Directory(s)") }
    fn hotkey_pos(&self) -> usize { 2 }
    fn exe_msg(&self, path: &String) -> Option<String> {
        Some(format!("{} {}\n", "Creating directory(s) at", path))
    }

    fn should_show_if_path_exists(&self) -> bool { false }
    fn should_show_if_path_exists_not(&self) -> bool { true }

    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let dest_path = Path::new(&path);
        if dest_path.exists() { // This shouldn't happen
            return true;
        }

        fs::create_dir_all(path).is_ok()
    }
}
