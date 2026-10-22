// Represents all errors that can occur during the execution of `sao`.
pub enum SaoError {
    // An I/O error occurred (e.g., executing a child process or reading a file).
    Io(std::io::Error),
    // An error occurred while parsing the TOML configuration file.
    Toml(toml::de::Error),
    // No valid configuration file could be found in the expected directories.
    NoConfigFile,
    // The `wpctl` command line tool is not installed or not available in PATH.
    NoCmd,
    // Failed to parse process stdout as valid UTF-8.
    Utf8(std::string::FromUtf8Error),
    // `wpctl` returned an empty list of audio sinks.
    NoSinks,
    // Failed to parse a sink ID into a numerical value.
    ParseInt(std::num::ParseIntError),
    // No audio sink matching the requested target/alias could be found.
    NoID(String),
    // None of the sinks listed in the configuration file match active system sinks.
    NoValidSinkInConfig,
}

// The status returned by [`Automaton::run`](crate::Automaton::run) upon successful execution step.
//
// Indicates whether the application state machine should continue running or exit 
// with a specific process exit code.
pub enum Output {
    // The application lifecycle has completed and should exit with the given code.
    Exit(code::ExitCode),
    // The current step succeeded and the automaton should continue to the next state.
    Continue,
}

pub mod code {
    // Numerical exit code for process termination.
    pub type ExitCode = i32;

    // Successful termination (exit code 0).
    pub const STANDARD: ExitCode = 0;

    // An I/O error occurred (exit code 1).
    pub const IO: ExitCode = 1;

    // A configuration error occurred (exit code 2).
    pub const CONFIG: ExitCode = 2;

    // An issue with `wpctl` or command output parsing (exit code 3).
    pub const WPCTL: ExitCode = 3;

    // Target sink selection or identification failed (exit code 4).
    pub const TARGET: ExitCode = 4;
}

impl From<std::io::Error> for SaoError {
    fn from(err: std::io::Error) -> Self {
        SaoError::Io(err)
    }
}

impl From<toml::de::Error> for SaoError {
    fn from(err: toml::de::Error) -> Self {
        SaoError::Toml(err)
    }
}

impl From<std::string::FromUtf8Error> for SaoError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        SaoError::Utf8(err)
    }
}

impl From<std::num::ParseIntError> for SaoError {
    fn from(err: std::num::ParseIntError) -> Self {
        SaoError::ParseInt(err)
    }
}

impl std::fmt::Display for SaoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaoError::NoConfigFile => writeln!(f, "There is no viable configuration file"),
            SaoError::Io(err) => writeln!(f, "{err}"),
            SaoError::Toml(err) => writeln!(f, "{err}"),
            SaoError::NoCmd => writeln!(f, "The command wpctl is unavailable"),
            SaoError::Utf8(err) => writeln!(f, "{err}"),
            SaoError::NoSinks => writeln!(f, "wpctl didn't find any sinks"),
            SaoError::ParseInt(err) => writeln!(f, "{err}"),
            SaoError::NoID(target) => writeln!(f, "No ID could be tied to '{target}'"),
            SaoError::NoValidSinkInConfig => writeln!(f, "Config file contains no valid sink"),
        }
    }
}
