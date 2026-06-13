use crate::{
    actions::{recoverable_action::RecoveryPolicy, resolved_action::ResolvedAction},
    allocations::allocation::Allocation,
    error::result::StockTrekResult,
    portfolios::portfolio::Portfolio,
    signal::signals::Signals,
};

pub struct ResolvedContext {
    pub enqueue_action: EnqueueActionFn,
    pub allocation: Allocation,
    pub portfolio: Portfolio,
    pub signals: Signals,
}

pub type EnqueueActionFn =
    fn(resolved_action: &ResolvedAction, recovery_policy: &RecoveryPolicy) -> StockTrekResult<()>;
