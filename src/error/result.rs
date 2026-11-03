#[cfg(feature = "dex")]
use crate::error::dex::DexError;
use crate::error::{domain::DomainError, stats::StatsError, value::ValueError};
use std::fmt;

/// Result type used by stock-trek operations.
pub type StockTrekResult<T> = Result<T, StockTrekError>;

/// Errors returned by stock-trek operations.
///
/// Each variant wraps the error type of one subsystem: numeric domain and
/// validation failures ([`DomainError`]), arithmetic evaluation
/// ([`StatsError`]), and signal lookups and type conversions ([`ValueError`]).
#[cfg_attr(
    feature = "dex",
    doc = "Blockchain identifier and address validation failures ([`DexError`]) are also covered."
)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StockTrekError {
    /// A blockchain address or identifier was rejected.
    #[cfg(feature = "dex")]
    Dex(DexError),
    /// An input was outside the domain of the operation.
    Domain(DomainError),
    /// A numeric calculation failed, for example a division by zero.
    Stats(StatsError),
    /// A signal value was missing or had an unexpected type.
    Value(ValueError),
}

impl fmt::Display for StockTrekError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "dex")]
            StockTrekError::Dex(e) => write!(f, "DEX error: {}", e),
            StockTrekError::Domain(e) => write!(f, "Domain error: {}", e),
            StockTrekError::Stats(e) => write!(f, "Stats error: {}", e),
            StockTrekError::Value(e) => write!(f, "Value error: {}", e),
        }
    }
}

impl std::error::Error for StockTrekError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "dex")]
            StockTrekError::Dex(e) => Some(e),
            StockTrekError::Domain(e) => Some(e),
            StockTrekError::Stats(e) => Some(e),
            StockTrekError::Value(e) => Some(e),
        }
    }
}

#[cfg(feature = "dex")]
impl From<DexError> for StockTrekError {
    fn from(value: DexError) -> Self {
        Self::Dex(value)
    }
}

impl From<DomainError> for StockTrekError {
    fn from(value: DomainError) -> Self {
        Self::Domain(value)
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
