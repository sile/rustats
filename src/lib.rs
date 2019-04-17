#[macro_use]
extern crate trackable;

pub use self::error::{Error, ErrorKind};

pub mod distributions;
pub mod fundamental;
pub mod matrix;
pub mod plot;
pub mod range;
pub mod samplers;

mod error;

pub type Result<T> = std::result::Result<T, Error>;
