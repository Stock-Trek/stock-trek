use crate::{
    conditions::condition::{Condition, ConditionTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuantityOf {
    All,
    Partial,
    None,
    Empty,
}

#[derive(Serialize, Deserialize)]
pub struct QuantityOfCondition {
    quantity_of: QuantityOf,
    conditions: Vec<Condition>,
}

impl QuantityOfCondition {
    pub fn new(quantity_of: QuantityOf, conditions: Vec<Condition>) -> Condition {
        Box::new(Self {
            quantity_of,
            conditions,
        })
    }
}

#[typetag::serde]
impl ConditionTrait for QuantityOfCondition {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        if self.conditions.is_empty() {
            let empty_result = match self.quantity_of {
                QuantityOf::All => true,
                QuantityOf::Partial => false,
                QuantityOf::None => true,
                QuantityOf::Empty => true,
            };
            return Ok(empty_result);
        }
        let mut true_count = 0;
        let mut false_count = 0;
        for condition in &self.conditions {
            if condition.test(c)? {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }
        let quantity = match self.quantity_of {
            QuantityOf::All => false_count == 0,
            QuantityOf::Partial => (true_count > 0) && (false_count > 0),
            QuantityOf::None => true_count == 0,
            QuantityOf::Empty => (true_count == 0) && (false_count == 0),
        };
        Ok(quantity)
    }
}
