use crate::{
    actions::action::Action,
    error::result::StockTrekResult,
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
    fn resolve(&self, _: &ResolvedContext, _: &mut Vec<Action>) -> StockTrekResult<()> {
        Ok(())
    }
}
