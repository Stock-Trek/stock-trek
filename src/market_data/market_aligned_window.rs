use crate::{
    error::result::StockTrekResult,
    market_data::{
        aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
    },
    validation::Validation,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;

/// Candles grouped by aligned windows.
///
/// Candles within a window are kept in chronological order and exposed either
/// as whole candles or as per-field series such as `opens` and `closes`, in
/// the order needed for indicator calculations.
#[derive(Debug, Clone, Serialize)]
pub struct MarketAlignedWindow {
    candles: BTreeMap<AlignedWindow, Vec<MarketCandle>>,
}

impl MarketAlignedWindow {
    /// Creates an aligned-window collection from candles grouped by window.
    ///
    /// Candles are sorted chronologically by start time. Returns an error when
    /// the same window appears more than once.
    pub fn new(
        candles: impl IntoIterator<Item = (AlignedWindow, Vec<MarketCandle>)>,
    ) -> StockTrekResult<Self> {
        let mut candles_by_window: BTreeMap<AlignedWindow, Vec<MarketCandle>> = BTreeMap::new();
        for (window, window_candles) in candles {
            if candles_by_window.insert(window, window_candles).is_some() {
                return Err(Validation::error(
                    "MarketAlignedWindow::new",
                    format!("duplicate candles for aligned window {window:?}"),
                ));
            }
        }
        Ok(Self::from_map(candles_by_window))
    }

    /// Creates an aligned-window collection from candles already grouped by
    /// window, sorting each window's candles chronologically.
    pub(super) fn from_map(
        mut candles_by_window: BTreeMap<AlignedWindow, Vec<MarketCandle>>,
    ) -> Self {
        for window_candles in candles_by_window.values_mut() {
            window_candles.sort_by_key(|candle| {
                (candle.start_time_millis_inc(), candle.end_time_millis_exc())
            });
        }
        Self {
            candles: candles_by_window,
        }
    }
    /// Returns all candles grouped by aligned window, ordered by window.
    pub fn candles(&self) -> &BTreeMap<AlignedWindow, Vec<MarketCandle>> {
        &self.candles
    }
    /// Returns the candles for an aligned window, ordered chronologically.
    pub fn candles_for(&self, window: AlignedWindow) -> Option<&[MarketCandle]> {
        self.candles.get(&window).map(Vec::as_slice)
    }
    /// The opening price of each candle in the window, chronologically.
    pub fn opens(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::open)
    }
    /// The highest price of each candle in the window, chronologically.
    pub fn highs(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::high)
    }
    /// The lowest price of each candle in the window, chronologically.
    pub fn lows(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::low)
    }
    /// The closing price of each candle in the window, chronologically.
    pub fn closes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::close)
    }
    /// The base-asset volume of each candle in the window, chronologically.
    pub fn volumes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::volume)
    }
    /// The quote-asset volume of each candle in the window, chronologically.
    pub fn quote_volumes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::quote_volume)
    }
    /// The volume-weighted average price of each candle in the window,
    /// chronologically.
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
            candles: BTreeMap<AlignedWindow, Vec<MarketCandle>>,
        }

        let data = MarketAlignedWindowData::deserialize(deserializer)?;
        Self::new(data.candles).map_err(serde::de::Error::custom)
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
            true,
            MarketOhlcv::new(
                open,
                open + 1.0,
                open - 0.5,
                open + 0.5,
                10.0,
                15.0,
                open + 0.4,
            )
            .unwrap(),
            3,
        )
        .unwrap()
    }

    #[test]
    fn new_rejects_duplicate_windows() {
        let market = MarketAlignedWindow::new([
            (AlignedWindow::Minutes1, vec![candle_at(2, 2.0)]),
            (AlignedWindow::Minutes1, vec![candle_at(1, 1.0)]),
        ]);

        assert!(market.is_err());
    }

    #[test]
    fn new_sorts_candles_chronologically() {
        let market = MarketAlignedWindow::new(BTreeMap::from([(
            AlignedWindow::Minutes1,
            vec![candle_at(2, 2.0), candle_at(1, 1.0)],
        )]))
        .unwrap();

        assert_eq!(market.opens(AlignedWindow::Minutes1), Some(vec![1.0, 2.0]));
    }

    #[test]
    fn serde_json_round_trip() {
        let market =
            MarketAlignedWindow::new(BTreeMap::from([(AlignedWindow::Minutes1, vec![candle()])]))
                .unwrap();
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

    #[test]
    fn candles_are_ordered_by_window_regardless_of_insertion_order() {
        let market = MarketAlignedWindow::new(std::collections::HashMap::from([
            (AlignedWindow::Hours1, vec![candle()]),
            (AlignedWindow::Seconds1, vec![candle()]),
            (AlignedWindow::Minutes1, vec![candle()]),
        ]))
        .unwrap();

        let windows: Vec<_> = market.candles().keys().copied().collect();

        assert_eq!(
            windows,
            vec![
                AlignedWindow::Seconds1,
                AlignedWindow::Minutes1,
                AlignedWindow::Hours1,
            ]
        );
    }
}
