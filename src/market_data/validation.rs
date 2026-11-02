use crate::error::{
    market_data::MarketDataError,
    result::{StockTrekError, StockTrekResult},
};

pub(super) fn error(function: &'static str, message: impl Into<String>) -> StockTrekError {
    StockTrekError::MarketData(MarketDataError::DomainError {
        function,
        message: message.into(),
    })
}

pub(super) fn finite(
    function: &'static str,
    field: &'static str,
    value: f64,
) -> StockTrekResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(error(
            function,
            format!("{field} {value} is not finite (NaN or infinity)"),
        ))
    }
}

pub(super) fn positive(
    function: &'static str,
    field: &'static str,
    value: f64,
) -> StockTrekResult<()> {
    finite(function, field, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(error(function, format!("{field} {value} must be positive")))
    }
}

pub(super) fn non_negative(
    function: &'static str,
    field: &'static str,
    value: f64,
) -> StockTrekResult<()> {
    finite(function, field, value)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(error(
            function,
            format!("{field} {value} must be non-negative"),
        ))
    }
}
