use std::fmt::Display;
use std::fs;
use std::io::Write;
use std::path::Path;
use super::Command;
use super::utils;
use termcolor::{
    Buffer,
    BufferWriter,
    Color,
    ColorChoice,
};

pub struct MoveCommand {}

impl MoveCommand {
    fn log_move_attempt<T>(&self, msg: T) where T: Display {
        let stdout = BufferWriter::stdout(ColorChoice::Auto);
        let mut buffer = stdout.buffer();
        writeln!(buffer, "Moving to ").expect("Buffer write error");
        utils::write(&mut buffer, msg, Color::Yellow);
        stdout.print(&buffer).expect("Stdout print error");
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
    fn followup_prompt(&self, path: &String) -> Buffer {
        let mut buffer = BufferWriter::stdout(ColorChoice::Auto).buffer();
        // TODO: can we prefill the answer with `path`, since usually it's related?
        write!(&mut buffer, "Where to? ").expect("Buffer write error");
        buffer
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
