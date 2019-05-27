use crate::mmm::Command;
use open;
use std::path::Path;

pub struct OpenCommand {}

impl Command for OpenCommand {
    fn name(&self) -> String { String::from("Open") }
    fn hotkey_pos(&self) -> usize { 0 }
    fn exe_msg(&self, path: &String) -> Option<String> { Some(format!("{} {}\n", "Opening", path)) }

    fn should_show(&self, path: &String) -> bool { Path::new(path).exists() }

    #[allow(unused_variables)]
    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let result = open::that(path);
        return result.is_ok() && result.unwrap().success();
    }
}
