use crate::market_data::market_quote::MarketQuote;
use crate::prelude::TimestampMillis;

#[derive(Debug)]
pub struct MarketTick {
    timestamp_millis: TimestampMillis,
    bid: MarketQuote,
    ask: MarketQuote,
    last: MarketQuote,
}

impl MarketTick {
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
    pub fn timestamp_millis(&self) -> TimestampMillis {
        self.timestamp_millis
    }
    pub fn bid(&self) -> &MarketQuote {
        &self.bid
    }
    pub fn ask(&self) -> &MarketQuote {
        &self.ask
    }
    pub fn last(&self) -> &MarketQuote {
        &self.last
    }
}
