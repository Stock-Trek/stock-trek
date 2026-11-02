use crate::{
    error::result::StockTrekResult, numeric::Price, resolvable::Resolvable,
    resolved_context::ResolvedContext, value::value::NumberValue,
};
use rust_decimal::Decimal;
use stock_trek_types::cex::activation::Activation;

impl Resolvable<Activation<Decimal>> for Activation<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<Activation<Decimal>> {
        match self {
            Self::Immediate => Ok(Activation::Immediate),
            Self::PriceTriggered {
                activation_price,
                basis,
                direction,
                mode,
            } => Ok(Activation::PriceTriggered {
                activation_price: Price::try_from_calc(activation_price.number(context)?)?
                    .to_decimal(),
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
                activation_price: Price::try_from_calc(activation_price.number(context)?)?
                    .to_decimal(),
                basis: *basis,
                callback_rate_bps: *callback_rate_bps,
                direction: *direction,
            }),
        }
    }
}
