//! Shared implementation for string-backed DEX identifiers.

/// Defines a validated, string-backed identifier type.
///
/// The generated type is serialized as a plain string, rejects empty and
/// whitespace-only values, and dereferences to `str`. Additional inherent
/// items can be supplied in a trailing block.
macro_rules! define_string_id {
    ($(#[$meta:meta])* $name:ident) => {
        define_string_id!($(#[$meta])* $name {});
    };
    (
        $(#[$meta:meta])*
        $name:ident
        { $($extra:tt)* }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(std::borrow::Cow<'static, str>);

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

            /// Returns the identifier as a string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the identifier and returns its owned string value.
            pub fn into_inner(self) -> String {
                self.0.into_owned()
            }

            $($extra)*
        }

        impl TryFrom<&str> for $name {
            type Error = crate::error::result::StockTrekError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::validate(value)?;
                Ok(Self(std::borrow::Cow::Owned(value.to_string())))
            }
        }

        impl TryFrom<String> for $name {
            type Error = crate::error::result::StockTrekError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::validate(&value)?;
                Ok(Self(std::borrow::Cow::Owned(value)))
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

pub(crate) use define_string_id;
