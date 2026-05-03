use crate::actions::{action::Action, order_request_action::OrderRequestAction};
use digdigdig3::{core::OrderRequest, ExchangeId};

pub struct ActionsFactory;

impl ActionsFactory {
    pub fn order_request(&self, exchange: ExchangeId, order_request: OrderRequest) -> Action {
        OrderRequestAction::new(exchange, order_request)
    }
}
