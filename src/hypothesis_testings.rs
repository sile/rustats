use crate::distributions::{Cdf as _, StandardNormal};
use crate::fundamental::average;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Alpha {
    /// 0.01
    P01,

    /// 0.05
    P05,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Group {
    X,
    Y,
}

#[derive(Debug)]
pub struct MannWhitneyU {
    xn: usize,
    yn: usize,
    counts: Vec<(usize, usize)>,
}
impl MannWhitneyU {
    pub fn new<X, Y, T>(xs: X, ys: Y) -> Self
    where
        X: Iterator<Item = T>,
        Y: Iterator<Item = T>,
        T: Ord,
    {
        let mut vs = xs
            .map(|x| (x, Group::X))
            .chain(ys.map(|y| (y, Group::Y)))
            .collect::<Vec<_>>();
        vs.sort();

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

    pub fn test(&self, alpha: Alpha) -> bool {
        if self.xn < 1 || self.yn < 1 {
            return false;
        }

        if self.xn <= 20 && self.yn <= 20 {
            let critical = match alpha {
                Alpha::P01 => TWO_TAILED_CRITICAL_VALUES_P001[self.xn - 1][self.yn - 1],
                Alpha::P05 => TWO_TAILED_CRITICAL_VALUES_P005[self.xn - 1][self.yn - 1],
            };
            return self.u() <= critical as f64;
        }

        let z = self.z();
        let p = (1.0 - StandardNormal.cdf(&z.abs())) * 2.0;
        match alpha {
            Alpha::P01 => p < 0.01,
            Alpha::P05 => p < 0.05,
        }
    }

    pub fn order(&self, alpha: Alpha) -> Ordering {
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

const TWO_TAILED_CRITICAL_VALUES_P005: [[i8; 20]; 20] = [
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    [
        -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2,
    ],
    [
        -1, -1, -1, -1, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8,
    ],
    [
        -1, -1, -1, 0, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 11, 11, 12, 13, 13,
    ],
    [
        -1, -1, 0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 17, 18, 19, 20,
    ],
    [
        -1, -1, 1, 2, 3, 5, 6, 8, 10, 11, 13, 14, 16, 17, 19, 21, 22, 24, 25, 27,
    ],
    [
        -1, -1, 1, 3, 5, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34,
    ],
    [
        -1, 0, 2, 4, 6, 8, 10, 13, 15, 17, 19, 22, 24, 26, 29, 31, 34, 36, 38, 41,
    ],
    [
        -1, 0, 2, 4, 7, 10, 12, 15, 17, 21, 23, 26, 28, 31, 34, 37, 39, 42, 45, 48,
    ],
    [
        -1, 0, 3, 5, 8, 11, 14, 17, 20, 23, 26, 29, 33, 36, 39, 42, 45, 48, 52, 55,
    ],
    [
        -1, 0, 3, 6, 9, 13, 16, 19, 23, 26, 30, 33, 37, 40, 44, 47, 51, 55, 58, 62,
    ],
    [
        -1, 1, 4, 7, 11, 14, 18, 22, 26, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65, 69,
    ],
    [
        -1, 1, 4, 8, 12, 16, 20, 24, 28, 33, 37, 41, 45, 50, 54, 59, 63, 67, 72, 76,
    ],
    [
        -1, 1, 5, 9, 13, 17, 22, 26, 31, 36, 40, 45, 50, 55, 59, 64, 67, 74, 78, 83,
    ],
    [
        -1, 1, 5, 10, 14, 19, 24, 29, 34, 39, 44, 49, 54, 59, 64, 70, 75, 80, 85, 90,
    ],
    [
        -1, 1, 6, 11, 15, 21, 26, 31, 37, 42, 47, 53, 59, 64, 70, 75, 81, 86, 92, 98,
    ],
    [
        -1, 2, 6, 11, 17, 22, 28, 34, 39, 45, 51, 57, 63, 67, 75, 81, 87, 93, 99, 105,
    ],
    [
        -1, 2, 7, 12, 18, 24, 30, 36, 42, 48, 55, 61, 67, 74, 80, 86, 93, 99, 106, 112,
    ],
    [
        -1, 2, 7, 13, 19, 25, 32, 38, 45, 52, 58, 65, 72, 78, 85, 92, 99, 106, 113, 119,
    ],
    [
        -1, 2, 8, 14, 20, 27, 34, 41, 48, 55, 62, 69, 76, 83, 90, 98, 105, 112, 119, 127,
    ],
];

const TWO_TAILED_CRITICAL_VALUES_P001: [[i8; 20]; 20] = [
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0,
    ],
    [
        -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3,
    ],
    [
        -1, -1, -1, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8,
    ],
    [
        -1, -1, -1, -1, 0, 1, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10, 11, 12, 13,
    ],
    [
        -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 15, 16, 17, 18,
    ],
    [
        -1, -1, -1, 0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22, 24,
    ],
    [
        -1, -1, -1, 1, 2, 4, 6, 7, 9, 11, 13, 15, 17, 18, 20, 22, 24, 26, 28, 30,
    ],
    [
        -1, -1, 0, 1, 3, 5, 7, 9, 11, 13, 16, 18, 20, 22, 24, 27, 29, 31, 33, 36,
    ],
    [
        -1, -1, 0, 2, 4, 6, 9, 11, 13, 16, 18, 21, 24, 26, 29, 31, 34, 37, 39, 42,
    ],
    [
        -1, -1, 0, 2, 5, 7, 10, 13, 16, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 46,
    ],
    [
        -1, -1, 1, 3, 6, 9, 12, 15, 18, 21, 24, 27, 31, 34, 37, 41, 44, 47, 51, 54,
    ],
    [
        -1, -1, 1, 3, 7, 10, 13, 17, 20, 24, 27, 31, 34, 38, 42, 45, 49, 53, 56, 60,
    ],
    [
        -1, -1, 1, 4, 7, 11, 15, 18, 22, 26, 30, 34, 38, 42, 46, 50, 54, 58, 63, 67,
    ],
    [
        -1, -1, 2, 5, 8, 12, 16, 20, 24, 29, 33, 37, 42, 46, 51, 55, 60, 64, 69, 73,
    ],
    [
        -1, -1, 2, 5, 9, 13, 18, 22, 27, 31, 36, 41, 45, 50, 55, 60, 65, 70, 74, 79,
    ],
    [
        -1, -1, 2, 6, 10, 15, 19, 24, 29, 34, 39, 44, 49, 54, 60, 65, 70, 75, 81, 86,
    ],
    [
        -1, -1, 2, 6, 11, 16, 21, 26, 31, 37, 42, 47, 53, 58, 64, 70, 75, 81, 87, 92,
    ],
    [
        -1, 0, 3, 7, 12, 17, 22, 28, 33, 39, 45, 51, 56, 63, 69, 74, 81, 87, 93, 99,
    ],
    [
        -1, 0, 3, 8, 13, 18, 24, 30, 36, 42, 46, 54, 60, 67, 73, 79, 86, 92, 99, 105,
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mann_whitney_u_works() {
        // See: http://sphweb.bumc.bu.edu/otlt/mph-modules/bs/bs704_nonparametric/BS704_Nonparametric4.html

        // Example-1
        let placebo = vec![7, 5, 6, 4, 12];
        let new_drug = vec![3, 6, 4, 2, 1];
        assert!(!MannWhitneyU::new(placebo.into_iter(), new_drug.into_iter()).test(Alpha::P05));

        // Example-2
        let usual_care = vec![8, 7, 6, 2, 5, 8, 7, 3];
        let new_program = vec![9, 9, 7, 8, 10, 9, 6];
        let mw = MannWhitneyU::new(usual_care.into_iter(), new_program.into_iter());
        assert!(mw.test(Alpha::P05));
        assert_eq!(mw.order(Alpha::P05), Ordering::Less);

        // Example-3
        let standard_therapy = vec![
            7500, 8000, 2000, 550, 1250, 1000, 2250, 6800, 3400, 6300, 9100, 970, 1040, 670, 400,
        ];
        let new_therapy = vec![
            400, 250, 800, 1400, 8000, 7400, 1020, 6000, 920, 1420, 2700, 4200, 5200, 4100,
        ];
        assert!(
            !MannWhitneyU::new(standard_therapy.into_iter(), new_therapy.into_iter())
                .test(Alpha::P05)
        );
    }
}
