//! Hypothesis testings.
use crate::distributions::{Cdf as _, StandardNormal};
use crate::fundamental::average;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Group {
    X,
    Y,
}

/// Mann-Whitney U test.
#[derive(Debug)]
pub struct MannWhitneyU {
    xn: usize,
    yn: usize,
    counts: Vec<(usize, usize)>,
}
impl MannWhitneyU {
    /// Makes a new `MannWhitneyU` instance.
    pub fn new<X, Y, T>(xs: X, ys: Y) -> Self
    where
        X: Iterator<Item = T>,
        Y: Iterator<Item = T>,
        T: PartialOrd,
    {
        let mut vs = xs
            .map(|x| (x, Group::X))
            .chain(ys.map(|y| (y, Group::Y)))
            .collect::<Vec<_>>();
        vs.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal));

        let n = vs.len();
        let xn = vs.iter().filter(|t| t.1 == Group::X).count();
        let yn = n - xn;

        let mut counts = Vec::with_capacity(vs.len());
        let mut prev = None;
        for (v, group) in vs {
            if prev.as_ref() != Some(&v) {
                counts.push((0, 0));
            }
            if group == Group::X {
                counts.last_mut().unwrap_or_else(|| unreachable!()).0 += 1;
            } else {
                counts.last_mut().unwrap_or_else(|| unreachable!()).1 += 1;
            }
            prev = Some(v);
        }

        Self { xn, yn, counts }
    }

    /// Tests whether there is a statistically significant difference between `xs` and `ys`.
    ///
    /// # Panics
    ///
    /// `alpha` must be a positive number.
    pub fn test(&self, alpha: f64) -> bool {
        assert!(alpha > 0.0);
        self.p_value().map_or(false, |p| p < alpha)
    }

    /// Return the p-value that is the probability indicating there is a statistically significant difference between `xs` and `ys`.
    ///
    /// Note that if either `xs` or `ys` is empty, this method returns `None`.
    pub fn p_value(&self) -> Option<f64> {
        if self.xn < 1 || self.yn < 1 {
            None
        } else {
            let z = self.z();
            let p = (1.0 - StandardNormal.cdf(&z.abs())) * 2.0;
            Some(p)
        }
    }

    /// Returns `Ordering::Less` if `xs` is statistically less than `ys`, otherwise `Ordering::Greater`.
    ///
    /// If there is no statistically significant difference, this method returns `Ordering::Equal`.
    ///
    /// # Panics
    ///
    /// `alpha` must be a positive number.
    pub fn order(&self, alpha: f64) -> Ordering {
        assert!(alpha > 0.0);

        if !self.test(alpha) {
            Ordering::Equal
        } else {
            let (xu, yu) = self.xyu();
            if xu < yu {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }

    fn n(&self) -> usize {
        self.xn + self.yn
    }

    fn xyu(&self) -> (f64, f64) {
        let mut xr = 0.0;
        let mut rank = 1;
        for (x, y) in self.counts.iter().cloned() {
            xr += average((rank..).take(x + y).map(|x| x as f64)) * x as f64;
            rank += x + y;
        }
        let yr = (self.n() * (self.n() + 1) / 2) as f64 - xr;

        let xu = xr - (self.xn * (self.xn + 1) / 2) as f64;
        let yu = yr - (self.yn * (self.yn + 1) / 2) as f64;
        (xu, yu)
    }

    fn u(&self) -> f64 {
        let (xu, yu) = self.xyu();
        xu.min(yu)
    }

    fn mu(&self) -> f64 {
        ((self.xn * self.yn) / 2) as f64
    }

    fn au(&self) -> f64 {
        let t = self
            .counts
            .iter()
            .map(|&(x, y)| x + y)
            .map(|t| t * t * t - t)
            .sum::<usize>() as f64;
        let n = self.n() as f64;
        let n1 = self.xn as f64;
        let n2 = self.yn as f64;
        ((n1 * n2 * ((n + 1.0) - t / (n * (n - 1.0)))) / 12.0).sqrt()
    }

    fn z(&self) -> f64 {
        (self.u() - self.mu()) / self.au()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mann_whitney_u_works() {
        // See: http://sphweb.bumc.bu.edu/otlt/mph-modules/bs/bs704_nonparametric/BS704_Nonparametric4.html

        // Example-1
        let placebo = vec![7, 5, 6, 4, 12];
        let new_drug = vec![3, 6, 4, 2, 1];
        assert!(!MannWhitneyU::new(placebo.into_iter(), new_drug.into_iter()).test(0.05));

        // Example-2
        let usual_care = vec![8, 7, 6, 2, 5, 8, 7, 3];
        let new_program = vec![9, 9, 7, 8, 10, 9, 6];
        let mw = MannWhitneyU::new(usual_care.into_iter(), new_program.into_iter());
        assert!(mw.test(0.05));
        assert_eq!(mw.order(0.05), Ordering::Less);

        // Example-3
        let standard_therapy = vec![
            7500, 8000, 2000, 550, 1250, 1000, 2250, 6800, 3400, 6300, 9100, 970, 1040, 670, 400,
        ];
        let new_therapy = vec![
            400, 250, 800, 1400, 8000, 7400, 1020, 6000, 920, 1420, 2700, 4200, 5200, 4100,
        ];
        assert!(
            !MannWhitneyU::new(standard_therapy.into_iter(), new_therapy.into_iter()).test(0.05)
        );
    }
}
