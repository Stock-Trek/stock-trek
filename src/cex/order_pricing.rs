use crate::{
    error::result::StockTrekResult, resolveable::Resolvable, resolved_context::ResolvedContext,
    values::value::NumberValue,
};
use stock_trek_types::cex::pricing::Pricing;

impl Resolvable<Pricing<f64>> for Pricing<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<Pricing<f64>> {
        match self {
            Self::Market => Ok(Pricing::Market),
            Self::Limit {
                price,
                time_in_force,
            } => Ok(Pricing::Limit {
                price: price.number(context)?,
                time_in_force: *time_in_force,
            }),
        }
    }
}
