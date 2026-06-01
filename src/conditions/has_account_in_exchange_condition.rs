use crate::{
    conditions::condition::{Condition, ConditionTrait},
    error::result::StockTrekResult,
    exchange_id::ExchangeId,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HasAccountInExchangeCondition {
    exchange_id: ExchangeId,
}

impl HasAccountInExchangeCondition {
    pub fn new(exchange_id: ExchangeId) -> Condition {
        Box::new(Self { exchange_id })
    }
}

#[typetag::serde]
impl ConditionTrait for HasAccountInExchangeCondition {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio.has_account_in_exchange(&self.exchange_id))
    }
}
