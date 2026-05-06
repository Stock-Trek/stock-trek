use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: String,
    pub client_order_ids: Vec<String>,
}
