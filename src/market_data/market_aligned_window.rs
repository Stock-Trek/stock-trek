use crate::market_data::{
    aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

/// Candles grouped by calendar-aligned windows.
#[derive(Debug, Clone, Serialize)]
pub struct MarketAlignedWindow {
    /// Candles for each aligned window.
    pub candles: HashMap<AlignedWindow, Vec<MarketCandle>>,
    #[serde(skip)]
    opens: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    #[serde(skip)]
    highs: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    #[serde(skip)]
    lows: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    #[serde(skip)]
    closes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    #[serde(skip)]
    volumes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    #[serde(skip)]
    quote_volumes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    #[serde(skip)]
    vwaps: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
}

impl<'de> Deserialize<'de> for MarketAlignedWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            candles: HashMap<AlignedWindow, Vec<MarketCandle>>,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(MarketAlignedWindow::new(helper.candles))
    }
}

impl MarketAlignedWindow {
    /// Creates an aligned-window collection from candles grouped by window.
    pub fn new(candles: HashMap<AlignedWindow, Vec<MarketCandle>>) -> Self {
        Self {
            candles,
            opens: OnceLock::new(),
            highs: OnceLock::new(),
            lows: OnceLock::new(),
            closes: OnceLock::new(),
            volumes: OnceLock::new(),
            quote_volumes: OnceLock::new(),
            vwaps: OnceLock::new(),
        }
    }
    /// Returns the open prices for an aligned window, if present.
    pub fn opens(&self, window: AlignedWindow) -> Option<&Vec<f64>> {
        self.values(&self.opens, window, |ohlcv| ohlcv.open)
    }
    /// Returns the high prices for an aligned window, if present.
    pub fn highs(&self, window: AlignedWindow) -> Option<&Vec<f64>> {
        self.values(&self.highs, window, |ohlcv| ohlcv.high)
    }
    /// Returns the low prices for an aligned window, if present.
    pub fn lows(&self, window: AlignedWindow) -> Option<&Vec<f64>> {
        self.values(&self.lows, window, |ohlcv| ohlcv.low)
    }
    /// Returns the close prices for an aligned window, if present.
    pub fn closes(&self, window: AlignedWindow) -> Option<&Vec<f64>> {
        self.values(&self.closes, window, |ohlcv| ohlcv.close)
    }
    /// Returns the base volumes for an aligned window, if present.
    pub fn volumes(&self, window: AlignedWindow) -> Option<&Vec<f64>> {
        self.values(&self.volumes, window, |ohlcv| ohlcv.volume)
    }
    /// Returns the quote volumes for an aligned window, if present.
    pub fn quote_volumes(&self, window: AlignedWindow) -> Option<&Vec<f64>> {
        self.values(&self.quote_volumes, window, |ohlcv| ohlcv.quote_volume)
    }
    /// Returns the volume-weighted average prices for an aligned window, if present.
    pub fn vwaps(&self, window: AlignedWindow) -> Option<&Vec<f64>> {
        self.values(&self.vwaps, window, |ohlcv| ohlcv.vwap)
    }

    fn values<'a>(
        &self,
        values: &'a OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
        window: AlignedWindow,
        to_value: impl Fn(&MarketOhlcv) -> f64,
    ) -> Option<&'a Vec<f64>> {
        let candles = self.candles.get(&window)?;
        let values = values.get_or_init(windows_map).get(&window)?;
        Some(values.get_or_init(|| {
            candles
                .iter()
                .map(|candle| to_value(&candle.ohlcv))
                .collect()
        }))
    }
}

fn windows_map() -> HashMap<AlignedWindow, OnceLock<Vec<f64>>> {
    AlignedWindow::iter()
        .map(|window| (window, OnceLock::new()))
        .collect()
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

        assert_eq!(market.opens(AlignedWindow::Minutes1), Some(&vec![1.0]));
        assert_eq!(market.highs(AlignedWindow::Minutes1), Some(&vec![2.0]));
        assert_eq!(market.lows(AlignedWindow::Minutes1), Some(&vec![0.5]));
        assert_eq!(market.closes(AlignedWindow::Minutes1), Some(&vec![1.5]));
        assert_eq!(market.volumes(AlignedWindow::Minutes1), Some(&vec![10.0]));
        assert_eq!(
            market.quote_volumes(AlignedWindow::Minutes1),
            Some(&vec![15.0])
        );
        assert_eq!(market.vwaps(AlignedWindow::Minutes1), Some(&vec![1.4]));
    }
}
