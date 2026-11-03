use super::operator_error::domain_check;
use crate::error::result::StockTrekResult;
use crate::validation::Validation;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// Unary operations that can be applied to a numeric value.
#[derive(
    Debug, Display, IntoStaticStr, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum UnaryOperator {
    /// Absolute value.
    Abs,
    /// Negation.
    Neg,
    /// Round down to the nearest integer.
    Floor,
    /// Round up to the nearest integer.
    Ceil,
    /// Round away from zero.
    RoundAwayFrom0,
    /// Round to the nearest integer with ties to even.
    RoundTiesEven,
    /// Truncate toward zero.
    Trunc,
    /// Fractional part.
    Frac,
    /// Square root.
    Sqrt,
    /// Natural exponential.
    Exp,
    /// Base-2 exponential.
    Exp2,
    /// Base-2 logarithm.
    Log2,
    /// Natural logarithm.
    LogE,
    /// Base-10 logarithm.
    Log10,
    /// Sine.
    Sin,
    /// Cosine.
    Cos,
    /// Tangent.
    Tan,
    /// Arcsine.
    Asin,
    /// Arccosine.
    Acos,
    /// Arctangent.
    Atan,
    /// Hyperbolic sine.
    Sinh,
    /// Hyperbolic cosine.
    Cosh,
    /// Hyperbolic tangent.
    Tanh,
    /// Inverse hyperbolic sine.
    Asinh,
    /// Inverse hyperbolic cosine.
    Acosh,
    /// Inverse hyperbolic tangent.
    Atanh,
}

impl UnaryOperator {
    /// Applies the operator to `value`.
    pub fn calculate(self, value: f64) -> StockTrekResult<f64> {
        let function: &'static str = self.into();
        Validation::finite(function, "value", value)?;

        let calculation_result = match self {
            UnaryOperator::Abs => value.abs(),
            UnaryOperator::Acos => {
                domain_check!(
                    value < -1.0 || value > 1.0,
                    function,
                    format!("value {} outside [-1, 1]", value)
                );
                value.acos()
            }
            UnaryOperator::Acosh => {
                domain_check!(value < 1.0, function, format!("value {} < 1", value));
                value.acosh()
            }
            UnaryOperator::Asin => {
                domain_check!(
                    value < -1.0 || value > 1.0,
                    function,
                    format!("value {} outside [-1, 1]", value)
                );
                value.asin()
            }
            UnaryOperator::Asinh => value.asinh(),
            UnaryOperator::Atan => value.atan(),
            UnaryOperator::Atanh => {
                domain_check!(
                    value == -1.0,
                    function,
                    "value = -1 produces negative infinity"
                );
                domain_check!(
                    value == 1.0,
                    function,
                    "value = 1 produces positive infinity"
                );
                domain_check!(
                    value < -1.0 || value > 1.0,
                    function,
                    format!("value {} outside [-1, 1]", value)
                );
                value.atanh()
            }
            UnaryOperator::Ceil => value.ceil(),
            UnaryOperator::Cos => value.cos(),
            UnaryOperator::Cosh => value.cosh(),
            UnaryOperator::Exp => value.exp(),
            UnaryOperator::Exp2 => value.exp2(),
            UnaryOperator::Floor => value.floor(),
            UnaryOperator::Frac => value.fract(),
            UnaryOperator::Log10 => {
                domain_check!(value == 0.0, function, "value = 0 is undefined");
                domain_check!(
                    value < 0.0,
                    function,
                    format!("value {} < 0 would produce a complex number", value)
                );
                value.log10()
            }
            UnaryOperator::Log2 => {
                domain_check!(value == 0.0, function, "value = 0 is undefined");
                domain_check!(
                    value < 0.0,
                    function,
                    format!("value {} < 0 would produce a complex number", value)
                );
                value.log2()
            }
            UnaryOperator::LogE => {
                domain_check!(value == 0.0, function, "value = 0 is undefined");
                domain_check!(
                    value < 0.0,
                    function,
                    format!("value {} < 0 would produce a complex number", value)
                );
                value.ln()
            }
            UnaryOperator::Neg => -value,
            UnaryOperator::RoundAwayFrom0 => value.round(),
            UnaryOperator::RoundTiesEven => value.round_ties_even(),
            UnaryOperator::Sin => value.sin(),
            UnaryOperator::Sinh => value.sinh(),
            UnaryOperator::Sqrt => {
                domain_check!(
                    value < 0.0,
                    function,
                    format!("value {} < 0 would produce a complex number", value)
                );
                value.sqrt()
            }
            UnaryOperator::Tan => value.tan(),
            UnaryOperator::Tanh => value.tanh(),
            UnaryOperator::Trunc => value.trunc(),
        };
        Validation::finite(function, "result", calculation_result)?;
        Ok(calculation_result)
    }
}

#[cfg(test)]
#[path = "unary_operator_tests.rs"]
mod tests;
