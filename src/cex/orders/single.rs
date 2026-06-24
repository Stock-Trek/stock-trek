use stock_trek_types::cex::{
    asset_id::AssetId, capability::CexCapability, orders::single_order::SingleOrder,
    pricing::Pricing, quantity::Quantity,
};

use crate::{
    cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, NumberValue},
};

impl Resolvable<SingleOrder<AssetId, f64>> for SingleOrder<AssetIdValue, NumberValue> {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<SingleOrder<AssetId, f64>> {
        Ok(SingleOrder {
            base: self.base.asset_id(c)?,
            quote: self.quote.asset_id(c)?,
            activation: self.activation.try_resolve(c)?,
            pricing: self.pricing.try_resolve(c)?,
            side: self.side,
            quantity: self.quantity.try_resolve(c)?,
            tag: self.tag.clone(),
        })
    }
}

impl<A, N> HasRequiredCapabilities for SingleOrder<A, N> {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        let mut capabilities = Vec::new();
        if let Quantity::OfQuote(_) = self.quantity
            && let Pricing::Limit { .. } = self.pricing
        {
            capabilities.push(CexCapability::QuoteQuantityOnLimitOrders);
        }
        capabilities
    }
}
