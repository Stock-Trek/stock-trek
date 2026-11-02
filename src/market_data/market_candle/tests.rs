use super::*;

fn ohlcv() -> MarketOhlcv {
    MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4)
}

fn candle() -> MarketCandle {
    MarketCandle::new(
        TimestampMillis::new(1_000),
        TimestampMillis::new(2_000),
        TimestampMillis::new(1_000),
        true,
        ohlcv(),
        3,
    )
    .unwrap()
}

#[test]
fn accepts_a_duration_that_matches_the_interval() {
    let candle = candle();

    assert_eq!(candle.duration_millis(), TimestampMillis::new(1_000));
    assert_eq!(
        candle
            .end_time_millis_exc()
            .checked_sub(candle.start_time_millis_inc()),
        Some(TimestampMillis::new(1_000))
    );
}

#[test]
fn accepts_a_zero_length_interval() {
    let candle = MarketCandle::new(
        TimestampMillis::new(1_000),
        TimestampMillis::new(1_000),
        TimestampMillis::new(0),
        true,
        ohlcv(),
        0,
    )
    .unwrap();

    assert_eq!(candle.duration_millis(), TimestampMillis::new(0));
}

#[test]
fn rejects_a_duration_that_does_not_match_the_interval() {
    let error = MarketCandle::new(
        TimestampMillis::new(1_000),
        TimestampMillis::new(2_000),
        TimestampMillis::new(999),
        true,
        ohlcv(),
        3,
    )
    .unwrap_err();

    assert!(matches!(
        error,
        StockTrekError::MarketData(MarketDataError::DomainError { .. })
    ));
}

#[test]
fn rejects_an_end_that_precedes_the_start() {
    let error = MarketCandle::new(
        TimestampMillis::new(2_000),
        TimestampMillis::new(1_000),
        TimestampMillis::new(1_000),
        true,
        ohlcv(),
        3,
    )
    .unwrap_err();

    assert!(matches!(
        error,
        StockTrekError::MarketData(MarketDataError::DomainError { .. })
    ));
}

#[test]
fn serde_round_trip_preserves_the_candle() {
    let candle = candle();
    let json = serde_json::to_value(&candle).unwrap();
    let deserialized: MarketCandle = serde_json::from_value(json).unwrap();

    assert_eq!(deserialized.duration_millis(), candle.duration_millis());
    assert_eq!(deserialized.ohlcv(), candle.ohlcv());
}

#[test]
fn deserialization_rejects_a_duration_that_does_not_match_the_interval() {
    let json = serde_json::json!({
        "start_time_millis_inc": 1_000,
        "end_time_millis_exc": 2_000,
        "duration_millis": 999,
        "is_candle_closed": true,
        "ohlcv": ohlcv(),
        "trade_count": 3,
    });

    assert!(serde_json::from_value::<MarketCandle>(json).is_err());
}

#[test]
fn deserialization_rejects_an_end_that_precedes_the_start() {
    let json = serde_json::json!({
        "start_time_millis_inc": 2_000,
        "end_time_millis_exc": 1_000,
        "duration_millis": 1_000,
        "is_candle_closed": true,
        "ohlcv": ohlcv(),
        "trade_count": 3,
    });

    assert!(serde_json::from_value::<MarketCandle>(json).is_err());
}
