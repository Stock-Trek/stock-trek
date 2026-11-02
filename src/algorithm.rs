use crate::{
    commands::Command, error::result::StockTrekResult, preferences::Preferences,
    signal::signals::Signals, signal_context::SignalContext, strategy_context::StrategyContext,
};

/// A trading algorithm that produces signals and strategy commands.
#[allow(missing_docs)]
pub trait Algorithm: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn preferences(&self) -> Preferences;
    /// Produces the signals used by the algorithm for the given market context.
    /// This is run *every* time.
    fn signals(&self, c: &SignalContext) -> StockTrekResult<Signals>;
    /// Produces the Command that defines the strategy to follow.
    /// Note that this function is run *exactly once*.
    /// The Command definition it produces is then interpreted every time
    /// in response to the signals generated and the latest portfolio.
    fn strategy(&self, c: &StrategyContext) -> Command;
}
