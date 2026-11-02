use rust_decimal::Decimal;
use std::fmt;

/// Errors produced while converting values at the Decimal/f64 boundary.
#[derive(Debug, Clone)]
pub enum NumericError {
    /// A calculated value was NaN or infinite.
    NonFinite {
        /// The kind of value being converted.
        kind: &'static str,
        /// The non-finite value.
        value: f64,
    },
    /// A value was not strictly positive.
    NotPositive {
        /// The kind of value being validated.
        kind: &'static str,
        /// The invalid value.
        value: Decimal,
    },
    /// A quantization increment was not strictly positive.
    InvalidIncrement {
        /// The kind of increment being validated.
        kind: &'static str,
        /// The invalid increment.
        value: Decimal,
    },
    /// A Decimal calculation overflowed or was otherwise not representable.
    Overflow {
        /// The operation that produced the error.
        operation: &'static str,
    },
    /// An order notional was below the market minimum.
    BelowMinimumNotional {
        /// The market's minimum notional.
        minimum: Decimal,
        /// The order's calculated notional.
        actual: Decimal,
    },
}

impl fmt::Display for NumericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumericError::NonFinite { kind, value } => {
                write!(f, "{} must be finite, but was {}", kind, value)
            }
            NumericError::NotPositive { kind, value } => {
                write!(f, "{} must be positive, but was {}", kind, value)
            }
            NumericError::InvalidIncrement { kind, value } => {
                write!(f, "{} must be positive, but was {}", kind, value)
            }
            NumericError::Overflow { operation } => {
                write!(f, "Decimal overflow while performing '{}'", operation)
            }
            NumericError::BelowMinimumNotional { minimum, actual } => write!(
                f,
                "order notional {} is below the market minimum {}",
                actual, minimum
            ),
        }
    }
}

impl std::error::Error for NumericError {}
