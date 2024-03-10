//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use cli_example_utils::read_stdin;
//! let input = read_stdin();
//! ```
//!
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read input line!"

use std::io::{BufRead, BufReader};

pub mod colors;
pub mod config;

/// This function reads a line from stdin and returns it as a String
/// It will panic if it fails to read a line with a message "Failed to read input line!"
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)

}

fn _read_stdin<T: BufRead>(reader: &mut T) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line!");
    line.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::_read_stdin;
    use std::io::Cursor;

    #[test]
    fn test_read_stdin() {
        let mut reader = Cursor::new("Test\n");
        let line = _read_stdin(&mut reader);
        assert_eq!(line, "Test")
    }

    #[test]
    fn test_read_stdin_empty() {
        let mut reader = Cursor::new("\n");
        let line = _read_stdin(&mut reader);
        assert_eq!(line, "")

    }
}
