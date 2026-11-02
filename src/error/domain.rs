use std::fmt;

/// An input was outside the domain of the operation.
#[derive(Debug, Clone)]
pub struct DomainError {
    /// The function that produced the error.
    pub function: &'static str,
    /// Details about the failed operation.
    pub message: String,
}

impl DomainError {
    pub(crate) fn new(function: &'static str, message: impl Into<String>) -> Self {
        Self {
            function,
            message: message.into(),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Domain error in '{}': {}", self.function, self.message)
    }
}

impl std::error::Error for DomainError {}
