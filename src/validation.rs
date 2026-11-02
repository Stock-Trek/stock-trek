use crate::error::{
    market_data::MarketDataError,
    result::{StockTrekError, StockTrekResult},
    stats::StatsError,
};

/// Shared numeric validation with the error domain selected by the caller.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Validation {
    MarketData,
    Stats,
}

impl Validation {
    pub(crate) fn error(
        self,
        function: &'static str,
        message: impl Into<String>,
    ) -> StockTrekError {
        let message = message.into();
        match self {
            Self::MarketData => {
                StockTrekError::MarketData(MarketDataError::DomainError { function, message })
            }
            Self::Stats => StockTrekError::Stats(StatsError::DomainError { function, message }),
        }
    }

    pub(crate) fn finite(
        self,
        function: &'static str,
        field: &'static str,
        value: f64,
    ) -> StockTrekResult<f64> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(self.error(
                function,
                format!("{field} {value} is not finite (NaN or infinity)"),
            ))
        }
    }

    pub(crate) fn positive(
        self,
        function: &'static str,
        field: &'static str,
        value: f64,
    ) -> StockTrekResult<f64> {
        let value = self.finite(function, field, value)?;
        if value > 0.0 {
            Ok(value)
        } else {
            Err(self.error(function, format!("{field} {value} must be positive")))
        }
    }

    pub(crate) fn non_negative(
        self,
        function: &'static str,
        field: &'static str,
        value: f64,
    ) -> StockTrekResult<f64> {
        let value = self.finite(function, field, value)?;
        if value >= 0.0 {
            Ok(value)
        } else {
            Err(self.error(function, format!("{field} {value} must be non-negative")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_accepts_finite_values_and_rejects_nan_and_infinity() {
        assert_eq!(Validation::Stats.finite("test", "value", 1.5).unwrap(), 1.5);
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(Validation::Stats.finite("test", "value", value).is_err());
        }
    }

    #[test]
    fn positive_rejects_zero_and_negative_values() {
        assert_eq!(
            Validation::Stats.positive("test", "value", 1.5).unwrap(),
            1.5
        );
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(Validation::Stats.positive("test", "value", value).is_err());
        }
    }

    #[test]
    fn non_negative_accepts_zero_and_rejects_negative_values() {
        assert_eq!(
            Validation::Stats
                .non_negative("test", "value", 0.0)
                .unwrap(),
            0.0
        );
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(
                Validation::Stats
                    .non_negative("test", "value", value)
                    .is_err()
            );
        }
    }

    #[test]
    fn keeps_the_selected_error_domain() {
        let error = Validation::MarketData
            .positive("test", "value", 0.0)
            .unwrap_err();
        assert!(matches!(
            error,
            StockTrekError::MarketData(MarketDataError::DomainError { .. })
        ));

        let error = Validation::Stats
            .positive("test", "value", 0.0)
            .unwrap_err();
        assert!(matches!(
            error,
            StockTrekError::Stats(StatsError::DomainError { .. })
        ));
    }
}
