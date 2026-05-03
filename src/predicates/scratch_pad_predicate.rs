use crate::{
    error::result::StockTrekResult, predicates::predicate::PredicateTrait,
    resolved_context::ResolvedContext, scratch::key::ScratchKey,
};

#[typetag::serde]
impl PredicateTrait for ScratchKey<bool> {
    fn test(&self, context: &ResolvedContext) -> StockTrekResult<bool> {
        context.scratch_pad.read(self)
    }
}
