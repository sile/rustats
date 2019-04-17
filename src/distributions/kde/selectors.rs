use super::Point;
use crate::fundamental::stddev;
use crate::matrix::{Matrix2, Matrix4, Transpose};
use std::f64::consts::PI;

pub trait SelectBandwidth<P: Point> {
    fn select_bandwidth(&self, points: &[P]) -> P::Bandwidth;
}
impl SelectBandwidth<(f64, f64)> for Matrix2 {
    fn select_bandwidth(&self, _points: &[(f64, f64)]) -> Matrix2 {
        self.clone()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SilvermanRot;
impl SelectBandwidth<(f64, f64)> for SilvermanRot {
    fn select_bandwidth(&self, points: &[(f64, f64)]) -> Matrix2 {
        let n = points.len() as f64;
        let sd0 = stddev(points.iter().map(|p| p.0));
        let sd1 = stddev(points.iter().map(|p| p.1));
        let a = (1.0 / n).powf(1.0 / 6.0);
        Matrix2::diagonal(a * sd0, a * sd1)
    }
}

pub fn amise2_normal<F>(points: &[(f64, f64)], h: &Matrix2, psi: Matrix4) -> f64 {
    let n = points.len() as f64;

    let r = (4.0 * PI).powf(-1.0);
    let a = n.powf(-1.0) * h.det().powf(-0.5) * r;

    let m2 = 1f64.powi(2);
    let b = 0.25 * m2;
    // let vec = h.cols().flat_map(|c| c.iter()).collect::<Vec<_>>();
    // let c = Transpose(vec) * (psi * vec.clone());
    // a + b * c
    unimplemented!()
}
