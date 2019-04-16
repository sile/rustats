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

/// Possible error kinds.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// Invalid input was given.
    InvalidInput,

    /// Implementation bug.
    Bug,

    /// Other error.
    Other,
}
impl TrackableErrorKind for ErrorKind {}
