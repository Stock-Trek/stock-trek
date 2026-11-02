use serde::{Deserialize, Serialize};

/// Open, high, low, close, and volume values for a candle.
///
/// `volume` is denominated in the base asset, while `quote_volume` is
/// denominated in the quote asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub fn new(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        quote_volume: f64,
        vwap: f64,
    ) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            quote_volume,
            vwap,
        }
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
