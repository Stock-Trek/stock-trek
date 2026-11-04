use super::market_rolling_window::*;
use crate::market_data::{
    market_candle::MarketCandle, market_ohlcv::MarketOhlcv, rolling_window::RollingWindow,
    timestamp::TimestampMillis,
};

fn ohlcv() -> MarketOhlcv {
    MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0).unwrap()
}

#[test]
fn new_rejects_duplicate_windows() {
    assert!(
        MarketRollingWindow::new([
            (
                RollingWindow::Minutes1,
                TimestampMillis::new(1_000),
                true,
                ohlcv(),
                3,
            ),
            (
                RollingWindow::Minutes1,
                TimestampMillis::new(61_000),
                true,
                ohlcv(),
                3,
            ),
        ])
        .is_err()
    );
}

#[test]
fn new_forces_the_candle_duration() {
    let market = MarketRollingWindow::new([
        (
            RollingWindow::Minutes1,
            TimestampMillis::new(1_000),
            true,
            ohlcv(),
            3,
        ),
        (
            RollingWindow::Hours1,
            TimestampMillis::new(2_000),
            false,
            ohlcv(),
            0,
        ),
    ])
    .unwrap();

    let minutes1 = market.candle(RollingWindow::Minutes1).unwrap();
    assert_eq!(
        minutes1.start_time_millis_inc(),
        TimestampMillis::new(1_000)
    );
    assert_eq!(minutes1.end_time_millis_exc(), TimestampMillis::new(61_000));
    assert_eq!(
        minutes1.duration_millis(),
        RollingWindow::Minutes1.millis_exact()
    );
    assert!(minutes1.is_candle_closed());

    let hours1 = market.candle(RollingWindow::Hours1).unwrap();
    assert_eq!(
        hours1.end_time_millis_exc(),
        TimestampMillis::new(2_000 + RollingWindow::Hours1.millis_exact())
    );
    assert!(!hours1.is_candle_closed());
}

#[test]
fn serde_json_round_trip() {
    let market = MarketRollingWindow::new([(
        RollingWindow::Minutes1,
        TimestampMillis::new(1_000),
        true,
        ohlcv(),
        3,
    )])
    .unwrap();
    let json = serde_json::to_string(&market).unwrap();
    let deserialized: MarketRollingWindow = serde_json::from_str(&json).unwrap();

    assert_eq!(
        deserialized.ohlcv(RollingWindow::Minutes1),
        Some(&MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0).unwrap())
    );
}

#[test]
fn deserialization_rejects_a_candle_that_does_not_match_its_window() {
    let mismatched = MarketCandle::new(
        RollingWindow::Seconds1,
        TimestampMillis::new(0),
        true,
        ohlcv(),
        3,
    )
    .unwrap();
    let json = serde_json::json!({
        "candles": {
            "1m": mismatched,
        }
    });

    assert!(serde_json::from_value::<MarketRollingWindow>(json).is_err());
}
