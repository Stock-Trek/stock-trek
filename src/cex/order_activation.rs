use crate::{
    error::result::StockTrekResult, resolvable::Resolvable, resolved_context::ResolvedContext,
    validation::Validation, value::value_types::NumberValue,
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
                activation_price: Validation::positive(
                    "Activation::PriceTriggered",
                    "activation price",
                    activation_price.number(context)?,
                )?,
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
                activation_price: Validation::positive(
                    "Activation::Trailing",
                    "activation price",
                    activation_price.number(context)?,
                )?,
                basis: *basis,
                callback_rate_bps: *callback_rate_bps,
                direction: *direction,
            }),
        }
    }
}
