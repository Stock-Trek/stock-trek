use crate::market_data::market_quote::MarketQuote;
use crate::prelude::TimestampMillis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketTick {
    pub timestamp_millis: TimestampMillis,
    pub bid: MarketQuote,
    pub ask: MarketQuote,
    pub last: MarketQuote,
}
