use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Matrix2 {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}
impl Matrix2 {
    pub fn new(r0: (f64, f64), r1: (f64, f64)) -> Self {
        Self {
            a: r0.0,
            b: r0.1,
            c: r1.0,
            d: r1.1,
        }
    }

    pub fn diagonal(a: f64, b: f64) -> Self {
        Self::new((a, 0.0), (0.0, b))
    }

    pub fn det(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    pub fn inverse(&self) -> Self {
        let det = self.det();
        Self {
            a: self.d / det,
            b: -self.b / det,
            c: -self.c / det,
            d: self.a / det,
        }
    }
}
impl Mul<(f64, f64)> for Matrix2 {
    type Output = (f64, f64);

    fn mul(self, rhs: (f64, f64)) -> Self::Output {
        let x = self.a * rhs.0 + self.b * rhs.1;
        let y = self.c * rhs.0 + self.d * rhs.1;
        (x, y)
    }
}

#[derive(Debug)]
pub struct Transpose<T>(pub T);
impl Mul<(f64, f64)> for Transpose<(f64, f64)> {
    type Output = f64;

    fn mul(self, rhs: (f64, f64)) -> Self::Output {
        (self.0).0 * rhs.0 + (self.0).1 * rhs.1
    }
}
