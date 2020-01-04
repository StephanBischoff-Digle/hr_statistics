use std::error::Error;
use std::fmt;
use std::io;

/// Fuses IO, Parse and Logic errors into one such that I can collect them and return them neatly
/// from main, if one occures.
#[derive(Debug)]
pub enum ProgramError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    Logic(String),
}

impl Error for ProgramError {}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "Io Error: {}", err),
            Self::Parse(err) => write!(f, "Parse Error: {}", err),
            Self::Logic(err) => write!(f, "Logic Error: {}", err),
        }
    }
}

impl std::convert::From<io::Error> for ProgramError {
    fn from(other: io::Error) -> Self {
        Self::Io(other)
    }
}

impl std::convert::From<std::num::ParseIntError> for ProgramError {
    fn from(other: std::num::ParseIntError) -> Self {
        Self::Parse(other)
    }
}
