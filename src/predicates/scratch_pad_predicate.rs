use crate::{
    assembler_context::AssemblerContext,
    predicates::predicate::{Predicate, PredicateTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScratchPadPredicate {
    key: String,
}

impl ScratchPadPredicate {
    pub fn new(key: String) -> Predicate {
        Box::new(Self { key })
    }
}

#[typetag::serde]
impl PredicateTrait for ScratchPadPredicate {
    fn test(&self, context: &AssemblerContext) -> Result<bool> {
        context.scratch_pad.read_required::<bool>(&self.key)
    }
}
