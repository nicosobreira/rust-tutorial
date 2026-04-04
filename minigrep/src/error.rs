use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    InvalidArgs(String),
    PatternNotFound,
}
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Io(err) => write!(f, "IO error: {}", err),
            CliError::InvalidArgs(msg) => write!(f, "Invalid arguments: {}", msg),
            CliError::PatternNotFound => write!(f, "Pattern not found"),
        }
    }
}
impl std::error::Error for CliError {}
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::Io(err)
    }
}
