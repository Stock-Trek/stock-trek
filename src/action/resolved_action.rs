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
///
/// This is the form the runtime executes: CEX and account identifiers are
/// concrete, asset identifiers are known, and order values are plain numbers.
pub enum ResolvedAction {
    /// Cancels all orders across all CEX accounts.
    CancelAllOrders {
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX.
    CancelAllOrdersInCex {
        /// The CEX whose orders should be cancelled.
        cex_id: CexId,
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX account.
    CancelAllOrdersInCexAccount {
        /// The CEX whose account should have its orders cancelled.
        cex_id: CexId,
        /// The account whose orders should be cancelled.
        account_id: AccountId,
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Places an order in a CEX account.
    PlaceOrder {
        /// The CEX on which the order should be placed.
        cex_id: CexId,
        /// The account within the CEX that should place the order.
        account_id: AccountId,
        /// The fully resolved order request.
        order_request: OrderRequest<AssetId, f64>,
    },
}
