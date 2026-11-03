use crate::{
    error::result::StockTrekResult,
    market_data::{market_quote::MarketQuote, timestamp::TimestampMillis},
    validation::Validation,
};
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
    ///
    /// Returns an error unless the bid price is below the ask price.
    pub fn new(
        timestamp_millis: TimestampMillis,
        bid: MarketQuote,
        ask: MarketQuote,
        last: MarketQuote,
    ) -> StockTrekResult<Self> {
        Validation::bid_below_ask("MarketTick::new", bid.price(), ask.price())?;
        Ok(Self {
            timestamp_millis,
            bid,
            ask,
            last,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_tick_with_a_bid_below_the_ask() {
        let tick = MarketTick::new(
            TimestampMillis::new(1),
            MarketQuote::new(100.0, 2.0).unwrap(),
            MarketQuote::new(101.0, 3.0).unwrap(),
            MarketQuote::new(100.5, 4.0).unwrap(),
        )
        .unwrap();

        assert_eq!(tick.timestamp_millis(), TimestampMillis::new(1));
        assert_eq!(tick.bid().price(), 100.0);
        assert_eq!(tick.ask().price(), 101.0);
        assert_eq!(tick.last().price(), 100.5);
    }

    #[test]
    fn rejects_ticks_with_a_crossed_or_equal_bid_and_ask() {
        for (bid_price, ask_price) in [(101.0, 100.0), (100.0, 100.0)] {
            assert!(
                MarketTick::new(
                    TimestampMillis::new(1),
                    MarketQuote::new(bid_price, 2.0).unwrap(),
                    MarketQuote::new(ask_price, 3.0).unwrap(),
                    MarketQuote::new(100.5, 4.0).unwrap(),
                )
                .is_err()
            );
        }
    }
}
