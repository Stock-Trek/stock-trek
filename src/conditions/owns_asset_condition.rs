use crate::{
    conditions::condition::{Condition, ConditionTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::asset_id::AssetId;

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetCondition {
    asset_id: AssetId,
}

impl OwnsAssetCondition {
    pub fn new(asset_id: AssetId) -> Condition {
        Box::new(Self { asset_id })
    }
}

#[typetag::serde]
impl ConditionTrait for OwnsAssetCondition {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio.owns_asset(&self.asset_id))
    }
}
