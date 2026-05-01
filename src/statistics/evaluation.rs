use crate::statistics::{evaluation, stats_error::StatsError, stats_result::StatsResult};

#[derive(Clone, Default)]
pub struct Evaluation;

impl Evaluation {
    pub fn akaike_information_criterion(
        &self,
        log_likelihood_value: f64,
        number_of_parameters: usize,
    ) -> StatsResult<f64> {
        evaluation::akaike_information_criterion(log_likelihood_value, number_of_parameters)
    }
    pub fn bayesian_information_criterion(
        &self,
        log_likelihood_value: f64,
        number_of_parameters: usize,
        number_of_observations: usize,
    ) -> StatsResult<f64> {
        evaluation::bayesian_information_criterion(
            log_likelihood_value,
            number_of_parameters,
            number_of_observations,
        )
    }
    pub fn log_likelihood(
        &self,
        model_parameters: &[f64],
        observed_data: &[f64],
    ) -> StatsResult<f64> {
        evaluation::log_likelihood(model_parameters, observed_data)
    }
    pub fn mean_absolute_error(
        &self,
        true_values: &[f64],
        predicted_values: &[f64],
    ) -> StatsResult<f64> {
        evaluation::mean_absolute_error(true_values, predicted_values)
    }
    pub fn mean_absolute_percentage_error(
        &self,
        true_values: &[f64],
        predicted_values: &[f64],
    ) -> StatsResult<f64> {
        evaluation::mean_absolute_percentage_error(true_values, predicted_values)
    }
    pub fn mean_squared_error(
        &self,
        true_values: &[f64],
        predicted_values: &[f64],
    ) -> StatsResult<f64> {
        evaluation::mean_squared_error(true_values, predicted_values)
    }
    pub fn root_mean_squared_error(
        &self,
        true_values: &[f64],
        predicted_values: &[f64],
    ) -> StatsResult<f64> {
        evaluation::root_mean_squared_error(true_values, predicted_values)
    }
}

pub fn akaike_information_criterion(
    log_likelihood_value: f64,
    number_of_parameters: usize,
) -> StatsResult<f64> {
    if number_of_parameters == 0 {
        return Err(StatsError::InvalidParameters);
    }
    let k = number_of_parameters as f64;
    Ok(2.0 * k - 2.0 * log_likelihood_value)
}

pub fn bayesian_information_criterion(
    log_likelihood_value: f64,
    number_of_parameters: usize,
    number_of_observations: usize,
) -> StatsResult<f64> {
    if number_of_parameters == 0 || number_of_observations == 0 {
        return Err(StatsError::InvalidParameters);
    }
    let k = number_of_parameters as f64;
    let n = number_of_observations as f64;
    Ok(k * n.ln() - 2.0 * log_likelihood_value)
}

pub fn log_likelihood(model_parameters: &[f64], observed_data: &[f64]) -> StatsResult<f64> {
    if model_parameters.len() < 2 {
        return Err(StatsError::InvalidParameters);
    }
    if observed_data.is_empty() {
        return Err(StatsError::EmptyInput);
    }
    let mu = model_parameters[0];
    let variance = model_parameters[1];
    if variance <= 0.0 {
        return Err(StatsError::InvalidParameters);
    }
    let n = observed_data.len() as f64;
    let log_term = -0.5 * n * (2.0 * std::f64::consts::PI * variance).ln();
    let sum_sq = observed_data
        .iter()
        .map(|x| {
            let diff = x - mu;
            diff * diff
        })
        .sum::<f64>();
    Ok(log_term - (sum_sq / (2.0 * variance)))
}

pub fn mean_absolute_error(true_values: &[f64], predicted_values: &[f64]) -> StatsResult<f64> {
    if true_values.len() != predicted_values.len() {
        return Err(StatsError::MismatchedLengths);
    }
    if true_values.is_empty() {
        return Err(StatsError::EmptyInput);
    }
    let mae = true_values
        .iter()
        .zip(predicted_values.iter())
        .map(|(t, p)| (t - p).abs())
        .sum::<f64>()
        / true_values.len() as f64;
    Ok(mae)
}

pub fn mean_absolute_percentage_error(
    true_values: &[f64],
    predicted_values: &[f64],
) -> StatsResult<f64> {
    if true_values.len() != predicted_values.len() {
        return Err(StatsError::MismatchedLengths);
    }
    if true_values.is_empty() {
        return Err(StatsError::EmptyInput);
    }
    let mut count = 0usize;
    let sum = true_values
        .iter()
        .zip(predicted_values.iter())
        .filter_map(|(t, p)| {
            if *t == 0.0 {
                None
            } else {
                count += 1;
                Some((t - p).abs() / t.abs())
            }
        })
        .sum::<f64>();
    if count == 0 {
        return Err(StatsError::DivisionByZero);
    }
    Ok(sum / count as f64)
}

pub fn mean_squared_error(true_values: &[f64], predicted_values: &[f64]) -> StatsResult<f64> {
    if true_values.len() != predicted_values.len() {
        return Err(StatsError::MismatchedLengths);
    }
    if true_values.is_empty() {
        return Err(StatsError::EmptyInput);
    }
    let mse = true_values
        .iter()
        .zip(predicted_values.iter())
        .map(|(t, p)| {
            let diff = t - p;
            diff * diff
        })
        .sum::<f64>()
        / true_values.len() as f64;
    Ok(mse)
}

pub fn root_mean_squared_error(true_values: &[f64], predicted_values: &[f64]) -> StatsResult<f64> {
    Ok(evaluation::mean_squared_error(true_values, predicted_values)?.sqrt())
}
