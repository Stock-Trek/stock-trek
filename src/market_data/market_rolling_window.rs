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
    /// The candle for each rolling window.
    pub candles: HashMap<RollingWindow, MarketCandle>,
}

impl MarketRollingWindow {
    /// Creates a rolling-window collection from candles grouped by window.
    pub fn new(candles: HashMap<RollingWindow, MarketCandle>) -> Self {
        Self { candles }
    }

    /// Returns the OHLCV tuple for a rolling window, if present.
    pub fn ohlcv(&self, window: RollingWindow) -> Option<Ohlcv> {
        let ohlcv = &self.candles.get(&window)?.ohlcv;
        Some((
            ohlcv.open,
            ohlcv.high,
            ohlcv.low,
            ohlcv.close,
            ohlcv.volume,
            ohlcv.quote_volume,
            ohlcv.vwap,
        ))
    }

    /// Returns the OHLCV values for a rolling window as a struct, if present.
    pub fn ohlcv_struct(&self, window: RollingWindow) -> Option<MarketOhlcv> {
        self.candles.get(&window).map(|candle| candle.ohlcv.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candle() -> MarketCandle {
        MarketCandle::new(
            0,
            1,
            1,
            true,
            MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4),
            3,
        )
    }

    #[test]
    fn ohlcv_returns_none_when_window_is_absent() {
        let market = MarketRollingWindow::new(HashMap::new());

        assert!(market.ohlcv(RollingWindow::Minutes1).is_none());
        assert!(market.ohlcv_struct(RollingWindow::Minutes1).is_none());
    }

    #[test]
    fn ohlcv_returns_values_when_window_is_present() {
        let market = MarketRollingWindow::new(HashMap::from([(RollingWindow::Minutes1, candle())]));

        assert_eq!(
            market.ohlcv(RollingWindow::Minutes1),
            Some((1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4))
        );
    }

    #[test]
    fn ohlcv_struct_returns_values_when_window_is_present() {
        let market = MarketRollingWindow::new(HashMap::from([(RollingWindow::Minutes1, candle())]));

        assert_eq!(
            market.ohlcv_struct(RollingWindow::Minutes1),
            Some(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4))
        );
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
            Some(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4))
        );
    }
}
