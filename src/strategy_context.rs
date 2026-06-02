use crate::{
    actions::action_factory::ActionFactory,
    commands::command_factory::CommandFactory,
    conditions::condition_factory::ConditionFactory,
    order::order_factory::OrderFactory,
    values::values_factory::{
        CalculationValuesFactory, LiteralValuesFactory, PortfolioValuesFactory, SignalValuesFactory,
    },
};

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
