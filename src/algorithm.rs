use crate::{
    commands::Command, error::result::StockTrekResult, preferences::Preferences,
    signal::signals::Signals, signal_context::SignalContext, strategy_context::StrategyContext,
};

/// A trading algorithm.
///
/// Implement this trait and register the implementation with the
/// `#[register_algorithm(default)]` annotation. The platform drives two
/// phases:
///
/// - [`Self::signals`] runs on every evaluation and turns the market data in
///   the [`SignalContext`] into a [`Signals`] map.
/// - [`Self::strategy`] runs exactly once, before the first evaluation, and
///   returns the [`Command`] tree that the platform interprets on every
///   evaluation thereafter. Values read from signals or the portfolio inside
///   the command tree are resolved anew on each interpretation, so the tree
///   must be built from the [`StrategyContext`] factories rather than from
///   values captured when the strategy phase runs.
pub trait Algorithm: Send + Sync {
    /// A stable identifier for the algorithm, used in registration and when
    /// selecting among registered algorithms.
    fn name(&self) -> &str;
    /// A human-readable summary of the trading behaviour, shown to users.
    fn description(&self) -> &str;
    /// Rounding and network preferences applied to the algorithm's orders.
    fn preferences(&self) -> Preferences;
    /// Computes this evaluation's signals from the market data of every CEX.
    ///
    /// Called on every evaluation, before the command tree is interpreted.
    /// Writing the same keys on every call keeps later reads predictable:
    /// required [`SignalKey`](crate::signals::SignalKey)s error when absent,
    /// so a signal that is only
    /// sometimes available should be paired with an optional key carrying a
    /// default.
    fn signals(&self, c: &SignalContext) -> StockTrekResult<Signals>;
    /// Builds the command tree that defines the strategy to follow.
    ///
    /// Called exactly once. The returned command tree is interpreted on every
    /// evaluation against the latest signals and portfolio, and its actions
    /// are enqueued for execution.
    fn strategy(&self, c: &StrategyContext) -> Command;
}
