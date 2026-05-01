use crate::{
    assembler_context::AssemblerContext,
    predicates::predicate::{Predicate, PredicateTrait},
    util::serde_ordering,
    values::value::NumberValue,
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
pub struct ComparePredicate {
    left: NumberValue,
    #[serde(with = "serde_ordering")]
    comparison: Ordering,
    right: NumberValue,
}

impl ComparePredicate {
    pub fn new(left: NumberValue, comparison: Ordering, right: NumberValue) -> Predicate {
        Box::new(Self {
            left,
            comparison,
            right,
        })
    }
}

#[typetag::serde]
impl PredicateTrait for ComparePredicate {
    fn test(&self, context: &AssemblerContext) -> Result<bool> {
        let left_value = self.left.number(context)?;
        let right_value = self.right.number(context)?;
        match left_value.partial_cmp(&right_value) {
            Some(Ordering::Less) => Ok(self.comparison.is_le()),
            Some(Ordering::Equal) => Ok(self.comparison.is_eq()),
            Some(Ordering::Greater) => Ok(self.comparison.is_ge()),
            None => bail!("Failed to compare {} and {}", left_value, right_value),
        }
    }
}
