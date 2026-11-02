use serde::{Deserialize, Serialize};

/// Open, high, low, close, and volume values for a candle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketOhlcv {
    /// The opening price.
    pub open: f64,
    /// The highest price.
    pub high: f64,
    /// The lowest price.
    pub low: f64,
    /// The closing price.
    pub close: f64,
    /// The base-asset volume.
    pub volume: f64,
    /// The quote-asset volume.
    pub quote_volume: f64,
    /// The volume-weighted average price.
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
