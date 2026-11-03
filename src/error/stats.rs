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
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatsError::DivisionByZero { function, detail } => {
                write!(f, "Division by zero in '{}': {}", function, detail)
            }
        }
    }
}

impl std::error::Error for StatsError {}
