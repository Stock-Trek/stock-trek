use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    resolved_context::ResolvedContext,
    values::value::{NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Log,
    Atan2,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BinaryCalculationValue {
    left: NumberValue,
    operator: BinaryOperator,
    right: NumberValue,
}

impl BinaryCalculationValue {
    pub fn new(left: NumberValue, operator: BinaryOperator, right: NumberValue) -> NumberValue {
        Box::new(Self {
            left,
            operator,
            right,
        })
    }
}

#[typetag::serde]
impl NumberValueTrait for BinaryCalculationValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let left_value = self.left.number(c)?;
        let right_value = self.right.number(c)?;
        let calculation_result = match self.operator {
            BinaryOperator::Add => left_value + right_value,
            BinaryOperator::Atan2 => left_value.atan2(right_value),
            BinaryOperator::Div => {
                if right_value == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DivisionByZero {
                        function: "Div",
                        detail: "divisor = 0 would produce +/- infinity".to_string(),
                    }));
                }
                left_value / right_value
            }
            BinaryOperator::Pow => {
                if left_value < 0.0 && right_value.fract() != 0.0 {
                    return Err(StockTrekError::Stats(StatsError::ComplexPowerResult {
                        operator: "Pow",
                        base: left_value,
                        exponent: right_value,
                    }));
                }
                left_value.powf(right_value)
            }
            BinaryOperator::Log => {
                if left_value == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::UndefinedLogarithm {
                        operator: "Log",
                        detail: "argument = 0 would produce undefined".to_string(),
                    }));
                }
                if right_value == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::UndefinedLogarithm {
                        operator: "Log",
                        detail: "base = 0 would produce undefined".to_string(),
                    }));
                }
                if right_value == 1.0 {
                    return Err(StockTrekError::Stats(StatsError::UndefinedLogarithm {
                        operator: "Log",
                        detail: "base = 1 would produce undefined".to_string(),
                    }));
                }
                if left_value < 0.0 {
                    return Err(StockTrekError::Stats(StatsError::ComplexLogarithm {
                        operator: "Log",
                        detail: format!(
                            "argument {} < 0 would produce a complex number",
                            left_value
                        ),
                    }));
                }
                if right_value < 0.0 {
                    return Err(StockTrekError::Stats(StatsError::ComplexLogarithm {
                        operator: "Log",
                        detail: format!("base {} < 0 would produce a complex number", right_value),
                    }));
                }
                left_value.log(right_value)
            }
            BinaryOperator::Mod => {
                if right_value == 0.0 {
                    return Err(StockTrekError::Stats(StatsError::DivisionByZero {
                        function: "Mod",
                        detail: "divisor = 0 would produce +/- infinity".to_string(),
                    }));
                }
                left_value % right_value
            }
            BinaryOperator::Mul => left_value * right_value,
            BinaryOperator::Sub => left_value - right_value,
        };
        Ok(calculation_result)
    }
}
