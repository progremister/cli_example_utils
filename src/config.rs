//! This module contains the configuration options for the application.

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String)
}

/// This structure contains the configuration options for controlling logging.
/// Examples: 
/// ```
/// use cli_example_utils::config::Logging;
/// let config = Logging::new();
/// ```
/// Create a new `Logging` struct with custom values
/// ```
/// use cli_example_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Warn,
///     destination: LogOutput::File("logs.txt".to_string())
/// }
/// ```

pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout
        }
    }
}
