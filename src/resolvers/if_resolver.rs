use crate::{
    capability::{combine_capabilities, Capability, HasRequiredCapabilities},
    error::result::StockTrekResult,
    predicates::predicate::Predicate,
    resolved_context::ResolvedContext,
    resolvers::resolver::{Resolver, ResolverTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IfResolver {
    condition: Predicate,
    if_true: Resolver,
    if_false: Resolver,
}

impl IfResolver {
    pub fn new(condition: Predicate, if_true: Resolver, if_false: Resolver) -> Resolver {
        Box::new(Self {
            condition,
            if_true,
            if_false,
        })
    }
}

#[typetag::serde]
impl ResolverTrait for IfResolver {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        let predicate = self.condition.test(c)?;
        if predicate {
            self.if_true.resolve(c)?;
        } else {
            self.if_false.resolve(c)?;
        }
        Ok(())
    }
}

impl HasRequiredCapabilities for IfResolver {
    fn required_capabilities(&self) -> Vec<Capability> {
        combine_capabilities(&[self.if_false.as_ref(), self.if_true.as_ref()])
    }
}
