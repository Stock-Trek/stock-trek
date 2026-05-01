use crate::statistics::{
    advanced::Advanced, decompose::Decompose, evaluation::Evaluation,
    exponential_smoothing::ExponentialSmoothing, filter::Filter, frequency::Frequency,
    hypothesis::Hypothesis, moving_average::MovingAverage, stats, stats_error::StatsError,
    stats_result::StatsResult, time_series::TimeSeries, transformation::Transformation,
    wavelet::Wavelet,
};

#[derive(Clone, Default)]
pub struct Stats {
    pub advanced: Advanced,
    pub decompose: Decompose,
    pub evaluation: Evaluation,
    pub exponential_smoothing: ExponentialSmoothing,
    pub filter: Filter,
    pub frequency: Frequency,
    pub hypothesis: Hypothesis,
    pub moving_average: MovingAverage,
    pub time_series: TimeSeries,
    pub transformation: Transformation,
    pub wavelet: Wavelet,
}

impl Stats {
    pub fn mean(&self, values: &[f64]) -> StatsResult<f64> {
        stats::mean(values)
    }
    pub fn variance(&self, values: &[f64], delta_degrees_of_freedom: usize) -> StatsResult<f64> {
        stats::variance(values, delta_degrees_of_freedom)
    }
    pub fn standard_deviation(
        &self,
        values: &[f64],
        delta_degrees_of_freedom: usize,
    ) -> StatsResult<f64> {
        stats::standard_deviation(values, delta_degrees_of_freedom)
    }
    pub fn covariance(&self, first: &[f64], second: &[f64]) -> StatsResult<f64> {
        stats::covariance(first, second)
    }
    pub fn correlation(&self, first: &[f64], second: &[f64]) -> StatsResult<f64> {
        stats::correlation(first, second)
    }
    pub fn skewness(&self, values: &[f64]) -> StatsResult<f64> {
        stats::skewness(values)
    }
    pub fn kurtosis(&self, values: &[f64]) -> StatsResult<f64> {
        stats::kurtosis(values)
    }
}

pub fn mean(values: &[f64]) -> StatsResult<f64> {
    if values.is_empty() {
        return Err(StatsError::EmptyInput);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Ok(mean)
}

pub fn variance(values: &[f64], delta_degrees_of_freedom: usize) -> StatsResult<f64> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    if n <= delta_degrees_of_freedom {
        return Err(StatsError::InsufficientDegreesOfFreedom);
    }
    let mu = stats::mean(values)?;
    let sum_sq: f64 = values
        .iter()
        .map(|x| {
            let d = x - mu;
            d * d
        })
        .sum();
    let variance = sum_sq / (n - delta_degrees_of_freedom) as f64;
    Ok(variance)
}

pub fn standard_deviation(values: &[f64], delta_degrees_of_freedom: usize) -> StatsResult<f64> {
    let standard_deviation_sq = stats::variance(values, delta_degrees_of_freedom)?;
    let standard_deviation = standard_deviation_sq.sqrt();
    Ok(standard_deviation)
}

pub fn covariance(first: &[f64], second: &[f64]) -> StatsResult<f64> {
    let n = first.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    if n != second.len() {
        return Err(StatsError::MismatchedLengths);
    }
    let mean_x = stats::mean(first)?;
    let mean_y = stats::mean(second)?;
    let cov: f64 = first
        .iter()
        .zip(second.iter())
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();
    let covariance = cov / n as f64;
    Ok(covariance)
}

pub fn correlation(first: &[f64], second: &[f64]) -> StatsResult<f64> {
    let cov = stats::covariance(first, second)?;
    let std_x = stats::standard_deviation(first, 0)?;
    let std_y = stats::standard_deviation(second, 0)?;
    if std_x == 0.0 || std_y == 0.0 {
        return Err(StatsError::ZeroVariance);
    }
    let correlation = cov / (std_x * std_y);
    Ok(correlation)
}

pub fn skewness(values: &[f64]) -> StatsResult<f64> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    let mu = stats::mean(values)?;
    let std = stats::standard_deviation(values, 0)?;
    if std == 0.0 {
        return Err(StatsError::ZeroVariance);
    }
    let m3: f64 = values
        .iter()
        .map(|x| {
            let z = (x - mu) / std;
            z * z * z
        })
        .sum::<f64>()
        / n as f64;
    Ok(m3)
}

pub fn kurtosis(values: &[f64]) -> StatsResult<f64> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput);
    }
    let mu = stats::mean(values)?;
    let std = stats::standard_deviation(values, 0)?;
    if std == 0.0 {
        return Err(StatsError::ZeroVariance);
    }
    let m4: f64 = values
        .iter()
        .map(|x| {
            let z = (x - mu) / std;
            z * z * z * z
        })
        .sum::<f64>()
        / n as f64;
    Ok(m4) // subtract 3.0 externally if you want excess kurtosis
}
