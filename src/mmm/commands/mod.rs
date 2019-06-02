use super::utils;
use std::io::Write;
use termcolor::{
    Buffer,
    BufferWriter,
    Color,
    ColorChoice,
};

pub trait Command {
    fn new() -> Self where Self: Sized;

    fn name(&self) -> String;
    fn hotkey_pos(&self) -> usize;

    fn exe_msg(&self, path: &String) -> Option<String>;
    fn need_followup(&self) -> bool { false }
    #[allow(unused_variables)]
    fn followup_prompt(&self, path: &String) -> Buffer {
        BufferWriter::stdout(ColorChoice::Auto).buffer()
    }

    fn need_wrapup(&self) -> bool { true }
    #[allow(unused_variables)]
    fn wrapup_msg(&self) -> String { String::from("Done!\n") }

    fn should_show_if_path_exists(&self) -> bool { true }
    fn should_show_if_path_exists_not(&self) -> bool { true }

    fn execute(&self, path: &String, followup_input: Option<String>) -> bool;

    fn display_text(&self) -> (Buffer, usize) {
        let mut name = self.name();
        if self.need_followup() {
            name.push('…');
        }
        let name_len = name.len() + 2;

        let pos = self.hotkey_pos();
        let key = name.chars().nth(pos).unwrap();

        let mut name_tail: String = name.split_off(pos);
        let _ = name_tail.remove(0);

        let mut buffer = BufferWriter::stdout(ColorChoice::Auto).buffer();

        write!(&mut buffer, "{}[", name).expect("Buffer write failure");
        utils::write(&mut buffer, key, Color::Red);
        write!(&mut buffer, "]{}", name_tail).expect("Buffer write failure");

        (buffer, name_len)
    }

    fn matches_hotkey(&self, key: char) -> bool {
        let hotkey = self.name().chars().nth(self.hotkey_pos()).unwrap();
        key.to_ascii_uppercase() == hotkey || key.to_ascii_lowercase() == hotkey
    }
}

mod copy_command;
mod delete_command;
mod edit_command;
mod git_command;
mod mkdir_command;
mod move_command;
mod open_command;
mod touch_command;

pub fn all_commands() -> Vec<Box<Command>> {
    vec![
        Box::new(edit_command::EditCommand::new()),
        Box::new(open_command::OpenCommand::new()),
        Box::new(move_command::MoveCommand::new()),
        Box::new(copy_command::CopyCommand::new()),
        Box::new(delete_command::DeleteCommand::new()),
        Box::new(touch_command::TouchCommand::new()),
        Box::new(mkdir_command::MkdirCommand::new()),
        Box::new(git_command::GitCommand::new()),
    ]
}
