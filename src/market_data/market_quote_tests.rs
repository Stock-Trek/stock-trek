use super::market_quote::*;

use crate::market_data::{market_quote::TimedPriceQuantity, timestamp::TimestampMillis};

#[test]
fn accepts_a_finite_positive_quote() {
    let quote = MarketQuote::new(100.0, 1.5).unwrap();

    assert_eq!(quote.price(), 100.0);
    assert_eq!(quote.quantity(), 1.5);
}

#[test]
fn rejects_non_finite_prices_and_quantities() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(MarketQuote::new(value, 1.0).is_err());
        assert!(MarketQuote::new(1.0, value).is_err());
    }
}

#[test]
fn rejects_zero_and_negative_prices_and_quantities() {
    for value in [0.0, -1.0] {
        assert!(MarketQuote::new(value, 1.0).is_err());
        assert!(MarketQuote::new(1.0, value).is_err());
    }
}

#[test]
fn serde_json_round_trip_preserves_the_quote() {
    let quote = MarketQuote::new(100.0, 1.5).unwrap();
    let json = serde_json::to_string(&quote).unwrap();
    let deserialized: MarketQuote = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.price(), 100.0);
    assert_eq!(deserialized.quantity(), 1.5);
}

#[test]
fn deserialization_rejects_invalid_quotes() {
    for (price, quantity) in [(0.0, 1.0), (1.0, 0.0), (-1.0, 1.0), (1.0, -1.0)] {
        let json = serde_json::json!({
            "price": price,
            "quantity": quantity,
        });

        assert!(serde_json::from_value::<MarketQuote>(json).is_err());
    }
}

#[test]
fn timed_price_quantity_exposes_named_values() {
    let timed = TimedPriceQuantity::new(TimestampMillis::new(1), 100.0, 1.5).unwrap();

    assert_eq!(timed.timestamp_millis(), TimestampMillis::new(1));
    assert_eq!(timed.price(), 100.0);
    assert_eq!(timed.quantity(), 1.5);
}

#[test]
fn timed_price_quantity_rejects_invalid_quotes() {
    assert!(TimedPriceQuantity::new(TimestampMillis::new(1), 0.0, 1.0).is_err());
}

#[test]
fn timed_price_quantity_serde_round_trip() {
    let timed = TimedPriceQuantity::new(TimestampMillis::new(1), 100.0, 1.5).unwrap();
    let json = serde_json::to_string(&timed).unwrap();
    let deserialized: TimedPriceQuantity = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized, timed);
}

#[test]
fn timed_price_quantity_deserialization_rejects_invalid_quotes() {
    let json = serde_json::json!({
        "timestamp_millis": 1,
        "price": 0.0,
        "quantity": 1.5,
    });

    assert!(serde_json::from_value::<TimedPriceQuantity>(json).is_err());
}
