use crate::{
    cex::order_tag::OrderTag,
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ActiveOrdersWithTagValue {
    order_tag: OrderTag,
}

impl ActiveOrdersWithTagValue {
    pub fn new(order_tag: OrderTag) -> NumberValue {
        Box::new(Self { order_tag })
    }
}

#[typetag::serde]
impl NumberValueTrait for ActiveOrdersWithTagValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        Ok(c.portfolio.active_orders_with_tag(&self.order_tag))
    }
}
