mod edit_command;
mod open_command;
mod move_command;

use super::Command;

pub fn all_commands() -> Vec<Box<Command>> {
    vec![
        Box::new(edit_command::EditCommand {}),
        Box::new(open_command::OpenCommand {}),
        Box::new(move_command::MoveCommand {}),
    ]
}
