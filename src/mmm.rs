pub mod commands;
pub mod utils;

use commands::Command;
use atty;
use core::iter::FromIterator;
use getch;
use std::io::Write;
use std::io;
use std::path::Path;
use std::time;
use colored::Color;

/// Returns content of the menu and the next line to display after an item is selected from the
/// menu.
fn create_initial_menu(commands: &Vec<Box<Command>>) -> (String, String) {
    let display = commands
        .iter()
        .map(|c| c.display_text());

    // TODO: can we get away with no clone?
    let color_text_count: usize = display.clone().map(|t| t.1).sum();
    let items: Vec<String> = display.map(|t| t.0).collect();
    let initial_text = items.join(" | ");

    let replace_text: String;
    if atty::is(atty::Stream::Stdout) {
        replace_text = format!("\r{}\r", " ".repeat(initial_text.len() - color_text_count));
    } else {
        replace_text = format!("\n");
    }
    return (initial_text, replace_text)
}

fn run_command(command: Box<Command>, path: &String) {
    let message = command.exe_msg(&path);
    if message.is_some() {
        utils::log(message.unwrap());
    }

    let mut followup_input = None;
    if command.need_followup() {
        let followup_message = command.followup_prompt(&path);
        utils::log(followup_message);
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("");
        followup_input = Some(buf);
    }

    let start_instant = time::Instant::now();
    if command.execute(&path, followup_input) {
        let exe_duration = start_instant.elapsed();
        if command.need_wrapup() {
            utils::slog(command.wrapup_msg());
        }

        utils::slog(format!("That took {}.{}s", exe_duration.as_secs(), exe_duration.subsec_millis()));
    } else {
        utils::elog(":(\n");
    }
}

fn initial_greeting(path: &String, path_exists: bool) -> String {
    if path_exists {
        format!("What would you like to do to {}?\n",
                utils::color_text(path, Color::Yellow))
    } else {
        format!("{} doesn't exists yet, what's next?\n",
                utils::color_text(path, Color::Yellow))
    }

}

fn commands_to_show(path_exists: bool) -> Vec<Box<Command>> {
    Vec::from_iter(
        commands::all_commands()
            .into_iter()
            .filter(|c| {
                path_exists && c.should_show_if_path_exists() ||
                    !path_exists && c.should_show_if_path_exists_not()
            })
        )
}

fn user_select_from_menu(commands: &Vec<Box<Command>>) -> char {
    let (initial_menu, replacement) = create_initial_menu(&commands);
    print!("{}", initial_menu);
    io::stdout().flush().expect("Flushing failed");

    let selection = char::from(getch::Getch::new().getch().unwrap());
    print!("{}", replacement);
    io::stdout().flush().expect("Flushing failed");
    return selection
}

pub fn run_mmm(path: &String) {
    let path_exists = Path::new(path).exists();

    utils::log(initial_greeting(path, path_exists));

    let commands = commands_to_show(path_exists);
    let user_input = user_select_from_menu(&commands);
    for command in commands {
        if command.matches_hotkey(user_input) {
            run_command(command, &path);
            return
        }
    }

    utils::slog("No action chosen. Bye!\n");
}

pub fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}
