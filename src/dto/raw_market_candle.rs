use crate::{dto::raw_market_ohlcv::RawMarketOhlcv, prelude::TimestampMillis};

#[derive(Debug)]
pub struct RawMarketCandle {
    pub start_time_millis_inc: TimestampMillis,
    pub end_time_millis_exc: TimestampMillis,
    pub duration_millis: TimestampMillis,
    pub is_candle_closed: bool,
    pub ohlcv: RawMarketOhlcv,
    pub trade_count: u64,
}
