use crate::{
    assembler_context::AssemblerContext,
    values::value::{NumberValue, NumberValueTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashTotalValue {}

impl CashTotalValue {
    pub fn new() -> NumberValue {
        Box::new(Self {})
    }
}

#[typetag::serde]
impl NumberValueTrait for CashTotalValue {
    fn number(&self, context: &AssemblerContext) -> Result<f64> {
        Ok(context.portfolio.cash_total())
    }
}
