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
        basis: OrderPriceBasis,
        direction: OrderTriggerDirection,
        mode: OrderTriggerMode,
        price: N,
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
                basis,
                direction,
                mode,
                price,
            } => Ok(OrderActivation::PriceTriggered {
                basis: *basis,
                direction: *direction,
                mode: *mode,
                price: price.number(context)?,
            }),
            Self::Trailing {
                activation_price,
                basis,
                callback_rate_bps: callback_rate,
                direction,
            } => Ok(OrderActivation::Trailing {
                activation_price: activation_price.number(context)?,
                basis: *basis,
                callback_rate_bps: callback_rate.number(context)?,
                direction: *direction,
            }),
        }
    }
}
