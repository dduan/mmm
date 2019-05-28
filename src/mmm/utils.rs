use atty;
use std::fmt::Display;
use std::io::Write;
use std::io;
use termion::{color, style};

pub fn color_text<T:, C>(text: T, text_color: C) -> String
    where T: Display, C: color::Color
{
    if atty::is(atty::Stream::Stdout) {
        return format!("{}{}{}", color::Fg(text_color), text, style::Reset);
    } else {
        return format!("{}", text);
    }
}

pub fn log<T>(log_text: T) where T: Display {
    let mmm_success = color_text("mmm", color::Green);
    print!("[{}] {}", mmm_success, log_text);
    io::stdout().flush().expect("Flushing failed");
}

pub fn elog<T>(log_text: T) where T: Display {
    let mmm_warning = color_text("mmm", color::Yellow);
    eprint!("[{}] {}", mmm_warning, log_text);
    io::stderr().flush().expect("Flushing failed");
}
