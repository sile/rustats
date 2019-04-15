use crate::matrix::Matrix2;
use crate::Point;

pub trait SelectBandwidth<P: Point> {
    fn select_bandwidth(&self, points: &[P]) -> P::Bandwidth;
}

#[derive(Debug)]
pub struct SilvermanRot;
impl SelectBandwidth<(f64, f64)> for SilvermanRot {
    fn select_bandwidth(&self, points: &[(f64, f64)]) -> Matrix2 {
        panic!()
    }
}

#[derive(Debug)]
pub struct ScottRot;
impl SelectBandwidth<(f64, f64)> for ScottRot {
    fn select_bandwidth(&self, points: &[(f64, f64)]) -> Matrix2 {
        panic!()
    }
}
