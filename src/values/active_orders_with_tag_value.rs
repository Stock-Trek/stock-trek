use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::tag::Tag;

#[derive(Clone, Serialize, Deserialize)]
pub struct ActiveOrdersWithTagValue {
    tag: Tag,
}

impl ActiveOrdersWithTagValue {
    pub fn new(tag: Tag) -> NumberValue {
        Box::new(Self { tag })
    }
}

#[typetag::serde]
impl NumberValueTrait for ActiveOrdersWithTagValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        Ok(c.portfolio.active_orders_with_tag(&self.tag))
    }
}
