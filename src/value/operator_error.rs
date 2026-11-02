use crate::error::{
    result::{StockTrekError, StockTrekResult},
    stats::StatsError,
};

pub(super) fn domain_error(function: &'static str, message: impl Into<String>) -> StockTrekError {
    StockTrekError::Stats(StatsError::DomainError {
        function,
        message: message.into(),
    })
}

pub(super) fn division_by_zero(
    function: &'static str,
    detail: impl Into<String>,
) -> StockTrekError {
    StockTrekError::Stats(StatsError::DivisionByZero {
        function,
        detail: detail.into(),
    })
}

/// Rejects `NaN` and infinite values, which would otherwise propagate through
/// calculations and into conditions or order quantities.
pub(super) fn check_finite(
    function: &'static str,
    argument: &'static str,
    value: f64,
) -> StockTrekResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(domain_error(
            function,
            format!("{argument} {value} is not finite (NaN or infinity)"),
        ))
    }
}

macro_rules! domain_check {
    ($condition:expr, $function:literal, $message:expr $(,)?) => {
        if $condition {
            return Err($crate::value::operator_error::domain_error(
                $function, $message,
            ));
        }
    };
}

pub(super) use domain_check;
