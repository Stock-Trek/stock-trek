use crate::{
    action::recoverable_action::RecoverableAction,
    cex::capability::{HasRequiredCapabilities, combine_capabilities},
    conditions::Condition,
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::capability::CexCapability;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    If {
        condition: Condition,
        if_true: Box<Command>,
        if_false: Box<Command>,
    },
    List {
        commands: Vec<Command>,
    },
    NoOp,
    Plan {
        actions: Vec<RecoverableAction>,
    },
}

impl Command {
    pub fn execute(&self, c: &mut ResolvedContext) -> StockTrekResult<()> {
        match self {
            Command::If {
                condition,
                if_true,
                if_false,
            } => {
                let condition = condition.test(c)?;
                if condition {
                    if_true.execute(c)
                } else {
                    if_false.execute(c)
                }
            }
            Command::List { commands } => {
                for command in commands {
                    command.execute(c)?;
                }
                Ok(())
            }
            Command::NoOp => Ok(()),
            Command::Plan { actions } => {
                for action in actions {
                    action.enqueue(c)?;
                }
                Ok(())
            }
        }
    }
}

impl HasRequiredCapabilities for Command {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        match self {
            Command::If {
                if_true, if_false, ..
            } => combine_capabilities(&[if_false.as_ref(), if_true.as_ref()]),
            Command::List { commands } => {
                let mut capabilities = Vec::new();
                for command in commands {
                    capabilities.extend(command.required_capabilities());
                }
                capabilities
            }
            Command::NoOp => Vec::new(),
            Command::Plan { actions } => {
                let mut capabilities = Vec::new();
                for action in actions {
                    capabilities.extend(action.required_capabilities());
                }
                capabilities
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandFactory;

impl CommandFactory {
    pub fn if_else(&self, condition: Condition, if_true: Command, if_false: Command) -> Command {
        Command::If {
            condition,
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
        }
    }
    pub fn list(&self, commands: Vec<Command>) -> Command {
        Command::List { commands }
    }
    pub fn no_op(&self) -> Command {
        Command::NoOp
    }
    pub fn plan(&self, actions: Vec<RecoverableAction>) -> Command {
        Command::Plan { actions }
    }
}
