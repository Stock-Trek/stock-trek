use crate::exchanges::exchange::Exchange;
use anyhow::{bail, Result};
use digdigdig3::{core::OrderRequest, ExchangeId};
use std::collections::HashMap;

pub type Action = Box<dyn ActionTrait>;

#[typetag::serde]
pub trait ActionTrait: Send + Sync {
    fn clone_box(&self) -> Box<dyn ActionTrait>;
    fn complete(&mut self, exchanges: &HashMap<ExchangeId, Exchange>) -> Result<()>;
    fn try_place_order(
        &self,
        exchanges: &HashMap<ExchangeId, Exchange>,
        exchange_id: &ExchangeId,
        request: &OrderRequest,
    ) -> Result<()> {
        if let Some(exchange) = exchanges.get(exchange_id) {
            let order = exchange.place_order(request)?;
            println!("{:?}", order);
            Ok(())
        } else {
            bail!("Could not find Exchange {:?}", exchange_id)
        }
    }
}

impl Clone for Box<dyn ActionTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
