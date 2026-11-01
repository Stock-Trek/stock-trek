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
#[derive(Debug, Clone)]
pub struct StrategyContext {
    /// Factory for actions.
    pub actions: ActionFactory,
    /// Factory for numeric calculations.
    pub calculations: CalculationValuesFactory,
    /// Factory for commands.
    pub commands: CommandFactory,
    /// Factory for conditions.
    pub conditions: ConditionFactory,
    /// Factory for literal values.
    pub literals: LiteralValuesFactory,
    /// Factory for order requests.
    pub orders: OrderFactory,
    /// Factory for portfolio values.
    pub portfolio: PortfolioValuesFactory,
    /// Factory for signal values.
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
