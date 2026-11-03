use crate::{
    error::result::{StockTrekError, StockTrekResult},
    market_data::{
        market_ohlcv::MarketOhlcv, rolling_window::RollingWindow, timestamp::TimestampMillis,
    },
    validation::Validation,
};
use serde::{Deserialize, Serialize};

/// A single OHLCV candle for a market.
///
/// The start is inclusive and the end is exclusive. `is_candle_closed` marks
/// candles that will no longer receive updates. The end timestamp is validated
/// against the start when the candle is deserialized.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MarketCandleData")]
pub struct MarketCandle {
    start_time_millis_inc: TimestampMillis,
    end_time_millis_exc: TimestampMillis,
    is_candle_closed: bool,
    ohlcv: MarketOhlcv,
    trade_count: u64,
}

impl MarketCandle {
    /// Creates a market candle that covers exactly one rolling window
    /// starting at `start_time_millis_inc`.
    ///
    /// The end timestamp is derived from the window length, so the candle
    /// duration always matches the window. Returns an error when the end
    /// timestamp would overflow the timestamp range.
    pub fn new(
        window: RollingWindow,
        start_time_millis_inc: TimestampMillis,
        is_candle_closed: bool,
        ohlcv: MarketOhlcv,
        trade_count: u64,
    ) -> StockTrekResult<Self> {
        let window_millis = window.millis_exact();
        let end_time_millis_exc = start_time_millis_inc
            .checked_add(TimestampMillis::new(window_millis))
            .ok_or_else(|| {
                Self::error(
                    "MarketCandle::new",
                    format!(
                        "start time {start_time_millis_inc} plus the {window} window duration {window_millis} ms overflows the timestamp range"
                    ),
                )
            })?;
        Self::from_interval(
            start_time_millis_inc,
            end_time_millis_exc,
            is_candle_closed,
            ohlcv,
            trade_count,
        )
    }

    pub(super) fn from_interval(
        start_time_millis_inc: TimestampMillis,
        end_time_millis_exc: TimestampMillis,
        is_candle_closed: bool,
        ohlcv: MarketOhlcv,
        trade_count: u64,
    ) -> StockTrekResult<Self> {
        end_time_millis_exc
            .checked_sub(start_time_millis_inc)
            .ok_or_else(|| {
                Self::error(
                    "MarketCandle::from_interval",
                    format!(
                        "end time {end_time_millis_exc} precedes start time {start_time_millis_inc}"
                    ),
                )
            })?;

        Ok(Self {
            start_time_millis_inc,
            end_time_millis_exc,
            is_candle_closed,
            ohlcv,
            trade_count,
        })
    }

    fn error(function: &'static str, message: String) -> StockTrekError {
        Validation::error(function, message)
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
    ///
    /// The result is a duration, not a timestamp, so it is returned as `u64`
    /// rather than [`TimestampMillis`].
    pub fn duration_millis(&self) -> u64 {
        self.end_time_millis_exc
            .as_millis()
            .checked_sub(self.start_time_millis_inc.as_millis())
            .expect("MarketCandle guarantees the end is not before the start")
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
    is_candle_closed: bool,
    ohlcv: MarketOhlcv,
    trade_count: u64,
}

impl TryFrom<MarketCandleData> for MarketCandle {
    type Error = StockTrekError;

    fn try_from(data: MarketCandleData) -> Result<Self, Self::Error> {
        Self::from_interval(
            data.start_time_millis_inc,
            data.end_time_millis_exc,
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
        MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap()
    }

    fn candle() -> MarketCandle {
        MarketCandle::new(
            RollingWindow::Seconds1,
            TimestampMillis::new(1_000),
            true,
            ohlcv(),
            3,
        )
        .unwrap()
    }

    #[test]
    fn new_derives_the_end_from_the_window_length() {
        let candle = MarketCandle::new(
            RollingWindow::Minutes1,
            TimestampMillis::new(1_000),
            true,
            ohlcv(),
            3,
        )
        .unwrap();

        assert_eq!(candle.start_time_millis_inc(), TimestampMillis::new(1_000));
        assert_eq!(candle.end_time_millis_exc(), TimestampMillis::new(61_000));
        assert_eq!(candle.duration_millis(), 60_000);
        assert!(candle.is_candle_closed());
    }

    #[test]
    fn new_rejects_an_end_that_overflows_the_timestamp_range() {
        assert!(
            MarketCandle::new(
                RollingWindow::Days1,
                TimestampMillis::new(u64::MAX),
                true,
                ohlcv(),
                3,
            )
            .is_err()
        );
    }

    #[test]
    fn derives_the_duration_from_the_window() {
        let candle = candle();

        assert_eq!(candle.duration_millis(), 1_000);
        assert_eq!(
            candle
                .end_time_millis_exc()
                .checked_sub(candle.start_time_millis_inc()),
            Some(TimestampMillis::new(1_000))
        );
    }

    #[test]
    fn from_interval_accepts_a_zero_length_interval() {
        let candle = MarketCandle::from_interval(
            TimestampMillis::new(1_000),
            TimestampMillis::new(1_000),
            true,
            ohlcv(),
            0,
        )
        .unwrap();

        assert_eq!(candle.duration_millis(), 0);
    }

    #[test]
    fn from_interval_rejects_an_end_that_precedes_the_start() {
        let error = MarketCandle::from_interval(
            TimestampMillis::new(2_000),
            TimestampMillis::new(1_000),
            true,
            ohlcv(),
            3,
        )
        .unwrap_err();

        assert!(matches!(error, StockTrekError::Domain(_)));
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
    fn deserialization_rejects_an_end_that_precedes_the_start() {
        let json = serde_json::json!({
            "start_time_millis_inc": 2_000,
            "end_time_millis_exc": 1_000,
            "is_candle_closed": true,
            "ohlcv": ohlcv(),
            "trade_count": 3,
        });

        assert!(serde_json::from_value::<MarketCandle>(json).is_err());
    }
}
