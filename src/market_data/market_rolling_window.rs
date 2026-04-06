use crate::{
    market_data::{extract::dec_to_f64, market_candle::MarketCandle},
    rolling_window::RollingWindow,
};
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct MarketRollingWindow {
    candles: HashMap<RollingWindow, MarketCandle>,
    ohlcv: OnceLock<HashMap<RollingWindow, OnceLock<Vec<f64>>>>,
}

impl MarketRollingWindow {
    pub fn new(candles: HashMap<RollingWindow, MarketCandle>) -> Self {
        Self {
            candles,
            ohlcv: OnceLock::new(),
        }
    }
    pub fn candles(&self) -> &HashMap<RollingWindow, MarketCandle> {
        &self.candles
    }
    pub fn ohlcv(&self, window: RollingWindow) -> &Vec<f64> {
        self.ohlcv
            .get_or_init(|| self.new_ohlcv_map())
            .get(&window)
            .unwrap()
            .get_or_init(|| self.ohlcv_values(window))
    }

    fn ohlcv_values(&self, window: RollingWindow) -> Vec<f64> {
        let ohlcv = self.candles.get(&window).unwrap().ohlcv();
        vec![
            dec_to_f64(ohlcv.exact_open()),
            dec_to_f64(ohlcv.exact_high()),
            dec_to_f64(ohlcv.exact_low()),
            dec_to_f64(ohlcv.exact_close()),
            dec_to_f64(ohlcv.exact_volume()),
            dec_to_f64(ohlcv.exact_quote_volume()),
            dec_to_f64(ohlcv.exact_vwap()),
        ]
    }
    fn new_ohlcv_map(&self) -> HashMap<RollingWindow, OnceLock<Vec<f64>>> {
        RollingWindow::iter()
            .map(|window| (window, OnceLock::new()))
            .collect()
    }
}
