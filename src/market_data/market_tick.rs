use crate::market_data::{market_quote::MarketQuote, timestamp::TimestampMillis};
use serde::{Deserialize, Serialize};

/// A single market tick with bid, ask, and last quotes.
///
/// The bid and ask are the best quotes at the tick's timestamp; `last` records
/// the most recent trade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    timestamp_millis: TimestampMillis,
    bid: MarketQuote,
    ask: MarketQuote,
    last: MarketQuote,
}

impl MarketTick {
    /// Creates a tick from its timestamp and bid, ask, and last quotes.
    pub fn new(
        timestamp_millis: TimestampMillis,
        bid: MarketQuote,
        ask: MarketQuote,
        last: MarketQuote,
    ) -> Self {
        Self {
            timestamp_millis,
            bid,
            ask,
            last,
        }
    }

    /// Returns the tick timestamp in milliseconds.
    pub fn timestamp_millis(&self) -> TimestampMillis {
        self.timestamp_millis
    }

    /// Returns the best bid quote.
    pub fn bid(&self) -> &MarketQuote {
        &self.bid
    }

    /// Returns the best ask quote.
    pub fn ask(&self) -> &MarketQuote {
        &self.ask
    }

    /// Returns the last traded price and quantity.
    pub fn last(&self) -> &MarketQuote {
        &self.last
    }
}
