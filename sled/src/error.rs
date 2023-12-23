use std::{error::Error, fmt};
use std::convert::From;

#[derive(Debug)]
pub struct SledError {
    pub message: String,
}

impl SledError {
    pub fn from_error(e: impl Error) -> Self {
        SledError {
            message: e.to_string(),
        }
    }
}

impl fmt::Display for SledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SledError {} // seems we can't have both. Might not be the best design; reconsider.
