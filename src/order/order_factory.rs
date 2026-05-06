use crate::{
    order::{
        order_intent::OrderIntent, order_pricing::OrderPricing, order_quantity::OrderQuantity,
        order_request::OrderRequest, order_side::OrderSide, order_time_in_force::OrderTimeInForce,
        order_timing::OrderTiming, order_trigger_direction::OrderTriggerDirection,
        single_order::SingleOrder,
    },
    values::value::{NumberValue, TokenValue},
};

pub struct OrderFactory;

impl OrderFactory {
    pub fn market(
        &self,
        base: TokenValue,
        quote: TokenValue,
        intent: OrderIntent,
        side: OrderSide,
        quantity: OrderQuantity<NumberValue>,
    ) -> OrderRequest<TokenValue, NumberValue> {
        OrderRequest::Single(SingleOrder {
            base,
            quote,
            timing: OrderTiming::Immediate,
            pricing: OrderPricing::Market,
            intent,
            side,
            quantity,
        })
    }
    pub fn limit(
        &self,
        base: TokenValue,
        quote: TokenValue,
        intent: OrderIntent,
        side: OrderSide,
        time_in_force: OrderTimeInForce,
        price: NumberValue,
        quantity: OrderQuantity<NumberValue>,
    ) -> OrderRequest<TokenValue, NumberValue> {
        OrderRequest::Single(SingleOrder {
            base,
            quote,
            timing: OrderTiming::Immediate,
            pricing: OrderPricing::Limit {
                price,
                time_in_force,
            },
            intent,
            side,
            quantity,
        })
    }
    pub fn stop_market(
        &self,
        base: TokenValue,
        quote: TokenValue,
        intent: OrderIntent,
        side: OrderSide,
        stop_price: NumberValue,
        quantity: OrderQuantity<NumberValue>,
    ) -> OrderRequest<TokenValue, NumberValue> {
        OrderRequest::Single(SingleOrder {
            base,
            quote,
            timing: OrderTiming::Conditional {
                stop_price,
                trigger: OrderTriggerDirection::Below,
            },
            pricing: OrderPricing::Market,
            intent,
            side,
            quantity,
        })
    }
    pub fn stop_limit(
        &self,
        base: TokenValue,
        quote: TokenValue,
        intent: OrderIntent,
        side: OrderSide,
        time_in_force: OrderTimeInForce,
        stop_price: NumberValue,
        limit_price: NumberValue,
        quantity: OrderQuantity<NumberValue>,
    ) -> OrderRequest<TokenValue, NumberValue> {
        OrderRequest::Single(SingleOrder {
            base,
            quote,
            timing: OrderTiming::Conditional {
                stop_price,
                trigger: OrderTriggerDirection::Below,
            },
            pricing: OrderPricing::Limit {
                price: limit_price,
                time_in_force,
            },
            intent,
            side,
            quantity,
        })
    }
}
