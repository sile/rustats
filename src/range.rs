use crate::{ErrorKind, Result};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::iter;
use std::ops::Sub;

// TODO: Add `MaybeEmptyRange`

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] // TODO: validate deserialization
pub struct Range<T> {
    // TODO: private
    pub low: T,  // inclusive
    pub high: T, // exclusive
}
impl<T> Range<T>
where
    T: PartialOrd,
{
    pub fn new(low: T, high: T) -> Result<Self> {
        track_assert!(low < high, ErrorKind::InvalidInput);
        Ok(Self { low, high })
    }

    pub fn contains(&self, x: &T) -> bool {
        match (self.low.partial_cmp(x), self.high.partial_cmp(x)) {
            (Some(Ordering::Equal), Some(Ordering::Greater))
            | (Some(Ordering::Less), Some(Ordering::Greater)) => true,
            _ => false,
        }
    }
}
impl Range<f64> {
    pub fn iter(&self, interval: f64) -> impl Iterator<Item = f64> {
        let Range { low, high } = *self;
        iter::successors(Some(low), move |x| Some(x + interval)).take_while(move |&x| x < high)
    }
}
impl Range<f64> {
    pub fn middle(&self) -> f64 {
        (self.low + self.high) * 0.5
    }
}
impl<T> Range<T>
where
    T: Sub<Output = T> + Clone,
{
    pub fn width(&self) -> T {
        self.high.clone() - self.low.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] // TODO: validate deserialization
pub struct MinMax<T> {
    min: T,
    max: T,
}
impl<T> MinMax<T> {
    pub const unsafe fn new_unchecked(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub const fn min(&self) -> &T {
        &self.min
    }

    pub const fn max(&self) -> &T {
        &self.max
    }
}
impl<T> MinMax<T>
where
    T: Sub<Output = T> + Clone,
{
    pub fn width(&self) -> T {
        self.max.clone() - self.min.clone()
    }
}
impl<T> MinMax<T>
where
    T: PartialOrd,
{
    pub fn new(min: T, max: T) -> Result<Self> {
        track_assert!(min <= max, ErrorKind::InvalidInput);
        Ok(Self { min, max })
    }

    pub fn contains(&self, x: &T) -> bool {
        self.min <= *x && *x <= self.max
    }
}
impl MinMax<f64> {
    // Normalizes scale of ...
    pub fn normalize(&self, v: f64) -> f64 {
        (v - self.min) / (self.max - self.min)
    }
}
