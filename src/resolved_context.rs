use crate::{
    Portfolio,
    action::{recoverable_action::RecoveryPolicy, resolved_action::ResolvedAction},
    error::result::StockTrekResult,
    signal::signals::Signals,
};

/// The context used when resolving actions and values.
///
/// A resolved context contains the portfolio and signals for the current
/// evaluation plus a callback that receives resolved actions. Commands use it
/// to read values and enqueue actions without depending on the runtime that
/// owns the exchange connections.
pub struct ResolvedContext {
    /// Callback invoked for each action produced by a command.
    pub enqueue_action: EnqueueActionFn,
    /// Portfolio balances and pending orders visible to the current evaluation.
    pub portfolio: Portfolio,
    /// Signals produced for the current evaluation.
    pub signals: Signals,
}

/// Callback type for enqueuing a resolved action with a recovery policy.
///
/// The runtime supplies this callback and is responsible for executing the
/// action and applying the recovery policy on failure.
pub type EnqueueActionFn = Box<dyn FnMut(&ResolvedAction, &RecoveryPolicy) -> StockTrekResult<()>>;

impl ResolvedContext {
    /// Creates a resolved context from an action callback, portfolio, and signals.
    pub fn new(enqueue_action: EnqueueActionFn, portfolio: Portfolio, signals: Signals) -> Self {
        Self {
            enqueue_action,
            portfolio,
            signals,
        }
    }
}

impl std::fmt::Debug for ResolvedContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolvedContext")
            .field("portfolio", &self.portfolio)
            .field("signals", &self.signals)
            .finish_non_exhaustive()
    }
}
