use crate::{error::result::StockTrekResult, resolved_context::ResolvedContext};
use stock_trek_types::cex::cex_id::CexId;

pub(crate) trait Resolvable<T> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<T>;
}

/// Resolves a value that needs the market of a specific CEX, such as an order
/// request whose prices and quantities must be quantized to market increments.
pub(crate) trait ResolvableInCex<T> {
    fn try_resolve_in_cex(&self, context: &ResolvedContext, cex_id: &CexId) -> StockTrekResult<T>;
}
