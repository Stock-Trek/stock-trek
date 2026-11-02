use crate::value::number_value::{AssetIdValue, NumberValue};
use stock_trek_types::cex::{
    activation::Activation, order_request::OrderRequest, quantity::Quantity, side::Side, tag::Tag,
    time_in_force::TimeInForce,
};

/// Factory for constructing CEX order requests.
///
/// Available as the `orders` field of a [`StrategyContext`].
///
/// [`StrategyContext`]: crate::StrategyContext
#[derive(Debug, Clone, Default)]
pub struct OrderFactory;

impl OrderFactory {
    /// A limit order, activated as specified, for the given trading pair.
    #[allow(clippy::too_many_arguments)]
    pub fn limit(
        &self,
        base: AssetIdValue,
        quote: AssetIdValue,
        side: Side,
        activation: Activation<NumberValue>,
        limit_price: NumberValue,
        time_in_force: TimeInForce,
        quantity: Quantity<NumberValue>,
        tag: Tag,
    ) -> OrderRequest<AssetIdValue, NumberValue> {
        OrderRequest::Limit {
            base,
            quote,
            side,
            activation,
            limit_price,
            time_in_force,
            quantity,
            tag,
        }
    }
    /// A market order that buys with the given quote-currency quantity.
    pub fn market_buy(
        &self,
        base: AssetIdValue,
        quote: AssetIdValue,
        activation: Activation<NumberValue>,
        quote_quantity: NumberValue,
        tag: Tag,
    ) -> OrderRequest<AssetIdValue, NumberValue> {
        OrderRequest::MarketBuy {
            base,
            quote,
            activation,
            quote_quantity,
            tag,
        }
    }
    /// A market order that sells the given base-asset quantity.
    pub fn market_sell(
        &self,
        base: AssetIdValue,
        quote: AssetIdValue,
        activation: Activation<NumberValue>,
        base_quantity: NumberValue,
        tag: Tag,
    ) -> OrderRequest<AssetIdValue, NumberValue> {
        OrderRequest::MarketSell {
            base,
            quote,
            activation,
            base_quantity,
            tag,
        }
    }
}
