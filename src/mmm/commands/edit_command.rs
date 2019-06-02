use std::env;
use std::process;
use super::Command;

pub struct EditCommand {
    editor: Option<String>
}

impl Command for EditCommand {
    fn new() -> EditCommand {
        EditCommand {
            editor: env::var("EDITOR").ok()
        }
    }

    fn should_show_if_path_exists(&self) -> bool { self.editor.is_some() }
    fn should_show_if_path_exists_not(&self) -> bool { self.editor.is_some() }

    fn name(&self) -> String { String::from("Edit") }
    fn hotkey_pos(&self) -> usize { 0 }
    fn exe_msg(&self, path: &String) -> Option<String> { Some(format!("{} {}\n", "Editing", path)) }

    #[allow(unused_variables)]
    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let mut editor_command = process::Command::new(format!("{}", self.editor.as_ref().unwrap()));
        editor_command.arg(path);
        editor_command
            .spawn()
            .ok()
            .expect("Couldn't launch editor")
            .wait()
            .expect("Editor exit with a failure");
        true
    }
}
