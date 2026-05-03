use crate::market_data::{market_ohlcv::MarketOhlcv, timestamp::TimestampMillis};
use serde::{Deserialize, Serialize};

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
