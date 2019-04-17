pub use self::normal::StandardNormal;

pub mod kde;

mod normal;

pub trait Pdf<T> {
    fn pdf(&self, x: &T) -> f64;
}

// This can be regarded as an unnormalized conditional probability distribution.
#[derive(Debug)]
pub struct FixY<D> {
    inner: D,
    y: f64,
}
impl<D> FixY<D>
where
    D: Pdf<(f64, f64)>,
{
    pub fn new(inner: D, y: f64) -> Self {
        FixY { inner, y }
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn inner(&self) -> &D {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut D {
        &mut self.inner
    }
}
impl<D> Pdf<f64> for FixY<D>
where
    D: Pdf<(f64, f64)>,
{
    fn pdf(&self, &x: &f64) -> f64 {
        // NOTE: incomplete PDF (The sum is less than 1.0)
        self.inner.pdf(&(x, self.y))
    }
}
