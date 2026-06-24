use crate::{
    error::result::StockTrekResult, resolveable::Resolvable, resolved_context::ResolvedContext,
    values::value::NumberValue,
};
use stock_trek_types::cex::quantity::Quantity;

impl Resolvable<Quantity<f64>> for Quantity<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<Quantity<f64>> {
        match self {
            Quantity::OfBase(q) => Ok(Quantity::OfBase(q.number(context)?)),
            Quantity::OfQuote(q) => Ok(Quantity::OfQuote(q.number(context)?)),
        }
    }
}
