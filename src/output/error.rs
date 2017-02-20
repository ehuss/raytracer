use std::{self, fmt};
use std::error::Error as std_Error;

pub type GenericError = Box<std::error::Error>;
pub type Result<T> = std::result::Result<T, GenericError>;

#[derive(Debug)]
pub enum Error {
    UnsupportedImageFormat,
    ExitRequested
}

impl std_Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnsupportedImageFormat => "Unsupported file format",
            Error::ExitRequested => "Exit Requested",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Output Error: {}", self.description())
    }
}
