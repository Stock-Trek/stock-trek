use crate::cex::{asset_id::AssetId, cex_id::CexId, order_request::OrderRequest};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Display, Serialize, Deserialize)]
pub enum ResolvedAction {
    PlaceOrder {
        cex_id: CexId,
        order_request: OrderRequest<AssetId, f64>,
    },
}
