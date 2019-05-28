use crate::mmm::Command;
use crate::mmm::utils;
use std::fmt::Display;
use std::path::Path;
use std::process;
use termion::color;

pub struct CopyCommand {}

impl CopyCommand {
    fn log_copy_attempt<T>(&self, msg: T) where T: Display {
        utils::log(format!("Copying to {}\n", utils::color_text(msg, color::Yellow)));
    }

    fn copy(&self, from: String, to: String) -> bool {
        let mut editor_command = process::Command::new("cp");
        editor_command.arg("-r");
        editor_command.arg(from);
        editor_command.arg(to);
        let cp_result = editor_command
            .spawn()
            .ok()
            .expect("couldn't launch cp")
            .wait();
        return cp_result.is_ok();
    }
}

impl Command for CopyCommand {
    fn name(&self) -> String { String::from("Copy") }
    fn hotkey_pos(&self) -> usize { 0 }
    fn exe_msg(&self, path: &String) -> Option<String> {
        Some(format!("{} {}\n", "Copying  ", path))
    }

    fn should_show_if_path_exists_not(&self) -> bool { false }

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
            self.log_copy_attempt(new_dest.to_str().unwrap());
            return self.copy(path.to_string(), new_dest.to_str().unwrap().to_string());
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
                    self.log_copy_attempt(&destination);
                    self.copy(path.to_string(), destination)
                }
            }
        }
    }
}
