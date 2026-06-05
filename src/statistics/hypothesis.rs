use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    statistics::{hypothesis, stats, time_series},
};

#[derive(Clone, Default)]
pub struct Hypothesis;

impl Hypothesis {
    pub fn augmented_dickey_fuller(
        &self,
        time_series_values: &[f64],
        maximum_lag: usize,
    ) -> StockTrekResult<(f64, f64)> {
        hypothesis::augmented_dickey_fuller(time_series_values, maximum_lag)
    }

    pub fn durbin_watson(&self, residual_values: &[f64]) -> StockTrekResult<f64> {
        hypothesis::durbin_watson(residual_values)
    }

    pub fn jarque_bera(&self, time_series_values: &[f64]) -> StockTrekResult<(f64, f64)> {
        hypothesis::jarque_bera(time_series_values)
    }

    pub fn kwiatkowski_phillips_schmidt_shin(
        &self,
        time_series_values: &[f64],
    ) -> StockTrekResult<(f64, f64)> {
        hypothesis::kwiatkowski_phillips_schmidt_shin(time_series_values)
    }

    pub fn ljung_box(
        &self,
        time_series_values: &[f64],
        number_of_lags: usize,
    ) -> StockTrekResult<f64> {
        hypothesis::ljung_box(time_series_values, number_of_lags)
    }
}

/// MacKinnon (1994) critical values for the ADF test (constant, no trend case).
/// Response surface: C(p) = b_inf + b1/n + b2/n^2
/// Coefficients for significance levels 1%, 5%, 10%.
const MACKINNON_CRITICAL_VALUES: [(f64, f64, f64, f64); 3] = [
    // (significance level, b_inf, b1, b2)
    (0.01, -3.430_35, -6.539_30, -16.786_00), // 1%
    (0.05, -2.861_54, -2.890_10, -13.123_40), // 5%
    (0.10, -2.566_77, -1.538_40, -4.887_00),  // 10%
];

/// Compute MacKinnon critical value for a given sample size and significance level.
fn mackinnon_critical_value(n: usize, level_idx: usize) -> f64 {
    let (_, b_inf, b1, b2) = MACKINNON_CRITICAL_VALUES[level_idx];
    let nn = n as f64;
    b_inf + b1 / nn + b2 / (nn * nn)
}

/// Approximate p-value from MacKinnon critical values using interpolation.
/// Uses the empirical CDF of the ADF statistic under the null, approximated
/// via the response surface critical values at 1%, 5%, and 10%.
fn mackinnon_p_value(stat: f64, n: usize) -> f64 {
    // Get critical values for this sample size
    let cv_1 = mackinnon_critical_value(n, 0); // 1%
    let cv_5 = mackinnon_critical_value(n, 1); // 5%
    let cv_10 = mackinnon_critical_value(n, 2); // 10%

    // The ADF test is one-sided: more negative = more evidence against null
    // If stat is more negative than the 1% critical value, p < 0.01
    if stat <= cv_1 {
        // Extrapolate below 1% using log-linear fit
        let log_p_low = (0.01_f64).ln();
        let log_p_high = (0.05_f64).ln();
        let slope = (log_p_high - log_p_low) / (cv_5 - cv_1);
        let p = (log_p_low + slope * (stat - cv_1)).exp();
        return p.clamp(0.0, 1.0);
    }
    // If stat is between 1% and 5% critical values
    if stat <= cv_5 {
        let log_p_low = (0.01_f64).ln();
        let log_p_high = (0.05_f64).ln();
        let frac = (stat - cv_1) / (cv_5 - cv_1);
        let log_p = log_p_low + frac * (log_p_high - log_p_low);
        return log_p.exp();
    }
    // If stat is between 5% and 10% critical values
    if stat <= cv_10 {
        let log_p_low = (0.05_f64).ln();
        let log_p_high = (0.10_f64).ln();
        let frac = (stat - cv_5) / (cv_10 - cv_5);
        let log_p = log_p_low + frac * (log_p_high - log_p_low);
        return log_p.exp();
    }
    // If stat is greater than 10% critical value, extrapolate above 10%
    let log_p_low = (0.10_f64).ln();
    let log_p_high = (0.50_f64).ln();
    // Use the 10% -> 50% slope (approximate)
    let slope = (log_p_high - log_p_low) / 2.0; // heuristic: ~2x cv_10 distance
    let p = (log_p_low + slope * (stat - cv_10) / (cv_10.abs())).exp();
    p.clamp(0.0, 1.0)
}

pub fn augmented_dickey_fuller(
    time_series_values: &[f64],
    maximum_lag: usize,
) -> StockTrekResult<(f64, f64)> {
    let n = time_series_values.len();
    if n <= maximum_lag + 1 {
        return Err(StockTrekError::Stats(StatsError::InvalidParameters {
            function: "augmented_dickey_fuller",
            message: format!(
                "n={} <= maximum_lag={} + 1, need more observations",
                n, maximum_lag
            ),
        }));
    }

    // Δy_t = y_t - y_{t-1}
    let mut diff = Vec::with_capacity(n - 1);
    for i in 1..n {
        diff.push(time_series_values[i] - time_series_values[i - 1]);
    }

    // Build lagged level and differenced series, accounting for maximum_lag
    // Regression: Δy_t = α + β*y_{t-1} + Σγ_i*Δy_{t-i} + ε_t
    // We start from t = maximum_lag (so we have y_{t-1} and enough lags)
    let eff_n = diff.len() - maximum_lag;
    if eff_n < 1 {
        return Err(StockTrekError::Stats(StatsError::InvalidParameters {
            function: "augmented_dickey_fuller",
            message: format!(
                "effective n={} after accounting for lags, need at least 1",
                eff_n
            ),
        }));
    }

    // Build design matrix: [1, y_{t-1}, Δy_{t-1}, Δy_{t-2}, ..., Δy_{t-maximum_lag}]
    // Number of regressors = 1 (constant) + 1 (y_{t-1}) + maximum_lag (lagged diffs)
    let num_regressors = 1 + 1 + maximum_lag;

    // Use direct regression via normal equations for simplicity
    // X'X and X'y matrices (symmetric, store upper triangle)
    // We'll build X as a matrix (eff_n rows, num_regressors cols) and compute X'X, X'y

    // Design matrix columns: [const, y_{t-1}, Δy_{t-1}, Δy_{t-2}, ..., Δy_{t-maximum_lag}]
    // Response: Δy_t

    // For the matrix computation, we'll use a simpler approach:
    // Store the regressors in a flat vector and compute X'X and X'y
    let mut xtx = vec![0.0_f64; num_regressors * num_regressors];
    let mut xty = vec![0.0_f64; num_regressors];

    for t in 0..eff_n {
        let y_t = diff[maximum_lag + t]; // Δy_{t+maximum_lag}
        let y_lag = time_series_values[maximum_lag + t]; // y_{t+maximum_lag-1}

        // Build row of X
        let mut row = vec![0.0_f64; num_regressors];
        row[0] = 1.0; // constant
        row[1] = y_lag; // y_{t-1}

        // Lagged differences: Δy_{t-1}, Δy_{t-2}, ..., Δy_{t-maximum_lag}
        for j in 0..maximum_lag {
            row[2 + j] = diff[maximum_lag + t - 1 - j];
        }

        // Update X'X
        for i in 0..num_regressors {
            for j in 0..num_regressors {
                xtx[i * num_regressors + j] += row[i] * row[j];
            }
        }
        // Update X'y
        for i in 0..num_regressors {
            xty[i] += row[i] * y_t;
        }
    }

    // Solve for coefficients via Gaussian elimination on X'X
    let mut coeffs = xty.clone();
    let mut a = xtx.clone();
    let m = num_regressors;

    // Gaussian elimination with partial pivoting
    for i in 0..m {
        // Find pivot
        let mut max_val = a[i * m + i].abs();
        let mut max_row = i;
        for k in (i + 1)..m {
            let val = a[k * m + i].abs();
            if val > max_val {
                max_val = val;
                max_row = k;
            }
        }
        if max_val < 1e-15 {
            return Err(StockTrekError::Stats(StatsError::DivisionByZero {
                function: "augmented_dickey_fuller",
                detail: "singular design matrix in first Gaussian elimination".to_string(),
            }));
        }
        // Swap rows
        if max_row != i {
            for k in i..m {
                a.swap(i * m + k, max_row * m + k);
            }
            coeffs.swap(i, max_row);
        }
        // Eliminate
        for k in (i + 1)..m {
            let factor = a[k * m + i] / a[i * m + i];
            for j in i..m {
                a[k * m + j] -= factor * a[i * m + j];
            }
            coeffs[k] -= factor * coeffs[i];
        }
    }

    // Back substitution
    for i in (0..m).rev() {
        for j in (i + 1)..m {
            coeffs[i] -= a[i * m + j] * coeffs[j];
        }
        coeffs[i] /= a[i * m + i];
    }

    let beta = coeffs[1]; // coefficient on y_{t-1}

    // Compute residuals and MSE for standard error of beta
    let mut sse = 0.0;
    for t in 0..eff_n {
        let y_t = diff[maximum_lag + t];
        let y_lag = time_series_values[maximum_lag + t];
        let mut pred = coeffs[0] + coeffs[1] * y_lag;
        for j in 0..maximum_lag {
            pred += coeffs[2 + j] * diff[maximum_lag + t - 1 - j];
        }
        let resid = y_t - pred;
        sse += resid * resid;
    }

    let dof = eff_n as f64 - num_regressors as f64;
    if dof <= 0.0 {
        return Err(StockTrekError::Stats(
            StatsError::InsufficientDegreesOfFreedom {
                function: "augmented_dickey_fuller",
                dof: 0,
                needed: 1,
            },
        ));
    }
    let mse = sse / dof;

    // Standard error of beta is sqrt(MSE * C[1,1]) where C = (X'X)^{-1}
    // We need (X'X)^{-1}[1,1] - the element corresponding to y_{t-1}
    // Compute inverse via Gauss-Jordan on the augmented matrix
    let mut inv = vec![0.0_f64; m * m];
    for i in 0..m {
        inv[i * m + i] = 1.0;
    }
    let mut aug = xtx.clone();
    for i in 0..m {
        // Find pivot
        let mut max_val = aug[i * m + i].abs();
        let mut max_row = i;
        for k in (i + 1)..m {
            let val = aug[k * m + i].abs();
            if val > max_val {
                max_val = val;
                max_row = k;
            }
        }
        if max_val < 1e-15 {
            return Err(StockTrekError::Stats(StatsError::DivisionByZero {
                function: "augmented_dickey_fuller",
                detail: "singular matrix in inverse computation".to_string(),
            }));
        }
        if max_row != i {
            for k in 0..m {
                aug.swap(i * m + k, max_row * m + k);
                inv.swap(i * m + k, max_row * m + k);
            }
        }
        let pivot = aug[i * m + i];
        for k in 0..m {
            aug[i * m + k] /= pivot;
            inv[i * m + k] /= pivot;
        }
        for k in 0..m {
            if k != i {
                let factor = aug[k * m + i];
                for j in 0..m {
                    aug[k * m + j] -= factor * aug[i * m + j];
                    inv[k * m + j] -= factor * inv[i * m + j];
                }
            }
        }
    }

    let se_beta = (mse * inv[m + 1]).sqrt();
    if se_beta == 0.0 {
        return Err(StockTrekError::Stats(StatsError::DivisionByZero {
            function: "augmented_dickey_fuller",
            detail: "standard error of beta is zero".to_string(),
        }));
    }

    // ADF test statistic: t-statistic for β = 0
    let stat = beta / se_beta;

    // Compute p-value using MacKinnon critical value response surfaces
    let p_value = mackinnon_p_value(stat, eff_n);

    Ok((stat, p_value))
}

pub fn durbin_watson(residual_values: &[f64]) -> StockTrekResult<f64> {
    let n = residual_values.len();
    if n < 2 {
        return Err(StockTrekError::Stats(StatsError::InvalidParameters {
            function: "durbin_watson",
            message: format!("need at least 2 residuals, got {}", n),
        }));
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
        return Err(StockTrekError::Stats(StatsError::DivisionByZero {
            function: "durbin_watson",
            detail: "sum of squared residuals is zero".to_string(),
        }));
    }
    Ok(num / den)
}

pub fn jarque_bera(time_series_values: &[f64]) -> StockTrekResult<(f64, f64)> {
    let n = time_series_values.len();
    if n == 0 {
        return Err(StockTrekError::Stats(StatsError::EmptyInput {
            function: "jarque_bera",
        }));
    }
    let mu = stats::mean(time_series_values)?;
    let var = stats::variance(time_series_values, 0)?;
    if var == 0.0 {
        return Err(StockTrekError::Stats(StatsError::DivisionByZero {
            function: "jarque_bera",
            detail: "variance is zero".to_string(),
        }));
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

pub fn kwiatkowski_phillips_schmidt_shin(
    time_series_values: &[f64],
) -> StockTrekResult<(f64, f64)> {
    let n = time_series_values.len();
    if n == 0 {
        return Err(StockTrekError::Stats(StatsError::EmptyInput {
            function: "kwiatkowski_phillips_schmidt_shin",
        }));
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
        return Err(StockTrekError::Stats(StatsError::DivisionByZero {
            function: "kwiatkowski_phillips_schmidt_shin",
            detail: "variance is zero".to_string(),
        }));
    }
    let stat = eta / var;
    let p_value = (-stat).exp();
    Ok((stat, p_value))
}

pub fn ljung_box(time_series_values: &[f64], number_of_lags: usize) -> StockTrekResult<f64> {
    let n = time_series_values.len();
    if n == 0 || number_of_lags == 0 || number_of_lags >= n {
        return Err(StockTrekError::Stats(StatsError::InvalidParameters {
            function: "ljung_box",
            message: format!(
                "n={}, number_of_lags={}: need n > 0, lags > 0, and lags < n",
                n, number_of_lags
            ),
        }));
    }
    let var = stats::variance(time_series_values, 0)?;
    if var == 0.0 {
        return Err(StockTrekError::Stats(StatsError::DivisionByZero {
            function: "ljung_box",
            detail: "variance is zero".to_string(),
        }));
    }
    let mut q = 0.0;
    for k in 1..=number_of_lags {
        let rho = time_series::autocorrelation(time_series_values, k)?;
        q += rho.powi(2) / (n as f64 - k as f64);
    }
    Ok(n as f64 * (n as f64 + 2.0) * q)
}
