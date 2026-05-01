use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum StatsError {
    #[error("")]
    DivisionByZero,
    #[error("")]
    EmptyInput,
    #[error("")]
    MismatchedLengths,
    #[error("")]
    InsufficientDegreesOfFreedom,
    #[error("")]
    DomainError { message: String },
    #[error("")]
    InvalidLag,
    #[error("")]
    InvalidParameters,
    #[error("")]
    ZeroVariance,
}
