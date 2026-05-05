use crate::{
    actions::action::{Action, ActionTrait},
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    exchanges::exchange::Exchange,
};
use digdigdig3::{core::OrderRequest, ExchangeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderRequestAction {
    exchange: ExchangeId,
    order_request: OrderRequest,
}

impl OrderRequestAction {
    pub fn new(exchange: ExchangeId, order_request: OrderRequest) -> Action {
        Box::new(Self {
            exchange,
            order_request,
        })
    }
}

#[typetag::serde]
impl ActionTrait for OrderRequestAction {
    fn clone_box(&self) -> Box<dyn ActionTrait> {
        Box::new(self.clone())
    }
    fn complete(
        &self,
        bot_id: &str,
        exchanges: &HashMap<ExchangeId, Exchange>,
    ) -> StockTrekResult<()> {
        if let Some(exchange) = exchanges.get(&self.exchange) {
            let order = exchange.place_order(bot_id, &self.order_request)?;
            println!("{:?}", order);
            Ok(())
        } else {
            Err(StockTrekError::Value(ValueError::NotFound {
                name: "Exchange".to_string(),
                key: self.exchange.as_str().to_string(),
            }))
        }
    }
}
