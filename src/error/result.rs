use crate::error::{
    dex::DexError, market_data::MarketDataError, stats::StatsError, value::ValueError,
};
use std::fmt;

/// Result type used by stock-trek operations.
pub type StockTrekResult<T> = Result<T, StockTrekError>;

/// Errors returned by stock-trek operations.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StockTrekError {
    /// A DEX error.
    Dex(DexError),
    /// A market-data error.
    MarketData(MarketDataError),
    /// A statistics or numeric error.
    Stats(StatsError),
    /// A value resolution error.
    Value(ValueError),
}

impl fmt::Display for StockTrekError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StockTrekError::Dex(e) => write!(f, "DEX error: {}", e),
            StockTrekError::MarketData(e) => write!(f, "Market data error: {}", e),
            StockTrekError::Stats(e) => write!(f, "Stats error: {}", e),
            StockTrekError::Value(e) => write!(f, "Value error: {}", e),
        }
    }
}

impl std::error::Error for StockTrekError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StockTrekError::Dex(e) => Some(e),
            StockTrekError::MarketData(e) => Some(e),
            StockTrekError::Stats(e) => Some(e),
            StockTrekError::Value(e) => Some(e),
        }
    }
}

impl From<DexError> for StockTrekError {
    fn from(value: DexError) -> Self {
        Self::Dex(value)
    }
}

impl From<MarketDataError> for StockTrekError {
    fn from(value: MarketDataError) -> Self {
        Self::MarketData(value)
    }
}

impl From<StatsError> for StockTrekError {
    fn from(value: StatsError) -> Self {
        Self::Stats(value)
    }
}

impl From<ValueError> for StockTrekError {
    fn from(value: ValueError) -> Self {
        Self::Value(value)
    }
}
