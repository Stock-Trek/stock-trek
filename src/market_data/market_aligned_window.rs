use crate::market_data::{
    aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Candles grouped by calendar-aligned windows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketAlignedWindow {
    /// Candles for each aligned window.
    pub candles: HashMap<AlignedWindow, Vec<MarketCandle>>,
}

impl MarketAlignedWindow {
    /// Creates an aligned-window collection from candles grouped by window.
    pub fn new(candles: HashMap<AlignedWindow, Vec<MarketCandle>>) -> Self {
        Self { candles }
    }
    /// Returns the open prices for an aligned window, if present.
    pub fn opens(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, |ohlcv| ohlcv.open)
    }
    /// Returns the high prices for an aligned window, if present.
    pub fn highs(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, |ohlcv| ohlcv.high)
    }
    /// Returns the low prices for an aligned window, if present.
    pub fn lows(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, |ohlcv| ohlcv.low)
    }
    /// Returns the close prices for an aligned window, if present.
    pub fn closes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, |ohlcv| ohlcv.close)
    }
    /// Returns the base volumes for an aligned window, if present.
    pub fn volumes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, |ohlcv| ohlcv.volume)
    }
    /// Returns the quote volumes for an aligned window, if present.
    pub fn quote_volumes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, |ohlcv| ohlcv.quote_volume)
    }
    /// Returns the volume-weighted average prices for an aligned window, if present.
    pub fn vwaps(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, |ohlcv| ohlcv.vwap)
    }

    fn values(
        &self,
        window: AlignedWindow,
        to_value: impl Fn(&MarketOhlcv) -> f64,
    ) -> Option<Vec<f64>> {
        let candles = self.candles.get(&window)?;
        Some(
            candles
                .iter()
                .map(|candle| to_value(&candle.ohlcv))
                .collect(),
        )
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
    fn values_return_none_when_window_is_absent() {
        let market = MarketAlignedWindow::new(HashMap::new());

        assert!(market.opens(AlignedWindow::Minutes1).is_none());
        assert!(market.highs(AlignedWindow::Minutes1).is_none());
        assert!(market.lows(AlignedWindow::Minutes1).is_none());
        assert!(market.closes(AlignedWindow::Minutes1).is_none());
        assert!(market.volumes(AlignedWindow::Minutes1).is_none());
        assert!(market.quote_volumes(AlignedWindow::Minutes1).is_none());
        assert!(market.vwaps(AlignedWindow::Minutes1).is_none());
    }

    #[test]
    fn values_return_data_when_window_is_present() {
        let market =
            MarketAlignedWindow::new(HashMap::from([(AlignedWindow::Minutes1, vec![candle()])]));

        assert_eq!(market.opens(AlignedWindow::Minutes1), Some(vec![1.0]));
        assert_eq!(market.highs(AlignedWindow::Minutes1), Some(vec![2.0]));
        assert_eq!(market.lows(AlignedWindow::Minutes1), Some(vec![0.5]));
        assert_eq!(market.closes(AlignedWindow::Minutes1), Some(vec![1.5]));
        assert_eq!(market.volumes(AlignedWindow::Minutes1), Some(vec![10.0]));
        assert_eq!(
            market.quote_volumes(AlignedWindow::Minutes1),
            Some(vec![15.0])
        );
        assert_eq!(market.vwaps(AlignedWindow::Minutes1), Some(vec![1.4]));
    }

    #[test]
    fn values_reflect_current_candles() {
        let mut market =
            MarketAlignedWindow::new(HashMap::from([(AlignedWindow::Minutes1, vec![candle()])]));

        assert_eq!(market.opens(AlignedWindow::Minutes1), Some(vec![1.0]));

        market.candles.get_mut(&AlignedWindow::Minutes1).unwrap()[0]
            .ohlcv
            .open = 1.1;

        assert_eq!(market.opens(AlignedWindow::Minutes1), Some(vec![1.1]));
    }

    #[test]
    fn serde_json_round_trip() {
        let market =
            MarketAlignedWindow::new(HashMap::from([(AlignedWindow::Minutes1, vec![candle()])]));
        let json = serde_json::to_string(&market).unwrap();
        let deserialized: MarketAlignedWindow = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.opens(AlignedWindow::Minutes1), Some(vec![1.0]));
    }
}
