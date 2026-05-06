use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    resolvers::resolver::{Resolver, ResolverTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListResolver {
    assemblers: Vec<Resolver>,
}

impl ListResolver {
    pub fn new(assemblers: Vec<Resolver>) -> Resolver {
        Box::new(Self { assemblers })
    }
}

#[typetag::serde]
impl ResolverTrait for ListResolver {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        for assembler in &self.assemblers {
            assembler.resolve(c)?;
        }
        Ok(())
    }
}
