use super::Pdf;
use std::f64::consts::PI;

#[derive(Debug, Default, Clone, Copy)]
pub struct StdNormal;
impl Pdf<(f64, f64)> for StdNormal {
    fn pdf(&self, x: &(f64, f64)) -> f64 {
        let a = 1.0 / (2.0 * PI);
        let b = x.0 * x.0 + x.1 * x.1;
        a * (-0.5 * b).exp()
    }
}
