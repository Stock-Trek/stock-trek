use crate::market_data::{market_quote::MarketQuote, timestamp::TimestampMillis};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    pub timestamp_millis: TimestampMillis,
    pub bid: MarketQuote,
    pub ask: MarketQuote,
    pub last: MarketQuote,
}
