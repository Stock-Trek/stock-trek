use crate::{
    actions::action::Action,
    assembler_context::AssemblerContext,
    assemblers::assembler::{Assembler, AssemblerTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ActionAssembler {
    action: Action,
}

impl ActionAssembler {
    pub fn new(action: Action) -> Assembler {
        Box::new(Self { action })
    }
}

#[typetag::serde]
impl AssemblerTrait for ActionAssembler {
    fn assemble(&self, _context: &AssemblerContext, actions: &mut Vec<Action>) -> Result<()> {
        actions.push(self.action.clone_box());
        Ok(())
    }
}
