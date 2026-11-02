use super::*;
use crate::market_data::timestamp::TimestampMillis;

fn candle() -> MarketCandle {
    MarketCandle::new(
        TimestampMillis::new(0),
        TimestampMillis::new(1),
        TimestampMillis::new(1),
        true,
        MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4),
        3,
    )
    .unwrap()
}

#[test]
fn serde_json_round_trip() {
    let market = MarketRollingWindow::new(HashMap::from([(RollingWindow::Minutes1, candle())]));
    let json = serde_json::to_string(&market).unwrap();
    let deserialized: MarketRollingWindow = serde_json::from_str(&json).unwrap();

    assert_eq!(
        deserialized.ohlcv(RollingWindow::Minutes1),
        Some((1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4))
    );
    assert_eq!(
        deserialized.ohlcv_struct(RollingWindow::Minutes1),
        Some(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4))
    );
}
