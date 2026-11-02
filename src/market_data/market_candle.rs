use crate::market_data::{market_ohlcv::MarketOhlcv, timestamp::TimestampMillis};
use serde::{Deserialize, Serialize};

/// A single OHLCV candle for a market.
///
/// The start is inclusive and the end is exclusive. `is_candle_closed` marks
/// candles that will no longer receive updates.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketCandle {
    pub start_time_millis_inc: TimestampMillis,
    pub end_time_millis_exc: TimestampMillis,
    pub duration_millis: TimestampMillis,
    pub is_candle_closed: bool,
    pub ohlcv: MarketOhlcv,
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
