use crate::market_data::{market_candle::MarketCandle, rolling_window::RollingWindow};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

pub type Ohlcv = (f64, f64, f64, f64, f64, f64, f64);

#[derive(Debug, Serialize)]
pub struct MarketRollingWindow {
    pub candles: HashMap<RollingWindow, MarketCandle>,
    #[serde(skip)]
    ohlcv: OnceLock<HashMap<RollingWindow, OnceLock<Ohlcv>>>,
}

impl<'de> Deserialize<'de> for MarketRollingWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            candles: HashMap<RollingWindow, MarketCandle>,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(MarketRollingWindow::new(helper.candles))
    }
}

impl MarketRollingWindow {
    pub fn new(candles: HashMap<RollingWindow, MarketCandle>) -> Self {
        Self {
            candles,
            ohlcv: OnceLock::new(),
        }
    }
    pub fn ohlcv(&self, window: RollingWindow) -> &Ohlcv {
        self.ohlcv
            .get_or_init(|| self.new_ohlcv_map())
            .get(&window)
            .unwrap()
            .get_or_init(|| self.ohlcv_values(window))
    }

    fn ohlcv_values(&self, window: RollingWindow) -> Ohlcv {
        let ohlcv = &self.candles.get(&window).unwrap().ohlcv;
        (
            ohlcv.open,
            ohlcv.high,
            ohlcv.low,
            ohlcv.close,
            ohlcv.volume,
            ohlcv.quote_volume,
            ohlcv.vwap,
        )
    }
    fn new_ohlcv_map(&self) -> HashMap<RollingWindow, OnceLock<Ohlcv>> {
        RollingWindow::iter()
            .map(|window| (window, OnceLock::new()))
            .collect()
    }
}
