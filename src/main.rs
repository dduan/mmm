use getch;
use std::env;
use std::io::Write;
use std::io;
use std::time;

mod mmm;

use mmm::Command;
use mmm::commands;
use mmm::utils;

fn create_initial_menu(commands: &Vec<Box<Command>>, path: &String) -> String {
    let initial_text: Vec<String> = commands
        .iter()
        .filter(|c| c.should_show(&path))
        .map(|c| c.display_text())
        .collect();

    return initial_text.join(" | ")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} PATH", args[0]);
        return;
    }

    let path = &args[1];
    let commands = commands::all_commands();

    let initial_menu = create_initial_menu(&commands, path);
    print!("{}", initial_menu);
    io::stdout().flush().expect("Flushing failed");

    let initial_selection = char::from(getch::Getch::new().getch().unwrap());

    print!("\r{}\r", " ".repeat(initial_menu.len()));
    io::stdout().flush().expect("Flushing failed");

    for command in commands {
        if command.matches_hotkey(initial_selection) {
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
                    utils::log(command.wrapup_msg());
                }

                utils::log(format!("That took {}.{}s", exe_duration.as_secs(), exe_duration.subsec_millis()));
            } else {
                utils::elog("Something went wrong :(\n");
            }

            return
        }
    }

    utils::log("No action chosen. Bye!\n");

    // if path exist:
    //   [E]dit | [O]pen | [M]ove... | [D]elete... | [S]tage | [U]nstage | [I]nfo
    // else:
    //   [E]dit | [T]ouch | Ma[k]e Directory

    // Move
    // Moving PATH
    // Enter destination: _
    // Done.

    // Delete
    // Deleting PATH
    // Confirm (y/N): _
    // Done.
}
