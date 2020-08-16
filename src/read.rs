use crate::preprocess::prompt;
use std::io::{stdin, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn read_loop() {
    // Get the standard input stream
    let stdin = stdin();

    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    // This buffer contains user input
    // After every Enter this buffer will be collected and processed in order to run the command
    // This buffer will be updated after every insert and delete
    let mut char_buf = Vec::<char>::new();

    // The print_prompt method prints the prompt and returns the minimum x coordinate that cursor can hold
    // Cursor can not go behind the prompt therefore this minimum value is the size of the printed prompt
    let mut min_cursor_x_bound = print_prompt(&mut stdout);

    for c in stdin.keys() {
        let (cursor_x, cursor_y) = stdout.cursor_pos().unwrap();

        match c.unwrap() {
            Key::Char('\n') => {
                print!("\n\r");
                min_cursor_x_bound = print_prompt(&mut stdout);
            }
            Key::Char(c) => {
                char_buf.push(c);
                print!("{}", c)
            }
            Key::Ctrl(c) if c == 'c' => break,
            Key::Left => move_cursor_left(&mut stdout, cursor_x, cursor_y, min_cursor_x_bound),
            Key::Right => move_cursor_right(
                &mut stdout,
                cursor_x,
                cursor_y,
                char_buf.len() as u16 + min_cursor_x_bound,
            ),
            Key::Backspace => {
                delete_last_char(&mut stdout, cursor_x, cursor_y, min_cursor_x_bound);
                char_buf.pop();
            }
            _ => continue,
        }

        // Flush again.
        stdout.flush().unwrap();
    }

    // go to a clear line and exit
    print!("\n\r");
    stdout.flush().unwrap();
}

// Prints the prompt
// Returns minimum x coordinate that cursor can get without interfering with the prompt text
fn print_prompt(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) -> u16 {
    // Get the prompt and its size without any style and color
    let (prompt_text, prompt_len) = prompt();

    // Print the prompt
    print!("{}", prompt_text);
    stdout.flush().unwrap();

    // Cursor must be on position after the last char of prompt
    // That plus one is because coordinate starts at 1. so the 1 is offset
    (prompt_len + 1) as u16
}

// Moves the cursor one position to left until it reaches the minimum allowed value
fn move_cursor_left(
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    cursor_x: u16,
    cursor_y: u16,
    min_cursor_x_bound: u16,
) {
    write!(
        stdout,
        "{}",
        termion::cursor::Goto(std::cmp::max(cursor_x - 1, min_cursor_x_bound), cursor_y),
    )
    .unwrap();
}

// Moves the cursor one position to right until it reaches the maximum allowed value
// Maximum value is the total number of typed chars
// Cursor can not go beyond last typed char
fn move_cursor_right(
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    cursor_x: u16,
    cursor_y: u16,
    max_cursor_x_bound: u16,
) {
    write!(
        stdout,
        "{}",
        termion::cursor::Goto(std::cmp::min(cursor_x + 1, max_cursor_x_bound), cursor_y),
    )
    .unwrap();
}

// Replaces the character behind the cursor with space(" ")
fn delete_last_char(
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    cursor_x: u16,
    cursor_y: u16,
    min_cursor_x_bound: u16,
) {
    // Must not delete any char from prompt
    if cursor_x - 1 >= min_cursor_x_bound {
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(cursor_x - 1, cursor_y),
            " ",
            termion::cursor::Goto(cursor_x - 1, cursor_y),
        )
        .unwrap();
    }
}
