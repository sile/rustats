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
