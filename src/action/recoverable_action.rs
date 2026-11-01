use crate::{
    action::action::Action, cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult, resolvable::Resolvable, resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use stock_trek_types::cex::capability::CexCapability;
use strum::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverableAction {
    action: Action,
    recovery_policy: RecoveryPolicy,
}

impl RecoverableAction {
    pub fn new(action: Action, recovery_policy: RecoveryPolicy) -> Self {
        Self {
            action,
            recovery_policy,
        }
    }
    pub fn enqueue(&self, c: &mut ResolvedContext) -> StockTrekResult<()> {
        let resolved_action = self.action.try_resolve(c)?;
        (c.enqueue_action)(&resolved_action, &self.recovery_policy)?;
        Ok(())
    }
}

impl HasRequiredCapabilities for RecoverableAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        self.action.required_capabilities()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPolicy {
    default_response: ActionErrorResponse,
    on_error: HashMap<ActionErrorCause, ActionErrorResponse>,
}

impl RecoveryPolicy {
    pub fn with_default_response(default_response: ActionErrorResponse) -> Self {
        Self {
            default_response,
            on_error: HashMap::new(),
        }
    }
    pub fn on_error(mut self, cause: ActionErrorCause, response: ActionErrorResponse) -> Self {
        self.on_error.insert(cause, response);
        self
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum ActionErrorCause {
    PermanentCexRejection,
    TemporaryCexRejection,
    InsufficientBalance,
    StaleAction,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum ActionErrorResponse {
    Stop,
    Ignore,
    Retry { max_retries: u8 },
    Instead { plan: Vec<RecoverableAction> },
}
