use std::convert::Infallible;
use std::io;
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt};
use trackable::error::{Failure, TrackableError};

/// This crate specific `Error` type.
#[derive(Debug, Clone, TrackableError)]
pub struct Error(TrackableError<ErrorKind>);
impl From<Failure> for Error {
    fn from(f: Failure) -> Self {
        ErrorKind::Other.takes_over(f).into()
    }
}
impl From<io::Error> for Error {
    fn from(f: io::Error) -> Self {
        ErrorKind::IoError.cause(f).into()
    }
}
impl From<Infallible> for Error {
    fn from(f: Infallible) -> Self {
        ErrorKind::Bug.cause(f).into()
    }
}

/// Possible error kinds.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// Invalid input was given.
    InvalidInput,

    /// I/O error.
    IoError,

    /// Implementation bug.
    Bug,

    /// Other error.
    Other,
}
impl TrackableErrorKind for ErrorKind {}
