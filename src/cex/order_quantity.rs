use crate::{
    error::result::StockTrekResult, resolvable::Resolvable, resolved_context::ResolvedContext,
    validation::Validation, value::value_types::NumberValue,
};
use stock_trek_types::cex::quantity::Quantity;

impl Resolvable<Quantity<f64>> for Quantity<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<Quantity<f64>> {
        match self {
            Quantity::OfBase(q) => Ok(Quantity::OfBase(Validation::positive(
                "Quantity::OfBase",
                "base quantity",
                q.number(context)?,
            )?)),
            Quantity::OfQuote(q) => Ok(Quantity::OfQuote(Validation::positive(
                "Quantity::OfQuote",
                "quote quantity",
                q.number(context)?,
            )?)),
        }
    }
}

#[cfg(test)]
#[path = "order_quantity_tests.rs"]
mod tests;
