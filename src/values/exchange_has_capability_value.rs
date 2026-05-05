use crate::{
    error::result::StockTrekResult,
    exchanges::order_capability::OrderCapability,
    resolved_context::ResolvedContext,
    values::value::{ExchangeValue, FlagValue, FlagValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExchangeHasCapabilityValue {
    exchange: ExchangeValue,
    capability: OrderCapability,
}

impl ExchangeHasCapabilityValue {
    pub fn new(exchange: ExchangeValue, capability: OrderCapability) -> FlagValue {
        Box::new(Self {
            exchange,
            capability,
        })
    }
}

#[typetag::serde]
impl FlagValueTrait for ExchangeHasCapabilityValue {
    fn flag(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        let exchange = self.exchange.exchange(c)?;
        let has_capability = c
            .exchange_capabilities
            .has_capability(&exchange, &self.capability)
            .unwrap_or_default();
        Ok(has_capability)
    }
}
