use crate::{
    error::result::StockTrekResult, resolver_context::ResolverContext,
    resolvers::resolver::Resolver, scratch::scratch_pad::ScratchPad,
    strategy_context::StrategyContext,
};

pub trait Strategy: Send + Sync {
    fn market_calculations(&self, c: &StrategyContext) -> StockTrekResult<ScratchPad>;
    fn resolver(&self, c: &ResolverContext) -> StockTrekResult<Resolver>;
}
