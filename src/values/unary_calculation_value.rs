use crate::{
    assembler_context::AssemblerContext,
    values::value::{NumberValue, NumberValueTrait},
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnaryOperator {
    Abs,
    Neg,
    Floor,
    Ceil,
    RoundAway0,
    RoundToEven,
    Trunc,
    Frac,
    Sqrt,
    Exp,
    Exp2,
    Log2,
    LogE,
    Log10,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
}

#[derive(Serialize, Deserialize)]
pub struct UnaryCalculationValue {
    number: NumberValue,
    operator: UnaryOperator,
}

impl UnaryCalculationValue {
    pub fn new(number: NumberValue, operator: UnaryOperator) -> NumberValue {
        Box::new(Self { number, operator })
    }
}

#[typetag::serde]
impl NumberValueTrait for UnaryCalculationValue {
    fn number(&self, context: &AssemblerContext) -> Result<f64> {
        let value = self.number.number(context)?;
        let calculation_result = match self.operator {
            UnaryOperator::Abs => value.abs(),
            UnaryOperator::Acos => {
                if value < -1.0 {
                    bail!(
                        "Acos: A value <-1 {} is not supported, it would give a complex number",
                        value
                    );
                }
                if value > 1.0 {
                    bail!(
                        "Acos: A value >1 {} is not supported, it would give a complex number",
                        value
                    );
                }
                value.acos()
            }
            UnaryOperator::Acosh => {
                if value < 1.0 {
                    bail!(
                        "Acos: A value <1 {} is not supported, it would give a complex number",
                        value
                    );
                }
                value.acosh()
            }
            UnaryOperator::Asin => {
                if value < -1.0 {
                    bail!(
                        "Asin: A value <-1 {} is not supported, it would give a complex number",
                        value
                    );
                }
                if value > 1.0 {
                    bail!(
                        "Asin: A value >1 {} is not supported, it would give a complex number",
                        value
                    );
                }
                value.asin()
            }
            UnaryOperator::Asinh => value.asinh(),
            UnaryOperator::Atan => value.atan(),
            UnaryOperator::Atanh => {
                if value == -1.0 {
                    bail!("Atanh: A value =-1 is not supported, it would give negative infinity");
                }
                if value == 1.0 {
                    bail!("Atanh: A value =1 is not supported, it would give positive infinity");
                }
                if value < -1.0 {
                    bail!(
                        "Atanh: A value <-1 {} is not supported, it would give a complex number",
                        value
                    );
                }
                if value > 1.0 {
                    bail!(
                        "Atanh: A value >1 {} is not supported, it would give a complex number",
                        value
                    );
                }
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
                if value == 0.0 {
                    bail!("Log10: A value =0 is not supported, it would give undefined");
                }
                if value < 0.0 {
                    bail!(
                        "Log10: A value <0 {} is not supported, it would give a complex number",
                        value
                    );
                }
                value.log10()
            }
            UnaryOperator::Log2 => {
                if value == 0.0 {
                    bail!("Log2: A value =0 is not supported, it would give undefined");
                }
                if value < 0.0 {
                    bail!(
                        "Log2: A value <0 {} is not supported, it would give a complex number",
                        value
                    );
                }
                value.log2()
            }
            UnaryOperator::LogE => {
                if value == 0.0 {
                    bail!("LogE: A value =0 is not supported, it would give undefined");
                }
                if value < 0.0 {
                    bail!(
                        "LogE: A value <0 {} is not supported, it would give a complex number",
                        value
                    );
                }
                value.ln()
            }
            UnaryOperator::Neg => -value,
            UnaryOperator::RoundAway0 => value.round(),
            UnaryOperator::RoundToEven => value.round_ties_even(),
            UnaryOperator::Sin => value.sin(),
            UnaryOperator::Sinh => value.sinh(),
            UnaryOperator::Sqrt => {
                if value < 0.0 {
                    bail!(
                        "Sqrt: A value <0 {} is not supported, it would give a complex number",
                        value
                    );
                }
                value.sqrt()
            }
            UnaryOperator::Tan => value.tan(),
            UnaryOperator::Tanh => value.tanh(),
            UnaryOperator::Trunc => value.trunc(),
        };
        Ok(calculation_result)
    }
}
