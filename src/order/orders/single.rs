use crate::{
    asset_id::AssetId,
    capability::{Capability, HasRequiredCapabilities, QuoteQuantityCapability},
    commands::resolveable::Resolvable,
    error::result::StockTrekResult,
    order::{
        order_activation::OrderActivation, order_constraint::OrderConstraint,
        order_intent::OrderIntent, order_pricing::OrderPricing, order_quantity::OrderQuantity,
        order_side::OrderSide,
    },
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
    pub intent: OrderIntent,
    pub side: OrderSide,
    pub quantity: OrderQuantity<N>,
    pub constraints: Vec<OrderConstraint>,
}

pub type SingleOrderRaw = SingleOrderGeneric<AssetIdValue, NumberValue>;
pub type SingleOrder = SingleOrderGeneric<AssetId, f64>;

impl Resolvable<SingleOrder> for SingleOrderRaw {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<SingleOrder> {
        Ok(SingleOrder {
            base: self.base.asset_id(context)?,
            quote: self.quote.asset_id(context)?,
            activation: self.activation.try_resolve(context)?,
            pricing: self.pricing.try_resolve(context)?,
            intent: self.intent,
            side: self.side,
            quantity: self.quantity.try_resolve(context)?,
            constraints: self.constraints.clone(),
        })
    }
}

impl<A, N> HasRequiredCapabilities for SingleOrderGeneric<A, N> {
    fn required_capabilities(&self) -> Vec<Capability> {
        let mut capabilities = Vec::new();
        if let OrderQuantity::OfQuote(_) = self.quantity {
            if let OrderPricing::Limit { .. } = self.pricing {
                capabilities.push(Capability::QuoteQuantity(
                    QuoteQuantityCapability::AllowLimitPricing,
                ));
            }
            match self.activation {
                OrderActivation::PriceTriggered { .. } | OrderActivation::Trailing { .. } => {
                    capabilities.push(Capability::QuoteQuantity(
                        QuoteQuantityCapability::AllowTriggeredTiming,
                    ));
                }
                _ => {}
            }
        }
        capabilities
    }
}
