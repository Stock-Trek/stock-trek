use crate::{
    commands::Command, preferences::Preferences, signal::signals::Signals,
    signal_context::SignalContext, strategy_context::StrategyContext,
};

pub trait Algorithm: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn preferences(&self) -> Preferences;
    fn signals(&self, c: &SignalContext) -> Signals;
    fn strategy(&self, c: &StrategyContext) -> Command;
}
