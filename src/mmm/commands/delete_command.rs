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

pub struct DeleteCommand {}

impl Command for DeleteCommand {
    fn new() -> DeleteCommand { DeleteCommand {} }
    fn name(&self) -> String { String::from("Delete") }
    fn hotkey_pos(&self) -> usize { 0 }
    fn exe_msg(&self, path: &String) -> Option<String> {
        let is_dir = Path::new(path).is_dir();
        if is_dir {
            Some(format!("Deleting {} and its content\n", path))
        } else {
            Some(format!("Deleting {}\n", path))
        }
    }

    fn should_show_if_path_exists_not(&self) -> bool { false }

    fn need_followup(&self) -> bool { true }

    #[allow(unused_variables)]
    fn followup_prompt(&self, path: &String) -> Buffer {
        let mut buffer = BufferWriter::stdout(ColorChoice::Auto).buffer();
        write!(&mut buffer, "Are you ").expect("Buffer write erroc");
        utils::write(&mut buffer, "sure", Color::Red);
        write!(&mut buffer, "? (y/N) ").expect("Buffer write error");
        buffer
    }

    fn need_wrapup(&self) -> bool { false }

    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let input = followup_input.unwrap_or_default().trim_end().to_ascii_lowercase();
        if input != "y" && input != "yes" {
            utils::slog("Ok, standing down.\n");
            return true
        }

        let result: bool;
        if Path::new(path).is_dir() {
            result = fs::remove_dir_all(path).is_ok()
        } else {
            result = fs::remove_file(path).is_ok()
        }

        if result {
            utils::slog("It's gone.\n");
        }

        result
    }
}
