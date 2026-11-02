use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
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
        StockTrekError::Stats(StatsError::DomainError {
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
mod tests {
    use super::*;

    fn ohlcv() -> MarketOhlcv {
        MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4)
    }

    fn candle() -> MarketCandle {
        MarketCandle::new(
            TimestampMillis::new(1_000),
            TimestampMillis::new(2_000),
            TimestampMillis::new(1_000),
            true,
            ohlcv(),
            3,
        )
        .unwrap()
    }

    #[test]
    fn accepts_a_duration_that_matches_the_interval() {
        let candle = candle();

        assert_eq!(candle.duration_millis(), TimestampMillis::new(1_000));
        assert_eq!(
            candle
                .end_time_millis_exc()
                .checked_sub(candle.start_time_millis_inc()),
            Some(TimestampMillis::new(1_000))
        );
    }

    #[test]
    fn accepts_a_zero_length_interval() {
        let candle = MarketCandle::new(
            TimestampMillis::new(1_000),
            TimestampMillis::new(1_000),
            TimestampMillis::new(0),
            true,
            ohlcv(),
            0,
        )
        .unwrap();

        assert_eq!(candle.duration_millis(), TimestampMillis::new(0));
    }

    #[test]
    fn rejects_a_duration_that_does_not_match_the_interval() {
        let error = MarketCandle::new(
            TimestampMillis::new(1_000),
            TimestampMillis::new(2_000),
            TimestampMillis::new(999),
            true,
            ohlcv(),
            3,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            StockTrekError::Stats(StatsError::DomainError { .. })
        ));
    }

    #[test]
    fn rejects_an_end_that_precedes_the_start() {
        let error = MarketCandle::new(
            TimestampMillis::new(2_000),
            TimestampMillis::new(1_000),
            TimestampMillis::new(1_000),
            true,
            ohlcv(),
            3,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            StockTrekError::Stats(StatsError::DomainError { .. })
        ));
    }

    #[test]
    fn serde_round_trip_preserves_the_candle() {
        let candle = candle();
        let json = serde_json::to_value(&candle).unwrap();
        let deserialized: MarketCandle = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized.duration_millis(), candle.duration_millis());
        assert_eq!(deserialized.ohlcv(), candle.ohlcv());
    }

    #[test]
    fn deserialization_rejects_a_duration_that_does_not_match_the_interval() {
        let json = serde_json::json!({
            "start_time_millis_inc": 1_000,
            "end_time_millis_exc": 2_000,
            "duration_millis": 999,
            "is_candle_closed": true,
            "ohlcv": ohlcv(),
            "trade_count": 3,
        });

        assert!(serde_json::from_value::<MarketCandle>(json).is_err());
    }

    #[test]
    fn deserialization_rejects_an_end_that_precedes_the_start() {
        let json = serde_json::json!({
            "start_time_millis_inc": 2_000,
            "end_time_millis_exc": 1_000,
            "duration_millis": 1_000,
            "is_candle_closed": true,
            "ohlcv": ohlcv(),
            "trade_count": 3,
        });

        assert!(serde_json::from_value::<MarketCandle>(json).is_err());
    }
}
