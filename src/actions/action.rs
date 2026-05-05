use crate::{error::result::StockTrekResult, exchanges::exchange::Exchange};
use digdigdig3::ExchangeId;
use std::collections::HashMap;

pub type Action = Box<dyn ActionTrait>;

#[typetag::serde]
pub trait ActionTrait: Send + Sync {
    fn clone_box(&self) -> Box<dyn ActionTrait>;
    fn complete(
        &self,
        bot_id: &str,
        exchanges: &HashMap<ExchangeId, Exchange>,
    ) -> StockTrekResult<()>;
}

impl Clone for Box<dyn ActionTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
