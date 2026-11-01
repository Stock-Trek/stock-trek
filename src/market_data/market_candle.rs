use crate::market_data::{market_ohlcv::MarketOhlcv, timestamp::TimestampMillis};
use serde::{Deserialize, Serialize};

/// A single OHLCV candle for a market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketCandle {
    /// The inclusive start timestamp in milliseconds.
    pub start_time_millis_inc: TimestampMillis,
    /// The exclusive end timestamp in milliseconds.
    pub end_time_millis_exc: TimestampMillis,
    /// The candle duration in milliseconds.
    pub duration_millis: TimestampMillis,
    /// Whether the candle is closed and no longer updating.
    pub is_candle_closed: bool,
    /// The candle's open, high, low, close, and volume values.
    pub ohlcv: MarketOhlcv,
    /// The number of trades in the candle.
    pub trade_count: u64,
}

impl MarketCandle {
    /// Creates a market candle from its timestamp, OHLCV, and trade count.
    pub fn new(
        start_time_millis_inc: TimestampMillis,
        end_time_millis_exc: TimestampMillis,
        duration_millis: TimestampMillis,
        is_candle_closed: bool,
        ohlcv: MarketOhlcv,
        trade_count: u64,
    ) -> Self {
        Self {
            start_time_millis_inc,
            end_time_millis_exc,
            duration_millis,
            is_candle_closed,
            ohlcv,
            trade_count,
        }
    }
}
