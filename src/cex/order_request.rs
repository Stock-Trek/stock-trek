use crate::{
    cex::{capability::HasRequiredCapabilities, order_quantity::positive_quantity},
    error::result::StockTrekResult,
    resolvable::Resolvable,
    resolved_context::ResolvedContext,
    value::value::{AssetIdValue, NumberValue},
};
use stock_trek_types::cex::{
    asset_id::AssetId, capability::CexCapability, order_request::OrderRequest, quantity::Quantity,
};

impl Resolvable<OrderRequest<AssetId, f64>> for OrderRequest<AssetIdValue, NumberValue> {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<OrderRequest<AssetId, f64>> {
        match self {
            Self::Limit {
                base,
                quote,
                side,
                activation,
                limit_price,
                time_in_force,
                quantity,
                tag,
            } => {
                let order_request = OrderRequest::Limit {
                    base: base.asset_id(c)?,
                    quote: quote.asset_id(c)?,
                    side: *side,
                    activation: activation.try_resolve(c)?,
                    limit_price: positive_quantity(
                        "OrderRequest::Limit",
                        "limit price",
                        limit_price.number(c)?,
                    )?,
                    time_in_force: *time_in_force,
                    quantity: quantity.try_resolve(c)?,
                    tag: tag.clone(),
                };
                Ok(order_request)
            }
            Self::MarketBuy {
                base,
                quote,
                activation,
                quote_quantity,
                tag,
            } => {
                let order_request = OrderRequest::MarketBuy {
                    base: base.asset_id(c)?,
                    quote: quote.asset_id(c)?,
                    activation: activation.try_resolve(c)?,
                    quote_quantity: positive_quantity(
                        "OrderRequest::MarketBuy",
                        "quote quantity",
                        quote_quantity.number(c)?,
                    )?,
                    tag: tag.clone(),
                };
                Ok(order_request)
            }
            Self::MarketSell {
                base,
                quote,
                activation,
                base_quantity,
                tag,
            } => {
                let order_request = OrderRequest::MarketSell {
                    base: base.asset_id(c)?,
                    quote: quote.asset_id(c)?,
                    activation: activation.try_resolve(c)?,
                    base_quantity: positive_quantity(
                        "OrderRequest::MarketSell",
                        "base quantity",
                        base_quantity.number(c)?,
                    )?,
                    tag: tag.clone(),
                };
                Ok(order_request)
            }
        }
    }
}

impl<A, N> HasRequiredCapabilities for OrderRequest<A, N> {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        match self {
            Self::Limit {
                quantity: Quantity::OfQuote(..),
                ..
            } => {
                vec![CexCapability::QuoteQuantityOnLimitOrders]
            }
            _ => vec![],
        }
    }
}
