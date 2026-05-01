use crate::{
    actions::action::Action,
    assembler_context::AssemblerContext,
    assemblers::assembler::{Assembler, AssemblerTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoOpAssembler {}

impl NoOpAssembler {
    pub fn new() -> Assembler {
        Box::new(Self {})
    }
}

#[typetag::serde]
impl AssemblerTrait for NoOpAssembler {
    fn assemble(&self, _: &AssemblerContext, _: &mut Vec<Action>) -> Result<()> {
        Ok(())
    }
}
