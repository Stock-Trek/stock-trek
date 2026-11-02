use crate::{
    Portfolio,
    action::{recoverable_action::RecoveryPolicy, resolved_action::ResolvedAction},
    error::result::StockTrekResult,
    signal::signals::Signals,
};

/// The context used when resolving actions and values.
pub struct ResolvedContext {
    enqueue_action: EnqueueActionFn,
    portfolio: Portfolio,
    signals: Signals,
}

/// Callback type for enqueuing a resolved action with a recovery policy.
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

    /// Returns the callback used to enqueue resolved actions.
    pub fn enqueue_action(&mut self) -> &mut EnqueueActionFn {
        &mut self.enqueue_action
    }

    /// Returns the portfolio used to resolve values and conditions.
    pub fn portfolio(&self) -> &Portfolio {
        &self.portfolio
    }

    /// Returns the signals used to resolve values and conditions.
    pub fn signals(&self) -> &Signals {
        &self.signals
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
