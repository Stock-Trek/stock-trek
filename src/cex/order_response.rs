use crate::cex::order_id::OrderId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: OrderId,
    // TODO
    // pub client_order_id: ClientOrderId,
}
