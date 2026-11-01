use crate::{
    errors::{StockTrekError, StockTrekResult, ValueError},
    market_data::{
        aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
    },
};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Serialize)]
pub struct MarketAlignedWindow {
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
    pub fn opens(&self, window: AlignedWindow) -> StockTrekResult<&Vec<f64>> {
        self.cached_values(&self.opens, window, |ohlcv| ohlcv.open)
    }
    pub fn highs(&self, window: AlignedWindow) -> StockTrekResult<&Vec<f64>> {
        self.cached_values(&self.highs, window, |ohlcv| ohlcv.high)
    }
    pub fn lows(&self, window: AlignedWindow) -> StockTrekResult<&Vec<f64>> {
        self.cached_values(&self.lows, window, |ohlcv| ohlcv.low)
    }
    pub fn closes(&self, window: AlignedWindow) -> StockTrekResult<&Vec<f64>> {
        self.cached_values(&self.closes, window, |ohlcv| ohlcv.close)
    }
    pub fn volumes(&self, window: AlignedWindow) -> StockTrekResult<&Vec<f64>> {
        self.cached_values(&self.volumes, window, |ohlcv| ohlcv.volume)
    }
    pub fn quote_volumes(&self, window: AlignedWindow) -> StockTrekResult<&Vec<f64>> {
        self.cached_values(&self.quote_volumes, window, |ohlcv| ohlcv.quote_volume)
    }
    pub fn vwaps(&self, window: AlignedWindow) -> StockTrekResult<&Vec<f64>> {
        self.cached_values(&self.vwaps, window, |ohlcv| ohlcv.vwap)
    }

    fn cached_values<'a>(
        &'a self,
        cache: &'a OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
        window: AlignedWindow,
        to_value: impl Fn(&MarketOhlcv) -> f64,
    ) -> StockTrekResult<&'a Vec<f64>> {
        let cached = cache
            .get_or_init(windows_map)
            .get(&window)
            .ok_or_else(|| Self::not_found(window))?;
        if let Some(values) = cached.get() {
            return Ok(values);
        }
        let values = self.aligned_values(window, to_value)?;
        let _ = cached.set(values);
        cached.get().ok_or_else(|| Self::not_found(window))
    }

    fn aligned_values(
        &self,
        window: AlignedWindow,
        to_value: impl Fn(&MarketOhlcv) -> f64,
    ) -> StockTrekResult<Vec<f64>> {
        let candles = self
            .candles
            .get(&window)
            .ok_or_else(|| Self::not_found(window))?;
        Ok(candles
            .iter()
            .map(|candle| to_value(&candle.ohlcv))
            .collect())
    }

    fn not_found(window: AlignedWindow) -> StockTrekError {
        StockTrekError::Value(ValueError::NotFound {
            name: "AlignedWindow".into(),
            key: window.to_string(),
        })
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
    use crate::market_data::{market_ohlcv::MarketOhlcv, timestamp::TimestampMillis};

    fn candle() -> MarketCandle {
        MarketCandle::new(
            0 as TimestampMillis,
            1_000 as TimestampMillis,
            1_000 as TimestampMillis,
            true,
            MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.25),
            1,
        )
    }

    #[test]
    fn values_are_returned_for_present_window() {
        let window = AlignedWindow::Minutes1;
        let market = MarketAlignedWindow::new(HashMap::from([(window, vec![candle()])]));

        assert_eq!(market.opens(window).unwrap(), &vec![1.0]);
        assert_eq!(market.highs(window).unwrap(), &vec![2.0]);
        assert_eq!(market.lows(window).unwrap(), &vec![0.5]);
        assert_eq!(market.closes(window).unwrap(), &vec![1.5]);
        assert_eq!(market.volumes(window).unwrap(), &vec![10.0]);
        assert_eq!(market.quote_volumes(window).unwrap(), &vec![15.0]);
        assert_eq!(market.vwaps(window).unwrap(), &vec![1.25]);
    }

    #[test]
    fn accessors_return_error_for_absent_window() {
        let market =
            MarketAlignedWindow::new(HashMap::from([(AlignedWindow::Minutes1, vec![candle()])]));
        let window = AlignedWindow::Minutes5;

        assert!(market.opens(window).is_err());
        assert!(market.highs(window).is_err());
        assert!(market.lows(window).is_err());
        assert!(market.closes(window).is_err());
        assert!(market.volumes(window).is_err());
        assert!(market.quote_volumes(window).is_err());
        assert!(market.vwaps(window).is_err());
    }
}
