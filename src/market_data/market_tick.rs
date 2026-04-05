use crate::dto::raw_market_tick::RawMarketTick;
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

impl From<RawMarketTick> for MarketTick {
    fn from(value: RawMarketTick) -> Self {
        let RawMarketTick {
            timestamp_millis,
            bid,
            ask,
            last,
        } = value;
        MarketTick {
            timestamp_millis,
            bid: MarketQuote::from(bid),
            ask: MarketQuote::from(ask),
            last: MarketQuote::from(last),
        }
    }
}
