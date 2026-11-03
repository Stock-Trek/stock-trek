use crate::error::{
    domain::DomainError,
    result::{StockTrekError, StockTrekResult},
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

    /// Returns `()` when the bid price is below the ask price.
    pub(crate) fn bid_below_ask(
        function: &'static str,
        bid_price: f64,
        ask_price: f64,
    ) -> StockTrekResult<()> {
        if bid_price < ask_price {
            Ok(())
        } else {
            Err(Self::error(
                function,
                format!("bid price {bid_price} must be below ask price {ask_price}"),
            ))
        }
    }
}

#[cfg(test)]
mod tests;
