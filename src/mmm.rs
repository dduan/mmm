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
use termcolor::{
    Buffer,
    BufferWriter,
    Color,
    ColorChoice,
};

fn run_command(command: Box<Command>, path: &String) {
    let message = command.exe_msg(&path);
    if message.is_some() {
        utils::log(message.unwrap());
    }

    let mut followup_input = None;
    if command.need_followup() {
        print!("[mmm] ");
        BufferWriter::stdout(ColorChoice::Auto).print(&command.followup_prompt(&path)).unwrap();
        io::stdout().flush().expect("Stdout flush error");
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

fn initial_greeting(buffer: &mut Buffer, path: &String, path_exists: bool) {
    if path_exists {
        write!(buffer, "What would you like to do to ").expect("Buffer write error");
        utils::write(buffer, path, Color::Yellow);
        write!(buffer, "?\n").expect("Buffer write error");
    } else {
        utils::write(buffer, path, Color::Yellow);
        write!(buffer, " doesn't exists yet, what's next?\n").expect("Buffer write error");
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
    let display = commands
        .iter()
        .map(|c| c.display_text());

    // TODO: can we get away with no clone?
    let uncolored_text_count: usize = display.clone().map(|t| t.1 + 3).sum();
    let items: Vec<Buffer> = display.map(|t| t.0).collect();

    let stdout = BufferWriter::stdout(ColorChoice::Auto);
    for item in items {
        stdout.print(&item).expect("Stdout print error");
        print!(" | ");
    }
    std::io::stdout().flush().unwrap();

    let selection = char::from(getch::Getch::new().getch().unwrap());

    if atty::is(atty::Stream::Stdout) {
        print!("\r{}\r", " ".repeat(uncolored_text_count));
    } else {
        print!("\n");
    }

    return selection
}

pub fn run_mmm(path: &String) {
    let path_exists = Path::new(path).exists();
    let stdout = BufferWriter::stdout(ColorChoice::Auto);
    let mut out_buffer = stdout.buffer();
    print!("[mmm] ");
    initial_greeting(&mut out_buffer, path, path_exists);
    stdout.print(&out_buffer).expect("Stdout print error");

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

pub fn print_help() {
    print!("mmm {}
A little interactive file system manipulator.

USAGE:
    mmm PATH_TO_MANIPULATE
    mmm -h, --help         Print this message.
    mmm -v, --version      Print version.

https://twitter.com/daniel_duan",
    env!("CARGO_PKG_VERSION"));
}
