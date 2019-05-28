mod copy_command;
mod delete_command;
mod edit_command;
mod move_command;
mod open_command;
mod touch_command;

use super::Command;

pub fn all_commands() -> Vec<Box<Command>> {
    vec![
        Box::new(edit_command::EditCommand {}),
        Box::new(open_command::OpenCommand {}),
        Box::new(move_command::MoveCommand {}),
        Box::new(copy_command::CopyCommand {}),
        Box::new(delete_command::DeleteCommand {}),
        Box::new(touch_command::TouchCommand {}),
    ]
}
