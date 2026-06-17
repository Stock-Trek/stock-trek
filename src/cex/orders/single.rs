use crate::{
    cex::{
        asset_id::AssetId,
        capability::{CexCapability, HasRequiredCapabilities, QuoteQuantityCexCapability},
        order_activation::OrderActivation,
        order_constraint::OrderConstraint,
        order_pricing::OrderPricing,
        order_quantity::OrderQuantity,
        order_side::OrderSide,
        order_tag::OrderTag,
    },
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SingleOrderGeneric<A, N> {
    pub base: A,
    pub quote: A,
    pub activation: OrderActivation<N>,
    pub pricing: OrderPricing<N>,
    pub side: OrderSide,
    pub quantity: OrderQuantity<N>,
    pub constraints: Vec<OrderConstraint>,
    pub order_tag: OrderTag,
}

pub type SingleOrderRaw = SingleOrderGeneric<AssetIdValue, NumberValue>;
pub type SingleOrder = SingleOrderGeneric<AssetId, f64>;

impl Resolvable<SingleOrder> for SingleOrderRaw {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<SingleOrder> {
        Ok(SingleOrder {
            base: self.base.asset_id(c)?,
            quote: self.quote.asset_id(c)?,
            activation: self.activation.try_resolve(c)?,
            pricing: self.pricing.try_resolve(c)?,
            side: self.side,
            quantity: self.quantity.try_resolve(c)?,
            constraints: self.constraints.clone(),
            order_tag: self.order_tag.clone(),
        })
    }
}

impl<A, N> HasRequiredCapabilities for SingleOrderGeneric<A, N> {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        let mut capabilities = Vec::new();
        if let OrderQuantity::OfQuote(_) = self.quantity {
            if let OrderPricing::Limit { .. } = self.pricing {
                capabilities.push(CexCapability::QuoteQuantity(
                    QuoteQuantityCexCapability::AllowLimitPricing,
                ));
            }
            match self.activation {
                OrderActivation::PriceTriggered { .. } | OrderActivation::Trailing { .. } => {
                    capabilities.push(CexCapability::QuoteQuantity(
                        QuoteQuantityCexCapability::AllowTriggeredTiming,
                    ));
                }
                _ => {}
            }
        }
        capabilities
    }
}
