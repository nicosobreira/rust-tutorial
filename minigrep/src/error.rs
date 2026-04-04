use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CustomError {
    Io(io::Error),
    MatchNotFound,
    IsDirectory(String),
    InvalidNumberOfArgs,
}
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Io(err) => write!(f, "IO error, {}", err),
            CustomError::MatchNotFound => write!(f, "Match not found"),
            CustomError::IsDirectory(str) => write!(f, "The argument \"{str}\" is a directory"),
            CustomError::InvalidNumberOfArgs => {
                write!(f, "The number of arguments must be exactly three")
            }
        }
    }
}

impl std::error::Error for CustomError {}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> Self {
        CustomError::Io(err)
    }
}
