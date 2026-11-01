use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, tag::Tag,
};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
/// An action with all signal and portfolio values resolved.
pub enum ResolvedAction {
    /// Cancels all orders across all CEX accounts.
    CancelAllOrders {
        /// The optional tag of the orders to cancel.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX.
    CancelAllOrdersInCex {
        /// The CEX whose orders should be cancelled.
        cex_id: CexId,
        /// The optional tag of the orders to cancel.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX account.
    CancelAllOrdersInCexAccount {
        /// The CEX containing the account.
        cex_id: CexId,
        /// The account whose orders should be cancelled.
        account_id: AccountId,
        /// The optional tag of the orders to cancel.
        tag: Option<Tag>,
    },
    /// Places an order in a CEX account.
    PlaceOrder {
        /// The CEX on which the order should be placed.
        cex_id: CexId,
        /// The account that should place the order.
        account_id: AccountId,
        /// The resolved order request to send.
        order_request: OrderRequest<AssetId, f64>,
    },
}
