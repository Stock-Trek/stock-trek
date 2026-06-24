use crate::values::value::{AssetIdValue, NumberValue};
use stock_trek_types::cex::{
    activation::Activation, order_request::OrderRequest, orders::single_order::SingleOrder,
    pricing::Pricing, quantity::Quantity, side::Side, tag::Tag,
};

pub struct OrderFactory;

impl OrderFactory {
    pub fn single(
        &self,
        base: AssetIdValue,
        quote: AssetIdValue,
        side: Side,
        activation: Activation<NumberValue>,
        pricing: Pricing<NumberValue>,
        quantity: Quantity<NumberValue>,
        tag: Tag,
    ) -> OrderRequest<AssetIdValue, NumberValue> {
        OrderRequest::Single(SingleOrder {
            base,
            quote,
            side,
            activation,
            pricing,
            quantity,
            tag,
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
