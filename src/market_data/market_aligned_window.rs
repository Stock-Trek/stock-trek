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
mod tests {
    use super::*;
    use crate::market_data::timestamp::TimestampMillis;

    fn candle() -> MarketCandle {
        candle_at(0, 1.0)
    }

    fn candle_at(start_time_millis_inc: u64, open: f64) -> MarketCandle {
        MarketCandle::new(
            TimestampMillis::new(start_time_millis_inc),
            TimestampMillis::new(start_time_millis_inc + 1),
            TimestampMillis::new(1),
            true,
            MarketOhlcv::new(open, open + 1.0, open - 0.5, open + 0.5, 10.0, 15.0, 1.4),
            3,
        )
        .unwrap()
    }

    #[test]
    fn new_sorts_candles_chronologically() {
        let market = MarketAlignedWindow::new(HashMap::from([(
            AlignedWindow::Minutes1,
            vec![candle_at(2, 2.0), candle_at(1, 1.0)],
        )]));

        assert_eq!(market.opens(AlignedWindow::Minutes1), Some(vec![1.0, 2.0]));
    }

    #[test]
    fn serde_json_round_trip() {
        let market =
            MarketAlignedWindow::new(HashMap::from([(AlignedWindow::Minutes1, vec![candle()])]));
        let json = serde_json::to_string(&market).unwrap();
        let deserialized: MarketAlignedWindow = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.opens(AlignedWindow::Minutes1), Some(vec![1.0]));
    }

    #[test]
    fn deserialization_sorts_candles_chronologically() {
        let json = serde_json::json!({
            "candles": {
                "1m": [
                    candle_at(2, 2.0),
                    candle_at(1, 1.0),
                ]
            }
        });
        let deserialized: MarketAlignedWindow = serde_json::from_value(json).unwrap();

        assert_eq!(
            deserialized.opens(AlignedWindow::Minutes1),
            Some(vec![1.0, 2.0])
        );
    }
}
