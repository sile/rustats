use crate::{ErrorKind, Result};
use std::cmp::Ordering;
use std::fmt;

/// An floating point number that is known not NaN.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NonNanF64(f64);
impl NonNanF64 {
    /// Creates a non NaN without checking the value.
    ///
    /// # Safety
    ///
    /// The value must not be NaN.
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        Self(n)
    }

    /// Creates a non NaN if the given value is not NaN.
    ///
    /// # Error
    ///
    /// If the given value is NaN, an `ErrorKind::InvalidInput` error will be returned.
    pub fn new(n: f64) -> Result<Self> {
        track_assert!(!n.is_nan(), ErrorKind::InvalidInput);
        Ok(Self(n))
    }

    /// Returns the value as a primitive type.
    pub const fn get(self) -> f64 {
        self.0
    }
}
impl Eq for NonNanF64 {}
impl Ord for NonNanF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or_else(|| unreachable!())
    }
}
impl fmt::Display for NonNanF64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}