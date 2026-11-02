//! Shared implementation for string-backed DEX identifiers.

/// Implements the shared behaviour for a validated, string-backed identifier.
///
/// This macro only defines implementations; the identifier type must be declared
/// separately as a tuple struct backed by `Cow<'static, str>`.
///
/// Identifiers are trimmed of leading and trailing whitespace when constructed.
macro_rules! string_id_funcs {
    ($name:ident) => {
        impl $name {
            fn from_string(value: String) -> Self {
                Self(std::borrow::Cow::Owned(value.trim().to_string()))
            }

            /// Returns the identifier as a string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the identifier and returns its owned string value.
            pub fn into_inner(self) -> String {
                self.0.into_owned()
            }

            fn validate(value: &str) -> crate::error::stock_trek_error::StockTrekResult<()> {
                if value.trim().is_empty() {
                    return Err(crate::error::stock_trek_error::StockTrekError::Dex(
                        crate::error::dex::DexError::InvalidIdentifier {
                            identifier_type: stringify!($name),
                            value: value.into(),
                        },
                    ));
                }
                Ok(())
            }

            /// Creates an identifier from a string.
            ///
            /// Leading and trailing whitespace is trimmed.
            ///
            /// # Errors
            ///
            /// Returns an error if the value is empty or whitespace-only.
            pub fn new(
                id: impl Into<String>,
            ) -> crate::error::stock_trek_error::StockTrekResult<Self> {
                Self::try_from(id.into())
            }
        }

        impl TryFrom<&str> for $name {
            type Error = crate::error::stock_trek_error::StockTrekError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::validate(value)?;
                Ok(Self::from_string(value.to_string()))
            }
        }

        impl TryFrom<String> for $name {
            type Error = crate::error::stock_trek_error::StockTrekError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::validate(&value)?;
                Ok(Self::from_string(value))
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::error::stock_trek_error::StockTrekError;

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
