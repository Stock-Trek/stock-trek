use crate::value::value_types::{AssetIdValue, NumberValue};
use stock_trek_types::cex::{
    activation::Activation, order_request::OrderRequest, quantity::Quantity, side::Side, tag::Tag,
    time_in_force::TimeInForce,
};

/// Parameters for a limit order.
///
/// Grouping the values into a named struct keeps [`OrderFactory::limit`]
/// readable and avoids a long list of positional arguments.
#[derive(Debug, Clone)]
pub struct LimitOrder {
    /// The base asset being bought or sold.
    pub base: AssetIdValue,
    /// The quote asset used to price the order.
    pub quote: AssetIdValue,
    /// Whether the order buys or sells the base asset.
    pub side: Side,
    /// When the order becomes active.
    pub activation: Activation<NumberValue>,
    /// The limit price.
    pub limit_price: NumberValue,
    /// How long the order remains active.
    pub time_in_force: TimeInForce,
    /// The quantity to trade.
    pub quantity: Quantity<NumberValue>,
    /// The tag used to group the order.
    pub tag: Tag,
}

/// Alias for [`LimitOrder`] documenting that these are request parameters.
pub type LimitOrderParams = LimitOrder;
/// Alias for [`LimitOrder`] documenting that these are request parameters.
pub type LimitOrderRequest = LimitOrder;

impl
    From<(
        AssetIdValue,
        AssetIdValue,
        Side,
        Activation<NumberValue>,
        NumberValue,
        TimeInForce,
        Quantity<NumberValue>,
        Tag,
    )> for LimitOrder
{
    fn from(
        (base, quote, side, activation, limit_price, time_in_force, quantity, tag): (
            AssetIdValue,
            AssetIdValue,
            Side,
            Activation<NumberValue>,
            NumberValue,
            TimeInForce,
            Quantity<NumberValue>,
            Tag,
        ),
    ) -> Self {
        Self {
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
}

/// Factory for constructing CEX order requests.
///
/// Available as the `orders` field of a [`StrategyContext`].
///
/// [`StrategyContext`]: crate::StrategyContext
#[derive(Debug, Clone, Default)]
pub struct OrderFactory;

impl OrderFactory {
    /// A limit order, activated as specified, for the given trading pair.
    pub fn limit(&self, order: impl Into<LimitOrder>) -> OrderRequest<AssetIdValue, NumberValue> {
        let LimitOrder {
            base,
            quote,
            side,
            activation,
            limit_price,
            time_in_force,
            quantity,
            tag,
        } = order.into();
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
