use super::Pdf;
use rand::distributions::Distribution;
use rand::{self, Rng};
use std::f64::consts::PI;

#[derive(Debug, Default, Clone, Copy)]
pub struct StandardNormal;
impl Distribution<f64> for StandardNormal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rand::distributions::StandardNormal.sample(rng)
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
