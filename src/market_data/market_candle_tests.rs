use super::*;

fn ohlcv() -> MarketOhlcv {
    MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap()
}

fn candle() -> MarketCandle {
    MarketCandle::new(
        RollingWindow::Seconds1,
        TimestampMillis::new(1_000),
        true,
        ohlcv(),
        3,
    )
    .unwrap()
}

#[test]
fn new_derives_the_end_from_the_window_length() {
    let candle = MarketCandle::new(
        RollingWindow::Minutes1,
        TimestampMillis::new(1_000),
        true,
        ohlcv(),
        3,
    )
    .unwrap();

    assert_eq!(candle.start_time_millis_inc(), TimestampMillis::new(1_000));
    assert_eq!(candle.end_time_millis_exc(), TimestampMillis::new(61_000));
    assert_eq!(candle.duration_millis(), 60_000);
    assert!(candle.is_candle_closed());
}

#[test]
fn new_rejects_an_end_that_overflows_the_timestamp_range() {
    assert!(
        MarketCandle::new(
            RollingWindow::Days1,
            TimestampMillis::new(u64::MAX),
            true,
            ohlcv(),
            3,
        )
        .is_err()
    );
}

#[test]
fn derives_the_duration_from_the_window() {
    let candle = candle();

    assert_eq!(candle.duration_millis(), 1_000);
    assert_eq!(
        candle
            .end_time_millis_exc()
            .checked_sub(candle.start_time_millis_inc()),
        Some(TimestampMillis::new(1_000))
    );
}

#[test]
fn from_interval_accepts_a_zero_length_interval() {
    let candle = MarketCandle::from_interval(
        TimestampMillis::new(1_000),
        TimestampMillis::new(1_000),
        true,
        ohlcv(),
        0,
    )
    .unwrap();

    assert_eq!(candle.duration_millis(), 0);
}

#[test]
fn from_interval_rejects_an_end_that_precedes_the_start() {
    let error = MarketCandle::from_interval(
        TimestampMillis::new(2_000),
        TimestampMillis::new(1_000),
        true,
        ohlcv(),
        3,
    )
    .unwrap_err();

    assert!(matches!(error, StockTrekError::Domain(_)));
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
fn deserialization_rejects_an_end_that_precedes_the_start() {
    let json = serde_json::json!({
        "start_time_millis_inc": 2_000,
        "end_time_millis_exc": 1_000,
        "is_candle_closed": true,
        "ohlcv": ohlcv(),
        "trade_count": 3,
    });

    assert!(serde_json::from_value::<MarketCandle>(json).is_err());
}
