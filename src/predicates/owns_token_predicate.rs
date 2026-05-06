use crate::{
    error::result::StockTrekResult,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
    scratch::key::TokenName,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsTokenPredicate {
    token: TokenName,
}

impl OwnsTokenPredicate {
    pub fn new(token: TokenName) -> Predicate {
        Box::new(Self { token })
    }
}

#[typetag::serde]
impl PredicateTrait for OwnsTokenPredicate {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio.owns_token(&self.token))
    }
}
