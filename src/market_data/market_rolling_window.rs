use crate::market_data::{
    market_candle::MarketCandle, market_ohlcv::MarketOhlcv, rolling_window::RollingWindow,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An OHLCV tuple: open, high, low, close, volume, quote volume, and VWAP.
pub type Ohlcv = (f64, f64, f64, f64, f64, f64, f64);

/// Candles for fixed-length rolling windows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRollingWindow {
    candles: HashMap<RollingWindow, MarketCandle>,
}

impl MarketRollingWindow {
    /// Creates a rolling-window collection from candles grouped by window.
    pub fn new(candles: HashMap<RollingWindow, MarketCandle>) -> Self {
        Self { candles }
    }

    /// Returns the candle for a rolling window, if present.
    pub fn candle(&self, window: RollingWindow) -> Option<&MarketCandle> {
        self.candles.get(&window)
    }

    /// Returns all candles grouped by rolling window.
    pub fn candles(&self) -> &HashMap<RollingWindow, MarketCandle> {
        &self.candles
    }

    /// Returns `(open, high, low, close, volume, quote volume, VWAP)` for a
    /// rolling window, if present.
    pub fn ohlcv(&self, window: RollingWindow) -> Option<Ohlcv> {
        let ohlcv = self.candles.get(&window)?.ohlcv();
        Some((
            ohlcv.open(),
            ohlcv.high(),
            ohlcv.low(),
            ohlcv.close(),
            ohlcv.volume(),
            ohlcv.quote_volume(),
            ohlcv.vwap(),
        ))
    }

    /// Returns the OHLCV values as a [`MarketOhlcv`] struct, if present.
    pub fn ohlcv_struct(&self, window: RollingWindow) -> Option<MarketOhlcv> {
        self.candles
            .get(&window)
            .map(|candle| candle.ohlcv().clone())
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
        let market = MarketRollingWindow::new(HashMap::from([(RollingWindow::Minutes1, candle())]));
        let json = serde_json::to_string(&market).unwrap();
        let deserialized: MarketRollingWindow = serde_json::from_str(&json).unwrap();

        assert_eq!(
            deserialized.ohlcv(RollingWindow::Minutes1),
            Some((1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4))
        );
        assert_eq!(
            deserialized.ohlcv_struct(RollingWindow::Minutes1),
            Some(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap())
        );
    }
}
