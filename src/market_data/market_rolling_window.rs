use crate::{
    error::result::StockTrekResult,
    market_data::{
        market_candle::MarketCandle, market_ohlcv::MarketOhlcv, rolling_window::RollingWindow,
        timestamp::TimestampMillis,
    },
    validation::Validation,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;

/// Candles for fixed-length rolling windows.
#[derive(Debug, Clone, Serialize)]
pub struct MarketRollingWindow {
    pub(super) candles: BTreeMap<RollingWindow, MarketCandle>,
}

impl MarketRollingWindow {
    /// Creates a rolling-window collection from candle starts grouped by
    /// window.
    ///
    /// Each entry is a `(window, start time, is candle closed, OHLCV, trade
    /// count)` tuple, and each candle ends exactly one window length after its
    /// start, so the candle duration always matches its window. Returns an
    /// error when the same window appears more than once or when a candle end
    /// would overflow the timestamp range.
    pub fn new(
        candles: impl IntoIterator<Item = (RollingWindow, TimestampMillis, bool, MarketOhlcv, u64)>,
    ) -> StockTrekResult<Self> {
        let mut candles_by_window = BTreeMap::new();
        for (window, start_time_millis_inc, is_candle_closed, ohlcv, trade_count) in candles {
            let candle = MarketCandle::new(
                window,
                start_time_millis_inc,
                is_candle_closed,
                ohlcv,
                trade_count,
            )?;
            if candles_by_window.insert(window, candle).is_some() {
                return Err(Validation::error(
                    "MarketRollingWindow::new",
                    format!("duplicate candle for rolling window {window:?}"),
                ));
            }
        }
        Ok(Self {
            candles: candles_by_window,
        })
    }

    /// Returns the candle for a rolling window, if present.
    pub fn candle(&self, window: RollingWindow) -> Option<&MarketCandle> {
        self.candles.get(&window)
    }

    /// Returns all candles grouped by rolling window, ordered by window.
    pub fn candles(&self) -> &BTreeMap<RollingWindow, MarketCandle> {
        &self.candles
    }

    /// Returns the OHLCV values for a rolling window, if present.
    pub fn ohlcv(&self, window: RollingWindow) -> Option<&MarketOhlcv> {
        self.candles.get(&window).map(MarketCandle::ohlcv)
    }
}

impl<'de> Deserialize<'de> for MarketRollingWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MarketRollingWindowData {
            candles: BTreeMap<RollingWindow, MarketCandle>,
        }

        let data = MarketRollingWindowData::deserialize(deserializer)?;
        for (window, candle) in &data.candles {
            let expected_duration = window.millis_exact();
            let actual_duration = candle.duration_millis();
            if actual_duration != expected_duration {
                return Err(serde::de::Error::custom(format!(
                    "candle duration {actual_duration} ms does not match the {window} window duration {expected_duration} ms"
                )));
            }
        }
        Ok(Self {
            candles: data.candles,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ohlcv() -> MarketOhlcv {
        MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap()
    }

    #[test]
    fn new_rejects_duplicate_windows() {
        assert!(
            MarketRollingWindow::new([
                (
                    RollingWindow::Minutes1,
                    TimestampMillis::new(1_000),
                    true,
                    ohlcv(),
                    3,
                ),
                (
                    RollingWindow::Minutes1,
                    TimestampMillis::new(61_000),
                    true,
                    ohlcv(),
                    3,
                ),
            ])
            .is_err()
        );
    }

    #[test]
    fn new_forces_the_candle_duration() {
        let market = MarketRollingWindow::new([
            (
                RollingWindow::Minutes1,
                TimestampMillis::new(1_000),
                true,
                ohlcv(),
                3,
            ),
            (
                RollingWindow::Hours1,
                TimestampMillis::new(2_000),
                false,
                ohlcv(),
                0,
            ),
        ])
        .unwrap();

        let minutes1 = market.candle(RollingWindow::Minutes1).unwrap();
        assert_eq!(
            minutes1.start_time_millis_inc(),
            TimestampMillis::new(1_000)
        );
        assert_eq!(minutes1.end_time_millis_exc(), TimestampMillis::new(61_000));
        assert_eq!(
            minutes1.duration_millis(),
            RollingWindow::Minutes1.millis_exact()
        );
        assert!(minutes1.is_candle_closed());

        let hours1 = market.candle(RollingWindow::Hours1).unwrap();
        assert_eq!(
            hours1.end_time_millis_exc(),
            TimestampMillis::new(2_000 + RollingWindow::Hours1.millis_exact())
        );
        assert!(!hours1.is_candle_closed());
    }

    #[test]
    fn serde_json_round_trip() {
        let market = MarketRollingWindow::new([(
            RollingWindow::Minutes1,
            TimestampMillis::new(1_000),
            true,
            ohlcv(),
            3,
        )])
        .unwrap();
        let json = serde_json::to_string(&market).unwrap();
        let deserialized: MarketRollingWindow = serde_json::from_str(&json).unwrap();

        assert_eq!(
            deserialized.ohlcv(RollingWindow::Minutes1),
            Some(&MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap())
        );
    }

    #[test]
    fn deserialization_rejects_a_candle_that_does_not_match_its_window() {
        let mismatched = MarketCandle::new(
            RollingWindow::Seconds1,
            TimestampMillis::new(0),
            true,
            ohlcv(),
            3,
        )
        .unwrap();
        let json = serde_json::json!({
            "candles": {
                "1m": mismatched,
            }
        });

        assert!(serde_json::from_value::<MarketRollingWindow>(json).is_err());
    }

    #[test]
    fn candles_are_ordered_by_window_regardless_of_insertion_order() {
        let market = MarketRollingWindow::new([
            (
                RollingWindow::Hours1,
                TimestampMillis::new(1_000),
                true,
                ohlcv(),
                3,
            ),
            (
                RollingWindow::Seconds1,
                TimestampMillis::new(1_000),
                true,
                ohlcv(),
                3,
            ),
            (
                RollingWindow::Minutes1,
                TimestampMillis::new(1_000),
                true,
                ohlcv(),
                3,
            ),
        ])
        .unwrap();

        let windows: Vec<_> = market.candles().keys().copied().collect();

        assert_eq!(
            windows,
            vec![
                RollingWindow::Seconds1,
                RollingWindow::Minutes1,
                RollingWindow::Hours1,
            ]
        );
    }
}
