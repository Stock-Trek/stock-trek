use std::fmt;

/// Errors produced while resolving values.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ValueError {
    /// A requested value was not found.
    NotFound {
        /// The kind of value that was requested.
        name: String,
        /// The key that was used to look up the value.
        key: String,
    },
    /// A value had an unexpected type.
    IncorrectType {
        /// The expected type.
        expected: String,
        /// The type that was found.
        found: String,
    },
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueError::NotFound { name, key } => {
                write!(f, "{} not found for '{}'", name, key)
            }
            ValueError::IncorrectType { expected, found } => {
                write!(f, "Expected type '{}' but found '{}'", expected, found)
            }
        }
    }
}

impl std::error::Error for ValueError {}
