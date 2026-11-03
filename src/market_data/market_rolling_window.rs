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
