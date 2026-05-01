use crate::{
    assembler_context::AssemblerContext,
    values::value::{ExchangeValue, NumberValue, NumberValueTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashInExchangeValue {
    exchange: ExchangeValue,
}

impl CashInExchangeValue {
    pub fn new(exchange: ExchangeValue) -> NumberValue {
        Box::new(Self { exchange })
    }
}

#[typetag::serde]
impl NumberValueTrait for CashInExchangeValue {
    fn number(&self, context: &AssemblerContext) -> Result<f64> {
        let exchange = self.exchange.exchange(context)?;
        Ok(context.portfolio.cash_in_exchange(exchange))
    }
}
