use crate::market_data::{market_ohlcv::MarketOhlcv, timestamp::TimestampMillis};
use serde::{Deserialize, Serialize};

/// A single OHLCV candle for a market.
///
/// The start is inclusive and the end is exclusive. `is_candle_closed` marks
/// candles that will no longer receive updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketCandle {
    start_time_millis_inc: TimestampMillis,
    end_time_millis_exc: TimestampMillis,
    duration_millis: TimestampMillis,
    is_candle_closed: bool,
    ohlcv: MarketOhlcv,
    trade_count: u64,
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

    /// Returns the inclusive start time of the candle.
    pub fn start_time_millis_inc(&self) -> TimestampMillis {
        self.start_time_millis_inc
    }

    /// Returns the exclusive end time of the candle.
    pub fn end_time_millis_exc(&self) -> TimestampMillis {
        self.end_time_millis_exc
    }

    /// Returns the candle duration in milliseconds.
    pub fn duration_millis(&self) -> TimestampMillis {
        self.duration_millis
    }

    /// Returns whether the candle is closed and will no longer receive updates.
    pub fn is_candle_closed(&self) -> bool {
        self.is_candle_closed
    }

    /// Returns the candle's OHLCV values.
    pub fn ohlcv(&self) -> &MarketOhlcv {
        &self.ohlcv
    }

    /// Returns the number of trades in the candle.
    pub fn trade_count(&self) -> u64 {
        self.trade_count
    }
}
