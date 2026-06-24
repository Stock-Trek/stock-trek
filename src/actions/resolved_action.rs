use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{
    asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, tag::Tag,
};
use strum::Display;

#[derive(Display, Serialize, Deserialize)]
pub enum ResolvedAction {
    CancelAllOrders,
    CancelAllOrdersWithTag {
        tag: Tag,
    },
    CancelAllOrdersInCex {
        cex_id: CexId,
    },
    CancelAllOrdersInCexWithTag {
        cex_id: CexId,
        tag: Tag,
    },
    PlaceOrder {
        cex_id: CexId,
        order_request: OrderRequest<AssetId, f64>,
    },
}
