use super::MaybeUniform;
use super::{Kernel, Point};
use crate::distributions::Pdf;
use crate::fundamental::{average, stddev};
use crate::matrix::Matrix2;
use crate::range::Range;
use crate::samplers::SliceSampler3d;
use rand::distributions::Distribution;

pub trait SelectBandwidth<P: Point> {
    fn select_bandwidth<K: Kernel<P>>(&self, kernel: &K, points: &[P]) -> P::Bandwidth;
}
impl SelectBandwidth<(f64, f64)> for Matrix2 {
    fn select_bandwidth<K: Kernel<(f64, f64)>>(
        &self,
        _kernel: &K,
        _points: &[(f64, f64)],
    ) -> Matrix2 {
        self.clone()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SilvermanRot;
impl SelectBandwidth<f64> for SilvermanRot {
    fn select_bandwidth<K: Kernel<f64>>(&self, _kernel: &K, points: &[f64]) -> f64 {
        let n = points.len() as f64;
        let sd = stddev(points.iter().cloned());
        1.06 * sd * n.powf(-0.2)
    }
}
impl SelectBandwidth<MaybeUniform<f64>> for SilvermanRot {
    fn select_bandwidth<K: Kernel<MaybeUniform<f64>>>(
        &self,
        _kernel: &K,
        points0: &[MaybeUniform<f64>],
    ) -> f64 {
        let points = points0
            .iter()
            .filter_map(|x| {
                if let MaybeUniform::Sample(x) = *x {
                    Some(x)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let n = points.len() as f64;
        if let Some(MaybeUniform::Uniform(range)) = points0.iter().find(|x| {
            if let MaybeUniform::Uniform(_) = x {
                true
            } else {
                false
            }
        }) {
            // TODO
            return range.width() / points0.len() as f64;
        }

        let sd = stddev(points.into_iter());
        1.06 * sd * n.powf(-0.2)
    }
}
impl SelectBandwidth<(f64, f64)> for SilvermanRot {
    fn select_bandwidth<K: Kernel<(f64, f64)>>(
        &self,
        _kernel: &K,
        points: &[(f64, f64)],
    ) -> Matrix2 {
        let n = points.len() as f64;
        let sd0 = stddev(points.iter().map(|p| p.0));
        let sd1 = stddev(points.iter().map(|p| p.1));
        let a = (1.0 / n).powf(1.0 / 6.0);
        Matrix2::diagonal(a * sd0, a * sd1)
    }
}

#[derive(Debug)]
pub struct BayesianSelector {
    range: Range<(f64, f64, f64)>,
}
impl BayesianSelector {
    pub fn new(range: Range<(f64, f64)>) -> Self {
        let mut max = 0.0;
        if (range.high.0 - range.low.0) > max {
            max = range.high.0 - range.low.0;
        }
        if (range.high.1 - range.low.1) > max {
            max = range.high.1 - range.low.1;
        }
        let range = Range {
            low: (0.0, 0.0, 0.0),
            high: (max, max, max),
        };

        Self { range }
    }

    pub fn set_range(&mut self, range: Range<(f64, f64)>) {
        let mut max = 0.0;
        if (range.high.0 - range.low.0) > max {
            max = range.high.0 - range.low.0;
        }
        if (range.high.1 - range.low.1) > max {
            max = range.high.1 - range.low.1;
        }
        self.range = Range {
            low: (0.0, 0.0, 0.0),
            high: (max, max, max),
        };
    }
}
impl SelectBandwidth<(f64, f64)> for BayesianSelector {
    fn select_bandwidth<K: Kernel<(f64, f64)>>(
        &self,
        kernel: &K,
        points: &[(f64, f64)],
    ) -> Matrix2 {
        let mut rng = rand::thread_rng(); // TODO
        let iterations = 500; // TODO
        let posterior = BayesianPosterior::new(points, kernel);
        let mut sampler = SliceSampler3d::new(posterior, self.range);

        let b = Matrix2::diagonal(1.0, 1.0);
        sampler.set_last_point(b.lower_triangular_tuple());

        let mut bs = Vec::new();
        for _ in 0..iterations {
            let b = sampler.sample(&mut rng);
            bs.push(b);
        }

        let x = average(bs.iter().map(|t| t.0));
        let y = average(bs.iter().map(|t| t.1));
        let z = average(bs.iter().map(|t| t.2));

        let b = Matrix2::from_lower_triangular((x, y, z));
        let l = b.inverse();
        l * l.transpose()
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
impl<'a, K> Pdf<(f64, f64, f64)> for BayesianPosterior<'a, (f64, f64), K>
where
    K: Pdf<(f64, f64)>,
{
    fn pdf(&self, b: &(f64, f64, f64)) -> f64 {
        let v0 = self.prior_density(b.0) * self.prior_density(b.1) * self.prior_density(b.2);

        let b = Matrix2::from_lower_triangular(*b);
        let mut v1 = 1.0;
        for i in 0..self.points.len() {
            v1 *= self.likelihood(i, &b);
            if v1 == 0.0 {
                return 0.0;
            }
        }
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
