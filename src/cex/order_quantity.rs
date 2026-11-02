use crate::{
    error::result::StockTrekResult, numeric::Quantity as ExactQuantity, resolvable::Resolvable,
    resolved_context::ResolvedContext, value::value::NumberValue,
};
use rust_decimal::Decimal;
use stock_trek_types::cex::quantity::Quantity;

impl Resolvable<Quantity<Decimal>> for Quantity<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<Quantity<Decimal>> {
        match self {
            Quantity::OfBase(q) => Ok(Quantity::OfBase(
                ExactQuantity::try_from_calc(q.number(context)?)?.to_decimal(),
            )),
            Quantity::OfQuote(q) => Ok(Quantity::OfQuote(
                ExactQuantity::try_from_calc(q.number(context)?)?.to_decimal(),
            )),
        }
    }
}
