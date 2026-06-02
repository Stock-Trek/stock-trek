use crate::{
    actions::recoverable_action::RecoverableAction,
    commands::{
        command::Command, if_command::IfCommand, list_command::ListCommand,
        no_op_command::NoOpCommand, plan_command::PlanCommand,
    },
    conditions::condition::Condition,
};

pub struct CommandFactory;

impl CommandFactory {
    pub fn if_else(&self, condition: Condition, if_true: Command, if_false: Command) -> Command {
        IfCommand::new(condition, if_true, if_false)
    }
    pub fn list(&self, commands: Vec<Command>) -> Command {
        ListCommand::new(commands)
    }
    pub fn no_op(&self) -> Command {
        NoOpCommand::new()
    }
    pub fn plan(&self, actions: Vec<RecoverableAction>) -> Command {
        PlanCommand::new(actions)
    }
}
