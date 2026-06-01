use crate::{
    order::order_factory::OrderFactory,
    predicates::predicates_factory::PredicatesFactory,
    resolvers::resolvers_factory::ResolversFactory,
    values::values_factory::{
        CalculationValuesFactory, LiteralValuesFactory, PortfolioValuesFactory, SignalValuesFactory,
    },
};

pub struct ResolverContext {
    pub calculations: CalculationValuesFactory,
    pub literals: LiteralValuesFactory,
    pub orders: OrderFactory,
    pub portfolio: PortfolioValuesFactory,
    pub predicates: PredicatesFactory,
    pub resolvers: ResolversFactory,
    pub signals: SignalValuesFactory,
}

impl ResolverContext {
    pub fn new() -> Self {
        Self {
            calculations: CalculationValuesFactory,
            literals: LiteralValuesFactory,
            orders: OrderFactory,
            portfolio: PortfolioValuesFactory,
            predicates: PredicatesFactory,
            resolvers: ResolversFactory,
            signals: SignalValuesFactory,
        }
    }
}

impl Default for ResolverContext {
    fn default() -> Self {
        Self::new()
    }
}
