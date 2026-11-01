use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest,
};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
/// An action with all signal and portfolio values resolved.
pub enum ResolvedAction {
    // TODO
    // CancelAllOrders,
    // CancelAllOrdersWithTag {
    //     tag: Tag,
    // },
    // CancelAllOrdersInCexAccount {
    //     cex_id: CexId,
    //     account_id: AccountId,
    // },
    // CancelAllOrdersInCexAccountWithTag {
    //     cex_id: CexId,
    //     account_id: AccountId,
    //     tag: Tag,
    // },
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
