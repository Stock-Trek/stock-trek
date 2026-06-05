use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum StatsError {
    // Original StatsError variants
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
    InvalidParameters {
        function: &'static str,
        message: String,
    },
    ZeroVariance {
        function: &'static str,
        detail: String,
    },
    // Moved from GeneralError (calculation errors)
    ComplexPowerResult {
        operator: &'static str,
        base: f64,
        exponent: f64,
    },
    UndefinedLogarithm {
        operator: &'static str,
        detail: String,
    },
    ComplexLogarithm {
        operator: &'static str,
        detail: String,
    },
    EdgeCase {
        operator: &'static str,
        detail: String,
    },
    ComplexResult {
        operator: &'static str,
        value: f64,
        relation: &'static str,
    },
    IncomparableValues {
        left: f64,
        right: f64,
    },
    // Moved from GeneralError: calculation domain error with value bounds
    CalculationDomainError {
        operator: &'static str,
        value: f64,
        domain_min: f64,
        domain_max: f64,
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
                    "Mismatched lengths in '{}': first series has {} elements, second has {}",
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
                    "Invalid lag in '{}': lag={} is not in [0, {})",
                    function, lag, max_lag
                )
            }
            StatsError::InvalidParameters { function, message } => {
                write!(f, "Invalid parameters in '{}': {}", function, message)
            }
            StatsError::ZeroVariance { function, detail } => {
                write!(f, "Zero variance in '{}': {}", function, detail)
            }
            StatsError::ComplexPowerResult {
                operator,
                base,
                exponent,
            } => {
                write!(
                    f,
                    "Complex result in '{}': combining base {} with exponent {} would produce a complex number",
                    operator, base, exponent
                )
            }
            StatsError::UndefinedLogarithm { operator, detail } => {
                write!(f, "Undefined logarithm in '{}': {}", operator, detail)
            }
            StatsError::ComplexLogarithm { operator, detail } => {
                write!(f, "Complex logarithm in '{}': {}", operator, detail)
            }
            StatsError::EdgeCase { operator, detail } => {
                write!(f, "Edge case in '{}': {}", operator, detail)
            }
            StatsError::ComplexResult {
                operator,
                value,
                relation,
            } => {
                write!(
                    f,
                    "Complex result in '{}': value {} is {} the supported domain",
                    operator, value, relation
                )
            }
            StatsError::IncomparableValues { left, right } => {
                write!(
                    f,
                    "Failed to compare {} and {}: values are incomparable (NaN?)",
                    left, right
                )
            }
            StatsError::CalculationDomainError {
                operator,
                value,
                domain_min,
                domain_max,
            } => {
                write!(
                    f,
                    "Domain error in '{}': value {} is outside the supported domain [{}, {}]",
                    operator, value, domain_min, domain_max
                )
            }
        }
    }
}

impl std::error::Error for StatsError {}
