use crate::{
    action::{
        action::Action,
        recoverable_action::{RecoverableAction, RecoveryPolicy},
    },
    value::value::{AccountIdValue, AssetIdValue, CexIdValue, NumberValue},
};
use stock_trek_types::cex::{order_request::OrderRequest, tag::Tag};

/// Factory for constructing actions.
#[derive(Debug, Clone, Default)]
pub struct ActionFactory;

impl ActionFactory {
    /// Places an order in a CEX account.
    ///
    /// `recovery_policy` determines what the runtime does if the exchange
    /// rejects the order or the account lacks sufficient balance.
    pub fn place_order(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            Action::PlaceOrder {
                cex_id_value,
                account_id_value,
                order_request: Box::new(order_request),
            },
            recovery_policy,
        )
    }
    /// Cancels every order across all CEX accounts.
    pub fn cancel_all_orders(&self, recovery_policy: RecoveryPolicy) -> RecoverableAction {
        RecoverableAction::new(Action::CancelAllOrders { tag: None }, recovery_policy)
    }

    /// Cancels the orders carrying `tag` across all CEX accounts.
    pub fn cancel_all_orders_with_tag(
        &self,
        tag: Tag,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(Action::CancelAllOrders { tag: Some(tag) }, recovery_policy)
    }
    /// Cancels every order on a CEX, regardless of account.
    pub fn cancel_all_orders_in_cex(
        &self,
        cex_id_value: CexIdValue,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            Action::CancelAllOrdersInCex {
                cex_id_value,
                tag: None,
            },
            recovery_policy,
        )
    }
    /// Cancels the orders carrying `tag` on a CEX, regardless of account.
    pub fn cancel_all_orders_in_cex_with_tag(
        &self,
        cex_id_value: CexIdValue,
        tag: Tag,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            Action::CancelAllOrdersInCex {
                cex_id_value,
                tag: Some(tag),
            },
            recovery_policy,
        )
    }
    /// Cancels every order in a specific CEX account.
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
                tag: None,
            },
            recovery_policy,
        )
    }
    /// Cancels the orders carrying `tag` in a specific CEX account.
    pub fn cancel_all_orders_in_cex_account_with_tag(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        tag: Tag,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            Action::CancelAllOrdersInCexAccount {
                cex_id_value,
                account_id_value,
                tag: Some(tag),
            },
            recovery_policy,
        )
    }
}
