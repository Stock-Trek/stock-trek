use crate::market_data::{
    aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// Candles grouped by aligned windows.
#[derive(Debug, Clone, Serialize)]
pub struct MarketAlignedWindow {
    candles: HashMap<AlignedWindow, Vec<MarketCandle>>,
}

#[allow(missing_docs)]
impl MarketAlignedWindow {
    /// Creates an aligned-window collection from candles grouped by window.
    ///
    /// Candles are sorted chronologically by start time.
    pub fn new(mut candles: HashMap<AlignedWindow, Vec<MarketCandle>>) -> Self {
        for window_candles in candles.values_mut() {
            window_candles.sort_by_key(|candle| {
                (candle.start_time_millis_inc(), candle.end_time_millis_exc())
            });
        }
        Self { candles }
    }
    /// Returns all candles grouped by aligned window.
    pub fn candles(&self) -> &HashMap<AlignedWindow, Vec<MarketCandle>> {
        &self.candles
    }
    /// Returns the candles for an aligned window, ordered chronologically.
    pub fn candles_for(&self, window: AlignedWindow) -> Option<&[MarketCandle]> {
        self.candles.get(&window).map(Vec::as_slice)
    }
    pub fn opens(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::open)
    }
    pub fn highs(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::high)
    }
    pub fn lows(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::low)
    }
    pub fn closes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::close)
    }
    pub fn volumes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::volume)
    }
    pub fn quote_volumes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::quote_volume)
    }
    pub fn vwaps(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::vwap)
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
                .map(|candle| to_value(candle.ohlcv()))
                .collect(),
        )
    }
}

impl<'de> Deserialize<'de> for MarketAlignedWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MarketAlignedWindowData {
            candles: HashMap<AlignedWindow, Vec<MarketCandle>>,
        }

        let data = MarketAlignedWindowData::deserialize(deserializer)?;
        Ok(Self::new(data.candles))
    }
}

#[cfg(test)]
mod tests;
