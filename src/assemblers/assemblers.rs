use crate::{
    actions::action::Action,
    assemblers::{
        action_assembler::ActionAssembler, assembler::Assembler, if_assembler::IfAssembler,
        list_assembler::ListAssembler, no_op_assembler::NoOpAssembler,
    },
    predicates::predicate::Predicate,
};

pub struct Assemblers {}

impl Assemblers {
    pub fn action(action: Action) -> Assembler {
        ActionAssembler::new(action)
    }
    pub fn if_(condition: Predicate, if_true: Assembler, if_false: Assembler) -> Assembler {
        IfAssembler::new(condition, if_true, if_false)
    }
    pub fn list(assemblers: Vec<Assembler>) -> Assembler {
        ListAssembler::new(assemblers)
    }
    pub fn no_op() -> Assembler {
        NoOpAssembler::new()
    }
}
