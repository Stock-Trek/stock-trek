use crate::statistics::{
    hypothesis, stats, stats_error::StatsError, stats_result::StatsResult, time_series,
};

#[derive(Clone, Default)]
pub struct Hypothesis;

impl Hypothesis {
    pub fn augmented_dickey_fuller(
        &self,
        time_series_values: &[f64],
        maximum_lag: usize,
    ) -> StatsResult<(f64, f64)> {
        hypothesis::augmented_dickey_fuller(time_series_values, maximum_lag)
    }

    pub fn durbin_watson(&self, residual_values: &[f64]) -> StatsResult<f64> {
        hypothesis::durbin_watson(residual_values)
    }

    pub fn jarque_bera(&self, time_series_values: &[f64]) -> StatsResult<(f64, f64)> {
        hypothesis::jarque_bera(time_series_values)
    }

    pub fn kwiatkowski_phillips_schmidt_shin(
        &self,
        time_series_values: &[f64],
    ) -> StatsResult<(f64, f64)> {
        hypothesis::kwiatkowski_phillips_schmidt_shin(time_series_values)
    }

    pub fn ljung_box(&self, time_series_values: &[f64], number_of_lags: usize) -> StatsResult<f64> {
        hypothesis::ljung_box(time_series_values, number_of_lags)
    }
}

pub fn augmented_dickey_fuller(
    time_series_values: &[f64],
    maximum_lag: usize,
) -> StatsResult<(f64, f64)> {
    let n = time_series_values.len();
    if n <= maximum_lag + 1 {
        return Err(StatsError::InvalidParameters);
    }
    // Δy_t
    let mut diff = Vec::with_capacity(n - 1);
    for i in 1..n {
        diff.push(time_series_values[i] - time_series_values[i - 1]);
    }
    // simple regression: Δy_t = α + β y_{t-1}
    let mut y_lag = Vec::new();
    let mut dy = Vec::new();
    for t in maximum_lag..diff.len() {
        y_lag.push(time_series_values[t]);
        dy.push(diff[t]);
    }
    let mean_x = stats::mean(&y_lag)?;
    let mean_y = stats::mean(&dy)?;
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..y_lag.len() {
        num += (y_lag[i] - mean_x) * (dy[i] - mean_y);
        den += (y_lag[i] - mean_x).powi(2);
    }
    if den == 0.0 {
        return Err(StatsError::DivisionByZero);
    }
    let beta = num / den;
    // crude t-stat approximation
    let stat = beta;
    // placeholder p-value (no distribution calc)
    let p_value = (-stat.abs()).exp();
    Ok((stat, p_value))
}

pub fn durbin_watson(residual_values: &[f64]) -> StatsResult<f64> {
    let n = residual_values.len();
    if n < 2 {
        return Err(StatsError::InvalidParameters);
    }
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 1..n {
        num += (residual_values[i] - residual_values[i - 1]).powi(2);
    }
    for &e in residual_values {
        den += e.powi(2);
    }
    if den == 0.0 {
        return Err(StatsError::DivisionByZero);
    }
    Ok(num / den)
}

pub fn jarque_bera(time_series_values: &[f64]) -> StatsResult<(f64, f64)> {
    let n = time_series_values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    let mu = stats::mean(time_series_values)?;
    let var = stats::variance(time_series_values, 0)?;
    if var == 0.0 {
        return Err(StatsError::DivisionByZero);
    }
    let std = var.sqrt();
    let mut skew = 0.0;
    let mut kurt = 0.0;
    for &x in time_series_values {
        let z = (x - mu) / std;
        skew += z.powi(3);
        kurt += z.powi(4);
    }
    skew /= n as f64;
    kurt /= n as f64;
    let jb = (n as f64 / 6.0) * (skew.powi(2) + 0.25 * (kurt - 3.0).powi(2));
    let p_value = (-0.5 * jb).exp();
    Ok((jb, p_value))
}

pub fn kwiatkowski_phillips_schmidt_shin(time_series_values: &[f64]) -> StatsResult<(f64, f64)> {
    let n = time_series_values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    let mu = stats::mean(time_series_values)?;
    let mut cumulative = Vec::with_capacity(n);
    let mut sum = 0.0;
    for &x in time_series_values {
        sum += x - mu;
        cumulative.push(sum);
    }
    let eta = cumulative.iter().map(|x| x.powi(2)).sum::<f64>() / (n as f64 * n as f64);
    let var = stats::variance(time_series_values, 0)?;
    if var == 0.0 {
        return Err(StatsError::DivisionByZero);
    }
    let stat = eta / var;
    let p_value = (-stat).exp();
    Ok((stat, p_value))
}

pub fn ljung_box(time_series_values: &[f64], number_of_lags: usize) -> StatsResult<f64> {
    let n = time_series_values.len();
    if n == 0 || number_of_lags == 0 || number_of_lags >= n {
        return Err(StatsError::InvalidParameters);
    }
    let var = stats::variance(time_series_values, 0)?;
    if var == 0.0 {
        return Err(StatsError::DivisionByZero);
    }
    let mut q = 0.0;
    for k in 1..=number_of_lags {
        let rho = time_series::autocorrelation(time_series_values, k)?;
        q += rho.powi(2) / (n as f64 - k as f64);
    }
    Ok(n as f64 * (n as f64 + 2.0) * q)
}
