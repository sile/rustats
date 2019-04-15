use crate::kernels::Kernel;
use crate::matrix::Matrix2;
use crate::selectors::SelectBandwidth;
use std::ops::Sub;

pub mod kernels;
pub mod matrix;
pub mod selectors;

pub trait Pdf<P> {
    fn pdf(&self, point: &P) -> f64;
}

pub trait Point {
    type Bandwidth;
}
impl Point for (f64, f64) {
    type Bandwidth = Matrix2;
}

#[derive(Debug)]
pub struct KernelDensityEstimator<P, S, K = crate::kernels::StdNormal> {
    kernel: K,
    selector: S,
    points: Vec<P>,
}
impl<P, S, K> Pdf<P> for KernelDensityEstimator<P, S, K>
where
    P: Point + Sub<Output = P> + Clone,
    S: SelectBandwidth<P>,
    K: Kernel<P::Output>,
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
