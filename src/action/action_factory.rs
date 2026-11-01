use crate::{
    action::{
        action::Action,
        recoverable_action::{RecoverableAction, RecoveryPolicy},
    },
    value::value::{AccountIdValue, AssetIdValue, CexIdValue, NumberValue},
};
use stock_trek_types::cex::{order_request::OrderRequest, tag::Tag};

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
    /// Creates an action that cancels all orders across all CEX accounts.
    pub fn cancel_all_orders(&self, recovery_policy: RecoveryPolicy) -> RecoverableAction {
        RecoverableAction::new(Action::CancelAllOrders, recovery_policy)
    }
    /// Creates an action that cancels all orders with `tag` across all CEX accounts.
    pub fn cancel_all_orders_with_tag(
        &self,
        tag: Tag,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(Action::CancelAllOrdersWithTag { tag }, recovery_policy)
    }
    /// Creates an action that cancels all orders in a CEX account.
    pub fn cancel_all_orders_in_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            Action::CancelAllOrdersInCexAccount {
                cex_id_value,
                account_id_value,
            },
            recovery_policy,
        )
    }
    /// Creates an action that cancels all orders with `tag` in a CEX account.
    pub fn cancel_all_orders_in_cex_account_with_tag(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        tag: Tag,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            Action::CancelAllOrdersInCexAccountWithTag {
                cex_id_value,
                account_id_value,
                tag,
            },
            recovery_policy,
        )
    }
}
