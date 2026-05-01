use crate::{
    assembler_context::AssemblerContext,
    predicates::predicate::{Predicate, PredicateTrait},
};
use anyhow::Result;
use digdigdig3::Asset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetPredicate {
    asset: Asset,
}

impl OwnsAssetPredicate {
    pub fn new(asset: Asset) -> Predicate {
        Box::new(Self { asset })
    }
}

#[typetag::serde]
impl PredicateTrait for OwnsAssetPredicate {
    fn test(&self, context: &AssemblerContext) -> Result<bool> {
        Ok(context.portfolio.owns_asset(&self.asset))
    }
}
