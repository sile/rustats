use crate::distributions::Pdf;
use crate::range::Range;
use rand::distributions::Distribution;
use rand::Rng;
use std::cell::Cell;
use std::f64::NAN;

#[derive(Debug)]
pub struct SliceSampler3d<D> {
    distribution: D,
    range: Range<(f64, f64, f64)>,
    last_point: Cell<Option<(f64, f64, f64)>>,
    last_y: Cell<Option<f64>>,
}
impl<D> SliceSampler3d<D> {
    pub fn new(distribution: D, range: Range<(f64, f64, f64)>) -> Self {
        Self {
            distribution,
            range,
            last_point: Cell::new(None),
            last_y: Cell::new(None),
        }
    }

    pub fn set_last_point(&mut self, point: (f64, f64, f64)) {
        self.last_point.set(Some(point));
    }

    fn gen_range<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        range: Range<(f64, f64, f64)>,
    ) -> (f64, f64, f64) {
        let x = if range.low.0 == range.high.0 {
            range.low.0
        } else {
            rng.gen_range(range.low.0, range.high.0)
        };
        let y = if range.low.1 == range.high.1 {
            range.low.1
        } else {
            rng.gen_range(range.low.1, range.high.1)
        };
        let z = if range.low.2 == range.high.2 {
            range.low.2
        } else {
            rng.gen_range(range.low.2, range.high.2)
        };
        (x, y, z)
    }
}
impl<D> Distribution<(f64, f64, f64)> for SliceSampler3d<D>
where
    D: Pdf<(f64, f64, f64)>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (f64, f64, f64) {
        let last_x = if let Some(p) = self.last_point.get() {
            p
        } else {
            self.gen_range(rng, self.range)
        };

        let last_y = if let Some(y) = self.last_y.get() {
            y
        } else {
            self.distribution.pdf(&last_x)
        };

        let border = if last_y == 0.0 {
            // TODO: remove 0 handling
            0.0
        } else {
            rng.gen_range(0.0, last_y)
        };
        let mut range = self.range;
        loop {
            let x = self.gen_range(rng, range);
            let y = self.distribution.pdf(&x);
            if y > border || border == 0.0 {
                self.last_point.set(Some(x));
                self.last_y.set(Some(y));
                return x;
            }

            if x.0 < last_x.0 {
                range.low.0 = x.0;
            } else {
                range.high.0 = x.0;
            }
            if x.1 < last_x.1 {
                range.low.1 = x.1;
            } else {
                range.high.1 = x.1;
            }
            if x.2 < last_x.2 {
                range.low.2 = x.2;
            } else {
                range.high.2 = x.2;
            }
        }
    }
}

#[derive(Debug)]
pub struct SliceSampler<D> {
    distribution: D,
    range: Range<f64>,
    last_x: Cell<f64>,
}
impl<D> SliceSampler<D>
where
    D: Pdf<f64>,
{
    pub fn new(distribution: D, range: Range<f64>) -> Self {
        Self {
            distribution,
            range,
            last_x: Cell::new(NAN),
        }
    }

    pub fn distribution(&self) -> &D {
        &self.distribution
    }

    pub fn distribution_mut(&mut self) -> &mut D {
        &mut self.distribution
    }

    pub fn range(&self) -> Range<f64> {
        self.range
    }

    pub fn set_last_x(&mut self, x: f64) {
        self.last_x.set(x);
    }
}
impl<D> Distribution<f64> for SliceSampler<D>
where
    D: Pdf<f64>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let last_x = if self.last_x.get().is_nan() {
            rng.gen_range(self.range.low, self.range.high)
        } else {
            self.last_x.get()
        };

        let last_y = self.distribution.pdf(&last_x);
        let border = if last_y == 0.0 {
            // TODO: remove 0 handling
            0.0
        } else {
            rng.gen_range(0.0, last_y)
        };
        let mut range = self.range;
        loop {
            let x = rng.gen_range(range.low, range.high);
            let y = self.distribution.pdf(&x);
            if y > border || border == 0.0 {
                self.last_x.set(x);
                return x;
            }
            if x < last_x {
                range.low = x;
            } else {
                range.high = x;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributions::StandardNormal;
    use rand;
    use trackable::result::TestResult;

    #[test]
    fn slice_sampler_works() -> TestResult {
        let mut rng = rand::thread_rng();
        let range = track!(Range::new(0.2, 1.5))?;
        let sampler = SliceSampler::new(StandardNormal, range);
        for x in sampler.sample_iter(&mut rng).take(100) {
            assert!(range.contains(&x));
        }
        Ok(())
    }
}
