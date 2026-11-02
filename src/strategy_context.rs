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
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct StrategyContext {
    pub actions: ActionFactory,
    pub calculations: CalculationValuesFactory,
    pub commands: CommandFactory,
    pub conditions: ConditionFactory,
    pub literals: LiteralValuesFactory,
    pub orders: OrderFactory,
    pub portfolio: PortfolioValuesFactory,
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
