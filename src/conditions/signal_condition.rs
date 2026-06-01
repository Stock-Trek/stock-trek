use crate::{
    conditions::condition::ConditionTrait, error::result::StockTrekResult,
    resolved_context::ResolvedContext, signal::key::SignalKey,
};

#[typetag::serde]
impl ConditionTrait for SignalKey<bool> {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        c.signals.read(self)
    }
}
