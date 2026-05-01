use crate::statistics::{stats, stats_error::StatsError, stats_result::StatsResult, time_series};

#[derive(Clone, Default)]
pub struct TimeSeries;

impl TimeSeries {
    pub fn autocorrelation(&self, values: &[f64], lag: usize) -> StatsResult<f64> {
        time_series::autocorrelation(values, lag)
    }
    pub fn autocovariance(&self, values: &[f64], lag: usize) -> StatsResult<f64> {
        time_series::autocovariance(values, lag)
    }
    pub fn cross_correlation(&self, first: &[f64], second: &[f64], lag: isize) -> StatsResult<f64> {
        time_series::cross_correlation(first, second, lag)
    }
    pub fn partial_autocorrelation(&self, values: &[f64], max_lag: usize) -> StatsResult<Vec<f64>> {
        time_series::partial_autocorrelation(values, max_lag)
    }
}

pub fn autocorrelation(values: &[f64], lag: usize) -> StatsResult<f64> {
    let gamma_0 = time_series::autocovariance(values, 0)?;
    if gamma_0 == 0.0 {
        return Err(StatsError::ZeroVariance);
    }
    let gamma_k = time_series::autocovariance(values, lag)?;
    let autocorrelation = gamma_k / gamma_0;
    Ok(autocorrelation)
}

pub fn autocovariance(values: &[f64], lag: usize) -> StatsResult<f64> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    if lag >= n {
        return Err(StatsError::InvalidLag);
    }
    let mu = stats::mean(values)?;
    let sum: f64 = (lag..n)
        .map(|t| (values[t] - mu) * (values[t - lag] - mu))
        .sum();
    let autocovariance = sum / n as f64;
    Ok(autocovariance)
}

pub fn cross_correlation(first: &[f64], second: &[f64], lag: isize) -> StatsResult<f64> {
    let n = first.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    if n != second.len() {
        return Err(StatsError::MismatchedLengths);
    }
    let mean_x = stats::mean(first)?;
    let mean_y = stats::mean(second)?;
    let numerator: f64 = (0..n)
        .filter_map(|t| {
            let j = t as isize - lag;
            if j >= 0 && (j as usize) < n {
                Some((first[t] - mean_x) * (second[j as usize] - mean_y))
            } else {
                None
            }
        })
        .sum();
    let var_x = stats::variance(first, 0)?;
    let var_y = stats::variance(second, 0)?;
    let cross_correlation = numerator / (n as f64 * (var_x.sqrt() * var_y.sqrt()));
    Ok(cross_correlation)
}

pub fn partial_autocorrelation(values: &[f64], max_lag: usize) -> StatsResult<Vec<f64>> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    if max_lag >= n {
        return Err(StatsError::InvalidLag);
    }
    // Precompute ACF
    let acf = (0..=max_lag)
        .map(|k| time_series::autocorrelation(values, k))
        .collect::<Result<Vec<f64>, _>>()?;
    // Durbin–Levinson recursion
    let mut pacf = vec![0.0; max_lag + 1];
    let mut phi = vec![vec![0.0; max_lag + 1]; max_lag + 1];
    pacf[0] = 1.0;
    if max_lag >= 1 {
        phi[1][1] = acf[1];
        pacf[1] = acf[1];
    }
    for k in 2..=max_lag {
        let sum: f64 = (1..k).map(|j| phi[k - 1][j] * acf[k - j]).sum();
        let denom: f64 = 1.0 - (1..k).map(|j| phi[k - 1][j] * acf[j]).sum::<f64>();
        if denom == 0.0 {
            return Err(StatsError::ZeroVariance);
        }
        let phi_kk = (acf[k] - sum) / denom;
        phi[k][k] = phi_kk;
        for j in 1..k {
            phi[k][j] = phi[k - 1][j] - phi_kk * phi[k - 1][k - j];
        }
        pacf[k] = phi_kk;
    }
    Ok(pacf)
}
