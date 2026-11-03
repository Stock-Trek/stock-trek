//! Shared label and string conversion implementation for window enums.

/// Declares a window enum and implements its label, display, serialization, and
/// parsing behaviour.
///
/// The enum variants are listed with their short labels, for example
/// `Minutes5 => "5m"`. The macro generates [`Self::label`],
/// [`std::fmt::Display`], [`serde::Serialize`], [`serde::Deserialize`],
/// [`TryFrom<&str>`], and [`std::str::FromStr`].
macro_rules! window_enum {
    (
        $(#[$meta:meta])*
        $name:ident,
        $($variant:ident => $label:literal),+ $(,)?
    ) => {
        $(#[$meta])*
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialOrd,
            Ord,
            PartialEq,
            Eq,
            Hash,
            ::strum::EnumCount,
            ::strum::EnumIter,
            ::strum::FromRepr,
        )]
        #[allow(missing_docs)]
        #[non_exhaustive]
        #[repr(u8)]
        pub enum $name {
            $($variant,)+
        }

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
            type Error = $crate::error::result::StockTrekError;

            fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
                match value {
                    $($label => Ok(Self::$variant),)+
                    _ => Err($crate::error::result::StockTrekError::Domain(
                        $crate::error::domain::DomainError::new(
                            concat!(stringify!($name), "::try_from"),
                            format!("unknown window label '{value}'"),
                        ),
                    )),
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::error::result::StockTrekError;

            fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
                Self::try_from(value)
            }
        }
    };
}

pub(crate) use window_enum;
