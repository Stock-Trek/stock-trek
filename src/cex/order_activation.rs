use crate::{
    error::result::StockTrekResult, resolveable::Resolvable, resolved_context::ResolvedContext,
    values::value::NumberValue,
};
use stock_trek_types::cex::activation::Activation;

impl Resolvable<Activation<f64>> for Activation<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<Activation<f64>> {
        match self {
            Self::Immediate => Ok(Activation::Immediate),
            Self::PriceTriggered {
                activation_price,
                basis,
                direction,
                mode,
            } => Ok(Activation::PriceTriggered {
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
            } => Ok(Activation::Trailing {
                activation_price: activation_price.number(context)?,
                basis: *basis,
                callback_rate_bps: callback_rate_bps.number(context)?,
                direction: *direction,
            }),
        }
    }
}
