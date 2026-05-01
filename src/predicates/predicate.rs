use crate::assembler_context::AssemblerContext;
use anyhow::Result;

pub type Predicate = Box<dyn PredicateTrait>;

#[typetag::serde]
pub trait PredicateTrait: Send + Sync {
    fn test(&self, context: &AssemblerContext) -> Result<bool>;
}
