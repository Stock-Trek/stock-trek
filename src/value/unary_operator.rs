use super::operator_error::{OperatorError, domain_check};
use crate::error::result::StockTrekResult;
use serde::{Deserialize, Serialize};
use strum::Display;

/// Unary operations that can be applied to a numeric value.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    fn name(self) -> &'static str {
        match self {
            UnaryOperator::Abs => "Abs",
            UnaryOperator::Acos => "Acos",
            UnaryOperator::Acosh => "Acosh",
            UnaryOperator::Asin => "Asin",
            UnaryOperator::Asinh => "Asinh",
            UnaryOperator::Atan => "Atan",
            UnaryOperator::Atanh => "Atanh",
            UnaryOperator::Ceil => "Ceil",
            UnaryOperator::Cos => "Cos",
            UnaryOperator::Cosh => "Cosh",
            UnaryOperator::Exp => "Exp",
            UnaryOperator::Exp2 => "Exp2",
            UnaryOperator::Floor => "Floor",
            UnaryOperator::Frac => "Frac",
            UnaryOperator::Log10 => "Log10",
            UnaryOperator::Log2 => "Log2",
            UnaryOperator::LogE => "LogE",
            UnaryOperator::Neg => "Neg",
            UnaryOperator::RoundAwayFrom0 => "RoundAwayFrom0",
            UnaryOperator::RoundTiesEven => "RoundTiesEven",
            UnaryOperator::Sin => "Sin",
            UnaryOperator::Sinh => "Sinh",
            UnaryOperator::Sqrt => "Sqrt",
            UnaryOperator::Tan => "Tan",
            UnaryOperator::Tanh => "Tanh",
            UnaryOperator::Trunc => "Trunc",
        }
    }

    /// Applies the operator to `value`.
    pub fn calculate(self, value: f64) -> StockTrekResult<f64> {
        let function = self.name();
        OperatorError::check_finite(function, "value", value)?;

        let calculation_result = match self {
            UnaryOperator::Abs => value.abs(),
            UnaryOperator::Acos => {
                domain_check!(
                    value < -1.0 || value > 1.0,
                    "Acos",
                    format!("value {} outside [-1, 1]", value)
                );
                value.acos()
            }
            UnaryOperator::Acosh => {
                domain_check!(value < 1.0, "Acosh", format!("value {} < 1", value));
                value.acosh()
            }
            UnaryOperator::Asin => {
                domain_check!(
                    value < -1.0 || value > 1.0,
                    "Asin",
                    format!("value {} outside [-1, 1]", value)
                );
                value.asin()
            }
            UnaryOperator::Asinh => value.asinh(),
            UnaryOperator::Atan => value.atan(),
            UnaryOperator::Atanh => {
                domain_check!(
                    value == -1.0,
                    "Atanh",
                    "value = -1 produces negative infinity"
                );
                domain_check!(
                    value == 1.0,
                    "Atanh",
                    "value = 1 produces positive infinity"
                );
                domain_check!(
                    value < -1.0 || value > 1.0,
                    "Atanh",
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
                domain_check!(value == 0.0, "Log10", "value = 0 is undefined");
                domain_check!(
                    value < 0.0,
                    "Log10",
                    format!("value {} < 0 would produce a complex number", value)
                );
                value.log10()
            }
            UnaryOperator::Log2 => {
                domain_check!(value == 0.0, "Log2", "value = 0 is undefined");
                domain_check!(
                    value < 0.0,
                    "Log2",
                    format!("value {} < 0 would produce a complex number", value)
                );
                value.log2()
            }
            UnaryOperator::LogE => {
                domain_check!(value == 0.0, "LogE", "value = 0 is undefined");
                domain_check!(
                    value < 0.0,
                    "LogE",
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
                    "Sqrt",
                    format!("value {} < 0 would produce a complex number", value)
                );
                value.sqrt()
            }
            UnaryOperator::Tan => value.tan(),
            UnaryOperator::Tanh => value.tanh(),
            UnaryOperator::Trunc => value.trunc(),
        };
        OperatorError::check_finite(function, "result", calculation_result)?;
        Ok(calculation_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPERATORS: [UnaryOperator; 26] = [
        UnaryOperator::Abs,
        UnaryOperator::Neg,
        UnaryOperator::Floor,
        UnaryOperator::Ceil,
        UnaryOperator::RoundAwayFrom0,
        UnaryOperator::RoundTiesEven,
        UnaryOperator::Trunc,
        UnaryOperator::Frac,
        UnaryOperator::Sqrt,
        UnaryOperator::Exp,
        UnaryOperator::Exp2,
        UnaryOperator::Log2,
        UnaryOperator::LogE,
        UnaryOperator::Log10,
        UnaryOperator::Sin,
        UnaryOperator::Cos,
        UnaryOperator::Tan,
        UnaryOperator::Asin,
        UnaryOperator::Acos,
        UnaryOperator::Atan,
        UnaryOperator::Sinh,
        UnaryOperator::Cosh,
        UnaryOperator::Tanh,
        UnaryOperator::Asinh,
        UnaryOperator::Acosh,
        UnaryOperator::Atanh,
    ];

    #[test]
    fn rejects_non_finite_inputs() {
        for operator in OPERATORS {
            for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                assert!(operator.calculate(value).is_err(), "{operator}");
            }
        }
    }

    #[test]
    fn rejects_non_finite_results() {
        assert!(UnaryOperator::Exp.calculate(1_000.0).is_err());
        assert!(UnaryOperator::Exp2.calculate(2_000.0).is_err());
        assert!(UnaryOperator::Sinh.calculate(1_000.0).is_err());
        assert!(UnaryOperator::Cosh.calculate(1_000.0).is_err());
    }

    #[test]
    fn accepts_finite_values() {
        assert_eq!(UnaryOperator::Sqrt.calculate(9.0).unwrap(), 3.0);
        assert_eq!(UnaryOperator::LogE.calculate(1.0).unwrap(), 0.0);
    }
}
