use crate::{
    commands::command::Command, preferences::Preferences, signal::signals::Signals,
    signal_context::SignalContext, strategy_context::StrategyContext,
};

pub trait Algorithm: Send + Sync {
    fn preferences(&self) -> Preferences;
    fn signals(&self, c: &SignalContext) -> Signals;
    fn strategy(&self, c: &StrategyContext) -> Command;
}
