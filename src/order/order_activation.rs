use crate::{
    error::result::StockTrekResult,
    order::{
        order_price_basis::OrderPriceBasis, order_trigger_direction::OrderTriggerDirection,
        order_trigger_mode::OrderTriggerMode,
    },
    resolved_context::ResolvedContext,
    resolvers::resolveable::Resolvable,
    values::value::NumberValue,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
pub enum OrderActivation<N> {
    Immediate,
    PriceTriggered {
        activation_price: N,
        basis: OrderPriceBasis,
        direction: OrderTriggerDirection,
        mode: OrderTriggerMode,
    },
    Trailing {
        activation_price: N,
        basis: OrderPriceBasis,
        callback_rate_bps: N,
        direction: OrderTriggerDirection,
    },
}

impl Resolvable<OrderActivation<f64>> for OrderActivation<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<OrderActivation<f64>> {
        match self {
            Self::Immediate => Ok(OrderActivation::Immediate),
            Self::PriceTriggered {
                activation_price,
                basis,
                direction,
                mode,
            } => Ok(OrderActivation::PriceTriggered {
                activation_price: activation_price.number(context)?,
                basis: *basis,
                direction: *direction,
                mode: *mode,
            }),
            Self::Trailing {
                activation_price,
                basis,
                callback_rate_bps,
                direction,
            } => Ok(OrderActivation::Trailing {
                activation_price: activation_price.number(context)?,
                basis: *basis,
                callback_rate_bps: callback_rate_bps.number(context)?,
                direction: *direction,
            }),
        }
    }
}
