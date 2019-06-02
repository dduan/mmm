use std::io::Write;
use std::fmt::Display;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub fn write<T>(mut buffer: &mut Buffer, text: T, color: Color) where T: Display {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(color));
    buffer.set_color(&spec).unwrap();
    write!(&mut buffer, "{}", text).expect("");
    buffer.reset().unwrap();
}

pub fn log<T>(log_text: T) where T: Display {
    print!("[mmm] {}", log_text);
    std::io::stdout().flush().expect("Flushing failed");
}

pub fn slog<T>(log_text: T) where T: Display {
    let stdout = BufferWriter::stdout(ColorChoice::Auto);
    let mut buffer = stdout.buffer();
    write!(&mut buffer, "[").expect("Buffer write failure");
    write(&mut buffer, "mmm", Color::Green);
    write!(&mut buffer, "] {}", log_text).expect("Buffer write failure");
    stdout.print(&buffer).expect("stdout print failure");
}

pub fn elog<T>(log_text: T) where T: Display {
    let stderr = BufferWriter::stderr(ColorChoice::Auto);
    let mut buffer = stderr.buffer();
    write!(&mut buffer, "[").expect("Buffer write failure");
    write(&mut buffer, "mmm", Color::Red);
    write!(&mut buffer, "] {}", log_text).expect("Buffer write failure");
    stderr.print(&buffer).expect("stdout print failure");
}
