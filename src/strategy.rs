use crate::{
    error::result::StockTrekResult, preferences::Preferences, resolver_context::ResolverContext,
    resolvers::resolver::Resolver, scratch::scratch_pad::ScratchPad,
    strategy_context::StrategyContext,
};

pub trait Strategy: Send + Sync {
    fn preferences(&self) -> StockTrekResult<Preferences>;
    fn calculate(&self, c: &StrategyContext) -> StockTrekResult<ScratchPad>;
    fn resolver(&self, c: &ResolverContext) -> StockTrekResult<Resolver>;
}
