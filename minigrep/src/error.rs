use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CustomError {
    Io(io::Error),
    MatchNotFound,
    InvalidNumberOfArgs,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Io(err) => write!(f, "{}", err),
            CustomError::MatchNotFound => write!(f, "Match not found"),
            CustomError::InvalidNumberOfArgs => {
                write!(f, "Invalid number of arguments")
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
