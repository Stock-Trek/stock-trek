use crate::{
    commands::command_factory::CommandFactory,
    order::order_factory::OrderFactory,
    predicates::predicates_factory::PredicatesFactory,
    values::values_factory::{
        CalculationValuesFactory, LiteralValuesFactory, PortfolioValuesFactory, SignalValuesFactory,
    },
};

pub struct StrategyContext {
    pub calculations: CalculationValuesFactory,
    pub literals: LiteralValuesFactory,
    pub orders: OrderFactory,
    pub portfolio: PortfolioValuesFactory,
    pub predicates: PredicatesFactory,
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
            predicates: PredicatesFactory,
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
