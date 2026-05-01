use crate::{actions::action::Action, assembler_context::AssemblerContext};
use anyhow::Result;

pub type Assembler = Box<dyn AssemblerTrait>;

#[typetag::serde]
pub trait AssemblerTrait: Send + Sync {
    fn assemble(&self, context: &AssemblerContext, actions: &mut Vec<Action>) -> Result<()>;
}
