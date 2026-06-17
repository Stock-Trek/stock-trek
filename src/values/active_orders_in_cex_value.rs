use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{CexIdValue, NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ActiveOrdersInCexValue {
    cex_id_value: CexIdValue,
}

impl ActiveOrdersInCexValue {
    pub fn new(cex_id_value: CexIdValue) -> NumberValue {
        Box::new(Self { cex_id_value })
    }
}

#[typetag::serde]
impl NumberValueTrait for ActiveOrdersInCexValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let cex_id = self.cex_id_value.cex_id(c)?;
        Ok(c.portfolio.active_orders_in_cex(&cex_id))
    }
}
