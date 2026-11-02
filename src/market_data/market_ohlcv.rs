use crate::{
    error::result::{StockTrekError, StockTrekResult},
    market_data::validation::Validation,
};
use serde::{Deserialize, Serialize};

/// Open, high, low, close, and volume values for a candle.
///
/// `volume` is denominated in the base asset, while `quote_volume` is
/// denominated in the quote asset. Values are validated when the candle data
/// is created or deserialized.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "MarketOhlcvData")]
pub struct MarketOhlcv {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    quote_volume: f64,
    vwap: f64,
}

impl MarketOhlcv {
    /// Creates OHLCV values from their individual components.
    ///
    /// Returns an error for non-finite or negative values, negative volumes,
    /// or prices that are inconsistent with the high and low.
    pub fn new(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        quote_volume: f64,
        vwap: f64,
    ) -> StockTrekResult<Self> {
        Self::from_parts(
            "MarketOhlcv::new",
            open,
            high,
            low,
            close,
            volume,
            quote_volume,
            vwap,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        function: &'static str,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        quote_volume: f64,
        vwap: f64,
    ) -> StockTrekResult<Self> {
        for (field, value) in [
            ("open", open),
            ("high", high),
            ("low", low),
            ("close", close),
            ("vwap", vwap),
        ] {
            Validation::non_negative(function, field, value)?;
        }
        Validation::non_negative(function, "volume", volume)?;
        Validation::non_negative(function, "quote volume", quote_volume)?;

        if high < low {
            return Err(Validation::error(
                function,
                format!("high {high} must not be below low {low}"),
            ));
        }
        if open < low || open > high {
            return Err(Validation::error(
                function,
                format!("open {open} must be between low {low} and high {high}"),
            ));
        }
        if close < low || close > high {
            return Err(Validation::error(
                function,
                format!("close {close} must be between low {low} and high {high}"),
            ));
        }

        Ok(Self {
            open,
            high,
            low,
            close,
            volume,
            quote_volume,
            vwap,
        })
    }

    /// Returns the opening price.
    pub fn open(&self) -> f64 {
        self.open
    }

    /// Returns the highest price.
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Returns the lowest price.
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Returns the closing price.
    pub fn close(&self) -> f64 {
        self.close
    }

    /// Returns the base-asset volume.
    pub fn volume(&self) -> f64 {
        self.volume
    }

    /// Returns the quote-asset volume.
    pub fn quote_volume(&self) -> f64 {
        self.quote_volume
    }

    /// Returns the volume-weighted average price.
    pub fn vwap(&self) -> f64 {
        self.vwap
    }
}

#[derive(Deserialize)]
struct MarketOhlcvData {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    quote_volume: f64,
    vwap: f64,
}

impl TryFrom<MarketOhlcvData> for MarketOhlcv {
    type Error = StockTrekError;

    fn try_from(data: MarketOhlcvData) -> Result<Self, Self::Error> {
        Self::new(
            data.open,
            data.high,
            data.low,
            data.close,
            data.volume,
            data.quote_volume,
            data.vwap,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_consistent_ohlcv_values() {
        let ohlcv = MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap();

        assert_eq!(ohlcv.open(), 1.0);
        assert_eq!(ohlcv.high(), 2.0);
        assert_eq!(ohlcv.low(), 0.5);
        assert_eq!(ohlcv.close(), 1.5);
        assert_eq!(ohlcv.volume(), 10.0);
        assert_eq!(ohlcv.quote_volume(), 15.0);
        assert_eq!(ohlcv.vwap(), 1.4);
    }

    #[test]
    fn rejects_non_finite_values() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(MarketOhlcv::new(value, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).is_err());
            assert!(MarketOhlcv::new(1.0, value, 0.5, 1.5, 10.0, 15.0, 1.4).is_err());
            assert!(MarketOhlcv::new(1.0, 2.0, value, 1.5, 10.0, 15.0, 1.4).is_err());
            assert!(MarketOhlcv::new(1.0, 2.0, 0.5, value, 10.0, 15.0, 1.4).is_err());
            assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, value, 15.0, 1.4).is_err());
            assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, value, 1.4).is_err());
            assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, value).is_err());
        }
    }

    #[test]
    fn rejects_negative_values() {
        assert!(MarketOhlcv::new(-1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(1.0, -2.0, -3.0, 1.5, 10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, -0.5, 1.5, 10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, -1.5, 10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, -10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, -15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, -1.4).is_err());
    }

    #[test]
    fn rejects_inconsistent_highs_and_lows() {
        assert!(MarketOhlcv::new(1.0, 0.5, 2.0, 1.5, 10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(3.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 3.0, 10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(0.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).is_err());
        assert!(MarketOhlcv::new(1.0, 2.0, 0.5, 0.0, 10.0, 15.0, 1.4).is_err());
    }

    #[test]
    fn serde_json_round_trip_preserves_the_values() {
        let ohlcv = MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap();
        let json = serde_json::to_string(&ohlcv).unwrap();
        let deserialized: MarketOhlcv = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, ohlcv);
    }

    #[test]
    fn deserialization_rejects_invalid_values() {
        let json = serde_json::json!({
            "open": 1.0,
            "high": -1.0,
            "low": 2.0,
            "close": 1.5,
            "volume": -10.0,
            "quote_volume": -15.0,
            "vwap": 1.4,
        });

        assert!(serde_json::from_value::<MarketOhlcv>(json).is_err());
    }
}
