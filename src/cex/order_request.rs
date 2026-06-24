use crate::{
    cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, NumberValue},
};
use stock_trek_types::cex::{
    asset_id::AssetId, capability::CexCapability, order_request::OrderRequest,
};

impl Resolvable<OrderRequest<AssetId, f64>> for OrderRequest<AssetIdValue, NumberValue> {
    fn try_resolve(
        &self,
        context: &ResolvedContext,
    ) -> StockTrekResult<OrderRequest<AssetId, f64>> {
        match self {
            Self::Single(order) => Ok(OrderRequest::Single(order.try_resolve(context)?)),
        }
    }
}

impl<A, N> HasRequiredCapabilities for OrderRequest<A, N> {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        match self {
            Self::Single(order) => order.required_capabilities(),
        }
    }
}
