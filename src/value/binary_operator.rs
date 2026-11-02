use super::operator_error::{OperatorError, domain_check};
use crate::error::stock_trek_error::StockTrekResult;
use crate::validation::Validation;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// Binary operations that can be applied to two numeric values.
#[derive(
    Debug, Display, IntoStaticStr, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
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
        let function: &'static str = self.into();
        Validation::finite(function, "left operand", left)?;
        Validation::finite(function, "right operand", right)?;

        let calculation_result = match self {
            BinaryOperator::Add => left + right,
            BinaryOperator::Atan2 => left.atan2(right),
            BinaryOperator::Div => {
                if right == 0.0 {
                    return Err(OperatorError::division_by_zero(
                        function,
                        "divisor = 0 would produce +/- infinity",
                    ));
                }
                left / right
            }
            BinaryOperator::Pow => {
                domain_check!(
                    left < 0.0 && right.fract() != 0.0,
                    function,
                    format!(
                        "base {} < 0 with fractional exponent {} would produce a complex number",
                        left, right
                    )
                );
                left.powf(right)
            }
            BinaryOperator::Log => {
                domain_check!(left == 0.0, function, "argument = 0 is undefined");
                domain_check!(right == 0.0, function, "base = 0 is undefined");
                domain_check!(right == 1.0, function, "base = 1 is undefined");
                domain_check!(
                    left < 0.0,
                    function,
                    format!("argument {} < 0 would produce a complex number", left)
                );
                domain_check!(
                    right < 0.0,
                    function,
                    format!("base {} < 0 would produce a complex number", right)
                );
                left.log(right)
            }
            BinaryOperator::Mod => {
                if right == 0.0 {
                    return Err(OperatorError::division_by_zero(
                        function,
                        "divisor = 0 would produce +/- infinity",
                    ));
                }
                left % right
            }
            BinaryOperator::Mul => left * right,
            BinaryOperator::Sub => left - right,
        };
        Validation::finite(function, "result", calculation_result)?;
        Ok(calculation_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPERATORS: [BinaryOperator; 8] = [
        BinaryOperator::Add,
        BinaryOperator::Sub,
        BinaryOperator::Mul,
        BinaryOperator::Div,
        BinaryOperator::Mod,
        BinaryOperator::Pow,
        BinaryOperator::Log,
        BinaryOperator::Atan2,
    ];

    #[test]
    fn rejects_nan_operands() {
        for operator in OPERATORS {
            assert!(operator.calculate(f64::NAN, 2.0).is_err(), "{operator}");
            assert!(operator.calculate(2.0, f64::NAN).is_err(), "{operator}");
        }
    }

    #[test]
    fn rejects_infinite_operands() {
        for operator in OPERATORS {
            for value in [f64::INFINITY, f64::NEG_INFINITY] {
                assert!(operator.calculate(value, 2.0).is_err(), "{operator}");
                assert!(operator.calculate(2.0, value).is_err(), "{operator}");
            }
        }
    }

    #[test]
    fn rejects_non_finite_results() {
        assert!(BinaryOperator::Add.calculate(f64::MAX, f64::MAX).is_err());
        assert!(BinaryOperator::Sub.calculate(f64::MIN, f64::MAX).is_err());
        assert!(BinaryOperator::Mul.calculate(f64::MAX, 2.0).is_err());
        assert!(BinaryOperator::Div.calculate(f64::MAX, 0.5).is_err());
        assert!(BinaryOperator::Pow.calculate(2.0, 1024.0).is_err());
    }
}
