use serde::{Deserialize, Serialize};
use std::{fmt, num::TryFromIntError, str::FromStr};

/// A timestamp expressed as milliseconds since the Unix epoch.
///
/// Arithmetic operators are deliberately not implemented, because silently
/// saturating or wrapping at the representable bounds would hide bugs. Use
/// [`Self::checked_add`], [`Self::checked_sub`], [`Self::saturating_add`], or
/// [`Self::saturating_sub`] when the overflow behaviour should be explicit.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct TimestampMillis(u64);

impl TimestampMillis {
    /// Creates a timestamp from milliseconds since the Unix epoch.
    pub const fn new(millis: u64) -> Self {
        Self(millis)
    }

    /// Returns the timestamp as milliseconds since the Unix epoch.
    pub const fn as_millis(&self) -> u64 {
        self.0
    }

    /// Adds `rhs` milliseconds, returning `None` on overflow.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(millis) => Some(Self::new(millis)),
            None => None,
        }
    }

    /// Subtracts `rhs` milliseconds, returning `None` if the result would be
    /// negative.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(millis) => Some(Self::new(millis)),
            None => None,
        }
    }

    /// Adds `rhs` milliseconds, saturating at the maximum representable
    /// timestamp.
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::new(self.0.saturating_add(rhs.0))
    }

    /// Subtracts `rhs` milliseconds, saturating at zero if the result would be
    /// negative.
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::new(self.0.saturating_sub(rhs.0))
    }
}

impl From<u64> for TimestampMillis {
    fn from(millis: u64) -> Self {
        Self::new(millis)
    }
}

impl From<TimestampMillis> for u64 {
    fn from(timestamp: TimestampMillis) -> Self {
        timestamp.as_millis()
    }
}

impl TryFrom<i64> for TimestampMillis {
    type Error = TryFromIntError;

    fn try_from(millis: i64) -> Result<Self, Self::Error> {
        u64::try_from(millis).map(Self::new)
    }
}

impl TryFrom<TimestampMillis> for i64 {
    type Error = TryFromIntError;

    fn try_from(timestamp: TimestampMillis) -> Result<Self, Self::Error> {
        i64::try_from(timestamp.as_millis())
    }
}

impl FromStr for TimestampMillis {
    type Err = std::num::ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse::<u64>().map(Self::new)
    }
}

impl fmt::Display for TimestampMillis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests;
