use crate::{
    assembler_context::AssemblerContext,
    predicates::predicate::{Predicate, PredicateTrait},
};
use anyhow::Result;
use digdigdig3::ExchangeId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HasAccountInExchangePredicate {
    exchange: ExchangeId,
}

impl HasAccountInExchangePredicate {
    pub fn new(exchange: ExchangeId) -> Predicate {
        Box::new(Self { exchange })
    }
}

#[typetag::serde]
impl PredicateTrait for HasAccountInExchangePredicate {
    fn test(&self, context: &AssemblerContext) -> Result<bool> {
        Ok(context.portfolio.has_account_in_exchange(self.exchange))
    }
}
