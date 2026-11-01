use crate::error::{
    result::{StockTrekError, StockTrekResult},
    stats::StatsError,
};
use serde::{Deserialize, Serialize};
use strum::Display;

/// Binary operations that can be applied to two numeric values.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum BinaryOperator {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Remainder after division.
    Mod,
    /// Exponentiation.
    Pow,
    /// Logarithm of the left value in the base given by the right value.
    Log,
    /// Two-argument arctangent.
    Atan2,
}

impl BinaryOperator {
    /// Applies the operator to `left` and `right`.
    pub fn calculate(self, left: f64, right: f64) -> StockTrekResult<f64> {
        let calculation_result = match self {
            BinaryOperator::Add => left + right,
            BinaryOperator::Atan2 => left.atan2(right),
            BinaryOperator::Div => {
                if right == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DivisionByZero {
                        function: "Div",
                        detail: "divisor = 0 would produce +/- infinity".to_string(),
                    }));
                }
                left / right
            }
            BinaryOperator::Pow => {
                if left < 0.0 && right.fract() != 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Pow",
                        message: format!(
                            "base {} < 0 with fractional exponent {} would produce a complex number",
                            left, right
                        ),
                    }));
                }
                left.powf(right)
            }
            BinaryOperator::Log => {
                if left == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log",
                        message: "argument = 0 is undefined".to_string(),
                    }));
                }
                if right == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log",
                        message: "base = 0 is undefined".to_string(),
                    }));
                }
                if right == 1.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log",
                        message: "base = 1 is undefined".to_string(),
                    }));
                }
                if left < 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log",
                        message: format!("argument {} < 0 would produce a complex number", left),
                    }));
                }
                if right < 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DomainError {
                        function: "Log",
                        message: format!("base {} < 0 would produce a complex number", right),
                    }));
                }
                left.log(right)
            }
            BinaryOperator::Mod => {
                if right == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DivisionByZero {
                        function: "Mod",
                        detail: "divisor = 0 would produce +/- infinity".to_string(),
                    }));
                }
                left % right
            }
            BinaryOperator::Mul => left * right,
            BinaryOperator::Sub => left - right,
        };
        Ok(calculation_result)
    }
}
