use crate::{
    actions::place_order_action::StaleOutMillis, asset_id::AssetId, exchange_id::ExchangeId,
    order::order_request::OrderRequest,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Display, Serialize, Deserialize)]
pub enum ResolvedAction {
    PlaceOrder {
        exchange_id: ExchangeId,
        order_request: OrderRequest<AssetId, f64>,
        stale_out_millis: StaleOutMillis,
    },
}
