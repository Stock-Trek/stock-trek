use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, tag::Tag,
};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[allow(missing_docs)]
#[non_exhaustive]
/// An action with all signal and portfolio values resolved.
pub enum ResolvedAction {
    /// Cancels all orders across all CEX accounts.
    CancelAllOrders { tag: Option<Tag> },
    /// Cancels all orders in a CEX.
    CancelAllOrdersInCex { cex_id: CexId, tag: Option<Tag> },
    /// Cancels all orders in a CEX account.
    CancelAllOrdersInCexAccount {
        cex_id: CexId,
        account_id: AccountId,
        tag: Option<Tag>,
    },
    /// Places an order in a CEX account.
    PlaceOrder {
        cex_id: CexId,
        account_id: AccountId,
        order_request: OrderRequest<AssetId, f64>,
    },
}
