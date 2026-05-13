use crate::{
    asset_id::AssetId,
    capability::{Capability, HasRequiredCapabilities},
    error::result::StockTrekResult,
    order::order_request::OrderRequest,
    resolved_context::ResolvedContext,
    resolvers::resolver::{Resolver, ResolverTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoOpResolver;

impl NoOpResolver {
    pub fn new() -> Resolver {
        Box::new(Self {})
    }
}

#[typetag::serde]
impl ResolverTrait for NoOpResolver {
    fn resolve(
        &self,
        _: &ResolvedContext,
        _: &mut Vec<OrderRequest<AssetId, f64>>,
    ) -> StockTrekResult<()> {
        Ok(())
    }
}

impl HasRequiredCapabilities for NoOpResolver {
    fn required_capabilities(&self) -> Vec<Capability> {
        Vec::new()
    }
}
