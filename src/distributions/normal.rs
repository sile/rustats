use super::{Cdf, Pdf};
use libm;
use rand::distributions::Distribution;
use rand::{self, Rng};
use rand_distr;
use std::f64::consts::{PI, SQRT_2};

#[derive(Debug, Default, Clone, Copy)]
pub struct StandardNormal;
impl Distribution<f64> for StandardNormal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rand_distr::StandardNormal.sample(rng)
    }
}
impl Pdf<f64> for StandardNormal {
    fn pdf(&self, x: &f64) -> f64 {
        let a = (2.0 * PI).sqrt();
        let b = -x.powi(2) / 2.0;
        b.exp() / a
    }
}
impl Pdf<(f64, f64)> for StandardNormal {
    fn pdf(&self, x: &(f64, f64)) -> f64 {
        let a = 1.0 / (2.0 * PI);
        let b = x.0 * x.0 + x.1 * x.1;
        a * (-0.5 * b).exp()
    }
}
impl Cdf<f64> for StandardNormal {
    fn cdf(&self, &x: &f64) -> f64 {
        0.5 * libm::erfc(-x / SQRT_2)
    }
}
