mod edit_command;

use super::Command;

pub fn all_commands() -> Vec<Box<Command>> {
    return vec![
        Box::new(edit_command::EditCommand {}),
    ];
}
