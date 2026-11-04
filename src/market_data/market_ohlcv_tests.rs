use super::market_ohlcv::*;

#[test]
fn accepts_consistent_ohlcv_values() {
    let ohlcv = MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0).unwrap();

    assert_eq!(ohlcv.open(), 1.0);
    assert_eq!(ohlcv.high(), 2.0);
    assert_eq!(ohlcv.low(), 0.5);
    assert_eq!(ohlcv.close(), 1.5);
    assert_eq!(ohlcv.volume(), 10.0);
    assert_eq!(ohlcv.quote_volume(), 15.0);
    assert_eq!(ohlcv.vwap(), 1.4);
}

#[test]
fn rejects_non_finite_values() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(MarketOhlcv::new(value, 2.0, 0.5, 1.5, 10.0, 15.0).is_err());
        assert!(MarketOhlcv::new(1.0, value, 0.5, 1.5, 10.0, 15.0).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, value, 1.5, 10.0, 15.0).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, value, 10.0, 15.0).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, value, 15.0).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, value).is_err());
    }
}

#[test]
fn rejects_negative_values() {
    assert!(MarketOhlcv::new(-1.0, 2.0, 0.5, 1.5, 10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(1.0, -2.0, -3.0, 1.5, 10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(1.0, 2.0, -0.5, 1.5, 10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(1.0, 2.0, 0.5, -1.5, 10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, -10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, -15.0).is_err());
    assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0).is_err());
}

#[test]
fn rejects_inconsistent_highs_and_lows() {
    assert!(MarketOhlcv::new(1.0, 0.5, 2.0, 1.5, 10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(3.0, 2.0, 0.5, 1.5, 10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 3.0, 10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(0.0, 2.0, 0.5, 1.5, 10.0, 15.0).is_err());
    assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 0.0, 10.0, 15.0).is_err());
}

#[test]
fn serde_json_round_trip_preserves_the_values() {
    let ohlcv = MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0).unwrap();
    let json = serde_json::to_string(&ohlcv).unwrap();
    let deserialized: MarketOhlcv = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized, ohlcv);
}

#[test]
fn deserialization_rejects_invalid_values() {
    let json = serde_json::json!({
        "open": 1.0,
        "high": -1.0,
        "low": 2.0,
        "close": 1.5,
        "volume": -10.0,
        "quote_volume": -15.0,
        "vwap": 1.4,
    });

    assert!(serde_json::from_value::<MarketOhlcv>(json).is_err());
}

#[test]
fn deserialization_rejects_a_vwap_outside_the_high_and_low() {
    let json = serde_json::json!({
        "open": 100.0,
        "high": 110.0,
        "low": 90.0,
        "close": 105.0,
        "volume": 10.0,
        "quote_volume": 15.0,
        "vwap": 1000.0,
    });

    assert!(serde_json::from_value::<MarketOhlcv>(json).is_err());
}
