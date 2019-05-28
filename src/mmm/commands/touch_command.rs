use crate::mmm::Command;
use crate::mmm::utils;
use std::fs::File;
use std::path::Path;

pub struct TouchCommand {}

impl Command for TouchCommand {
    fn name(&self) -> String { String::from("Touch") }
    fn hotkey_pos(&self) -> usize { 0 }
    fn exe_msg(&self, path: &String) -> Option<String> {
        Some(format!("{} {}\n", "Creating empty file at", path))
    }

    fn should_show_if_path_exists(&self) -> bool { false }
    fn should_show_if_path_exists_not(&self) -> bool { true }

    #[allow(unused_variables)]
    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let dest_path = Path::new(&path);
        if dest_path.exists() { // This shouldn't happen
            return true;
        }

        match dest_path.parent() {
            None => false,
            Some(parent_path) => {
                if !parent_path.exists() {
                    utils::elog(format!(
                        "I can't create file at {}, that place doesn't exist yet!\n",
                        parent_path.to_str().unwrap()));
                    false
                } else {
                    File::create(path).is_ok()
                }
            }
        }
    }
}
