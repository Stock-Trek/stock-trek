use std::fmt;

/// Errors produced while resolving values.
#[derive(Debug, Clone)]
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
    /// Two values that were expected to be equal were not.
    ValuesNotEqual {
        /// The first value.
        a: String,
        /// The second value.
        b: String,
    },
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueError::NotFound { name, key } => {
                write!(f, "{} Not found for '{}'", name, key)
            }
            ValueError::IncorrectType { expected, found } => {
                write!(f, "Expected type '{}' but found '{}'", expected, found)
            }
            ValueError::ValuesNotEqual { a, b } => {
                write!(f, "Values were '{}' and '{}' but expected equal", a, b)
            }
        }
    }
}

impl std::error::Error for ValueError {}
