use crate::{
    assembler_context::AssemblerContext,
    predicates::predicate::{Predicate, PredicateTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuantityOf {
    All,
    Partial,
    None,
    Empty,
}

#[derive(Serialize, Deserialize)]
pub struct QuantityOfPredicate {
    quantity_of: QuantityOf,
    predicates: Vec<Predicate>,
}

impl QuantityOfPredicate {
    pub fn new(quantity_of: QuantityOf, predicates: Vec<Predicate>) -> Predicate {
        Box::new(Self {
            quantity_of,
            predicates,
        })
    }
}

#[typetag::serde]
impl PredicateTrait for QuantityOfPredicate {
    fn test(&self, context: &AssemblerContext) -> Result<bool> {
        if self.predicates.is_empty() {
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
        for predicate in &self.predicates {
            if predicate.test(context)? {
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
