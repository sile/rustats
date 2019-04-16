use crate::distributions::Pdf;
use crate::range::Range;
use rand::distributions::Distribution;
use rand::Rng;
use std::cell::Cell;
use std::f64::NAN;

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
        let border = rng.gen_range(0.0, last_y);
        let mut range = self.range;
        loop {
            let x = rng.gen_range(range.low, range.high);
            let y = self.distribution.pdf(&x);
            if y > border {
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
