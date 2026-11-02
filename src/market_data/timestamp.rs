use serde::{Deserialize, Serialize};
use std::{fmt, num::TryFromIntError, ops::Add, ops::Sub, str::FromStr};

/// A timestamp expressed as milliseconds since the Unix epoch.
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

    /// Returns the underlying milliseconds since the Unix epoch.
    pub const fn into_inner(self) -> u64 {
        self.0
    }

    /// Returns the difference between two timestamps, or `None` if `rhs` is
    /// later than `self`.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(millis) => Some(Self::new(millis)),
            None => None,
        }
    }
}

impl From<u64> for TimestampMillis {
    fn from(millis: u64) -> Self {
        Self::new(millis)
    }
}

impl From<TimestampMillis> for u64 {
    fn from(timestamp: TimestampMillis) -> Self {
        timestamp.into_inner()
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
        i64::try_from(timestamp.into_inner())
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
        Self::new(self.0 + rhs.0)
    }
}

impl Sub for TimestampMillis {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_and_from_milliseconds() {
        let timestamp = TimestampMillis::new(1_700_000_000_123);

        assert_eq!(timestamp.as_millis(), 1_700_000_000_123);
        assert_eq!(timestamp.into_inner(), 1_700_000_000_123);
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

        assert_eq!(upper - lower, TimestampMillis::new(1_000));
        assert_eq!(
            lower + TimestampMillis::new(500),
            TimestampMillis::new(1_500)
        );
        assert_eq!(upper.checked_sub(lower), Some(TimestampMillis::new(1_000)));
        assert_eq!(lower.checked_sub(upper), None);
    }
}
