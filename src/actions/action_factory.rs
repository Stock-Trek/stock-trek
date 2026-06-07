use crate::{
    actions::{
        place_order_action::PlaceOrderAction,
        recoverable_action::{RecoverableAction, RecoveryPolicy},
    },
    order::order_request::OrderRequest,
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue},
};

pub struct ActionFactory;

impl ActionFactory {
    pub fn place_order(
        &self,
        exchange_id_value: ExchangeIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            PlaceOrderAction::new(exchange_id_value, order_request),
            recovery_policy,
        )
    }
    // TODO
    // pub fn cancel_order(&self, exchange_id_value: ExchangeIdValue, order_id: OrderId, recovery_policy: RecoveryPolicy) -> RecoverableAction {
    //   RecoverableAction::new(
    //     PlaceOrderAction::new(exchange_id_value, order_request),
    //     recovery_policy,
    //   )
    // }
}
