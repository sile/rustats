use self::selectors::{SelectBandwidth, SilvermanRot};
use crate::distributions::{Pdf, StandardNormal};
use crate::matrix::Matrix2;

pub use self::kernel::Kernel;

mod kernel;
pub mod selectors;

pub trait Point {
    type Bandwidth;
}
impl Point for (f64, f64) {
    type Bandwidth = Matrix2;
}

#[derive(Debug)]
pub struct KernelDensityEstimatorBuilder<S = SilvermanRot, K = StandardNormal> {
    kernel: K,
    selector: S,
}
impl KernelDensityEstimatorBuilder<SilvermanRot, StandardNormal> {
    pub fn new() -> Self {
        Self {
            kernel: StandardNormal,
            selector: SilvermanRot,
        }
    }
}
impl<S, K> KernelDensityEstimatorBuilder<S, K> {
    pub fn selector<P, S1>(self, selector: S1) -> KernelDensityEstimatorBuilder<S1, K>
    where
        P: Point,
        S1: SelectBandwidth<P>,
    {
        KernelDensityEstimatorBuilder {
            kernel: self.kernel,
            selector,
        }
    }

    pub fn kernel<P, K1>(self, kernel: K1) -> KernelDensityEstimatorBuilder<S, K1>
    where
        P: Point,
        K1: Kernel<P>,
    {
        KernelDensityEstimatorBuilder {
            kernel,
            selector: self.selector,
        }
    }

    pub fn finish<P>(self) -> KernelDensityEstimator<P, S, K>
    where
        P: Point,
        S: SelectBandwidth<P>,
        K: Kernel<P>,
    {
        KernelDensityEstimator {
            kernel: self.kernel,
            selector: self.selector,
            points: Vec::new(),
            bandwidth: None,
        }
    }
}

#[derive(Debug)]
pub struct KernelDensityEstimator<P, S = SilvermanRot, K = StandardNormal>
where
    P: Point,
{
    kernel: K,
    selector: S,
    points: Vec<P>,
    bandwidth: Option<P::Bandwidth>,
}
impl<P, S, K> KernelDensityEstimator<P, S, K>
where
    P: Point,
    S: SelectBandwidth<P> + Default,
    K: Kernel<P> + Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}
impl<P, S, K> KernelDensityEstimator<P, S, K>
where
    P: Point,
    S: SelectBandwidth<P>,
    K: Kernel<P>,
{
    pub fn push(&mut self, point: P) {
        self.points.push(point);

        // TODO: optimize
        self.bandwidth = Some(self.selector.select_bandwidth(&self.points));
    }

    pub fn points(&self) -> &[P] {
        &self.points
    }

    pub fn points_mut(&mut self) -> &mut Vec<P> {
        &mut self.points
    }

    pub fn selector(&self) -> &S {
        &self.selector
    }

    pub fn selector_mut(&mut self) -> &mut S {
        &mut self.selector
    }

    pub fn kernel(&self) -> &K {
        &self.kernel
    }

    pub fn kernel_mut(&mut self) -> &mut K {
        &mut self.kernel
    }
}
impl<P, S, K> Default for KernelDensityEstimator<P, S, K>
where
    P: Point,
    S: SelectBandwidth<P> + Default,
    K: Kernel<P> + Default,
{
    fn default() -> Self {
        KernelDensityEstimatorBuilder::new()
            .selector(S::default())
            .kernel(K::default())
            .finish()
    }
}
impl<P, S, K> Pdf<P> for KernelDensityEstimator<P, S, K>
where
    P: Point,
    S: SelectBandwidth<P>,
    K: Kernel<P>,
{
    fn pdf(&self, x: &P) -> f64 {
        let bandwidth = self.bandwidth.as_ref().unwrap_or_else(|| unimplemented!());
        let n = self.points.len() as f64;
        let s = self
            .points
            .iter()
            .map(|xi| self.kernel.density(x, xi, bandwidth))
            .sum::<f64>();
        s / n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributions::Pdf;

    #[test]
    fn kde_rot_works() {
        let mut kde = KernelDensityEstimator::<(f64, f64)>::new();
        let xs = [
            0.38897972954436744,
            0.21575530912512608,
            0.4594677812717819,
            0.3517222887315343,
            0.9778939800250716,
            0.111707692159418,
            0.42829174692685035,
            0.3578677422355636,
            0.08399333064039338,
            0.5204669069143946,
        ];
        let ys = [
            0.8282605782037772,
            0.3399377882894066,
            0.5576939017521526,
            0.1923711081922198,
            0.6086854735552321,
            0.5131020176289642,
            0.7632850336269744,
            0.6099470684522489,
            0.41308355846616196,
            0.962957265549666,
        ];
        for point in xs.iter().cloned().zip(ys.iter().cloned()) {
            kde.push(point);
        }

        assert_eq!(kde.pdf(&(0.2, 0.2)), 0.5279661447250167);
        assert_eq!(kde.pdf(&(0.1, 0.4)), 0.6216566550321325);
        assert_eq!(kde.pdf(&(2.0, 2.4)), 0.00000023735062151042504);
    }
}
