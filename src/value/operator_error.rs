use crate::error::{result::StockTrekError, stats::StatsError};

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
