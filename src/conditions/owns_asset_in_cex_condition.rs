use crate::{
    conditions::condition::{Condition, ConditionTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetInCexCondition {
    cex_id: CexId,
    asset_id: AssetId,
}

impl OwnsAssetInCexCondition {
    pub fn new(asset_id: AssetId, cex_id: CexId) -> Condition {
        Box::new(Self { asset_id, cex_id })
    }
}

#[typetag::serde]
impl ConditionTrait for OwnsAssetInCexCondition {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio.owns_asset_in_cex(&self.asset_id, &self.cex_id))
    }
}
