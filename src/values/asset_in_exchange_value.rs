use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{AssetValue, ExchangeValue, NumberValue, NumberValueTrait},
};
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
    fn number(&self, context: &ResolvedContext) -> StockTrekResult<f64> {
        let exchange = self.exchange.exchange(context)?;
        let asset = self.asset.asset(context)?;
        Ok(context.portfolio.asset_in_exchange(&asset, &exchange))
    }
}
