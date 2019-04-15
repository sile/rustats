use self::kernels::Kernel;
use self::selectors::{SelectBandwidth, SilvermanRot};
use crate::distributions::{Pdf, StdNormal};
use crate::matrix::Matrix2;
use std::ops::Sub;

pub mod kernels;
pub mod selectors;

pub trait Point {
    type Bandwidth;
}
impl Point for (f64, f64) {
    type Bandwidth = Matrix2;
}

#[derive(Debug)]
pub struct KernelDensityEstimator<P, S = SilvermanRot, K = StdNormal> {
    kernel: K,
    selector: S,
    points: Vec<P>,
}
impl<P, S, K> Pdf<P> for KernelDensityEstimator<P, S, K>
where
    P: Point + Sub<Output = P> + Clone,
    S: SelectBandwidth<P>,
    K: Kernel<P>,
{
    fn pdf(&self, x: &P) -> f64 {
        let bandwidth = self.selector.select_bandwidth(&self.points);
        let n = self.points.len() as f64;
        let s = self
            .points
            .iter()
            .cloned()
            .map(|xi| self.kernel.density(&(x.clone() - xi), &bandwidth))
            .sum::<f64>();
        s / n
    }
}
