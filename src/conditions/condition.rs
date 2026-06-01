use crate::{error::result::StockTrekResult, resolved_context::ResolvedContext};

pub type Condition = Box<dyn ConditionTrait>;

#[typetag::serde]
pub trait ConditionTrait: Send + Sync {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool>;
}
