use crate::{
    action::action::Action, cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult, resolvable::Resolvable, resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Debug};
use stock_trek_types::cex::capability::CexCapability;
use strum::Display;

/// An action paired with the policy used to recover from errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverableAction {
    action: Action,
    recovery_policy: RecoveryPolicy,
}

impl RecoverableAction {
    /// Creates a recoverable action from an action and a recovery policy.
    pub fn new(action: Action, recovery_policy: RecoveryPolicy) -> Self {
        Self {
            action,
            recovery_policy,
        }
    }

    /// Returns the action to execute.
    pub fn action(&self) -> &Action {
        &self.action
    }

    /// Returns how the runtime should respond if the action fails.
    pub fn recovery_policy(&self) -> &RecoveryPolicy {
        &self.recovery_policy
    }

    /// Resolves and enqueues the action on the resolved context.
    pub fn enqueue(&self, c: &mut ResolvedContext) -> StockTrekResult<()> {
        let resolved_action = self.action.try_resolve(c)?;
        (c.enqueue_action())(&resolved_action, &self.recovery_policy)?;
        Ok(())
    }
}

impl HasRequiredCapabilities for RecoverableAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        self.action.required_capabilities()
    }
}

/// Defines how an action should respond to different error causes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPolicy {
    default_response: ActionErrorResponse,
    on_error: BTreeMap<ActionErrorCause, ActionErrorResponse>,
}

impl RecoveryPolicy {
    /// Creates a policy using `default_response` for unlisted error causes.
    pub fn with_default_response(default_response: ActionErrorResponse) -> Self {
        Self {
            default_response,
            on_error: BTreeMap::new(),
        }
    }
    /// Maps a specific error cause to a response.
    pub fn with_error_response(
        mut self,
        cause: ActionErrorCause,
        response: ActionErrorResponse,
    ) -> Self {
        self.on_error.insert(cause, response);
        self
    }

    /// Returns the response used for causes without an explicit override.
    pub fn default_response(&self) -> &ActionErrorResponse {
        &self.default_response
    }

    /// Returns the responses that override the default for specific causes.
    pub fn on_error(&self) -> &BTreeMap<ActionErrorCause, ActionErrorResponse> {
        &self.on_error
    }
}

/// Causes of action errors that can trigger a recovery response.
#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ActionErrorCause {
    /// The CEX permanently rejected the action.
    PermanentCexRejection,
    /// The CEX temporarily rejected the action.
    TemporaryCexRejection,
    /// The account has insufficient balance for the action.
    InsufficientBalance,
    /// The action is no longer valid.
    StaleAction,
}

/// Responses that a recovery policy can use after an action error.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ActionErrorResponse {
    /// Stops processing the current plan.
    Stop,
    /// Ignores the error and continues.
    Ignore,
    /// Retries the action up to `max_retries` times.
    Retry {
        /// The maximum number of retry attempts.
        max_retries: u8,
    },
    /// Replaces the action with an alternative plan.
    Instead {
        /// The actions to enqueue instead of the failed action.
        plan: Vec<RecoverableAction>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_overrides_are_ordered_by_cause() {
        let policy = RecoveryPolicy::with_default_response(ActionErrorResponse::Stop)
            .with_error_response(ActionErrorCause::StaleAction, ActionErrorResponse::Ignore)
            .with_error_response(
                ActionErrorCause::InsufficientBalance,
                ActionErrorResponse::Ignore,
            )
            .with_error_response(
                ActionErrorCause::TemporaryCexRejection,
                ActionErrorResponse::Retry { max_retries: 1 },
            )
            .with_error_response(
                ActionErrorCause::PermanentCexRejection,
                ActionErrorResponse::Ignore,
            );

        let causes: Vec<_> = policy.on_error().keys().copied().collect();

        assert_eq!(
            causes,
            vec![
                ActionErrorCause::PermanentCexRejection,
                ActionErrorCause::TemporaryCexRejection,
                ActionErrorCause::InsufficientBalance,
                ActionErrorCause::StaleAction,
            ]
        );
    }
}
