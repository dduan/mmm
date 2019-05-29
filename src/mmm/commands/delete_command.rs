use crate::mmm::Command;
use crate::mmm::utils;
use std::path::Path;
use std::process;
use colored::Color;

pub struct DeleteCommand {}

impl DeleteCommand {
    fn delete(&self, path: &String) -> bool {
        let mut editor_command = process::Command::new("rm");
        editor_command.arg("-rf");
        editor_command.arg(path);
        let cp_result = editor_command
            .spawn()
            .ok()
            .expect("couldn't launch rm")
            .wait();
        return cp_result.is_ok();
    }
}

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
    fn followup_prompt(&self, path: &String) -> String {
        format!("Are you {}? (y/N) ", utils::color_text("sure", Color::Red))
    }

    fn need_wrapup(&self) -> bool { false }

    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let input = followup_input.unwrap_or_default().trim_end().to_ascii_lowercase();
        if input != "y" && input != "yes" {
            utils::slog("Ok, standing down.\n");
            return true
        }

        let result = self.delete(path);
        if result {
            utils::slog("It's gone.\n");
        }

        result
    }
}
