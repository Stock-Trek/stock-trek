use crate::{
    preferences::Preferences, resolver_context::ResolverContext, resolvers::resolver::Resolver,
    scratch::scratch_pad::ScratchPad, strategy_context::StrategyContext,
};

pub trait Strategy: Send + Sync {
    fn preferences(&self) -> Preferences;
    fn calculate(&self, c: &StrategyContext) -> ScratchPad;
    fn resolver(&self, c: &ResolverContext) -> Resolver;
}
