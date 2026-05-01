use crate::statistics::{
    stats, stats_error::StatsError, stats_result::StatsResult, transformation,
};

#[derive(Clone, Default)]
pub struct Transformation;

impl Transformation {
    pub fn box_cox(&self, values: &[f64], lambda: f64) -> StatsResult<Vec<f64>> {
        transformation::box_cox(values, lambda)
    }
    pub fn detrend_linear(&self, values: &[f64]) -> StatsResult<Vec<f64>> {
        transformation::detrend_linear(values)
    }
    pub fn difference(&self, values: &[f64], order: usize) -> StatsResult<Vec<f64>> {
        transformation::difference(values, order)
    }
    pub fn lag(&self, values: &[f64], lag: usize) -> StatsResult<Vec<Option<f64>>> {
        transformation::lag(values, lag)
    }
    pub fn logarithm(&self, values: &[f64]) -> StatsResult<Vec<f64>> {
        transformation::logarithm(values)
    }
    pub fn rolling_mean(&self, values: &[f64], window: usize) -> StatsResult<Vec<f64>> {
        transformation::rolling_mean(values, window)
    }
    pub fn rolling_standard_deviation(
        &self,
        values: &[f64],
        window: usize,
    ) -> StatsResult<Vec<f64>> {
        transformation::rolling_standard_deviation(values, window)
    }
    pub fn seasonal_difference(&self, values: &[f64], period: usize) -> StatsResult<Vec<f64>> {
        transformation::seasonal_difference(values, period)
    }
}

pub fn box_cox(values: &[f64], lambda: f64) -> StatsResult<Vec<f64>> {
    if values.is_empty() {
        return Err(StatsError::EmptyInput.into());
    }
    let mut result = Vec::with_capacity(values.len());
    for &x in values {
        if x <= 0.0 {
            return Err(StatsError::DomainError {
                message: "Box-Cox requires positive values".into(),
            });
        }
        if lambda == 0.0 {
            result.push(x.ln());
        } else {
            result.push((x.powf(lambda) - 1.0) / lambda);
        }
    }
    Ok(result)
}

pub fn detrend_linear(values: &[f64]) -> StatsResult<Vec<f64>> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let mean_x = stats::mean(&x)?;
    let mean_y = stats::mean(values)?;
    let numerator: f64 = x
        .iter()
        .zip(values.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();
    let denominator: f64 = x
        .iter()
        .map(|xi| {
            let d = xi - mean_x;
            d * d
        })
        .sum();
    if denominator == 0.0 {
        return Err(StatsError::ZeroVariance.into());
    }
    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;
    let detrended: Vec<f64> = x
        .iter()
        .zip(values.iter())
        .map(|(xi, yi)| yi - (slope * xi + intercept))
        .collect();
    Ok(detrended)
}

pub fn difference(values: &[f64], order: usize) -> StatsResult<Vec<f64>> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    if order == 0 {
        return Ok(values.to_vec());
    }
    if order >= n {
        return Err(StatsError::InsufficientDegreesOfFreedom.into());
    }
    let mut result = values.to_vec();
    for _ in 0..order {
        result = result.windows(2).map(|w| w[1] - w[0]).collect();
    }
    Ok(result)
}

pub fn lag(values: &[f64], lag: usize) -> StatsResult<Vec<Option<f64>>> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        if i < lag {
            result.push(None);
        } else {
            result.push(Some(values[i - lag]));
        }
    }
    Ok(result)
}

pub fn logarithm(values: &[f64]) -> StatsResult<Vec<f64>> {
    if values.is_empty() {
        return Err(StatsError::EmptyInput.into());
    }
    let mut result = Vec::with_capacity(values.len());
    for &x in values {
        if x <= 0.0 {
            return Err(StatsError::DomainError {
                message: "log undefined for non-positive values".into(),
            });
        }
        result.push(x.ln());
    }
    Ok(result)
}

pub fn rolling_mean(values: &[f64], window: usize) -> StatsResult<Vec<f64>> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    if window == 0 || window > n {
        return Err(StatsError::InvalidLag.into());
    }
    let result: Vec<f64> = values
        .windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect();
    Ok(result)
}

pub fn rolling_standard_deviation(values: &[f64], window: usize) -> StatsResult<Vec<f64>> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    if window == 0 || window > n {
        return Err(StatsError::InvalidLag.into());
    }
    let mut result = Vec::with_capacity(n - window + 1);
    for w in values.windows(window) {
        let var = stats::variance(w, 0)?;
        result.push(var.sqrt());
    }
    Ok(result)
}

pub fn seasonal_difference(values: &[f64], period: usize) -> StatsResult<Vec<f64>> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    if period == 0 || period >= n {
        return Err(StatsError::InvalidLag.into());
    }
    let result: Vec<f64> = (period..n)
        .map(|i| values[i] - values[i - period])
        .collect();
    Ok(result)
}
