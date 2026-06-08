use crate::{
    cex::order_time_in_force::OrderTimeInForce, error::result::StockTrekResult,
    resolveable::Resolvable, resolved_context::ResolvedContext, values::value::NumberValue,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
pub enum OrderPricing<N> {
    Market,
    Limit {
        price: N,
        time_in_force: OrderTimeInForce,
    },
}

impl Resolvable<OrderPricing<f64>> for OrderPricing<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<OrderPricing<f64>> {
        match self {
            Self::Market => Ok(OrderPricing::Market),
            Self::Limit {
                price,
                time_in_force,
            } => Ok(OrderPricing::Limit {
                price: price.number(context)?,
                time_in_force: *time_in_force,
            }),
        }
    }
}

impl Eq for OrderPricing<Decimal> {}
impl PartialEq for OrderPricing<Decimal> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            OrderPricing::Market => match other {
                OrderPricing::Market => true,
                OrderPricing::Limit { .. } => false,
            },
            OrderPricing::Limit {
                price,
                time_in_force,
            } => match other {
                OrderPricing::Market => false,
                OrderPricing::Limit {
                    price: o_price,
                    time_in_force: o_time_in_force,
                } => price == o_price && time_in_force == o_time_in_force,
            },
        }
    }
}
