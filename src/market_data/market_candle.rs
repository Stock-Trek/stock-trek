use crate::{market_data::market_ohlcv::MarketOhlcv, prelude::TimestampMillis};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketCandle {
    pub start_time_millis_inc: TimestampMillis,
    pub end_time_millis_exc: TimestampMillis,
    pub duration_millis: TimestampMillis,
    pub is_candle_closed: bool,
    pub ohlcv: MarketOhlcv,
    pub trade_count: u64,
}
