use crate::order::{
    order_intent::OrderIntent, order_pricing::OrderPricing, order_quantity::OrderQuantity,
    order_side::OrderSide, order_timing::OrderTiming,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SingleOrder<T, N> {
    pub base: T,
    pub quote: T,
    pub timing: OrderTiming<N>,
    pub pricing: OrderPricing<N>,
    pub intent: OrderIntent,
    pub side: OrderSide,
    pub quantity: OrderQuantity<N>,
}
