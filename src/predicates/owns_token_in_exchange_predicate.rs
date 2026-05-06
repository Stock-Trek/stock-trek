use crate::{
    error::result::StockTrekResult,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
    scratch::key::{ExchangeName, TokenName},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsTokenInExchangePredicate {
    token: TokenName,
    exchange: ExchangeName,
}

impl OwnsTokenInExchangePredicate {
    pub fn new(token: TokenName, exchange: ExchangeName) -> Predicate {
        Box::new(Self { token, exchange })
    }
}

#[typetag::serde]
impl PredicateTrait for OwnsTokenInExchangePredicate {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio
            .owns_token_in_exchange(&self.token, &self.exchange))
    }
}
