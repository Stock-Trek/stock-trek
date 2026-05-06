use crate::order::order_time_in_force::OrderTimeInForce;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderPricing<P> {
    Market,
    Limit {
        price: P,
        time_in_force: OrderTimeInForce,
    },
}
