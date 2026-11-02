use crate::{
    action::action_factory::ActionFactory,
    cex::order_factory::OrderFactory,
    commands::CommandFactory,
    conditions::ConditionFactory,
    value::values_factory::{
        CalculationValuesFactory, LiteralValuesFactory, PortfolioValuesFactory, SignalValuesFactory,
    },
};

/// The factories available while building a strategy command.
///
/// Provided to [`Algorithm::strategy`](crate::Algorithm::strategy), which
/// runs exactly once. Every field is a factory for a category of building
/// block; the values they produce are deferred and resolved against fresh
/// portfolio and signal data each time the command tree is interpreted.
#[derive(Debug, Clone)]
pub struct StrategyContext {
    /// Enqueueable actions, such as placing or cancelling orders.
    pub actions: ActionFactory,
    /// Arithmetic over other values, such as multiplying price by quantity.
    pub calculations: CalculationValuesFactory,
    /// Command combinators, such as `if`/else and plans.
    pub commands: CommandFactory,
    /// Testable conditions, such as comparisons and portfolio checks.
    pub conditions: ConditionFactory,
    /// Values fixed at strategy-build time, such as asset identifiers.
    pub literals: LiteralValuesFactory,
    /// Order requests, such as limit and market orders.
    pub orders: OrderFactory,
    /// Values read from the portfolio, such as available balances.
    pub portfolio: PortfolioValuesFactory,
    /// Values read from the signals produced by
    /// [`Algorithm::signals`](crate::Algorithm::signals).
    pub signals: SignalValuesFactory,
}

impl StrategyContext {
    /// Creates a strategy context with default factories.
    pub fn new() -> Self {
        Self {
            actions: ActionFactory,
            calculations: CalculationValuesFactory,
            commands: CommandFactory,
            conditions: ConditionFactory,
            literals: LiteralValuesFactory,
            orders: OrderFactory,
            portfolio: PortfolioValuesFactory,
            signals: SignalValuesFactory,
        }
    }
}

impl Default for StrategyContext {
    fn default() -> Self {
        Self::new()
    }
}
