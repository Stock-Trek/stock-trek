use crate::{
    assembler_context::AssemblerContext,
    values::value::{NumberValue, NumberValueTrait},
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
    fn number(&self, context: &AssemblerContext) -> Result<f64> {
        let left_value = self.left.number(context)?;
        let right_value = self.right.number(context)?;
        let calculation_result = match self.operator {
            BinaryOperator::Add => left_value + right_value,
            BinaryOperator::Atan2 => left_value.atan2(right_value),
            BinaryOperator::Div => {
                if right_value == 0.0 {
                    bail!("Divide: A divisor =0 is not supported, it would give +/- infinity");
                }
                left_value / right_value
            }
            BinaryOperator::Pow => {
                if left_value < 0.0 && right_value.fract() != 0.0 {
                    bail!(
                        "Power: Combining a base <0 {} with a non-integer exponent {} is not supported, it would give a complex number",
                        left_value,
                        right_value,
                    );
                }
                left_value.powf(right_value)
            }
            BinaryOperator::Log => {
                if left_value == 0.0 {
                    bail!("Log: An argument =0 is not supported, it would give undefined");
                }
                if right_value == 0.0 {
                    bail!("Log: A base =0 is not supported, it would give undefined");
                }
                if right_value == 1.0 {
                    bail!("Log: A base =1 is not supported, it would give undefined");
                }
                if left_value < 0.0 {
                    bail!(
                        "Log: An argument <0 {} is not supported, it would give a complex number",
                        left_value
                    );
                }
                if right_value < 0.0 {
                    bail!(
                        "Log: A base <0 {} is not supported, it would give a complex number",
                        right_value
                    );
                }
                left_value.log(right_value)
            }
            BinaryOperator::Mod => {
                if right_value == 0.0 {
                    bail!("Modulo: A modulus =0 is not supported, it would give undefined");
                }
                left_value % right_value
            }
            BinaryOperator::Mul => left_value * right_value,
            BinaryOperator::Sub => left_value - right_value,
        };
        Ok(calculation_result)
    }
}
