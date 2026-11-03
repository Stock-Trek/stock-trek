use super::market_tick::*;

use crate::market_data::{market_quote::MarketQuote, timestamp::TimestampMillis};

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
