use crate::{
    conditions::condition::{Condition, ConditionTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::cex_id::CexId;

#[derive(Serialize, Deserialize)]
pub struct HasAccountInCexCondition {
    cex_id: CexId,
}

impl HasAccountInCexCondition {
    pub fn new(cex_id: CexId) -> Condition {
        Box::new(Self { cex_id })
    }
}

#[typetag::serde]
impl ConditionTrait for HasAccountInCexCondition {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio.has_account_in_cex(&self.cex_id))
    }
}
