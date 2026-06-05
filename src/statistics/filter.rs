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

/// Apply the Hodrick-Prescott filter using a direct tridiagonal matrix solve
/// via the Thomas algorithm (TDMA).
///
/// Solves `(I + λ * DᵀD) * trend = y` where `D` is the second-difference matrix.
/// This produces the exact trend in O(n) time without iteration.
///
/// The tridiagonal system is:
///   (1+λ) * trend[0]     - λ * trend[1]                                    = y[0]
///   -λ * trend[t-1]  + (1+2λ) * trend[t]  - λ * trend[t+1]                 = y[t]    (1 ≤ t ≤ n-2)
///   -λ * trend[n-2]  + (1+λ) * trend[n-1]                                  = y[n-1]
pub fn hodrick_prescott_filter(
    time_series_values: &[f64],
    lambda: f64,
) -> (Vec<f64>, Vec<f64>) {
    let n = time_series_values.len();
    if n < 3 {
        let trend = time_series_values.to_vec();
        let cyclical = vec![0.0; n];
        return (trend, cyclical);
    }

    // Thomas algorithm (TDMA) for the tridiagonal system.
    //
    // Sub-diagonal (a[i]): coefficient of trend[i-1], i = 1..n-1
    // Main diagonal (b[i]): coefficient of trend[i],   i = 0..n-1
    // Super-diagonal (c[i]): coefficient of trend[i+1], i = 0..n-2
    // RHS (d[i]): y[i]

    let neg_lambda = -lambda;
    let one_plus_2lambda = 1.0 + 2.0 * lambda;
    let one_plus_lambda = 1.0 + lambda;

    // Slices for the tridiagonal coefficients.
    // To avoid per-element allocations, we operate on vectors.
    let mut c_star = vec![0.0; n]; // modified super-diagonal for forward sweep
    let mut d_star = vec![0.0; n]; // modified RHS for forward sweep
    let mut trend = vec![0.0; n];

    // Forward sweep
    // For i = 0 (first row): b[0] = 1+λ, c[0] = -λ, d[0] = y[0]
    c_star[0] = neg_lambda / one_plus_lambda;
    d_star[0] = time_series_values[0] / one_plus_lambda;

    for i in 1..n - 1 {
        // Row i: a[i] = -λ, b[i] = 1+2λ, c[i] = -λ, d[i] = y[i]
        let a_i = neg_lambda;
        let b_i = one_plus_2lambda;
        let denom = b_i + a_i * c_star[i - 1];
        c_star[i] = neg_lambda / denom;
        d_star[i] = (time_series_values[i] - a_i * d_star[i - 1]) / denom;
    }

    // Last row i = n-1: a[n-1] = -λ, b[n-1] = 1+λ, c[n-1] = 0 (unused), d[n-1] = y[n-1]
    {
        let i = n - 1;
        let a_i = neg_lambda;
        let b_i = one_plus_lambda;
        let denom = b_i + a_i * c_star[i - 1];
        d_star[i] = (time_series_values[i] - a_i * d_star[i - 1]) / denom;
    }

    // Back substitution
    trend[n - 1] = d_star[n - 1];
    for i in (0..n - 1).rev() {
        trend[i] = d_star[i] - c_star[i] * trend[i + 1];
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
        let noise = var;
        let signal = (var - noise).max(0.0);
        let value = mean + (signal / (signal + noise + 1e-12)) * (time_series_values[i] - mean);
        result.push(value);
    }
    result
}
