
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ArgParseError(pub String);

impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ArgParseError {}
