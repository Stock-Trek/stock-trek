use crate::{error::stock_trek_error::StockTrekResult, resolved_context::ResolvedContext};

pub(crate) trait Resolvable<T> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<T>;
}
