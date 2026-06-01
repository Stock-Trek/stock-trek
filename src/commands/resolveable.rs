use crate::{error::result::StockTrekResult, resolved_context::ResolvedContext};

pub trait Resolvable<T> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<T>;
}
