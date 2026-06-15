use crate::{
    cex::{
        order_activation::OrderActivation, order_constraint::OrderConstraint,
        order_intent::OrderIntent, order_pricing::OrderPricing, order_quantity::OrderQuantity,
        order_request::OrderRequest, order_side::OrderSide, orders::single::SingleOrderGeneric,
    },
    values::value::{AssetIdValue, NumberValue},
};

pub struct OrderFactory;

impl OrderFactory {
    pub fn single(
        &self,
        base: AssetIdValue,
        quote: AssetIdValue,
        intent: OrderIntent,
        side: OrderSide,
        timing: OrderActivation<NumberValue>,
        pricing: OrderPricing<NumberValue>,
        quantity: OrderQuantity<NumberValue>,
        constraints: Vec<OrderConstraint>,
    ) -> OrderRequest<AssetIdValue, NumberValue> {
        OrderRequest::Single(SingleOrderGeneric {
            base,
            quote,
            intent,
            side,
            activation: timing,
            pricing,
            quantity,
            constraints,
        })
    }
    // pub fn one_cancels_other(
    //     &self,
    //     primary: SingleOrderRaw,
    //     secondary: SingleOrderRaw,
    // ) -> OrderRequest<AssetIdValue, NumberValue> {
    //     OrderRequest::OneCancelsOther(OneCancelsOtherOrderGeneric { primary, secondary })
    // }
    // pub fn one_triggers_other(
    //     &self,
    //     primary: SingleOrderRaw,
    //     secondary: SingleOrderRaw,
    // ) -> OrderRequest<AssetIdValue, NumberValue> {
    //     OrderRequest::OneTriggersOther(OneTriggersOtherOrderGeneric { primary, secondary })
    // }
    // pub fn one_triggers_oco(
    //     &self,
    //     primary: SingleOrderRaw,
    //     oco_order: OneCancelsOtherOrderRaw,
    // ) -> OrderRequest<AssetIdValue, NumberValue> {
    //     OrderRequest::OneTriggersOco(OneTriggersOcoOrderGeneric { primary, oco_order })
    // }
}
