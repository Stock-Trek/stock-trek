use crate::{
    asset_id::AssetId,
    conditions::condition::{Condition, ConditionTrait},
    error::result::StockTrekResult,
    exchange_id::ExchangeId,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetInExchangeCondition {
    exchange_id: ExchangeId,
    asset_id: AssetId,
}

impl OwnsAssetInExchangeCondition {
    pub fn new(asset_id: AssetId, exchange_id: ExchangeId) -> Condition {
        Box::new(Self {
            asset_id,
            exchange_id,
        })
    }
}

#[typetag::serde]
impl ConditionTrait for OwnsAssetInExchangeCondition {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio
            .owns_asset_in_exchange(&self.asset_id, &self.exchange_id))
    }
}
