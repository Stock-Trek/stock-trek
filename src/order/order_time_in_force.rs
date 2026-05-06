use crate::market_data::timestamp::TimestampMillis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderTimeInForce {
    // TODO
    // GoodTillCancelled,
    GoodTillTime(TimestampMillis),
    FillOrKill,
    ImmediateOrCancel,
    PostOnly,
}
