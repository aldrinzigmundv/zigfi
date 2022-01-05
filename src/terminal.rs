#![forbid(unsafe_code)]
#[allow(dead_code)]

use std::io::{stdout, Write};

use crossterm::{
    cursor::{position, Hide, MoveTo, MoveToNextLine, Show},
    style::{Color, SetForegroundColor},
    terminal, ExecutableCommand,
};

pub fn setup() {
    stdout()
        .execute(terminal::EnterAlternateScreen)
        .expect("Terminal Error");
    crossterm::terminal::enable_raw_mode().expect("Terminal Error");
    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .expect("Terminal Error");
    stdout().execute(Hide).expect("Terminal Error");
    stdout().execute(MoveTo(0, 0)).expect("Terminal error.");
    stdout().flush().expect("Terminal error.");
}

pub fn write(text: &str) {
    stdout()
        .write(text.to_string().as_bytes())
        .expect("Terminal Error");
    stdout().flush().expect("Terminal error.");
}

pub fn write_then_nextline(text: &str) {
    stdout()
        .write(text.to_string().as_bytes())
        .expect("Terminal Error");
    stdout()
        .execute(MoveToNextLine(1))
        .expect("Terminal error.");
    stdout().flush().expect("Terminal error.");
}

pub fn write_within_space(text: &str, space: u16) {
    stdout()
        .write(text.to_string().as_bytes())
        .expect("Terminal Error");
    let (column, _) = position().expect("Terminal error.");
    let blanks = space - column;
    for _ in 0..blanks {
        stdout().write(" ".as_bytes()).expect("Terminal error.");
    }
    stdout().flush().expect("Terminal error.");
}

pub fn write_multiline(texts: Vec<&str>) {
    for text in texts {
        write_then_nextline(text);
    }
    stdout().flush().expect("Terminal error.");
}

pub fn skip_line() {
    stdout()
        .execute(MoveToNextLine(1))
        .expect("Terminal error.");
    stdout().flush().expect("Terminal error.");
}

pub fn set_color(color: Color) {
    stdout()
        .execute(SetForegroundColor(color))
        .expect("Terminal error.");
    stdout().flush().expect("Terminal error.");
}

pub fn reset_color() {
    stdout()
        .execute(SetForegroundColor(Color::Reset))
        .expect("Terminal error.");
    stdout().flush().expect("Terminal error.");
}

pub fn reset_cursor() {
    stdout().execute(MoveTo(0, 0)).expect("Terminal error.");
}

pub fn cleanup() {
    terminal::disable_raw_mode().unwrap();
    stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    stdout().execute(Show).unwrap();
}
