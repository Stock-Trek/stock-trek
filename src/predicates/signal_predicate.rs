use crate::{
    error::result::StockTrekResult, predicates::predicate::PredicateTrait,
    resolved_context::ResolvedContext, signal::key::SignalKey,
};

#[typetag::serde]
impl PredicateTrait for SignalKey<bool> {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        c.signals.read(self)
    }
}
