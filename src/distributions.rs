pub use self::normal::StdNormal;

pub mod kde;

mod normal;

pub trait Pdf<T> {
    fn pdf(&self, x: &T) -> f64;
}
