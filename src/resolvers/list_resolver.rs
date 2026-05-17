use crate::{
    capability::{Capability, HasRequiredCapabilities},
    error::result::StockTrekResult,
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
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        for resolver in &self.resolvers {
            resolver.resolve(c)?;
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
