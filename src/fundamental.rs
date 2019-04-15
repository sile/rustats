pub fn stddev<I>(xs: I) -> f64
where
    I: ExactSizeIterator<Item = f64> + Clone,
{
    let n = xs.len() as f64;
    let sum = xs.clone().into_iter().sum::<f64>();
    let avg = sum / n;
    let var = xs.into_iter().map(|x| (x - avg).powi(2)).sum::<f64>() / n;
    var.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stddev_works() {
        assert_eq!(
            stddev([71.0, 80.0, 89.0].iter().cloned()),
            7.3484692283495345
        );
        assert_eq!(
            stddev([77.0, 80.0, 83.0].iter().cloned()),
            2.449489742783178
        );
    }
}
