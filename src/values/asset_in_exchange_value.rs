use crate::{
    assembler_context::AssemblerContext,
    values::value::{AssetValue, ExchangeValue, NumberValue, NumberValueTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssetInExchangeValue {
    exchange: ExchangeValue,
    asset: AssetValue,
}

impl AssetInExchangeValue {
    pub fn new(exchange: ExchangeValue, asset: AssetValue) -> NumberValue {
        Box::new(Self { exchange, asset })
    }
}

#[typetag::serde]
impl NumberValueTrait for AssetInExchangeValue {
    fn number(&self, context: &AssemblerContext) -> Result<f64> {
        let exchange = self.exchange.exchange(context)?;
        let asset = self.asset.asset(context)?;
        Ok(context.portfolio.asset_in_exchange(asset, exchange))
    }
}
