use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, tag::Tag,
};
use strum::Display;

/// An action with all signal and portfolio values resolved.
///
/// Produced when an [`Action`](crate::actions::Action) is enqueued and handed to the
/// [`EnqueueActionFn`](crate::resolved_context::EnqueueActionFn) callback for
/// execution.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum ResolvedAction {
    /// Cancels all orders across all CEX accounts.
    CancelAllOrders {
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX.
    CancelAllOrdersInCex {
        /// The CEX whose orders are cancelled.
        cex_id: CexId,
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX account.
    CancelAllOrdersInCexAccount {
        /// The CEX the account is on.
        cex_id: CexId,
        /// The account whose orders are cancelled.
        account_id: AccountId,
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Places an order in a CEX account.
    PlaceOrder {
        /// The CEX to place the order on.
        cex_id: CexId,
        /// The account to place the order in.
        account_id: AccountId,
        /// The order to place, with quantities resolved to numbers.
        order_request: OrderRequest<AssetId, f64>,
    },
}
