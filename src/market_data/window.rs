//! Shared label, duration, and string conversion implementation for window enums.

/// The length of a window in milliseconds.
///
/// Fixed-length windows return [`WindowMillis::Exact`]. Calendar-based windows
/// vary in length and only guarantee an upper bound, so they return
/// [`WindowMillis::Upper`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowMillis {
    /// The exact window length in milliseconds.
    Exact(u64),
    /// An upper bound on the window length in milliseconds.
    Upper(u64),
}

impl WindowMillis {
    /// Returns the exact window length in milliseconds, or `None` when only an
    /// upper bound is known.
    pub fn exact(self) -> Option<u64> {
        match self {
            Self::Exact(millis) => Some(millis),
            Self::Upper(_) => None,
        }
    }

    /// Returns an upper bound on the window length in milliseconds.
    ///
    /// Exact lengths are also upper bounds.
    pub fn upper_bound(self) -> u64 {
        match self {
            Self::Exact(millis) | Self::Upper(millis) => millis,
        }
    }
}

/// Declares a window enum and implements its label, duration, display,
/// serialization, and parsing behaviour.
///
/// The enum variants are listed with their short labels and lengths in
/// milliseconds, for example `Minutes5 => "5m" => 5 * MILLIS_PER_MINUTE`.
/// Variants with a fixed length are listed under `exact`, while calendar-based
/// variants with only an upper bound are listed under `upper`.
///
/// The macro generates [`Self::label`], [`std::fmt::Display`],
/// [`serde::Serialize`], [`serde::Deserialize`], [`TryFrom<&str>`],
/// [`std::str::FromStr`], and [`strum::VariantArray`]. It also generates
/// [`Self::millis`] returning the window length as a [`WindowMillis`], and
/// [`Self::millis_exact`] for enums whose variants all have a fixed length.
///
/// ```ignore
/// window_enum!(
///     RollingWindow,
///     exact {
///         Minutes5 => "5m" => 5 * MILLIS_PER_MINUTE,
///         Hours1 => "1h" => MILLIS_PER_HOUR,
///     }
/// );
///
/// window_enum!(
///     AlignedWindow,
///     exact {
///         Minutes5 => "5m" => 5 * MILLIS_PER_MINUTE,
///     }
///     upper {
///         Months1 => "1mo" => MILLIS_PER_MONTH_UPPER_BOUND,
///     }
/// );
/// ```
macro_rules! window_enum {
    (
        @define
        $(#[$meta:meta])*
        $name:ident,
        [
            $($variant:ident => $label:literal,)+
        ],
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
            ::strum::VariantArray,
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

    (
        $(#[$meta:meta])*
        $name:ident,
        exact {
            $($variant:ident => $label:literal => $millis:expr),+ $(,)?
        }
    ) => {
        window_enum!(
            @define
            $(#[$meta])*
            $name,
            [
                $($variant => $label,)+
            ],
        );

        impl $name {
            /// Returns the window length in milliseconds as a
            /// [`WindowMillis`](crate::market_data::WindowMillis).
            ///
            /// Every variant of this enum has a fixed length, so the result is
            /// always `WindowMillis::Exact`. Use [`Self::millis_exact`] when a
            /// plain `u64` is preferred.
            pub fn millis(&self) -> $crate::market_data::WindowMillis {
                match self {
                    $(Self::$variant => $crate::market_data::WindowMillis::Exact($millis),)+
                }
            }

            /// Returns the exact window length in milliseconds.
            pub fn millis_exact(&self) -> u64 {
                match self {
                    $(Self::$variant => $millis,)+
                }
            }
        }
    };

    (
        $(#[$meta:meta])*
        $name:ident,
        exact {
            $($variant:ident => $label:literal => $millis:expr),+ $(,)?
        }
        upper {
            $($upper_variant:ident => $upper_label:literal => $upper_millis:expr),+ $(,)?
        }
    ) => {
        window_enum!(
            @define
            $(#[$meta])*
            $name,
            [
                $($variant => $label,)+
                $($upper_variant => $upper_label,)+
            ],
        );

        impl $name {
            /// Returns the window length in milliseconds as a
            /// [`WindowMillis`](crate::market_data::WindowMillis).
            ///
            /// Fixed-length windows return `WindowMillis::Exact`. Calendar
            /// windows have no fixed length, so they return
            /// `WindowMillis::Upper` with an upper bound based on the longest
            /// possible month or year.
            pub fn millis(&self) -> $crate::market_data::WindowMillis {
                match self {
                    $(Self::$variant => $crate::market_data::WindowMillis::Exact($millis),)+
                    $(Self::$upper_variant => $crate::market_data::WindowMillis::Upper($upper_millis),)+
                }
            }
        }
    };
}

pub(crate) use window_enum;
