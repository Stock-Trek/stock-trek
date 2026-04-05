use crate::{
    aligned_window::AlignedWindow,
    dto::raw_market_candles::RawMarketCandles,
    market_data::{extract::dec_to_f64, market_candle::MarketCandle, market_ohlcv::MarketOhlcv},
};
use rust_decimal::Decimal;
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct MarketAlignedWindow {
    candles: HashMap<AlignedWindow, Vec<MarketCandle>>,
    opens: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    highs: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    lows: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    closes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    volumes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    quote_volumes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    vwaps: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
}

impl MarketAlignedWindow {
    pub fn exact(&self) -> &HashMap<AlignedWindow, Vec<MarketCandle>> {
        &self.candles
    }
    pub fn opens(&self, window: AlignedWindow) -> &Vec<f64> {
        self.opens
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.exact_open()))
    }
    pub fn highs(&self, window: AlignedWindow) -> &Vec<f64> {
        self.highs
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.exact_high()))
    }
    pub fn lows(&self, window: AlignedWindow) -> &Vec<f64> {
        self.lows
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.exact_low()))
    }
    pub fn closes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.closes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.exact_close()))
    }
    pub fn volumes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.volumes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.exact_volume()))
    }
    pub fn quote_volumes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.quote_volumes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.exact_quote_volume()))
    }
    pub fn vwaps(&self, window: AlignedWindow) -> &Vec<f64> {
        self.vwaps
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.exact_vwap()))
    }

    fn aligned_values(
        &self,
        window: AlignedWindow,
        to_value: impl Fn(&MarketOhlcv) -> Decimal,
    ) -> Vec<f64> {
        self.candles
            .get(&window)
            .unwrap()
            .iter()
            .map(|candle| dec_to_f64(to_value(candle.ohlcv())))
            .collect()
    }
}

fn windows_map() -> HashMap<AlignedWindow, OnceLock<Vec<f64>>> {
    AlignedWindow::iter()
        .map(|window| (window, OnceLock::new()))
        .collect()
}

impl From<HashMap<AlignedWindow, RawMarketCandles>> for MarketAlignedWindow {
    fn from(value: HashMap<AlignedWindow, RawMarketCandles>) -> Self {
        let candles = value
            .into_iter()
            .map(|(window, raw_candles)| {
                let candles = raw_candles
                    .candles
                    .into_iter()
                    .map(MarketCandle::from)
                    .collect();
                (window, candles)
            })
            .collect();
        MarketAlignedWindow {
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
}
