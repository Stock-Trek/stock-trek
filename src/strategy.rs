use crate::{
    error::result::StockTrekResult, resolver_context::ResolverContext,
    resolvers::resolver::Resolver, scratch::scratch_pad::ScratchPad,
    strategy_context::StrategyContext,
};

pub trait Strategy: Send + Sync {
    fn market_calculations(&self, context: StrategyContext) -> StockTrekResult<ScratchPad>;
    fn action_resolver(&self, context: ResolverContext) -> StockTrekResult<Resolver>;
}
