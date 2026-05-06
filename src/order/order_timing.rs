use crate::order::order_trigger_direction::OrderTriggerDirection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderTiming<N> {
    Immediate,
    Conditional {
        stop_price: N,
        trigger: OrderTriggerDirection,
    },
    Trailing {
        callback_rate: N,
        trigger: OrderTriggerDirection,
    },
}
