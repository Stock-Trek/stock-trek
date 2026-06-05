use crate::statistics::frequency;
use num_complex::Complex;
use std::f64::consts::PI;

#[derive(Clone, Default)]
pub struct Frequency;

impl Frequency {
    pub fn discrete_fourier_transform(&self, time_series_values: &[f64]) -> Vec<Complex<f64>> {
        frequency::discrete_fourier_transform(time_series_values)
    }
    pub fn inverse_discrete_fourier_transform(
        &self,
        frequency_domain_values: &[Complex<f64>],
    ) -> Vec<f64> {
        frequency::inverse_discrete_fourier_transform(frequency_domain_values)
    }
    pub fn periodogram(&self, time_series_values: &[f64], sampling_frequency: f64) -> Vec<f64> {
        frequency::periodogram(time_series_values, sampling_frequency)
    }
    pub fn spectral_density(&self, time_series_values: &[f64]) -> Vec<f64> {
        frequency::spectral_density(time_series_values)
    }
}

/// Compute the discrete Fourier transform of a time series using the Cooley-Tukey radix-2 FFT
/// algorithm.
///
/// The input length **must** be a power of two. If it is not, `None` is returned.
///
/// Complexity: O(n log n) vs the naive O(n²) DFT.
pub fn discrete_fourier_transform(time_series_values: &[f64]) -> Vec<Complex<f64>> {
    let n = time_series_values.len();
    let mut a: Vec<Complex<f64>> = time_series_values
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    if !n.is_power_of_two() {
        // Fallback to naive O(n²) DFT for non-power-of-two lengths
        return dft_naive(time_series_values);
    }
    fft(&mut a, false);
    a
}

/// Naive O(n²) discrete Fourier transform (fallback for non-power-of-two inputs).
fn dft_naive(time_series_values: &[f64]) -> Vec<Complex<f64>> {
    let n = time_series_values.len();
    let mut result = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (t, &x) in time_series_values.iter().enumerate() {
            let angle = -2.0 * PI * (k as f64 * t as f64) / n as f64;
            let w = Complex::new(angle.cos(), angle.sin());
            sum += w * x;
        }
        result.push(sum);
    }
    result
}

/// In-place Cooley-Tukey radix-2 FFT (or IFFT when `inverse = true`).
///
/// Assumes `a.len()` is a power of two and greater than 1.
fn fft(a: &mut [Complex<f64>], inverse: bool) {
    let n = a.len();
    debug_assert!(n.is_power_of_two(), "FFT requires power-of-two length");

    // Bit-reversal permutation
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }

    // Iterative FFT butterfly stages (Cooley-Tukey)
    let mut len = 2;
    while len <= n {
        let half = len >> 1;
        let sign = if inverse { 1.0 } else { -1.0 };
        for i in (0..n).step_by(len) {
            for k in 0..half {
                let angle = sign * 2.0 * PI * (k as f64) / len as f64;
                let w = Complex::new(angle.cos(), angle.sin());
                let ev = a[i + k];
                let od = a[i + k + half] * w;
                a[i + k] = ev + od;
                a[i + k + half] = ev - od;
            }
        }
        len <<= 1;
    }

    if inverse {
        let inv_n = 1.0 / n as f64;
        for x in a.iter_mut() {
            *x *= inv_n;
        }
    }
}

/// Compute the inverse discrete Fourier transform of frequency-domain values using FFT.
///
/// The input length **must** be a power of two. If it is not, `None` is returned.
pub fn inverse_discrete_fourier_transform(frequency_domain_values: &[Complex<f64>]) -> Vec<f64> {
    let n = frequency_domain_values.len();
    let mut a: Vec<Complex<f64>> = frequency_domain_values.to_vec();
    if !n.is_power_of_two() {
        // Fallback to naive O(n²) IDFT for non-power-of-two lengths
        return idft_naive(frequency_domain_values);
    }
    fft(&mut a, true);
    a.iter().map(|c| c.re).collect()
}

/// Naive O(n²) inverse discrete Fourier transform (fallback for non-power-of-two inputs).
fn idft_naive(frequency_domain_values: &[Complex<f64>]) -> Vec<f64> {
    let n = frequency_domain_values.len();
    let mut result = Vec::with_capacity(n);
    for t in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (k, &xk) in frequency_domain_values.iter().enumerate() {
            let angle = 2.0 * PI * (k as f64 * t as f64) / n as f64;
            let w = Complex::new(angle.cos(), angle.sin());
            sum += xk * w;
        }
        result.push((sum / n as f64).re);
    }
    result
}

pub fn periodogram(time_series_values: &[f64], sampling_frequency: f64) -> Vec<f64> {
    let n = time_series_values.len();
    let dft = frequency::discrete_fourier_transform(time_series_values);
    dft.iter()
        .map(|xk| (xk.norm_sqr() / n as f64) * sampling_frequency)
        .collect()
}

pub fn spectral_density(time_series_values: &[f64]) -> Vec<f64> {
    let n = time_series_values.len();
    let dft = frequency::discrete_fourier_transform(time_series_values);
    dft.iter().map(|xk| xk.norm_sqr() / n as f64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_fft_equivalence_with_dft_power_of_two() {
        // Verify FFT produces exactly the same result as the naive DFT for power-of-two lengths.
        let values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let fft_result = discrete_fourier_transform(&values);

        // Naive DFT for reference
        let n = values.len();
        let mut dft_result = Vec::with_capacity(n);
        for k in 0..n {
            let mut sum = Complex::new(0.0, 0.0);
            for (t, &x) in values.iter().enumerate() {
                let angle = -2.0 * PI * (k as f64 * t as f64) / n as f64;
                let w = Complex::new(angle.cos(), angle.sin());
                sum += w * x;
            }
            dft_result.push(sum);
        }

        assert_eq!(fft_result.len(), dft_result.len());
        for (fft_val, dft_val) in fft_result.iter().zip(dft_result.iter()) {
            assert!(
                (fft_val.re - dft_val.re).abs() < 1e-10,
                "FFT re {} != DFT re {}",
                fft_val.re,
                dft_val.re
            );
            assert!(
                (fft_val.im - dft_val.im).abs() < 1e-10,
                "FFT im {} != DFT im {}",
                fft_val.im,
                dft_val.im
            );
        }
    }

    #[test]
    fn test_non_power_of_two_falls_back_to_dft() {
        // For non-power-of-two, verify the result still matches naive DFT exactly.
        let values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = discrete_fourier_transform(&values);

        let n = values.len();
        let mut expected = Vec::with_capacity(n);
        for k in 0..n {
            let mut sum = Complex::new(0.0, 0.0);
            for (t, &x) in values.iter().enumerate() {
                let angle = -2.0 * PI * (k as f64 * t as f64) / n as f64;
                let w = Complex::new(angle.cos(), angle.sin());
                sum += w * x;
            }
            expected.push(sum);
        }

        assert_eq!(result.len(), expected.len());
        for (v, e) in result.iter().zip(expected.iter()) {
            assert!((v.re - e.re).abs() < 1e-10, "re {} != {}", v.re, e.re);
            assert!((v.im - e.im).abs() < 1e-10, "im {} != {}", v.im, e.im);
        }
    }

    #[test]
    fn test_inverse_dft_roundtrip_power_of_two() {
        // Round-trip test: time series → DFT → IDFT → original
        let values: Vec<f64> = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        let freq = discrete_fourier_transform(&values);
        let recovered = inverse_discrete_fourier_transform(&freq);

        assert_eq!(recovered.len(), values.len());
        for (orig, rec) in values.iter().zip(recovered.iter()) {
            assert!(
                (orig - rec).abs() < 1e-10,
                "Round-trip failed: {} != {}",
                orig,
                rec
            );
        }
    }

    #[test]
    fn test_inverse_dft_roundtrip_non_power_of_two() {
        // Round-trip with non-power-of-two length (uses naive fallback)
        let values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let freq = discrete_fourier_transform(&values);
        let recovered = inverse_discrete_fourier_transform(&freq);

        assert_eq!(recovered.len(), values.len());
        for (orig, rec) in values.iter().zip(recovered.iter()) {
            assert!(
                (orig - rec).abs() < 1e-10,
                "Round-trip failed: {} != {}",
                orig,
                rec
            );
        }
    }

    #[test]
    fn test_single_element() {
        // Edge case: single element
        let values = vec![42.0];
        let result = discrete_fourier_transform(&values);
        assert_eq!(result.len(), 1);
        assert!((result[0].re - 42.0).abs() < 1e-10);
        assert!((result[0].im).abs() < 1e-10);

        // Round-trip
        let recovered = inverse_discrete_fourier_transform(&result);
        assert!((recovered[0] - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_input() {
        let values: Vec<f64> = vec![];
        let result = discrete_fourier_transform(&values);
        assert!(result.is_empty());

        let freq: Vec<Complex<f64>> = vec![];
        let result = inverse_discrete_fourier_transform(&freq);
        assert!(result.is_empty());
    }
}
