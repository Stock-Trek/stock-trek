use crate::{
    actions::action::Action,
    assembler_context::AssemblerContext,
    assemblers::assembler::{Assembler, AssemblerTrait},
    predicates::predicate::Predicate,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IfAssembler {
    condition: Predicate,
    if_true: Assembler,
    if_false: Assembler,
}

impl IfAssembler {
    pub fn new(condition: Predicate, if_true: Assembler, if_false: Assembler) -> Assembler {
        Box::new(Self {
            condition,
            if_true,
            if_false,
        })
    }
}

#[typetag::serde]
impl AssemblerTrait for IfAssembler {
    fn assemble(&self, context: &AssemblerContext, actions: &mut Vec<Action>) -> Result<()> {
        let predicate = self.condition.test(context)?;
        if predicate {
            self.if_true.assemble(context, actions)?;
        } else {
            self.if_false.assemble(context, actions)?;
        }
        Ok(())
    }
}
