use crate::{
    commands::Command, preferences::Preferences, signal::signals::Signals,
    signal_context::SignalContext, strategy_context::StrategyContext,
};

/// A trading algorithm that produces signals and strategy commands.
pub trait Algorithm: Send + Sync {
    /// Returns the name of the algorithm.
    fn name(&self) -> &str;
    /// Returns a description of the algorithm.
    fn description(&self) -> &str;
    /// Returns the preferences used when running the algorithm.
    fn preferences(&self) -> Preferences;
    /// Produces the signals used by the algorithm for the given market context.
    fn signals(&self, c: &SignalContext) -> Signals;
    /// Produces the strategy command for the given strategy context.
    fn strategy(&self, c: &StrategyContext) -> Command;
}
