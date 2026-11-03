use crate::error::{dex::DexError, domain::DomainError, stats::StatsError, value::ValueError};
use std::fmt;

/// Result type used by stock-trek operations.
pub type StockTrekResult<T> = Result<T, StockTrekError>;

/// Errors returned by stock-trek operations.
///
/// Each variant wraps the error type of one subsystem: blockchain identifier
/// and address validation ([`DexError`]), numeric domain and validation
/// failures ([`DomainError`]), arithmetic evaluation ([`StatsError`]), and
/// signal lookups and type conversions ([`ValueError`]).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StockTrekError {
    /// A blockchain address or identifier was rejected.
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
            StockTrekError::Dex(e) => Some(e),
            StockTrekError::Domain(e) => Some(e),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_variants_are_prefixed_consistently() {
        let cases = [
            (
                StockTrekError::Dex(DexError::InvalidIdentifier {
                    identifier_type: "id",
                    value: String::new(),
                }),
                "DEX error: ",
            ),
            (
                StockTrekError::Domain(DomainError::new("test::fn", "message")),
                "Domain error: ",
            ),
            (
                StockTrekError::Stats(StatsError::IncomparableValues {
                    left: f64::NAN,
                    right: 1.0,
                }),
                "Stats error: ",
            ),
            (
                StockTrekError::Value(ValueError::IncorrectType {
                    expected: "u64".into(),
                    found: "x".into(),
                }),
                "Value error: ",
            ),
        ];

        for (error, prefix) in cases {
            let display = error.to_string();
            assert!(display.starts_with(prefix), "{display:?} lacks {prefix:?}");
            assert!(display.len() > prefix.len(), "{display:?} has no details");
        }
    }

    #[test]
    fn domain_display_includes_function_and_message() {
        let display = StockTrekError::Domain(DomainError::new("test::fn", "message")).to_string();

        assert_eq!(display, "Domain error: In 'test::fn': message");
    }
}
