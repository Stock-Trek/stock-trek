use crate::{
    action::action::Action, cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult, resolvable::Resolvable, resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Debug};
use stock_trek_types::cex::capability::CexCapability;
use strum::Display;

/// An action paired with the policy used to recover from errors.
///
/// Build these through [`ActionFactory`](crate::actions::ActionFactory) so the
/// action and its recovery policy stay together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverableAction {
    /// The action to execute.
    pub action: Action,
    /// How the runtime should respond if the action fails.
    pub recovery_policy: RecoveryPolicy,
}

impl RecoverableAction {
    /// Creates a recoverable action from an action and a recovery policy.
    pub fn new(action: Action, recovery_policy: RecoveryPolicy) -> Self {
        Self {
            action,
            recovery_policy,
        }
    }
    /// Resolves the action's values and passes it to the context's enqueue
    /// callback together with the recovery policy.
    ///
    /// Resolution can fail before the action reaches the runtime, for example
    /// when a required signal is missing or an order price is not positive.
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

/// Defines how an action should respond to different error causes.
///
/// The policy has a default response plus optional overrides for specific
/// causes. For example, a strategy can retry temporary exchange rejections while
/// stopping on permanent ones. Build policies with
/// [`RecoveryPolicy::with_default_response`] and
/// [`RecoveryPolicy::on_error`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPolicy {
    /// The response used for causes that have no explicit override.
    pub default_response: ActionErrorResponse,
    /// Responses that override the default for specific causes.
    pub on_error: BTreeMap<ActionErrorCause, ActionErrorResponse>,
}

impl RecoveryPolicy {
    /// Returns a new policy that uses `default_response` for unlisted causes.
    pub fn with_default_response(default_response: ActionErrorResponse) -> Self {
        Self {
            default_response,
            on_error: BTreeMap::new(),
        }
    }
    /// Maps a specific error cause to a response.
    pub fn on_error(mut self, cause: ActionErrorCause, response: ActionErrorResponse) -> Self {
        self.on_error.insert(cause, response);
        self
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
            .on_error(ActionErrorCause::StaleAction, ActionErrorResponse::Ignore)
            .on_error(
                ActionErrorCause::InsufficientBalance,
                ActionErrorResponse::Ignore,
            )
            .on_error(
                ActionErrorCause::TemporaryCexRejection,
                ActionErrorResponse::Retry { max_retries: 1 },
            )
            .on_error(
                ActionErrorCause::PermanentCexRejection,
                ActionErrorResponse::Ignore,
            );

        let causes: Vec<_> = policy.on_error.keys().copied().collect();

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
