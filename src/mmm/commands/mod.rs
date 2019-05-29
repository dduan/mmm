mod copy_command;
mod delete_command;
mod edit_command;
mod mkdir_command;
mod move_command;
mod open_command;
mod touch_command;

use super::Command;

pub fn all_commands() -> Vec<Box<Command>> {
    vec![
        Box::new(edit_command::EditCommand::new()),
        Box::new(open_command::OpenCommand::new()),
        Box::new(move_command::MoveCommand::new()),
        Box::new(copy_command::CopyCommand::new()),
        Box::new(delete_command::DeleteCommand::new()),
        Box::new(touch_command::TouchCommand::new()),
        Box::new(mkdir_command::MkdirCommand::new()),
    ]
}
