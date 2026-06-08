use crate::{
    error::result::StockTrekResult, resolveable::Resolvable, resolved_context::ResolvedContext,
    values::value::NumberValue,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
pub enum OrderQuantity<N> {
    OfBase(N),
    OfQuote(N),
}

impl Resolvable<OrderQuantity<f64>> for OrderQuantity<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<OrderQuantity<f64>> {
        match self {
            OrderQuantity::OfBase(q) => Ok(OrderQuantity::OfBase(q.number(context)?)),
            OrderQuantity::OfQuote(q) => Ok(OrderQuantity::OfQuote(q.number(context)?)),
        }
    }
}
