//! Shared label and string conversion implementation for window enums.

/// Implements the label, display, serialization, and parsing behaviour shared
/// by the window enums.
///
/// The type must be an enum whose variants are listed with their short labels,
/// for example `Minutes5 => "5m"`. The macro generates [`Self::label`],
/// [`std::fmt::Display`], [`serde::Serialize`], [`serde::Deserialize`],
/// [`TryFrom<&str>`], and [`std::str::FromStr`].
macro_rules! window_label_funcs {
    ($name:ident, $($variant:ident => $label:literal),+ $(,)?) => {
        impl $name {
            /// Returns the short label for the window, such as `1h` or `7d`.
            pub fn label(self) -> &'static str {
                match self {
                    $(Self::$variant => $label,)+
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.label())
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(self.label())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let label =
                    <::std::string::String as ::serde::Deserialize>::deserialize(deserializer)?;
                Self::try_from(label.as_str()).map_err(::serde::de::Error::custom)
            }
        }

        impl ::std::convert::TryFrom<&str> for $name {
            type Error = $crate::error::stock_trek_error::StockTrekError;

            fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
                match value {
                    $($label => Ok(Self::$variant),)+
                    _ => Err($crate::error::stock_trek_error::StockTrekError::Value(
                        $crate::error::value::ValueError::IncorrectType {
                            expected: stringify!($name).into(),
                            found: value.into(),
                        },
                    )),
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::error::stock_trek_error::StockTrekError;

            fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
                Self::try_from(value)
            }
        }
    };
}

pub(crate) use window_label_funcs;
