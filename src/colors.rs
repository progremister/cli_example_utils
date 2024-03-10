//! ANSI color codes for use in terminal output. With helper functions to wrap strings in color codes.
//! # Examples: 
//! ```
//! use cli_example_utils::{red, blue};
//! let red_string = red("This is a red string.");
//! let blue_string = blue("This is a blue string.");
//! ```

/// Returns a string wrapped in ANSI red color codes.
/// # Examples: 
/// ```
/// use cli_example_utils::colors::red;
/// let red_string = red("This is a red string!");
/// ```

pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns a string wrapped in ANSI green color codes.
/// # Examples: 
/// ```
/// use cli_example_utils::colors::green;
/// let green_string = blue("This is a green string!");
/// `

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string wrapped in ANSI blue color codes.
/// # Examples: 
/// ```
/// use cli_example_utils::colors::blue;
/// let blue_string = blue("This is a blue string!");
/// `

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string wrapped in ANSI bold codes.
/// # Examples: 
/// ```
/// use cli_example_utils::colors::bold;
/// let bold_string = bold("This is a bold string!");
/// `

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

pub enum Color {
    Red,
    Green,
    Blue,
    Bold
}

/// This structure contains the options for controlling colors.
/// Examples: 
/// ```
/// use cli_example_utils::colors::ColorString;
/// let color_string = ColorString {
///     color: Color::Red,
///     string: "Hello, world!".to_string()
///     colorized: "\x1b[31m{}\x1b[0m".to_string(),
/// }
/// ```
/// Set the string color depending on the `color` attribute
/// ```
/// color_string.paint();
/// ```
/// Reset the colorized string 
/// ```
/// color_string.reset();
/// ```


pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}

impl ColorString {
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string)
        };
    }

    pub fn reset(&mut self) {
        self.colorized = reset(&self.string); 
    }
}
