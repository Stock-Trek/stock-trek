use super::*;
use crate::market_data::timestamp::TimestampMillis;

fn candle() -> MarketCandle {
    candle_at(0, 1.0)
}

fn candle_at(start_time_millis_inc: u64, open: f64) -> MarketCandle {
    MarketCandle::new(
        TimestampMillis::new(start_time_millis_inc),
        TimestampMillis::new(start_time_millis_inc + 1),
        TimestampMillis::new(1),
        true,
        MarketOhlcv::new(open, open + 1.0, open - 0.5, open + 0.5, 10.0, 15.0, 1.4),
        3,
    )
    .unwrap()
}

#[test]
fn new_sorts_candles_chronologically() {
    let market = MarketAlignedWindow::new(HashMap::from([(
        AlignedWindow::Minutes1,
        vec![candle_at(2, 2.0), candle_at(1, 1.0)],
    )]));

    assert_eq!(market.opens(AlignedWindow::Minutes1), Some(vec![1.0, 2.0]));
}

#[test]
fn serde_json_round_trip() {
    let market =
        MarketAlignedWindow::new(HashMap::from([(AlignedWindow::Minutes1, vec![candle()])]));
    let json = serde_json::to_string(&market).unwrap();
    let deserialized: MarketAlignedWindow = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.opens(AlignedWindow::Minutes1), Some(vec![1.0]));
}

#[test]
fn deserialization_sorts_candles_chronologically() {
    let json = serde_json::json!({
        "candles": {
            "1m": [
                candle_at(2, 2.0),
                candle_at(1, 1.0),
            ]
        }
    });
    let deserialized: MarketAlignedWindow = serde_json::from_value(json).unwrap();

    assert_eq!(
        deserialized.opens(AlignedWindow::Minutes1),
        Some(vec![1.0, 2.0])
    );
}
