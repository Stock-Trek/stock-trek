use std::fmt;

/// Errors produced by numeric and statistical operations.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StatsError {
    /// A division by zero was attempted.
    DivisionByZero {
        /// The function that produced the error.
        function: &'static str,
        /// Details about the failed operation.
        detail: String,
    },
    /// An input was outside the domain of the operation.
    DomainError {
        /// The function that produced the error.
        function: &'static str,
        /// Details about the failed operation.
        message: String,
    },
    /// Two values could not be compared.
    IncomparableValues {
        /// The left-hand value.
        left: f64,
        /// The right-hand value.
        right: f64,
    },
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatsError::DivisionByZero { function, detail } => {
                write!(f, "Division by zero in '{}': {}", function, detail)
            }
            StatsError::DomainError { function, message } => {
                write!(f, "Domain error in '{}': {}", function, message)
            }
            StatsError::IncomparableValues { left, right } => {
                write!(f, "Incomparable values {} and {} (NaN?)", left, right)
            }
        }
    }
}

impl std::error::Error for StatsError {}
