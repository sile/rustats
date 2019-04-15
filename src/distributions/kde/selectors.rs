use super::Point;
use crate::fundamental::stddev;
use crate::matrix::Matrix2;

pub trait SelectBandwidth<P: Point> {
    fn select_bandwidth(&self, points: &[P]) -> P::Bandwidth;
}

#[derive(Debug)]
pub struct SilvermanRot;
impl SelectBandwidth<(f64, f64)> for SilvermanRot {
    fn select_bandwidth(&self, points: &[(f64, f64)]) -> Matrix2 {
        let n = points.len() as f64;
        let sd0 = stddev(points.iter().map(|p| p.0));
        let sd1 = stddev(points.iter().map(|p| p.1));
        let a = (1.0 / n).powf(1.0 / 6.0);
        Matrix2::new((a * sd0, 0.0), (0.0, a * sd1))
    }
}
