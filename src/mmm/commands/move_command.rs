use crate::mmm::Command;
use std::fmt::Display;
use crate::mmm::utils;
use std::fs;
use std::path::Path;
use colored::Color;

pub struct MoveCommand {}

impl MoveCommand {
    fn log_move_attempt<T>(&self, msg: T) where T: Display {
        utils::log(format!("Moving to {}\n", utils::color_text(msg, Color::Yellow)));
    }
}

impl Command for MoveCommand {
    fn new() -> MoveCommand { MoveCommand {} }
    fn name(&self) -> String { String::from("Move") }
    fn hotkey_pos(&self) -> usize { 0 }
    fn exe_msg(&self, path: &String) -> Option<String> {
        Some(format!("{} {}\n", "Moving   ", path))
    }

    fn should_show_if_path_exists_not(&self) -> bool { false }

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
            return false
        }

        let dest_path = Path::new(&destination);
        let dest_is_dir = dest_path.is_dir();

        if destination.ends_with(std::path::MAIN_SEPARATOR) && !dest_is_dir {
            utils::elog(format!(
                "I can't move it to {}, that place doesn't exist yet!\n",
                dest_path.to_str().unwrap()));
            return false
        }

        if dest_is_dir {
            let new_dest = dest_path.join(Path::new(path).file_name().unwrap());
            self.log_move_attempt(new_dest.to_str().unwrap());
            return fs::rename(path, new_dest).is_ok()
        }

        match dest_path.parent() {
            None => false,
            Some(parent_path) => {
                if !parent_path.to_str().unwrap().is_empty() && !parent_path.exists() {
                    utils::elog(format!(
                        "I can't move it to {}, that place doesn't exist yet!\n",
                        parent_path.to_str().unwrap()));
                    false
                } else {
                    self.log_move_attempt(&destination);
                    fs::rename(path, destination).is_ok()
                }
            }
        }
    }

    fn wrapup_msg(&self) -> String {
        String::from("Done!\n")
    }
}
