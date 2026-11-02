use crate::error::{result::StockTrekError, stats::StatsError};

/// Error helpers shared by the binary and unary operators.
pub(super) struct OperatorError;

impl OperatorError {
    pub(super) fn division_by_zero(
        function: &'static str,
        detail: impl Into<String>,
    ) -> StockTrekError {
        StockTrekError::Stats(StatsError::DivisionByZero {
            function,
            detail: detail.into(),
        })
    }
}

macro_rules! domain_check {
    ($condition:expr, $function:expr, $message:expr $(,)?) => {
        if $condition {
            return Err($crate::validation::Validation::error($function, $message));
        }
    };
}

pub(super) use domain_check;
