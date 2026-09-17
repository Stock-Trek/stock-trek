use crate::{
    Portfolio,
    action::{recoverable_action::RecoveryPolicy, resolved_action::ResolvedAction},
    error::result::StockTrekResult,
    signal::signals::Signals,
};

pub struct ResolvedContext {
    pub enqueue_action: EnqueueActionFn,
    pub portfolio: Portfolio,
    pub signals: Signals,
}

pub type EnqueueActionFn = Box<dyn FnMut(&ResolvedAction, &RecoveryPolicy) -> StockTrekResult<()>>;
