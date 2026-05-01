use crate::{
    actions::action::Action,
    assembler_context::AssemblerContext,
    assemblers::assembler::{Assembler, AssemblerTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListAssembler {
    assemblers: Vec<Assembler>,
}

impl ListAssembler {
    pub fn new(assemblers: Vec<Assembler>) -> Assembler {
        Box::new(Self { assemblers })
    }
}

#[typetag::serde]
impl AssemblerTrait for ListAssembler {
    fn assemble(&self, context: &AssemblerContext, actions: &mut Vec<Action>) -> Result<()> {
        for assembler in &self.assemblers {
            assembler.assemble(context, actions)?;
        }
        Ok(())
    }
}
