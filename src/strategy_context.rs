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
/// A strategy receives this context in [`crate::Algorithm::strategy`] and uses
/// it to construct values, conditions, orders, and actions. The factories are
/// stateless; market and portfolio data is read when the resulting command
/// executes, not when it is built.
#[derive(Debug, Clone)]
pub struct StrategyContext {
    /// Builds executable actions and their recovery policies.
    pub actions: ActionFactory,
    /// Builds arithmetic and unary calculations over [`NumberValue`](crate::values::NumberValue)s.
    pub calculations: CalculationValuesFactory,
    /// Builds commands such as conditionals, lists, and action plans.
    pub commands: CommandFactory,
    /// Builds conditions used to branch within a command.
    pub conditions: ConditionFactory,
    /// Builds literal values such as asset identifiers and numbers.
    pub literals: LiteralValuesFactory,
    /// Builds CEX order requests.
    pub orders: OrderFactory,
    /// Builds values derived from the portfolio, such as balances and pending-order counts.
    pub portfolio: PortfolioValuesFactory,
    /// Builds values read from signals produced by the algorithm.
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
