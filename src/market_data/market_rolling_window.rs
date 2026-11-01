use crate::market_data::{market_candle::MarketCandle, rolling_window::RollingWindow};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

/// An OHLCV tuple: open, high, low, close, volume, quote volume, and VWAP.
pub type Ohlcv = (f64, f64, f64, f64, f64, f64, f64);

/// Candles for fixed-length rolling windows.
#[derive(Debug, Clone, Serialize)]
pub struct MarketRollingWindow {
    /// The candle for each rolling window.
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
    /// Creates a rolling-window collection from candles grouped by window.
    pub fn new(candles: HashMap<RollingWindow, MarketCandle>) -> Self {
        Self {
            candles,
            ohlcv: OnceLock::new(),
        }
    }
    /// Returns the OHLCV values for a rolling window, if present.
    pub fn ohlcv(&self, window: RollingWindow) -> Option<&Ohlcv> {
        let candle = self.candles.get(&window)?;
        let ohlcv = self
            .ohlcv
            .get_or_init(|| self.new_ohlcv_map())
            .get(&window)?;
        Some(ohlcv.get_or_init(|| {
            let ohlcv = &candle.ohlcv;
            (
                ohlcv.open,
                ohlcv.high,
                ohlcv.low,
                ohlcv.close,
                ohlcv.volume,
                ohlcv.quote_volume,
                ohlcv.vwap,
            )
        }))
    }
    fn new_ohlcv_map(&self) -> HashMap<RollingWindow, OnceLock<Ohlcv>> {
        RollingWindow::iter()
            .map(|window| (window, OnceLock::new()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::market_ohlcv::MarketOhlcv;

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
    fn ohlcv_returns_none_when_window_is_absent() {
        let market = MarketRollingWindow::new(HashMap::new());

        assert!(market.ohlcv(RollingWindow::Minutes1).is_none());
    }

    #[test]
    fn ohlcv_returns_values_when_window_is_present() {
        let market = MarketRollingWindow::new(HashMap::from([(RollingWindow::Minutes1, candle())]));

        assert_eq!(
            market.ohlcv(RollingWindow::Minutes1),
            Some(&(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4))
        );
    }
}
