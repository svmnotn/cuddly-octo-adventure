use ::json;
use std::error;
use std::fmt;
use std::io;
use std::result;
use super::zip::result::ZipError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Json(json::Error),
    Zip(ZipError),
    Io(io::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Json(ref error) => error.description(),
            Error::Zip(ref error) => error.description(),
            Error::Io(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Json(ref error) => Some(error),
            Error::Zip(ref error) => Some(error),
            Error::Io(ref error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Json(ref error) => fmt::Display::fmt(error, fmt),
            Error::Zip(ref error) => fmt::Display::fmt(error, fmt),
            Error::Io(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error { Error::Io(error) }
}

impl From<json::Error> for Error {
    fn from(error: json::Error) -> Error { Error::Json(error) }
}

impl From<ZipError> for Error {
    fn from(error: ZipError) -> Error { Error::Zip(error) }
}
