use std::fmt;

/// Errors produced while working with market data.
#[derive(Debug, Clone)]
pub enum MarketDataError {
    /// A market-data value was outside the domain of the operation.
    DomainError {
        /// The function that produced the error.
        function: &'static str,
        /// Details about the failed operation.
        message: String,
    },
}

impl fmt::Display for MarketDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketDataError::DomainError { function, message } => {
                write!(f, "Domain error in '{}': {}", function, message)
            }
        }
    }
}

impl std::error::Error for MarketDataError {}
