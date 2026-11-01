use crate::market_data::{market_quote::MarketQuote, timestamp::TimestampMillis};
use serde::{Deserialize, Serialize};

/// A single market tick with bid, ask, and last quotes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    /// The tick timestamp in milliseconds.
    pub timestamp_millis: TimestampMillis,
    /// The best bid quote.
    pub bid: MarketQuote,
    /// The best ask quote.
    pub ask: MarketQuote,
    /// The last traded price and quantity.
    pub last: MarketQuote,
}
