use crate::mmm::Command;
use crate::mmm::utils;
use std::env;
use std::process;

pub struct EditCommand {}

impl Command for EditCommand {
    fn name(&self) -> String { String::from("Edit") }
    fn hotkey_pos(&self) -> usize { 0 }
    fn exe_msg(&self, path: &String) -> Option<String> { Some(format!("{} {}\n", "Editing", path)) }

    #[allow(unused_variables)]
    fn should_show(&self, path: &String) -> bool { true }

    #[allow(unused_variables)]
    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let editor = env::var("EDITOR");
        if editor.is_err() {
            utils::elog("$EDITOR not set\n");
            return false;
        }

        let mut editor_command = process::Command::new(format!("{}", editor.unwrap()));
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
