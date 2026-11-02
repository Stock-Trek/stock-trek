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
/// Commands form a small expression language for strategies. They can branch on
/// [`Condition`]s, run commands in sequence, and enqueue actions. Command
/// construction is separate from execution so the same command can be
/// evaluated repeatedly as market data and the portfolio change.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Command {
    /// Executes one of two commands based on a condition.
    If {
        /// The condition to evaluate.
        condition: Condition,
        /// The command to execute when the condition is true.
        if_true: Box<Command>,
        /// The command to execute when the condition is false.
        if_false: Box<Command>,
    },
    /// Executes a list of commands in order.
    List {
        /// The commands to execute.
        commands: Vec<Command>,
    },
    /// Does nothing.
    NoOp,
    /// Enqueues a list of actions.
    Plan {
        /// The actions to enqueue, in order. Each action carries its own
        /// recovery policy.
        actions: Vec<RecoverableAction>,
    },
}

impl Command {
    /// Executes the command against the resolved context.
    ///
    /// Conditions are evaluated against `c`, and every action reached by the
    /// command is resolved and passed to `c.enqueue_action`. Execution stops at
    /// the first resolution or enqueue error.
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
#[derive(Debug, Clone, Default)]
pub struct CommandFactory;

impl CommandFactory {
    /// Creates a command that executes `if_true` when `condition` is true and
    /// `if_false` otherwise.
    pub fn if_else(&self, condition: Condition, if_true: Command, if_false: Command) -> Command {
        Command::If {
            condition,
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
        }
    }
    /// Creates a command that executes the given commands in order.
    pub fn list(&self, commands: Vec<Command>) -> Command {
        Command::List { commands }
    }

    /// Creates a command that does nothing. Useful as an empty branch.
    pub fn no_op(&self) -> Command {
        Command::NoOp
    }

    /// Creates a command that enqueues all of the given actions.
    pub fn plan(&self, actions: Vec<RecoverableAction>) -> Command {
        Command::Plan { actions }
    }
}
