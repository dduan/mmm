use atty;
use std::fmt::Display;
use std::io::Write;
use std::io;
use colored::*;

pub fn color_text<T: Display>(text: T, text_color: Color) -> String {
    let plain_text = format!("{}", text);
    if atty::is(atty::Stream::Stdout) {
        plain_text.color(text_color).to_string()
    } else {
        plain_text
    }
}

pub fn log<T>(log_text: T) where T: Display {
    print!("[mmm] {}", log_text);
    io::stdout().flush().expect("Flushing failed");
}

pub fn slog<T>(log_text: T) where T: Display {
    let mmm_success = color_text("mmm", Color::Green);
    print!("[{}] {}", mmm_success, log_text);
    io::stdout().flush().expect("Flushing failed");
}

pub fn elog<T>(log_text: T) where T: Display {
    let mmm_warning = color_text("mmm", Color::Red);
    eprint!("[{}] {}", mmm_warning, log_text);
    io::stderr().flush().expect("Flushing failed");
}
