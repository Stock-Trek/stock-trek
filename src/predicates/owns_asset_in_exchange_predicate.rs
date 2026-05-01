use crate::{
    assembler_context::AssemblerContext,
    predicates::predicate::{Predicate, PredicateTrait},
};
use anyhow::Result;
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetInExchangePredicate {
    asset: Asset,
    exchange: ExchangeId,
}

impl OwnsAssetInExchangePredicate {
    pub fn new(asset: Asset, exchange: ExchangeId) -> Predicate {
        Box::new(Self { asset, exchange })
    }
}

#[typetag::serde]
impl PredicateTrait for OwnsAssetInExchangePredicate {
    fn test(&self, context: &AssemblerContext) -> Result<bool> {
        Ok(context
            .portfolio
            .owns_asset_in_exchange(&self.asset, &self.exchange))
    }
}
