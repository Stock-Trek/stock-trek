use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum ValueError {
    NotFound { name: String, key: String },
    IncorrectType { expected: String, found: String },
    ValuesNotEqual { a: String, b: String },
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
