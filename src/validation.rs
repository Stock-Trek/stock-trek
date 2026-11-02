use crate::error::{
    domain::DomainError,
    stock_trek_error::{StockTrekError, StockTrekResult},
};

/// Shared finite, positive, and non-negative numeric validation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Validation;

impl Validation {
    /// Builds a domain error attributed to `function`.
    pub(crate) fn error(function: &'static str, message: impl Into<String>) -> StockTrekError {
        StockTrekError::Domain(DomainError::new(function, message))
    }

    /// Returns `value` when it is finite.
    pub(crate) fn finite(
        function: &'static str,
        field: &'static str,
        value: f64,
    ) -> StockTrekResult<f64> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(Self::error(
                function,
                format!("{field} {value} is not finite (NaN or infinity)"),
            ))
        }
    }

    /// Returns `value` when it is finite and greater than zero.
    pub(crate) fn positive(
        function: &'static str,
        field: &'static str,
        value: f64,
    ) -> StockTrekResult<f64> {
        let value = Self::finite(function, field, value)?;
        if value > 0.0 {
            Ok(value)
        } else {
            Err(Self::error(
                function,
                format!("{field} {value} must be positive"),
            ))
        }
    }

    /// Returns `value` when it is finite and at least zero.
    pub(crate) fn non_negative(
        function: &'static str,
        field: &'static str,
        value: f64,
    ) -> StockTrekResult<f64> {
        let value = Self::finite(function, field, value)?;
        if value >= 0.0 {
            Ok(value)
        } else {
            Err(Self::error(
                function,
                format!("{field} {value} must be non-negative"),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_accepts_finite_values_and_rejects_nan_and_infinity() {
        assert_eq!(Validation::finite("test", "value", 1.5).unwrap(), 1.5);
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(Validation::finite("test", "value", value).is_err());
        }
    }

    #[test]
    fn positive_rejects_zero_and_negative_values() {
        assert_eq!(Validation::positive("test", "value", 1.5).unwrap(), 1.5);
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(Validation::positive("test", "value", value).is_err());
        }
    }

    #[test]
    fn non_negative_accepts_zero_and_rejects_negative_values() {
        assert_eq!(Validation::non_negative("test", "value", 0.0).unwrap(), 0.0);
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(Validation::non_negative("test", "value", value).is_err());
        }
    }

    #[test]
    fn failures_produce_a_domain_error() {
        let error = Validation::positive("test", "value", 0.0).unwrap_err();
        assert!(matches!(error, StockTrekError::Domain(DomainError { .. })));
    }
}
