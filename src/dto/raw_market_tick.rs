use crate::dto::raw_market_quote::RawMarketQuote;
use crate::prelude::TimestampMillis;

#[derive(Debug)]
pub struct RawMarketTick {
    pub timestamp_millis: TimestampMillis,
    pub bid: RawMarketQuote,
    pub ask: RawMarketQuote,
    pub last: RawMarketQuote,
}
