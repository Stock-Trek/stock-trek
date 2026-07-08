use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum StatsError {
    DivisionByZero {
        function: &'static str,
        detail: String,
    },
    EmptyInput {
        function: &'static str,
    },
    MismatchedLengths {
        function: &'static str,
        first_len: usize,
        second_len: usize,
    },
    InsufficientDegreesOfFreedom {
        function: &'static str,
        dof: usize,
        needed: usize,
    },
    DomainError {
        function: &'static str,
        message: String,
    },
    InvalidLag {
        function: &'static str,
        lag: usize,
        max_lag: usize,
    },
    ZeroVariance {
        function: &'static str,
        detail: String,
    },
    IncomparableValues {
        left: f64,
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
