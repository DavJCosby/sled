use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Simple error type used by fallible Sled operations.
pub struct SledError {
    pub message: String,
}

impl SledError {
    pub fn new(message: String) -> Self {
        SledError { message }
    }

    pub fn from_error(e: impl Error) -> Self {
        SledError {
            message: e.to_string(),
        }
    }

    pub fn as_err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl std::convert::From<&str> for SledError {
    fn from(value: &str) -> Self {
        SledError::new(value.to_string())
    }
}

impl fmt::Display for SledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SledError {}