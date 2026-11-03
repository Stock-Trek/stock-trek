use crate::{
    error::result::{StockTrekError, StockTrekResult},
    validation::Validation,
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
    /// or prices that are inconsistent with the high and low. The VWAP must
    /// also be between the low and high.
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
        if vwap < low || vwap > high {
            return Err(Validation::error(
                function,
                format!("vwap {vwap} must be between low {low} and high {high}"),
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
#[path = "market_ohlcv_tests.rs"]
mod tests;
