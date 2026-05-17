use crate::{
    capability::HasRequiredCapabilities, error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};

pub type Resolver = Box<dyn ResolverTrait>;

#[typetag::serde]
pub trait ResolverTrait: HasRequiredCapabilities + Send + Sync {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()>;
}
