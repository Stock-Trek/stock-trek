use crate::statistics::filter;

#[derive(Clone, Default)]
pub struct Filter;

impl Filter {
    pub fn hodrick_prescott_filter(
        &self,
        time_series_values: &[f64],
        lambda: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        filter::hodrick_prescott_filter(time_series_values, lambda)
    }
    pub fn wiener_filter(&self, time_series_values: &[f64], window_size: usize) -> Vec<f64> {
        filter::wiener_filter(time_series_values, window_size)
    }
}

pub fn hodrick_prescott_filter(time_series_values: &[f64], lambda: f64) -> (Vec<f64>, Vec<f64>) {
    let n = time_series_values.len();
    let mut trend = time_series_values.to_vec();
    if n < 3 {
        return (trend.clone(), vec![0.0; n]);
    }
    for _ in 0..100 {
        for t in 1..n - 1 {
            trend[t] = (time_series_values[t] + lambda * (trend[t - 1] + trend[t + 1]))
                / (1.0 + 2.0 * lambda);
        }
    }
    let cyclical: Vec<f64> = time_series_values
        .iter()
        .zip(trend.iter())
        .map(|(x, t)| x - t)
        .collect();
    (trend, cyclical)
}

pub fn wiener_filter(time_series_values: &[f64], window_size: usize) -> Vec<f64> {
    let n = time_series_values.len();
    let mut result = Vec::with_capacity(n);
    if n == 0 || window_size == 0 {
        return result;
    }
    for i in 0..n {
        let start = i.saturating_sub(window_size / 2);
        let end = usize::min(n, i + window_size / 2 + 1);
        let window = &time_series_values[start..end];
        let mean = window.iter().sum::<f64>() / window.len() as f64;
        let var = window
            .iter()
            .map(|x| {
                let d = x - mean;
                d * d
            })
            .sum::<f64>()
            / window.len() as f64;
        // Estimate noise variance from local differences (simple noise floor)
        let noise = window
            .windows(2)
            .map(|w| (w[1] - w[0]) * (w[1] - w[0]))
            .sum::<f64>()
            / (window.len().max(1) - 1).max(1) as f64;
        let signal = (var - noise).max(0.0);
        let value = mean + (signal / (signal + noise + 1e-12)) * (time_series_values[i] - mean);
        result.push(value);
    }
    result
}
