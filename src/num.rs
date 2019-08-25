use crate::{ErrorKind, Result};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::ops;

macro_rules! impl_basic_ops {
    ($ty:ident) => {
        impl ops::Add for $ty {
            type Output = $ty;

            fn add(self, other: Self) -> Self::Output {
                $ty::new(self.0 + other.0).unwrap_or_else(|e| {
                    panic!("self={:?}, other={:?}, error={}", self.0, other.0, e)
                })
            }
        }
        impl ops::AddAssign for $ty {
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }
        impl ops::Sub for $ty {
            type Output = $ty;

            fn sub(self, other: Self) -> Self::Output {
                $ty::new(self.0 - other.0).unwrap_or_else(|e| {
                    panic!("self={:?}, other={:?}, error={}", self.0, other.0, e)
                })
            }
        }
        impl ops::SubAssign for $ty {
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }
        impl ops::Mul for $ty {
            type Output = $ty;

            fn mul(self, other: Self) -> Self::Output {
                $ty::new(self.0 * other.0).unwrap_or_else(|e| {
                    panic!("self={:?}, other={:?}, error={}", self.0, other.0, e)
                })
            }
        }
        impl ops::MulAssign for $ty {
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other;
            }
        }
        impl ops::Div for $ty {
            type Output = $ty;

            fn div(self, other: Self) -> Self::Output {
                $ty::new(self.0 / other.0).unwrap_or_else(|e| {
                    panic!("self={:?}, other={:?}, error={}", self.0, other.0, e)
                })
            }
        }
        impl ops::DivAssign for $ty {
            fn div_assign(&mut self, other: Self) {
                *self = *self / other;
            }
        }
        impl ops::Neg for $ty {
            type Output = $ty;

            fn neg(self) -> Self::Output {
                $ty::new(-self.0).unwrap_or_else(|e| panic!("self={:?}, error={}", self, e))
            }
        }
    };
}

/// An floating point number that is neither infinite nor NaN.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] // TODO: validate deserialization
pub struct FiniteF64(f64);
impl FiniteF64 {
    /// Creates a `FiniteF64` instance without checking the value.
    ///
    /// # Safety
    ///
    /// The value must not be NaN or infinite.
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        Self(n)
    }

    /// Creates a finite number.
    ///
    /// # Error
    ///
    /// If the given value is NaN or infinite, an `ErrorKind::InvalidInput` error will be returned.
    pub fn new(n: f64) -> Result<Self> {
        track_assert!(n.is_finite(), ErrorKind::InvalidInput; n);
        Ok(Self(n))
    }

    /// Returns the value as a primitive type.
    pub const fn get(self) -> f64 {
        self.0
    }
}
impl Eq for FiniteF64 {}
impl Ord for FiniteF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or_else(|| unreachable!())
    }
}
impl fmt::Display for FiniteF64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl_basic_ops!(FiniteF64);

/// An floating point number that is known not NaN.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] // TODO: validate deserialization
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
impl_basic_ops!(NonNanF64);
