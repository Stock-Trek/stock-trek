use crate::{
    capability::HasRequiredCapabilities, error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};

pub type Command = Box<dyn CommandTrait>;

#[typetag::serde]
pub trait CommandTrait: HasRequiredCapabilities + Send + Sync {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()>;
}
