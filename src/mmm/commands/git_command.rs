use std::process;
use std::io::Write;
use super::Command;
use super::utils;
use termcolor::{
    Buffer,
    BufferWriter,
    Color,
    ColorChoice,
};

pub struct GitCommand {
    in_git: bool
}

fn run_git(arg1: String, arg2: String) -> bool {
    let mut git = process::Command::new("git");
    git.arg(arg1);
    git.arg(arg2);
    let git_proc = git.spawn();

    if !git_proc.is_ok() {
        return false
    }

    git_proc.unwrap()
        .wait()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn is_in_git() -> bool {
    process::Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .map(|o| o.status.success())
        .ok()
        .unwrap_or(false)
}

impl Command for GitCommand {
    fn new() -> GitCommand {
        GitCommand {
            in_git: is_in_git()
        }
    }

    fn name(&self) -> String { String::from("Git") }
    fn hotkey_pos(&self) -> usize { 0 }
    #[allow(unused_variables)]
    fn exe_msg(&self, path: &String) -> Option<String> {
        Some(String::from("Now, pick a git subcommand\n"))
    }

    fn should_show_if_path_exists(&self) -> bool { self.in_git }
    fn should_show_if_path_exists_not(&self) -> bool { false }

    fn need_followup(&self) -> bool { true }
    fn followup_prompt(&self, path: &String) -> Buffer {
        let mut buffer = BufferWriter::stdout(ColorChoice::Auto).buffer();

        write!(&mut buffer, "[mmm] `git ").expect("Buffer write error");
        utils::write(&mut buffer, ".", Color::Yellow);
        write!(&mut buffer, " {}`\n           ", path).expect("Buffer write error");
        utils::write(&mut buffer, "└", Color::Yellow);
        write!(&mut buffer, " ").expect("Buffer write error");

        buffer
    }

    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let subcommand = String::from(followup_input.unwrap_or_default().trim_end());
        if subcommand.is_empty() {
            utils::elog("Please provide a valid git subcommand.\n");
            return false
        }

        let whole_command = format!("git {} {}", subcommand, path);
        let stdout = BufferWriter::stdout(ColorChoice::Auto);
        let mut buffer = stdout.buffer();
        write!(&mut buffer, "[mmm] Running `").expect("Buffer write failure");
        utils::write(&mut buffer, whole_command, Color::Yellow);
        write!(&mut buffer, "`\n").expect("Buffer write failure");
        stdout.print(&buffer).expect("Stdout print failure");

        run_git(subcommand, path.to_string())
    }
}
