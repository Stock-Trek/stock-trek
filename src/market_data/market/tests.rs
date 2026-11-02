use super::*;
use crate::market_data::{
    aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
    market_quote::MarketQuote, rolling_window::RollingWindow, timestamp::TimestampMillis,
};

fn market() -> Market {
    MarketBuilder::new(0.000_001, 0.01, 1.0).build().unwrap()
}

fn candle(start_millis: u64) -> MarketCandle {
    MarketCandle::new(
        TimestampMillis::new(start_millis),
        TimestampMillis::new(start_millis + 1_000),
        true,
        MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap(),
        3,
    )
    .unwrap()
}

#[test]
fn builds_a_market_with_valid_parameters() {
    let market = market();

    assert_eq!(market.base_increment(), 0.000_001);
    assert_eq!(market.quote_increment(), 0.01);
    assert_eq!(market.minimum_notional(), 1.0);
}

#[test]
fn builder_methods_can_be_chained() {
    let market = MarketBuilder::new(0.000_001, 0.01, 1.0)
        .tick(
            TimestampMillis::new(1),
            MarketQuote::new(100.0, 1.0).unwrap(),
            MarketQuote::new(101.0, 2.0).unwrap(),
            MarketQuote::new(100.5, 3.0).unwrap(),
        )
        .rolling_set_candle(RollingWindow::Minutes1, candle(1_000))
        .aligned_push_candle(AlignedWindow::Minutes1, candle(1_000))
        .order_book_bid(100.0, 1.0)
        .order_book_ask(101.0, 2.0)
        .build()
        .unwrap();

    assert_eq!(market.base_increment(), 0.000_001);
    assert_eq!(market.quote_increment(), 0.01);
    assert_eq!(market.minimum_notional(), 1.0);
    assert_eq!(market.ticks().ticks().len(), 1);
    assert_eq!(market.rolling().candles().len(), 1);
    assert_eq!(market.aligned().candles().len(), 1);
    assert_eq!(market.order_book().bids().len(), 1);
    assert_eq!(market.order_book().asks().len(), 1);
}

#[test]
fn rejects_non_finite_and_non_positive_parameters() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -1.0] {
        assert!(MarketBuilder::new(value, 0.01, 1.0).build().is_err());
        assert!(MarketBuilder::new(0.000_001, value, 1.0).build().is_err());
    }
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
        assert!(MarketBuilder::new(0.000_001, 0.01, value).build().is_err());
    }
}

#[test]
fn rejects_invalid_order_book_quotes() {
    let builder = MarketBuilder::new(0.000_001, 0.01, 1.0)
        .order_book_bid(f64::NAN, 1.0)
        .order_book_ask(101.0, f64::INFINITY);

    assert!(builder.build().is_err());
}

#[test]
fn serde_json_round_trip_preserves_the_market() {
    let market = MarketBuilder::new(0.000_001, 0.01, 1.0)
        .order_book_bid(100.0, 1.0)
        .order_book_ask(101.0, 1.0)
        .build()
        .unwrap();

    let json = serde_json::to_string(&market).unwrap();
    let deserialized: Market = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.base_increment(), market.base_increment());
    assert_eq!(deserialized.order_book().bids(), market.order_book().bids());
}

#[test]
fn deserialization_rejects_invalid_parameters() {
    let mut value = serde_json::to_value(market()).unwrap();
    value["base_increment"] = serde_json::json!(-1.0);

    assert!(serde_json::from_value::<Market>(value).is_err());
}

#[test]
fn deserialization_rejects_invalid_nested_quotes() {
    let market = MarketBuilder::new(0.000_001, 0.01, 1.0)
        .order_book_bid(100.0, 1.0)
        .build()
        .unwrap();
    let mut value = serde_json::to_value(market).unwrap();
    value["order_book"]["bids"][0]["quantity"] = serde_json::json!(0.0);

    assert!(serde_json::from_value::<Market>(value).is_err());
}
