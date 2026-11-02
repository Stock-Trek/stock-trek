use crate::market_data::{
    market_candle::MarketCandle, market_ohlcv::MarketOhlcv, rolling_window::RollingWindow,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Candles for fixed-length rolling windows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRollingWindow {
    candles: BTreeMap<RollingWindow, MarketCandle>,
}

impl MarketRollingWindow {
    /// Creates a rolling-window collection from candles grouped by window.
    pub fn new(candles: impl IntoIterator<Item = (RollingWindow, MarketCandle)>) -> Self {
        Self {
            candles: candles.into_iter().collect(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::timestamp::TimestampMillis;

    fn candle() -> MarketCandle {
        MarketCandle::new(
            TimestampMillis::new(0),
            TimestampMillis::new(1),
            TimestampMillis::new(1),
            true,
            MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap(),
            3,
        )
        .unwrap()
    }

    #[test]
    fn serde_json_round_trip() {
        let market =
            MarketRollingWindow::new(BTreeMap::from([(RollingWindow::Minutes1, candle())]));
        let json = serde_json::to_string(&market).unwrap();
        let deserialized: MarketRollingWindow = serde_json::from_str(&json).unwrap();

        assert_eq!(
            deserialized.ohlcv(RollingWindow::Minutes1),
            Some(&MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap())
        );
    }

    #[test]
    fn candles_are_ordered_by_window_regardless_of_insertion_order() {
        let market = MarketRollingWindow::new(std::collections::HashMap::from([
            (RollingWindow::Hours1, candle()),
            (RollingWindow::Seconds1, candle()),
            (RollingWindow::Minutes1, candle()),
        ]));

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
