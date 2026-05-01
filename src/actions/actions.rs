use crate::actions::{action::Action, order_request_action::OrderRequestAction};
use digdigdig3::{core::OrderRequest, ExchangeId};

pub struct Actions {}

impl Actions {
    pub fn order_request(exchange: ExchangeId, order_request: OrderRequest) -> Action {
        OrderRequestAction::new(exchange, order_request)
    }
}
