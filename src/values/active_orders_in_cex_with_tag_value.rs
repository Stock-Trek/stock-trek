use crate::{
    cex::order_tag::OrderTag,
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{CexIdValue, NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ActiveOrdersInCexWithTagValue {
    cex_id_value: CexIdValue,
    order_tag: OrderTag,
}

impl ActiveOrdersInCexWithTagValue {
    pub fn new(cex_id_value: CexIdValue, order_tag: OrderTag) -> NumberValue {
        Box::new(Self {
            cex_id_value,
            order_tag,
        })
    }
}

#[typetag::serde]
impl NumberValueTrait for ActiveOrdersInCexWithTagValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let cex_id = self.cex_id_value.cex_id(c)?;
        Ok(c.portfolio
            .active_orders_in_cex_with_tag(&cex_id, &self.order_tag))
    }
}
