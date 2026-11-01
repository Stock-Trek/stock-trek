use crate::{
    commands::Command, error::result::StockTrekResult, preferences::Preferences,
    signal::signals::Signals, signal_context::SignalContext, strategy_context::StrategyContext,
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
    /// This is run *every* time.
    ///
    /// Returns an error when the available market data is insufficient to
    /// produce the signals.
    fn signals(&self, c: &SignalContext) -> StockTrekResult<Signals>;
    /// Produces the Command that defines the strategy to follow.
    /// Note that this function is run *exactly once*.
    /// The Command definition it produces is then interpreted every time
    /// in response to the signals generated and the latest portfolio.
    fn strategy(&self, c: &StrategyContext) -> Command;
}
