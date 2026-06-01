use crate::{
    preferences::Preferences, resolver_context::ResolverContext, resolvers::resolver::Resolver,
    signal::signals::Signals, signal_context::SignalContext,
};

pub trait Strategy: Send + Sync {
    fn preferences(&self) -> Preferences;
    fn create_signals(&self, c: &SignalContext) -> Signals;
    fn resolver(&self, c: &ResolverContext) -> Resolver;
}
