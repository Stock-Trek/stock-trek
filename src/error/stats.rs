use std::fmt;

/// Errors produced by numeric and statistical operations.
#[derive(Debug, Clone)]
pub enum StatsError {
    /// A division by zero was attempted.
    DivisionByZero {
        /// The function that produced the error.
        function: &'static str,
        /// Details about the failed operation.
        detail: String,
    },
    /// An operation received empty input.
    EmptyInput {
        /// The function that produced the error.
        function: &'static str,
    },
    /// Two inputs had different lengths.
    MismatchedLengths {
        /// The function that produced the error.
        function: &'static str,
        /// The length of the first input.
        first_len: usize,
        /// The length of the second input.
        second_len: usize,
    },
    /// An operation did not have enough degrees of freedom.
    InsufficientDegreesOfFreedom {
        /// The function that produced the error.
        function: &'static str,
        /// The available degrees of freedom.
        dof: usize,
        /// The minimum required degrees of freedom.
        needed: usize,
    },
    /// An input was outside the domain of the operation.
    DomainError {
        /// The function that produced the error.
        function: &'static str,
        /// Details about the failed operation.
        message: String,
    },
    /// A lag was outside the supported range.
    InvalidLag {
        /// The function that produced the error.
        function: &'static str,
        /// The supplied lag.
        lag: usize,
        /// The maximum supported lag.
        max_lag: usize,
    },
    /// An operation encountered zero variance.
    ZeroVariance {
        /// The function that produced the error.
        function: &'static str,
        /// Details about the failed operation.
        detail: String,
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
            StatsError::EmptyInput { function } => {
                write!(
                    f,
                    "Empty input in '{}': expected at least one value",
                    function
                )
            }
            StatsError::MismatchedLengths {
                function,
                first_len,
                second_len,
            } => {
                write!(
                    f,
                    "Mismatched lengths in '{}': first has {} elements, second has {}",
                    function, first_len, second_len
                )
            }
            StatsError::InsufficientDegreesOfFreedom {
                function,
                dof,
                needed,
            } => {
                write!(
                    f,
                    "Insufficient degrees of freedom in '{}': got {}, need at least {}",
                    function, dof, needed
                )
            }
            StatsError::DomainError { function, message } => {
                write!(f, "Domain error in '{}': {}", function, message)
            }
            StatsError::InvalidLag {
                function,
                lag,
                max_lag,
            } => {
                write!(
                    f,
                    "Invalid lag in '{}': lag={} not in [0, {})",
                    function, lag, max_lag
                )
            }
            StatsError::ZeroVariance { function, detail } => {
                write!(f, "Zero variance in '{}': {}", function, detail)
            }
            StatsError::IncomparableValues { left, right } => {
                write!(f, "Incomparable values {} and {} (NaN?)", left, right)
            }
        }
    }
}

impl std::error::Error for StatsError {}
