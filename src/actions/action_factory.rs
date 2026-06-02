use crate::{
    actions::{
        place_order_action::{PlaceOrderAction, StaleOutMillis},
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
        stale_out_millis: StaleOutMillis,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            PlaceOrderAction::new(exchange_id_value, order_request, stale_out_millis),
            recovery_policy,
        )
    }
    // TODO
    // pub fn cancel_order(&self, exchange_id_value: ExchangeIdValue, order_id: OrderId, recovery_policy: RecoveryPolicy) -> RecoverableAction {
    //   RecoverableAction::new(
    //     PlaceOrderAction::new(exchange_id_value, order_request, stale_out_millis),
    //     recovery_policy,
    //   )
    // }
}
