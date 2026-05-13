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
pub struct ListResolver {
    resolvers: Vec<Resolver>,
}

impl ListResolver {
    pub fn new(resolvers: Vec<Resolver>) -> Resolver {
        Box::new(Self { resolvers })
    }
}

#[typetag::serde]
impl ResolverTrait for ListResolver {
    fn resolve(
        &self,
        c: &ResolvedContext,
        order_requests: &mut Vec<OrderRequest<AssetId, f64>>,
    ) -> StockTrekResult<()> {
        for resolver in &self.resolvers {
            resolver.resolve(c, order_requests)?;
        }
        Ok(())
    }
}

impl HasRequiredCapabilities for ListResolver {
    fn required_capabilities(&self) -> Vec<Capability> {
        let mut capabilities = Vec::new();
        for resolver in &self.resolvers {
            capabilities.extend(resolver.required_capabilities());
        }
        capabilities
    }
}
