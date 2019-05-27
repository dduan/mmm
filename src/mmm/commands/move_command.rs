use crate::mmm::Command;
use crate::mmm::utils;
use std::fs;
use std::path::Path;

pub struct MoveCommand {}

impl Command for MoveCommand {
    fn name(&self) -> String { String::from("Move") }
    fn hotkey_pos(&self) -> usize { 0 }
    fn exe_msg(&self, path: &String) -> Option<String> {
        Some(format!("{} {}\n", "Moving", path))
    }

    fn should_show(&self, path: &String) -> bool { Path::new(path).exists() }

    #[allow(unused_variables)]
    fn need_followup(&self) -> bool { true }

    #[allow(unused_variables)]
    fn followup_prompt(&self, path: &String) -> String {
        // TODO: can we prefill the answer with `path`, since usually it's related?
        String::from("Where to? ")
    }

    #[allow(unused_variables)]
    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let destination = String::from(followup_input.unwrap_or(String::from("")).trim_end());
        if destination.is_empty() {
            utils::elog("You didn't provide a destination ¯\\_(ツ)_/¯\n");
        }

        let dest_path = Path::new(&destination);
        match dest_path.parent() {
            None => false,
            Some(parent_path) => {
                if !parent_path.exists() {
                    utils::elog(format!("I can't move it to {}\n", parent_path.to_str().unwrap()));
                    utils::elog("that place doesn't exist yet!\n");
                    false
                } else {
                    fs::rename(path, destination).is_ok()
                }
            }
        }
    }
}
