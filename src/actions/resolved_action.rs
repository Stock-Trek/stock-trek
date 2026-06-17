use crate::cex::{
    asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, order_tag::OrderTag,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Display, Serialize, Deserialize)]
pub enum ResolvedAction {
    CancelAllOrders,
    CancelAllOrdersWithTag {
        order_tag: OrderTag,
    },
    CancelAllOrdersInCex {
        cex_id: CexId,
    },
    CancelAllOrdersInCexWithTag {
        cex_id: CexId,
        order_tag: OrderTag,
    },
    PlaceOrder {
        cex_id: CexId,
        order_request: OrderRequest<AssetId, f64>,
    },
}
