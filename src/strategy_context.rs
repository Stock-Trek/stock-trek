use crate::{
    commands::command_factory::CommandFactory,
    conditions::condition_factory::ConditionFactory,
    order::order_factory::OrderFactory,
    values::values_factory::{
        CalculationValuesFactory, LiteralValuesFactory, PortfolioValuesFactory, SignalValuesFactory,
    },
};

pub struct StrategyContext {
    pub calculations: CalculationValuesFactory,
    pub literals: LiteralValuesFactory,
    pub orders: OrderFactory,
    pub portfolio: PortfolioValuesFactory,
    pub conditions: ConditionFactory,
    pub commands: CommandFactory,
    pub signals: SignalValuesFactory,
}

impl StrategyContext {
    pub fn new() -> Self {
        Self {
            calculations: CalculationValuesFactory,
            literals: LiteralValuesFactory,
            orders: OrderFactory,
            portfolio: PortfolioValuesFactory,
            conditions: ConditionFactory,
            commands: CommandFactory,
            signals: SignalValuesFactory,
        }
    }
}

impl Default for StrategyContext {
    fn default() -> Self {
        Self::new()
    }
}
