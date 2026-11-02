use crate::{
    action::recoverable_action::RecoverableAction,
    cex::capability::{HasRequiredCapabilities, combine_capabilities},
    conditions::Condition,
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::capability::CexCapability;
use strum::Display;

/// A command produced by a strategy and executed against a resolved context.
///
/// Commands form a tree. `If` and `List` structure it, `Plan` enqueues
/// actions, and `NoOp` does nothing. Interpretation walks the tree in order,
/// evaluating conditions against the current portfolio and signals.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Command {
    /// Executes one of two commands based on a condition.
    If {
        /// The condition evaluated against the current context.
        condition: Condition,
        /// The command run when the condition holds.
        if_true: Box<Command>,
        /// The command run when the condition does not hold.
        if_false: Box<Command>,
    },
    /// Executes a list of commands in order.
    List {
        /// The commands run in order.
        commands: Vec<Command>,
    },
    /// Does nothing.
    NoOp,
    /// Enqueues a list of actions.
    Plan {
        /// The actions enqueued, in order, each with its recovery policy.
        actions: Vec<RecoverableAction>,
    },
}

impl Command {
    /// Interprets the command tree, enqueueing the actions it selects.
    ///
    /// Conditions are evaluated against the current portfolio and signals in
    /// `c`; enqueueing an action does not execute it, execution happens after
    /// interpretation. The first error aborts the walk, leaving already
    /// enqueued actions in place.
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
            } => combine_capabilities(&[if_true.as_ref(), if_false.as_ref()]),
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

/// Factory for constructing commands.
///
/// Available as the `commands` field of a [`StrategyContext`].
#[derive(Debug, Clone, Default)]
pub struct CommandFactory;

impl CommandFactory {
    /// Runs `if_true` when `condition` holds and `if_false` otherwise.
    pub fn if_else(&self, condition: Condition, if_true: Command, if_false: Command) -> Command {
        Command::If {
            condition,
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
        }
    }
    /// Runs each command in order.
    pub fn list(&self, commands: Vec<Command>) -> Command {
        Command::List { commands }
    }
    /// A command that does nothing.
    pub fn no_op(&self) -> Command {
        Command::NoOp
    }
    /// Enqueues each action when the branch is reached.
    pub fn plan(&self, actions: Vec<RecoverableAction>) -> Command {
        Command::Plan { actions }
    }
}
