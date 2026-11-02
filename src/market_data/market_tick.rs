use crate::market_data::{market_quote::MarketQuote, timestamp::TimestampMillis};
use serde::{Deserialize, Serialize};

/// A single market tick with bid, ask, and last quotes.
///
/// The bid and ask are the best quotes at the tick's timestamp; `last` records
/// the most recent trade.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    pub timestamp_millis: TimestampMillis,
    pub bid: MarketQuote,
    pub ask: MarketQuote,
    pub last: MarketQuote,
}
