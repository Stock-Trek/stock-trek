use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum ValueError {
    #[error("{} not found for '{}'", name, key)]
    NotFound { name: String, key: String },
    #[error("Value expects type '{}' but found type '{}'", expected, found)]
    IncorrectType { expected: String, found: String },
}
