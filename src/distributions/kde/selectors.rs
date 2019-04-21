use super::{Kernel, Point};
use crate::distributions::Pdf;
use crate::fundamental::stddev;
use crate::matrix::Matrix2;

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

#[derive(Debug)]
pub struct BayesianPosterior<'a, P, K> {
    points: &'a [P],
    lambda: f64,
    kernel: &'a K,
}
impl<'a, P, K> BayesianPosterior<'a, P, K> {
    pub fn new(points: &'a [P], kernel: &'a K) -> Self {
        assert!(points.len() > 1);
        Self {
            points,
            kernel,
            lambda: 1.0,
        }
    }
}
impl<'a, K> BayesianPosterior<'a, (f64, f64), K>
where
    K: Pdf<(f64, f64)>,
{
    fn likelihood(&self, i: usize, b: &Matrix2) -> f64 {
        let xi = self.points[i];

        let mut v = 0.0;
        for j in (0..self.points.len()).filter(|&j| j != i) {
            let xj = self.points[j];
            let x = (xi.0 - xj.0, xi.1 - xj.1);
            v += self.kernel.pdf(&(b.clone() * x));
        }

        let n = self.points.len() as f64;
        (v * b.det()) / (n - 1.0)
    }

    fn prior_density(&self, bij: f64) -> f64 {
        1.0 / (1.0 + self.lambda * bij * bij)
    }
}
impl<'a, K> Pdf<Matrix2> for BayesianPosterior<'a, (f64, f64), K>
where
    K: Pdf<(f64, f64)>,
{
    fn pdf(&self, b: &Matrix2) -> f64 {
        let v0 = b
            .lower_triangular()
            .map(|bij| self.prior_density(bij))
            .product::<f64>();
        let v1 = (0..self.points.len())
            .map(|i| self.likelihood(i, b))
            .product::<f64>();
        v0 * v1
    }
}

pub fn likelihood_cv<P, K>(points: &[P], kernel: &K, bandwidth: &P::Bandwidth) -> f64
where
    P: Point,
    K: Kernel<P>,
{
    assert!(points.len() > 1);

    let n = points.len() as f64;
    let mut likelihood = 0.0;
    for i in 0..points.len() {
        let mut v = 0.0;
        for j in (0..points.len()).filter(|&j| j != i) {
            v += kernel.density(&points[i], &points[j], bandwidth);
        }
        likelihood += (v / (n - 1.0)).ln();
    }
    likelihood
}
