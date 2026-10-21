pub enum SaoError {
    Io(std::io::Error),
    Toml(toml::de::Error),
    NoConfigFile,
    NoCmd,
    Utf8(std::string::FromUtf8Error),
    NoSinks,
    ParseInt(std::num::ParseIntError),
    NoID(String),
    NoValidSinkInConfig,
}

pub enum Output {
    Exit(ExitCode),
    Continue,
}

pub type ExitCode = i32;
pub const ERROR_CODE: ExitCode = 1;
pub const STANDARD_CODE: ExitCode = 0;

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
            SaoError::NoID(target) => writeln!(f, "No ID couldn't be tied to {target}"),
            SaoError::NoValidSinkInConfig => writeln!(f, "Config file contains no valid sink"),
        }
    }
}
