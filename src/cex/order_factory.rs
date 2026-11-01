use crate::value::value::{AssetIdValue, NumberValue};
use stock_trek_types::cex::{
    activation::Activation, order_request::OrderRequest, quantity::Quantity, side::Side, tag::Tag,
    time_in_force::TimeInForce,
};

/// Factory for constructing CEX order requests.
#[derive(Debug, Clone)]
pub struct OrderFactory;

impl OrderFactory {
    /// Creates a limit order request.
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
    /// Creates a market buy order request using a quote-asset quantity.
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
    /// Creates a market sell order request using a base-asset quantity.
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
