use serde::{Deserialize, Serialize};

/// Open, high, low, close, and volume values for a candle.
///
/// `volume` is denominated in the base asset, while `quote_volume` is
/// denominated in the quote asset.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketOhlcv {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub quote_volume: f64,
    pub vwap: f64,
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
}
