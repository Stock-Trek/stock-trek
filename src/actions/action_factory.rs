use crate::{
    actions::{
        recoverable_action::{RecoverableAction, RecoveryPolicy},
        send_order_request_action::SendOrderRequestAction,
    },
    cex::order_request::OrderRequest,
    values::value::{AssetIdValue, CexIdValue, NumberValue},
};

pub struct ActionFactory;

impl ActionFactory {
    pub fn send_order_request(
        &self,
        cex_id_value: CexIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            SendOrderRequestAction::new(cex_id_value, order_request),
            recovery_policy,
        )
    }
    // TODO
    // pub fn cancel_order(&self, cex_id_value: CexIdValue, order_id: OrderId, recovery_policy: RecoveryPolicy) -> RecoverableAction {
    //   RecoverableAction::new(
    //     PlaceOrderAction::new(cex_id_value, order_request),
    //     recovery_policy,
    //   )
    // }
}
