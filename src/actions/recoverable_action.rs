use crate::{
    actions::action::Action,
    cex::capability::{CexCapability, HasRequiredCapabilities},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::Display;

#[derive(Serialize, Deserialize)]
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
    pub fn enqueue(&self, c: &ResolvedContext) -> StockTrekResult<()> {
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

#[derive(Serialize, Deserialize)]
pub struct RecoveryPolicy {
    default: ErrorResponse,
    on_error: HashMap<ErrorCause, ErrorResponse>,
}

impl RecoveryPolicy {
    pub fn with_default(default: ErrorResponse) -> Self {
        Self {
            default,
            on_error: HashMap::new(),
        }
    }
    pub fn on_error(mut self, cause: ErrorCause, response: ErrorResponse) -> Self {
        self.on_error.insert(cause, response);
        self
    }
}

#[derive(Display, Serialize, Deserialize)]
pub enum ErrorResponse {
    Stop,
    Ignore,
    Retry { max_retries: u8 },
    Instead { plan: Vec<RecoverableAction> },
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCause {
    PermanentCexRejection,
    TemporaryCexRejection,
    InsufficientBalance,
    StaleAction,
}
