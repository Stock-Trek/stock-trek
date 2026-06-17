use crate::cex::order_tag::OrderTag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub tag: OrderTag,
}
