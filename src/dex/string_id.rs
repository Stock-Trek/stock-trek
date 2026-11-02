//! Shared implementation for string-backed DEX identifiers.

/// Implements the shared behaviour for a validated, string-backed identifier.
///
/// This macro only defines implementations; the identifier type must be declared
/// separately. By default the type is expected to be a tuple struct backed by
/// `Cow<'static, str>`. Pass `custom` for a type with a different representation
/// that provides its own `from_string`, `as_str`, and `into_inner` methods.
macro_rules! string_id_funcs {
    ($name:ident) => {
        impl $name {
            fn from_string(value: String) -> Self {
                Self(std::borrow::Cow::Owned(value))
            }

            /// Returns the identifier as a string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the identifier and returns its owned string value.
            pub fn into_inner(self) -> String {
                self.0.into_owned()
            }
        }

        string_id_funcs!(@shared $name);
    };
    ($name:ident, custom) => {
        string_id_funcs!(@shared $name);
    };
    (@shared $name:ident) => {
        impl $name {
            fn validate(value: &str) -> crate::error::result::StockTrekResult<()> {
                if value.trim().is_empty() {
                    return Err(crate::error::result::StockTrekError::Value(
                        crate::error::value::ValueError::IncorrectType {
                            expected: stringify!($name).into(),
                            found: value.into(),
                        },
                    ));
                }
                Ok(())
            }

            /// Creates an identifier from a string.
            ///
            /// # Errors
            ///
            /// Returns an error if the value is empty or whitespace-only.
            pub fn new(id: &str) -> crate::error::result::StockTrekResult<Self> {
                Self::try_from(id)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = crate::error::result::StockTrekError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::validate(value)?;
                Ok(Self::from_string(value.to_string()))
            }
        }

        impl TryFrom<String> for $name {
            type Error = crate::error::result::StockTrekError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::validate(&value)?;
                Ok(Self::from_string(value))
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::error::result::StockTrekError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::try_from(value)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.into_inner()
            }
        }
    };
}

pub(crate) use string_id_funcs;
