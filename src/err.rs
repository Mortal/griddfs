use std::{result, fmt};
use std::os::raw::c_uint;
use bridge::CError;

#[derive(Debug)]
pub enum ErrorKind {
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

pub type Result<T> = result::Result<T, Error>;

/*
impl From<str::Utf8Error> for Error {
    fn from(e: str::Utf8Error) -> Error {
        ErrorKind::UnicodeDecode(e).into()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        ErrorKind::Io(e).into()
    }
}
*/

impl Into<Error> for ErrorKind {
    fn into(self) -> Error {
        Error { kind: self }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
        }
    }
}

impl CError for Error {
    fn get_error_code(&self) -> c_uint {
        match self.kind {
        }
    }
}

