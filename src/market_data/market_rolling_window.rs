use crate::{
    errors::{StockTrekError, StockTrekResult, ValueError},
    market_data::{market_candle::MarketCandle, rolling_window::RollingWindow},
};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

pub type Ohlcv = (f64, f64, f64, f64, f64, f64, f64);

#[derive(Debug, Clone, Serialize)]
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
    pub fn ohlcv(&self, window: RollingWindow) -> StockTrekResult<&Ohlcv> {
        let cached = self
            .ohlcv
            .get_or_init(|| self.new_ohlcv_map())
            .get(&window)
            .ok_or_else(|| Self::not_found(window))?;
        if let Some(values) = cached.get() {
            return Ok(values);
        }
        let values = self.ohlcv_values(window)?;
        let _ = cached.set(values);
        cached.get().ok_or_else(|| Self::not_found(window))
    }

    fn ohlcv_values(&self, window: RollingWindow) -> StockTrekResult<Ohlcv> {
        let ohlcv = &self
            .candles
            .get(&window)
            .ok_or_else(|| Self::not_found(window))?
            .ohlcv;
        Ok((
            ohlcv.open,
            ohlcv.high,
            ohlcv.low,
            ohlcv.close,
            ohlcv.volume,
            ohlcv.quote_volume,
            ohlcv.vwap,
        ))
    }
    fn new_ohlcv_map(&self) -> HashMap<RollingWindow, OnceLock<Ohlcv>> {
        RollingWindow::iter()
            .map(|window| (window, OnceLock::new()))
            .collect()
    }
    fn not_found(window: RollingWindow) -> StockTrekError {
        StockTrekError::Value(ValueError::NotFound {
            name: "RollingWindow".into(),
            key: window.to_string(),
        })
    }
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
    fn ohlcv_returns_values_for_present_window() {
        let window = RollingWindow::Minutes1;
        let candles = HashMap::from([(window, candle())]);
        let market = MarketRollingWindow::new(candles);

        assert_eq!(
            market.ohlcv(window).unwrap(),
            &(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.25)
        );
    }

    #[test]
    fn ohlcv_returns_error_for_absent_window() {
        let market = MarketRollingWindow::new(HashMap::from([(RollingWindow::Minutes1, candle())]));

        assert!(market.ohlcv(RollingWindow::Minutes5).is_err());
    }
}
