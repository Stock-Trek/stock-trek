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
    actions: ActionFactory,
    calculations: CalculationValuesFactory,
    commands: CommandFactory,
    conditions: ConditionFactory,
    literals: LiteralValuesFactory,
    orders: OrderFactory,
    portfolio: PortfolioValuesFactory,
    signals: SignalValuesFactory,
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

    /// Returns the factory for building actions.
    pub fn actions(&self) -> &ActionFactory {
        &self.actions
    }

    /// Returns the factory for building calculations.
    pub fn calculations(&self) -> &CalculationValuesFactory {
        &self.calculations
    }

    /// Returns the factory for building commands.
    pub fn commands(&self) -> &CommandFactory {
        &self.commands
    }

    /// Returns the factory for building conditions.
    pub fn conditions(&self) -> &ConditionFactory {
        &self.conditions
    }

    /// Returns the factory for building literal values.
    pub fn literals(&self) -> &LiteralValuesFactory {
        &self.literals
    }

    /// Returns the factory for building orders.
    pub fn orders(&self) -> &OrderFactory {
        &self.orders
    }

    /// Returns the factory for building portfolio values.
    pub fn portfolio(&self) -> &PortfolioValuesFactory {
        &self.portfolio
    }

    /// Returns the factory for building signal values.
    pub fn signals(&self) -> &SignalValuesFactory {
        &self.signals
    }
}

impl Default for StrategyContext {
    fn default() -> Self {
        Self::new()
    }
}
