use serde::{Deserialize, Serialize};
use std::{fmt, num::TryFromIntError, ops::Add, ops::Sub, str::FromStr};

/// A timestamp expressed as milliseconds since the Unix epoch.
///
/// The `+` and `-` operators saturate at the representable bounds. Use
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

impl Add for TimestampMillis {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl Sub for TimestampMillis {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_and_from_milliseconds() {
        let timestamp = TimestampMillis::new(1_700_000_000_123);

        assert_eq!(timestamp.as_millis(), 1_700_000_000_123);
        assert_eq!(u64::from(timestamp), 1_700_000_000_123);
        assert_eq!(TimestampMillis::from(1_700_000_000_123), timestamp);
        assert_eq!(
            TimestampMillis::try_from(1_700_000_000_123_i64).unwrap(),
            timestamp
        );
        assert_eq!(i64::try_from(timestamp).unwrap(), 1_700_000_000_123);
    }

    #[test]
    fn rejects_negative_timestamps() {
        assert!(TimestampMillis::try_from(-1_i64).is_err());
    }

    #[test]
    fn serde_round_trips_as_a_number() {
        let timestamp = TimestampMillis::new(1_700_000_000_123);
        let json = serde_json::to_string(&timestamp).unwrap();

        assert_eq!(json, "1700000000123");
        assert_eq!(
            serde_json::from_str::<TimestampMillis>(&json).unwrap(),
            timestamp
        );
    }

    #[test]
    fn displays_and_parses_milliseconds() {
        let timestamp = TimestampMillis::new(1_700_000_000_123);

        assert_eq!(timestamp.to_string(), "1700000000123");
        assert_eq!(
            timestamp.to_string().parse::<TimestampMillis>().unwrap(),
            timestamp
        );
    }

    #[test]
    fn supports_timestamp_arithmetic() {
        let lower = TimestampMillis::new(1_000);
        let upper = TimestampMillis::new(2_000);

        assert_eq!(upper.checked_sub(lower), Some(TimestampMillis::new(1_000)));
        assert_eq!(
            lower.checked_add(TimestampMillis::new(500)),
            Some(TimestampMillis::new(1_500))
        );
        assert_eq!(lower.checked_sub(upper), None);
    }

    #[test]
    fn checked_arithmetic_reports_overflow_and_underflow() {
        let zero = TimestampMillis::new(0);
        let one = TimestampMillis::new(1);
        let max = TimestampMillis::new(u64::MAX);

        assert_eq!(one.checked_sub(one), Some(zero));
        assert_eq!(zero.checked_sub(one), None);
        assert_eq!(max.checked_add(one), None);
    }

    #[test]
    fn saturating_arithmetic_clamps_to_bounds() {
        let zero = TimestampMillis::new(0);
        let one = TimestampMillis::new(1);
        let max = TimestampMillis::new(u64::MAX);

        assert_eq!(zero.saturating_sub(one), zero);
        assert_eq!(max.saturating_add(one), max);
        assert_eq!(zero.saturating_add(one), one);
        assert_eq!(max.saturating_sub(one), TimestampMillis::new(u64::MAX - 1));
    }

    #[test]
    fn operators_saturate_instead_of_panicking_or_wrapping() {
        let zero = TimestampMillis::new(0);
        let one = TimestampMillis::new(1);
        let max = TimestampMillis::new(u64::MAX);

        assert_eq!(zero - one, zero);
        assert_eq!(max + one, max);
    }
}
