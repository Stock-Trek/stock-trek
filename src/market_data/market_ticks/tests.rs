use super::*;
use crate::market_data::timestamp::TimestampMillis;

fn tick(price: f64) -> MarketTick {
    tick_with_timestamp(1, price)
}

fn tick_with_timestamp(timestamp_millis: u64, price: f64) -> MarketTick {
    MarketTick::new(
        TimestampMillis::new(timestamp_millis),
        MarketQuote::new(price, 2.0),
        MarketQuote::new(price + 1.0, 3.0),
        MarketQuote::new(price + 0.5, 4.0),
    )
}

#[test]
fn ticks_are_sorted_chronologically() {
    let ticks = MarketTicks::new(vec![
        tick_with_timestamp(3, 103.0),
        tick_with_timestamp(1, 100.0),
        tick_with_timestamp(2, 101.0),
    ]);

    assert_eq!(
        ticks.bid_timed_price_quantities(),
        vec![
            (TimestampMillis::new(1), 100.0, 2.0),
            (TimestampMillis::new(2), 101.0, 2.0),
            (TimestampMillis::new(3), 103.0, 2.0)
        ]
    );
}

#[test]
fn serde_json_round_trip() {
    let ticks = MarketTicks::new(vec![tick(100.0)]);
    let json = serde_json::to_string(&ticks).unwrap();
    let deserialized: MarketTicks = serde_json::from_str(&json).unwrap();

    assert_eq!(
        deserialized.bid_timed_price_quantities(),
        vec![(TimestampMillis::new(1), 100.0, 2.0)]
    );
}

#[test]
fn deserialization_sorts_ticks_chronologically() {
    let json = r#"{
        "ticks": [
            {
                "timestamp_millis": 3,
                "bid": {"price": 103.0, "quantity": 2.0},
                "ask": {"price": 104.0, "quantity": 3.0},
                "last": {"price": 103.5, "quantity": 4.0}
            },
            {
                "timestamp_millis": 1,
                "bid": {"price": 100.0, "quantity": 2.0},
                "ask": {"price": 101.0, "quantity": 3.0},
                "last": {"price": 100.5, "quantity": 4.0}
            }
        ]
    }"#;
    let deserialized: MarketTicks = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized.bid_timed_price_quantities(),
        vec![
            (TimestampMillis::new(1), 100.0, 2.0),
            (TimestampMillis::new(3), 103.0, 2.0)
        ]
    );
}
