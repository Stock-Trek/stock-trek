use crate::{
    Portfolio,
    action::{recoverable_action::RecoveryPolicy, resolved_action::ResolvedAction},
    error::result::StockTrekResult,
    signal::signals::Signals,
};

/// The context used when interpreting a command tree.
///
/// Carries the latest portfolio and signals, plus the callback that receives
/// the actions the tree enqueues.
pub struct ResolvedContext {
    /// Receives each action as it is enqueued, together with the recovery
    /// policy the action should follow on error.
    pub enqueue_action: EnqueueActionFn,
    /// The latest portfolio snapshot.
    pub portfolio: Portfolio,
    /// The signals from the latest
    /// [`Algorithm::signals`](crate::Algorithm::signals) evaluation.
    pub signals: Signals,
}

/// Receives a resolved action and its recovery policy during interpretation.
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
