use crate::market_data::{
    aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
};
use hashbrown::HashMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::OnceLock;
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
    pub fn opens(&self, window: AlignedWindow) -> &Vec<f64> {
        self.opens
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.open))
    }
    pub fn highs(&self, window: AlignedWindow) -> &Vec<f64> {
        self.highs
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.high))
    }
    pub fn lows(&self, window: AlignedWindow) -> &Vec<f64> {
        self.lows
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.low))
    }
    pub fn closes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.closes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.close))
    }
    pub fn volumes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.volumes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.volume))
    }
    pub fn quote_volumes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.quote_volumes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.quote_volume))
    }
    pub fn vwaps(&self, window: AlignedWindow) -> &Vec<f64> {
        self.vwaps
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.vwap))
    }

    fn aligned_values(
        &self,
        window: AlignedWindow,
        to_value: impl Fn(&MarketOhlcv) -> f64,
    ) -> Vec<f64> {
        self.candles
            .get(&window)
            .unwrap()
            .iter()
            .map(|candle| to_value(&candle.ohlcv))
            .collect()
    }
}

fn windows_map() -> HashMap<AlignedWindow, OnceLock<Vec<f64>>> {
    AlignedWindow::iter()
        .map(|window| (window, OnceLock::new()))
        .collect()
}
