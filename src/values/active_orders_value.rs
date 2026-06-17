use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ActiveOrdersValue {}

impl ActiveOrdersValue {
    pub fn new() -> NumberValue {
        Box::new(Self {})
    }
}

#[typetag::serde]
impl NumberValueTrait for ActiveOrdersValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        Ok(c.portfolio.active_orders())
    }
}
