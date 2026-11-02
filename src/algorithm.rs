use crate::{
    commands::Command, error::result::StockTrekResult, preferences::Preferences,
    signal::signals::Signals, signal_context::SignalContext, strategy_context::StrategyContext,
};

/// A trading algorithm that produces signals and strategy commands.
///
/// The runtime calls [`Algorithm::signals`] on every evaluation and
/// [`Algorithm::strategy`] once, then executes the returned command against the
/// latest signals and portfolio. Implementations are usually stateless: all
/// state that influences a decision should be derived from market data,
/// signals, or the portfolio.
///
/// Register an implementation with [`crate::register_algorithm`] so the runtime
/// can instantiate it.
pub trait Algorithm: Send + Sync {
    /// Returns the algorithm's name for display and identification.
    fn name(&self) -> &str;

    /// Returns a short human-readable description of what the algorithm does.
    fn description(&self) -> &str;

    /// Returns the runtime preferences applied to orders produced by this
    /// algorithm, such as rounding strategies and the maximum tolerated network
    /// delay.
    fn preferences(&self) -> Preferences;

    /// Produces the signals used by the algorithm for the given market context.
    ///
    /// This runs on every evaluation, before the strategy command is executed.
    /// Signals not written during an evaluation are absent from that
    /// evaluation, so a strategy should use optional keys with defaults for
    /// values that may be unavailable.
    fn signals(&self, c: &SignalContext) -> StockTrekResult<Signals>;

    /// Produces the [`Command`] that defines the strategy to follow.
    ///
    /// This runs exactly once. The returned command is interpreted on every
    /// evaluation in response to the latest signals and portfolio. Build
    /// values, conditions, and actions with the factories on [`StrategyContext`]
    /// rather than reading state directly, so the command can be re-evaluated
    /// as conditions change.
    fn strategy(&self, c: &StrategyContext) -> Command;
}
