use crate::cex::{asset_id::AssetId, exchange_id::ExchangeId, order_request::OrderRequest};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Display, Serialize, Deserialize)]
pub enum ResolvedAction {
    PlaceOrder {
        exchange_id: ExchangeId,
        order_request: OrderRequest<AssetId, f64>,
    },
}
