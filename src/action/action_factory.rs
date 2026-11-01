use crate::{
    action::{
        action::Action,
        recoverable_action::{RecoverableAction, RecoveryPolicy},
    },
    value::value::{AccountIdValue, AssetIdValue, CexIdValue, NumberValue},
};
use stock_trek_types::cex::order_request::OrderRequest;

/// Factory for constructing actions.
#[derive(Debug, Clone)]
pub struct ActionFactory;

impl ActionFactory {
    /// Creates an action that sends an order request using the given recovery policy.
    pub fn send_order_request(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            Action::SendOrderRequest {
                cex_id_value,
                account_id_value,
                order_request: Box::new(order_request),
            },
            recovery_policy,
        )
    }
    // TODO
    // pub fn cancel_order(&self, cex_id_value: CexIdValue, account_id_value: AccountIdValue, order_id: OrderId, recovery_policy: RecoveryPolicy) -> RecoverableAction {
    //   RecoverableAction::new(
    //     Action::CancelOrder { cex_id_value, account_id_value, order_id },
    //     recovery_policy,
    //   )
    // }
}
