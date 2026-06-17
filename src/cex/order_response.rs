use crate::cex::{order_id::OrderId, order_tag::OrderTag};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: OrderId,
    pub tag: OrderTag,
}
