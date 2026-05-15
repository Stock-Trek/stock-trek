use crate::market_data::timestamp::TimestampMillis;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderTimeInForce {
    // TODO
    // GoodTillCancelled,
    GoodTillTime(TimestampMillis),
    FillOrKill,
    ImmediateOrCancel,
    PostOnly,
}
