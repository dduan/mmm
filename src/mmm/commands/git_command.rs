use colored::Color;
use std::process;
use super::Command;
use super::utils;

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
    fn should_show_if_path_exists_not(&self) -> bool { self.in_git }

    fn need_followup(&self) -> bool { true }
    fn followup_prompt(&self, path: &String) -> String {
        format!("`git {} {}`\n           {} ",
                utils::color_text(".", Color::Yellow),
                path,
                utils::color_text("└", Color::Yellow))
    }

    fn execute(&self, path: &String, followup_input: Option<String>) -> bool {
        let subcommand = String::from(followup_input.unwrap_or_default().trim_end());
        if subcommand.is_empty() {
            utils::elog("Please provide a valid git subcommand.\n");
            return false
        }

        let whole_command = format!("git {} {}", subcommand, path);
        utils::log(format!("Running `{}`\n", utils::color_text(whole_command, Color::Yellow)));

        run_git(subcommand, path.to_string())
    }
}
