use crate::{ErrorKind, Result};
use std::cmp::Ordering;
use std::iter;

#[derive(Debug, Clone, Copy)]
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
        track_assert_eq!(
            low.partial_cmp(&high),
            Some(Ordering::Less),
            ErrorKind::InvalidInput
        );
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

    pub fn width(&self) -> f64 {
        self.high - self.low
    }
}
