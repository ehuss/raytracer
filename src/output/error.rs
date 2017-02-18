use std::{self, io, fmt};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => e.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref e) => e.fmt(fmt),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
