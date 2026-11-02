use crate::{
    error::{
        market_data::MarketDataError,
        result::{StockTrekError, StockTrekResult},
    },
    market_data::{market_ohlcv::MarketOhlcv, timestamp::TimestampMillis},
};
use serde::{Deserialize, Serialize};

/// A single OHLCV candle for a market.
///
/// The start is inclusive and the end is exclusive. `is_candle_closed` marks
/// candles that will no longer receive updates. The duration is validated
/// against the timestamp bounds when the candle is created or deserialized.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MarketCandleData")]
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
    ///
    /// Returns an error when the end precedes the start or when `duration_millis`
    /// does not equal the difference between the end and start timestamps.
    pub fn new(
        start_time_millis_inc: TimestampMillis,
        end_time_millis_exc: TimestampMillis,
        duration_millis: TimestampMillis,
        is_candle_closed: bool,
        ohlcv: MarketOhlcv,
        trade_count: u64,
    ) -> StockTrekResult<Self> {
        let expected_duration = end_time_millis_exc
            .checked_sub(start_time_millis_inc)
            .ok_or_else(|| {
                Self::error(format!(
                    "end time {end_time_millis_exc} precedes start time {start_time_millis_inc}"
                ))
            })?;

        if duration_millis != expected_duration {
            return Err(Self::error(format!(
                "duration {duration_millis} does not match the interval between start time \
                 {start_time_millis_inc} and end time {end_time_millis_exc} ({expected_duration})"
            )));
        }

        Ok(Self {
            start_time_millis_inc,
            end_time_millis_exc,
            duration_millis,
            is_candle_closed,
            ohlcv,
            trade_count,
        })
    }

    fn error(message: String) -> StockTrekError {
        StockTrekError::MarketData(MarketDataError::DomainError {
            function: "MarketCandle::new",
            message,
        })
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

#[derive(Deserialize)]
struct MarketCandleData {
    start_time_millis_inc: TimestampMillis,
    end_time_millis_exc: TimestampMillis,
    duration_millis: TimestampMillis,
    is_candle_closed: bool,
    ohlcv: MarketOhlcv,
    trade_count: u64,
}

impl TryFrom<MarketCandleData> for MarketCandle {
    type Error = StockTrekError;

    fn try_from(data: MarketCandleData) -> Result<Self, Self::Error> {
        Self::new(
            data.start_time_millis_inc,
            data.end_time_millis_exc,
            data.duration_millis,
            data.is_candle_closed,
            data.ohlcv,
            data.trade_count,
        )
    }
}

#[cfg(test)]
mod tests;
