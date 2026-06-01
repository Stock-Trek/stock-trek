use crate::{
    conditions::condition::{Condition, ConditionTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NotCondition {
    condition: Condition,
}

impl NotCondition {
    pub fn new(condition: Condition) -> Condition {
        Box::new(Self { condition })
    }
}

#[typetag::serde]
impl ConditionTrait for NotCondition {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        let test_result = self.condition.test(c)?;
        Ok(!test_result)
    }
}
