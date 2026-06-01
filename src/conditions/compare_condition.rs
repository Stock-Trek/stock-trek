use crate::{
    conditions::condition::{Condition, ConditionTrait},
    error::{
        general::GeneralError,
        result::{StockTrekError, StockTrekResult},
    },
    resolved_context::ResolvedContext,
    util::serde_ordering,
    values::value::NumberValue,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
pub struct CompareCondition {
    left: NumberValue,
    #[serde(with = "serde_ordering")]
    comparison: Ordering,
    right: NumberValue,
}

impl CompareCondition {
    pub fn new(left: NumberValue, comparison: Ordering, right: NumberValue) -> Condition {
        Box::new(Self {
            left,
            comparison,
            right,
        })
    }
}

#[typetag::serde]
impl ConditionTrait for CompareCondition {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        let left_value = self.left.number(c)?;
        let right_value = self.right.number(c)?;
        match left_value.partial_cmp(&right_value) {
            Some(Ordering::Less) => Ok(self.comparison.is_le()),
            Some(Ordering::Equal) => Ok(self.comparison.is_eq()),
            Some(Ordering::Greater) => Ok(self.comparison.is_ge()),
            None => Err(StockTrekError::General(GeneralError::Message(format!(
                "Failed to compare {} and {}",
                left_value, right_value
            )))),
        }
    }
}
