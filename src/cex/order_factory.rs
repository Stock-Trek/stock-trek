use crate::value::value::{AssetIdValue, NumberValue};
use stock_trek_types::cex::{
    activation::Activation, order_request::OrderRequest, quantity::Quantity, side::Side, tag::Tag,
    time_in_force::TimeInForce,
};

/// Factory for constructing CEX order requests.
#[derive(Debug, Clone, Default)]
pub struct OrderFactory;

impl OrderFactory {
    /// Builds a limit order.
    ///
    /// `quantity` may be denominated in the base or quote asset. `activation`
    /// controls whether the order is placed immediately or when a trigger price
    /// is reached. `tag` identifies the order for later cancellation and
    /// pending-order queries.
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
    /// Builds a market buy order that spends `quote_quantity` of the quote
    /// asset. The filled base quantity is determined by the exchange.
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
    /// Builds a market sell order for `base_quantity` of the base asset. The
    /// received quote quantity is determined by the exchange.
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
